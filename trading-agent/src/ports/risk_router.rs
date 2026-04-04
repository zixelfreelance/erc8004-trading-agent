use async_trait::async_trait;

use crate::domain::intent::TradeIntent;

#[async_trait]
pub trait RiskRouterPort: Send + Sync {
    /// Submit a signed trade intent to the on-chain RiskRouter.
    /// Returns Some(tx_hash) on success, None if not configured.
    async fn submit_intent(
        &self,
        intent: &TradeIntent,
        signature: &str,
    ) -> anyhow::Result<Option<String>>;
}
