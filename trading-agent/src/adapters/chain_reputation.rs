use async_trait::async_trait;
use crate::ports::reputation::{ReputationMetric, ReputationPort};

pub struct ChainReputationAdapter {
    pub registry_address: String,
    pub rpc_url: String,
}

impl ChainReputationAdapter {
    pub fn new(registry_address: String, rpc_url: String) -> Self {
        Self { registry_address, rpc_url }
    }

    pub fn noop() -> Self {
        Self {
            registry_address: String::new(),
            rpc_url: String::new(),
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.registry_address.is_empty() && !self.rpc_url.is_empty()
    }
}

#[async_trait]
impl ReputationPort for ChainReputationAdapter {
    async fn post_feedback(
        &self,
        agent_id: u64,
        metric: &ReputationMetric,
        feedback_uri: &str,
    ) -> anyhow::Result<String> {
        if !self.is_configured() {
            eprintln!("chain: reputation registry not configured, skipping feedback");
            return Ok("noop".to_string());
        }
        eprintln!(
            "chain: post_feedback(agent={agent_id}, {}/{}={} dec={}, uri={feedback_uri})",
            metric.tag1, metric.tag2, metric.value, metric.decimals
        );
        // TODO: Replace with real ethers-rs contract call
        // let contract = AgentReputationRegistry::new(addr, provider);
        // contract.give_feedback(agent_id, metric.value, metric.decimals, ...).send().await?;
        Ok("pending".to_string())
    }

    async fn get_reputation(
        &self,
        agent_id: u64,
        tag1: &str,
        tag2: &str,
    ) -> anyhow::Result<Option<(u64, i128, u8)>> {
        if !self.is_configured() { return Ok(None); }
        eprintln!("chain: get_reputation(agent={agent_id}, {tag1}/{tag2})");
        Ok(None)
    }
}
