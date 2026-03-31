# Hackathon project report — trading agent & dashboard

**Workspace:** `/Users/bm/hack01`  
**Primary deliverables:** `trading-agent` (Rust), `trading-dashboard` (SvelteKit), local `adk-rust` integration for LLM-assisted modes.  
**Stated hackathon context (from crate metadata):** LabLab / Kraken-oriented CLI and agent shell.

---

## 1. Vision

Build a **credible, inspectable autonomous trading agent** that:

- Connects to **Kraken** market data and **paper-style execution** suitable for demos and judging.
- Supports **multiple decision backends** (rules, Claude, Google ADK, hybrid) behind one clean domain flow.
- Exposes a **live audit trail** (HTTP JSON logs) and a **dashboard** so judges and teammates see reasoning, PnL, and drawdown—not a black box.

Long-term vision: a **template for responsible agentic finance**—risk gates first, signed intents, reproducible logs, and swappable “brains” without rewriting the core loop.

---

## 2. Mission (hackathon scope)

| Objective | Success signal |
|-----------|----------------|
| **Working demo loop** | Agent ticks on a schedule, fetches OHLC/price, decides, executes (paper), logs. |
| **Explainability** | Each tick has action, confidence, reasoning, risk overrides, and performance snapshot in logs. |
| **Differentiation** | At least one **LLM or ADK path** (`AGENT_DECISION=claude|adk|hybrid`) plus rule baseline (`momentum`). |
| **Polished story** | Dashboard charts + narrative: strategy, risk policy, and failure modes. |
| **Operational clarity** | Env-driven config, single binary, documented ports and endpoints. |

---

## 3. Product context (what the code actually does)

1. **Market:** `KrakenMarket` loads pair data and OHLC (interval and lookback configurable via env).
2. **Decision:** Pluggable driver—momentum/volatility rules, Claude, ADK, or hybrid (rules + ADK).
3. **Risk:** Domain layer applies drawdown cap, min confidence for trades, and single-position rules before execution.
4. **Intent & signing:** `build_intent` + `SimpleSigner` produce auditable signed artifacts (demo-oriented).
5. **Execution:** `KrakenPaperExecution` simulates fills and balance updates for the performance tracker.
6. **Observability:** Axum server serves **`GET /logs`**; the Svelte app polls and visualizes PnL/drawdown with Chart.js.

This is intentionally **demo- and learning-first**: emphasize transparency, safety rails, and architecture over unverifiable live-alpha claims.

---

## 4. Technical stack

| Layer | Technology | Role |
|-------|------------|------|
| Agent runtime | Rust 2021, Tokio | Async tick loop, concurrency |
| HTTP API | Axum, tower-http (CORS) | Log endpoint for dashboard |
| LLM / agents | `adk-rust` (path dep), optional Claude adapter | ADK / hybrid / Claude decision paths |
| Serialization / time | serde, serde_json, chrono | Intents, logs, timestamps |
| Config | dotenvy, env vars | 12-factor style tuning without recompiles |
| Frontend | SvelteKit 2, Vite 6, TypeScript strict | Dashboard SPA |
| UI charts | Chart.js 4 | PnL and drawdown series |
| Exchange | Kraken (REST-style adapters in code) | Market + paper execution |

**Package managers (team norms):** `pnpm` for Node, `cargo` for Rust (per project preferences).

---

## 5. Repository layout (high signal)

```
trading-agent/          # Hexagonal trading agent binary
  src/
    main.rs             # Composition root: wiring, HTTP server, main loop
    domain/             # Pure logic: strategy, risk, model, intent
    ports/              # Traits: Decision, Market, Execution, etc.
    application/        # TradingAgent orchestration, intent_builder
    adapters/           # Kraken, HTTP logs, momentum, ADK, Claude, hybrid, signer, validation
trading-dashboard/      # SvelteKit app polling /logs
adk-rust/               # Local ADK Rust crate (agent features)
claude-code-best-practice/  # Optional reference material for tooling workflows
```

---

## 6. Design patterns & architecture

### 6.1 Hexagonal architecture (ports & adapters)

- **Ports** (`src/ports/`): `DecisionPort`, `MarketPort`, `ExecutionPort`, `ValidationPort`, `SignerPort`, `PerformancePort`—framework-free interfaces.
- **Adapters** (`src/adapters/`): Kraken, HTTP logging, concrete decision engines, etc.
- **Application** (`TradingAgent`): Single `run_once` pipeline—fetch → decide → risk → intent → sign → execute → update position → validate/log.

**Why it wins in a hackathon:** judges can see **clean boundaries**; you can swap Kraken for mock data or swap LLM for rules in minutes.

### 6.2 Strategy / driver pattern for decisions

`DecisionDriver` enum in `main` selects implementation without changing `TradingAgent` generics usage at the type level beyond the chosen concrete `D`.

Modes (env `AGENT_DECISION`): `momentum` (default), `claude`, `adk`, `hybrid`.

### 6.3 Domain-driven risk policy

`domain/risk.rs` centralizes **policy** (drawdown, confidence floor, position constraints) separate from **signal generation**—clear story for “safe agent” narrative.

### 6.4 Application service

`TradingAgent::run_once` is the **transaction script** for one tick: orchestration only, no exchange details.

### 6.5 Observability adapter

`http_logs` decouples **persistence/streaming of decisions** from core logic; dashboard depends only on the log schema.

### 6.6 Frontend: Svelte 5 runes

`$state` for logs and canvas refs; `onMount` / `onDestroy` for polling and Chart.js lifecycle—aligns with strict TS + modern Svelte.

---

## 7. Implementation approach (how to extend without thrash)

1. **Preserve the pipeline:** New features should plug in as new adapters or domain functions, not scattered `if demo` branches in `run_once`.
2. **Version the log JSON:** If you add fields, keep backward compatibility or bump a `schema_version` in each row for the dashboard.
3. **Env as feature flags:** `AGENT_DECISION`, risk thresholds, intervals—document in README for judges.
4. **Demo path:** Run agent + dashboard on known ports; use recorded logs as fallback if venue API is flaky.
5. **Pitch alignment:** Lead with **risk gates + audit trail**, then **multi-brain** (rules vs LLM), then **live charts**.

---

## 8. Key environment variables (agent)

| Variable | Purpose |
|----------|---------|
| `AGENT_PAIR` | Trading pair (default `BTCUSD`) |
| `AGENT_VOLUME` / intent sizing | Order size string |
| `AGENT_ID` | Agent identity in intents |
| `AGENT_SIGNING_KEY` | Demo signing secret |
| `AGENT_INITIAL_BALANCE` | Performance tracker starting equity |
| `AGENT_HTTP_PORT` | Log server port (default `3030`) |
| `AGENT_DECISION` | `momentum` \| `claude` \| `adk` \| `hybrid` |
| `AGENT_INTERVAL_SECS` | Tick period |
| `AGENT_OHLC_INTERVAL`, `AGENT_MOMENTUM_LOOKBACK` | Market history shaping |
| `AGENT_STRATEGY_*`, `AGENT_RISK_*`, `AGENT_MAX_DRAWDOWN` | Strategy and risk tuning |

---

## 9. Code reference map (anchor files)

| Concern | Location |
|---------|----------|
| Composition root & loop | `trading-agent/src/main.rs` |
| Tick orchestration | `trading-agent/src/application/agent.rs` |
| Rule-based strategy | `trading-agent/src/domain/strategy.rs` |
| Risk policy | `trading-agent/src/domain/risk.rs` |
| Intent construction | `trading-agent/src/application/intent_builder.rs` |
| Dashboard + charts | `trading-dashboard/src/routes/+page.svelte` |

---

## 10. Prompts for tooling (copy-paste)

Use these as **system or first messages** depending on the product’s UI. Replace `{JUDGING_CRITERIA}` with the hackathon’s published rubric when available.

### 10.1 Claude Code (implementation pair-programmer)

```text
You are working in a Rust + SvelteKit monorepo for a hackathon trading agent.

Constraints:
- Rust: edition 2021, Tokio, anyhow in the binary, hexagonal layout: domain/, ports/, application/, adapters/.
- Do not break the TradingAgent::run_once pipeline: market → decision → risk → intent → sign → execute → log.
- New behavior: prefer new adapter modules or domain functions; keep ports as traits.
- Frontend: Svelte 5 runes ($state), TypeScript strict, pnpm only.
- After edits, run cargo check in trading-agent and pnpm check in trading-dashboard when relevant.

Task: {DESCRIBE_FEATURE_OR_BUG}

Files to respect: trading-agent/src/main.rs, application/agent.rs, domain/*, ports/*, adapters/*, trading-dashboard/src/routes/+page.svelte.

Deliver: minimal diff, list env vars if added, and a one-paragraph judge-facing explanation of the change.
```

### 10.2 Perplexity (research, links, competitive landscape)

```text
Research for a hackathon project: autonomous crypto trading agent with Kraken integration, paper trading, risk gates (drawdown, confidence, position limits), audit logs, and optional LLM/ADK decision layer. Stack: Rust hexagonal architecture, SvelteKit dashboard.

Please find and summarize:
1. Kraken REST/WebSocket best practices for OHLC and rate limits (official docs preferred).
2. Recent hackathon winners or fintech demos where "explainable AI" or audit trails were differentiators.
3. Legal/compliance talking points for demo-only / paper trading (no investment advice).
4. Any open-source Rust trading or exchange client patterns we should align with.

Output: bullet summary + full URLs for each source. Flag contradictions between sources.
```

### 10.3 Google Gemini (synthesis, rubric alignment, deck outline)

```text
Context: Our project is a Rust hexagonal "trading agent" with pluggable decisions (rules, Claude, Google ADK hybrid), Kraken market + paper execution, risk layer, signed intents, HTTP JSON logs, and a SvelteKit + Chart.js dashboard.

Hackathon goal: maximize judging impact in {TIME_LIMIT} minutes.

Tasks:
1. Given rubric items: {PASTE_RUBRIC}, map each item to a specific demo beat (what we show on screen, what we say in 15 seconds).
2. Propose a 5-slide storyline: problem → architecture → safety → live demo → roadmap.
3. List 5 sharp Q&A answers judges might ask (overfitting, live money, API keys, model cost, failure modes).
4. Suggest one optional visual (diagram description) we could add to the repo README or deck.

Be concrete; refer to components: TradingAgent run_once, risk apply_risk, GET /logs, dashboard charts.
```

### 10.4 Cross-tool “single context block” (paste into any tool)

```text
Project: hack01 — trading-agent (Rust) + trading-dashboard (SvelteKit 5).
Architecture: Hexagonal — ports in src/ports, adapters in src/adapters, orchestration in application/agent.rs, pure logic in domain/.
Decision modes (env AGENT_DECISION): momentum | claude | adk | hybrid.
Risk: drawdown cap, min confidence for trades, single long position semantics.
Observability: Axum GET /logs consumed by dashboard; Chart.js for PnL/drawdown.
Hackathon angle: explainable agentic trading with safety rails and swappable brains, Kraken-oriented, paper execution for demo safety.
```

---

## 11. Winning playbook (concise)

1. **Demo reliability:** Prefer paper + logged scenario; preflight dashboard against a static JSON if network fails.
2. **Narrative:** “Safety first, intelligence optional”—risk module before any “AI trader” claim.
3. **Architecture slide:** One diagram—ports/adapters + `run_once` sequence.
4. **Honesty:** State limits (simulation, not financial advice, model latency/cost).
5. **Rubric fit:** Explicitly tick each judging criterion in the final 60 seconds.

---

## 12. Document control

| Version | Date | Notes |
|---------|------|--------|
| 1.0 | 2026-03-31 | Generated from current `hack01` tree; align rubric and sponsor names when announced |

---

*This report is a living artifact: update Section 2 and Section 10 when official hackathon rules, sponsors, or judging criteria are published.*
