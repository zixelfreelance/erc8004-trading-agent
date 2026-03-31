use crate::domain::performance::Performance;

pub trait PerformancePort: Send + Sync {
    fn snapshot(&self) -> Performance;
    fn record_balance(&self, new_balance: f64);
}
