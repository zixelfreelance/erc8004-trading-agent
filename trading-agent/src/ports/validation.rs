use crate::domain::model::{Decision, MarketData};
use crate::domain::performance::Performance;
use crate::domain::signed_intent::SignedIntent;

pub trait ValidationPort: Send + Sync {
    fn log_decision(
        &self,
        data: &MarketData,
        decision: &Decision,
        blocked: bool,
        signed_intent: &SignedIntent,
        performance: &Performance,
    ) -> anyhow::Result<()>;
}
