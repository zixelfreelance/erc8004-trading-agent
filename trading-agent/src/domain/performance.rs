use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Performance {
    pub balance: f64,
    pub peak_balance: f64,
    pub drawdown: f64,
    pub pnl: f64,
}

impl Performance {
    pub fn initial(initial_balance: f64) -> Self {
        Self {
            balance: initial_balance,
            peak_balance: initial_balance,
            drawdown: 0.0,
            pnl: 0.0,
        }
    }

    pub fn apply_balance(self, initial_balance: f64, new_balance: f64) -> Self {
        let peak_balance = self.peak_balance.max(new_balance);
        let drawdown = if peak_balance > 0.0 {
            (peak_balance - new_balance) / peak_balance
        } else {
            0.0
        };
        let pnl = new_balance - initial_balance;
        Self {
            balance: new_balance,
            peak_balance,
            drawdown,
            pnl,
        }
    }
}
