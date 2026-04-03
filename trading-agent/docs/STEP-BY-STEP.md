# Step-by-Step Instructions — Remaining Tasks

> Complete these before April 12, 2026 7:30 PM IRST (16:00 UTC)

---

## 1. Push Code to GitHub

```bash
cd /Users/bm/hack01/trading-agent
git remote add origin https://github.com/zixelfreelance/erc8004-trading-agent.git
git branch -M main
git add -A
git commit -m "feat: initial commit — Proof-of-Trust Trading Agent"
git push -u origin main
```

---

## 2. Deploy Dashboard to Get a Public Demo URL

### Backend (Rust Agent) → Railway

1. Go to https://railway.app and sign in with GitHub
2. Click **"New Project"** → **"Deploy from GitHub Repo"**
3. Select `zixelfreelance/erc8004-trading-agent`
4. Railway will detect the Rust project. Set these environment variables:
   ```
   AGENT_DEMO_MODE=true
   AGENT_HTTP_PORT=3030
   PORT=3030
   ```
5. In **Settings → Networking**, click **"Generate Domain"** — you'll get something like:
   ```
   erc8004-trading-agent-production.up.railway.app
   ```
6. Wait for the build to complete (~3-5 min for Rust)
7. Test: visit `https://YOUR-RAILWAY-DOMAIN/metrics` — should return JSON

### Frontend (SvelteKit Dashboard) → Vercel

1. Go to https://vercel.com and sign in with GitHub
2. Click **"Add New Project"** → import `zixelfreelance/erc8004-trading-agent`
3. Set **Root Directory** to `ui`
4. Set **Framework Preset** to `SvelteKit`
5. Add environment variable:
   ```
   VITE_LOGS_URL=https://YOUR-RAILWAY-DOMAIN
   ```
   (Replace with the Railway domain from step above)
6. Click **Deploy**
7. You'll get a URL like: `https://erc8004-trading-agent.vercel.app`
8. Visit it — you should see the dashboard pulling data from Railway backend

### Alternative: Vercel-Only (No Backend)

If Railway gives trouble, deploy a static snapshot:

1. Run the agent locally in demo mode, let it complete 50 ticks
2. Save the JSON responses:
   ```bash
   curl http://localhost:3030/logs > ui/static/demo-logs.json
   curl http://localhost:3030/metrics > ui/static/demo-metrics.json
   ```
3. Edit `ui/src/routes/+page.svelte` — change the fetch URLs to load from `/demo-logs.json` and `/demo-metrics.json` as fallback
4. Deploy to Vercel as above (no backend needed)

---

## 3. Post on X — First Tweet

1. Open https://x.com/compose/tweet
2. Copy-paste this (fits in 280 chars):
   ```
   Building a trustless AI trading agent for @lablabai hackathon

   The AI can't break its own rules:
   → ERC-8004 on-chain identity
   → 7 enforced risk gates
   → EIP-712 signed intents

   AI proposes. Contracts enforce.

   @krakenfx @Surgexyz_ #AITradingAgents
   ```
3. Attach the cover image (`docs/cover.png`) or a dashboard screenshot
4. Click **Post**
5. Copy the tweet URL (click the tweet → copy from browser bar)
6. Save the URL — you'll need it for the submission form

---

## 4. Post on LinkedIn — Announcement

1. Open https://www.linkedin.com/feed/
2. Click **"Start a post"**
3. Paste this:
   ```
   I'm building a trustless AI trading agent for the lablab.ai
   AI Trading Agents hackathon ($55K prize pool).

   The core idea: AI is untrusted input. The agent proposes —
   the system decides.

   What I built:
   • On-chain agent identity via ERC-8004 (ERC-721 NFT)
   • 7 enforced risk gates the AI cannot bypass
   • EIP-712 signed trade intents for every decision
   • Real-time dashboard with Bollinger Bands, PnL, and drawdown
   • Cryptographic audit trail — not logging, proof

   Tech: Rust, Solidity, SvelteKit, Claude AI, Kraken CLI

   The thesis: the winning AI trading agents won't have the best
   returns — they'll be the ones you can verify and trust.

   GitHub: https://github.com/zixelfreelance/erc8004-trading-agent

   #AI #DeFi #Trading #ERC8004 #Hackathon #Rust #Solidity
   ```
4. Attach the cover image or dashboard screenshot
5. Click **Post**
6. Copy the LinkedIn post URL

---

## 5. Connect Surge Email/Social to Claim Ignites

1. Go to https://early.surge.xyz
2. Login with admin / JBRv2xWG7AzwVrLz88
3. Click your profile icon (top right)
4. Go to **Settings** or **Profile**
5. Add your **email address** or connect **X/Twitter** (@zixlancer)
6. This unlocks your 10 Ignites for use in the Discovery section

---

## 6. Friend Tasks — Kraken Account, API Keys, Multi-Sig

### Send this message to your friend:

```
Hey! I need your help with the hackathon registration. Here's what to do:

STEP 1: Create Kraken Account (15 min)
1. Go to https://www.kraken.com/sign-up
2. Create account with your email
3. Complete KYC verification (ID + selfie)
4. Wait for approval (usually same day)

STEP 2: Generate API Keys (2 min)
1. Go to Settings → API → Create Key
2. Create KEY 1 — "Trading":
   - Enable: "Create & Modify Orders", "Query Open Orders & Trades",
     "Query Closed Orders & Trades", "Query Ledger Entries"
   - Disable: "Withdraw Funds" (IMPORTANT — keep this off)
   - Save the API key + secret — send them to me securely
3. Create KEY 2 — "Read-Only":
   - Enable: "Query" permissions only
   - Disable everything else
   - Save — this one goes to the hackathon leaderboard

STEP 3: Fund Account (2 min)
1. Deposit $50-100 USD (or USDT/USDC)
2. This is for live trading during the competition
3. The agent has strict risk limits ($5 daily max loss, 5% drawdown cap)

STEP 4: Multi-Sig Wallet for Prizes (5 min)
1. Go to https://app.safe.global
2. Connect your wallet (MetaMask or similar)
3. Select network: Base (or Ethereum)
4. Click "Create Safe"
5. Add 2 signers:
   - Your wallet address
   - My wallet address: [SEND YOUR ADDRESS]
6. Set threshold: 2 of 2
7. Deploy the Safe
8. Send me the Safe address

STEP 5: Submit on lablab.ai (Day 11-12, I'll guide you)
1. I'll send you the final submission details
2. You'll click "Submit" on the hackathon page
3. Paste the read-only API key when prompted
```

### After receiving API keys from friend:

1. Store them securely (NOT in git):
   ```bash
   # Create a .env file (already in .gitignore)
   cd /Users/bm/hack01/trading-agent
   cat > .env << 'EOF'
   KRAKEN_API_KEY=your-trading-key-here
   KRAKEN_API_SECRET=your-trading-secret-here
   KRAKEN_READONLY_KEY=your-readonly-key-here
   KRAKEN_READONLY_SECRET=your-readonly-secret-here
   EOF
   ```
2. Verify `.env` is in `.gitignore`:
   ```bash
   grep ".env" .gitignore || echo ".env" >> .gitignore
   ```

---

## 7. Smart Contract Deployment — Foundry + Base Sepolia

### Step 1: Install Foundry

```bash
curl -L https://foundry.paradigm.xyz | bash
source ~/.zshrc
foundryup
```

Verify: `forge --version`

### Step 2: Get Base Sepolia ETH (testnet)

1. Get a wallet with a private key (MetaMask export or generate new):
   ```bash
   cast wallet new
   ```
   Save the private key securely.

2. Get testnet ETH from faucet:
   - Go to https://www.coinbase.com/faucets/base-ethereum-goerli-faucet
   - Or https://faucet.quicknode.com/base/sepolia
   - Enter your wallet address
   - You need ~0.01 ETH for deployments

### Step 3: Deploy Contracts

```bash
cd /Users/bm/hack01/trading-agent/contracts

# Set your private key (don't commit this!)
export PRIVATE_KEY=0xyour_private_key_here
export RPC_URL=https://sepolia.base.org

# Deploy AgentIdentityRegistry
forge create --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \
  src/AgentIdentityRegistry.sol:AgentIdentityRegistry \
  --broadcast
# Note the "Deployed to:" address → IDENTITY_REGISTRY_ADDRESS

# Deploy AgentReputationRegistry
forge create --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \
  src/AgentReputationRegistry.sol:AgentReputationRegistry \
  --broadcast
# Note the "Deployed to:" address → REPUTATION_REGISTRY_ADDRESS

# Deploy RiskRouter (pass identity registry address as constructor arg if needed)
forge create --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \
  src/RiskRouter.sol:RiskRouter \
  --broadcast
# Note the "Deployed to:" address → RISK_ROUTER_ADDRESS
```

### Step 4: Register Agent Identity

```bash
# Mint an agent identity NFT
cast send $IDENTITY_REGISTRY_ADDRESS \
  "registerAgent(string)" \
  "https://YOUR-DOMAIN/.well-known/agent-card.json" \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY
```

### Step 5: Update Configuration

1. Update `contracts/agent-card.json`:
   ```
   Replace "0xIDENTITY_REGISTRY_ADDRESS" with actual address
   Replace chain ID with 84532 (Base Sepolia)
   ```

2. Update agent environment:
   ```bash
   # Add to .env
   IDENTITY_REGISTRY=0xYOUR_IDENTITY_REGISTRY_ADDRESS
   REPUTATION_REGISTRY=0xYOUR_REPUTATION_REGISTRY_ADDRESS
   RISK_ROUTER=0xYOUR_RISK_ROUTER_ADDRESS
   CHAIN_ID=84532
   ```

### Step 6: Verify Contracts on Basescan (optional but impressive)

```bash
forge verify-contract $IDENTITY_REGISTRY_ADDRESS \
  src/AgentIdentityRegistry.sol:AgentIdentityRegistry \
  --chain base-sepolia \
  --etherscan-api-key YOUR_BASESCAN_API_KEY
```

Get a free API key at https://basescan.org/apis

---

## 8. Kraken CLI Setup — Once API Keys Arrive

### Step 1: Install Kraken CLI

```bash
# Option A: From GitHub (recommended)
git clone https://github.com/krakenfx/kraken-cli.git
cd kraken-cli
cargo install --path .

# Option B: Direct cargo install (if published)
cargo install kraken-cli
```

### Step 2: Configure API Access

```bash
# Set up Kraken CLI config
kraken-cli config set api-key YOUR_TRADING_API_KEY
kraken-cli config set api-secret YOUR_TRADING_API_SECRET
```

Or use environment variables:
```bash
export KRAKEN_API_KEY=your-trading-key
export KRAKEN_API_SECRET=your-trading-secret
```

### Step 3: Test Paper Trading

```bash
# Check balance
kraken-cli balance

# Check BTC/USD ticker
kraken-cli ticker XBTUSD

# Test paper order (if supported)
kraken-cli order --pair XBTUSD --type buy --volume 0.0001 --price 65000 --validate
# --validate flag = paper trade, no real execution
```

### Step 4: Connect to Agent

Update `.env`:
```bash
AGENT_EXECUTION_MODE=paper   # Start with paper
KRAKEN_API_KEY=your-key
KRAKEN_API_SECRET=your-secret
```

Run the agent:
```bash
cd /Users/bm/hack01/trading-agent
cargo run
```

### Step 5: Switch to Live (when ready)

```bash
AGENT_EXECUTION_MODE=live AGENT_VOLUME=0.0001 cargo run
```

Start with **tiny volume** (0.0001 BTC = ~$6.50). Monitor the dashboard. Scale up only after confirming it works.

---

## 9. Update Submission with Final URLs

### Step 1: Gather All URLs

| Asset | URL |
|-------|-----|
| GitHub | https://github.com/zixelfreelance/erc8004-trading-agent |
| YouTube | https://www.youtube.com/watch?v=7zc0qDvCOKo |
| Demo URL | https://YOUR-VERCEL-URL.vercel.app |
| X Post 1 | https://x.com/zixlancer/status/XXXXXXXXX |
| LinkedIn | https://linkedin.com/posts/XXXXXXXXX |
| Surge | https://early.surge.xyz/discovery/proofoftrust-agent |

### Step 2: Update lablab.ai Submission

1. Go to https://lablab.ai/ai-hackathons/ai-trading-agents/proof-of-trust/submission
2. Update these fields:
   - **Demo Application Platform**: Vercel (or Other)
   - **Demo Application URL**: your Vercel dashboard URL
   - **Social Media Post Link 1**: your X tweet URL
   - **Social Media Post Link 2**: your LinkedIn post URL
   - **Social Media Post Link 3**: your YouTube video URL
3. Click **Save** (or Submit if final)

### Step 3: Update Surge Project

1. Go to https://early.surge.xyz → your project
2. Update:
   - **Website URL**: Vercel dashboard URL
   - **YouTube pitch URL**: https://www.youtube.com/watch?v=7zc0qDvCOKo
   - Upload pitch deck PDF if not done

### Step 4: Final Submission (April 11-12)

1. Ensure all fields are complete on lablab.ai
2. Have friend provide read-only Kraken API key for leaderboard
3. Double-check video is public on YouTube
4. Click **Submit** before April 12, 7:30 PM IRST

---

## Priority Order (What to Do First)

| Priority | Task | Time | Blocked By |
|----------|------|------|------------|
| **P0** | Push code to GitHub | 5 min | Nothing |
| **P0** | Post on X + LinkedIn | 10 min | Nothing |
| **P1** | Deploy dashboard (Vercel) | 30 min | Code pushed |
| **P1** | Message friend re: Kraken | 5 min | Nothing |
| **P1** | Connect Surge email | 2 min | Nothing |
| **P2** | Install Foundry | 5 min | Nothing |
| **P2** | Deploy contracts | 30 min | Foundry + testnet ETH |
| **P3** | Kraken CLI setup | 20 min | Friend's API keys |
| **P3** | Live trading | Ongoing | Kraken CLI + API keys |
| **LAST** | Final submission update | 15 min | All above |

---

## Deadline: April 12, 2026 — 7:30 PM IRST (16:00 UTC)
## Days Remaining: 10
