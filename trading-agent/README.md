# Proof-of-Trust Trading Agent

![Cover](docs/cover.png)

> An AI trading agent that cannot break its own rules — and proves it on-chain.

[![Demo Video](https://img.shields.io/badge/Demo-YouTube-red)](https://youtu.be/7zc0qDvCOKo)
[![Dashboard](https://img.shields.io/badge/Dashboard-Cloud%20Run-blue)](https://trading-dashboard-675072986521.us-central1.run.app)
[![Docs](https://img.shields.io/badge/Docs-GitHub%20Pages-blue)](https://zixelfreelance.github.io/erc8004-trading-agent/)
[![X](https://img.shields.io/badge/@zixlancer-X-black)](https://x.com/zixlancer)
[![LinkedIn](https://img.shields.io/badge/Amin%20Sarafraz-LinkedIn-blue)](https://www.linkedin.com/in/a-s-abab883bb/)

## Vision

AI agents in finance today are black boxes. You can't verify what they do, why they do it, or whether they'll blow up your capital.

We're building the **safety layer for autonomous financial agents**, demonstrated through a live trading agent on Kraken.

**Core principle:** AI is untrusted input. The agent proposes — the system decides.

### Trust Triangle

Every agent must satisfy all three simultaneously:

- **Identity** (ERC-8004) — who acts
- **Constraints** (Risk Router) — what is allowed
- **Auditability** (Validation Artifacts) — what happened

### Proof of Decision

Every trade produces a verifiable artifact: inputs, reasoning, signed intent, risk validation result, and execution outcome. This is not logging — it's a cryptographic audit trail.

## Architecture

```
ERC-8004 Identity Registry
        |
  AI Strategy Engine (Momentum + Claude/ADK Hybrid)
        |
  Intent Builder + Signer (EIP-712)
        |
  Risk Gates (drawdown, position, confidence, circuit breaker)
        |
  Execution (Kraken CLI — paper or live)
        |
  Validation & Reputation (artifact hash -> on-chain)
        |
  Dashboard (SvelteKit) + HTTP API
```

**Hexagonal architecture:** Ports define contracts. Adapters are swappable. Domain logic is pure.

## Execution Modes

| Mode | What it does | Kraken API key? | Cost |
|---|---|---|---|
| **Paper** (default) | Simulates trades via `kraken paper buy/sell` — fake money, real prices | No | $0 |
| **Demo** | Replays 50 hardcoded ticks — fully offline, reproducible | No | $0 |
| **Live** | Places real orders on Kraken exchange with real money | Yes | Real USD |

Paper mode is the default and requires no API keys. Live mode requires a Kraken account with trading API keys and funded balance.

## Decision Modes

| Mode | How it decides | Anthropic API key? |
|---|---|---|
| **Momentum** (default) | Pure rules: RSI, MACD, Bollinger, ATR, regime detection | No |
| **ADK** | Claude makes all trading decisions via Anthropic ADK-Rust | Yes |
| **Hybrid** | Rules generate signal, Claude makes final call | Yes |

Momentum mode is fully self-contained — no external AI API needed. ADK and Hybrid modes use Claude as an additional decision layer with adversarial bull/bear prompts.

## Quick Start

```bash
# Clone and build
cd trading-agent
cargo build

# Run in paper mode (default — no API keys needed)
cargo run

# Run in demo mode (reproducible 50-tick sequence for presentations)
AGENT_DEMO_MODE=true cargo run

# Run with Claude AI decisions (requires ANTHROPIC_API_KEY in .env)
AGENT_DECISION=hybrid cargo run

# Run in live mode (requires Kraken API keys configured)
AGENT_EXECUTION_MODE=live AGENT_VOLUME=0.0001 cargo run

# Access endpoints
curl http://localhost:3030/metrics
curl http://localhost:3030/logs
curl http://localhost:3030/.well-known/agent-card.json
```

## Configuration

| Variable | Default | Description |
|---|---|---|
| `AGENT_PAIR` | `BTCUSD` | Trading pair |
| `AGENT_VOLUME` | `0.001` | Trade size |
| `AGENT_EXECUTION_MODE` | `paper` | `paper` or `live` |
| `AGENT_DECISION` | `momentum` | `momentum`, `adk`, or `hybrid` |
| `AGENT_INTERVAL_SECS` | `10` | Loop interval |
| `AGENT_MAX_DRAWDOWN` | `0.05` | Max drawdown before auto-hold |
| `AGENT_RISK_MIN_CONFIDENCE` | `0.6` | Min confidence to trade |
| `AGENT_MIN_EDGE_PCT` | `0.7` | Min price edge % to justify trade (fee filter) |
| `AGENT_ATR_STOP_MULTIPLIER` | `1.5` | ATR trailing stop distance (1.5 = entry - 1.5*ATR) |
| `AGENT_MAX_CONSECUTIVE_LOSSES` | `3` | Circuit breaker: pause after N losses |
| `AGENT_DAILY_LOSS_LIMIT` | `5.0` | Circuit breaker: max daily loss in USD |
| `AGENT_HTTP_PORT` | `3030` | Dashboard API port |
| `ANTHROPIC_API_KEY` | — | Only needed for `adk` / `hybrid` decision modes |
| `AGENT_SIGNING_KEY` | `dev-local-key` | Hex private key → EIP-712, else SHA-256 |
| `CHAIN_ID` | `11155111` | Chain ID for EIP-712 domain (Sepolia) |
| `PINATA_API_KEY` | — | IPFS pinning for audit artifacts (optional) |
| `PINATA_API_SECRET` | — | IPFS pinning for audit artifacts (optional) |

## HTTP API

| Endpoint | Description |
|---|---|
| `GET /metrics` | `{ ticks, trades_executed, trades_blocked, holds, errors }` |
| `GET /logs` | Full trade history with decision artifacts |
| `GET /decision-schema` | JSON Schema for Decision type |
| `GET /.well-known/agent-card.json` | ERC-8004 Agent Card for discovery |

## Strategies

- **Momentum + Volatility Guard** — deterministic signal based on price momentum with volatility band filtering
- **ADK/Claude** — LLM-powered decisions via Anthropic ADK-Rust with 4 tool-augmented signals (price action, technical indicators, risk limits, sentiment)
- **Hybrid** — momentum signal as "strong prior" refined by Claude (recommended)

### Technical Indicators (live in main loop)

RSI(14), MACD(12,26,9), Bollinger Bands(20,2), ATR(14), ADX(14) — all computed from 50-candle OHLC history and fed to Claude via ADK tools.

### Regime Detection

Stateful detector with hysteresis classifies market as **Trending** (ADX > 22), **Ranging** (low ADX + narrow Bollinger bandwidth), or **Transition** (hold — wait for clarity). Prevents whipsaw by requiring 3 consecutive confirming bars before switching state.

## Risk Controls

- **Max drawdown cap** (default 5%) — forced hold
- **Single position limit** — no stacking
- **Confidence floor** (below 0.6 = forced hold)
- **Fee-aware filter** — rejects trades with expected edge < 0.7% (covers 0.52% round-trip fee)
- **Regime filter** — holds during market transition (unclear regime)
- **Circuit breaker** — auto-pause after 3 consecutive losses or $5 daily loss
- **ATR trailing stop** — set at entry - 1.5x ATR, trails upward, force sells on breach
- **Metrics tracking** — every blocked trade is counted and exposed via `/metrics`

## Tech Stack

- **Agent:** Rust (hexagonal architecture, async tokio)
- **AI:** Anthropic ADK-Rust (Claude Sonnet) with tool-augmented decisions
- **Execution:** Kraken CLI (paper + live modes)
- **Signing:** EIP-712 ECDSA (secp256k1) with SHA-256 fallback
- **On-chain:** Solidity on Sepolia — Identity Registry, Reputation Registry, Risk Router
- **IPFS:** Pinata — cryptographic audit trail for trade artifacts
- **Dashboard:** SvelteKit on Google Cloud Run

## Deployed Contracts (Sepolia)

| Contract | Address |
|---|---|
| AgentIdentityRegistry | [`0xc83F0B94E7969Cc2265aB0A187Ba0F2e6A5B9554`](https://sepolia.etherscan.io/address/0xc83F0B94E7969Cc2265aB0A187Ba0F2e6A5B9554) |
| AgentReputationRegistry | [`0x40dB57F7D848457289CEda81F39df15C4203D576`](https://sepolia.etherscan.io/address/0x40dB57F7D848457289CEda81F39df15C4203D576) |
| RiskRouter | [`0xCbC5DFeD364b6D65233DfA6edCcb95088F8f189B`](https://sepolia.etherscan.io/address/0xCbC5DFeD364b6D65233DfA6edCcb95088F8f189B) |

## Links

| | |
|---|---|
| Demo Video | https://youtu.be/7zc0qDvCOKo |
| Dashboard | https://trading-dashboard-675072986521.us-central1.run.app |
| Backend API | https://trading-agent-675072986521.us-central1.run.app |
| Agent Card | https://trading-agent-675072986521.us-central1.run.app/.well-known/agent-card.json |
| Docs | https://zixelfreelance.github.io/erc8004-trading-agent/ |
| Hackathon | https://lablab.ai/ai-hackathons/ai-trading-agents |
| Project Page | https://lablab.ai/ai-hackathons/ai-trading-agents/proof-of-trust/proof-of-trust-trading-agent |
| X | https://x.com/zixlancer |
| LinkedIn | https://www.linkedin.com/in/a-s-abab883bb/ |

## Hackathon

[AI Trading Agents](https://lablab.ai/ai-hackathons/ai-trading-agents) — March 30 – April 12, 2026

Combined submission: Kraken Challenge + ERC-8004 Challenge

## License

MIT
