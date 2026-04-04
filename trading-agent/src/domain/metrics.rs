use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use serde::Serialize;

#[derive(Debug, Default)]
pub struct AgentMetricsInner {
    pub ticks: AtomicU64,
    pub executed: AtomicU64,
    pub blocked: AtomicU64,
    pub holds: AtomicU64,
    pub errors: AtomicU64,
    pub wins: AtomicU64,
    pub losses: AtomicU64,
    pub regime: Mutex<String>,
    pub returns: Mutex<Vec<f64>>,
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
    pub wins: u64,
    pub losses: u64,
    pub win_rate: f64,
    pub sharpe_ratio: f64,
    pub regime: String,
}

impl AgentMetricsInner {
    pub fn snapshot(&self) -> MetricsSnapshot {
        let wins = self.wins.load(Ordering::Relaxed);
        let losses = self.losses.load(Ordering::Relaxed);
        let total = wins + losses;
        let win_rate = if total > 0 {
            wins as f64 / total as f64
        } else {
            0.0
        };

        let sharpe_ratio = {
            let returns = self.returns.lock().unwrap_or_else(|e| e.into_inner());
            compute_sharpe(&returns)
        };

        let regime = self
            .regime
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();

        MetricsSnapshot {
            ticks: self.ticks.load(Ordering::Relaxed),
            trades_executed: self.executed.load(Ordering::Relaxed),
            trades_blocked: self.blocked.load(Ordering::Relaxed),
            holds: self.holds.load(Ordering::Relaxed),
            errors: self.errors.load(Ordering::Relaxed),
            wins,
            losses,
            win_rate,
            sharpe_ratio,
            regime,
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

    pub fn record_win(&self) {
        self.wins.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_loss(&self) {
        self.losses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_return(&self, pnl_pct: f64) {
        if let Ok(mut returns) = self.returns.lock() {
            returns.push(pnl_pct);
        }
    }

    pub fn set_regime(&self, r: &str) {
        if let Ok(mut regime) = self.regime.lock() {
            *regime = r.to_string();
        }
    }
}

/// Annualized Sharpe ratio: mean(returns) / std(returns) * sqrt(252)
fn compute_sharpe(returns: &[f64]) -> f64 {
    if returns.len() < 2 {
        return 0.0;
    }
    let n = returns.len() as f64;
    let mean = returns.iter().sum::<f64>() / n;
    let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let std_dev = variance.sqrt();
    if std_dev < 1e-12 {
        return 0.0;
    }
    (mean / std_dev) * 252.0_f64.sqrt()
}
