use async_trait::async_trait;

use crate::domain::model::{Decision, MarketData};

#[async_trait]
pub trait DecisionPort: Send + Sync {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision>;
}
