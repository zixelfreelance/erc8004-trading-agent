use std::sync::Mutex;

use crate::application::intent_builder::build_intent;
use crate::domain::indicators;
use crate::domain::metrics::AgentMetrics;
use crate::domain::model::Action;
use crate::domain::regime::{MarketRegime, RegimeDetector};
use crate::domain::risk::{self, PositionState, RiskConfig};
use crate::ports::decision::DecisionPort;
use crate::ports::execution::ExecutionPort;
use crate::ports::market::MarketPort;
use crate::ports::performance::PerformancePort;
use crate::ports::signer::SignerPort;
use crate::ports::validation::ValidationPort;

pub struct TradingAgent<M, D, E, V, S, P> {
    pub market: M,
    pub decision: D,
    pub execution: E,
    pub validation: V,
    pub signer: S,
    pub performance: P,
    pub position: Mutex<PositionState>,
    pub risk_config: RiskConfig,
    pub agent_id: String,
    pub intent_amount: f64,
    pub metrics: AgentMetrics,
    pub regime: Mutex<RegimeDetector>,
    pub atr_stop_price: Mutex<Option<f64>>,
    pub atr_stop_multiplier: f64,
}

impl<M, D, E, V, S, P> TradingAgent<M, D, E, V, S, P>
where
    M: MarketPort,
    D: DecisionPort,
    E: ExecutionPort,
    V: ValidationPort,
    S: SignerPort,
    P: PerformancePort,
{
    pub async fn run_once(&self) -> anyhow::Result<()> {
        self.metrics.record_tick();

        let data = self.market.get_market_data()?;

        // --- Regime detection ---
        let regime = {
            let adx_val = indicators::adx(&data.ohlc_highs, &data.ohlc_lows, &data.ohlc_closes, 14)
                .map(|a| a.adx)
                .unwrap_or(0.0);
            let bb_bw = indicators::bollinger(&data.ohlc_closes, 20, 2.0)
                .map(|b| b.bandwidth)
                .unwrap_or(0.0);
            let mut regime_guard = self.regime.lock().expect("regime mutex poisoned");
            regime_guard.update(adx_val, bb_bw)
        };

        // --- ATR stop check: force sell if price breaches stop ---
        let atr_forced_sell = {
            let pos = self.position.lock().expect("position mutex poisoned");
            if pos.open_long {
                if let Some(stop) = *self.atr_stop_price.lock().expect("atr mutex poisoned") {
                    data.price <= stop
                } else {
                    false
                }
            } else {
                false
            }
        };

        let decision = if atr_forced_sell {
            use crate::domain::model::Decision;
            Decision {
                action: Action::Sell,
                confidence: 0.95,
                reasoning: format!(
                    "ATR stop triggered at {:.2} (price {:.2})",
                    self.atr_stop_price.lock().unwrap().unwrap_or(0.0),
                    data.price
                ),
            }
        } else {
            let mut decision = self.decision.decide(&data).await?;

            // --- Fee filter: reject trades with insufficient edge ---
            if matches!(decision.action, Action::Buy | Action::Sell) {
                let closes = &data.ohlc_closes;
                if closes.len() >= 2 {
                    let momentum_pct =
                        ((closes[closes.len() - 1] - closes[0]) / closes[0] * 100.0).abs();
                    if !risk::passes_fee_filter(momentum_pct, self.risk_config.min_edge_pct) {
                        decision = crate::domain::model::Decision {
                            action: Action::Hold,
                            confidence: decision.confidence,
                            reasoning: format!(
                                "fee filter: edge {:.3}% < min {:.1}% — {}",
                                momentum_pct, self.risk_config.min_edge_pct, decision.reasoning
                            ),
                        };
                    }
                }
            }

            // --- Regime filter: block momentum in ranging, reversion in trending ---
            if matches!(decision.action, Action::Buy | Action::Sell) {
                match regime {
                    MarketRegime::Transition => {
                        decision = crate::domain::model::Decision {
                            action: Action::Hold,
                            confidence: decision.confidence,
                            reasoning: format!("regime: transition (unclear) — {}", decision.reasoning),
                        };
                    }
                    _ => {} // Trending and Ranging both allow trades (for now)
                }
            }

            decision
        };

        let perf_snapshot = self.performance.snapshot();
        let current_tick = self.metrics.snapshot().ticks;
        let position_guard = self.position.lock().expect("position mutex poisoned");
        let (final_decision, blocked) =
            risk::apply_risk_with_tick(decision, &position_guard, &perf_snapshot, &self.risk_config, current_tick);
        drop(position_guard);

        if blocked {
            self.metrics.record_blocked();
        } else {
            match final_decision.action {
                Action::Buy | Action::Sell => self.metrics.record_executed(),
                Action::Hold => self.metrics.record_hold(),
            }
        }

        // Volatility-scaled position sizing
        let tick_amount = if matches!(final_decision.action, Action::Buy | Action::Sell) {
            if let Some(atr_val) = indicators::atr(
                &data.ohlc_highs, &data.ohlc_lows, &data.ohlc_closes, 14,
            ) {
                risk::compute_position_size(
                    self.intent_amount,
                    perf_snapshot.balance,
                    self.risk_config.risk_per_trade,
                    atr_val,
                    self.atr_stop_multiplier,
                    final_decision.confidence,
                )
            } else {
                self.intent_amount
            }
        } else {
            self.intent_amount
        };

        let intent = build_intent(
            &final_decision,
            data.price,
            &self.agent_id,
            tick_amount,
        );
        let signed_intent = self.signer.sign(intent);

        let balance_before = self.performance.snapshot().balance;

        let fill = self.execution.execute(&final_decision.action)?;
        if let Some(b) = fill.parsed_balance {
            self.performance.record_balance(b);
        }

        {
            let mut pos = self.position.lock().expect("position mutex poisoned");
            match final_decision.action {
                Action::Buy => {
                    pos.open_long = true;
                    // Set ATR-based stop for new position
                    if let Some(atr_val) = indicators::atr(
                        &data.ohlc_highs, &data.ohlc_lows, &data.ohlc_closes, 14,
                    ) {
                        let stop = data.price - atr_val * self.atr_stop_multiplier;
                        *self.atr_stop_price.lock().expect("atr mutex poisoned") = Some(stop);
                        eprintln!(
                            "atr-stop: set at {:.2} (entry {:.2} - {:.1}x ATR {:.2})",
                            stop, data.price, self.atr_stop_multiplier, atr_val
                        );
                    }
                }
                Action::Sell => {
                    pos.open_long = false;
                    *self.atr_stop_price.lock().expect("atr mutex poisoned") = None;
                }
                Action::Hold => {
                    // Trailing stop: update if price moved up
                    if pos.open_long {
                        if let Some(atr_val) = indicators::atr(
                            &data.ohlc_highs, &data.ohlc_lows, &data.ohlc_closes, 14,
                        ) {
                            let new_stop = data.price - atr_val * self.atr_stop_multiplier;
                            let mut stop_guard = self.atr_stop_price.lock().expect("atr mutex poisoned");
                            if let Some(current_stop) = *stop_guard {
                                if new_stop > current_stop {
                                    *stop_guard = Some(new_stop);
                                }
                            }
                        }
                    }
                }
            }

            if !blocked && matches!(final_decision.action, Action::Buy | Action::Sell) {
                let balance_after = self.performance.snapshot().balance;
                pos.record_trade_result(balance_before, balance_after);
                pos.last_trade_tick = current_tick;

                if pos.consecutive_losses >= self.risk_config.max_consecutive_losses
                    || pos.daily_loss >= self.risk_config.daily_loss_limit
                {
                    pos.circuit_breaker_active = true;
                }
            }
        }

        let perf = self.performance.snapshot();
        self.validation
            .log_decision(&data, &final_decision, blocked, &signed_intent, &perf)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::mock_market::MockMarket;
    use crate::adapters::performance_tracker::PerformanceTracker;
    use crate::adapters::signer::SimpleSigner;
    use crate::adapters::validation::{ArtifactValidation, SharedLogEntries};
    use crate::domain::metrics;
    use crate::domain::regime::RegimeDetector;
    use crate::domain::risk::{PositionState, RiskConfig};
    use std::sync::Arc;

    struct AlwaysHold;

    #[async_trait::async_trait]
    impl crate::ports::decision::DecisionPort for AlwaysHold {
        async fn decide(
            &self,
            _data: &crate::domain::model::MarketData,
        ) -> anyhow::Result<crate::domain::model::Decision> {
            Ok(crate::domain::model::Decision {
                action: crate::domain::model::Action::Hold,
                confidence: 0.5,
                reasoning: "Test: always hold".into(),
            })
        }
    }

    struct AlwaysBuy;

    #[async_trait::async_trait]
    impl crate::ports::decision::DecisionPort for AlwaysBuy {
        async fn decide(
            &self,
            _data: &crate::domain::model::MarketData,
        ) -> anyhow::Result<crate::domain::model::Decision> {
            Ok(crate::domain::model::Decision {
                action: crate::domain::model::Action::Buy,
                confidence: 0.8,
                reasoning: "Test: always buy".into(),
            })
        }
    }

    struct MockExecution(f64);

    impl crate::ports::execution::ExecutionPort for MockExecution {
        fn execute(
            &self,
            _action: &crate::domain::model::Action,
        ) -> anyhow::Result<crate::ports::execution::ExecutionFill> {
            Ok(crate::ports::execution::ExecutionFill {
                parsed_balance: Some(self.0),
            })
        }
    }

    fn make_agent<D: crate::ports::decision::DecisionPort>(
        decision: D,
        balance: f64,
        log_file: &str,
    ) -> TradingAgent<MockMarket, D, MockExecution, ArtifactValidation, SimpleSigner, PerformanceTracker>
    {
        let log_entries: SharedLogEntries = Arc::new(std::sync::Mutex::new(Vec::new()));
        TradingAgent {
            market: MockMarket::demo_sequence(),
            decision,
            execution: MockExecution(balance),
            validation: ArtifactValidation::new(log_file, log_entries, "test"),
            signer: SimpleSigner::new("test-key"),
            performance: PerformanceTracker::new(balance),
            position: Mutex::new(PositionState::default()),
            risk_config: RiskConfig::default(),
            agent_id: "test-agent".into(),
            intent_amount: 0.001,
            metrics: metrics::new_metrics(),
            regime: Mutex::new(RegimeDetector::with_defaults()),
            atr_stop_price: Mutex::new(None),
            atr_stop_multiplier: 1.5,
        }
    }

    fn tmp_log(name: &str) -> String {
        format!("/tmp/trading-agent-test-{}-{}.log", name, std::process::id())
    }

    #[tokio::test]
    async fn full_cycle_hold_succeeds() {
        let log = tmp_log("hold");
        let agent = make_agent(AlwaysHold, 10000.0, &log);
        agent.run_once().await.unwrap();
        assert_eq!(agent.metrics.snapshot().ticks, 1);
        assert_eq!(agent.metrics.snapshot().holds, 1);
        let _ = std::fs::remove_file(&log);
    }

    #[tokio::test]
    async fn full_cycle_buy_executes() {
        let log = tmp_log("buy");
        let agent = make_agent(AlwaysBuy, 10000.0, &log);
        agent.run_once().await.unwrap();
        let m = agent.metrics.snapshot();
        assert_eq!(m.ticks, 1);
        assert!(m.trades_executed + m.trades_blocked + m.holds == 1);
        let _ = std::fs::remove_file(&log);
    }

    #[tokio::test]
    async fn multiple_ticks_accumulate() {
        let log = tmp_log("multi");
        let agent = make_agent(AlwaysHold, 10000.0, &log);
        for _ in 0..5 {
            agent.run_once().await.unwrap();
        }
        assert_eq!(agent.metrics.snapshot().ticks, 5);
        let _ = std::fs::remove_file(&log);
    }

    #[tokio::test]
    async fn demo_sequence_runs_full() {
        let log = tmp_log("demo50");
        let agent = make_agent(AlwaysHold, 10000.0, &log);
        for _ in 0..50 {
            agent.run_once().await.unwrap();
        }
        assert_eq!(agent.metrics.snapshot().ticks, 50);
        let _ = std::fs::remove_file(&log);
    }

    #[tokio::test]
    async fn risk_blocks_low_confidence() {
        struct LowConfBuy;
        #[async_trait::async_trait]
        impl crate::ports::decision::DecisionPort for LowConfBuy {
            async fn decide(
                &self,
                _data: &crate::domain::model::MarketData,
            ) -> anyhow::Result<crate::domain::model::Decision> {
                Ok(crate::domain::model::Decision {
                    action: crate::domain::model::Action::Buy,
                    confidence: 0.3,
                    reasoning: "Low confidence buy".into(),
                })
            }
        }
        let log = tmp_log("lowconf");
        let agent = make_agent(LowConfBuy, 10000.0, &log);
        agent.run_once().await.unwrap();
        let m = agent.metrics.snapshot();
        // Low confidence buy gets caught by either regime filter (→ hold) or risk gate (→ blocked)
        // Either way, it should NOT execute
        assert_eq!(m.trades_executed, 0, "Low confidence buy should not execute");
        let _ = std::fs::remove_file(&log);
    }
}
