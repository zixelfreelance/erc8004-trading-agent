# Live Mode Checkpoint — 2026-04-01

> Priority-ordered checklist: demo → live. Tackle top-down.

## Status

| Component | State | Blocker |
|---|---|---|
| Rust agent (paper/demo) | PASS | — |
| mdBook site | PASS (GitHub Pages) | — |
| YouTube video | Uploaded | — |
| Pitch deck (PDF + HTML) | Done | — |
| Smart contracts (3) | Not compiled | Foundry |
| Live trading | Not started | Kraken API key |
| IPFS pinning | Stub | Pinata API key |
| Dashboard deploy | Not started | Vercel |

---

## 1. Kraken API Key (blocks live PnL + leaderboard)

- [ ] Friend creates Kraken account + completes KYC
- [ ] Friend generates **trading API key** (Create & Modify Orders)
- [ ] Friend generates **read-only API key** (Query only — for leaderboard)
- [ ] Friend funds account with $50-100
- [ ] Install Kraken CLI: `cargo install kraken-cli`
- [ ] Configure CLI: `kraken auth set` → paste key + secret
- [ ] Verify: `kraken balance`
- [ ] Test paper: `AGENT_EXECUTION_MODE=paper cargo run`
- [ ] Go live: `AGENT_EXECUTION_MODE=live AGENT_VOLUME=0.0001 cargo run`

## 2. Smart Contract Deployment (ERC-8004 track)

- [ ] Install Foundry: `curl -L https://foundry.paradigm.xyz | bash && foundryup`
- [ ] Compile: `cd contracts && forge build`
- [ ] Run tests: `forge test -vvv` (13 tests)
- [ ] Get Sepolia ETH from faucet
  - https://cloud.google.com/application/web3/faucet/ethereum/sepolia
  - https://www.alchemy.com/faucets/ethereum-sepolia
- [ ] Get RPC URL from Alchemy (free): https://dashboard.alchemy.com
- [ ] Deploy:
  ```bash
  export DEPLOYER_PRIVATE_KEY=0x...
  export RPC_URL=https://eth-sepolia.g.alchemy.com/v2/KEY
  ./deploy.sh
  ```
- [ ] Save 3 contract addresses
- [ ] Set `IDENTITY_REGISTRY`, `REPUTATION_REGISTRY` env vars
- [ ] Mint agent identity (ERC-721)
- [ ] Update agent-card.json with real addresses
- [ ] Set `CHAIN_RPC_URL` and `CHAIN_ID=11155111`

## 3. IPFS Pinning (cryptographic audit trail)

- [ ] Create free Pinata account: https://pinata.cloud
- [ ] Generate API keys
- [ ] Set `PINATA_API_KEY` and `PINATA_API_SECRET`
- [ ] Verify: run agent, check artifacts appear on IPFS

## 4. Dashboard Deployment (Vercel)

- [ ] `cd ui && pnpm install && pnpm build`
- [ ] Go to https://vercel.com → Import `erc8004-trading-agent`
- [ ] Root Directory: `ui`, Framework: SvelteKit
- [ ] Set env var: `VITE_LOGS_URL=https://YOUR_AGENT_HOST:3030`
- [ ] Deploy → copy public URL

## 5. Final Submission (April 12, 16:00 UTC)

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
AGENT_SIGNING_KEY=0x...         # hex private key for EIP-712
CHAIN_ID=11155111
CHAIN_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/KEY
IDENTITY_REGISTRY=0x...
REPUTATION_REGISTRY=0x...

# IPFS
PINATA_API_KEY=...
PINATA_API_SECRET=...
```

## Minimum Viable Demo (if short on time)

```
1. cargo run                              ← paper mode (works now)
2. open http://localhost:3030             ← dashboard (works now)
3. curl localhost:3030/.well-known/agent-card.json  ← ERC-8004 (works now)
4. Deploy UI to Vercel                    ← 5 min
5. Record screen + upload                 ← 15 min
```

Paper mode with real prices is a valid demo. Contracts + live trading are bonus.

## Deadline: April 12, 2026 — 7:30 PM IRST (16:00 UTC)
