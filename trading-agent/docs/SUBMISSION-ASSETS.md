# Submission Assets Checklist

> Everything needed to submit "ERC-8004 Trading Agent" on lablab.ai

## Registration (Prize Eligibility)

- [ ] Register project on **early.surge.xyz** (required for prizes)
- [ ] Enroll team on **lablab.ai** hackathon page
- [ ] Connect email or social account on Surge to claim 10 Ignites

## GitHub Repository

- [ ] Repo name: `erc8004-trading-agent`
- [ ] Make repo **public** on https://github.com/zixelfreelance
- [ ] Clean README with: project description, architecture, quick start, screenshots
- [ ] MIT license file

## lablab.ai Submission Form

### Basic Information
- [ ] **Project Title**: ERC-8004 Trading Agent
- [ ] **Short Description**: Autonomous AI trading agent that proves compliance on-chain via ERC-8004 — it cannot break its own risk rules
- [ ] **Long Description**: Full write-up covering problem, solution, architecture, tech stack, risk controls, ERC-8004 integration, results
- [ ] **Technology & Category Tags**: ERC-8004, Kraken CLI, Rust, Solidity, SvelteKit, AI Trading

### Visual Assets
- [x] **Cover Image** (16:9, high-res) → `docs/cover.png`
- [ ] **Screenshot: Dashboard** — price chart with Bollinger Bands, trade markers
- [ ] **Screenshot: Risk block** — trade blocked by risk gate (visible in trade log)
- [ ] **Screenshot: Terminal** — agent running in demo mode with JSON output
- [ ] **Screenshot: Agent Card** — `curl localhost:3030/.well-known/agent-card.json`

### Video Presentation (3-5 min)
- [ ] Record with OBS Studio or QuickTime (macOS: Cmd+Shift+5)
- [ ] **Script:**
  1. (0:00-0:30) Intro — who you are, project name, one-line pitch
  2. (0:30-1:30) Problem — AI agents trade with zero accountability
  3. (1:30-2:30) Demo — run `AGENT_DEMO_MODE=true cargo run`, show terminal output
  4. (2:30-3:30) Dashboard — open browser, show charts, PnL, trade log, risk blocks
  5. (3:30-4:00) ERC-8004 — show agent card, identity registry, reputation flow
  6. (4:00-4:30) Architecture — show diagram (slide or README)
  7. (4:30-5:00) Closing — business value, what's next, call to action
- [ ] Upload to YouTube (unlisted or public)

### Slide Presentation (7-10 slides)
- [ ] **Slide 1** — Title: "ERC-8004 Trading Agent" + tagline + hackathon name
- [ ] **Slide 2** — Problem: AI agents trade with no identity, no audit trail, no limits
- [ ] **Slide 3** — Solution: Constrained AI in a hard risk envelope, proves compliance on-chain
- [ ] **Slide 4** — Architecture diagram: Agent → Risk Gates → Risk Router → On-chain
- [ ] **Slide 5** — Tech Stack: Rust, SvelteKit, 3 Solidity contracts, Kraken CLI, ERC-8004
- [ ] **Slide 6** — Risk Controls: 10 mechanisms (position limits, drawdown breaker, daily loss cap...)
- [ ] **Slide 7** — Demo screenshot: dashboard with charts + blocked trades
- [ ] **Slide 8** — Results: 70 tests, 45+ Rust files, 3 contracts, full demo mode
- [ ] **Slide 9** — Business Value: regulatory-aligned, auditable, trustless
- [ ] **Slide 10** — Team: @zixelfreelance (GitHub) · @zixlancer (X)
- [ ] Tool: Google Slides / Canva / Keynote
- [ ] Export as PDF for upload

### App Hosting
- [ ] **Demo URL** — deploy dashboard (options: Vercel, Fly.io, Railway)
- [ ] **Application URL** — live link to the running agent or dashboard

## Social Engagement (Kraken Prize)

- [ ] First post on X from @zixlancer — announce the project
- [ ] Tag @krakenfx @lablabai @Surgexyz_ on every post
- [ ] Share: build progress, screenshots, architecture decisions, demo clips
- [ ] Post on LinkedIn with same content

## Challenge-Specific

### ERC-8004 Track
- [ ] Agent Identity registered on Identity Registry
- [ ] Reputation accumulation from trade outcomes
- [ ] Validation artifacts for trade intents, risk checks
- [ ] Risk Router execution (sandbox vault)

### Kraken Track (if entering)
- [ ] Kraken CLI integration for trade execution
- [ ] Read-only API key for leaderboard verification
- [ ] Live PnL during competition window

## Deadline

**April 12, 2026 — 7:30 PM IRST (≈16:00 UTC)**
