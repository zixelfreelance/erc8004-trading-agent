use super::model::{Action, Decision};

/// Returns `(final_decision, risk_overrode)` where `risk_overrode` is true if policy changed the action.
pub fn apply_risk(decision: Decision) -> (Decision, bool) {
    if decision.confidence < 0.6 && decision.action != Action::Hold {
        return (
            Decision {
                action: Action::Hold,
                confidence: decision.confidence,
                reason: format!("risk: low confidence ({:.2}) — {}", decision.confidence, decision.reason),
            },
            true,
        );
    }
    (decision, false)
}
