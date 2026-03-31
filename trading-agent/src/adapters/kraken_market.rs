use std::process::Command;

use serde_json::Value;

use crate::domain::model::MarketData;
use crate::ports::market::MarketPort;

pub struct KrakenMarket {
    pub pair: String,
}

impl KrakenMarket {
    pub fn new(pair: impl Into<String>) -> Self {
        Self { pair: pair.into() }
    }

    fn parse_ticker_json(stdout: &[u8]) -> anyhow::Result<(String, f64)> {
        let root: Value = serde_json::from_slice(stdout)?;
        if let Some(err) = root.get("error").and_then(|e| e.as_str()) {
            anyhow::bail!("kraken cli: {err} — {}", root.get("message").and_then(|m| m.as_str()).unwrap_or(""));
        }
        let obj = root
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("ticker: expected JSON object"))?;
        let (pair_key, ticker) = obj
            .iter()
            .find(|(_, v)| v.get("c").is_some())
            .ok_or_else(|| anyhow::anyhow!("ticker: no pair data in response"))?;
        let last = ticker["c"]
            .get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("ticker: missing last price (c[0])"))?
            .parse::<f64>()?;
        Ok((pair_key.clone(), last))
    }
}

impl MarketPort for KrakenMarket {
    fn get_market_data(&self) -> anyhow::Result<MarketData> {
        let output = Command::new("kraken")
            .args(["ticker", &self.pair, "-o", "json"])
            .output()?;
        if !output.status.success() {
            anyhow::bail!(
                "kraken ticker failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        let (pair, last) = Self::parse_ticker_json(&output.stdout)?;
        Ok(MarketData { pair, last })
    }
}
