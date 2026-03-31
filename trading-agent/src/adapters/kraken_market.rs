use std::process::Command;

use serde_json::Value;

use crate::domain::model::MarketData;
use crate::ports::market::MarketPort;

pub struct KrakenMarket {
    pub pair: String,
    pub ohlc_interval_minutes: u32,
    pub ohlc_lookback: usize,
}

impl KrakenMarket {
    pub fn new(pair: impl Into<String>) -> Self {
        Self {
            pair: pair.into(),
            ohlc_interval_minutes: 5,
            ohlc_lookback: 10,
        }
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

    fn close_from_ohlc_row(v: &Value) -> Option<f64> {
        let row = v.as_array()?;
        let c = row.get(4)?;
        if let Some(s) = c.as_str() {
            return s.parse().ok();
        }
        c.as_f64()
    }

    fn parse_ohlc_closes(stdout: &[u8], lookback: usize) -> anyhow::Result<Vec<f64>> {
        let root: Value = serde_json::from_slice(stdout)?;
        if let Some(err) = root.get("error").and_then(|e| e.as_str()) {
            anyhow::bail!(
                "kraken ohlc: {err} — {}",
                root.get("message").and_then(|m| m.as_str()).unwrap_or("")
            );
        }
        let obj = root
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("ohlc: expected JSON object"))?;
        let arr = obj
            .values()
            .find(|v| v.is_array())
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow::anyhow!("ohlc: no candle array"))?;
        let n = lookback.min(arr.len());
        let start = arr.len().saturating_sub(n);
        let closes: Vec<f64> = arr[start..]
            .iter()
            .filter_map(Self::close_from_ohlc_row)
            .collect();
        Ok(closes)
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

        let ohlc_closes = match Command::new("kraken")
            .args([
                "ohlc",
                &self.pair,
                "--interval",
                &self.ohlc_interval_minutes.to_string(),
                "-o",
                "json",
            ])
            .output()
        {
            Ok(out) if out.status.success() => Self::parse_ohlc_closes(&out.stdout, self.ohlc_lookback)
                .unwrap_or_default(),
            _ => Vec::new(),
        };

        Ok(MarketData {
            pair,
            price: last,
            ohlc_closes,
        })
    }
}
