mod adapters;
mod application;
mod domain;
mod ports;

use std::net::SocketAddr;
use std::sync::Arc;

use adapters::decision_driver::DecisionDriver;
use adapters::http_logs;
use adapters::kraken_execution::KrakenPaperExecution;
use adapters::kraken_market::KrakenMarket;
use adapters::momentum_decision::MomentumVolatilityDecision;
use adapters::performance_tracker::PerformanceTracker;
use adapters::signer::SimpleSigner;
use adapters::validation::{ArtifactValidation, SharedLogEntries};
use application::agent::TradingAgent;
use domain::risk::{PositionState, RiskConfig};
use domain::strategy::{MomentumVolConfig, StrategyConfig, STRATEGY_DISPLAY_NAME};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pair = std::env::var("AGENT_PAIR").unwrap_or_else(|_| "BTCUSD".to_string());
    let volume = std::env::var("AGENT_VOLUME").unwrap_or_else(|_| "0.001".to_string());
    let intent_amount: f64 = volume.parse().unwrap_or(0.001);
    let agent_id = std::env::var("AGENT_ID").unwrap_or_else(|_| "agent-001".to_string());
    let signing_key =
        std::env::var("AGENT_SIGNING_KEY").unwrap_or_else(|_| "dev-local-key".to_string());
    let initial_balance: f64 = std::env::var("AGENT_INITIAL_BALANCE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10_000.0);

    let http_port: u16 = std::env::var("AGENT_HTTP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3030);

    let decision_mode = std::env::var("AGENT_DECISION").unwrap_or_else(|_| "momentum".to_string());
    let decision_mode = decision_mode.to_lowercase();

    let ohlc_interval: u32 = std::env::var("AGENT_OHLC_INTERVAL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    let ohlc_lookback: usize = std::env::var("AGENT_MOMENTUM_LOOKBACK")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let momentum_cfg = MomentumVolConfig {
        lookback: ohlc_lookback,
        momentum_threshold_pct: std::env::var("AGENT_MOMENTUM_THRESHOLD_PCT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.00015),
        volatility_cv_low: std::env::var("AGENT_VOL_CV_LOW")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.00008),
        volatility_cv_high: std::env::var("AGENT_VOL_CV_HIGH")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.025),
    };

    let strategy_cfg_hybrid = StrategyConfig {
        momentum_threshold: momentum_cfg.momentum_threshold_pct,
        volatility_min: momentum_cfg.volatility_cv_low,
        volatility_max: momentum_cfg.volatility_cv_high,
    };

    let risk_config = RiskConfig {
        min_confidence_trade: std::env::var("AGENT_RISK_MIN_CONFIDENCE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.6),
        max_drawdown: std::env::var("AGENT_MAX_DRAWDOWN")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.05),
    };

    let strategy_display_name =
        std::env::var("AGENT_STRATEGY_LABEL").unwrap_or_else(|_| match decision_mode.as_str() {
            "claude" | "adk" => "LLM-assisted decision (risk gates + signed intents)".to_string(),
            "hybrid" => "Hybrid: rule-based signal + ADK final decision".to_string(),
            _ => STRATEGY_DISPLAY_NAME.to_string(),
        });

    let decision: DecisionDriver = match decision_mode.as_str() {
        "claude" => DecisionDriver::Claude(adapters::claude_decision::ClaudeDecision),
        "adk" => DecisionDriver::Adk(Arc::new(adapters::adk_decision::AdkDecision::new().await?)),
        "hybrid" => DecisionDriver::Hybrid(Arc::new(
            adapters::hybrid_decision::HybridAdkDecision::new(strategy_cfg_hybrid.clone()).await?,
        )),
        _ => DecisionDriver::Momentum(MomentumVolatilityDecision::new(momentum_cfg)),
    };

    let log_entries: SharedLogEntries = Arc::new(std::sync::Mutex::new(Vec::new()));
    let log_entries_srv = Arc::clone(&log_entries);

    let app = http_logs::router(log_entries_srv);
    let addr = SocketAddr::from(([0, 0, 0, 0], http_port));
    tokio::spawn(async move {
        if let Err(e) = http_logs::serve(app, addr).await {
            eprintln!("http server error: {e:#}");
        }
    });
    eprintln!("GET /logs on http://{addr}/logs");

    let mut market = KrakenMarket::new(&pair);
    market.ohlc_interval_minutes = ohlc_interval;
    market.ohlc_lookback = ohlc_lookback;

    let agent = TradingAgent {
        market,
        decision,
        execution: KrakenPaperExecution::new(&pair, &volume),
        validation: ArtifactValidation::new("trades.log", log_entries, strategy_display_name),
        signer: SimpleSigner::new(signing_key),
        performance: PerformanceTracker::new(initial_balance),
        position: std::sync::Mutex::new(PositionState::default()),
        risk_config,
        agent_id,
        intent_amount,
    };

    let interval_secs: u64 = std::env::var("AGENT_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    loop {
        if let Err(e) = agent.run_once().await {
            eprintln!("tick error: {e:#}");
        }
        tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
    }
}
