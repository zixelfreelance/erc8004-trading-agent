use std::process::Command;

use crate::domain::model::Action;
use crate::ports::execution::ExecutionPort;

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

    fn run_paper(&self, args: &[&str]) -> anyhow::Result<()> {
        let output = Command::new("kraken").args(args).output()?;
        if !output.status.success() {
            anyhow::bail!(
                "kraken {}: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }
}

impl ExecutionPort for KrakenPaperExecution {
    fn execute(&self, action: &Action) -> anyhow::Result<()> {
        match action {
            Action::Buy => {
                self.run_paper(&[
                    "paper",
                    "buy",
                    &self.pair,
                    &self.volume,
                    "-o",
                    "json",
                ])?;
            }
            Action::Sell => {
                self.run_paper(&[
                    "paper",
                    "sell",
                    &self.pair,
                    &self.volume,
                    "-o",
                    "json",
                ])?;
            }
            Action::Hold => {}
        }
        Ok(())
    }
}
