use std::sync::Mutex;

use crate::application::intent_builder::build_intent;
use crate::domain::risk::{self, PositionState, RiskConfig};
use crate::ports::decision::DecisionPort;
use crate::ports::execution::ExecutionPort;
use crate::ports::market::MarketPort;
use crate::ports::performance::PerformancePort;
use crate::ports::signer::SignerPort;
use crate::ports::validation::ValidationPort;

pub struct TradingAgent<M, D, E, V, S, P> {
    pub market: M,
    pub decision: D,
    pub execution: E,
    pub validation: V,
    pub signer: S,
    pub performance: P,
    pub position: Mutex<PositionState>,
    pub risk_config: RiskConfig,
    pub agent_id: String,
    pub intent_amount: f64,
}

impl<M, D, E, V, S, P> TradingAgent<M, D, E, V, S, P>
where
    M: MarketPort,
    D: DecisionPort,
    E: ExecutionPort,
    V: ValidationPort,
    S: SignerPort,
    P: PerformancePort,
{
    pub async fn run_once(&self) -> anyhow::Result<()> {
        let data = self.market.get_market_data()?;

        let decision = self.decision.decide(&data).await?;

        let perf_snapshot = self.performance.snapshot();
        let position_guard = self.position.lock().expect("position mutex poisoned");
        let (final_decision, blocked) =
            risk::apply_risk(decision, &position_guard, &perf_snapshot, &self.risk_config);
        drop(position_guard);

        let intent = build_intent(
            &final_decision,
            data.price,
            &self.agent_id,
            self.intent_amount,
        );
        let signed_intent = self.signer.sign(intent);

        let fill = self.execution.execute(&final_decision.action)?;
        if let Some(b) = fill.parsed_balance {
            self.performance.record_balance(b);
        }

        {
            let mut pos = self.position.lock().expect("position mutex poisoned");
            match final_decision.action {
                crate::domain::model::Action::Buy => pos.open_long = true,
                crate::domain::model::Action::Sell => pos.open_long = false,
                crate::domain::model::Action::Hold => {}
            }
        }

        let perf = self.performance.snapshot();
        self.validation
            .log_decision(&data, &final_decision, blocked, &signed_intent, &perf)?;

        Ok(())
    }
}
