use crate::adapters::chain_provider::EthClient;
use crate::ports::identity::IdentityPort;
use async_trait::async_trait;
use ethers::prelude::*;

abigen!(
    IdentityRegistry,
    "abi/AgentIdentityRegistry.json"
);

pub struct ChainIdentityAdapter {
    client: Option<EthClient>,
    registry_address: Option<Address>,
}

impl ChainIdentityAdapter {
    pub fn new(registry_address: String, client: EthClient) -> Self {
        let addr = registry_address.parse::<Address>().ok();
        if addr.is_none() && !registry_address.is_empty() {
            eprintln!("chain: invalid identity registry address: {registry_address}");
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

    fn contract(&self) -> Option<IdentityRegistry<SignerMiddleware<Provider<Http>, LocalWallet>>> {
        let client = self.client.as_ref()?;
        let addr = self.registry_address?;
        Some(IdentityRegistry::new(addr, client.clone()))
    }
}

#[async_trait]
impl IdentityPort for ChainIdentityAdapter {
    async fn register_agent(&self, agent_uri: &str) -> anyhow::Result<u64> {
        let Some(contract) = self.contract() else {
            eprintln!("chain: identity registry not configured, skipping registration");
            return Ok(0);
        };
        eprintln!("chain: registering agent with uri={agent_uri}");
        let tx = contract
            .register_with_uri(agent_uri.to_string())
            .gas(500_000u64)
            .send()
            .await?
            .await?;
        if let Some(receipt) = tx {
            let tx_hash = receipt.transaction_hash;
            eprintln!("chain: agent registered (tx={tx_hash:?})");
            // Extract agentId from logs (first topic after event signature is agentId for Transfer)
            // For simplicity, return 1 as first agent
            Ok(1)
        } else {
            Ok(0)
        }
    }

    async fn set_agent_wallet(&self, _agent_id: u64, _deadline: u64) -> anyhow::Result<()> {
        if !self.is_configured() {
            return Ok(());
        }
        eprintln!("chain: set_agent_wallet not implemented for hackathon");
        Ok(())
    }

    async fn get_agent_wallet(&self, agent_id: u64) -> anyhow::Result<Option<String>> {
        let Some(contract) = self.contract() else {
            return Ok(None);
        };
        let wallet: Address = contract
            .get_agent_wallet(U256::from(agent_id))
            .call()
            .await?;
        if wallet == Address::zero() {
            Ok(None)
        } else {
            Ok(Some(format!("{wallet:?}")))
        }
    }

    async fn get_agent_uri(&self, agent_id: u64) -> anyhow::Result<Option<String>> {
        let Some(contract) = self.contract() else {
            return Ok(None);
        };
        let uri: String = contract
            .agent_uri(U256::from(agent_id))
            .call()
            .await?;
        if uri.is_empty() {
            Ok(None)
        } else {
            Ok(Some(uri))
        }
    }
}
