use async_trait::async_trait;

use crate::domain::model::{Decision, MarketData};
use crate::domain::strategy::{evaluate_momentum_volatility, MomentumVolConfig};
use crate::ports::decision::DecisionPort;

pub struct MomentumVolatilityDecision {
    pub config: MomentumVolConfig,
}

impl MomentumVolatilityDecision {
    pub fn new(config: MomentumVolConfig) -> Self {
        Self { config }
    }
}

impl Default for MomentumVolatilityDecision {
    fn default() -> Self {
        Self::new(MomentumVolConfig::default())
    }
}

#[async_trait]
impl DecisionPort for MomentumVolatilityDecision {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision> {
        Ok(evaluate_momentum_volatility(data, &self.config))
    }
}
