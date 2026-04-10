# Your Action Steps — Submission Countdown

> Step-by-step instructions for everything only YOU can do.
> Generated: April 5, 2026. Deadline: April 12, 7:30 PM IRST.

---

## TODAY (April 5) — Do These First

### Step 1: Push to GitHub
```bash
cd /Users/bm/hack01/trading-agent
git push origin main
```
This triggers Render auto-redeploy with the MockExecution fix. The "Failed deploy" / 71 errors should be gone.

### Step 2: Wake Render Backend
After push, visit: https://trading-agent-675072986521.us-central1.run.app/metrics

- Free tier takes 1-2 min to spin up on first hit
- You should see JSON with `"ticks":0, "errors":0`
- If still 503 after 3 min, go to Render dashboard and click "Manual Deploy"

### Step 3: Set Vercel Env Var
1. Go to https://vercel.com → project `erc8004-trading-agent` → Settings → Environment Variables
2. Add: `VITE_LOGS_URL` = `https://trading-agent-675072986521.us-central1.run.app`
3. Click "Redeploy" (Production)
4. Visit https://trading-dashboard-675072986521.us-central1.run.app — dashboard should show live data

### Step 4: Take Dashboard Screenshot
Once dashboard shows data (price chart + trade log):
1. Open https://trading-dashboard-675072986521.us-central1.run.app in browser
2. Wait for a few ticks to populate
3. Screenshot the full page → save as `docs/screenshots/dashboard.png`
4. Look for a trade with `"blocked_by_risk": true` in the trade log → screenshot that too → `docs/screenshots/risk-block.png`

### Step 5: Post on X (Today's Post)
Copy-paste from `docs/SOCIAL-MEDIA-POSTS.md` → Post 3 (Dashboard Demo, Apr 5).
Attach the dashboard screenshot.

Account: https://x.com/zixlancer

### Step 6: Connect Surge Social
1. Go to https://early.surge.xyz
2. Connect your email or X account
3. Claim 10 Ignites

---

## THIS WEEK (April 6-11)

### Step 7: LinkedIn Post (April 6)
Copy-paste from `docs/SOCIAL-MEDIA-POSTS.md` → LinkedIn Post 2 (Mid-Hackathon Update).
Account: https://www.linkedin.com/in/amin-sarafraz-abab883bb/

### Step 8: X Posts (April 7, 9, 11)
Follow the posting schedule in `docs/SOCIAL-MEDIA-POSTS.md`:
- Apr 7: Post 4 — Agent Card
- Apr 9: Post 5 — Risk Controls
- Apr 11: Post 6 — Final Push (X + LinkedIn)

### Step 9: Get Kraken API Key from Friend
Ask your friend to:
1. Log into Kraken → API → Create Key
2. Permissions: "Create & Modify Orders" only, **disable Withdraw**
3. Send you the key + secret
4. Also create a **read-only** key for leaderboard verification

Once you have it:
```bash
# Add to .env
echo 'KRAKEN_API_KEY=your-key-here' >> .env
echo 'KRAKEN_API_SECRET=your-secret-here' >> .env
```

### Step 10: Mint Agent Identity On-Chain
Run the agent once with chain config to trigger identity registration:
```bash
cd /Users/bm/hack01/trading-agent
cargo run --release
```
It will call `register_with_uri()` on the IdentityRegistry contract. Watch for:
```
chain: agent registered with id=...
```

### Step 11: Consider Re-recording Demo Video
Your current video (https://youtu.be/7zc0qDvCOKo) may not show:
- Live chain integration (Sepolia tx hashes in terminal)
- IPFS pinning (CID output)
- Dashboard with real data

If time allows, record a 3-5 min screen recording showing:
1. Agent starting up → chain registration → IPFS pin
2. Dashboard populating in real-time
3. A trade being blocked by risk gates
4. Agent card at `/.well-known/agent-card.json`

---

## SUBMISSION DAY (April 12)

### Step 12: Paste Long Description
1. Open `docs/LABLAB-LONG-DESCRIPTION.md`
2. Copy the full content
3. Go to https://lablab.ai/ai-hackathons/ai-trading-agents/proof-of-trust/proof-of-trust-trading-agent
4. Paste into the "Long Description" field

### Step 13: Add Social Media Links
After you've posted on X/LinkedIn, add the post URLs to the lablab.ai submission form under "Social Media Posts".

### Step 14: Final Checks
- [ ] Dashboard is live and showing data: https://trading-dashboard-675072986521.us-central1.run.app
- [ ] Backend responds: `curl https://trading-agent-675072986521.us-central1.run.app/metrics`
- [ ] Agent card works: `curl https://trading-agent-675072986521.us-central1.run.app/.well-known/agent-card.json`
- [ ] GitHub repo is public: https://github.com/zixelfreelance/erc8004-trading-agent
- [ ] Video is accessible: https://youtu.be/7zc0qDvCOKo
- [ ] At least 3 X posts published
- [ ] At least 1 LinkedIn post published

### Step 15: Submit
Click "Submit" on lablab.ai **before 7:30 PM IRST (16:00 UTC)**.

---

## Quick Reference

| What | Where |
|------|-------|
| Long description | `docs/LABLAB-LONG-DESCRIPTION.md` |
| Social posts | `docs/SOCIAL-MEDIA-POSTS.md` |
| Terminal screenshots | `docs/screenshots/` |
| Dashboard | https://trading-dashboard-675072986521.us-central1.run.app |
| Backend | https://trading-agent-675072986521.us-central1.run.app |
| GitHub | https://github.com/zixelfreelance/erc8004-trading-agent |
| lablab.ai | https://lablab.ai/ai-hackathons/ai-trading-agents/proof-of-trust/proof-of-trust-trading-agent |
| Surge | https://early.surge.xyz/discovery/proofoftrust-agent |
