# Demo Video Script — Proof-of-Trust Trading Agent (3 min)

## [0:00-0:20] Intro
"This is Proof-of-Trust — an AI trading agent that can't break its own rules. Every decision is signed, validated, and provable on-chain."

## [0:20-0:50] Terminal — Agent Startup
- Show Cloud Run logs or local `cargo run` output
- Point out: identity registration, IPFS agent card pin, risk config
- "The agent registers its identity on-chain at startup"

## [0:50-1:30] Dashboard — Live Trading
- Open `https://trading-dashboard-XXXX.us-central1.run.app`
- Show price chart with Bollinger Bands filling in
- Show buy/sell markers appearing on the chart
- Show PnL curve updating
- Point to a **blocked trade** in the log: "This trade was rejected by the fee filter — the expected edge didn't cover the 0.52% round-trip fee"

## [1:30-2:00] Agent Card — ERC-8004 Discovery
- Open `https://trading-agent-675072986521.us-central1.run.app/.well-known/agent-card.json`
- "Any ERC-8004-aware system can discover this agent — its identity, capabilities, and risk policy"
- Show the `/metrics` endpoint: ticks, executed, blocked counts

## [2:00-2:30] On-Chain Proof
- Open Sepolia Etherscan for AgentIdentityRegistry (`0xc83F0B94E7969Cc2265aB0A187Ba0F2e6A5B9554`)
- Show the registered agent identity
- Open a Pinata IPFS link showing a validation artifact
- "Every trade decision — executed or blocked — is hashed and pinned to IPFS"

## [2:30-3:00] Closing
- Show GitHub repo briefly
- "AI proposes. The system decides. The chain proves it."
- "Proof-of-Trust — because the winning AI trading agents won't have the best returns. They'll be the ones you can verify."
- Flash: `github.com/zixelfreelance/erc8004-trading-agent`

## Recording Tips
- Use QuickTime (File > New Screen Recording) or OBS
- 1080p, no webcam needed (screen only is fine)
- Upload to YouTube (@ZixelLLC) as unlisted, grab the link for submission
