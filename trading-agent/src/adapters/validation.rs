use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::domain::log_record::TradeLogRecord;
use crate::domain::model::{Decision, MarketData};
use crate::domain::performance::Performance;
use crate::domain::signed_intent::SignedIntent;
use crate::ports::validation::ValidationPort;

pub type SharedLogEntries = Arc<Mutex<Vec<TradeLogRecord>>>;

pub struct ArtifactValidation {
    pub path: String,
    pub entries: SharedLogEntries,
    pub strategy_display_name: String,
}

impl ArtifactValidation {
    pub fn new(
        path: impl Into<String>,
        entries: SharedLogEntries,
        strategy_display_name: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            entries,
            strategy_display_name: strategy_display_name.into(),
        }
    }
}

impl Default for ArtifactValidation {
    fn default() -> Self {
        Self::new(
            "trades.log",
            Arc::new(Mutex::new(Vec::new())),
            crate::domain::strategy::STRATEGY_DISPLAY_NAME,
        )
    }
}

impl ValidationPort for ArtifactValidation {
    fn log_decision(
        &self,
        data: &MarketData,
        decision: &Decision,
        blocked: bool,
        signed_intent: &SignedIntent,
        performance: &Performance,
        regime: &str,
    ) -> anyhow::Result<()> {
        let action_str = match decision.action {
            crate::domain::model::Action::Buy => "Buy",
            crate::domain::model::Action::Sell => "Sell",
            crate::domain::model::Action::Hold => "Hold",
        };

        let blocked_reason = if blocked {
            Some(decision.reasoning.clone())
        } else {
            None
        };

        let record = TradeLogRecord {
            timestamp: chrono::Utc::now().to_rfc3339(),
            action: action_str.to_string(),
            price: data.price,
            confidence: decision.confidence,
            reasoning: decision.reasoning.clone(),
            pnl: performance.pnl,
            drawdown: performance.drawdown,
            balance: performance.balance,
            peak_balance: performance.peak_balance,
            blocked_by_risk: blocked,
            regime: Some(regime.to_string()),
            tx_hash: None, // Set by main loop after chain submission
            blocked_reason: blocked_reason.clone(),
        };

        {
            let mut g = self.entries.lock().expect("log mutex poisoned");
            g.push(record.clone());
        }

        let artifact = serde_json::json!({
            "type": "trade_execution",
            "timestamp": record.timestamp,
            "strategy": self.strategy_display_name,
            "agent": signed_intent.intent.agent_id,
            "blocked_by_risk": blocked,
            "regime": regime,
            "market": {
                "pair": data.pair,
                "price": data.price,
                "ohlc_closes": data.ohlc_closes
            },
            "decision": decision,
            "intent": signed_intent.intent,
            "signature": signed_intent.signature,
            "risk": {
                "blocked": blocked,
                "reason": blocked_reason,
            },
            "performance": {
                "pnl": performance.pnl,
                "drawdown": performance.drawdown,
                "balance": performance.balance,
                "peak_balance": performance.peak_balance
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
