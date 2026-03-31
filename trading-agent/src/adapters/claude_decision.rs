use async_trait::async_trait;

use crate::domain::model::{Action, Decision, MarketData};
use crate::ports::decision::DecisionPort;

/// Placeholder decision adapter — swap for ADK + Anthropic / structured JSON later.
pub struct MockClaudeDecision;

#[async_trait]
impl DecisionPort for MockClaudeDecision {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision> {
        Ok(Decision {
            action: Action::Hold,
            confidence: 0.75,
            reason: format!("mock: observing {} last={}", data.pair, data.last),
        })
    }
}
