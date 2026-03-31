use async_trait::async_trait;

#[async_trait]
pub trait IdentityPort: Send + Sync {
    /// Register agent on the identity registry, returns agent ID
    async fn register_agent(&self, agent_uri: &str) -> anyhow::Result<u64>;
    /// Set the agent's hot wallet via EIP-712 authorization
    async fn set_agent_wallet(&self, agent_id: u64, deadline: u64) -> anyhow::Result<()>;
    /// Get the agent wallet address for a given agent ID
    async fn get_agent_wallet(&self, agent_id: u64) -> anyhow::Result<Option<String>>;
    /// Get the agent URI
    async fn get_agent_uri(&self, agent_id: u64) -> anyhow::Result<Option<String>>;
}
