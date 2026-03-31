use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeIntent {
    pub agent_id: String,
    pub action: String,
    pub amount: f64,
    pub price: f64,
    pub timestamp: i64,
}
