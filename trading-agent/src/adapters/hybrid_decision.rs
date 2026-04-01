use async_trait::async_trait;

use crate::domain::indicators;
use crate::domain::model::{Decision, MarketData};
use crate::domain::regime;
use crate::domain::risk::RiskConfig;
use crate::domain::strategy::{compute_regime_aware_decision, StrategyConfig};
use crate::ports::decision::DecisionPort;

use super::adk_decision::{format_recent_trades, AdkDecision};
use super::validation::SharedLogEntries;

/// Runs regime-aware deterministic strategy first, then ADK with that signal + indicators as prior.
pub struct HybridAdkDecision {
    adk: AdkDecision,
    strategy: StrategyConfig,
    log_entries: Option<SharedLogEntries>,
}

impl HybridAdkDecision {
    pub async fn new(strategy: StrategyConfig, risk_limits: RiskConfig) -> anyhow::Result<Self> {
        Ok(Self {
            adk: AdkDecision::new(risk_limits).await?,
            strategy,
            log_entries: None,
        })
    }

    pub fn with_log_entries(mut self, entries: SharedLogEntries) -> Self {
        self.adk.log_entries = Some(entries.clone());
        self.log_entries = Some(entries);
        self
    }
}

#[async_trait]
impl DecisionPort for HybridAdkDecision {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision> {
        let detected_regime = regime::detect_regime(
            &data.ohlc_highs,
            &data.ohlc_lows,
            &data.ohlc_closes,
            &regime::RegimeConfig::default(),
        );

        let prior = compute_regime_aware_decision(data, &self.strategy, detected_regime);
        let action_s = match prior.action {
            crate::domain::model::Action::Buy => "Buy",
            crate::domain::model::Action::Sell => "Sell",
            crate::domain::model::Action::Hold => "Hold",
        };

        let rsi = indicators::rsi(&data.ohlc_closes, 14)
            .map(|v| format!("{:.0}", v))
            .unwrap_or_else(|| "N/A".into());
        let macd_hist = indicators::macd(&data.ohlc_closes, 12, 26, 9)
            .map(|m| format!("{:.2}", m.histogram))
            .unwrap_or_else(|| "N/A".into());
        let atr = indicators::atr(&data.ohlc_highs, &data.ohlc_lows, &data.ohlc_closes, 14)
            .map(|v| format!("{:.2}", v))
            .unwrap_or_else(|| "N/A".into());
        let spread = data
            .spread
            .map(|s| format!("{:.2}", s))
            .unwrap_or_else(|| "N/A".into());

        let mut extra = format!(
            r#"Deterministic strategy signal:
- action: {action_s}
- confidence: {:.4}
- reasoning: {}
- regime: {detected_regime}

Indicator snapshot:
- RSI(14): {rsi}
- MACD histogram: {macd_hist}
- ATR(14): {atr}
- Spread: {spread}

Treat the strategy signal as a strong prior. Override only with clear justification referencing the indicators.

IMPORTANT: Do not default to Hold out of uncertainty. If both bull and bear cases have quantitative merit, commit to the side with stronger backing. A confident wrong trade (caught by risk gates) is more useful than endless Holds. Risk gates downstream protect capital — your job is conviction."#,
            prior.confidence, prior.reasoning
        );

        // Append recent trade history if available
        let history = self
            .log_entries
            .as_ref()
            .map(|e| format_recent_trades(e, 5))
            .unwrap_or_default();
        if !history.is_empty() {
            extra.push_str("\n\n");
            extra.push_str(&history);
        }

        self.adk.decide_with_extra_context(data, &extra).await
    }
}
