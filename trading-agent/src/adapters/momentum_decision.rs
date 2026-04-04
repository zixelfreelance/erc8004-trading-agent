use async_trait::async_trait;

use crate::domain::model::{Decision, MarketData};
use crate::domain::regime;
use crate::domain::strategy::{compute_regime_aware_decision, StrategyConfig};
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
        let detected_regime = regime::detect_regime(
            &data.ohlc_highs,
            &data.ohlc_lows,
            &data.ohlc_closes,
            &regime::RegimeConfig::default(),
        );
        Ok(compute_regime_aware_decision(
            data,
            &self.config,
            detected_regime,
        ))
    }
}
