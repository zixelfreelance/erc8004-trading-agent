# Proof-of-Trust Trading Agent

## The Problem: AI Trading Agents Are Black Boxes

Every AI trading agent today asks you to do the same thing: hand over your capital and hope for the best. There is no way to verify who controls the agent, what rules it follows, whether those rules are actually enforced, or what happened after a trade goes wrong. Risk controls exist in code the AI can override. Audit trails are server logs the operator can edit. Identity is a brand name, not a cryptographic fact.

This is not a theoretical concern. Autonomous agents managing real capital with no enforced constraints and no verifiable audit trail is the single biggest barrier to adoption. The problem is not intelligence -- it is trust. And trust requires proof, not promises.

## The Solution: AI Proposes, the System Decides

The Proof-of-Trust Trading Agent introduces a hard architectural separation between what the AI wants to do and what the system allows it to do. The AI is treated as untrusted input. It can propose trades, but it cannot execute them. Every proposal must pass through a deterministic risk pipeline that the AI cannot bypass, modify, or influence.

This is built around what we call the **Trust Triangle** -- three properties that every trade must satisfy simultaneously:

1. **Identity** -- The agent has a verifiable on-chain identity (ERC-8004). Anyone can look up who this agent is, who operates it, and what its declared capabilities are.
2. **Constraints** -- Every trade proposal passes through 7 enforced risk gates. The AI has no mechanism to skip or weaken them. If a gate says no, the trade does not happen.
3. **Auditability** -- Every decision produces a validation artifact: the market data that was seen, the reasoning that was generated, the signed intent, the risk gate results, and the execution outcome. These artifacts are hashed and pinned to IPFS, with CIDs recorded on-chain. This is not logging -- it is cryptographic proof.

## Architecture: Hexagonal Design in Rust

The agent is built using hexagonal (ports-and-adapters) architecture in Rust. The domain layer contains pure business logic -- strategy computation, risk evaluation, performance tracking -- with zero dependencies on external systems. Ports define abstract contracts for execution, market data, signing, chain interaction, and IPFS pinning. Adapters implement those contracts for specific infrastructure: Kraken CLI for execution, Anthropic ADK-Rust for AI decisions, secp256k1 for EIP-712 signing, Pinata for IPFS.

This means every external dependency is swappable. You can replace Kraken with Binance, Claude with GPT, Sepolia with mainnet -- without touching the domain logic. The architecture enforces this at the type level.

The decision pipeline flows through seven stages:

```
ERC-8004 Identity Registry (on-chain ERC-721)
  -> AI Strategy Engine (Momentum + Claude/ADK Hybrid)
  -> Intent Builder + Signer (EIP-712 typed data, secp256k1 ECDSA)
  -> Risk Gates (7 enforced gates)
  -> Execution (Kraken CLI -- paper or live)
  -> Validation & Reputation (artifact hash -> on-chain)
  -> Dashboard (SvelteKit) + HTTP API
```

## ERC-8004 Integration: Full Implementation

The agent implements all four pillars of the ERC-8004 standard:

**Identity Registry** -- The agent mints an ERC-721 identity token on Sepolia. It exposes a standards-compliant Agent Card at `/.well-known/agent-card.json`, making it discoverable by any ERC-8004-aware system. The card declares the agent's capabilities, operator, and risk policy.

**Reputation Registry** -- Every 100 ticks, the agent posts a performance summary on-chain: realized PnL, drawdown, Sharpe ratio, and trade count. This creates a public, tamper-proof performance record that accumulates over time.

**Validation Artifacts** -- Every trade decision (executed or blocked) generates a structured artifact containing market inputs, indicator values, AI reasoning, the signed intent, risk gate results, and the execution outcome. This artifact is serialized, hashed, and pinned to IPFS via Pinata. The CID is then submitted on-chain, creating an immutable link between the decision and its proof.

**Risk Router** -- The on-chain Risk Router contract receives signed trade intents (EIP-712 typed data with secp256k1 ECDSA signatures) and records validation results. This creates a verifiable on-chain record that the agent's risk policy was consulted before execution.

Three Solidity contracts are deployed and verified on Sepolia:
- `AgentIdentityRegistry` at `0xc83F0B94E7969Cc2265aB0A187Ba0F2e6A5B9554`
- `AgentReputationRegistry` at `0x40dB57F7D848457289CEda81F39df15C4203D576`
- `RiskRouter` at `0xCbC5DFeD364b6D65233DfA6edCcb95088F8f189B`

## Risk Controls: 7 Gates the AI Cannot Bypass

Every trade proposal passes through a cascade of risk gates implemented in the Rust domain layer. These are not suggestions -- they are hard stops. Over 20% of trade proposals are blocked in practice, proving the gates are active, not decorative.

| Gate | Default | Enforcement |
|---|---|---|
| **Max Drawdown Cap** | 5% | Hard stop -- agent pauses all trading if equity drops more than 5% from peak |
| **Position Limit** | 1 open | No pyramiding -- only one position at a time, prevents stacking risk |
| **Confidence Floor** | 60% | Rejects any trade signal where the strategy's confidence score is below 0.6 |
| **Fee-Aware Filter** | 0.7% edge | Blocks trades where the expected price move does not exceed the 0.52% round-trip fee |
| **Regime Filter** | Auto | Holds during market regime transitions (unclear trend vs. range) -- requires 3 confirming bars |
| **Circuit Breaker** | 3 losses / $5 daily | Auto-pauses trading after 3 consecutive losses or $5 cumulative daily loss |
| **ATR Trailing Stop** | 1.5x ATR | Mechanical exit at entry minus 1.5 times the Average True Range -- no AI discretion on exits |

Additionally, the agent enforces a **trade cooldown** (minimum 3 ticks between trades) to prevent overtrading, and uses **ATR-scaled position sizing** that dynamically adjusts trade volume between 20% and 100% of the base size based on volatility and confidence.

## Tech Stack

- **Agent Core:** Rust with async Tokio runtime, hexagonal architecture, pure domain logic
- **AI Engine:** Anthropic ADK-Rust with Claude Sonnet -- adversarial bull/bear prompts, 4 tool-augmented signals (price action, technical indicators, risk limits, sentiment)
- **Technical Indicators:** RSI(14), MACD(12,26,9), Bollinger Bands(20,2), ATR(14), ADX(14) -- all computed from 50-candle OHLC history with stateful regime detection (Trending/Ranging/Transition)
- **Execution:** Kraken CLI with paper mode (simulated, no API key needed), demo mode (reproducible 50-tick sequence), and live mode (real orders)
- **Signing:** EIP-712 ECDSA with secp256k1, SHA-256 fallback for environments without private keys
- **On-Chain:** 3 Solidity contracts on Sepolia -- Identity Registry, Reputation Registry, Risk Router
- **IPFS:** Pinata for cryptographic audit trail -- every decision artifact pinned with CID backfill
- **Dashboard:** SvelteKit on Google Cloud Run with Chart.js -- price chart with Bollinger Bands, buy/sell markers, PnL tracking, drawdown monitoring, risk gate decision log
- **Decision Modes:** Pure momentum (no API key), Claude ADK (full AI), or Hybrid (rules + AI refinement)

## Results

- **72 passing tests** across the Rust codebase covering indicators, risk gates, position sizing, fee filtering, circuit breaker logic, signing, and the full agent loop
- **3 deployed and verified contracts** on Sepolia with real on-chain state
- **Real-time dashboard** at [trading-dashboard-675072986521.us-central1.run.app](https://trading-dashboard-675072986521.us-central1.run.app) showing live price charts, trade markers, PnL curves, and risk gate decisions
- **IPFS audit trail** with Pinata integration for every trade decision artifact
- **HTTP API** exposing `/metrics`, `/logs`, `/decision-schema`, and `/.well-known/agent-card.json` for programmatic access and ERC-8004 discovery
- **Three execution modes** (paper, demo, live) making the system immediately runnable with zero configuration
- **Full documentation** published at [zixelfreelance.github.io/erc8004-trading-agent](https://zixelfreelance.github.io/erc8004-trading-agent/)

## Why It Matters

The race in AI trading is not about who builds the smartest agent. It is about who builds the most trustworthy one. An agent that generates 50% returns but cannot prove it followed its own rules is a liability. An agent that generates 10% returns with a cryptographic audit trail, enforced risk limits, and on-chain identity is an institution-grade product.

The Proof-of-Trust architecture is not specific to trading. Any autonomous agent managing resources on behalf of humans -- whether financial, operational, or social -- needs the same three properties: verified identity, enforced constraints, and auditable decisions. We built the trading agent as a concrete demonstration of this pattern, but the architecture generalizes to any domain where AI agents need to be trusted, not just believed.

The winning AI trading agents will not be the ones with the best returns. They will be the ones you can verify.
