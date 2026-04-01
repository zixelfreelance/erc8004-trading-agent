# Changelog

All notable changes to this project will be documented in this file.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)

## [Unreleased]

### Added
- **Demo mode:** `AGENT_DEMO_MODE=true` replays 50-tick MockMarket sequence for reliable demos
- **WebSocket streaming:** `KrakenWsStream` logs real-time price/bid/ask each tick
- **Order book depth:** periodic `kraken book` fetch with spread + imbalance logging
- **IPFS pinning:** Pinata adapter for pinning validation artifacts (`adapters/ipfs_pinner.rs`)
- **Integration tests:** 5 end-to-end tests with MockMarket (hold, buy, multi-tick, demo, risk block)
- **Trade cooldown:** `min_ticks_between_trades` in RiskConfig (env: `AGENT_MIN_TICKS_BETWEEN_TRADES`, default 3)
- **Volatility-scaled position sizing:** `compute_position_size()` — risk_budget * confidence / (ATR * stop), 20-100% of base volume
- **Mean-reversion strategy:** Bollinger + RSI for ranging markets, auto-selected by regime
- **Dual-mode strategy:** `compute_regime_aware_decision()` — momentum in trending, mean-reversion in ranging, hold in transition
- **Adversarial prompts:** Bull/bear framing in ADK instruction, conviction-biased
- **Recent trade history:** Claude sees last 5 executed trades as learning context
- **Agent Card endpoint:** `/.well-known/agent-card.json` served via HTTP (ERC-8004 discovery)
- **Dashboard metrics bar:** ticks, executed, blocked, holds, errors
- **Dashboard price chart:** Bollinger Bands + buy/sell trade markers
- **Wired into main loop:** regime detection, fee filter, ATR trailing stops
- Regime detector feeds ADX + Bollinger bandwidth each tick, blocks trades during transition
- Fee filter rejects trades with edge < min_edge_pct (default 0.7%)
- ATR trailing stop: set at entry - 1.5x ATR on buy, trails upward, forces sell on breach
- `AGENT_ATR_STOP_MULTIPLIER` env var (default 1.5)
- Live execution mode (`AGENT_EXECUTION_MODE=live`) — real Kraken trades via `kraken buy/sell`
- Agent metrics: atomic counters for ticks, executed, blocked, holds, errors
- `GET /metrics` HTTP endpoint exposing real-time counters
- `domain/metrics.rs` — lock-free metrics with `AtomicU64`
- Circuit breaker: auto-pause after N consecutive losses or daily $ limit
- `RiskConfig.max_consecutive_losses` (env: `AGENT_MAX_CONSECUTIVE_LOSSES`, default 3)
- `RiskConfig.daily_loss_limit` (env: `AGENT_DAILY_LOSS_LIMIT`, default $5)
- `PositionState.record_trade_result()` — tracks win/loss streaks
- `PositionState.reset_circuit_breaker()` — manual reset
- 29 unit tests: risk gates (13), strategy (8), signer (4), intent builder (4)
- Project docs: README.md, CHANGELOG.md, STATUS.md
- Sprint breakdowns: `docs/sprints/sprint-{1,2,3,4}.md`
- Agent orchestration design: `docs/AGENTS.md`
- MarketData extended: bid, ask, spread, vwap_24h, volume_24h, ohlc_highs, ohlc_lows
- OHLC lookback increased to 50 candles (was 10)
- EIP-712 signer with secp256k1 ECDSA (`ethers-core` + `ethers-signers`)
- `SignerDriver` enum (Simple | Eip712) with auto-detection from key format
- `domain/indicators.rs` — SMA, EMA, RSI(14), MACD(12,26,9), Bollinger(20,2), ATR(14), ADX(14)
- `domain/regime.rs` — stateful regime detector with hysteresis (trending/ranging/transition)
- Fee-aware trade filter: `passes_fee_filter()` with configurable `min_edge_pct` (default 0.7%)
- `ports/identity.rs` + `adapters/chain_identity.rs` — on-chain identity adapter shell
- `ports/reputation.rs` + `adapters/chain_reputation.rs` — on-chain reputation adapter shell
- `adapters/kraken_mcp.rs` — Kraken MCP subprocess manager with JSON-RPC + tool whitelisting
- Solidity: `AgentIdentityRegistry.sol` — ERC-721 + EIP-712 wallet auth
- Solidity: `AgentReputationRegistry.sol` — feedback, tag-filtered summary
- Solidity: `RiskRouter.sol` — EIP-712 intent validation, risk limits, pair whitelist
- Solidity: `Deploy.s.sol` — deploys all 3 contracts
- Agent Card JSON (`contracts/agent-card.json`) per ERC-8004 spec
- 13 Solidity tests (7 identity + 6 router)

### Removed
- `claude_decision.rs` stub adapter (hardcoded responses, dead code)
- `Claude` variant from `DecisionDriver` enum

### Changed
- `KrakenPaperExecution` renamed to `KrakenExecution` with `ExecutionMode` enum
- `http_logs::router()` now accepts `AgentMetrics` parameter
- `TradingAgent` struct now includes `metrics: AgentMetrics` field
- `run_once()` increments correct metric counter after each risk gate decision

## [0.1.0] — 2026-03-30

### Added
- Hexagonal architecture: ports, adapters, domain, application layers
- Kraken CLI integration: market data (ticker + OHLC) and paper trading
- Momentum + Volatility Guard strategy (`domain/strategy.rs`)
- ADK-Rust Claude decision adapter with tool-augmented signals
- Hybrid decision mode (momentum prior + Claude refinement)
- Risk gates: max drawdown, position limits, confidence floor
- ERC-8004-style trade intents with SHA-256 signing
- Validation artifacts: JSON logs to file + in-memory store
- HTTP log server with CORS (axum)
- `GET /logs` and `GET /decision-schema` endpoints
- SvelteKit dashboard (`ui/`)
- Performance tracking: PnL, drawdown, peak balance
- Configurable via environment variables
- Agent Registration JSON for intent metadata
