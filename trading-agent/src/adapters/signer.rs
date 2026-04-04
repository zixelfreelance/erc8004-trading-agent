use ethers::core::types::{Address, H256, U256};
use ethers::core::utils::keccak256;
use ethers::signers::{LocalWallet, Signer};
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
        let payload = serde_json::to_string(&intent).expect("TradeIntent serializes to JSON");
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        hasher.update(self.private_key.as_bytes());
        let signature = format!("{:x}", hasher.finalize());
        SignedIntent { intent, signature }
    }
}

pub struct Eip712Signer {
    wallet: LocalWallet,
    verifying_contract: Address,
}

impl Eip712Signer {
    pub fn new(
        private_key_hex: &str,
        chain_id: u64,
        verifying_contract: Address,
    ) -> anyhow::Result<Self> {
        let clean = private_key_hex
            .strip_prefix("0x")
            .unwrap_or(private_key_hex);
        let wallet: LocalWallet = clean.parse::<LocalWallet>()?.with_chain_id(chain_id);
        Ok(Self {
            wallet,
            verifying_contract,
        })
    }

    /// EIP-712 domain separator matching OZ's EIP712("RiskRouter", "1")
    /// Domain type: EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)
    fn domain_separator(&self) -> H256 {
        let type_hash = keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
        );
        let name_hash = keccak256("RiskRouter");
        let version_hash = keccak256("1");
        let chain_id = U256::from(self.wallet.chain_id());

        let mut encoded = Vec::new();
        encoded.extend_from_slice(&type_hash);
        encoded.extend_from_slice(&name_hash);
        encoded.extend_from_slice(&version_hash);
        let mut buf = [0u8; 32];
        chain_id.to_big_endian(&mut buf);
        encoded.extend_from_slice(&buf);
        // verifyingContract as left-padded 32 bytes
        let mut addr_buf = [0u8; 32];
        addr_buf[12..].copy_from_slice(self.verifying_contract.as_bytes());
        encoded.extend_from_slice(&addr_buf);

        H256::from(keccak256(&encoded))
    }

    /// Struct hash matching RiskRouter.TRADE_INTENT_TYPEHASH
    fn struct_hash(&self, intent: &TradeIntent) -> H256 {
        let type_hash = keccak256(
            "TradeIntent(uint256 agentId,address agentWallet,string pair,uint8 action,uint128 amountUsdScaled,uint32 maxSlippageBps,uint64 nonce,uint64 deadline)",
        );

        let agent_id = U256::from(
            intent
                .agent_id
                .strip_prefix("agent-")
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(1),
        );
        let wallet: Address = intent.agent_wallet.parse().unwrap_or(Address::zero());
        let pair_hash = keccak256(intent.pair.as_bytes());
        let action: u8 = match intent.action.as_str() {
            "Buy" => 0,
            "Sell" => 1,
            _ => 2,
        };

        let mut encoded = Vec::new();
        encoded.extend_from_slice(&type_hash);
        let mut buf = [0u8; 32];
        // agentId
        agent_id.to_big_endian(&mut buf);
        encoded.extend_from_slice(&buf);
        // agentWallet (address, left-padded)
        buf = [0u8; 32];
        buf[12..].copy_from_slice(wallet.as_bytes());
        encoded.extend_from_slice(&buf);
        // pair (hashed string)
        encoded.extend_from_slice(&pair_hash);
        // action (uint8)
        buf = [0u8; 32];
        buf[31] = action;
        encoded.extend_from_slice(&buf);
        // amountUsdScaled (uint128)
        buf = [0u8; 32];
        U256::from(intent.amount_usd_scaled).to_big_endian(&mut buf);
        encoded.extend_from_slice(&buf);
        // maxSlippageBps (uint32)
        buf = [0u8; 32];
        U256::from(intent.max_slippage_bps).to_big_endian(&mut buf);
        encoded.extend_from_slice(&buf);
        // nonce (uint64)
        buf = [0u8; 32];
        U256::from(intent.nonce).to_big_endian(&mut buf);
        encoded.extend_from_slice(&buf);
        // deadline (uint64)
        buf = [0u8; 32];
        U256::from(intent.deadline).to_big_endian(&mut buf);
        encoded.extend_from_slice(&buf);

        H256::from(keccak256(&encoded))
    }

    fn eip712_digest(&self, intent: &TradeIntent) -> H256 {
        let domain_sep = self.domain_separator();
        let struct_h = self.struct_hash(intent);

        let mut msg = Vec::with_capacity(2 + 32 + 32);
        msg.push(0x19);
        msg.push(0x01);
        msg.extend_from_slice(domain_sep.as_bytes());
        msg.extend_from_slice(struct_h.as_bytes());

        H256::from(keccak256(&msg))
    }
}

impl SignerPort for Eip712Signer {
    fn sign(&self, intent: TradeIntent) -> SignedIntent {
        let digest = self.eip712_digest(&intent);
        let signature = self
            .wallet
            .sign_hash(digest)
            .expect("EIP-712 signing failed");

        SignedIntent {
            intent,
            signature: format!("0x{signature}"),
        }
    }
}

pub enum SignerDriver {
    Simple(SimpleSigner),
    Eip712(Eip712Signer),
}

impl SignerPort for SignerDriver {
    fn sign(&self, intent: TradeIntent) -> SignedIntent {
        match self {
            Self::Simple(s) => s.sign(intent),
            Self::Eip712(s) => s.sign(intent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_intent() -> TradeIntent {
        TradeIntent {
            agent_id: "agent-1".to_string(),
            action: "Buy".to_string(),
            amount: 1.0,
            price: 100.0,
            timestamp: 1000000,
            pair: "BTCUSD".to_string(),
            agent_wallet: "0x0000000000000000000000000000000000000000".to_string(),
            amount_usd_scaled: 10_000_000,
            max_slippage_bps: 50,
            nonce: 1000000,
            deadline: 1000300,
        }
    }

    #[test]
    fn sign_produces_hex_signature() {
        let signer = SimpleSigner::new("secret-key");
        let signed = signer.sign(test_intent());
        assert!(signed.signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn same_input_same_signature() {
        let signer = SimpleSigner::new("secret-key");
        let sig1 = signer.sign(test_intent()).signature;
        let sig2 = signer.sign(test_intent()).signature;
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn different_key_different_signature() {
        let s1 = SimpleSigner::new("key-a");
        let s2 = SimpleSigner::new("key-b");
        let sig1 = s1.sign(test_intent()).signature;
        let sig2 = s2.sign(test_intent()).signature;
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn signature_is_64_chars() {
        let signer = SimpleSigner::new("any-key");
        let signed = signer.sign(test_intent());
        assert_eq!(signed.signature.len(), 64);
    }

    #[test]
    fn eip712_sign_produces_0x_prefix() {
        let key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let contract = "0x0000000000000000000000000000000000000001"
            .parse()
            .unwrap();
        let signer = Eip712Signer::new(key, 11155111, contract).unwrap();
        let signed = signer.sign(test_intent());
        assert!(signed.signature.starts_with("0x"));
    }

    #[test]
    fn eip712_deterministic() {
        let key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let contract = "0x0000000000000000000000000000000000000001"
            .parse()
            .unwrap();
        let signer = Eip712Signer::new(key, 11155111, contract).unwrap();
        let sig1 = signer.sign(test_intent()).signature;
        let sig2 = signer.sign(test_intent()).signature;
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn eip712_different_from_sha256() {
        let key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let contract = "0x0000000000000000000000000000000000000001"
            .parse()
            .unwrap();
        let eip = Eip712Signer::new(key, 11155111, contract).unwrap();
        let simple = SimpleSigner::new(key);
        let sig_eip = eip.sign(test_intent()).signature;
        let sig_simple = simple.sign(test_intent()).signature;
        assert_ne!(sig_eip, sig_simple);
    }
}
