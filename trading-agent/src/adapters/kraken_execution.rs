use std::process::Command;

use serde_json::Value;

use crate::domain::model::Action;
use crate::ports::execution::{ExecutionFill, ExecutionPort};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Paper,
    Live,
}

impl ExecutionMode {
    pub fn from_env() -> Self {
        match std::env::var("AGENT_EXECUTION_MODE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "live" => Self::Live,
            _ => Self::Paper,
        }
    }
}

pub struct KrakenExecution {
    pub pair: String,
    pub volume: String,
    pub mode: ExecutionMode,
}

impl KrakenExecution {
    pub fn new(pair: impl Into<String>, volume: impl Into<String>, mode: ExecutionMode) -> Self {
        Self {
            pair: pair.into(),
            volume: volume.into(),
            mode,
        }
    }

    fn run_cmd(&self, args: &[&str]) -> anyhow::Result<Vec<u8>> {
        let output = Command::new("kraken").args(args).output()?;
        if !output.status.success() {
            anyhow::bail!(
                "kraken {}: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(output.stdout)
    }

    fn build_args(&self, side: &str) -> Vec<String> {
        match self.mode {
            ExecutionMode::Paper => vec![
                "paper".to_string(),
                side.to_string(),
                self.pair.clone(),
                self.volume.clone(),
                "-o".to_string(),
                "json".to_string(),
            ],
            ExecutionMode::Live => vec![
                side.to_string(),
                self.pair.clone(),
                self.volume.clone(),
                "-o".to_string(),
                "json".to_string(),
            ],
        }
    }
}

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

fn balance_from_value(v: &Value) -> Option<f64> {
    match v {
        Value::Object(map) => {
            for (k, val) in map {
                let lk = k.to_lowercase();
                if lk.contains("balance") {
                    if let Some(n) = val.as_f64() {
                        return Some(n);
                    }
                    if let Some(s) = val.as_str() {
                        if let Ok(n) = s.parse::<f64>() {
                            return Some(n);
                        }
                    }
                }
                if let Some(n) = balance_from_value(val) {
                    return Some(n);
                }
            }
            None
        }
        Value::Array(items) => {
            for x in items {
                if let Some(n) = balance_from_value(x) {
                    return Some(n);
                }
            }
            None
        }
        _ => None,
    }
}

fn parse_balance(stdout: &[u8]) -> Option<f64> {
    let text = String::from_utf8_lossy(stdout);
    let slice = extract_json_object(&text);
    let v: Value = serde_json::from_str(slice).ok()?;
    balance_from_value(&v)
}

impl ExecutionPort for KrakenExecution {
    fn execute(&self, action: &Action) -> anyhow::Result<ExecutionFill> {
        match action {
            Action::Buy => {
                let args = self.build_args("buy");
                let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                let out = self.run_cmd(&arg_refs)?;
                Ok(ExecutionFill {
                    parsed_balance: parse_balance(&out),
                })
            }
            Action::Sell => {
                let args = self.build_args("sell");
                let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                let out = self.run_cmd(&arg_refs)?;
                Ok(ExecutionFill {
                    parsed_balance: parse_balance(&out),
                })
            }
            Action::Hold => Ok(ExecutionFill::default()),
        }
    }
}
