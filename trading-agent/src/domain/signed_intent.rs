use serde::{Deserialize, Serialize};

use super::intent::TradeIntent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedIntent {
    pub intent: TradeIntent,
    pub signature: String,
}
