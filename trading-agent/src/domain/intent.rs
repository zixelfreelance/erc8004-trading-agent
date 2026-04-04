use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeIntent {
    pub agent_id: String,
    pub action: String,
    pub amount: f64,
    pub price: f64,
    pub timestamp: i64,
    // RiskRouter compatibility
    pub pair: String,
    pub agent_wallet: String,
    pub amount_usd_scaled: u128,
    pub max_slippage_bps: u32,
    pub nonce: u64,
    pub deadline: u64,
}

impl Default for TradeIntent {
    fn default() -> Self {
        Self {
            agent_id: String::new(),
            action: String::new(),
            amount: 0.0,
            price: 0.0,
            timestamp: 0,
            pair: "BTCUSD".to_string(),
            agent_wallet: String::new(),
            amount_usd_scaled: 0,
            max_slippage_bps: 50,
            nonce: 0,
            deadline: 0,
        }
    }
}
