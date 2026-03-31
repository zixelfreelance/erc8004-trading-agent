use super::model::{Action, Decision, MarketData};

pub const STRATEGY_DISPLAY_NAME: &str = "Momentum + Volatility Guard Agent";

#[derive(Debug, Clone)]
pub struct MomentumVolConfig {
    pub lookback: usize,
    pub momentum_threshold_pct: f64,
    pub volatility_cv_low: f64,
    pub volatility_cv_high: f64,
}

impl Default for MomentumVolConfig {
    fn default() -> Self {
        Self {
            lookback: 10,
            momentum_threshold_pct: 0.00015,
            volatility_cv_low: 0.00008,
            volatility_cv_high: 0.025,
        }
    }
}

/// Rule-based momentum vs rolling OHLC average, gated by coefficient-of-variation band.
pub fn evaluate_momentum_volatility(data: &MarketData, cfg: &MomentumVolConfig) -> Decision {
    let full = &data.ohlc_closes;
    const MIN_CANDLES: usize = 5;

    if full.len() < MIN_CANDLES {
        return Decision {
            action: Action::Hold,
            confidence: 0.4,
            reasoning: format!(
                "{STRATEGY_DISPLAY_NAME}: need at least {MIN_CANDLES} OHLC closes (got {})",
                full.len()
            ),
        };
    }

    let take = cfg.lookback.min(full.len()).max(MIN_CANDLES);
    let start = full.len().saturating_sub(take);
    let closes = &full[start..];

    let avg: f64 = closes.iter().sum::<f64>() / closes.len() as f64;
    if !avg.is_finite() || avg <= 0.0 {
        return Decision {
            action: Action::Hold,
            confidence: 0.35,
            reasoning: format!("{STRATEGY_DISPLAY_NAME}: invalid average over closes"),
        };
    }

    let momentum_pct = (data.price - avg) / avg;

    let mean = avg;
    let variance: f64 =
        closes.iter().map(|c| (c - mean).powi(2)).sum::<f64>() / closes.len() as f64;
    let stdev = variance.sqrt();
    let cv = stdev / mean;

    if cv < cfg.volatility_cv_low {
        return Decision {
            action: Action::Hold,
            confidence: 0.55,
            reasoning: format!(
                "{STRATEGY_DISPLAY_NAME}: volatility too low (CV={cv:.6}); no opportunity — momentum {:+.4}% vs {}-candle avg",
                momentum_pct * 100.0,
                closes.len(),
            ),
        };
    }
    if cv > cfg.volatility_cv_high {
        return Decision {
            action: Action::Hold,
            confidence: 0.58,
            reasoning: format!(
                "{STRATEGY_DISPLAY_NAME}: volatility too high (CV={cv:.6}); stand aside — momentum {:+.4}% vs avg",
                momentum_pct * 100.0,
            ),
        };
    }

    let th = cfg.momentum_threshold_pct;
    let vol_note = format!("volatility OK (CV={cv:.6})");

    if momentum_pct > th {
        let strength = ((momentum_pct / th).min(3.0) - 1.0) / 2.0;
        let confidence = (0.62 + strength * 0.2).clamp(0.6, 0.88);
        Decision {
            action: Action::Buy,
            confidence,
            reasoning: format!(
                "{STRATEGY_DISPLAY_NAME}: upward momentum vs {}-candle avg ({:+.4}%); {vol_note}",
                closes.len(),
                momentum_pct * 100.0,
            ),
        }
    } else if momentum_pct < -th {
        let strength = (((-momentum_pct) / th).min(3.0) - 1.0) / 2.0;
        let confidence = (0.62 + strength * 0.2).clamp(0.6, 0.88);
        Decision {
            action: Action::Sell,
            confidence,
            reasoning: format!(
                "{STRATEGY_DISPLAY_NAME}: downward momentum vs {}-candle avg ({:+.4}%); {vol_note}",
                closes.len(),
                momentum_pct * 100.0,
            ),
        }
    } else {
        Decision {
            action: Action::Hold,
            confidence: 0.52,
            reasoning: format!(
                "{STRATEGY_DISPLAY_NAME}: no clear momentum ({:+.4}% vs threshold {:.4}%); {vol_note}",
                momentum_pct * 100.0,
                th * 100.0,
            ),
        }
    }
}

// --- Hybrid ADK: same thresholds as `MomentumVolConfig`, keyed for optional overrides ---

#[derive(Debug, Clone)]
pub struct StrategyConfig {
    pub momentum_threshold: f64,
    pub volatility_min: f64,
    pub volatility_max: f64,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        let m = MomentumVolConfig::default();
        Self {
            momentum_threshold: m.momentum_threshold_pct,
            volatility_min: m.volatility_cv_low,
            volatility_max: m.volatility_cv_high,
        }
    }
}

pub fn compute_decision(data: &MarketData, config: &StrategyConfig) -> Decision {
    let cfg = MomentumVolConfig {
        lookback: data.ohlc_closes.len().max(1),
        momentum_threshold_pct: config.momentum_threshold,
        volatility_cv_low: config.volatility_min,
        volatility_cv_high: config.volatility_max,
    };
    evaluate_momentum_volatility(data, &cfg)
}
