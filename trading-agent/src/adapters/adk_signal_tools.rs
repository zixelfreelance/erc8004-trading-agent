use std::sync::Arc;

use adk_rust::prelude::*;
use adk_rust::serde_json::{json, Value};
use schemars::JsonSchema;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::domain::model::MarketData;
use crate::domain::risk::RiskConfig;

#[derive(JsonSchema, Serialize)]
pub struct EmptyArgs {}

fn log_returns(closes: &[f64]) -> Vec<f64> {
    closes
        .windows(2)
        .filter(|w| w[0] > 0.0 && w[1] > 0.0)
        .map(|w| (w[1] / w[0]).ln())
        .collect()
}

fn mean_sample_std(xs: &[f64]) -> Option<(f64, f64)> {
    if xs.is_empty() {
        return None;
    }
    let n = xs.len() as f64;
    let mean = xs.iter().sum::<f64>() / n;
    if xs.len() < 2 {
        return Some((mean, 0.0));
    }
    let var: f64 = xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    Some((mean, var.sqrt()))
}

pub fn price_action_payload(data: &MarketData) -> Value {
    let closes = &data.ohlc_closes;
    let n = closes.len();
    let (momentum_pct, trend) = if n >= 2 && closes[0] > 0.0 {
        let m = (closes[n - 1] - closes[0]) / closes[0] * 100.0;
        let t = if m > 0.05 {
            "up"
        } else if m < -0.05 {
            "down"
        } else {
            "flat"
        };
        (Some(m), t)
    } else {
        (None, "unknown")
    };

    let lr = log_returns(closes);
    let vol_log = mean_sample_std(&lr).map(|(_, s)| s).unwrap_or(0.0);

    let z_last = if n >= 2 {
        mean_sample_std(closes).and_then(|(mean, std)| {
            if std > 1e-12 {
                Some((data.price - mean) / std)
            } else {
                None
            }
        })
    } else {
        None
    };

    json!({
        "pair": data.pair,
        "last_price": data.price,
        "ohlc_closes_available": n,
        "simple_momentum_pct_across_window": momentum_pct,
        "log_return_sample_stdev": vol_log,
        "last_price_z_vs_ohlc_mean": z_last,
        "trend_label": trend,
        "insufficient_history": n < 3,
    })
}

pub fn risk_limits_payload(cfg: &RiskConfig) -> Value {
    json!({
        "min_confidence_to_trade": cfg.min_confidence_trade,
        "max_drawdown_limit_fraction": cfg.max_drawdown,
        "note": "These values match the live TradingAgent risk_config. The orchestrator still enforces drawdown, position, and confidence after your JSON output; align your stated confidence with these limits.",
    })
}

pub fn sentiment_stub_payload() -> Value {
    json!({
        "status": "unavailable",
        "bias": "neutral",
        "note": "No live sentiment/news feed wired in this build. Do not invent headlines; rely on price_action + risk tools.",
    })
}

pub fn signal_tools(
    tick: Arc<RwLock<Option<MarketData>>>,
    risk_limits: Arc<RiskConfig>,
) -> Vec<Arc<dyn Tool>> {
    let t1 = Arc::clone(&tick);
    let price_action = Arc::new(
        FunctionTool::new(
            "compute_price_action_signals",
            "OHLC-derived momentum, log-return volatility, z-score vs window mean, trend label. Call once per tick before final JSON.",
            move |_ctx, _args: Value| {
                let t1 = Arc::clone(&t1);
                async move {
                    let guard = t1.read().await;
                    let Some(data) = guard.as_ref() else {
                        return Err(AdkError::tool(
                            "internal: no market tick snapshot (adapter bug)",
                        ));
                    };
                    Ok(price_action_payload(data))
                }
            },
        )
        .with_parameters_schema::<EmptyArgs>(),
    );

    let risk_arc = Arc::clone(&risk_limits);
    let risk_tool = Arc::new(
        FunctionTool::new(
            "risk_limits_snapshot",
            "Hard portfolio policy limits from the same RiskConfig as the running agent. Call before final JSON.",
            move |_ctx, _args: Value| {
                let risk_arc = Arc::clone(&risk_arc);
                async move { Ok(risk_limits_payload(&risk_arc)) }
            },
        )
        .with_parameters_schema::<EmptyArgs>(),
    );

    let sentiment = Arc::new(
        FunctionTool::new(
            "external_sentiment_stub",
            "Placeholder for news/social sentiment; currently reports unavailable so you do not hallucinate feeds.",
            |_ctx, _args: Value| async move { Ok(sentiment_stub_payload()) },
        )
        .with_parameters_schema::<EmptyArgs>(),
    );

    vec![price_action, risk_tool, sentiment]
}
