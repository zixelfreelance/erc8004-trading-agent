mod adapters;
mod application;
mod domain;
mod ports;

use adapters::claude_decision::MockClaudeDecision;
use adapters::kraken_execution::KrakenPaperExecution;
use adapters::kraken_market::KrakenMarket;
use application::agent::TradingAgent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pair = std::env::var("AGENT_PAIR").unwrap_or_else(|_| "BTCUSD".to_string());

    let agent = TradingAgent {
        market: KrakenMarket::new(&pair),
        decision: MockClaudeDecision,
        execution: KrakenPaperExecution::new(&pair, "0.001"),
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
