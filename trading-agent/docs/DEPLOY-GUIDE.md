# Step-by-Step Deployment Guide

> Cost: $0 for phases 1, 2, 4. Only phase 3 (live trading) requires ~$50.

---

## Phase 1 — Foundation (free)

### Step 1.1: Install Foundry

```bash
curl -L https://foundry.paradigm.xyz | bash
```

Close and reopen your terminal (or `source ~/.zshrc`), then:

```bash
foundryup
```

Verify:

```bash
forge --version
# Should print: forge 0.x.x
```

### Step 1.2: Compile & Test Contracts

```bash
cd /Users/bm/hack01/trading-agent/contracts
forge build
forge test -vvv
```

You should see 13 tests passing.

### Step 1.3: Get a Free Alchemy RPC URL

1. Go to https://dashboard.alchemy.com
2. Sign up (free, no credit card)
3. Click **Create New App**
4. Name: `proof-of-trust` | Chain: **Ethereum** | Network: **Sepolia**
5. Click **Create App** → click the app → copy the **HTTPS** URL

It looks like: `https://eth-sepolia.g.alchemy.com/v2/abc123xyz`

### Step 1.4: Create a Deployer Wallet

If you already have MetaMask with a Sepolia wallet, export its private key. Otherwise:

```bash
cast wallet new
```

This prints an address + private key. Save both. **Never share the private key.**

### Step 1.5: Get Free Sepolia ETH

Go to one of these faucets and paste your wallet address:

- https://cloud.google.com/application/web3/faucet/ethereum/sepolia (Google, no signup)
- https://www.alchemy.com/faucets/ethereum-sepolia (Alchemy, already signed up)

You need ~0.01 Sepolia ETH. It arrives in 1-2 minutes.

Verify:

```bash
cast balance YOUR_ADDRESS --rpc-url YOUR_ALCHEMY_URL
```

### Step 1.6: Deploy Contracts to Sepolia

```bash
cd /Users/bm/hack01/trading-agent/contracts

export DEPLOYER_PRIVATE_KEY=0xYOUR_PRIVATE_KEY
export RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY

./deploy.sh
```

The script prints 3 contract addresses. Copy them:

```
AgentIdentityRegistry: 0x...
AgentReputationRegistry: 0x...
RiskRouter: 0x...
```

### Step 1.7: Create `.env` File

```bash
cd /Users/bm/hack01/trading-agent

cat > .env << 'EOF'
# Core
AGENT_PAIR=BTCUSD
AGENT_VOLUME=0.001
AGENT_EXECUTION_MODE=paper
AGENT_DECISION=momentum
AGENT_HTTP_PORT=3030
AGENT_INTERVAL_SECS=10

# On-chain (paste your addresses from Step 1.6)
CHAIN_ID=11155111
CHAIN_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY
IDENTITY_REGISTRY=0xYOUR_ADDRESS
REPUTATION_REGISTRY=0xYOUR_ADDRESS

# Signing key (generate one)
AGENT_SIGNING_KEY=0xYOUR_DEPLOYER_PRIVATE_KEY
EOF
```

### Step 1.8: Verify Agent Runs with Chain Config

```bash
cargo run
```

You should see:

```
mode: paper | pair: BTCUSD | volume: 0.001 | chain: configured
```

`chain: configured` confirms contracts are wired.

---

## Phase 2 — Deploy (free)

### Step 2.1: Push Code to GitHub

```bash
cd /Users/bm/hack01/trading-agent

# If repo already exists on GitHub:
git remote add origin git@github.com:zixelfreelance/erc8004-trading-agent.git
git push -u origin main
```

If the repo doesn't exist yet, create it at https://github.com/new:
- Name: `erc8004-trading-agent`
- Public
- Don't initialize with README (you already have one)

### Step 2.2: Deploy Dashboard to Vercel (5 min)

1. Go to https://vercel.com and sign in with GitHub
2. Click **Add New... → Project**
3. Import `erc8004-trading-agent`
4. Configure:
   - **Root Directory:** `ui`
   - **Framework Preset:** SvelteKit (auto-detected)
   - **Build Command:** `pnpm build` (auto-detected)
5. Under **Environment Variables**, add:
   - `VITE_LOGS_URL` = `http://127.0.0.1:3030` (placeholder — update after agent deploy)
6. Click **Deploy**
7. Copy the Vercel URL (e.g. `https://erc8004-trading-agent.vercel.app`)

### Step 2.3: Deploy Agent Backend to Railway (free tier)

**Why Railway:** Free tier gives 500 hours/month, auto-detects Rust, no Dockerfile needed.

1. Go to https://railway.com and sign in with GitHub
2. Click **New Project → Deploy from GitHub Repo**
3. Select `erc8004-trading-agent`
4. Railway auto-detects Rust and builds with `cargo build --release`

**Important:** The build will fail because of the `adk-rust` local path dependency. Fix it first:

```bash
# Option A: Copy adk-rust into the repo for CI builds
cp -r /Users/bm/adk-rust /Users/bm/hack01/trading-agent/adk-rust-vendor
```

Then update `Cargo.toml`:

```toml
# Change this line:
adk-rust = { path = "../adk-rust/adk-rust", ... }
# To:
adk-rust = { path = "adk-rust-vendor/adk-rust", ... }
```

5. In Railway dashboard, go to **Variables** tab and add all `.env` vars from Step 1.7
6. Go to **Settings → Networking → Generate Domain** to get a public URL
7. Copy the Railway URL (e.g. `https://proof-of-trust-agent.up.railway.app`)
8. Go back to Vercel → Settings → Environment Variables → update `VITE_LOGS_URL` to your Railway URL
9. Redeploy on Vercel

**Alternative — Run agent locally for demo:**

If you just need the agent running during your demo recording, skip Railway entirely. Run `cargo run` locally and point the dashboard to `localhost:3030`.

### Step 2.4: Verify End-to-End

Open your Vercel URL in a browser. You should see:
- Metrics bar (ticks, executed, blocked, holds, errors)
- Price chart with Bollinger Bands
- PnL chart
- Trade log table

---

## Phase 3 — Live Trading ($50-100)

### Step 3.1: Friend Creates Kraken Account

1. Go to https://www.kraken.com → **Create Account**
2. Complete **identity verification** (KYC) — takes minutes to hours
3. Deposit $50-100 via bank transfer, card, or crypto

### Step 3.2: Friend Generates API Keys

In Kraken web UI:

1. Go to **Security → API**
2. Create **trading key**:
   - Name: `trading-agent`
   - Permissions: Query Funds, Query Open Orders & Trades, Query Closed Orders & Trades, Create & Modify Orders
   - Click **Generate Key**
   - Copy the **Key** and **Private Key** (shown once)
3. Create **read-only key** (for leaderboard submission):
   - Name: `leaderboard-readonly`
   - Permissions: Query Funds, Query Open Orders & Trades, Query Closed Orders & Trades only
   - Copy these too

### Step 3.3: Configure Kraken CLI

```bash
kraken auth set
```

Paste the **trading** API key when prompted, then the secret.

Verify:

```bash
kraken balance
```

Should show your friend's USD balance.

### Step 3.4: Test with Minimum Volume

```bash
cd /Users/bm/hack01/trading-agent
AGENT_EXECUTION_MODE=live AGENT_VOLUME=0.0001 cargo run
```

0.0001 BTC ~ $7 per trade. Watch the first few ticks to confirm fills are real.

### Step 3.5: Monitor

- Dashboard: http://localhost:3030
- Kraken paper history: `kraken paper history` (won't show live trades)
- Kraken real history: `kraken trades --pair BTCUSD`
- Agent logs: `tail -f trades.log`

### Step 3.6: Let It Run

Once confirmed working, let it accumulate PnL for the leaderboard. The agent has 10 risk controls built in — it won't blow up the account.

---

## Phase 4 — Polish (free)

### Step 4.1: Record Demo Video

Open two terminal windows + browser:

```bash
# Terminal 1: run agent
cd /Users/bm/hack01/trading-agent
cargo run

# Terminal 2: query endpoints
curl -s http://localhost:3030/metrics | jq
curl -s http://localhost:3030/.well-known/agent-card.json | jq
```

Open http://localhost:3030 in browser for the dashboard.

Record with **Cmd+Shift+5** → select **Record Entire Screen** or a window. Talk through:
1. What the agent does (30s)
2. Show terminal with live ticks (30s)
3. Show dashboard charts (30s)
4. Show agent card endpoint (20s)
5. Explain architecture + risk controls (60s)
6. Show on-chain identity if deployed (30s)

Export and upload to YouTube (public or unlisted).

### Step 4.2: Social Posts

From @zixlancer on X:

> Building a trustless AI trading agent for the @lablabai hackathon
> Proof-of-Trust: ERC-8004 agent identity + cryptographic audit trail
> @krakenfx @Surgexyz_
> [screenshot of dashboard]

### Step 4.3: Submit on lablab.ai

Fill the submission form with:
- **GitHub:** https://github.com/zixelfreelance/erc8004-trading-agent
- **Website:** https://zixelfreelance.github.io/erc8004-trading-agent/
- **Video:** your YouTube URL
- **Dashboard:** your Vercel URL
- **Read-only Kraken API key:** the one from Step 3.2

---

## Quick Reference

| What | Command |
|---|---|
| Run paper mode | `cargo run` |
| Run demo mode | `AGENT_DEMO_MODE=true cargo run` |
| Run live mode | `AGENT_EXECUTION_MODE=live AGENT_VOLUME=0.0001 cargo run` |
| Dashboard | http://localhost:3030 |
| Agent card | http://localhost:3030/.well-known/agent-card.json |
| Metrics | http://localhost:3030/metrics |
| Trade log | http://localhost:3030/logs |
| Kraken balance | `kraken balance` |
| Paper balance | `kraken paper balance` |
| Compile contracts | `cd contracts && forge build` |
| Test contracts | `cd contracts && forge test -vvv` |

## Deadline: April 12, 2026 — 7:30 PM IRST (16:00 UTC)
