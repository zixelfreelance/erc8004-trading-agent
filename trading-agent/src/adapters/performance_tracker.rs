use std::sync::Mutex;

use crate::domain::performance::Performance;
use crate::ports::performance::PerformancePort;

pub struct PerformanceTracker {
    initial_balance: f64,
    inner: Mutex<Performance>,
}

impl PerformanceTracker {
    pub fn new(initial_balance: f64) -> Self {
        Self {
            initial_balance,
            inner: Mutex::new(Performance::initial(initial_balance)),
        }
    }
}

impl PerformancePort for PerformanceTracker {
    fn snapshot(&self) -> Performance {
        self.inner
            .lock()
            .expect("performance mutex poisoned")
            .clone()
    }

    fn record_balance(&self, new_balance: f64) {
        let mut g = self.inner.lock().expect("performance mutex poisoned");
        *g = g.clone().apply_balance(self.initial_balance, new_balance);
    }
}
