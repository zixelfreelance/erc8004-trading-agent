# Hackathon source of truth — trustless AI trading agents

**Workspace:** `/Users/bm/hack01`  
**Primary code:** `trading-agent` (Rust, hexagonal), `trading-dashboard` (SvelteKit), `adk-rust` (LLM/agent paths).  
**External canonical spec:** [EIP-8004: Trustless Agents](https://eips.ethereum.org/EIPS/eip-8004) — on-chain identity, reputation, and validation registries for autonomous agents.

This document is the **single narrative and requirements map** for the team: vision, mission, sponsor challenges, ERC-8004 + Kraken CLI synergy, current implementation, gaps, and research prompts.

---

## 1. Mission (north star)

**Design, build, and deploy trustless AI trading agents** that:

1. **Safely interact with capital** — hard limits, sandboxes, and circuit breakers; AI treated as untrusted input.  
2. **Execute strategies** — real programmatic trading and measurement (Kraken CLI / API path per challenge rules).  
3. **Demonstrate transparent behavior** — auditable decisions, signed intents, validation artifacts, and explainable logs.

**Critical problem (reframed):** Agents fail in finance without **verifiable identity** (who acted?), **enforced constraints** (what is allowed?), and **auditable behavior** (can we trust the trail?). The winning build treats **trust as a product feature**, not a side effect.

---

## 2. Vision

Ship a **combined-track** story where:

- **Execution intelligence** lives off-chain (rules, LLMs, ADK) but **never bypasses** risk and verification gates.  
- **Kraken CLI** (or equivalent Kraken programmatic access) is the **execution surface** for the Kraken challenge: market data + orders, build-in-public progress.  
- **EIP-8004** is the **trust surface** for the ERC-8004 challenge: agent identity on the Identity Registry, reputation from outcomes, validation artifacts for intents, risk checks, and checkpoints — operating within the **hackathon capital sandbox** (vault + risk router) when required.

Long-term: a **proto–financial infrastructure layer for AI agents** — separate **intelligence**, **trust**, and **execution**, with rules enforced where judges can verify them (chain + artifacts), not only in comments or README claims.

---

## 3. Official hackathon structure (two equal challenges)

Teams may enter **one** challenge, **the other**, or **both** in a single submission. Combined submissions can win prizes from **both** tracks.

### 3.1 Kraken challenge

| Expectation | Notes |
|-------------|--------|
| Use **Kraken CLI** as the AI-native CLI for programmatic trading | Retrieve market data, execute trades per official rules |
| **AI-driven** strategies | Signals + automation; align repo modes (`momentum`, `claude`, `adk`, `hybrid`) with pitch |
| **Build in public** | Development progress shared as part of hackathon culture |
| **Ranking** | Emphasis on **net PnL** during the competition period (plus any published tie-breakers) |
| **Prizes** | Additional Kraken-specific prizes on top of main pool |

### 3.2 ERC-8004 challenge

| Expectation | Notes |
|-------------|--------|
| **Register identity** on the **ERC-8004 Identity Registry** | Tie every action to an on-chain agent identity |
| **Reputation** | Measurable, objective outcomes over time |
| **Validation artifacts** | Trade intents, risk checks, strategy checkpoints — verifiable trail |
| **Capital sandbox** | Operate through the **provided vault and risk router** so performance is measurable on-chain in **stablecoin terms** |
| **Ranking** | **Risk-adjusted** profitability, **drawdown control**, **validation quality** — not raw PnL alone |

### 3.3 Combined submission (recommended positioning)

**One closed-loop system:**

`EIP-8004 identity + reputation + validation` → `signed, risk-gated intents` → `Kraken CLI execution` → `outcomes feed reputation and dashboards`

Pitch line (internal): *A verifiable financial agent with on-chain identity, enforced risk constraints, and transparent execution — not “an AI bot that trades.”*

---

## 4. Why ERC-8004 (EIP-8004) + Kraken CLI together

You cannot put full AI + low-latency exchange logic entirely on-chain. The **workable pattern** is **hybrid**:

| Layer | Role |
|-------|------|
| **On-chain (EIP-8004)** | Agent NFT / identity, policy hooks, reputation updates, validation registry references, sandbox vault compliance |
| **Off-chain brain** | Models, indicators, LLM/ADK decisions — output structured **proposals** only |
| **Verifier / gate** | Validates signatures, schema, limits; aligns with risk router rules before any order |
| **Execution (Kraken CLI)** | The only path to live market actions after approval — **no direct “model → exchange”** shortcut |

**EIP-8004 (high level, per EIP):** Composable registries for **identity**, **reputation**, and **validation** so agents are first-class, verifiable economic actors. Reference implementation and networks: see [eip-8004-contracts](https://github.com/erc-8004/erc-8004-contracts) and the EIP text.

**Kraken CLI:** Operational layer — **DevOps-style** trading automation, data access, and scripting. It does **not** replace on-chain trust; it **implements** approved execution off-chain.

**The bridge (mandatory in a real combined build):** A service (Rust, Node, or Python) that:

1. Subscribes to chain events / registry state / sandbox policy  
2. Accepts **signed intents** from the agent core  
3. Calls **Kraken CLI** (or API) only when **on-chain + risk-router + local policy** agree  
4. Emits **validation artifacts** and updates reputation inputs per spec

Without this bridge, “token triggers CLI” remains a slide-deck idea, not a demo.

---

## 5. Trust model (what “trustless” actually means here)

Honest framing for judges:

- **Trustless** = cryptographic and **on-chain enforcement of constraints** + **verifiable artifacts**; not “no humans” and not “decentralized exchange.”  
- **CEX execution** (Kraken) is **centralized**; trustlessness applies to **identity, policy, audit trail, and sandbox math** — partial but valuable.  
- **AI is unreliable** — sandbox it: confidence floors, drawdown stops, whitelisted pairs, max size, circuit breakers.

**Non-negotiables for a strong ERC-8004 story:**

- Structured decision payload (action, pair, size, confidence, timestamp, `agent_id`)  
- **Signature** (or protocol-prescribed validation) over that payload  
- **Risk validation result** stored or referenced where judges can inspect  
- **Execution receipt** correlated back to the intent

This repo already moves in that direction with **domain risk**, **intent builder**, and **signer**; the gap is **EIP-8004 registration**, **sandbox integration**, and **on-chain artifact commitment** (see Section 8).

---

## 6. Current codebase (what exists today)

1. **Market:** `KrakenMarket` — pair data and OHLC (interval / lookback via env).  
2. **Decision:** `DecisionDriver` — `momentum`, `claude`, `adk`, `hybrid`.  
3. **Risk:** `domain/risk.rs` — drawdown cap, min confidence for trades, single-position semantics.  
4. **Intent & signing:** `intent_builder` + `SimpleSigner` — demo-oriented structured + signed artifacts.  
5. **Execution:** `KrakenPaperExecution` — **simulated** fills and balance for safe iteration (may **not** satisfy “real trades only” for final Kraken submission — confirm rules).  
6. **Observability:** Axum **`GET /logs`**; dashboard polls and charts PnL / drawdown (Chart.js).

**Design patterns:** Hexagonal **ports & adapters**, **decision driver** strategy, **application service** (`TradingAgent::run_once`), **validation/logging adapter**, Svelte 5 **runes** on the UI.

---

## 7. Technical stack (as implemented + extensions)

| Layer | Now | Likely additions for full combined track |
|-------|-----|------------------------------------------|
| Agent core | Rust, Tokio, anyhow | Keep; add CLI wrapper or sidecar for Kraken CLI if not in-process |
| HTTP | Axum, CORS | Optional: webhook for chain events |
| LLM / agents | adk-rust, Claude adapter | Same |
| Frontend | SvelteKit 2, Vite 6, TS strict, Chart.js | Optional: on-chain tx links, artifact viewer |
| On-chain | — | Solidity/Foundry or Hardhat; EIP-8004 registry clients |
| Execution | Paper adapter | Kraken CLI or REST **behind** gate; record receipts |

**Team norms:** `pnpm` (Node), `cargo` (Rust).

---

## 8. Gap analysis — repo vs challenges

| Requirement | Kraken track | ERC-8004 track | This repo today |
|-------------|--------------|----------------|-----------------|
| Kraken market data | Yes | Optional via execution | Yes (adapter) |
| Programmatic / CLI execution | Yes | After sandbox approval | Paper execution only |
| AI-driven strategy | Yes | Yes | Yes (multi-mode) |
| Build in public | Yes | Check brief | Process, not code |
| Identity registry registration | — | Yes | Not implemented |
| Reputation from outcomes | — | Yes | Not on-chain |
| Validation artifacts (intents, risk, checkpoints) | Partially via logs | Yes, formal | HTTP logs; not EIP-8004 validation registry |
| Sandbox vault + risk router | — | Yes | Not integrated |
| Ranking focus | Net PnL | Risk-adjusted + drawdown + validation | Dashboard supports narrative |

**Priority order for a combined win:** (1) **Sandbox + risk router compliance**, (2) **real or rule-compliant Kraken execution path**, (3) **EIP-8004 identity + validation hooks**, (4) polish PnL story without sacrificing drawdown discipline.

---

## 9. Phased build plan (minimal → full story)

**Phase A — Controlled agent (fastest demo)**  
Rule-based or simple AI + strict risk + rich logs + dashboard. Prove the **pipeline** and **transparency**.

**Phase B — Kraken track hardening**  
Swap or add execution adapter: **Kraken CLI** subprocess or API with keys in env; retain **intent → verify → execute** ordering. Build-in-public artifacts (commits, thread, or devlog per rules).

**Phase C — ERC-8004 track**  
Register agent; commit or reference **validation payloads** per EIP-8004 flows; route capital via **sandbox vault**; update reputation inputs from measured outcomes.

**Phase D — Combined narrative**  
End-to-end demo: show **identity** → **signed intent** → **risk router OK** → **Kraken execution** → **log + reputation/dashboard**.

---

## 10. Killer features (pick 1–2 for scope)

1. **Proof-of-decision** — For each trade: inputs, reasoning, signed intent, risk outcome, execution receipt, PnL delta (matches “validation quality”).  
2. **Circuit breaker** — Auto-pause on drawdown / daily loss (matches ERC-8004 ranking).  
3. **Reputation-weighted allocation** (advanced) — More capital to agents with better validated history.  
4. **Multi-agent competition** (stretch) — Same infra, different `AGENT_ID` / registry entries.

---

## 11. Demo flow (about two minutes)

1. Show **on-chain agent identity** (registry ID / link).  
2. Show **live or recent decision** with reasoning + confidence.  
3. Show **signed intent** and **risk / sandbox check** passed or blocked.  
4. Show **Kraken-side execution** (CLI or UI proof) consistent with intent.  
5. Show **dashboard** — PnL, drawdown, validation trail.

If live APIs fail, fall back to **canned last run** with honest labeling.

---

## 12. Repository layout

```
trading-agent/          # Hexagonal agent
  src/main.rs           # Wiring, HTTP logs server, tick loop
  src/domain/           # strategy, risk, model, intent
  src/ports/            # traits
  src/application/      # TradingAgent, intent_builder
  src/adapters/         # Kraken, logs, decisions, signer, validation, …
trading-dashboard/      # SvelteKit + Chart.js
adk-rust/               # Local ADK crate
```

---

## 13. Key environment variables (`trading-agent`)

| Variable | Purpose |
|----------|---------|
| `AGENT_PAIR` | Pair (e.g. `BTCUSD`) |
| `AGENT_VOLUME` | Size string |
| `AGENT_ID` | Agent id in intents (align with on-chain id later) |
| `AGENT_SIGNING_KEY` | Signing secret (replace with wallet/session keys per EIP-8004 plan) |
| `AGENT_INITIAL_BALANCE` | Performance tracker seed |
| `AGENT_HTTP_PORT` | Log server (default `3030`) |
| `AGENT_DECISION` | `momentum` \| `claude` \| `adk` \| `hybrid` |
| `AGENT_INTERVAL_SECS` | Tick interval |
| `AGENT_OHLC_INTERVAL`, `AGENT_MOMENTUM_LOOKBACK` | History |
| `AGENT_STRATEGY_*`, `AGENT_RISK_*`, `AGENT_MAX_DRAWDOWN` | Strategy + risk |

---

## 14. Code reference map

| Concern | Path |
|---------|------|
| Composition & loop | `trading-agent/src/main.rs` |
| Orchestration | `trading-agent/src/application/agent.rs` |
| Rule strategy | `trading-agent/src/domain/strategy.rs` |
| Risk | `trading-agent/src/domain/risk.rs` |
| Intent | `trading-agent/src/application/intent_builder.rs` |
| Dashboard | `trading-dashboard/src/routes/+page.svelte` |

---

## 15. Prompts for tooling (copy-paste)

### 15.1 Claude Code

```text
You are working in hack01: Rust hexagonal trading-agent + SvelteKit trading-dashboard + adk-rust.

Hackathon: Combined Kraken CLI execution + EIP-8004 (ERC-8004) identity, reputation, validation registries and capital sandbox / risk router per official brief.

Mission: Trustless AI trading agents — safe capital, executed strategies, transparent behavior. AI proposes; risk layer and chain policy gate; Kraken executes only after approval.

Constraints:
- Preserve TradingAgent::run_once: market → decision → risk → intent → sign → execute → log.
- Prefer new adapters (execution, chain, registry client) over branching inside run_once.
- Rust 2021, Tokio, anyhow in binary; pnpm only for frontend; Svelte 5 runes, TS strict.
- When adding Kraken CLI: subprocess or official SDK per repo choice; never skip risk checks.
- When adding EIP-8004: follow eips.ethereum.org/EIPS/eip-8004 and hackathon’s contract addresses / ABIs.

Task: {DESCRIBE_FEATURE}

Deliver: minimal diff, new env vars, how demo proves Kraken + ERC-8004 judges’ criteria.
```

### 15.2 Perplexity (research)

```text
Research with authoritative links:

1. EIP-8004: Identity, Reputation, Validation registries — interfaces, events, and reference contracts (github.com/erc-8004/erc-8004-contracts).
2. Kraken CLI: install, auth, market data, place order — official Kraken / Kraken API docs only.
3. “AI trading agents ERC-8004” LabLab or CompeteHub hackathon: sandbox vault, risk router, ranking metrics (risk-adjusted, drawdown, validation quality).
4. Patterns for linking signed off-chain intents to on-chain validation (EIP-712, registries, or hackathon-specific).
5. Legal/disclaimer language for hackathon demos (not investment advice).

Output: bullets + full URLs; note testnet vs mainnet and any rule contradictions.
```

### 15.3 Google Gemini (rubric, deck, Q&A)

```text
Project: Combined Kraken + EIP-8004 trading agent. Rust core with risk gates, signed intents, logs, Svelte dashboard. Target: win both execution (PnL) and trust (risk-adjusted + validation) narratives.

Tasks:
1. Map each official judging criterion to a 20-second demo beat and on-screen artifact.
2. 5-slide story: problem (trust triangle) → hybrid architecture → safety (sandbox + risk) → live demo → metrics.
3. Q&A: fake trades vs real CLI, partial trustlessness with CEX, model failure modes, registry costs/latency, reproducibility.
4. One architecture diagram description (identity → intent → validation → Kraken → reputation loop).

Constraints: Acknowledge current repo uses paper execution until adapter is swapped; combined submission must show path to real Kraken + on-chain validation.
```

### 15.4 Single context block (any tool)

```text
hack01: Rust hexagonal trading-agent, SvelteKit dashboard, GET /logs, Chart.js.
run_once: market → decide → apply_risk → build_intent → sign → execute → log.
AGENT_DECISION: momentum|claude|adk|hybrid. Risk: drawdown, confidence, position rules.
Hackathon: Kraken challenge (CLI, AI, build in public, PnL) + ERC-8004 challenge (identity registry, reputation, validation artifacts, sandbox vault/risk router, risk-adjusted ranking). Combined = EIP-8004 trust + Kraken execution + proof-of-decision trail.
Spec: eips.ethereum.org/EIPS/eip-8004
```

---

## 16. Winning playbook (short)

1. **Do not fake execution** if rules require real Kraken trades; use paper only when allowed and label it.  
2. **Optimize ERC-8004 track** for drawdown and validation clarity, not only raw return.  
3. **One diagram** in deck: hybrid stack + bridge.  
4. **Treat AI as untrusted**; lead demo with **risk blocked** case, then **approved** case.  
5. **Build in public** on schedule required by Kraken track.

---

## 17. Document control

| Version | Date | Notes |
|---------|------|--------|
| 1.0 | 2026-03-31 | Initial tree-based report |
| 2.0 | 2026-03-31 | Source of truth: mission, two-track hackathon, EIP-8004 + Kraken synergy, gaps, phased plan, prompts |

---

*Update this file when organizers publish final addresses, rubrics, or CLI/sandbox API versions.*
