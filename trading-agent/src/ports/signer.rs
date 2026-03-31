use crate::domain::intent::TradeIntent;
use crate::domain::signed_intent::SignedIntent;

pub trait SignerPort: Send + Sync {
    fn sign(&self, intent: TradeIntent) -> SignedIntent;
}
