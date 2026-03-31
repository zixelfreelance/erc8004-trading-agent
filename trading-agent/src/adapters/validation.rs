use std::fs::OpenOptions;
use std::io::Write;

use crate::domain::model::{Decision, MarketData};
use crate::domain::signed_intent::SignedIntent;
use crate::ports::validation::ValidationPort;

pub struct ArtifactValidation {
    pub path: String,
}

impl ArtifactValidation {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl Default for ArtifactValidation {
    fn default() -> Self {
        Self::new("trades.log")
    }
}

impl ValidationPort for ArtifactValidation {
    fn log_decision(
        &self,
        data: &MarketData,
        decision: &Decision,
        blocked: bool,
        signed_intent: &SignedIntent,
    ) -> anyhow::Result<()> {
        let artifact = serde_json::json!({
            "type": "trade_execution",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "agent": signed_intent.intent.agent_id,
            "market": {
                "pair": data.pair,
                "price": data.price
            },
            "decision": decision,
            "intent": signed_intent.intent,
            "signature": signed_intent.signature,
            "risk": {
                "blocked": blocked
            }
        });

        println!("{}", serde_json::to_string_pretty(&artifact)?);

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)?;
        writeln!(file, "{}", serde_json::to_string(&artifact)?)?;
        Ok(())
    }
}
