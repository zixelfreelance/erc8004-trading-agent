use std::collections::HashMap;
use std::sync::Arc;

use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use async_trait::async_trait;

use crate::domain::model::{Decision, MarketData};
use crate::ports::decision::DecisionPort;

const APP_NAME: &str = "trading-agent";

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

/// ADK-backed decision port: Claude via Anthropic, structured JSON decisions.
pub struct AdkDecision {
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
    session_id: SessionId,
}

impl AdkDecision {
    pub async fn new() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY is not set"))?;

        let model_name = std::env::var("ANTHROPIC_MODEL")
            .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string());

        let model = Arc::new(
            AnthropicClient::new(AnthropicConfig::new(api_key, model_name))
                .map_err(|e| anyhow::anyhow!("{e:#}"))?,
        );

        let instruction = r#"You are an autonomous trading agent.

Your goal:
- Maximize risk-adjusted return
- Minimize drawdown
- Avoid unnecessary trades

Constraints:
- If confidence < 0.6 → HOLD
- If market unclear → HOLD

Return ONLY JSON:

{
  "action": "Buy | Sell | Hold",
  "confidence": 0.0-1.0,
  "reasoning": "clear, concise explanation"
}"#;

        let agent = Arc::new(
            LlmAgentBuilder::new("trader")
                .instruction(instruction)
                .model(model)
                .build()
                .map_err(|e| anyhow::anyhow!("{e:#}"))?,
        );

        let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
        let session_id = SessionId::generate();

        sessions
            .create(CreateRequest {
                app_name: APP_NAME.into(),
                user_id: "trader".into(),
                session_id: Some(session_id.to_string()),
                state: HashMap::new(),
            })
            .await
            .map_err(|e| anyhow::anyhow!("{e:#}"))?;

        Ok(Self {
            agent,
            sessions,
            session_id,
        })
    }

    pub async fn decide_with_extra_context(
        &self,
        data: &MarketData,
        extra_context: &str,
    ) -> anyhow::Result<Decision> {
        let input = format!(
            "Market pair: {}\nMarket price: {}\n\n{}\n\nWhat should we do?",
            data.pair, data.price, extra_context
        );

        let runner = Runner::new(RunnerConfig {
            app_name: APP_NAME.into(),
            agent: self.agent.clone(),
            session_service: self.sessions.clone(),
            artifact_service: None,
            memory_service: None,
            plugin_manager: None,
            run_config: None,
            compaction_config: None,
            context_cache_config: None,
            cache_capable: None,
            request_context: None,
            cancellation_token: None,
        })
        .map_err(|e| anyhow::anyhow!("{e:#}"))?;

        let content = Content::new("user").with_text(input);
        let stream = runner
            .run(UserId::new("trader")?, self.session_id.clone(), content)
            .await
            .map_err(|e| anyhow::anyhow!("{e:#}"))?;

        let mut text = String::new();
        let mut s = stream;
        while let Some(event) = s.next().await {
            let event = event.map_err(|e| anyhow::anyhow!("{e:#}"))?;
            if let Some(content) = event.content() {
                for part in &content.parts {
                    if let Some(t) = part.text() {
                        text.push_str(t);
                    }
                }
            }
        }

        parse_decision_json(&text)
    }
}

#[async_trait]
impl DecisionPort for AdkDecision {
    async fn decide(&self, data: &MarketData) -> anyhow::Result<Decision> {
        self.decide_with_extra_context(data, "").await
    }
}
