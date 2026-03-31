use crate::domain::intent::TradeIntent;
use crate::domain::model::Decision;

pub fn build_intent(
    decision: &Decision,
    price: f64,
    agent_id: &str,
    amount: f64,
) -> TradeIntent {
    TradeIntent {
        agent_id: agent_id.to_string(),
        action: format!("{:?}", decision.action),
        amount,
        price,
        timestamp: chrono::Utc::now().timestamp(),
    }
}
