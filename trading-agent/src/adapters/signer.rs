use sha2::{Digest, Sha256};

use crate::domain::intent::TradeIntent;
use crate::domain::signed_intent::SignedIntent;
use crate::ports::signer::SignerPort;

pub struct SimpleSigner {
    pub private_key: String,
}

impl SimpleSigner {
    pub fn new(private_key: impl Into<String>) -> Self {
        Self {
            private_key: private_key.into(),
        }
    }
}

impl SignerPort for SimpleSigner {
    fn sign(&self, intent: TradeIntent) -> SignedIntent {
        let payload =
            serde_json::to_string(&intent).expect("TradeIntent serializes to JSON");
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        hasher.update(self.private_key.as_bytes());
        let signature = format!("{:x}", hasher.finalize());
        SignedIntent {
            intent,
            signature,
        }
    }
}
