# Project Status

> Last updated: 2026-04-01 (Day 3 — Sprint 2 near-complete)

## Current Sprint: Sprint 2 — On-Chain Trust Layer (Days 4–7)

**Goal:** Full ERC-8004 integration. Agent has on-chain identity + reputation.

## Build Progress

### Done — Sprint 1

- [x] Hexagonal architecture (ports/adapters/domain)
- [x] Kraken CLI market data (ticker + OHLC)
- [x] Kraken CLI execution (paper + live modes)
- [x] Momentum + Volatility Guard strategy
- [x] ADK/Claude hybrid decisions with tool-augmented signals
- [x] Risk gates: drawdown, position, confidence, circuit breaker, daily loss
- [x] Trade intent signing (SHA-256)
- [x] Validation artifacts (JSON to file + memory)
- [x] HTTP server: `/logs`, `/metrics`, `/decision-schema`
- [x] SvelteKit dashboard
- [x] Performance tracking, agent metrics
- [x] 32 unit tests (risk, strategy, signer, intent)
- [x] Project docs + sprint plans + agent orchestration design

### Done — Sprint 2

- [x] MarketData enriched: bid, ask, spread, vwap_24h, volume_24h, ohlc_highs, ohlc_lows
- [x] OHLC lookback → 50 candles
- [x] EIP-712 signer (ethers-core + ethers-signers, secp256k1 ECDSA)
- [x] SignerDriver enum (Simple | Eip712, auto-detect)
- [x] `domain/indicators.rs` — SMA, EMA, RSI, MACD, Bollinger, ATR, ADX
- [x] `domain/regime.rs` — stateful detector with hysteresis (trending/ranging/transition)
- [x] Fee-aware filter (`passes_fee_filter`, min_edge_pct=0.7%)
- [x] `ports/identity.rs` + `adapters/chain_identity.rs` — on-chain identity shell
- [x] `ports/reputation.rs` + `adapters/chain_reputation.rs` — on-chain reputation shell
- [x] `adapters/kraken_mcp.rs` — MCP subprocess + JSON-RPC + tool whitelist
- [x] `AgentIdentityRegistry.sol` — ERC-721 + EIP-712 wallet auth (7 tests)
- [x] `AgentReputationRegistry.sol` — feedback + tag-filtered summary
- [x] `RiskRouter.sol` — EIP-712 intent validation + risk limits (6 tests)
- [x] `Deploy.s.sol` — deploys all 3 contracts
- [x] Agent Card JSON per ERC-8004 spec
- [x] `deploy.sh` helper script
- [x] ADK signal tools expanded: `compute_technical_indicators` (RSI/MACD/BB/ATR/ADX/spread/vwap/vol)
- [x] **Wired into main loop:** regime detection (ADX + BB bandwidth → stateful detector)
- [x] **Wired into main loop:** fee filter (edge < 0.7% → hold)
- [x] **Wired into main loop:** regime filter (transition → hold)
- [x] **Wired into main loop:** ATR trailing stop (set on buy, trail on hold, force sell on breach)

### Blocked — Needs Human Action

- [ ] Install Foundry → `forge build` + `forge test` all contracts
- [ ] Deploy contracts to Sepolia
- [ ] Kraken API key → PnL accumulation
- [ ] surge.xyz registration → prize eligibility
- [ ] Hackathon Discord → Risk Router ABI confirmation

### Not Started — Sprint 2 Remaining

- [ ] Mint agent identity NFT
- [ ] Host Agent Card on IPFS
- [ ] Set agentWallet via EIP-712
- [ ] IPFS pinning adapter (reqwest + Pinata)
- [ ] Event listener for TradeApproved
- [ ] Wire chain components into main.rs

### Not Started — Sprint 3 (Days 8–10)

- [ ] ATR in-agent trailing stops
- [ ] Mean-reversion strategy (Bollinger + RSI for ranging regime)
- [ ] Expanded LLM signal card (wire indicators to ADK) — partially done
- [ ] Multi-pair scanning (BTC, ETH, SOL)
- [ ] WebSocket streaming (replace polling)
- [ ] Order book depth
- [ ] Dashboard: reputation, on-chain activity, indicator charts
- [ ] Deploy dashboard publicly
- [ ] Integration tests + deterministic demo mode

### Not Started — Sprint 4 (Days 11–12)

- [ ] Video demo, slide deck, submission

## Codebase Stats

- **Rust source files:** 40+
- **Rust tests:** 58 passing
- **Solidity contracts:** 3 (Identity, Reputation, Router)
- **Solidity tests:** 13
- **HTTP endpoints:** 3 (`/logs`, `/metrics`, `/decision-schema`)
- **Decision modes:** 3 (momentum, adk, hybrid)
- **Execution modes:** 2 (paper, live)
- **Risk gates:** 7 (circuit breaker, consecutive losses, daily loss, drawdown, position, confidence, fee filter)
- **Indicators:** 7 (SMA, EMA, RSI, MACD, Bollinger, ATR, ADX)

## Research Artifacts

| File | Summary |
|---|---|
| `gemini-research.md` | Architecture validation, on-chain gap analysis |
| `perplexity-strategy-research.md` | Strategy enhancements (indicators, regime, sizing) |
| `perplexity-infra-research.md` | Kraken CLI surface, ERC-8004 interfaces, ethers-rs patterns |
