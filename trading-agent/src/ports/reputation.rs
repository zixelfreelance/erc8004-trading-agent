use async_trait::async_trait;

/// Metric to post as feedback
#[derive(Debug, Clone)]
pub struct ReputationMetric {
    pub tag1: String,        // e.g. "performance"
    pub tag2: String,        // e.g. "sharpe"
    pub value: i128,         // encoded value (e.g. 187 for 1.87)
    pub decimals: u8,        // e.g. 2
}

#[async_trait]
pub trait ReputationPort: Send + Sync {
    /// Post a feedback metric for an agent
    async fn post_feedback(
        &self,
        agent_id: u64,
        metric: &ReputationMetric,
        feedback_uri: &str,
    ) -> anyhow::Result<String>; // returns tx hash or log ID

    /// Get summary reputation for an agent
    async fn get_reputation(
        &self,
        agent_id: u64,
        tag1: &str,
        tag2: &str,
    ) -> anyhow::Result<Option<(u64, i128, u8)>>; // (count, value, decimals)
}
