# Submission Assets Checklist

> Everything needed to submit "ERC-8004 Trading Agent" on lablab.ai
> Last updated: 2026-04-05

## Registration (Prize Eligibility)

- [x] Register project on **early.surge.xyz** (required for prizes)
- [x] Enroll team on **lablab.ai** hackathon page
- [ ] Connect email or social account on Surge to claim 10 Ignites

## GitHub Repository

- [x] Repo name: `erc8004-trading-agent`
- [x] Make repo **public** on https://github.com/zixelfreelance
- [x] Clean README with: project description, architecture, quick start, screenshots
- [x] MIT license file

## lablab.ai Submission Form

### Basic Information
- [x] **Project Title**: Proof-of-Trust Trading Agent
- [x] **Short Description**: Autonomous AI trading agent that proves compliance on-chain via ERC-8004 — it cannot break its own risk rules
- [x] **Long Description**: `docs/LABLAB-LONG-DESCRIPTION.md` — ready to paste into submission form
- [x] **Technology & Category Tags**: ERC-8004, Kraken CLI, Rust, Solidity, SvelteKit, AI Trading, IPFS, EIP-712

### Visual Assets
- [x] **Cover Image** (16:9, high-res) → `docs/cover.png`
- [ ] **Screenshot: Dashboard** — price chart with Bollinger Bands, trade markers (need backend running)
- [x] **Screenshot: Terminal** — agent running in demo mode with JSON output → `docs/screenshots/`
- [x] **Screenshot: Agent Card** — `docs/screenshots/agent-card-output.json`
- [x] **Screenshot: Metrics** — `docs/screenshots/metrics-output.json`

### Video Presentation (3-5 min)
- [x] Video overview uploaded to YouTube — https://www.youtube.com/watch?v=7zc0qDvCOKo
- [ ] Consider re-recording with live demo showing chain integration + IPFS pinning

### Slide Presentation
- [x] **Pitch deck PDF** — via Gamma (docs/Proof-of-Trust Trading Agent — Pitch Deck.pdf)
- [x] **Slides HTML** — docs/slides.html (backup)

### App Hosting
- [x] **Dashboard URL** — https://erc8004-trading-agent.vercel.app (Vercel)
- [x] **Backend URL** — https://trading-agent-95p9.onrender.com (Render)
- [x] **Docs URL** — https://zixelfreelance.github.io/erc8004-trading-agent/ (GitHub Pages)
- [ ] ⚠️ Wake Render backend (currently 503 — free tier idle)

## Social Engagement (Kraken Prize)

- [ ] First post on X from @zixlancer — announce the project
- [ ] Tag @krakenfx @lablabai @Surgexyz_ on every post
- [ ] Share: build progress, screenshots, architecture decisions, demo clips
- [ ] Post on LinkedIn with same content

## Challenge-Specific

### ERC-8004 Track
- [x] Agent Identity adapter — calls registry on Sepolia
- [x] Reputation accumulation — posts PnL feedback on-chain every 100 ticks
- [x] Validation artifacts — per-trade IPFS pinning with CID backfill
- [x] Risk Router — submits signed intents on-chain, returns tx_hash
- [ ] Mint agent identity (ERC-721) — run agent with CHAIN_RPC_URL + AGENT_SIGNING_KEY

### Kraken Track (if entering)
- [x] Kraken CLI integration for trade execution (paper + live modes)
- [ ] Read-only API key for leaderboard verification (needs friend)
- [ ] Live PnL during competition window (needs API key)

## Deadline

**April 12, 2026 — 7:30 PM IRST (≈16:00 UTC) — 7 days remaining**
