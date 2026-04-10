# Live Mode Checkpoint — 2026-04-03

> Priority-ordered checklist: demo → live. Tackle top-down.

## Status

| Component | State | Blocker |
|---|---|---|
| Rust agent (paper/demo) | PASS | — |
| mdBook site | PASS (GitHub Pages) | — |
| YouTube video | Uploaded | — |
| Pitch deck (PDF + HTML) | Done | — |
| Foundry installed | DONE | — |
| Smart contracts compiled | DONE (13/13 tests) | — |
| Alchemy RPC URL | DONE (Sepolia) | — |
| Deployer wallet | DONE (0x5F55...E4d3) | — |
| Contracts deployed (Sepolia) | DONE (3 contracts) | — |
| `.env` created | DONE | — |
| `agent-card.json` updated | DONE | — |
| Code pushed to GitHub | DONE (subtree push) | — |
| Live trading | Not started | Kraken API key ($50) |
| IPFS pinning | Stub | Pinata API key |
| Dashboard deploy | Not started | Vercel (instructions below) |
| Render account | Created | Not configured yet |

## Deployed Contract Addresses (Sepolia)

| Contract | Address |
|---|---|
| AgentIdentityRegistry | `0xc83F0B94E7969Cc2265aB0A187Ba0F2e6A5B9554` |
| AgentReputationRegistry | `0x40dB57F7D848457289CEda81F39df15C4203D576` |
| RiskRouter | `0xCbC5DFeD364b6D65233DfA6edCcb95088F8f189B` |

---

## Next: Deploy Dashboard to Vercel

1. Go to https://vercel.com → **Add New... → Project**
2. Click **Import** next to `erc8004-trading-agent`
3. Configure:
   - **Root Directory:** click **Edit** → type `ui` → confirm
   - **Framework Preset:** SvelteKit (auto-detected)
   - **Build Command:** leave default (`vite build`)
4. Expand **Environment Variables**, add:
   - Key: `VITE_LOGS_URL`
   - Value: `http://127.0.0.1:3030`
5. Click **Deploy**
6. Wait ~1 min → copy the Vercel URL (e.g. `https://trading-dashboard-675072986521.us-central1.run.app`)

> Note: The dashboard will show "connection error" when your local agent isn't running.
> That's fine — during demo you run `cargo run` locally and it connects.

---

## Next: Kraken API Key (blocks live PnL + leaderboard)

- [ ] Friend creates Kraken account + completes KYC
- [ ] Friend generates **trading API key** (Create & Modify Orders)
- [ ] Friend generates **read-only API key** (Query only — for leaderboard)
- [ ] Friend funds account with $50-100
- [ ] Configure CLI: `kraken auth set` → paste key + secret
- [ ] Verify: `kraken balance`
- [ ] Test paper: `AGENT_EXECUTION_MODE=paper cargo run`
- [ ] Go live: `AGENT_EXECUTION_MODE=live AGENT_VOLUME=0.0001 cargo run`

---

## Next: IPFS Pinning (cryptographic audit trail)

- [ ] Create free Pinata account: https://pinata.cloud
- [ ] Generate API keys
- [ ] Add to `.env`:
  ```
  PINATA_API_KEY=...
  PINATA_API_SECRET=...
  ```
- [ ] Verify: run agent, check artifacts appear on IPFS

---

## Next: Render (agent backend hosting — optional)

- [ ] Log into https://dashboard.render.com
- [ ] New → Web Service → Connect `erc8004-trading-agent`
- [ ] Build Command: `cargo build --release`
- [ ] Start Command: `./target/release/trading-agent`
- [ ] Add env vars from `.env`
- [ ] Note: needs `adk-rust` vendored into repo first

> Skip this if running agent locally during demo is good enough.

---

## Final Submission (April 12, 16:00 UTC)

- [ ] Fill lablab.ai submission form:
  - GitHub: https://github.com/zixelfreelance/erc8004-trading-agent
  - Website: https://zixelfreelance.github.io/erc8004-trading-agent/
  - Video: https://youtu.be/7zc0qDvCOKo
  - PDF deck: on site
  - Dashboard: Vercel URL
- [ ] Submit read-only Kraken API key for leaderboard
- [ ] Social post from @zixlancer tagging @krakenfx @lablabai @Surgexyz_

---

## Environment Variables (complete live config)

```bash
# Execution
AGENT_EXECUTION_MODE=live
AGENT_VOLUME=0.0001
AGENT_DECISION=hybrid
AGENT_INITIAL_BALANCE=100       # match actual Kraken balance

# AI
ANTHROPIC_API_KEY=sk-ant-...

# Signing & Chain
AGENT_SIGNING_KEY=0x85b2e0101ed734b7a4f3600fb16b473dcfc979c617d666faaacb2ab2a22a5ad7
CHAIN_ID=11155111
CHAIN_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/jELZANLWh9IjCBwyUHguo
IDENTITY_REGISTRY=0xc83F0B94E7969Cc2265aB0A187Ba0F2e6A5B9554
REPUTATION_REGISTRY=0x40dB57F7D848457289CEda81F39df15C4203D576

# IPFS
PINATA_API_KEY=...
PINATA_API_SECRET=...
```

## Quick Commands

| What | Command |
|---|---|
| Run paper mode | `cargo run` |
| Run demo mode | `AGENT_DEMO_MODE=true cargo run` |
| Run live mode | `AGENT_EXECUTION_MODE=live AGENT_VOLUME=0.0001 cargo run` |
| Dashboard | http://localhost:3030 |
| Agent card | http://localhost:3030/.well-known/agent-card.json |
| Push to GitHub | `cd /Users/bm/hack01 && git subtree push --prefix=trading-agent origin main` |

## Deadline: April 12, 2026 — 7:30 PM IRST (16:00 UTC)
