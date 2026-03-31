# Agent & Subagent Design

> How we use Claude Code agents to parallelize development and move fast.

## Principle

One main conversation coordinates. Specialized subagents execute in parallel using worktrees or focused scopes. Each agent has a clear boundary — files it owns, deliverables it produces, and zero overlap with other agents.

## Agent Roster

### `main` — Orchestrator

**Role:** Coordinate sprints, wire components, resolve conflicts, review.

**Owns:** `main.rs`, `application/`, doc updates, git commits.

**Does NOT:** Write Solidity, write tests in isolation, touch UI.

---

### `agent-risk` — Risk & Safety

**Role:** Circuit breaker, risk gate enhancements, safety invariants.

**Owns:** `domain/risk.rs`, risk-related fields in `application/agent.rs`.

**Deliverables:**
- Circuit breaker: auto-pause after N consecutive losses or daily $ limit
- Enhanced risk config (daily loss limit env var)
- Risk gate for blocked-trade reasoning

**Sprint:** 1 (Day 3)

---

### `agent-test` — Testing

**Role:** Write all unit and integration tests.

**Owns:** `#[cfg(test)]` modules in domain files, integration test files.

**Deliverables:**
- `domain/risk.rs` tests: drawdown blocking, position limits, confidence floor
- `domain/strategy.rs` tests: momentum signals, volatility gating, edge cases
- `application/intent_builder.rs` + `adapters/signer.rs` tests
- Integration test: full `run_once` with mock ports
- Deterministic demo mode (Sprint 3)

**Sprint:** 1 (Day 3), 3 (Day 10)

---

### `agent-contracts` — Solidity Smart Contracts

**Role:** Write, test, deploy all on-chain contracts.

**Owns:** `contracts/` directory (Foundry project).

**Deliverables:**
- `AgentIdentityRegistry.sol` (ERC-721 + metadata URI)
- `ValidationRegistry.sol` (scores + artifact hashes)
- `RiskRouter.sol` (intent validation + on-chain limits)
- Deploy scripts for Base Sepolia
- Solidity tests

**Sprint:** 2 (Days 4–7)

**Isolation:** Runs in worktree. No Rust changes. Produces contract addresses + ABIs that `agent-chain` consumes.

---

### `agent-chain` — Rust ↔ Chain Bridge

**Role:** All Rust code that talks to Ethereum.

**Owns:** New files:
- `adapters/chain_identity.rs`
- `adapters/chain_reputation.rs`
- `adapters/chain_submitter.rs`
- `adapters/chain_events.rs`
- `adapters/eip712_signer.rs`
- `ports/identity.rs`
- `ports/reputation.rs`

**Deliverables:**
- alloy-rs integration in Cargo.toml
- EIP-712 typed data signing (replace SHA-256)
- Submit intents to Risk Router
- Post validation scores on-chain
- Read agent identity from registry

**Sprint:** 2 (Days 4–7), 3 (Day 8)

**Dependencies:** Needs contract addresses from `agent-contracts` (available by Day 5).

---

### `agent-data` — Market Data & Real-Time Feeds

**Role:** Enrich Kraken CLI data parsing. Add WebSocket streaming and order book depth.

**Owns:** `adapters/kraken_market.rs`, `adapters/kraken_ws.rs` (new), `adapters/kraken_book.rs` (new), `domain/model.rs` (MarketData struct only).

**Sprint 2 deliverables (Day 4):**
- Parse full ticker: bid, ask, spread, vwap, volume from `kraken ticker`
- Parse OHLC highs + lows (columns 2, 3) from `kraken ohlc`
- Extend `MarketData` struct with new fields
- Increase default lookback to 50 candles

**Sprint 3 deliverables (Day 8):**
- WebSocket streaming: `kraken ws ticker/book/trades BTC/USD -o json` → NDJSON `BufReader::lines()`
- Replace 10s polling loop with event-driven architecture
- Order book depth: `kraken book BTCUSD --depth 10` → spread + depth imbalance signal

**Sprint:** 2 (Day 4), 3 (Day 8)

**Isolation:** Independent of agent-contracts and agent-chain. No cross-dependencies.

---

### `agent-mcp` — Kraken MCP Integration

**Role:** Register Kraken CLI's MCP server tools in the ADK tool catalog so Claude can call Kraken natively.

**Owns:** `adapters/kraken_mcp.rs` (new), MCP tool registration in ADK config.

**Deliverables:**
- Start `kraken mcp` subprocess (stdio JSON-RPC)
- Register market tools (ticker, ohlc, book, trades, spread) in ADK
- Register paper tools (buy, sell, balance, positions) in ADK
- Claude can now call `kraken.market.ticker` directly during decision-making

**Sprint:** 2 (Day 5 only)

**Why this matters for judges:** Kraken is a Technology Partner. Deep MCP integration shows we use their infrastructure natively, not just as a CLI wrapper. This is exactly what they sponsor for.

---

### `agent-strategy` — Strategy Enrichment

**Role:** Build indicator module, regime detection, stops, fee filter, multi-pair scanning, parameter tuning.

**Owns:** `domain/strategy.rs`, new `domain/indicators.rs`, stop-loss logic in `adapters/kraken_execution.rs`.

**Deliverables:**
- Indicator module: RSI(14), MACD(12,26,9), Bollinger(20,2), ATR(14), ADX(14)
- Regime detector: trending/ranging/transition with hysteresis
- Fee-aware filter (edge > 0.7%)
- ATR-based trailing stop + take-profit
- Limit/stop-loss order types in CLI execution
- Multi-pair scanning: BTCUSD, ETHUSD, SOLUSD — select best opportunity
- Parameter tuning from 7+ days of live data
- Strykr PRISM API integration (free credits)

**Sprint:** 3 (Days 8–10)

**Dependencies:** Requires enriched MarketData from `agent-data` (Sprint 2 Day 4).

---

### `agent-ui` — Dashboard

**Role:** SvelteKit dashboard features and deployment.

**Owns:** `ui/` directory.

**Deliverables:**
- Reputation score + identity display
- On-chain activity feed (tx hashes, events)
- Prominent executed/blocked counter (the "mic-drop" metric)
- Deploy to Vercel/Cloudflare

**Sprint:** 3 (Day 9)

**Isolation:** Runs in worktree. Only reads `/logs`, `/metrics` API — no Rust changes.

---

### `agent-demo` — Submission Materials

**Role:** Video script, slides, project descriptions.

**Owns:** `docs/demo/` directory.

**Deliverables:**
- 2-minute video demo script (word-for-word)
- Slide deck structure (10-12 slides)
- Short description (1 paragraph) for lablab.ai
- Long description (full writeup) for lablab.ai

**Sprint:** 4 (Day 11)

---

## Parallelization Map by Sprint

### Sprint 1 (Days 1–3)

```
main ─────────────── [coordinate, wire, docs]
  ├── agent-risk ──── [circuit breaker]
  ├── agent-test ──── [unit tests: risk, strategy, signing]
  └── human ───────── [surge.xyz, Kraken key, social post]
```

**Parallel agents:** 2 (agent-risk + agent-test run simultaneously)

### Sprint 2 (Days 4–7)

```
main ─────────────── [wire chain into main.rs on Day 7]
  ├── agent-contracts [Identity → Validation → Router, sequential]
  ├── agent-chain ──── [alloy + EIP-712 → adapters, sequential]
  └── human ───────── [wallet, testnet ETH, deploy verification]
```

**Parallel agents:** 2 (contracts + chain run in parallel, sync on addresses mid-sprint)

### Sprint 3 (Days 8–10)

```
main ──────────────── [coordinate, merge]
  ├── agent-chain ──── [Merkle root, reputation scoring]
  ├── agent-contracts  [risk enforcement refinement]
  ├── agent-strategy ── [PnL analysis, param tuning]
  ├── agent-ui ──────── [dashboard upgrade + deploy]
  ├── agent-test ────── [integration tests + demo mode]
  └── human ─────────── [social posts, monitor PnL]
```

**Parallel agents:** 5 (maximum parallelism, no cross-dependencies)

### Sprint 4 (Days 11–12)

```
main ──────────────── [code cleanup, final fixes]
  ├── agent-demo ───── [script, slides, descriptions]
  └── human ─────────── [record video, submit, social]
```

**Parallel agents:** 1 (mostly human-driven)

## How to Launch Agents

Each subagent is invoked via Claude Code's Agent tool with:
- **Isolation:** `worktree` for agents that touch different file sets (contracts, UI)
- **Foreground:** for agents whose output is needed before next step
- **Background:** for independent work (tests, UI) while main continues

### Example: Sprint 1 Day 3

```
# Launch simultaneously:
Agent(agent-risk):  "Implement circuit breaker in domain/risk.rs..."
Agent(agent-test):  "Write unit tests for risk, strategy, signing..."

# While human does:
# - Configure Kraken CLI with API keys
# - Set AGENT_EXECUTION_MODE=live
# - Run agent with tiny volume
```

## Rules

1. **No overlap.** Each agent has exclusive file ownership. No two agents modify the same file.
2. **main wires.** Only main touches `main.rs` and `application/agent.rs` to compose components.
3. **Contracts first.** In Sprint 2, contract addresses gate chain adapters. Deploy early.
4. **Tests are independent.** agent-test never blocks other agents. Always runs in background.
5. **Human gates.** API keys, registrations, social posts, video recording — these only the human can do. Never block code work on them.
