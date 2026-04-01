use async_trait::async_trait;

use crate::domain::indicators;
use crate::domain::model::{Decision, MarketData};
use crate::domain::regime;
use crate::domain::risk::RiskConfig;
use crate::domain::strategy::{compute_regime_aware_decision, StrategyConfig};
use crate::ports::decision::DecisionPort;

use super::adk_decision::AdkDecision;

/// Runs regime-aware deterministic strategy first, then ADK with that signal + indicators as prior.
pub struct HybridAdkDecision {
    adk: AdkDecision,
    strategy: StrategyConfig,
}

impl HybridAdkDecision {
    pub async fn new(strategy: StrategyConfig, risk_limits: RiskConfig) -> anyhow::Result<Self> {
        Ok(Self {
            adk: AdkDecision::new(risk_limits).await?,
            strategy,
        })
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

        let extra = format!(
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

Treat the strategy signal as a strong prior. Override only with clear justification referencing the indicators."#,
            prior.confidence, prior.reasoning
        );
        self.adk.decide_with_extra_context(data, &extra).await
    }
}
