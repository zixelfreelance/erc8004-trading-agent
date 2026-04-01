# Revenue Roadmap — Proof-of-Trust Trading Agent

> From hackathon prototype to sustainable revenue. This document covers monetization strategy, development phases, and the path to production.

---

## 1. Revenue Model

### 1.1 Primary Revenue Streams

| Stream | Model | Target | Timeline |
|---|---|---|---|
| **Agent Trading Profit** | Direct P&L from live trading | Self-funded bootstrap | Q2 2026 |
| **Platform Fees** | 2% AUM + 20% performance fee | External capital allocators | Q4 2026 |
| **Protocol Fees** | 0.5% on routed capital | Multi-agent marketplace users | Q2 2027 |
| **Enterprise Licensing** | Annual license for trust framework | Funds, prop desks, fintechs | Q3 2027 |

### 1.2 Revenue Stream Details

#### Stream 1: Agent Trading Profit (Immediate)
- Deploy live on Kraken with real capital
- Start small ($1K–$5K), scale with validated Sharpe > 1.0
- Reinvest profits to compound AUM
- **Revenue:** 100% of trading PnL
- **Risk:** Market risk — mitigated by 7 risk gates + max 5% drawdown

#### Stream 2: Platform Fees (Growth Phase)
- Open the platform to external capital depositors
- Depositors allocate to verified agents with on-chain track records
- **Fee structure:**
  - 2% annual management fee on AUM
  - 20% performance fee on profits (high-water mark)
- **Example:** $1M AUM → $20K/yr management + $40K performance (at 20% annual return)
- **Moat:** Cryptographic proof that risk limits were honored — no other platform offers this

#### Stream 3: Protocol Fees (Scale Phase)
- Multi-agent marketplace: anyone can deploy a trading agent
- Capital Vault routes funds to agents proportional to reputation score
- Protocol takes 0.5% fee on all routed capital
- **Example:** $50M total routed → $250K/yr protocol fees
- **Network effect:** More agents → more strategies → more capital → more fees

#### Stream 4: Enterprise Licensing (Parallel Track)
- License the trust framework (Risk Router + Identity Registry + audit pipeline) to:
  - Hedge funds needing AI audit trails for regulators
  - Prop desks deploying autonomous strategies
  - Fintechs building compliant AI products
- **Pricing:** $50K–$200K/yr per enterprise seat
- **Why they pay:** MiFID II, SEC AI governance rules, and institutional LP demands for AI explainability are all tightening — this solves a real compliance gap

---

## 2. Development Roadmap

### Phase 1: Hackathon MVP (Current — Apr 2026)

**Status:** Sprint 2 near-complete. Agent is live in paper mode.

**Deliverables:**
- [x] Hexagonal architecture (ports/adapters/domain)
- [x] Momentum + Claude/ADK hybrid strategies
- [x] 7 risk gates (drawdown, position, confidence, circuit breaker, daily loss, fee filter, consecutive losses)
- [x] EIP-712 signed intents
- [x] 7 technical indicators (SMA, EMA, RSI, MACD, Bollinger, ATR, ADX)
- [x] Regime detection (trending/ranging/transition)
- [x] SvelteKit dashboard + HTTP audit API
- [x] Solidity contracts (Identity, Reputation, Risk Router)
- [ ] Deploy to Sepolia
- [ ] Video demo + submission

**Revenue:** $0 (validation phase)

---

### Phase 2: Live Trading + Track Record (May–Jul 2026)

**Goal:** Prove the agent makes money. Build a verifiable track record.

| Task | Priority | Est. Effort |
|---|---|---|
| Go live on Kraken with real capital ($1K–$5K) | P0 | 1 week |
| WebSocket streaming (replace polling) | P0 | 3 days |
| Multi-pair scanning (BTC, ETH, SOL) | P0 | 1 week |
| Order book depth signals | P1 | 3 days |
| Limit + stop-loss order types via Kraken CLI | P0 | 3 days |
| ATR trailing stops with real stop-loss orders | P0 | 2 days |
| Mean-reversion strategy for ranging markets | P1 | 1 week |
| Backtesting engine (replay historical data) | P1 | 2 weeks |
| Parameter tuning from live data | P0 | Ongoing |
| Deploy contracts to mainnet (Base L2) | P1 | 1 week |
| Automated reputation scoring on-chain | P1 | 1 week |
| Deploy dashboard publicly (Vercel) | P0 | 1 day |
| CI/CD pipeline (cargo test + forge test) | P1 | 3 days |

**Target metrics:**
- Sharpe Ratio > 1.0
- Max Drawdown < 5%
- Win Rate > 55%
- 90+ days of continuous live operation

**Revenue:** Direct trading profit. Target $500–$2K/month at small scale.

---

### Phase 3: External Capital + Platform (Aug–Dec 2026)

**Goal:** Accept outside money. Charge fees. Build trust at scale.

| Task | Priority | Est. Effort |
|---|---|---|
| Capital Vault smart contract (deposit/withdraw/rebalance) | P0 | 3 weeks |
| KYC/AML integration for depositors | P0 | 2 weeks |
| Multi-agent registry (deploy multiple strategies) | P0 | 4 weeks |
| Agent leaderboard (ranked by risk-adjusted returns) | P0 | 2 weeks |
| Fee collection smart contracts (management + performance) | P0 | 2 weeks |
| Depositor dashboard (portfolio view, allocation controls) | P0 | 3 weeks |
| Legal entity formation + fund structure | P0 | External counsel |
| Security audit of smart contracts | P0 | External auditor |
| Institutional-grade monitoring + alerting | P1 | 2 weeks |
| Multi-exchange support (Binance, Coinbase, Bybit) | P1 | 4 weeks |
| API for programmatic capital allocation | P1 | 2 weeks |

**Revenue model activates:**
- 2% AUM + 20% performance fee
- Target: $500K–$2M AUM by end of Q4 2026
- Projected revenue: $10K–$40K/yr at this scale

---

### Phase 4: Protocol + Marketplace (Q1–Q3 2027)

**Goal:** Permissionless agent marketplace. Protocol-level revenue.

| Task | Priority | Est. Effort |
|---|---|---|
| Open agent registration (anyone can deploy) | P0 | 4 weeks |
| Reputation-weighted capital routing (automated vault rebalancing) | P0 | 6 weeks |
| Strategy tokens (ERC-8004 — fractional ownership of agent performance) | P0 | 4 weeks |
| Protocol fee router (0.5% on routed capital) | P0 | 2 weeks |
| Governance token + DAO structure | P1 | 6 weeks |
| zk-proof or TEE attestation for AI decision verification | P1 | 8 weeks |
| On-chain execution (DEX routing — remove CEX dependency) | P2 | 8 weeks |
| Cross-chain deployment (Ethereum, Arbitrum, Base, Solana) | P2 | 6 weeks |
| SDK for third-party agent builders | P1 | 4 weeks |

**Revenue:** Protocol fees + strategy token trading fees
- Target: $10M–$50M routed capital
- Projected revenue: $50K–$250K/yr protocol fees + growth from token economics

---

### Phase 5: Enterprise + Compliance (Q3 2027+)

**Goal:** License the trust framework to institutions.

| Task | Priority | Est. Effort |
|---|---|---|
| White-label trust framework (Risk Router + Identity + audit) | P0 | 8 weeks |
| Compliance reporting module (MiFID II, SEC AI rules) | P0 | 6 weeks |
| Enterprise SSO + role-based access | P1 | 3 weeks |
| Custom risk policy engine (configurable per client) | P0 | 4 weeks |
| SLA + support infrastructure | P0 | Ongoing |
| Regulatory sandbox participation (FCA, MAS, FINMA) | P1 | External |

**Revenue:** $50K–$200K/yr per enterprise license

---

## 3. Financial Projections

| Quarter | AUM | Trading PnL | Platform Fees | Protocol Fees | Enterprise | Total |
|---|---|---|---|---|---|---|
| Q2 2026 | $5K (own) | $500 | — | — | — | **$500** |
| Q3 2026 | $25K | $2.5K | — | — | — | **$2.5K** |
| Q4 2026 | $500K | $5K | $10K | — | — | **$15K** |
| Q1 2027 | $2M | $10K | $40K | — | — | **$50K** |
| Q2 2027 | $10M | $20K | $100K | $25K | — | **$145K** |
| Q3 2027 | $25M | $30K | $250K | $62K | $100K | **$442K** |
| Q4 2027 | $50M | $40K | $500K | $125K | $300K | **$965K** |

*Assumptions: 20% annual return on AUM, 2%+20% fee structure, 0.5% protocol fee, conservative growth.*

---

## 4. Competitive Advantages (Moat)

| Advantage | Why It Matters | Hard to Copy? |
|---|---|---|
| **Cryptographic audit trail** | Every decision is signed + on-chain verifiable | Medium — architecture is novel |
| **On-chain risk enforcement** | Risk Router rejects bad trades even if AI + operator agree | High — requires full stack integration |
| **ERC-8004 identity** | Agent has portable, verifiable reputation | High — first-mover in agent identity standard |
| **Hexagonal architecture** | Swap AI engines, exchanges, chains without rewrite | Medium — engineering discipline |
| **Rust performance** | Sub-millisecond decision pipeline | Low — but combined with trust layer = unique |
| **Regulatory readiness** | Built for compliance from day one, not bolted on | High — most competitors ignore this |

---

## 5. Key Risks + Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Strategy underperforms | No trading PnL, weak track record | Backtest extensively, diversify strategies, hard drawdown limits |
| Regulatory uncertainty | Fund structure may not be viable in all jurisdictions | Start in crypto-friendly jurisdiction, engage legal counsel early |
| Smart contract vulnerability | Loss of depositor funds | Professional audit, bug bounty, insurance fund |
| Market regime shift | Strategy stops working | Regime detection + multiple strategy modes + circuit breakers |
| No product-market fit for enterprise | Enterprise licensing doesn't close | Validate with 3–5 pilot customers before building white-label |
| Competitor copies approach | Commoditized trust layer | Move fast, build network effects, own the ERC-8004 standard |

---

## 6. Immediate Next Steps (Post-Hackathon)

1. **Deploy live on Kraken** — real capital, small position ($1K). Start building track record.
2. **Ship backtesting engine** — validate strategies against 6+ months of historical data before scaling.
3. **Deploy contracts to Base mainnet** — on-chain identity + reputation scoring active.
4. **Publish the dashboard** — public URL becomes the live proof-of-trust demo.
5. **Legal structure** — consult counsel on fund structure for accepting external capital.
6. **Pitch 5 early depositors** — $50K–$100K each, using the verifiable track record.
7. **Apply to accelerators** — crypto-native programs (Alliance, a16z CSS, Delphi Ventures).

---

## 7. Team Needs

| Role | When | Why |
|---|---|---|
| **Solidity auditor** | Phase 3 (before accepting deposits) | Smart contract security |
| **Compliance/legal counsel** | Phase 3 | Fund structure, KYC/AML, regulatory |
| **Quant researcher** | Phase 2–3 | Strategy development, backtesting, alpha research |
| **Frontend engineer** | Phase 3 | Depositor dashboard, onboarding flows |
| **DevOps/infra** | Phase 2+ | Uptime, monitoring, multi-region deployment |
| **BD/sales** | Phase 5 | Enterprise licensing pipeline |

---

*This is a living document. Update quarterly as the project evolves.*
