use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Action {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub pair: String,
    pub price: f64,
    #[serde(default)]
    pub ohlc_closes: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub action: Action,
    pub confidence: f64,
    pub reasoning: String,
}
