use crate::domain::model::Action;
use crate::ports::execution::{ExecutionFill, ExecutionPort};

/// Simulates trade execution for demo mode — no external dependencies.
pub struct MockExecution {
    balance: std::sync::Mutex<f64>,
}

impl MockExecution {
    pub fn new(initial_balance: f64) -> Self {
        Self {
            balance: std::sync::Mutex::new(initial_balance),
        }
    }
}

impl ExecutionPort for MockExecution {
    fn execute(&self, action: &Action) -> anyhow::Result<ExecutionFill> {
        let balance = *self.balance.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
        match action {
            Action::Hold => Ok(ExecutionFill::default()),
            Action::Buy | Action::Sell => {
                eprintln!("demo-exec: simulated {action:?} at current balance {balance:.2}");
                Ok(ExecutionFill {
                    parsed_balance: Some(balance),
                })
            }
        }
    }
}
