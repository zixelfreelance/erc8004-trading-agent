use serde_json::Value;

pub struct IpfsPinner {
    api_key: String,
    api_secret: String,
}

impl IpfsPinner {
    /// Create from Pinata API credentials. Returns None if not configured.
    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("PINATA_API_KEY").ok()?;
        let api_secret = std::env::var("PINATA_API_SECRET").ok()?;
        if api_key.is_empty() || api_secret.is_empty() {
            return None;
        }
        Some(Self {
            api_key,
            api_secret,
        })
    }

    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
        }
    }

    /// Pin a JSON value to IPFS via Pinata. Returns the IPFS CID (hash).
    pub async fn pin_json(&self, name: &str, content: &Value) -> anyhow::Result<String> {
        let client = reqwest::Client::new();

        let body = serde_json::json!({
            "pinataContent": content,
            "pinataMetadata": {
                "name": name
            }
        });

        let response = client
            .post("https://api.pinata.cloud/pinning/pinJSONToIPFS")
            .header("pinata_api_key", &self.api_key)
            .header("pinata_secret_api_key", &self.api_secret)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Pinata pin failed ({status}): {text}");
        }

        let result: Value = response.json().await?;
        let cid = result
            .get("IpfsHash")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Pinata response missing IpfsHash"))?
            .to_string();

        eprintln!("ipfs: pinned '{name}' → ipfs://{cid}");
        Ok(cid)
    }

    /// Pin a validation artifact (trade decision + context).
    pub async fn pin_artifact(&self, artifact: &Value) -> anyhow::Result<String> {
        let name = format!(
            "trade-artifact-{}",
            chrono::Utc::now().format("%Y%m%d-%H%M%S")
        );
        self.pin_json(&name, artifact).await
    }

    /// Pin the agent card JSON.
    pub async fn pin_agent_card(&self, card: &Value) -> anyhow::Result<String> {
        self.pin_json("agent-card", card).await
    }
}
