use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use super::indicators;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketRegime {
    Trending,
    Ranging,
    Transition,
}

impl std::fmt::Display for MarketRegime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketRegime::Trending => write!(f, "trending"),
            MarketRegime::Ranging => write!(f, "ranging"),
            MarketRegime::Transition => write!(f, "transition"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegimeConfig {
    /// ADX above this for `persistence_bars` consecutive updates → enter Trending.
    pub adx_enter_trend: f64,
    /// ADX below this → exit Trending.
    pub adx_exit_trend: f64,
    /// Bollinger bandwidth percentile below this (in Ranging candidate) → Ranging.
    pub bbw_range_percentile: f64,
    /// Number of consecutive bars a signal must hold before state switches.
    pub persistence_bars: u32,
    /// ADX lookback period.
    pub adx_period: usize,
    /// Rolling window size for bandwidth percentile computation.
    pub bbw_window: usize,
}

impl Default for RegimeConfig {
    fn default() -> Self {
        Self {
            adx_enter_trend: 22.0,
            adx_exit_trend: 18.0,
            bbw_range_percentile: 0.4,
            persistence_bars: 3,
            adx_period: 14,
            bbw_window: 50,
        }
    }
}

/// Stateful regime detector with hysteresis to prevent whipsaw.
pub struct RegimeDetector {
    config: RegimeConfig,
    current: MarketRegime,
    candidate: MarketRegime,
    candidate_bars: u32,
    recent_bbw: VecDeque<f64>,
}

impl RegimeDetector {
    pub fn new(config: RegimeConfig) -> Self {
        Self {
            config,
            current: MarketRegime::Transition,
            candidate: MarketRegime::Transition,
            candidate_bars: 0,
            recent_bbw: VecDeque::new(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(RegimeConfig::default())
    }

    /// Feed new ADX and Bollinger bandwidth values. Returns the (possibly unchanged) regime.
    pub fn update(&mut self, adx: f64, bollinger_bandwidth: f64) -> MarketRegime {
        // Track bandwidth history for percentile calc
        if self.recent_bbw.len() >= self.config.bbw_window {
            self.recent_bbw.pop_front();
        }
        self.recent_bbw.push_back(bollinger_bandwidth);

        let raw = self.classify_raw(adx);

        if raw == self.candidate {
            self.candidate_bars += 1;
        } else {
            self.candidate = raw;
            self.candidate_bars = 1;
        }

        // Switch only after persistence threshold
        if self.candidate != self.current && self.candidate_bars >= self.config.persistence_bars {
            self.current = self.candidate;
        }

        self.current
    }

    /// Stateless classification for a single tick (used internally).
    fn classify_raw(&self, adx: f64) -> MarketRegime {
        match self.current {
            MarketRegime::Trending => {
                // Stay trending unless ADX drops below exit threshold
                if adx < self.config.adx_exit_trend {
                    if self.bbw_percentile() < self.config.bbw_range_percentile {
                        MarketRegime::Ranging
                    } else {
                        MarketRegime::Transition
                    }
                } else {
                    MarketRegime::Trending
                }
            }
            MarketRegime::Ranging | MarketRegime::Transition => {
                if adx >= self.config.adx_enter_trend {
                    MarketRegime::Trending
                } else if adx <= self.config.adx_exit_trend
                    && self.bbw_percentile() < self.config.bbw_range_percentile
                {
                    MarketRegime::Ranging
                } else {
                    MarketRegime::Transition
                }
            }
        }
    }

    /// What fraction of recent bandwidth values are >= the current one.
    fn bbw_percentile(&self) -> f64 {
        if self.recent_bbw.len() < 2 {
            return 0.5; // not enough data, assume middle
        }
        let current = *self.recent_bbw.back().unwrap();
        let count_below = self.recent_bbw.iter().filter(|&&v| v < current).count();
        count_below as f64 / self.recent_bbw.len() as f64
    }

    pub fn state(&self) -> MarketRegime {
        self.current
    }

    pub fn should_trade_momentum(&self) -> bool {
        self.current == MarketRegime::Trending
    }

    pub fn should_trade_reversion(&self) -> bool {
        self.current == MarketRegime::Ranging
    }
}

/// Stateless convenience for one-shot regime detection from OHLC data.
pub fn detect_regime(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    config: &RegimeConfig,
) -> MarketRegime {
    match indicators::adx(highs, lows, closes, config.adx_period) {
        Some(result) => {
            if result.adx >= config.adx_enter_trend {
                MarketRegime::Trending
            } else if result.adx <= config.adx_exit_trend {
                MarketRegime::Ranging
            } else {
                MarketRegime::Transition
            }
        }
        None => MarketRegime::Transition,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_in_transition() {
        let det = RegimeDetector::with_defaults();
        assert_eq!(det.state(), MarketRegime::Transition);
    }

    #[test]
    fn enters_trending_after_persistence() {
        let mut det = RegimeDetector::new(RegimeConfig {
            persistence_bars: 3,
            ..Default::default()
        });
        // Feed high ADX + some bandwidth
        assert_eq!(det.update(25.0, 0.05), MarketRegime::Transition); // bar 1
        assert_eq!(det.update(26.0, 0.05), MarketRegime::Transition); // bar 2
        assert_eq!(det.update(27.0, 0.05), MarketRegime::Trending);   // bar 3 → switch
    }

    #[test]
    fn stays_trending_despite_single_bar_dip() {
        let mut det = RegimeDetector::new(RegimeConfig {
            persistence_bars: 3,
            ..Default::default()
        });
        // Get into Trending
        for _ in 0..3 {
            det.update(25.0, 0.05);
        }
        assert_eq!(det.state(), MarketRegime::Trending);

        // Single bar dip below exit threshold
        det.update(15.0, 0.05);
        assert_eq!(det.state(), MarketRegime::Trending); // still trending (persistence)

        // Back to high ADX
        det.update(25.0, 0.05);
        assert_eq!(det.state(), MarketRegime::Trending); // candidate reset, still trending
    }

    #[test]
    fn exits_trending_on_sustained_low_adx() {
        let mut det = RegimeDetector::new(RegimeConfig {
            persistence_bars: 3,
            ..Default::default()
        });
        // Get into Trending
        for _ in 0..3 {
            det.update(25.0, 0.05);
        }
        assert_eq!(det.state(), MarketRegime::Trending);

        // Sustained low ADX + narrow bandwidth → should go to Ranging
        // First fill the bbw window with similar values so percentile is low
        for _ in 0..3 {
            det.update(15.0, 0.01);
        }
        assert_eq!(det.state(), MarketRegime::Ranging);
    }

    #[test]
    fn enters_ranging_with_low_adx_and_narrow_bandwidth() {
        let mut det = RegimeDetector::new(RegimeConfig {
            persistence_bars: 2,
            ..Default::default()
        });
        // Fill bandwidth window with varied values
        for i in 0..10 {
            det.update(15.0, 0.01 + i as f64 * 0.01);
        }
        // Now feed low ADX + the smallest bandwidth (low percentile)
        det.update(15.0, 0.001);
        det.update(15.0, 0.001);
        assert_eq!(det.state(), MarketRegime::Ranging);
    }

    #[test]
    fn transition_blocks_both_strategies() {
        let det = RegimeDetector::with_defaults();
        assert!(!det.should_trade_momentum());
        assert!(!det.should_trade_reversion());
    }

    #[test]
    fn trending_allows_momentum_only() {
        let mut det = RegimeDetector::new(RegimeConfig {
            persistence_bars: 1,
            ..Default::default()
        });
        det.update(30.0, 0.05);
        assert!(det.should_trade_momentum());
        assert!(!det.should_trade_reversion());
    }

    #[test]
    fn ranging_allows_reversion_only() {
        let mut det = RegimeDetector::new(RegimeConfig {
            persistence_bars: 1,
            ..Default::default()
        });
        // Fill window with varied bandwidths so percentile is meaningful
        for i in 0..10 {
            det.update(10.0, 0.02 + i as f64 * 0.01);
        }
        // Now feed the lowest bandwidth → low percentile → Ranging
        det.update(10.0, 0.001);
        assert!(det.should_trade_reversion());
        assert!(!det.should_trade_momentum());
    }

    #[test]
    fn stateless_detect_regime_trending() {
        let n = 50;
        let highs: Vec<f64> = (0..n).map(|i| 110.0 + i as f64 * 3.0).collect();
        let lows: Vec<f64> = (0..n).map(|i| 100.0 + i as f64 * 3.0).collect();
        let closes: Vec<f64> = (0..n).map(|i| 105.0 + i as f64 * 3.0).collect();
        let config = RegimeConfig::default();
        let regime = detect_regime(&highs, &lows, &closes, &config);
        assert_eq!(regime, MarketRegime::Trending);
    }

    #[test]
    fn stateless_detect_regime_insufficient_data() {
        let config = RegimeConfig::default();
        let regime = detect_regime(&[101.0, 102.0], &[99.0, 100.0], &[100.0, 101.0], &config);
        assert_eq!(regime, MarketRegime::Transition);
    }
}
