use super::indicators;
use super::model::{Action, Decision, MarketData};
use super::regime::MarketRegime;

pub const STRATEGY_DISPLAY_NAME: &str = "Regime-Aware Dual Strategy (Momentum + Mean-Reversion)";

#[derive(Debug, Clone)]
pub struct MarketSnapshot {
    pub prices: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct StrategyConfig {
    pub momentum_threshold: f64,
    pub volatility_min: f64,
    pub volatility_max: f64,
    pub rsi_oversold: f64,
    pub rsi_overbought: f64,
    pub bollinger_period: usize,
    pub bollinger_std: f64,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            momentum_threshold: 50.0,
            volatility_min: 5.0,
            volatility_max: 500.0,
            rsi_oversold: 30.0,
            rsi_overbought: 70.0,
            bollinger_period: 20,
            bollinger_std: 2.0,
        }
    }
}

pub fn market_snapshot_from(data: &MarketData) -> MarketSnapshot {
    let prices = if data.ohlc_closes.is_empty() {
        vec![data.price]
    } else {
        data.ohlc_closes.clone()
    };
    MarketSnapshot { prices }
}

pub fn compute_decision(data: &MarketSnapshot, config: &StrategyConfig) -> Decision {
    if data.prices.is_empty() {
        return Decision {
            action: Action::Hold,
            confidence: 0.3,
            reasoning: "No price history".to_string(),
        };
    }

    let current = *data.prices.last().expect("non-empty checked");

    let avg: f64 = data.prices.iter().sum::<f64>() / data.prices.len() as f64;

    let momentum = current - avg;

    let volatility = compute_volatility(&data.prices);

    let (action, confidence, reasoning) = if volatility < config.volatility_min {
        (Action::Hold, 0.3, "Volatility too low".to_string())
    } else if volatility > config.volatility_max {
        (Action::Hold, 0.4, "Volatility too high".to_string())
    } else if momentum > config.momentum_threshold {
        (
            Action::Buy,
            normalize(momentum),
            "Upward momentum with stable volatility".to_string(),
        )
    } else if momentum < -config.momentum_threshold {
        (
            Action::Sell,
            normalize(-momentum),
            "Downward momentum with stable volatility".to_string(),
        )
    } else {
        (Action::Hold, 0.5, "No strong signal".to_string())
    };

    Decision {
        action,
        confidence,
        reasoning,
    }
}

/// Mean-reversion strategy for ranging markets.
/// Buy when price touches lower Bollinger band AND RSI is oversold.
/// Sell when price touches upper Bollinger band AND RSI is overbought.
pub fn compute_reversion_decision(
    closes: &[f64],
    _highs: &[f64],
    _lows: &[f64],
    config: &StrategyConfig,
) -> Decision {
    if closes.len() < 26 {
        return Decision {
            action: Action::Hold,
            confidence: 0.3,
            reasoning: "Insufficient data for mean-reversion".into(),
        };
    }

    let price = *closes.last().unwrap();
    let rsi_val = indicators::rsi(closes, 14);
    let bb = indicators::bollinger(closes, config.bollinger_period, config.bollinger_std);

    match (rsi_val, bb) {
        (Some(rsi), Some(bands)) => {
            if price <= bands.lower && rsi <= config.rsi_oversold {
                let confidence = 0.6 + (config.rsi_oversold - rsi) / 100.0;
                Decision {
                    action: Action::Buy,
                    confidence: confidence.min(0.9),
                    reasoning: format!(
                        "Mean-reversion: price at lower Bollinger ({:.0}), RSI={:.0} oversold",
                        bands.lower, rsi
                    ),
                }
            } else if price >= bands.upper && rsi >= config.rsi_overbought {
                let confidence = 0.6 + (rsi - config.rsi_overbought) / 100.0;
                Decision {
                    action: Action::Sell,
                    confidence: confidence.min(0.9),
                    reasoning: format!(
                        "Mean-reversion: price at upper Bollinger ({:.0}), RSI={:.0} overbought",
                        bands.upper, rsi
                    ),
                }
            } else {
                Decision {
                    action: Action::Hold,
                    confidence: 0.5,
                    reasoning: format!(
                        "Mean-reversion: no extreme (RSI={:.0}, price between bands)",
                        rsi
                    ),
                }
            }
        }
        _ => Decision {
            action: Action::Hold,
            confidence: 0.3,
            reasoning: "Mean-reversion: insufficient indicator data".into(),
        },
    }
}

/// Combined strategy: momentum for trending, mean-reversion for ranging, hold for transition.
pub fn compute_regime_aware_decision(
    data: &MarketData,
    config: &StrategyConfig,
    regime: MarketRegime,
) -> Decision {
    match regime {
        MarketRegime::Trending => {
            let snapshot = market_snapshot_from(data);
            let mut d = compute_decision(&snapshot, config);
            d.reasoning = format!("[trending] {}", d.reasoning);
            d
        }
        MarketRegime::Ranging => {
            let mut d = compute_reversion_decision(
                &data.ohlc_closes,
                &data.ohlc_highs,
                &data.ohlc_lows,
                config,
            );
            d.reasoning = format!("[ranging] {}", d.reasoning);
            d
        }
        MarketRegime::Transition => Decision {
            action: Action::Hold,
            confidence: 0.4,
            reasoning: "[transition] Regime unclear, holding".into(),
        },
    }
}

fn compute_volatility(prices: &[f64]) -> f64 {
    if prices.is_empty() {
        return 0.0;
    }
    let mean = prices.iter().sum::<f64>() / prices.len() as f64;

    let variance = prices.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / prices.len() as f64;

    variance.sqrt()
}

fn normalize(value: f64) -> f64 {
    (value.abs() / 1000.0).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> StrategyConfig {
        StrategyConfig::default()
    }

    #[test]
    fn empty_prices_hold() {
        let snap = MarketSnapshot { prices: vec![] };
        let d = compute_decision(&snap, &cfg());
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn single_price_hold() {
        let snap = MarketSnapshot {
            prices: vec![100.0],
        };
        let d = compute_decision(&snap, &cfg());
        // Single price: momentum = price - avg = 0, volatility = 0 < min → Hold
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn upward_momentum_buys() {
        // Prices trending up so current >> avg, with enough volatility
        let snap = MarketSnapshot {
            prices: vec![100.0, 110.0, 120.0, 130.0, 200.0],
        };
        let d = compute_decision(&snap, &cfg());
        assert_eq!(d.action, Action::Buy);
    }

    #[test]
    fn downward_momentum_sells() {
        let snap = MarketSnapshot {
            prices: vec![200.0, 190.0, 180.0, 170.0, 100.0],
        };
        let d = compute_decision(&snap, &cfg());
        assert_eq!(d.action, Action::Sell);
    }

    #[test]
    fn flat_prices_hold() {
        let snap = MarketSnapshot {
            prices: vec![100.0, 100.0, 100.0, 100.0],
        };
        let d = compute_decision(&snap, &cfg());
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn low_volatility_hold() {
        // All prices nearly identical → volatility < min (5.0)
        let snap = MarketSnapshot {
            prices: vec![100.0, 100.001, 100.002, 100.001],
        };
        let d = compute_decision(&snap, &cfg());
        assert_eq!(d.action, Action::Hold);
        assert!(d.reasoning.contains("low"));
    }

    #[test]
    fn high_volatility_hold() {
        // Extreme spread → volatility > max (500.0)
        let snap = MarketSnapshot {
            prices: vec![1.0, 2000.0, 1.0, 2000.0],
        };
        let d = compute_decision(&snap, &cfg());
        assert_eq!(d.action, Action::Hold);
        assert!(d.reasoning.contains("high"));
    }

    #[test]
    fn confidence_capped_at_one() {
        // Extreme momentum → normalize should cap at 1.0
        let snap = MarketSnapshot {
            prices: vec![100.0, 100.0, 100.0, 5000.0],
        };
        let d = compute_decision(&snap, &cfg());
        assert!(d.confidence <= 1.0);
    }

    #[test]
    fn regime_aware_trending_uses_momentum() {
        let data = MarketData {
            pair: "BTCUSD".into(),
            price: 200.0,
            ohlc_closes: vec![100.0, 110.0, 120.0, 130.0, 200.0],
            ohlc_highs: vec![105.0, 115.0, 125.0, 135.0, 205.0],
            ohlc_lows: vec![95.0, 105.0, 115.0, 125.0, 195.0],
            ..Default::default()
        };
        let d = compute_regime_aware_decision(&data, &cfg(), MarketRegime::Trending);
        assert!(d.reasoning.contains("[trending]"));
    }

    #[test]
    fn regime_aware_transition_holds() {
        let data = MarketData {
            pair: "BTCUSD".into(),
            price: 100.0,
            ohlc_closes: vec![100.0; 30],
            ohlc_highs: vec![101.0; 30],
            ohlc_lows: vec![99.0; 30],
            ..Default::default()
        };
        let d = compute_regime_aware_decision(&data, &cfg(), MarketRegime::Transition);
        assert_eq!(d.action, Action::Hold);
        assert!(d.reasoning.contains("[transition]"));
    }

    #[test]
    fn reversion_hold_insufficient_data() {
        let d = compute_reversion_decision(&[100.0; 5], &[101.0; 5], &[99.0; 5], &cfg());
        assert_eq!(d.action, Action::Hold);
    }
}
