use crate::domain::model::MarketData;
use crate::ports::market::MarketPort;

/// Replays a fixed sequence of market data for deterministic demos and testing.
///
/// # Usage (demo mode)
///
/// Set `AGENT_DEMO_MODE=true` in your environment. When that var is set,
/// `main.rs` should construct `MockMarket::demo_sequence()` instead of
/// `KrakenMarket` so the agent runs a reproducible 50-tick scenario covering
/// uptrend, consolidation, sharp drop, and recovery phases.
pub struct MockMarket {
    ticks: Vec<MarketData>,
    index: std::sync::atomic::AtomicUsize,
}

impl MockMarket {
    pub fn new(ticks: Vec<MarketData>) -> Self {
        Self {
            ticks,
            index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Create a demo sequence with realistic BTC price movement.
    /// Includes: uptrend, consolidation, drop, recovery — triggers all strategy modes.
    pub fn demo_sequence() -> Self {
        let mut ticks = Vec::new();
        let base = 65000.0;

        // Phase 1: Uptrend (triggers momentum buy)
        for i in 0..15 {
            let price = base + i as f64 * 50.0;
            ticks.push(Self::make_tick(price, i));
        }

        // Phase 2: Consolidation/ranging (triggers mean-reversion)
        for i in 0..15 {
            let price = base + 750.0 + (i as f64 * 0.5).sin() * 100.0;
            ticks.push(Self::make_tick(price, 15 + i));
        }

        // Phase 3: Sharp drop (triggers ATR stop, circuit breaker)
        for i in 0..10 {
            let price = base + 750.0 - i as f64 * 80.0;
            ticks.push(Self::make_tick(price, 30 + i));
        }

        // Phase 4: Recovery (new buy opportunity after cooldown)
        for i in 0..10 {
            let price = base - 50.0 + i as f64 * 60.0;
            ticks.push(Self::make_tick(price, 40 + i));
        }

        Self::new(ticks)
    }

    fn make_tick(price: f64, seq: usize) -> MarketData {
        let n = 50;
        let mut closes = Vec::with_capacity(n);
        let mut highs = Vec::with_capacity(n);
        let mut lows = Vec::with_capacity(n);

        for j in 0..n {
            let offset = (n - j) as f64;
            let c = price - offset * 10.0 + (j as f64 * 0.3).sin() * 30.0;
            closes.push(c);
            highs.push(c + 20.0);
            lows.push(c - 20.0);
        }
        // Last candle = current price
        *closes.last_mut().unwrap() = price;
        *highs.last_mut().unwrap() = price + 15.0;
        *lows.last_mut().unwrap() = price - 15.0;

        MarketData {
            pair: "BTCUSD".to_string(),
            price,
            bid: Some(price - 0.5),
            ask: Some(price + 0.5),
            spread: Some(1.0),
            vwap_24h: Some(price - 50.0),
            volume_24h: Some(1000.0 + seq as f64 * 10.0),
            ohlc_closes: closes,
            ohlc_highs: highs,
            ohlc_lows: lows,
        }
    }

    pub fn len(&self) -> usize {
        self.ticks.len()
    }

    pub fn is_exhausted(&self) -> bool {
        self.index.load(std::sync::atomic::Ordering::Relaxed) >= self.ticks.len()
    }
}

impl MarketPort for MockMarket {
    fn get_market_data(&self) -> anyhow::Result<MarketData> {
        let idx = self
            .index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if idx >= self.ticks.len() {
            let wrapped = idx % self.ticks.len();
            Ok(self.ticks[wrapped].clone())
        } else {
            Ok(self.ticks[idx].clone())
        }
    }
}
