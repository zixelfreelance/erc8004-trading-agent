use crate::domain::model::MarketData;

pub trait MarketPort: Send + Sync {
    fn get_market_data(&self) -> anyhow::Result<MarketData>;
}

impl MarketPort for Box<dyn MarketPort> {
    fn get_market_data(&self) -> anyhow::Result<MarketData> {
        (**self).get_market_data()
    }
}
