# Proof-of-Trust Trading Agent

> An AI trading agent that cannot break its own rules — and proves it on-chain.

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

## Quick Start

```bash
# Clone and build
cd trading-agent
cargo build

# Run in paper mode (default)
cargo run

# Run in demo mode (reproducible 50-tick sequence for presentations)
AGENT_DEMO_MODE=true cargo run

# Run in live mode (requires Kraken API keys configured)
AGENT_EXECUTION_MODE=live AGENT_VOLUME=0.0001 cargo run

# Access endpoints
curl http://localhost:3030/metrics
curl http://localhost:3030/logs
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
| `ANTHROPIC_API_KEY` | — | Required for `adk` / `hybrid` modes |
| `AGENT_SIGNING_KEY` | `dev-local-key` | Hex private key → EIP-712, else SHA-256 |
| `CHAIN_ID` | `11155111` | Chain ID for EIP-712 domain (Sepolia) |

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
- **Dashboard:** SvelteKit
- **On-chain:** Solidity on Base Sepolia (in progress)

## Hackathon

[AI Trading Agents](https://lablab.ai/ai-hackathons/ai-trading-agents) — March 30 – April 12, 2026

Combined submission: Kraken Challenge + ERC-8004 Challenge

## License

MIT
