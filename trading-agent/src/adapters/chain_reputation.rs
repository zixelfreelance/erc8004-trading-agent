use crate::adapters::chain_provider::EthClient;
use crate::ports::reputation::{ReputationMetric, ReputationPort};
use async_trait::async_trait;
use ethers::prelude::*;

abigen!(
    ReputationRegistry,
    "abi/AgentReputationRegistry.json"
);

pub struct ChainReputationAdapter {
    client: Option<EthClient>,
    registry_address: Option<Address>,
}

impl ChainReputationAdapter {
    pub fn new(registry_address: String, client: EthClient) -> Self {
        let addr = registry_address.parse::<Address>().ok();
        if addr.is_none() && !registry_address.is_empty() {
            eprintln!("chain: invalid reputation registry address: {registry_address}");
        }
        Self {
            client: Some(client),
            registry_address: addr,
        }
    }

    pub fn noop() -> Self {
        Self {
            client: None,
            registry_address: None,
        }
    }

    pub fn is_configured(&self) -> bool {
        self.client.is_some() && self.registry_address.is_some()
    }

    fn contract(&self) -> Option<ReputationRegistry<SignerMiddleware<Provider<Http>, LocalWallet>>> {
        let client = self.client.as_ref()?;
        let addr = self.registry_address?;
        Some(ReputationRegistry::new(addr, client.clone()))
    }
}

#[async_trait]
impl ReputationPort for ChainReputationAdapter {
    async fn post_feedback(
        &self,
        agent_id: u64,
        metric: &ReputationMetric,
        feedback_uri: &str,
    ) -> anyhow::Result<String> {
        let Some(contract) = self.contract() else {
            eprintln!("chain: reputation registry not configured, skipping feedback");
            return Ok("noop".to_string());
        };
        let feedback_hash: [u8; 32] = ethers::core::utils::keccak256(feedback_uri.as_bytes());
        eprintln!(
            "chain: posting feedback agent={agent_id} {}/{}={} dec={}",
            metric.tag1, metric.tag2, metric.value, metric.decimals
        );
        let tx = contract
            .give_feedback(
                U256::from(agent_id),
                metric.value,
                metric.decimals,
                metric.tag1.clone(),
                metric.tag2.clone(),
                String::new(), // endpoint
                feedback_uri.to_string(),
                feedback_hash,
            )
            .gas(500_000u64)
            .send()
            .await?
            .await?;
        if let Some(receipt) = tx {
            let tx_hash = format!("{:?}", receipt.transaction_hash);
            eprintln!("chain: feedback posted (tx={tx_hash})");
            Ok(tx_hash)
        } else {
            Ok("pending".to_string())
        }
    }

    async fn get_reputation(
        &self,
        agent_id: u64,
        tag1: &str,
        tag2: &str,
    ) -> anyhow::Result<Option<(u64, i128, u8)>> {
        let Some(contract) = self.contract() else {
            return Ok(None);
        };
        let (count, value, decimals) = contract
            .get_summary(U256::from(agent_id), tag1.to_string(), tag2.to_string())
            .call()
            .await?;
        Ok(Some((count, value, decimals)))
    }
}
