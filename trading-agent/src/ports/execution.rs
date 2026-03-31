use serde::{Deserialize, Serialize};

use crate::domain::model::Action;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionFill {
    pub parsed_balance: Option<f64>,
}

pub trait ExecutionPort: Send + Sync {
    fn execute(&self, action: &Action) -> anyhow::Result<ExecutionFill>;
}
