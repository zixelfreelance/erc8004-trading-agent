use std::process::Command;

use serde_json::Value;

use crate::domain::model::Action;
use crate::ports::execution::{ExecutionFill, ExecutionPort};

pub struct KrakenPaperExecution {
    pub pair: String,
    pub volume: String,
}

impl KrakenPaperExecution {
    pub fn new(pair: impl Into<String>, volume: impl Into<String>) -> Self {
        Self {
            pair: pair.into(),
            volume: volume.into(),
        }
    }

    fn run_paper(&self, args: &[&str]) -> anyhow::Result<Vec<u8>> {
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

impl ExecutionPort for KrakenPaperExecution {
    fn execute(&self, action: &Action) -> anyhow::Result<ExecutionFill> {
        match action {
            Action::Buy => {
                let out = self.run_paper(&[
                    "paper",
                    "buy",
                    &self.pair,
                    &self.volume,
                    "-o",
                    "json",
                ])?;
                Ok(ExecutionFill {
                    parsed_balance: parse_balance(&out),
                })
            }
            Action::Sell => {
                let out = self.run_paper(&[
                    "paper",
                    "sell",
                    &self.pair,
                    &self.volume,
                    "-o",
                    "json",
                ])?;
                Ok(ExecutionFill {
                    parsed_balance: parse_balance(&out),
                })
            }
            Action::Hold => Ok(ExecutionFill::default()),
        }
    }
}
