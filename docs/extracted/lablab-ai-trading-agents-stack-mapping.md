# Hackathon three pillars → your stack (ADK-Rust · Claude · Kraken CLI · SvelteKit)

**Stack assumption:** Rust orchestration with **[adk-rust](https://github.com/zavora-ai/adk-rust)** (graph or loop), **Anthropic** model via **ADK’s `AnthropicClient`** (recommended), **Kraken CLI** for data + **paper** execution (`-o json`), **SvelteKit** for a thin audit UI reading logs/API.

**Note:** Prefer **Anthropic API** inside ADK over **Claude Code CLI** subprocess for demos — stable JSON, lower latency, fewer parse failures. Use CLI only if you accept extra fragility.

---

## 1. Can your agent handle capital? (financial actor, not chatbot)

| Sub-ask | What it means | Your implementation |
|--------|----------------|----------------------|
| **Trade** | Real or simulated orders | `kraken paper init` → `kraken paper buy/sell … -o json`; optional live `kraken order …` only with keys + rules you understand |
| **Manage risk** | Limits, stops, caps | **Rust / ADK custom node:** max position, max daily loss, max trades/day, min confidence, cooldown after loss — **before** calling `kraken paper` |
| **Optimize yield** | Don’t force alpha genius | Conservative: reduce churn, fees awareness (paper applies taker fee), prefer HOLD when uncertain — **document** the policy |
| **Protect funds** | Show governance over greed | **Demo moment:** Claude proposes `BUY` → risk engine **blocks** → final `HOLD` + logged reason |

**One-liner for judges:** *Capital flows only through Kraken CLI (paper or live); Rust policy can veto any trade.*

---

## 2. Can it act autonomously? (decision-making agent, not assistive)

| Sub-ask | What it means | Your implementation |
|--------|----------------|----------------------|
| **Reads data** | No manual refresh | Scheduled loop (e.g. `tokio::interval`) or ADK workflow step calling `kraken ticker` / `ohlc` / `orderbook` **`-o json`** |
| **Makes decisions** | Policy + LLM | **ADK `LlmAgent`** with a **strict JSON schema** prompt (BUY/SELL/HOLD, confidence, reason, risk_note) |
| **Executes without human input** | No click-to-trade | Same loop: fetch → LLM proposal → risk → **automatic** `kraken paper …` or skip — no UI gate in the hot path |

**One-liner for judges:** *A single long-running process (or ADK graph) runs the full cycle without human approval per tick.*

---

## 3. Can you prove it behaves correctly? (most important)

| Sub-ask | What it means | Your implementation |
|--------|----------------|----------------------|
| **Logs** | Audit trail | **JSONL** (or SQLite) per tick: market snapshot, raw LLM output, risk verdict, CLI request/response, PnL snapshot |
| **Validation artifacts** | Third party could replay | Stable schema + timestamps + hashes optional; **ERC-8004 upgrade path:** same payload → signed intent / registry event when infra is ready |
| **Transparent reasoning** | Not black box | Store Claude’s `reason`, `risk_note`, `invalid_if` in log + show in **SvelteKit** timeline |

**Killer demo row:** *Proposed action* vs *Risk allowed?* vs *Final action* vs *PnL/drawdown snapshot*.

**One-liner for judges:** *Every tick is reproducible from logs; governance overrides are first-class events.*

---

## End-to-end flow (matches all three pillars)

```text
Kraken CLI (ticker/ohlc, -o json)
       → ADK: Market / Fetch node
       → ADK: LlmAgent (Claude) → structured proposal
       → ADK: Risk node (Rust rules) → allow | block | resize
       → Kraken CLI: paper buy/sell OR no-op
       → Logger: append JSONL (+ optional expose via small HTTP API)
       → SvelteKit: read API / tail log → timeline + metrics
```

---

## What to skip (scope control)

- RAG, browser tools, voice, multi-agent soup — **not** required for these three pillars  
- **Full** ERC-8004 on day one — add **thin** identity/signature only if core loop + demo are done  

---

## Related docs in this repo

- [`lablab-ai-trading-agents-essentials.md`](lablab-ai-trading-agents-essentials.md)  
- [`lablab-ai-trading-agents-rules-research.md`](lablab-ai-trading-agents-rules-research.md)  
- [`kraken-cli-announcement.md`](kraken-cli-announcement.md) + local `kraken-cli/` README (paper, `-o json`, MCP)  

---

*Planning note only. Kraken CLI [DISCLAIMER](https://github.com/krakenfx/kraken-cli/blob/main/DISCLAIMER.md) applies to real funds.*
