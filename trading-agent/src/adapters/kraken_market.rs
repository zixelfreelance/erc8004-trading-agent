use std::process::Command;

use serde_json::Value;

use crate::domain::model::MarketData;
use crate::ports::market::MarketPort;

pub struct KrakenMarket {
    pub pair: String,
    pub ohlc_interval_minutes: u32,
    pub ohlc_lookback: usize,
}

struct TickerData {
    pair: String,
    last: f64,
    bid: Option<f64>,
    ask: Option<f64>,
    spread: Option<f64>,
    vwap_24h: Option<f64>,
    volume_24h: Option<f64>,
}

struct OhlcRow {
    high: f64,
    low: f64,
    close: f64,
}

struct OhlcData {
    closes: Vec<f64>,
    highs: Vec<f64>,
    lows: Vec<f64>,
}

impl KrakenMarket {
    pub fn new(pair: impl Into<String>) -> Self {
        Self {
            pair: pair.into(),
            ohlc_interval_minutes: 5,
            ohlc_lookback: 10,
        }
    }

    fn parse_field(ticker: &Value, key: &str, index: usize) -> Option<f64> {
        ticker.get(key)?.get(index).and_then(|v| {
            v.as_str()
                .and_then(|s| s.parse().ok())
                .or_else(|| v.as_f64())
        })
    }

    fn parse_ticker_json(stdout: &[u8]) -> anyhow::Result<TickerData> {
        let root: Value = serde_json::from_slice(stdout)?;
        if let Some(err) = root.get("error").and_then(|e| e.as_str()) {
            anyhow::bail!(
                "kraken cli: {err} — {}",
                root.get("message").and_then(|m| m.as_str()).unwrap_or("")
            );
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

        let bid = Self::parse_field(ticker, "b", 0);
        let ask = Self::parse_field(ticker, "a", 0);
        let spread = match (ask, bid) {
            (Some(a), Some(b)) => Some(a - b),
            _ => None,
        };
        let vwap_24h = Self::parse_field(ticker, "p", 1);
        let volume_24h = Self::parse_field(ticker, "v", 1);

        Ok(TickerData {
            pair: pair_key.clone(),
            last,
            bid,
            ask,
            spread,
            vwap_24h,
            volume_24h,
        })
    }

    fn parse_ohlc_row(v: &Value) -> Option<OhlcRow> {
        let row = v.as_array()?;
        let parse_val = |idx: usize| -> Option<f64> {
            let c = row.get(idx)?;
            if let Some(s) = c.as_str() {
                return s.parse().ok();
            }
            c.as_f64()
        };
        Some(OhlcRow {
            high: parse_val(2)?,
            low: parse_val(3)?,
            close: parse_val(4)?,
        })
    }

    fn parse_ohlc_data(stdout: &[u8], lookback: usize) -> anyhow::Result<OhlcData> {
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
        let mut closes = Vec::with_capacity(n);
        let mut highs = Vec::with_capacity(n);
        let mut lows = Vec::with_capacity(n);
        for row in &arr[start..] {
            if let Some(ohlc) = Self::parse_ohlc_row(row) {
                closes.push(ohlc.close);
                highs.push(ohlc.high);
                lows.push(ohlc.low);
            }
        }
        Ok(OhlcData {
            closes,
            highs,
            lows,
        })
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
        let ticker = Self::parse_ticker_json(&output.stdout)?;

        let ohlc = match Command::new("kraken")
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
            Ok(out) if out.status.success() => {
                Self::parse_ohlc_data(&out.stdout, self.ohlc_lookback).unwrap_or(OhlcData {
                    closes: Vec::new(),
                    highs: Vec::new(),
                    lows: Vec::new(),
                })
            }
            _ => OhlcData {
                closes: Vec::new(),
                highs: Vec::new(),
                lows: Vec::new(),
            },
        };

        Ok(MarketData {
            pair: ticker.pair,
            price: ticker.last,
            bid: ticker.bid,
            ask: ticker.ask,
            spread: ticker.spread,
            vwap_24h: ticker.vwap_24h,
            volume_24h: ticker.volume_24h,
            ohlc_closes: ohlc.closes,
            ohlc_highs: ohlc.highs,
            ohlc_lows: ohlc.lows,
        })
    }
}
