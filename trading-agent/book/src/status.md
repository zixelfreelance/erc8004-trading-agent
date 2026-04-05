# Project Status

> Last updated: 2026-04-05

## End-to-End Integration Complete

The agent is **fully wired** — chain adapters call real Sepolia contracts, IPFS pins trade artifacts via Pinata, EIP-712 signing produces on-chain-verifiable intents, and the dashboard is deployed on Vercel. Live Kraken trading is code-complete but awaiting API keys.

## Integration Status

| Component | Status | Details |
|---|---|---|
| Chain identity | ✅ Live | Calls `AgentIdentityRegistry` on Sepolia via ethers-rs |
| Chain reputation | ✅ Live | Posts PnL feedback on-chain every 100 ticks |
| Chain RiskRouter | ✅ Live | Submits signed intents on-chain, backfills tx_hash |
| IPFS pinning | ✅ Live | Agent card on startup + per-trade + periodic snapshots (Pinata) |
| EIP-712 signing | ✅ Live | secp256k1 ECDSA with proper domain separator |
| Agent card | ✅ Production URLs | Endpoints point to Render, not localhost |
| Dashboard | ✅ Deployed | Vercel: erc8004-trading-agent.vercel.app |
| Backend | ⚠️ Sleeping | Render free tier goes idle — needs wake/redeploy |
| Kraken live trading | ⏳ Blocked | Code ready, needs API key from friend |

## Full Feature Inventory

### Domain Layer (11 files)

- `model.rs` — MarketData (bid/ask/spread/vwap/vol + OHLC H/L/C), Action, Decision
- `indicators.rs` — SMA, EMA, RSI(14), MACD(12,26,9), Bollinger(20,2), ATR(14), ADX(14)
- `regime.rs` — Stateful detector with hysteresis (trending/ranging/transition)
- `strategy.rs` — Dual-mode: momentum (trending) + mean-reversion (ranging)
- `risk.rs` — 8 risk gates + fee filter + position sizing + cooldown
- `metrics.rs` — Atomic counters (ticks, executed, blocked, holds, errors)
- `performance.rs` — Balance, PnL, drawdown, peak tracking, Sharpe ratio, win rate
- `intent.rs` + `signed_intent.rs` — ERC-8004-style trade intents
- `decision_json.rs` — Parse Claude's JSON output
- `log_record.rs` — Validation artifact schema (with tx_hash + cid fields)

### Ports Layer (10 files)

- `decision.rs`, `market.rs`, `execution.rs`, `signer.rs`, `validation.rs`, `performance.rs`
- `identity.rs` — On-chain agent registration
- `reputation.rs` — On-chain feedback posting
- `risk_router.rs` — On-chain intent submission

### Adapters Layer (19 files)

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
- `chain_identity.rs` — Calls AgentIdentityRegistry on Sepolia (register, get wallet/uri)
- `chain_reputation.rs` — Calls AgentReputationRegistry on Sepolia (post feedback, get summary)
- `chain_risk_router.rs` — Calls RiskRouter on Sepolia (submit signed intent, get tx_hash)
- `chain_provider.rs` — Shared SignerMiddleware builder for all chain adapters
- `ipfs_pinner.rs` — Pinata IPFS pinning (agent card + per-trade + periodic artifacts)
- `validation.rs` — JSONL artifact logger
- `performance_tracker.rs` — In-memory PnL tracking
- `http_logs.rs` — Axum server: /logs, /metrics, /decision-schema, /.well-known/agent-card.json

### Application Layer (2 files)

- `agent.rs` — Main loop with regime detection, fee filter, ATR stops, cooldown, position sizing
- `intent_builder.rs` — TradeIntent construction

### Solidity Contracts (3 deployed on Sepolia, 13 tests)

- `AgentIdentityRegistry.sol` — ERC-721 + EIP-712 wallet authorization → `0xc83F0B94E7969Cc2265aB0A187Ba0F2e6A5B9554`
- `AgentReputationRegistry.sol` — Feedback with tag-filtered summary → `0x40dB57F7D848457289CEda81F39df15C4203D576`
- `RiskRouter.sol` — EIP-712 intent validation, risk limits, pair whitelist → `0xCbC5DFeD364b6D65233DfA6edCcb95088F8f189B`
- `Deploy.s.sol` — Deploys all 3 contracts
- `agent-card.json` — ERC-8004 compliant Agent Card (production URLs)

### Dashboard (SvelteKit on Vercel)

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

- **Rust source files:** 50+
- **Rust tests:** 72 passing
- **Solidity contracts:** 3 (deployed on Sepolia)
- **Solidity tests:** 13
- **Technical indicators:** 7 (SMA, EMA, RSI, MACD, Bollinger, ATR, ADX)
- **Risk mechanisms:** 10
- **HTTP endpoints:** 4 (/logs, /metrics, /decision-schema, /.well-known/agent-card.json)
- **Decision modes:** 3 (momentum/regime-aware, ADK/Claude, hybrid)
- **Execution modes:** 3 (paper, live, demo)
- **ADK tools for Claude:** 4 (price action, technical indicators, risk limits, sentiment)

## Remaining Human Actions

| Action | Status | Impact |
|---|---|---|
| **Kraken API key** (from friend) | ⏳ Waiting | No live PnL = no Kraken leaderboard prize |
| **Wake Render backend** | ⚠️ 503 | Dashboard shows empty — needs redeploy or wake |
| **Social media posts** | ⏳ Not started | Required for social engagement prize track |
| **Surge social connect** | ⏳ Not done | Claim 10 Ignites |
| **MIT license file** | ⏳ Not done | GitHub best practice |
| **Final lablab.ai submit** | ⏳ Before Apr 12 | Deadline: April 12, 7:30 PM IRST |
