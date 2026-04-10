# Project Status

> Last updated: 2026-04-10 (Day 11 — deployed to Google Cloud Run)

## Build Complete — Pending Human Gates Only

The agent is **feature-complete**. All code is written, tested, wired, and committed. What remains is deployment and submission artifacts that require human action.

## Full Feature Inventory

### Domain Layer (11 files)

- `model.rs` — MarketData (bid/ask/spread/vwap/vol + OHLC H/L/C), Action, Decision
- `indicators.rs` — SMA, EMA, RSI(14), MACD(12,26,9), Bollinger(20,2), ATR(14), ADX(14)
- `regime.rs` — Stateful detector with hysteresis (trending/ranging/transition)
- `strategy.rs` — Dual-mode: momentum (trending) + mean-reversion (ranging)
- `risk.rs` — 8 risk gates + fee filter + position sizing + cooldown
- `metrics.rs` — Atomic counters (ticks, executed, blocked, holds, errors)
- `performance.rs` — Balance, PnL, drawdown, peak tracking
- `intent.rs` + `signed_intent.rs` — ERC-8004-style trade intents
- `decision_json.rs` — Parse Claude's JSON output
- `log_record.rs` — Validation artifact schema

### Ports Layer (8 files)

- `decision.rs`, `market.rs`, `execution.rs`, `signer.rs`, `validation.rs`, `performance.rs`
- `identity.rs` — On-chain agent registration
- `reputation.rs` — On-chain feedback posting

### Adapters Layer (17 files)

- `kraken_market.rs` — Full ticker + OHLC H/L/C, 50 candle lookback
- `kraken_execution.rs` — Paper/Live modes with balance parsing
- `kraken_ws.rs` — WebSocket streaming (background thread, latest tick)
- `kraken_book.rs` — Order book depth (spread, imbalance)
- `kraken_mcp.rs` — MCP subprocess manager (JSON-RPC, tool whitelisting)
- `mock_market.rs` — 50-tick demo sequence for reproducible demos
- `adk_decision.rs` — Claude via ADK with adversarial bull/bear prompts + trade history
- `hybrid_decision.rs` — Regime-aware rules + Claude with indicator context
- `momentum_decision.rs` — Regime-aware dual-mode (auto-selects strategy)
- `decision_driver.rs` — Strategy selector enum
- `signer.rs` — EIP-712 ECDSA + SHA-256 fallback with auto-detect
- `chain_identity.rs` — On-chain identity adapter (stub, ready for ethers-rs)
- `chain_reputation.rs` — On-chain reputation adapter (stub, ready for ethers-rs)
- `ipfs_pinner.rs` — Pinata IPFS pinning for validation artifacts
- `validation.rs` — JSONL artifact logger
- `performance_tracker.rs` — In-memory PnL tracking
- `http_logs.rs` — Axum server: /logs, /metrics, /decision-schema, /.well-known/agent-card.json

### Application Layer (2 files)

- `agent.rs` — Main loop with regime detection, fee filter, ATR stops, cooldown, position sizing
- `intent_builder.rs` — TradeIntent construction

### Solidity Contracts (3 contracts, 13 tests)

- `AgentIdentityRegistry.sol` — ERC-721 + EIP-712 wallet authorization
- `AgentReputationRegistry.sol` — Feedback with tag-filtered summary
- `RiskRouter.sol` — EIP-712 intent validation, risk limits, pair whitelist
- `Deploy.s.sol` — Deploys all 3 contracts
- `agent-card.json` — ERC-8004 compliant Agent Card

### Dashboard (SvelteKit)

- Metrics bar: ticks, executed, blocked, holds, errors
- Price + Bollinger Bands chart with buy/sell trade markers
- PnL chart + drawdown chart
- Trade log table with block reason column
- Auto-refresh from /logs + /metrics

## Risk Controls (10 mechanisms)

1. Max drawdown cap (5%)
2. Single position limit
3. Confidence floor (0.6)
4. Fee-aware filter (edge < 0.7% → hold)
5. Regime filter (transition → hold)
6. Circuit breaker (3 consecutive losses)
7. Daily loss limit ($5)
8. Trade cooldown (min 3 ticks between trades)
9. ATR trailing stop (1.5x ATR, trails upward)
10. Volatility-scaled position sizing (20-100% of base)

## Codebase Stats

- **Rust source files:** 45+
- **Rust tests:** 70 passing
- **Solidity contracts:** 3
- **Solidity tests:** 13
- **Technical indicators:** 7 (SMA, EMA, RSI, MACD, Bollinger, ATR, ADX)
- **Risk mechanisms:** 10
- **HTTP endpoints:** 4 (/logs, /metrics, /decision-schema, /.well-known/agent-card.json)
- **Decision modes:** 3 (momentum/regime-aware, ADK/Claude, hybrid)
- **Execution modes:** 3 (paper, live, demo)
- **ADK tools for Claude:** 4 (price action, technical indicators, risk limits, sentiment)
- **Commits this session:** 12

## Human Gates (Blocking Deployment)

| Gate | Status | Impact |
|---|---|---|
| **Kraken API key** | Not done | No PnL on leaderboard = no Kraken prize |
| **surge.xyz registration** | Not done | No prize eligibility at all |
| **Foundry install** | ✅ Done | Contracts compiled and tested |
| **First social post** | Not done | Missing social engagement prize track |
| **Sepolia wallet + ETH** | ✅ Done | Contracts deployed on Sepolia |
| **Pinata API key** | Not done | Cannot pin artifacts to IPFS |

## What's Done

```
✅ Foundry installed → contracts compiled, tested, deployed to Sepolia
✅ Backend deployed to Google Cloud Run (demo mode, running)
✅ Dashboard deployed to Google Cloud Run
✅ Demo video on YouTube
✅ Project page on lablab.ai
✅ 3 contracts verified on Sepolia Etherscan
```

## What Happens When Remaining Gates Clear

```
Kraken API key:
  → AGENT_EXECUTION_MODE=live AGENT_VOLUME=0.0001 cargo run
  → PnL accumulation starts on leaderboard
  → Submit read-only key for verification

surge.xyz:
  → Register project → eligible for prizes

Pinata API key:
  → IPFS artifact pinning for audit trail
```

## Session Commits

| # | Hash | Summary |
|---|---|---|
| 1 | `7111067` | Sprint 1: live mode, metrics, circuit breaker, tests |
| 2 | `9aa5676` | Sprint 2: EIP-712, indicators, regime, MCP, chain adapters |
| 3 | `3b927ab` | 3 Solidity contracts + 13 tests + project docs |
| 4 | `5d7e4c7` | Wire regime, fee filter, ATR stops into main loop |
| 5 | `ff1a07e` | Docs update |
| 6 | `1499f03` | Dual-mode strategy, adversarial prompts, chain wiring, dashboard metrics |
| 7 | `f787374` | Trade history context, agent card endpoint, conviction prompts |
| 8 | `91726ad` | Demo mode, integration tests, WebSocket, order book, IPFS, dashboard charts |
| 9 | `f25492d` | Wire demo mode into main.rs |
| 10 | `02461d3` | Wire WebSocket + order book into main loop |
