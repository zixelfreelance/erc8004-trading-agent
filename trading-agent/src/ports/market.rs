use crate::domain::model::MarketData;

pub trait MarketPort: Send + Sync {
    fn get_market_data(&self) -> anyhow::Result<MarketData>;
}
