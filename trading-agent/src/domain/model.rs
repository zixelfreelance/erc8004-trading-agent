#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Buy/Sell used when decision adapter returns them
pub enum Action {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone)]
pub struct MarketData {
    pub pair: String,
    pub last: f64,
}

#[derive(Debug, Clone)]
pub struct Decision {
    pub action: Action,
    pub confidence: f64,
    pub reason: String,
}
