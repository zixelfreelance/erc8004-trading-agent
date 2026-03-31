use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::model::{Decision, MarketData};
use crate::ports::decision::DecisionPort;

use super::adk_decision::AdkDecision;
use super::claude_decision::ClaudeDecision;
use super::hybrid_decision::HybridAdkDecision;
use super::momentum_decision::MomentumVolatilityDecision;

pub enum DecisionDriver {
    Momentum(MomentumVolatilityDecision),
    Claude(ClaudeDecision),
    Adk(Arc<AdkDecision>),
    Hybrid(Arc<HybridAdkDecision>),
}

#[async_trait]
impl DecisionPort for DecisionDriver {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision> {
        match self {
            Self::Momentum(m) => m.decide(data).await,
            Self::Claude(c) => c.decide(data).await,
            Self::Adk(a) => a.decide(data).await,
            Self::Hybrid(h) => h.decide(data).await,
        }
    }
}
