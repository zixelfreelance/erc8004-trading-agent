# Social Media Posts — Proof-of-Trust Trading Agent

## X/Twitter Posts (max 280 chars for free tier)

### Post 1 — Announcement (April 1)
```
Building a trustless AI trading agent for @lablabai hackathon

The AI can't break its own rules:
→ ERC-8004 on-chain identity
→ 7 enforced risk gates
→ EIP-712 signed intents

AI proposes. Contracts enforce.

@krakenfx @Surgexyz_ #AITradingAgents
```

### Post 2 — Tech Thread (April 3)
```
How do you trust an AI trading agent?

You don't. You verify.

I built a trading agent with 7 risk gates it CANNOT bypass — and every decision is signed on-chain.

Thread 🧵

@lablabai @krakenfx @Surgexyz_
```

Reply 1:
```
The Trust Triangle — every trade must satisfy:

1. Identity (ERC-8004) — who acted
2. Constraints (Risk Router) — what was allowed
3. Auditability (Validation Artifacts) — what happened

All three. Every time.
```

Reply 2:
```
7 risk gates the AI cannot bypass:

• 5% max drawdown
• 1 position limit
• 60% confidence floor
• 0.7% fee filter
• Regime filter
• Circuit breaker (3 losses)
• ATR trailing stop

20%+ of trades get blocked. Proof the gates work.
```

Reply 3:
```
Tech: Rust + 3 Solidity contracts + SvelteKit dashboard + Claude AI + Kraken CLI

79 tests. EIP-712 signed intents. Agent Card at /.well-known/agent-card.json

github.com/zixelfreelance/erc8004-trading-agent

#ERC8004 #DeFi
```

### Post 2.5 — Day 5 Progress Update (April 5)
```
Day 5 of @lablabai AI Trading Agents hackathon

103 commits deep. Proof-of-Trust is live:
- 3 contracts on Sepolia
- EIP-712 signed trade intents
- Dashboard on Vercel
- 10 risk controls, 7 technical indicators
- IPFS audit trail via Pinata
- 79 tests (72 unit + 7 e2e)

Demo mode working. MIT licensed.

@krakenfx @Surgexyz_ #AITradingAgents
```
(Attach dashboard screenshot from erc8004-trading-agent.vercel.app)

### Post 2.6 — Execution Modes Explainer (April 4)
```
How our AI agent handles trust at every layer:

3 execution modes:
  Paper → real prices, fake money (no API key)
  Demo → offline replay (reproducible)
  Live → real Kraken orders (API key required)

3 decision modes:
  Momentum → pure rules (no LLM needed)
  ADK → Claude makes all calls
  Hybrid → rules + Claude final say

Default: paper + momentum = $0, no keys, full demo.

@lablabai @krakenfx @Surgexyz_
```

### Post 2.7 — Demo Mode Fix (April 5)
```
We test what we preach.

Fixed MockExecution today — demo mode now runs e2e with zero external deps.

79 tests passing (72 unit + 7 e2e). Every risk gate, every signal, every signed intent — verified in CI.

Trust isn't a claim. It's a test suite.

@lablabai @krakenfx @Surgexyz_ #AITradingAgents
```

### Post 3 — Dashboard Demo (April 5)
```
Real-time dashboard for our AI trading agent:

• Price + Bollinger Bands
• Buy/Sell markers
• PnL tracking
• Risk gate decisions in trade log

103 commits, 79 tests, 3 contracts on Sepolia.

Demo video: youtu.be/7zc0qDvCOKo

@lablabai @krakenfx @Surgexyz_ #AITradingAgents
```
(Attach dashboard screenshot)

### Post 4 — Agent Card (April 7)
```
Our AI agent serves an Agent Card at /.well-known/agent-card.json

Like robots.txt — but for AI trading agents.

Identity, capabilities, risk controls, endpoints — all discoverable on-chain via ERC-8004.

@lablabai @krakenfx @Surgexyz_
```
(Attach agent-card.json screenshot)

### Post 5 — Risk Controls (April 9)
```
"But what if the AI goes rogue?"

It can't. 7 gates enforce limits BEFORE execution:
- Drawdown cap
- Position limit
- Confidence floor
- Fee filter
- Regime filter
- Circuit breaker
- Trailing stop

The AI proposes. The system decides.

@lablabai @krakenfx @Surgexyz_
```

### Post 6 — Final Push (April 11)
```
Submitting our AI trading agent for @lablabai hackathon tomorrow.

Proof-of-Trust: an agent that proves every decision on-chain.

Rust + Solidity + SvelteKit + Claude AI + Kraken CLI

github.com/zixelfreelance/erc8004-trading-agent

@krakenfx @Surgexyz_ #AITradingAgents
```

---

## LinkedIn Posts

### Post 1 — Announcement (April 1)
```
I'm building a trustless AI trading agent for the lablab.ai AI Trading Agents hackathon ($55K prize pool).

The core idea: AI is untrusted input. The agent proposes — the system decides.

What I built:
• On-chain agent identity via ERC-8004 (ERC-721 NFT)
• 7 enforced risk gates the AI cannot bypass
• EIP-712 signed trade intents for every decision
• Real-time dashboard with Bollinger Bands, PnL, and drawdown tracking
• Cryptographic audit trail — not logging, proof

Tech: Rust, Solidity, SvelteKit, Claude AI (Anthropic ADK), Kraken CLI

The thesis: the winning AI trading agents won't have the best returns — they'll be the ones you can verify and trust.

GitHub: https://github.com/zixelfreelance/erc8004-trading-agent

#AI #DeFi #Trading #ERC8004 #Hackathon #Rust #Solidity
```

### Post 2 — Mid-Hackathon Update (April 6)
```
Week 1 update on my AI trading agent build:

✅ 79 tests passing (72 unit + 7 e2e)
✅ 103 commits
✅ 3 Solidity contracts deployed to Sepolia
✅ Real-time SvelteKit dashboard on Vercel
✅ ERC-8004 agent identity registered
✅ Risk gates blocking 20%+ of trades
✅ IPFS audit trail via Pinata
✅ Demo mode fully working

The best part? Every decision is signed with EIP-712 and verifiable on-chain. Not logging — cryptographic proof.

Demo video: https://youtu.be/7zc0qDvCOKo
GitHub: https://github.com/zixelfreelance/erc8004-trading-agent

#AI #DeFi #BuildInPublic
```

---

## Posting Schedule

| Date | Platform | Post |
|------|----------|------|
| Apr 1 | X + LinkedIn | Post 1 — Announcement |
| Apr 3 | X | Post 2 — Tech Thread |
| Apr 4 | X | Post 2.6 — Execution Modes Explainer |
| Apr 5 | X | Post 2.5 — Day 5 Progress Update |
| Apr 5 | X | Post 2.7 — Demo Mode Fix |
| Apr 5 | X | Post 3 — Dashboard Demo (with screenshot) |
| Apr 6 | LinkedIn | Post 2 — Mid-Hackathon Update |
| Apr 7 | X | Post 4 — Agent Card |
| Apr 9 | X | Post 5 — Risk Controls |
| Apr 11 | X + LinkedIn | Post 6 — Final Push |
