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
| **I** | Programs, venues, timelines | `early-surge-*.md`, LabLab paste file |
| **II** | Kraken challenge — tooling & narrative | `kraken-cli-announcement.md` + local `kraken-cli/` |
| **III** | ERC-8004 — spec, debate, tutorials | `lablab-*-technology-access.md`, Magicians, Medium, `awesome-erc8004/` |
| **IV** | Market data / resolution partner | `strykr-*.md` |
| **V** | Workspace mechanics | `.gitignore`, this file |
| **VI** | Way ahead | Bottom of this document |

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
| [`extracted/lablab-ai-trading-agents-technology-access.md`](extracted/lablab-ai-trading-agents-technology-access.md) | Official-style requirements: registries, EIP-712/1271/155, Risk Router, vault, leaderboard metrics. |
| [`extracted/medium-savvysid-erc-8004-trustless-agent-layer.md`](extracted/medium-savvysid-erc-8004-trustless-agent-layer.md) | Tutorial article: three registries, Solidity/JS sketches, ROFL/TEE workflow **caveats**. |
| [`extracted/ethereum-magicians-erc-8004-25098.md`](extracted/ethereum-magicians-erc-8004-25098.md) | Forum debate: on-chain composability, payments vs scope, Agent Card URL/domain, singleton registry intent. |

**Local clone (ignored):** `awesome-erc8004/` — curated links; `git -C awesome-erc8004 pull`.

**Deep dive:** Read **[EIP-8004](https://eips.ethereum.org/EIPS/eip-8004)** and the **[Ethereum Magicians](https://ethereum-magicians.org/t/erc-8004-trustless-agents/25098)** thread on the live site (drafts change). *Optional:* add a future `extracted/eip-8004-snapshot.md` if you want a trimmed on-repo spec digest.

---

## Part IV — Strykr / PRISM (technology partner angle)

| File | One-line purpose |
|------|------------------|
| [`extracted/strykr-platform-overview.md`](extracted/strykr-platform-overview.md) | Consumer Strykr product: web, iOS, AI agent pillar, signals/news narrative. |
| [`extracted/strykr-prism-product-overview.md`](extracted/strykr-prism-product-overview.md) | PRISM API: resolve/family/venues, endpoint groups, pricing tier snapshot, MCP mention. |

**Deep dive:** Official PRISM docs + dashboard; treat hackathon promo/credits as **time-boxed** and verify vendor terms.

---

## Part V — Repository layout (tracked vs local)

```
hack01/
├── .gitignore              # kraken-cli/, awesome-erc8004/
├── docs/
│   ├── HACKATHON-CORPUS.md # this index
│   └── extracted/          # nine topic extractions (see glob)
├── kraken-cli/             # local only (ignored)
└── awesome-erc8004/        # local only (ignored)
```

---

## Part VI — Way ahead (suggested path)

1. **Lock scope:** Kraken-only, ERC-8004-only, or **combined** submission; match the **exact** hackathon row you joined (Surge vs LabLab dates/prizes).
2. **Consent / risk UX:** If you revisit the earlier “consent button” idea, gate “connect keys / start agent” behind explicit acknowledgements + link-out to Kraken/Surge privacy terms (no secrets in repo).
3. **Kraken path:** Install CLI from clone → paper mode → minimal autonomous loop (signals → decision → `kraken` JSON) → document read-only key handling.
4. **ERC-8004 path:** Read current EIP + hackathon Risk Router docs → identity JSON template → testnet deploy checklist → validation/reputation event indexing plan.
5. **Data layer:** If using PRISM, prototype `resolve` + one price/signal endpoint behind an interface your agent can mock in tests.
6. **Corpus hygiene:** When organizers update copy, refresh the relevant `extracted/*.md` and commit with a short “why” (your standing rule).

---

*Last indexed: 2026-03-31. Amend this file when you add chapters.*
