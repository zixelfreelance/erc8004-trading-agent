use std::collections::HashMap;
use std::sync::Arc;

use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use async_trait::async_trait;
use tokio::sync::RwLock;

use super::adk_signal_tools::signal_tools;
use super::validation::SharedLogEntries;
use crate::domain::decision_json::parse_decision_json;
use crate::domain::model::{Decision, MarketData};
use crate::domain::risk::RiskConfig;
use crate::ports::decision::DecisionPort;

const APP_NAME: &str = "trading-agent";

/// Format last N executed (non-Hold) trades as context for Claude.
pub fn format_recent_trades(entries: &SharedLogEntries, max: usize) -> String {
    let guard = entries.lock().expect("log mutex poisoned");
    let executed: Vec<_> = guard
        .iter()
        .filter(|r| r.action != "Hold" && !r.blocked_by_risk)
        .collect();
    if executed.is_empty() {
        return String::new();
    }
    let recent = &executed[executed.len().saturating_sub(max)..];
    let mut lines = String::from("Recent trade outcomes (learn from these):\n");
    for (i, r) in recent.iter().enumerate() {
        lines.push_str(&format!(
            "{}. {} @ {:.2}, conf={:.2}, pnl={:.2}, dd={:.2}%\n",
            i + 1,
            r.action,
            r.price,
            r.confidence,
            r.pnl,
            r.drawdown * 100.0,
        ));
    }
    lines.push_str("Avoid repeating losing patterns in the same regime.\n");
    lines
}

/// ADK-backed decision port: Claude via Anthropic, tool-grounded signals, structured JSON.
pub struct AdkDecision {
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
    session_id: SessionId,
    /// Current tick’s market snapshot for tool handlers (`compute_price_action_signals`).
    tick: Arc<RwLock<Option<MarketData>>>,
    /// Shared trade log for recent trade history context.
    pub log_entries: Option<SharedLogEntries>,
}

impl AdkDecision {
    /// `risk_limits` should be the same values as `TradingAgent::risk_config` so `risk_limits_snapshot` matches enforcement.
    pub async fn new(risk_limits: RiskConfig) -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY is not set"))?;

        let model_name = std::env::var("ANTHROPIC_MODEL")
            .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string());

        let model = Arc::new(
            AnthropicClient::new(AnthropicConfig::new(api_key, model_name))
                .map_err(|e| anyhow::anyhow!("{e:#}"))?,
        );

        let instruction = r#"You are the decision head of a paper-trading agent (Kraken CLI, BTC/USD).

Tools (call before your final answer):
- compute_price_action_signals — OHLC momentum, log-return vol, z vs mean, trend label.
- compute_technical_indicators — RSI(14), MACD(12,26,9), Bollinger(20,2), ATR(14), ADX(14), spread, VWAP, volume.
- risk_limits_snapshot — min confidence and max drawdown policy the runtime enforces.
- external_sentiment_stub — explicit "no feed" so you do not invent news.

Call compute_price_action_signals, compute_technical_indicators, and risk_limits_snapshot each turn.

Decision framework — ADVERSARIAL ANALYSIS:
Before deciding, construct both cases:
BULL CASE: What signals support buying? (momentum, RSI direction, MACD expansion, regime trending, volume)
BEAR CASE: What signals warn against? (spread widening, volume decline, ATR elevated, near resistance, fee cost ~0.52% round-trip, recent losses)
Then synthesize: which case is stronger and by how much? If both cases have quantitative merit, commit to the stronger one. Do not retreat to Hold unless signals genuinely contradict. The runtime enforces risk gates — your role is conviction.

You may receive:
- "Deterministic strategy signal" — Bayesian prior from the rule-based engine. Confirm or override with clear reasoning.
- "Recent trade outcomes" — last few trades. Avoid repeating losing patterns in the same regime.

Objectives (in order):
1) Preserve capital and keep drawdown small.
2) Take trades only when edge clearly exceeds fee cost (~0.52% round-trip).
3) Prefer risk-adjusted outcomes over activity.
4) Learn from recent trade outcomes — if recent trades in this regime lost money, be more cautious.

Hard rules:
- If not genuinely ≥0.6 confident in Buy or Sell, output Hold.
- Default to Hold when data is thin, contradictory, or you would trade for the sake of trading.
- Never output prose outside the JSON object.

Output exactly one JSON object:
{
  "action": "Buy",
  "confidence": 0.72,
  "reasoning": "Bull/bear synthesis: [thesis]. Prior: [agree/override because...]."
}

action: "Buy" | "Sell" | "Hold". confidence: 0.0-1.0. reasoning: 1-2 sentences max."#;

        let tick: Arc<RwLock<Option<MarketData>>> = Arc::new(RwLock::new(None));
        let risk_arc = Arc::new(risk_limits);
        let tools = signal_tools(Arc::clone(&tick), Arc::clone(&risk_arc));
        let mut builder = LlmAgentBuilder::new("trader")
            .instruction(instruction)
            .model(model);
        for t in tools {
            builder = builder.tool(t);
        }
        let agent = Arc::new(builder.build().map_err(|e| anyhow::anyhow!("{e:#}"))?);

        let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
        let session_id = SessionId::generate();

        sessions
            .create(CreateRequest {
                app_name: APP_NAME.into(),
                user_id: "trader".into(),
                session_id: Some(session_id.to_string()),
                state: HashMap::new(),
            })
            .await
            .map_err(|e| anyhow::anyhow!("{e:#}"))?;

        Ok(Self {
            agent,
            sessions,
            session_id,
            tick,
            log_entries: None,
        })
    }

    pub async fn decide_with_extra_context(
        &self,
        data: &MarketData,
        extra_context: &str,
    ) -> anyhow::Result<Decision> {
        *self.tick.write().await = Some(data.clone());

        let input = format!(
            "Market pair: {}\nLast price: {}\n\n{}\n\nUse tools as required, then respond with only the JSON object specified in your instructions.",
            data.pair, data.price, extra_context
        );

        let outcome = async {
            let runner = Runner::new(RunnerConfig {
                app_name: APP_NAME.into(),
                agent: self.agent.clone(),
                session_service: self.sessions.clone(),
                artifact_service: None,
                memory_service: None,
                plugin_manager: None,
                run_config: None,
                compaction_config: None,
                context_cache_config: None,
                cache_capable: None,
                request_context: None,
                cancellation_token: None,
            })
            .map_err(|e| anyhow::anyhow!("{e:#}"))?;

            let content = Content::new("user").with_text(input);
            let stream = runner
                .run(UserId::new("trader")?, self.session_id.clone(), content)
                .await
                .map_err(|e| anyhow::anyhow!("{e:#}"))?;

            let mut text = String::new();
            let mut s = stream;
            while let Some(event) = s.next().await {
                let event = event.map_err(|e| anyhow::anyhow!("{e:#}"))?;
                if let Some(content) = event.content() {
                    for part in &content.parts {
                        if let Some(t) = part.text() {
                            text.push_str(t);
                        }
                    }
                }
            }

            parse_decision_json(&text)
        }
        .await;

        *self.tick.write().await = None;

        outcome
    }
}

#[async_trait]
impl DecisionPort for AdkDecision {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision> {
        let history = self
            .log_entries
            .as_ref()
            .map(|e| format_recent_trades(e, 5))
            .unwrap_or_default();
        self.decide_with_extra_context(data, &history).await
    }
}
