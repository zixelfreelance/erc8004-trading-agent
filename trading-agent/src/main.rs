mod adapters;
mod application;
mod domain;
mod ports;

use std::net::SocketAddr;
use std::sync::Arc;

use adapters::chain_identity::ChainIdentityAdapter;
use adapters::chain_reputation::ChainReputationAdapter;
use adapters::decision_driver::DecisionDriver;
use adapters::http_logs;
use adapters::kraken_execution::{ExecutionMode, KrakenExecution};
use adapters::kraken_market::KrakenMarket;
use adapters::momentum_decision::MomentumVolatilityDecision;
use adapters::performance_tracker::PerformanceTracker;
use adapters::signer::{Eip712Signer, SignerDriver, SimpleSigner};
use adapters::validation::{ArtifactValidation, SharedLogEntries};
use application::agent::TradingAgent;
use domain::metrics;
use domain::regime::RegimeDetector;
use domain::risk::{PositionState, RiskConfig};
use domain::strategy::{StrategyConfig, STRATEGY_DISPLAY_NAME};
use ports::identity::IdentityPort;
use ports::performance::PerformancePort;
use ports::reputation::{ReputationMetric, ReputationPort};

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

    let exec_mode = ExecutionMode::from_env();

    let ohlc_interval: u32 = std::env::var("AGENT_OHLC_INTERVAL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    let ohlc_lookback: usize = std::env::var("AGENT_MOMENTUM_LOOKBACK")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(50);

    let strategy_cfg = StrategyConfig {
        momentum_threshold: std::env::var("AGENT_STRATEGY_MOMENTUM_THRESHOLD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(50.0),
        volatility_min: std::env::var("AGENT_STRATEGY_VOL_MIN")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5.0),
        volatility_max: std::env::var("AGENT_STRATEGY_VOL_MAX")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(500.0),
        rsi_oversold: std::env::var("AGENT_RSI_OVERSOLD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30.0),
        rsi_overbought: std::env::var("AGENT_RSI_OVERBOUGHT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(70.0),
        bollinger_period: std::env::var("AGENT_BOLLINGER_PERIOD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(20),
        bollinger_std: std::env::var("AGENT_BOLLINGER_STD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2.0),
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
        max_consecutive_losses: std::env::var("AGENT_MAX_CONSECUTIVE_LOSSES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3),
        daily_loss_limit: std::env::var("AGENT_DAILY_LOSS_LIMIT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5.0),
        min_edge_pct: std::env::var("AGENT_MIN_EDGE_PCT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.7),
    };

    let strategy_display_name =
        std::env::var("AGENT_STRATEGY_LABEL").unwrap_or_else(|_| match decision_mode.as_str() {
            "adk" => "LLM-assisted decision (risk gates + signed intents)".to_string(),
            "hybrid" => "Hybrid: rule-based signal + ADK final decision".to_string(),
            _ => STRATEGY_DISPLAY_NAME.to_string(),
        });

    let decision: DecisionDriver = match decision_mode.as_str() {
        "adk" => DecisionDriver::Adk(Arc::new(
            adapters::adk_decision::AdkDecision::new(risk_config.clone()).await?,
        )),
        "hybrid" => DecisionDriver::Hybrid(Arc::new(
            adapters::hybrid_decision::HybridAdkDecision::new(
                strategy_cfg.clone(),
                risk_config.clone(),
            )
            .await?,
        )),
        _ => DecisionDriver::Momentum(MomentumVolatilityDecision::new(strategy_cfg)),
    };

    let log_entries: SharedLogEntries = Arc::new(std::sync::Mutex::new(Vec::new()));
    let log_entries_srv = Arc::clone(&log_entries);
    let agent_metrics = metrics::new_metrics();
    let metrics_srv = Arc::clone(&agent_metrics);

    let app = http_logs::router(log_entries_srv, metrics_srv);
    let addr = SocketAddr::from(([0, 0, 0, 0], http_port));
    tokio::spawn(async move {
        if let Err(e) = http_logs::serve(app, addr).await {
            eprintln!("http server error: {e:#}");
        }
    });

    // Chain adapters (noop if env vars not set)
    let identity_registry = std::env::var("IDENTITY_REGISTRY").unwrap_or_default();
    let reputation_registry = std::env::var("REPUTATION_REGISTRY").unwrap_or_default();
    let chain_rpc_url = std::env::var("CHAIN_RPC_URL").unwrap_or_default();

    let identity_adapter = ChainIdentityAdapter::new(
        identity_registry.clone(),
        chain_rpc_url.clone(),
    );
    let reputation_adapter = ChainReputationAdapter::new(
        reputation_registry,
        chain_rpc_url,
    );

    // Register agent identity on-chain if configured
    if identity_adapter.is_configured() {
        let agent_uri = std::env::var("AGENT_CARD_URI").unwrap_or_default();
        match identity_adapter.register_agent(&agent_uri).await {
            Ok(id) => eprintln!("chain: agent registered with id={id}"),
            Err(e) => eprintln!("chain: identity registration failed: {e:#}"),
        }
    }

    let mode_label = match exec_mode {
        ExecutionMode::Paper => "paper",
        ExecutionMode::Live => "LIVE",
    };
    let chain_status = if identity_adapter.is_configured() { "configured" } else { "not configured" };
    eprintln!("mode: {mode_label} | pair: {pair} | volume: {volume} | chain: {chain_status}");
    eprintln!("audit: GET http://{addr}/logs  GET http://{addr}/metrics");

    let mut market = KrakenMarket::new(&pair);
    market.ohlc_interval_minutes = ohlc_interval;
    market.ohlc_lookback = ohlc_lookback;

    let chain_id: u64 = std::env::var("CHAIN_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(11155111);

    let signer: SignerDriver = if signing_key.starts_with("0x")
        || (signing_key.len() == 64 && signing_key.chars().all(|c| c.is_ascii_hexdigit()))
    {
        SignerDriver::Eip712(Eip712Signer::new(&signing_key, chain_id)?)
    } else {
        SignerDriver::Simple(SimpleSigner::new(signing_key))
    };

    let atr_stop_multiplier: f64 = std::env::var("AGENT_ATR_STOP_MULTIPLIER")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.5);

    let agent = TradingAgent {
        market,
        decision,
        execution: KrakenExecution::new(&pair, &volume, exec_mode),
        validation: ArtifactValidation::new("trades.log", log_entries, strategy_display_name),
        signer,
        performance: PerformanceTracker::new(initial_balance),
        position: std::sync::Mutex::new(PositionState::default()),
        risk_config,
        agent_id,
        intent_amount,
        metrics: agent_metrics,
        regime: std::sync::Mutex::new(RegimeDetector::with_defaults()),
        atr_stop_price: std::sync::Mutex::new(None),
        atr_stop_multiplier,
    };

    let interval_secs: u64 = std::env::var("AGENT_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let reputation_interval: u64 = std::env::var("AGENT_REPUTATION_INTERVAL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    loop {
        if let Err(e) = agent.run_once().await {
            agent.metrics.record_error();
            eprintln!("tick error: {e:#}");
        }

        // Post reputation on-chain every N ticks
        let tick_count = agent.metrics.snapshot().ticks;
        if reputation_adapter.is_configured()
            && tick_count > 0
            && tick_count % reputation_interval == 0
        {
            let perf = agent.performance.snapshot();
            let metric = ReputationMetric {
                tag1: "performance".into(),
                tag2: "pnl".into(),
                value: (perf.pnl * 100.0) as i128,
                decimals: 2,
            };
            match reputation_adapter.post_feedback(0, &metric, "").await {
                Ok(tx) => eprintln!("chain: reputation posted (tx={tx}, pnl={:.2})", perf.pnl),
                Err(e) => eprintln!("chain: reputation post failed: {e:#}"),
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
    }
}
