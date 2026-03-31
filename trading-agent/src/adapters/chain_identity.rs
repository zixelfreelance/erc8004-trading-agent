use async_trait::async_trait;
use crate::ports::identity::IdentityPort;

pub struct ChainIdentityAdapter {
    pub registry_address: String,  // contract address (from env)
    pub rpc_url: String,           // Sepolia RPC URL (from env)
    pub agent_uri: String,         // cached after registration
}

impl ChainIdentityAdapter {
    pub fn new(registry_address: String, rpc_url: String) -> Self {
        Self {
            registry_address,
            rpc_url,
            agent_uri: String::new(),
        }
    }

    /// Create a no-op adapter when chain config is not available
    pub fn noop() -> Self {
        Self {
            registry_address: String::new(),
            rpc_url: String::new(),
            agent_uri: String::new(),
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.registry_address.is_empty() && !self.rpc_url.is_empty()
    }
}

#[async_trait]
impl IdentityPort for ChainIdentityAdapter {
    async fn register_agent(&self, agent_uri: &str) -> anyhow::Result<u64> {
        if !self.is_configured() {
            eprintln!("chain: identity registry not configured, skipping registration");
            return Ok(0);
        }
        eprintln!("chain: register_agent(uri={agent_uri}) on {}", self.registry_address);
        // TODO: Replace with real ethers-rs contract call
        // let contract = AgentIdentityRegistry::new(addr, provider);
        // let tx = contract.register_with_uri(agent_uri).send().await?;
        Ok(0) // placeholder
    }

    async fn set_agent_wallet(&self, agent_id: u64, deadline: u64) -> anyhow::Result<()> {
        if !self.is_configured() { return Ok(()); }
        eprintln!("chain: set_agent_wallet(id={agent_id}, deadline={deadline})");
        Ok(())
    }

    async fn get_agent_wallet(&self, agent_id: u64) -> anyhow::Result<Option<String>> {
        if !self.is_configured() { return Ok(None); }
        eprintln!("chain: get_agent_wallet(id={agent_id})");
        Ok(None)
    }

    async fn get_agent_uri(&self, agent_id: u64) -> anyhow::Result<Option<String>> {
        if !self.is_configured() { return Ok(None); }
        eprintln!("chain: get_agent_uri(id={agent_id})");
        Ok(None)
    }
}
