use super::model::{Action, Decision};
use super::performance::Performance;

#[derive(Debug, Clone, Default)]
pub struct PositionState {
    pub open_long: bool,
}

#[derive(Debug, Clone)]
pub struct RiskConfig {
    pub min_confidence_trade: f64,
    pub max_drawdown: f64,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            min_confidence_trade: 0.6,
            max_drawdown: 0.05,
        }
    }
}

/// Returns `(final_decision, risk_overrode)` where `risk_overrode` is true if policy changed the action.
pub fn apply_risk(
    decision: Decision,
    position: &PositionState,
    perf: &Performance,
    cfg: &RiskConfig,
) -> (Decision, bool) {
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
