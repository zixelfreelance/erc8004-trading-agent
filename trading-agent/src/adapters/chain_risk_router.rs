use crate::adapters::chain_provider::EthClient;
use crate::domain::intent::TradeIntent;
use crate::ports::risk_router::RiskRouterPort;
use async_trait::async_trait;
use ethers::prelude::*;

abigen!(RiskRouterContract, "abi/RiskRouter.json");

pub struct ChainRiskRouterAdapter {
    client: Option<EthClient>,
    router_address: Option<Address>,
}

impl ChainRiskRouterAdapter {
    pub fn new(router_address: String, client: EthClient) -> Self {
        let addr = router_address.parse::<Address>().ok();
        if addr.is_none() && !router_address.is_empty() {
            eprintln!("chain: invalid risk router address: {router_address}");
        }
        Self {
            client: Some(client),
            router_address: addr,
        }
    }

    pub fn noop() -> Self {
        Self {
            client: None,
            router_address: None,
        }
    }

    pub fn is_configured(&self) -> bool {
        self.client.is_some() && self.router_address.is_some()
    }

    fn contract(
        &self,
    ) -> Option<RiskRouterContract<SignerMiddleware<Provider<Http>, LocalWallet>>> {
        let client = self.client.as_ref()?;
        let addr = self.router_address?;
        Some(RiskRouterContract::new(addr, client.clone()))
    }
}

#[async_trait]
impl RiskRouterPort for ChainRiskRouterAdapter {
    async fn submit_intent(
        &self,
        intent: &TradeIntent,
        signature: &str,
    ) -> anyhow::Result<Option<String>> {
        let Some(contract) = self.contract() else {
            return Ok(None);
        };

        let agent_id = U256::from(
            intent
                .agent_id
                .strip_prefix("agent-")
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(1),
        );
        let wallet: Address = intent.agent_wallet.parse().unwrap_or(Address::zero());
        let action: u8 = match intent.action.as_str() {
            "Buy" => 0,
            "Sell" => 1,
            _ => 2,
        };

        let on_chain_intent = risk_router_contract::TradeIntent {
            agent_id,
            agent_wallet: wallet,
            pair: intent.pair.clone(),
            action,
            amount_usd_scaled: intent.amount_usd_scaled.into(),
            max_slippage_bps: intent.max_slippage_bps,
            nonce: intent.nonce,
            deadline: intent.deadline,
        };

        // Parse signature bytes from hex
        let sig_clean = signature.strip_prefix("0x").unwrap_or(signature);
        let sig_bytes = hex::decode(sig_clean).unwrap_or_default();

        eprintln!(
            "chain: submitting intent to RiskRouter (agent={}, action={}, nonce={})",
            intent.agent_id, intent.action, intent.nonce
        );

        let tx = contract
            .submit_intent(on_chain_intent, sig_bytes.into())
            .gas(500_000u64)
            .send()
            .await?
            .await?;

        if let Some(receipt) = tx {
            let tx_hash = format!("{:?}", receipt.transaction_hash);
            eprintln!("chain: intent submitted (tx={tx_hash})");
            Ok(Some(tx_hash))
        } else {
            Ok(Some("pending".to_string()))
        }
    }
}
