use async_trait::async_trait;

use crate::domain::model::{Decision, MarketData};
use crate::domain::risk::RiskConfig;
use crate::domain::strategy::{compute_decision, market_snapshot_from, StrategyConfig};
use crate::ports::decision::DecisionPort;

use super::adk_decision::AdkDecision;

/// Runs deterministic strategy first, then ADK with that signal as prior context.
pub struct HybridAdkDecision {
    adk: AdkDecision,
    strategy: StrategyConfig,
}

impl HybridAdkDecision {
    pub async fn new(strategy: StrategyConfig, risk_limits: RiskConfig) -> anyhow::Result<Self> {
        Ok(Self {
            adk: AdkDecision::new(risk_limits).await?,
            strategy,
        })
    }
}

#[async_trait]
impl DecisionPort for HybridAdkDecision {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision> {
        let snapshot = market_snapshot_from(data);
        let prior = compute_decision(&snapshot, &self.strategy);
        let action_s = match prior.action {
            crate::domain::model::Action::Buy => "Buy",
            crate::domain::model::Action::Sell => "Sell",
            crate::domain::model::Action::Hold => "Hold",
        };
        let extra = format!(
            "Deterministic strategy signal:\n- action: {action_s}\n- confidence: {:.4}\n- reasoning: {}\n\nTreat this as a strong prior; you may override only with clear justification in your JSON reasoning.",
            prior.confidence, prior.reasoning
        );
        self.adk.decide_with_extra_context(data, &extra).await
    }
}
