use crate::domain::intent::TradeIntent;
use crate::domain::model::Decision;

pub fn build_intent(
    decision: &Decision,
    price: f64,
    agent_id: &str,
    amount: f64,
    pair: &str,
    agent_wallet: &str,
) -> TradeIntent {
    let now = chrono::Utc::now().timestamp();
    TradeIntent {
        agent_id: agent_id.to_string(),
        action: format!("{:?}", decision.action),
        amount,
        price,
        timestamp: now,
        pair: pair.to_string(),
        agent_wallet: agent_wallet.to_string(),
        amount_usd_scaled: (amount * price * 1e5) as u128,
        max_slippage_bps: 50,
        nonce: now as u64,
        deadline: (now + 300) as u64,
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
        let intent = build_intent(&d, 100.0, "agent-1", 1.0, "BTCUSD", "0xdead");
        assert_eq!(intent.action, "Buy");
        assert_eq!(intent.pair, "BTCUSD");
        assert_eq!(intent.agent_wallet, "0xdead");
    }

    #[test]
    fn builds_sell_intent() {
        let d = Decision {
            action: Action::Sell,
            confidence: 0.7,
            reasoning: "test".to_string(),
        };
        let intent = build_intent(&d, 50.0, "agent-1", 2.0, "BTCUSD", "0xbeef");
        assert_eq!(intent.action, "Sell");
    }

    #[test]
    fn intent_has_correct_fields() {
        let d = Decision {
            action: Action::Buy,
            confidence: 0.9,
            reasoning: "test".to_string(),
        };
        let intent = build_intent(&d, 42.5, "my-agent", 3.14, "ETHUSD", "0x1234");
        assert_eq!(intent.agent_id, "my-agent");
        assert!((intent.price - 42.5).abs() < f64::EPSILON);
        assert!((intent.amount - 3.14).abs() < f64::EPSILON);
        assert_eq!(intent.pair, "ETHUSD");
    }

    #[test]
    fn timestamp_is_recent() {
        let d = Decision {
            action: Action::Hold,
            confidence: 0.5,
            reasoning: "test".to_string(),
        };
        let intent = build_intent(&d, 100.0, "a", 1.0, "BTCUSD", "0x0");
        let now = chrono::Utc::now().timestamp();
        assert!((now - intent.timestamp).abs() < 5);
    }

    #[test]
    fn nonce_and_deadline_set() {
        let d = Decision {
            action: Action::Buy,
            confidence: 0.9,
            reasoning: "test".to_string(),
        };
        let intent = build_intent(&d, 100.0, "a", 1.0, "BTCUSD", "0x0");
        assert!(intent.nonce > 0);
        assert_eq!(intent.deadline, intent.nonce + 300);
    }

    #[test]
    fn amount_usd_scaled_correct() {
        let d = Decision {
            action: Action::Buy,
            confidence: 0.9,
            reasoning: "test".to_string(),
        };
        // 0.001 BTC at $50000 = $50 → scaled by 1e5 = 5_000_000
        let intent = build_intent(&d, 50_000.0, "a", 0.001, "BTCUSD", "0x0");
        assert_eq!(intent.amount_usd_scaled, 5_000_000);
    }
}
