use crate::domain::intent::TradeIntent;
use crate::domain::model::Decision;

pub fn build_intent(decision: &Decision, price: f64, agent_id: &str, amount: f64) -> TradeIntent {
    TradeIntent {
        agent_id: agent_id.to_string(),
        action: format!("{:?}", decision.action),
        amount,
        price,
        timestamp: chrono::Utc::now().timestamp(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::Action;

    #[test]
    fn builds_buy_intent() {
        let d = Decision {
            action: Action::Buy,
            confidence: 0.8,
            reasoning: "test".to_string(),
        };
        let intent = build_intent(&d, 100.0, "agent-1", 1.0);
        assert_eq!(intent.action, "Buy");
    }

    #[test]
    fn builds_sell_intent() {
        let d = Decision {
            action: Action::Sell,
            confidence: 0.7,
            reasoning: "test".to_string(),
        };
        let intent = build_intent(&d, 50.0, "agent-1", 2.0);
        assert_eq!(intent.action, "Sell");
    }

    #[test]
    fn intent_has_correct_fields() {
        let d = Decision {
            action: Action::Buy,
            confidence: 0.9,
            reasoning: "test".to_string(),
        };
        let intent = build_intent(&d, 42.5, "my-agent", 3.14);
        assert_eq!(intent.agent_id, "my-agent");
        assert!((intent.price - 42.5).abs() < f64::EPSILON);
        assert!((intent.amount - 3.14).abs() < f64::EPSILON);
    }

    #[test]
    fn timestamp_is_recent() {
        let d = Decision {
            action: Action::Hold,
            confidence: 0.5,
            reasoning: "test".to_string(),
        };
        let intent = build_intent(&d, 100.0, "a", 1.0);
        let now = chrono::Utc::now().timestamp();
        assert!((now - intent.timestamp).abs() < 5);
    }
}
