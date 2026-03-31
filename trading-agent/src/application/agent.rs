use chrono::Utc;
use serde_json::json;

use crate::domain::risk;
use crate::ports::decision::DecisionPort;
use crate::ports::execution::ExecutionPort;
use crate::ports::market::MarketPort;

pub struct TradingAgent<M, D, E> {
    pub market: M,
    pub decision: D,
    pub execution: E,
}

impl<M, D, E> TradingAgent<M, D, E>
where
    M: MarketPort,
    D: DecisionPort,
    E: ExecutionPort,
{
    pub async fn run_once(&self) -> anyhow::Result<()> {
        let data = self.market.get_market_data()?;
        let proposed = self.decision.decide(&data).await?;
        let proposed_action = format!("{:?}", proposed.action);
        let proposed_confidence = proposed.confidence;
        let proposed_reason = proposed.reason.clone();
        let (final_decision, risk_blocked) = risk::apply_risk(proposed);

        self.execution.execute(&final_decision.action)?;

        let line = json!({
            "ts": Utc::now().to_rfc3339(),
            "pair": data.pair,
            "last": data.last,
            "proposed_action": proposed_action,
            "proposed_confidence": proposed_confidence,
            "proposed_reason": proposed_reason,
            "final_action": format!("{:?}", final_decision.action),
            "risk_blocked": risk_blocked,
            "final_reason": final_decision.reason,
        });
        println!("{}", line);

        Ok(())
    }
}
