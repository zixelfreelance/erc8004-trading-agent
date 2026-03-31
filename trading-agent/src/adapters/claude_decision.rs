use async_trait::async_trait;

use crate::domain::decision_json::parse_decision_json;
use crate::domain::model::{Decision, MarketData};
use crate::ports::decision::DecisionPort;

pub struct ClaudeDecision;

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
