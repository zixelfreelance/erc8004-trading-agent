# Product Scope — Proof-of-Trust Trading Agent

> Investor-ready product statement + compliance alignment. Updated 2026-04-01.

---

## Product Statement (Plain Language)

**What:** A constrained AI trading agent that automates BTC/USD trades on Kraken within user-approved risk limits.

**How:** No custody. Per-user API keys. Clear mandate capture. Every decision is cryptographically signed and verifiable on-chain via ERC-8004.

**Not:** Not a hedge fund. Not custodial. Not unbounded AI. Not on-chain execution. The agent proposes — the risk system decides.

---

## Q2 2026 Scope (Frozen)

| Dimension | Constraint | Reasoning |
|---|---|---|
| **Venue** | Kraken CLI only | Single integration, paper + live, well-documented API |
| **Asset** | BTC/USD only | Deepest liquidity, tightest spreads, most data |
| **Users** | Advanced individuals | Self-serve, own API keys, understand crypto risks |
| **Mode** | Paper default, opt-in live | De-risk while building track record |
| **Strategy** | Momentum + Claude hybrid | Validated, regime-aware, fee-filtered |
| **Surfaces** | CLI loop + SvelteKit dashboard | Minimal UI, maximum auditability |
| **Chain** | Base L2 (ERC-8004 identity only) | Low gas, EVM-compatible, no execution on-chain |
| **Classification** | Signals + assisted automation | Not discretionary, not copy-trading |

### Explicitly Out of Scope (Q2)

- Multi-exchange or multi-asset
- External capital / deposits from others
- Copy trading / social features
- On-chain execution (DEX routing)
- Mobile app or margin trading

---

## Control Plane — 7 Risk Gates

| Gate | Default | Enforcement |
|---|---|---|
| Max Drawdown | 5% | Hard stop — agent pauses if equity drops 5% from peak |
| Position Limit | 1 open | No pyramiding — one position at a time |
| Confidence Floor | 60% | Reject low-conviction signals |
| Fee Filter | 0.7% min edge | Don't trade unless expected move covers fees + slippage |
| Circuit Breaker | 3 consecutive losses | Pause after losing streak |
| Daily Loss Limit | $5 | Hard daily cap on realized losses |
| ATR Trailing Stop | 1.5x ATR | Mechanical exit — no discretion |

All gates enforced **before** execution. The AI cannot bypass them.

---

## Trust Plane — ERC-8004 as Trust Wrapper (Not Execution Dependency)

| On-Chain (Verifiable) | Off-Chain (Performant) |
|---|---|
| Agent identity (ERC-721 NFT) | Strategy logic (Rust) |
| Risk policy summary | Real-time indicator computation |
| Performance metrics (Sharpe, drawdown, win rate) | Trade execution (Kraken CLI) |
| Validation artifact hashes (Merkle roots) | Full decision artifacts (JSON) |
| MCP endpoint for discoverability | Dashboard + HTTP API |

The agent works without the chain. The chain makes it **verifiable and discoverable**.

---

## Where We Are Today

### Product Model: "Constrained Agent Inside a Hard Risk Envelope"

This is exactly what the regulatory analysis recommends as the safest near-term use of AI in trading. Our agent:

- **Proposes** actions (AI/Claude) but **cannot violate** risk constraints
- Operates within deterministic guards: allowed symbols, max order size, max daily loss, max drawdown, approved order types
- Produces human-readable rationale logging for every decision
- Signs every intent cryptographically (EIP-712) before execution
- Posts validation artifacts on-chain for audit

This maps to the regulatory text's "assisted execution" model — not fully discretionary, not signals-only.

### What We've Built (Technical Inventory)

| System | Status | Regulatory Relevance |
|---|---|---|
| **Strategy engine** | Dual-mode (momentum + mean-reversion), regime-aware | Strategy validation |
| **Risk engine** | 7 gates + circuit breaker + ATR stops + fee filter | Loss containment |
| **Execution engine** | Kraken CLI (paper + live), with order confirmation | Venue integration |
| **Audit log** | JSONL artifacts with full decision context | Supervision, explainability |
| **Monitoring** | HTTP `/logs`, `/metrics`, dashboard | Real-time oversight |
| **Agent identity** | ERC-8004 on-chain registry (ERC-721 NFT) | Agent identification |
| **Signed intents** | EIP-712 typed data + secp256k1 ECDSA | Non-repudiation |
| **Risk Router** | On-chain constraint verification + TradeApproved/Rejected events | Enforced mandate |
| **Reputation** | On-chain feedback registry with tag-filtered scoring | Track record |
| **Agent Card** | ERC-8004 compliant, served at `/.well-known/agent-card.json` | Discovery, disclosure |

### Codebase Stats

- 61 Rust tests passing
- 3 Solidity contracts (Identity, Reputation, Risk Router) + 13 Solidity tests
- 7 technical indicators live in main loop
- Stateful regime detector with hysteresis
- Adversarial bull/bear Claude prompting with trade history context
- Agent Card endpoint for ERC-8004 discovery

## How This Maps to the Regulatory Framework

### Phase 1 (Current — Hackathon)

> "Signals and paper trading, prove demand, performance, and logging quality"

**Status: Mostly complete.**
- Paper trading works, live mode toggle exists
- Performance logging with full audit trail
- Strategy proven with 61 tests
- Missing: actual PnL track record (blocked on Kraken API key)

### Phase 2 (Post-Hackathon)

> "User-configured automation with strict guardrails and explicit mandate capture"

**What we'd need:**
- Mandate capture UI (user sets risk limits, allowed pairs, max position)
- Per-user risk config (currently global)
- User authentication + account isolation
- Disclosure language for AI use and limitations

### Phase 3 (Product)

> "Limited live execution on one venue with deep monitoring, incident response, and disclosures"

**What we'd need:**
- Incident response playbook
- Strategy versioning and model governance
- Replay tooling (deterministic demo mode is a start)
- Legal review of disclosure language
- Monitoring alerts (not just dashboard)

### Phase 4 (Scale)

> "Copy trading or broader discretionary models"

**What our architecture enables:**
- Multi-agent registry (ERC-8004 already supports this)
- Reputation-weighted capital allocation (our protocol vision)
- Strategy tokens (tokenized performance streams)
- Cross-venue execution (hexagonal arch = adapter swap)

## What We're Already Doing Right (Per Regulatory Analysis)

| Regulatory Requirement | Our Implementation |
|---|---|
| "Deterministic guards around an agent" | 7 risk gates + circuit breaker + fee filter + regime filter |
| "Allowed symbols, max order size, max daily turnover" | RiskRouter.sol enforces pair whitelist + position limits |
| "Human-readable rationale logging" | Every decision has `reasoning` field in JSONL artifact |
| "One venue, one asset class" | Kraken + BTC/USD focus |
| "Prove edge after fees, slippage, latency" | Fee filter rejects trades < 0.7% edge |
| "Kill switches" | Circuit breaker (consecutive losses + daily loss limit) |
| "Strategy has edge after fees" | Fee-aware filter is live in main loop |
| "Constrained agent, not autonomous trader" | AI proposes → risk gates enforce → contract verifies |

## What We're Missing (Gaps for a Real Product)

| Gap | Category | Priority for Product |
|---|---|---|
| No user mandate capture | Compliance | P0 for any user-facing product |
| No disclosure language | Legal | P0 |
| No suitability assessment | Regulatory | P1 (required for advisory) |
| No per-user risk isolation | Architecture | P1 |
| No strategy versioning/governance | Operations | P1 |
| No incident response | Operations | P1 |
| No walk-forward validation | Strategy | P2 (backtests ≠ production) |
| No multi-venue reconciliation | Integration | P2 |

## Perplexity Research Priorities

These are the areas where we need external research to inform next steps beyond the hackathon:

### 1. Jurisdiction Map for Crypto Trading Automation
**Query:** "What licenses/registrations are required to operate an AI-powered crypto trading automation service in US, EU, and UAE? Specifically for BTC/USD on Kraken, where the agent executes trades within user-defined risk limits. Compare signals-only vs assisted-execution vs discretionary models."

**Why:** Determines whether we need SEC/FINRA registration, MiFID II authorization, or can operate under lighter regimes.

### 2. Kraken Partner Policies for Third-Party Automation
**Query:** "What are Kraken's policies for third-party trading automation? Do they allow API-based automated trading by third-party apps? What restrictions exist? Are there partner programs for trading tool developers?"

**Why:** We're building on their CLI/API — need to know if this is commercially viable or just hackathon-scoped.

### 3. Disclosure Language for AI Trading Agents
**Query:** "What disclosure language is required by SEC and ESMA for AI-powered trading assistants? Specifically for robo-advisers and automated execution systems. What must be disclosed about AI limitations, strategy risks, and conflict of interest?"

**Why:** If we productize, every user-facing surface needs compliant disclosure.

### 4. Copy Trading Regulatory Framework
**Query:** "What is the current regulatory framework for copy trading in EU (ESMA), UK (FCA), and US (SEC/FINRA)? How does on-chain reputation-based capital allocation (ERC-8004 style) map to copy-trading regulations?"

**Why:** Our protocol vision (reputation-weighted capital allocation) is essentially on-chain copy trading — need to understand the compliance boundary.

### 5. Walk-Forward Strategy Validation Standards
**Query:** "What are industry standards for validating algorithmic trading strategies before live deployment? Walk-forward testing methodology, minimum sample sizes, statistical significance requirements, and how regulators evaluate strategy validation."

**Why:** "61 tests pass" is engineering quality. Regulators want statistical evidence the strategy has edge.

### 6. AI Agent Fiduciary Obligations
**Query:** "Do AI trading agents have fiduciary obligations? How does SEC/FINRA treat AI systems that make investment decisions on behalf of users? What supervision requirements exist for algorithmic and AI-driven trading?"

**Why:** Determines whether our "constrained agent" model avoids fiduciary designation or not.

## Hackathon vs. Product — The Honest Gap

| Dimension | Hackathon (now) | Real Product |
|---|---|---|
| Users | Just us | Real people's money |
| Risk | $5 daily limit, paper mode | Millions in AUM |
| Supervision | Dashboard + logs | 24/7 monitoring + alerts + incident response |
| Compliance | None needed | SEC/FINRA/ESMA registration |
| Strategy proof | 61 unit tests | Walk-forward validation + live track record |
| Disclosure | Agent Card JSON | Legal-reviewed risk disclosures |
| Mandate | Global config | Per-user risk limits + suitability |

**Bottom line for the hackathon:** We've built the right architecture. The "constrained agent inside a hard risk envelope" is exactly what regulators and the analysis recommend. The hackathon proves it works. Productizing it requires legal, compliance, and operational infrastructure that's beyond code.

**Bottom line for judges:** Position this as "we built the infrastructure for accountable AI finance" not "we built a trading bot." The regulatory alignment is a feature, not a burden.

---

## Q2 Deliverables

1. **Live agent** on Kraken with real capital ($1K–$5K BTC/USD)
2. **30+ days track record** with walk-forward validation report
3. **Public dashboard** at a deployed URL
4. **ERC-8004 identity** on Base mainnet with reputation scoring
5. **Backtesting engine** with 6-month historical validation
6. **Compliance docs**: AI disclosure, risk governance, mandate capture flow

## Success Metrics

| Metric | Target | How Measured |
|---|---|---|
| Sharpe Ratio | > 1.0 | Walk-forward validation over 30+ days |
| Max Drawdown | < 5% | Live tracking via risk gate |
| Win Rate | > 55% | Trade log analysis |
| Uptime | > 99% | Agent health monitoring |
| Trades Blocked | > 20% | Proof that risk gates are active, not decorative |
| On-Chain Artifacts | 100% of trades | Merkle root anchoring on Base |
