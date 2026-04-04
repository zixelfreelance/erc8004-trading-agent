//! Deterministic parsing of LLM output into [`Decision`](crate::domain::model::Decision).

use crate::domain::model::Decision;

pub fn extract_json_object(raw: &str) -> &str {
    let s = raw.trim();
    let Some(start) = s.find('{') else {
        return s;
    };
    let tail = &s[start..];
    let Some(end_rel) = tail.rfind('}') else {
        return tail;
    };
    &tail[..=end_rel]
}

pub fn parse_decision_json(raw: &str) -> anyhow::Result<Decision> {
    let slice = extract_json_object(raw);
    serde_json::from_str(slice).map_err(|e| anyhow::anyhow!("decision JSON: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::Action;

    #[test]
    fn parses_minimal_object() {
        let d =
            parse_decision_json(r#"{"action":"Buy","confidence":0.9,"reasoning":"ok"}"#).unwrap();
        assert_eq!(d.action, Action::Buy);
        assert!((d.confidence - 0.9).abs() < f64::EPSILON);
        assert_eq!(d.reasoning, "ok");
    }

    #[test]
    fn extracts_from_wrapped_text() {
        let raw = "Here you go:\n```\n{\"action\":\"Hold\",\"confidence\":0.5,\"reasoning\":\"wait\"}\n```";
        let d = parse_decision_json(raw).unwrap();
        assert_eq!(d.action, Action::Hold);
    }

    #[test]
    fn rejects_invalid_action() {
        assert!(
            parse_decision_json(r#"{"action":"Long","confidence":1.0,"reasoning":"x"}"#).is_err()
        );
    }
}
