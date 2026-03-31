use crate::domain::model::Action;

pub trait ExecutionPort: Send + Sync {
    fn execute(&self, action: &Action) -> anyhow::Result<()>;
}
