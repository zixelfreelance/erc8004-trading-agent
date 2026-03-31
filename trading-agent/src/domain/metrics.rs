use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use serde::Serialize;

#[derive(Debug, Default)]
pub struct AgentMetricsInner {
    pub ticks: AtomicU64,
    pub executed: AtomicU64,
    pub blocked: AtomicU64,
    pub holds: AtomicU64,
    pub errors: AtomicU64,
}

pub type AgentMetrics = Arc<AgentMetricsInner>;

pub fn new_metrics() -> AgentMetrics {
    Arc::new(AgentMetricsInner::default())
}

#[derive(Debug, Clone, Serialize)]
pub struct MetricsSnapshot {
    pub ticks: u64,
    pub trades_executed: u64,
    pub trades_blocked: u64,
    pub holds: u64,
    pub errors: u64,
}

impl AgentMetricsInner {
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            ticks: self.ticks.load(Ordering::Relaxed),
            trades_executed: self.executed.load(Ordering::Relaxed),
            trades_blocked: self.blocked.load(Ordering::Relaxed),
            holds: self.holds.load(Ordering::Relaxed),
            errors: self.errors.load(Ordering::Relaxed),
        }
    }

    pub fn record_tick(&self) {
        self.ticks.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_executed(&self) {
        self.executed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_blocked(&self) {
        self.blocked.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_hold(&self) {
        self.holds.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }
}
