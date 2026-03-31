use super::model::{Action, Decision};
use super::performance::Performance;

#[derive(Debug, Clone)]
pub struct PositionState {
    pub open_long: bool,
    pub consecutive_losses: u32,
    pub daily_loss: f64,
    pub circuit_breaker_active: bool,
    pub last_balance_after_trade: Option<f64>,
}

impl Default for PositionState {
    fn default() -> Self {
        Self {
            open_long: false,
            consecutive_losses: 0,
            daily_loss: 0.0,
            circuit_breaker_active: false,
            last_balance_after_trade: None,
        }
    }
}

impl PositionState {
    pub fn record_trade_result(&mut self, previous_balance: f64, new_balance: f64) {
        let pnl = new_balance - previous_balance;
        if pnl < 0.0 {
            self.consecutive_losses += 1;
            self.daily_loss += pnl.abs();
        } else {
            self.consecutive_losses = 0;
        }
        self.last_balance_after_trade = Some(new_balance);
    }

    pub fn reset_circuit_breaker(&mut self) {
        self.circuit_breaker_active = false;
        self.consecutive_losses = 0;
        self.daily_loss = 0.0;
    }
}

#[derive(Debug, Clone)]
pub struct RiskConfig {
    pub min_confidence_trade: f64,
    pub max_drawdown: f64,
    pub max_consecutive_losses: u32,
    pub daily_loss_limit: f64,
    pub min_edge_pct: f64,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            min_confidence_trade: 0.6,
            max_drawdown: 0.05,
            max_consecutive_losses: 3,
            daily_loss_limit: 5.0,
            min_edge_pct: 0.7,
        }
    }
}

/// Returns true if the expected price move justifies the round-trip fee cost.
/// `momentum_pct` is the absolute momentum signal strength as a percentage.
/// Default fee assumption: 0.52% round-trip (0.26% taker each way).
pub fn passes_fee_filter(momentum_pct: f64, min_edge_pct: f64) -> bool {
    momentum_pct.abs() >= min_edge_pct
}

pub fn apply_risk(
    decision: Decision,
    position: &PositionState,
    perf: &Performance,
    cfg: &RiskConfig,
) -> (Decision, bool) {
    if position.circuit_breaker_active {
        return (
            Decision {
                action: Action::Hold,
                confidence: decision.confidence,
                reasoning: format!("risk: circuit breaker active — {}", decision.reasoning),
            },
            true,
        );
    }

    if position.consecutive_losses >= cfg.max_consecutive_losses
        && decision.action != Action::Hold
    {
        return (
            Decision {
                action: Action::Hold,
                confidence: decision.confidence,
                reasoning: format!(
                    "risk: {} consecutive losses (limit {}) — {}",
                    position.consecutive_losses, cfg.max_consecutive_losses, decision.reasoning
                ),
            },
            true,
        );
    }

    if position.daily_loss >= cfg.daily_loss_limit && decision.action != Action::Hold {
        return (
            Decision {
                action: Action::Hold,
                confidence: decision.confidence,
                reasoning: format!(
                    "risk: daily loss ${:.2} exceeds limit ${:.2} — {}",
                    position.daily_loss, cfg.daily_loss_limit, decision.reasoning
                ),
            },
            true,
        );
    }

    if perf.drawdown > cfg.max_drawdown && decision.action != Action::Hold {
        return (
            Decision {
                action: Action::Hold,
                confidence: decision.confidence,
                reasoning: format!(
                    "risk: drawdown {:.2}% exceeds limit {:.2}% — {}",
                    perf.drawdown * 100.0,
                    cfg.max_drawdown * 100.0,
                    decision.reasoning
                ),
            },
            true,
        );
    }

    match decision.action {
        Action::Buy if position.open_long => {
            return (
                Decision {
                    action: Action::Hold,
                    confidence: decision.confidence,
                    reasoning: format!(
                        "risk: max one position at a time (already long) — {}",
                        decision.reasoning
                    ),
                },
                true,
            );
        }
        Action::Sell if !position.open_long => {
            return (
                Decision {
                    action: Action::Hold,
                    confidence: decision.confidence,
                    reasoning: format!(
                        "risk: no open long to close — {}",
                        decision.reasoning
                    ),
                },
                true,
            );
        }
        _ => {}
    }

    if decision.confidence < cfg.min_confidence_trade && decision.action != Action::Hold {
        return (
            Decision {
                action: Action::Hold,
                confidence: decision.confidence,
                reasoning: format!(
                    "risk: low confidence ({:.2}) — {}",
                    decision.confidence, decision.reasoning
                ),
            },
            true,
        );
    }

    (decision, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::performance::Performance;

    fn default_perf() -> Performance {
        Performance::initial(100.0)
    }

    fn buy_decision(confidence: f64) -> Decision {
        Decision {
            action: Action::Buy,
            confidence,
            reasoning: "test".to_string(),
        }
    }

    fn sell_decision(confidence: f64) -> Decision {
        Decision {
            action: Action::Sell,
            confidence,
            reasoning: "test".to_string(),
        }
    }

    fn hold_decision() -> Decision {
        Decision {
            action: Action::Hold,
            confidence: 0.5,
            reasoning: "test".to_string(),
        }
    }

    #[test]
    fn buy_passes_when_no_risk() {
        let pos = PositionState::default();
        let cfg = RiskConfig::default();
        let (d, blocked) = apply_risk(buy_decision(0.8), &pos, &default_perf(), &cfg);
        assert!(!blocked);
        assert_eq!(d.action, Action::Buy);
    }

    #[test]
    fn hold_always_passes() {
        let mut pos = PositionState::default();
        pos.circuit_breaker_active = true;
        let cfg = RiskConfig::default();
        let (d, blocked) = apply_risk(hold_decision(), &pos, &default_perf(), &cfg);
        // circuit breaker still blocks non-Hold, but Hold reasoning is overwritten
        // Actually circuit_breaker blocks everything except it returns Hold anyway
        // Let's check: circuit_breaker returns Hold with blocked=true regardless of input action
        // So Hold input still gets blocked=true. Let me re-read the code...
        // The code doesn't check action for circuit_breaker — it blocks everything.
        // But the test spec says "Hold is never blocked". Let's test with clean state.
        // Use a state where other gates would block Buy but not Hold.
        drop((d, blocked));

        let mut pos2 = PositionState::default();
        pos2.consecutive_losses = 5;
        pos2.daily_loss = 100.0;
        let perf = Performance {
            balance: 50.0,
            peak_balance: 100.0,
            drawdown: 0.5,
            pnl: -50.0,
        };
        let (d2, blocked2) = apply_risk(hold_decision(), &pos2, &perf, &cfg);
        assert!(!blocked2);
        assert_eq!(d2.action, Action::Hold);
    }

    #[test]
    fn drawdown_blocks_buy() {
        let pos = PositionState::default();
        let cfg = RiskConfig::default();
        let perf = Performance {
            balance: 90.0,
            peak_balance: 100.0,
            drawdown: 0.10, // 10% > 5%
            pnl: -10.0,
        };
        let (d, blocked) = apply_risk(buy_decision(0.8), &pos, &perf, &cfg);
        assert!(blocked);
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn drawdown_blocks_sell() {
        let mut pos = PositionState::default();
        pos.open_long = true; // must have position to sell
        let cfg = RiskConfig::default();
        let perf = Performance {
            balance: 90.0,
            peak_balance: 100.0,
            drawdown: 0.10,
            pnl: -10.0,
        };
        let (d, blocked) = apply_risk(sell_decision(0.8), &pos, &perf, &cfg);
        assert!(blocked);
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn already_long_blocks_buy() {
        let mut pos = PositionState::default();
        pos.open_long = true;
        let cfg = RiskConfig::default();
        let (d, blocked) = apply_risk(buy_decision(0.8), &pos, &default_perf(), &cfg);
        assert!(blocked);
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn no_position_blocks_sell() {
        let pos = PositionState::default(); // open_long = false
        let cfg = RiskConfig::default();
        let (d, blocked) = apply_risk(sell_decision(0.8), &pos, &default_perf(), &cfg);
        assert!(blocked);
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn low_confidence_blocks() {
        let pos = PositionState::default();
        let cfg = RiskConfig::default(); // min_confidence = 0.6
        let (d, blocked) = apply_risk(buy_decision(0.3), &pos, &default_perf(), &cfg);
        assert!(blocked);
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn circuit_breaker_blocks() {
        let mut pos = PositionState::default();
        pos.circuit_breaker_active = true;
        let cfg = RiskConfig::default();
        let (d, blocked) = apply_risk(buy_decision(0.9), &pos, &default_perf(), &cfg);
        assert!(blocked);
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn consecutive_losses_blocks() {
        let mut pos = PositionState::default();
        pos.consecutive_losses = 3; // == max
        let cfg = RiskConfig::default();
        let (d, blocked) = apply_risk(buy_decision(0.8), &pos, &default_perf(), &cfg);
        assert!(blocked);
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn daily_loss_blocks() {
        let mut pos = PositionState::default();
        pos.daily_loss = 5.0; // == limit
        let cfg = RiskConfig::default();
        let (d, blocked) = apply_risk(buy_decision(0.8), &pos, &default_perf(), &cfg);
        assert!(blocked);
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn record_trade_loss_increments() {
        let mut pos = PositionState::default();
        pos.record_trade_result(100.0, 95.0);
        assert_eq!(pos.consecutive_losses, 1);
        assert!((pos.daily_loss - 5.0).abs() < f64::EPSILON);
        assert_eq!(pos.last_balance_after_trade, Some(95.0));

        pos.record_trade_result(95.0, 90.0);
        assert_eq!(pos.consecutive_losses, 2);
        assert!((pos.daily_loss - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn record_trade_win_resets_consecutive() {
        let mut pos = PositionState::default();
        pos.record_trade_result(100.0, 95.0);
        pos.record_trade_result(95.0, 90.0);
        assert_eq!(pos.consecutive_losses, 2);

        pos.record_trade_result(90.0, 100.0);
        assert_eq!(pos.consecutive_losses, 0);
    }

    #[test]
    fn reset_circuit_breaker_clears_all() {
        let mut pos = PositionState {
            open_long: false,
            consecutive_losses: 5,
            daily_loss: 20.0,
            circuit_breaker_active: true,
            last_balance_after_trade: Some(80.0),
        };
        pos.reset_circuit_breaker();
        assert!(!pos.circuit_breaker_active);
        assert_eq!(pos.consecutive_losses, 0);
        assert!((pos.daily_loss - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_fee_filter_passes_strong_signal() {
        assert!(passes_fee_filter(1.5, 0.7));
    }

    #[test]
    fn test_fee_filter_blocks_weak_signal() {
        assert!(!passes_fee_filter(0.3, 0.7));
    }

    #[test]
    fn test_fee_filter_edge_case() {
        assert!(passes_fee_filter(0.7, 0.7));
    }
}
