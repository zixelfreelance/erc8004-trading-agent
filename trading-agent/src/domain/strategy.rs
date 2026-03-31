use super::model::{Action, Decision, MarketData};

pub const STRATEGY_DISPLAY_NAME: &str = "Momentum + Volatility Guard Agent";

#[derive(Debug, Clone)]
pub struct MarketSnapshot {
    pub prices: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct StrategyConfig {
    pub momentum_threshold: f64,
    pub volatility_min: f64,
    pub volatility_max: f64,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            momentum_threshold: 50.0,
            volatility_min: 5.0,
            volatility_max: 500.0,
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

fn compute_volatility(prices: &[f64]) -> f64 {
    if prices.is_empty() {
        return 0.0;
    }
    let mean = prices.iter().sum::<f64>() / prices.len() as f64;

    let variance = prices
        .iter()
        .map(|p| (p - mean).powi(2))
        .sum::<f64>()
        / prices.len() as f64;

    variance.sqrt()
}

fn normalize(value: f64) -> f64 {
    (value.abs() / 1000.0).min(1.0)
}
