use async_trait::async_trait;

use crate::domain::model::{Decision, MarketData};
use crate::domain::strategy::{compute_decision, market_snapshot_from, StrategyConfig};
use crate::ports::decision::DecisionPort;

pub struct MomentumVolatilityDecision {
    pub config: StrategyConfig,
}

impl MomentumVolatilityDecision {
    pub fn new(config: StrategyConfig) -> Self {
        Self { config }
    }
}

impl Default for MomentumVolatilityDecision {
    fn default() -> Self {
        Self::new(StrategyConfig::default())
    }
}

#[async_trait]
impl DecisionPort for MomentumVolatilityDecision {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision> {
        let snapshot = market_snapshot_from(data);
        Ok(compute_decision(&snapshot, &self.config))
    }
}
