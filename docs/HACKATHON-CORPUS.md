# AI Trading Agents — research corpus (mini index)

**Workspace:** `hack01` — pair-coding / learning vault: **curated extractions**, **local clones** (not tracked), and **git history** of decisions.

---

## Executive summary

This repo is a **structured notebook** for the **LabLab “AI Trading Agents”** era: two parallel technical tracks (**Kraken CLI** execution vs **ERC-8004** trust layer), anchored by **Surge** discovery/hackathon listings and optional **Strykr / PRISM** market-resolution APIs. We captured official and community sources as **markdown extractions** under `docs/extracted/` so you can skim fast, cite links, and drill into originals. **Kraken CLI** and **awesome-erc8004** live as **sibling git clones** at the repo root; they are **gitignored** here so this project stays a thin index, not a fork of upstream history.

**What this is not:** legal advice, a submission repo, or a guarantee that dates/prizes match every banner. Always reconcile **Surge**, **LabLab**, and **EIP** text on the live web.

---

## Table of contents (how to read this “mini book”)

| Part | Topic | Go deep via |
|------|--------|-------------|
| **I** | Programs, venues, timelines | `early-surge-*.md` |
| **II** | Kraken challenge — tooling & narrative | `kraken-cli-announcement.md` + local `kraken-cli/` |
| **III** | ERC-8004 — spec, debate, tutorials | `eip-8004-trustless-agents-snapshot.md`, `lablab-*`, Magicians, Medium, `awesome-erc8004/` |
| **IV** | Market data / resolution partner | `strykr-*.md` |
| **V** | Workspace mechanics | `.gitignore`, **Appendix B** |
| **VI** | Way ahead (expanded) | [§ Part VI](#part-vi--way-ahead-expanded) |
| **VII** | Official EIP digest | [`extracted/eip-8004-trustless-agents-snapshot.md`](extracted/eip-8004-trustless-agents-snapshot.md) |
| **Appendix A** | Complete file manifest | [§ Appendix A](#appendix-a--complete-file-manifest-tracked) |
| **Appendix B** | Folder diagram (expanded) | [§ Appendix B](#appendix-b--folder-diagram-expanded) |
| **Appendix C** | Map to “Battle Plan” HTML | [§ Appendix C](#appendix-c--map-to-hackathon-battle-plan-html-q2-2026) |

---

## Part I — Surge & hackathon context

| File | One-line purpose |
|------|------------------|
| [`extracted/early-surge-landing.md`](extracted/early-surge-landing.md) | Home snapshot: featured hackathon banner, Surge Picks, trending, journey map. |
| [`extracted/early-surge-hackathons.md`](extracted/early-surge-hackathons.md) | `/hackathons` list: **Live** AI Trading Agents (dates/pool), **The Rise of AI Agents**, past editions. |
| [`extracted/early-surge-discovery.md`](extracted/early-surge-discovery.md) | `/discovery` hub: filters, LabLab-labeled listing sample, pagination. |

**Deep dive:** Compare rows in `early-surge-hackathons.md` to your actual enrollment (two different April windows and prize figures have appeared across surfaces).

---

## Part II — Kraken track

| File | One-line purpose |
|------|------------------|
| [`extracted/kraken-cli-announcement.md`](extracted/kraken-cli-announcement.md) | Kraken blog TL;DR: MCP, paper trading, NDJSON, install one-liner, risk lines + **local clone note**. |

**Local clone (ignored):** `kraken-cli/` — `gh repo clone krakenfx/kraken-cli`; update with `git -C kraken-cli pull`.

**Deep dive:** Read upstream `README.md`, `DISCLAIMER.md`, and `kraken mcp` in that tree; align agent design to **read-only keys** for leaderboard rules.

---

## Part III — ERC-8004 track

| File | One-line purpose |
|------|------------------|
| [`extracted/eip-8004-trustless-agents-snapshot.md`](extracted/eip-8004-trustless-agents-snapshot.md) | **Official EIP digest:** identifiers, registration JSON, identity/reputation/validation APIs, rationale, security — **diff vs live EIP** before ship. |
| [`extracted/lablab-ai-trading-agents-technology-access.md`](extracted/lablab-ai-trading-agents-technology-access.md) | Hackathon-style requirements: Risk Router, vault, EIP-712/1271/155, leaderboard metrics. |
| [`extracted/medium-savvysid-erc-8004-trustless-agent-layer.md`](extracted/medium-savvysid-erc-8004-trustless-agent-layer.md) | Tutorial article: three registries, Solidity/JS sketches, ROFL/TEE workflow **caveats**. |
| [`extracted/ethereum-magicians-erc-8004-25098.md`](extracted/ethereum-magicians-erc-8004-25098.md) | Forum debate: on-chain composability, payments vs scope, Agent Card URL/domain, singleton registry intent. |

**Local clone (ignored):** `awesome-erc8004/` — curated links; `git -C awesome-erc8004 pull`.

**Deep dive order:** 1) [EIP-8004](https://eips.ethereum.org/EIPS/eip-8004) live → 2) on-repo **snapshot** → 3) LabLab tech access → 4) Magicians thread → 5) Medium tutorial as **non-normative**.

---

## Part IV — Strykr / PRISM (technology partner angle)

| File | One-line purpose |
|------|------------------|
| [`extracted/strykr-platform-overview.md`](extracted/strykr-platform-overview.md) | Consumer Strykr product: web, iOS, AI agent pillar, signals/news narrative. |
| [`extracted/strykr-prism-product-overview.md`](extracted/strykr-prism-product-overview.md) | PRISM API: resolve/family/venues, endpoint groups, pricing tier snapshot, MCP mention. |

**Deep dive:** Official PRISM docs + dashboard; treat hackathon promo/credits as **time-boxed** and verify vendor terms.

---

## Part V — Repository layout (summary)

Tracked tree lives under **`.git`**, **`.gitignore`**, and **`docs/`**. Local clones populate **`kraken-cli/`** and **`awesome-erc8004/`** but are excluded from this parent repo. See **Appendix B** for the full filename-level diagram.

---

## Part VI — Way ahead (expanded)

### Phase 0 — Lock the job to be done

- **Pick one primary story:** Kraken-only, ERC-8004-only, or **combined** (plus optional PRISM).
- **Reconcile enrollment** against Surge `/hackathons` + LabLab event page (dates, prize copy, eligibility such as Surge project registration).
- **Risk & consent:** define what users must acknowledge before API keys, wallet connect, or “paper vs live” toggles; link to vendor terms (no credentials in git).

### Phase 1 — Kraken execution spine

- Run **`kraken-cli`** from local clone: config, **`kraken mcp`**, **paper** trading path.
- Ship a **minimal closed loop:** ingest signal (static fixture → later live) → policy stub → CLI invocation with **`-o json`** → structured logs.
- Document **read-only key** posture for leaderboard verification; never commit keys.

### Phase 2 — ERC-8004 trust spine

- Validate **registration JSON** against EIP-8004 `registration-v1` expectations (services, `supportedTrust`, `registrations`).
- Map hackathon **vault / Risk Router** flows onto EIP concepts (trade intents, validation artifacts — use organizer docs when available).
- Plan **indexer** or subgraph for `Registered`, `NewFeedback`, `ValidationRequest` / `ValidationResponse` (names per deployed interfaces).

### Phase 3 — Integration & demo narrative

- Optional **PRISM** adapter module with interface + mocks for tests.
- **Build-in-public** artifacts aligned to hackathon social rules (tags, Surge profile, etc.).
- **Submission checklist:** repo, demo URL, video/slides per LabLab guidelines.

### Phase 4 — Corpus & spec hygiene

- When Surge/LabLab/Kraken/EIP text moves, **patch the matching `extracted/*.md`** and commit with explicit **reason / effect / purpose**.
- Re-run a **diff** of [`eip-8004-trustless-agents-snapshot.md`](extracted/eip-8004-trustless-agents-snapshot.md) against [EIP-8004](https://eips.ethereum.org/EIPS/eip-8004) before milestone reviews.

### Risks to track

| Risk | Mitigation |
|------|------------|
| EIP still **Draft** | Pin discussion + commit date in snapshots; watch ERCs PR merges. |
| Conflicting hackathon dates/pools | Single **source of truth** row copied into your README when you freeze scope. |
| Key leakage | Env-only secrets, `.env` in `.gitignore`, pre-commit scan habits. |
| Over-scoping | Time-box Phase 1 loop before on-chain work. |

---

## Part VII — EIP-8004 official snapshot (on repo)

Full digest: **[`docs/extracted/eip-8004-trustless-agents-snapshot.md`](extracted/eip-8004-trustless-agents-snapshot.md)** — abstract, motivation, global ID, registration JSON, three registries’ **functions/events** at a glance, rationale, security, citation. **Always** verify against the live EIP before implementation.

---

## Appendix A — Complete file manifest (tracked)

All paths relative to repo root `hack01/`.

| Path | Purpose |
|------|---------|
| `.gitignore` | Excludes `kraken-cli/`, `awesome-erc8004/` sibling clones from parent history. |
| `docs/HACKATHON-CORPUS.md` | This index: TOC, parts I–VII, appendices, expanded roadmap. |
| `docs/extracted/early-surge-landing.md` | Surge home capture. |
| `docs/extracted/early-surge-hackathons.md` | Surge hackathons index capture. |
| `docs/extracted/early-surge-discovery.md` | Surge discovery hub capture. |
| `docs/extracted/kraken-cli-announcement.md` | Kraken CLI blog extraction + local clone note. |
| `docs/extracted/lablab-ai-trading-agents-technology-access.md` | LabLab Technology & Access (dual challenge) extraction. |
| `docs/extracted/medium-savvysid-erc-8004-trustless-agent-layer.md` | Medium ERC-8004 tutorial extraction. |
| `docs/extracted/ethereum-magicians-erc-8004-25098.md` | Ethereum Magicians thread extraction. |
| `docs/extracted/eip-8004-trustless-agents-snapshot.md` | **EIP-8004 structured snapshot** (draft digest). |
| `docs/extracted/strykr-platform-overview.md` | Strykr consumer product capture. |
| `docs/extracted/strykr-prism-product-overview.md` | PRISM API / pricing capture. |

**Not tracked (present locally when cloned):**

| Path | Purpose |
|------|---------|
| `kraken-cli/` | Upstream [krakenfx/kraken-cli](https://github.com/krakenfx/kraken-cli). |
| `awesome-erc8004/` | Upstream [sudeepb02/awesome-erc8004](https://github.com/sudeepb02/awesome-erc8004). |

---

## Appendix B — Folder diagram (expanded)

```text
hack01/
├── .git/
├── .gitignore                 # kraken-cli/, awesome-erc8004/
├── docs/
│   ├── HACKATHON-CORPUS.md    # mini-book index (this file)
│   └── extracted/
│       ├── early-surge-landing.md
│       ├── early-surge-hackathons.md
│       ├── early-surge-discovery.md
│       ├── kraken-cli-announcement.md
│       ├── lablab-ai-trading-agents-technology-access.md
│       ├── medium-savvysid-erc-8004-trustless-agent-layer.md
│       ├── ethereum-magicians-erc-8004-25098.md
│       ├── eip-8004-trustless-agents-snapshot.md
│       ├── strykr-platform-overview.md
│       └── strykr-prism-product-overview.md
├── kraken-cli/                # gitignored — separate repository
└── awesome-erc8004/           # gitignored — separate repository
```

---

## Appendix C — Map to “Hackathon Battle Plan” HTML (Q2 2026)

External one-pager title: *Hackathon Battle Plan — Q2 2026* (five-target roadmap, monorepo diagram, timeline, judge cheat sheet). **`hack01` is not that monorepo**; it is the **research corpus + local tool clones** for the **LabLab / Surge / Kraken / ERC-8004** thread.

| HTML `h2` / block | What it is | How **`hack01`** maps today | Gap |
|-------------------|------------|-----------------------------|-----|
| **§1 — The Five Targets** | Five hackathon cards | **Primary:** **Card #1 — AI Trading Agents** (lablab × Surge × Kraken). Extractions + `kraken-cli/` + `awesome-erc8004/` + EIP snapshot **directly support** that card. **Cards #2–#5** (Four.Meme, Solana Frontier, Ontology, Arc×Circle) are **not** represented in this repo’s files. | Add separate repos or monorepo root per battle plan; do not pretend `hack01` covers BNB/Solana/Ontology/Arc unless you add artifacts. |
| **§2 — Shared Codebase Architecture** | `packages/agent-brain`, `agent-identity`, `chain-adapters`, `apps/*` | **Out of scope.** Battle plan describes a **TypeScript monorepo**; `hack01` only has **`docs/`** + **ignored** upstream clones. | Initialize `hackathon-q2` (or rename) elsewhere, or fold corpus into that monorepo as `docs/research/`. |
| **§3 — Execution Timeline** | Day-by-day checklist | **Reference only.** No automation here; timeline lives in your HTML/PM tool. | Track tasks in issues; link to this corpus for specs. |
| **§4 — What Judges Want** | Per-hackathon table | **Row: AI Trading Agents** ↔ [`lablab-ai-trading-agents-technology-access.md`](extracted/lablab-ai-trading-agents-technology-access.md), Surge extracts, Kraken blog, EIP snapshot. **Caution:** the HTML card mixes **“risk-adjusted / Sharpe / drawdown / ERC-8004”** into one list; **official** LabLab copy splits **Kraken track (net PnL + social)** vs **ERC-8004 track (risk-adjusted, drawdown, validation)**. Reconcile before you optimize metrics. | Pull latest judging text from lablab + Surge; avoid a single PnL story if you compete on both tracks. |
| **§5 — lablab.ai Universal Scoring** | Four pillars | **Conceptual overlap** with how you pitch #1/#5 on LabLab; this repo does **not** yet store a dedicated “judging criteria” extract beyond Technology & Access. | Optional: add `extracted/lablab-judging-criteria.md` from the event page. |
| **§6 — Submission Checklist** | Universal deliverables | **Orthogonal** — process checklist, not stored as a file here (could mirror into `docs/SUBMISSION-CHECKLIST.md` later). | — |
| **§7 — Risk Map** | Spread, Solana time, etc. | **Partial overlap** with [Part VI risks](#part-vi--way-ahead-expanded) in this corpus (EIP draft, date drift, keys). HTML adds **multi-hackathon triage** — keep in battle plan, not necessarily in `hack01`. | — |
| **§8 — Next Steps** | Registrations, turbo scaffold | **Local clone piece:** “Kraken CLI” ↔ `kraken-cli/` + [`kraken-cli-announcement.md`](extracted/kraken-cli-announcement.md). **Monorepo** ↔ not in `hack01` yet. | Run registrations outside git; never commit secrets. |

**One-line answer:** **`hack01` targets the *intel + tooling footnotes* for HTML §1 Card #1 and the ERC-8004 / Kraken portions of §4’s first row — not §2’s monorepo, not hacks #2–#5, not the timeline/checklists themselves.**

---

*Last indexed: 2026-03-31. Amend this file when you add chapters or extractions.*
