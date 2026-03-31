use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeLogRecord {
    pub timestamp: String,
    pub action: String,
    pub price: f64,
    pub confidence: f64,
    pub reasoning: String,
    pub pnl: f64,
    pub drawdown: f64,
}
