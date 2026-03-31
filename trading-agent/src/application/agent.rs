use crate::application::intent_builder::build_intent;
use crate::domain::risk;
use crate::ports::decision::DecisionPort;
use crate::ports::execution::ExecutionPort;
use crate::ports::market::MarketPort;
use crate::ports::signer::SignerPort;
use crate::ports::validation::ValidationPort;

pub struct TradingAgent<M, D, E, V, S> {
    pub market: M,
    pub decision: D,
    pub execution: E,
    pub validation: V,
    pub signer: S,
    pub agent_id: String,
    pub intent_amount: f64,
}

impl<M, D, E, V, S> TradingAgent<M, D, E, V, S>
where
    M: MarketPort,
    D: DecisionPort,
    E: ExecutionPort,
    V: ValidationPort,
    S: SignerPort,
{
    pub async fn run_once(&self) -> anyhow::Result<()> {
        let data = self.market.get_market_data()?;

        let decision = self.decision.decide(&data).await?;

        let (final_decision, blocked) = risk::apply_risk(decision);

        let intent = build_intent(
            &final_decision,
            data.price,
            &self.agent_id,
            self.intent_amount,
        );
        let signed_intent = self.signer.sign(intent);

        self.execution.execute(&final_decision.action)?;

        self.validation
            .log_decision(&data, &final_decision, blocked, &signed_intent)?;

        Ok(())
    }
}
