use async_trait::async_trait;

use crate::domain::model::{Decision, MarketData};
use crate::ports::decision::DecisionPort;

pub struct ClaudeDecision;

fn extract_json_object(raw: &str) -> &str {
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

fn parse_decision_json(raw: &str) -> anyhow::Result<Decision> {
    let slice = extract_json_object(raw);
    serde_json::from_str(slice).map_err(|e| anyhow::anyhow!("decision JSON: {e}"))
}

async fn call_llm(prompt: String) -> anyhow::Result<String> {
    let _ = prompt;
    Ok(r#"{"action":"Hold","confidence":0.72,"reasoning":"Stub LLM: swap for adk_rust::run or LlmAgentBuilder"}"#.to_string())
}

#[async_trait]
impl DecisionPort for ClaudeDecision {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision> {
        let prompt = format!(
            r#"You are a trading agent.

Market pair: {}
Market price: {}

Return ONLY valid JSON (no markdown, no prose):
{{
  "action": "Buy" | "Sell" | "Hold",
  "confidence": <number 0.0-1.0>,
  "reasoning": "short explanation"
}}"#,
            data.pair, data.price
        );

        let response = call_llm(prompt).await?;
        parse_decision_json(&response)
    }
}
