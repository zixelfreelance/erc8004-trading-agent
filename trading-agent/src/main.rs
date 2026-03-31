mod adapters;
mod application;
mod domain;
mod ports;

use adapters::claude_decision::ClaudeDecision;
use adapters::kraken_execution::KrakenPaperExecution;
use adapters::kraken_market::KrakenMarket;
use adapters::signer::SimpleSigner;
use adapters::validation::ArtifactValidation;
use application::agent::TradingAgent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pair = std::env::var("AGENT_PAIR").unwrap_or_else(|_| "BTCUSD".to_string());
    let volume = std::env::var("AGENT_VOLUME").unwrap_or_else(|_| "0.001".to_string());
    let intent_amount: f64 = volume.parse().unwrap_or(0.001);
    let agent_id = std::env::var("AGENT_ID").unwrap_or_else(|_| "agent-001".to_string());
    let signing_key =
        std::env::var("AGENT_SIGNING_KEY").unwrap_or_else(|_| "dev-local-key".to_string());

    let agent = TradingAgent {
        market: KrakenMarket::new(&pair),
        decision: ClaudeDecision,
        execution: KrakenPaperExecution::new(&pair, &volume),
        validation: ArtifactValidation::default(),
        signer: SimpleSigner::new(signing_key),
        agent_id,
        intent_amount,
    };

    let interval_secs: u64 = std::env::var("AGENT_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    loop {
        if let Err(e) = agent.run_once().await {
            eprintln!("tick error: {e:#}");
        }
        tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
    }
}
