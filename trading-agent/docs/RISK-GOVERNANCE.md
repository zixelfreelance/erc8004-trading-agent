# Risk Governance Framework — Proof-of-Trust Trading Agent

> How the agent manages risk at every layer. Evidence for regulators, investors, and judges.

---

## 1. Risk Philosophy

**"Constraints over predictions."**

We don't try to predict markets better than everyone else. We constrain risk better. The agent that loses 2% in a drawdown while others lose 15% wins — risk-adjusted returns beat raw PnL.

### Core Principles

1. **AI is untrusted input.** Every AI proposal passes through cryptographic signing, risk validation, and constraint enforcement before touching capital.
2. **Contracts enforce, agents propose.** Intelligence is off-chain (fast, cheap). Trust is on-chain (immutable, verifiable).
3. **Defense in depth.** 7 independent risk gates, any one of which can block a trade. No single point of failure.
4. **Fail safe, not fail open.** On error, ambiguity, or missing data — the agent holds. It never defaults to action.

---

## 2. Risk Gate Architecture

### Pre-Execution Gates (All Must Pass)

```
AI Decision
    │
    ├── Gate 1: Confidence Floor (≥ 60%)
    │   └── Reject: "AI not confident enough"
    │
    ├── Gate 2: Fee Filter (edge ≥ 0.7%)
    │   └── Reject: "Expected move doesn't cover fees"
    │
    ├── Gate 3: Regime Filter (not in transition)
    │   └── Reject: "Market regime unclear, wait for clarity"
    │
    ├── Gate 4: Position Limit (≤ 1 open)
    │   └── Reject: "Already in a position"
    │
    ├── Gate 5: Drawdown Check (< 5% from peak)
    │   └── Reject: "Max drawdown breached — agent halted"
    │
    ├── Gate 6: Circuit Breaker (< 3 consecutive losses)
    │   └── Reject: "Losing streak — cooling off"
    │
    ├── Gate 7: Daily Loss Limit (< $5/day)
    │   └── Reject: "Daily loss cap reached"
    │
    └── ✅ ALL PASS → Sign Intent → Execute via Kraken CLI
```

### Post-Execution Controls

| Control | Mechanism | Trigger |
|---|---|---|
| ATR Trailing Stop | 1.5x ATR below entry (long) / above entry (short) | Set on entry, trails with price, forces exit on breach |
| Validation Artifact | JSON with full decision context | Generated for every decision (trade or hold) |
| On-Chain Anchor | Merkle root of artifact hashes | Anchored periodically to Base L2 |

---

## 3. Risk Metrics — What We Track

### Real-Time (Dashboard + /metrics endpoint)

| Metric | Formula | Purpose |
|---|---|---|
| Current Drawdown | (peak_equity - current_equity) / peak_equity | Distance from halt threshold |
| Consecutive Losses | Count of sequential losing trades | Circuit breaker proximity |
| Daily P&L | Sum of realized gains/losses today | Daily loss limit tracking |
| Win Rate | winning_trades / total_trades | Strategy health |
| Trades Blocked | blocked_count / total_decisions | Proof gates are active |

### Periodic (Walk-Forward Validation)

| Metric | Target | Frequency |
|---|---|---|
| Sharpe Ratio | > 1.0 | Weekly computation over trailing 30 days |
| Sortino Ratio | > 1.2 | Weekly |
| Calmar Ratio | > 2.0 | Weekly |
| Max Drawdown Duration | < 48 hours | Continuous |
| Fee-Adjusted Returns | > 0 after 0.52% round-trip | Per-trade |

---

## 4. Strategy Validation Process

### Walk-Forward Methodology

```
Historical Data (6 months, 1-min BTC/USD candles from Kraken)
    │
    ├── Split: 70% in-sample / 30% out-of-sample (anchored window)
    │
    ├── In-sample: optimize indicator parameters
    │   ├── RSI period, MACD windows, Bollinger width
    │   ├── ATR multiplier for stops
    │   └── Confidence thresholds
    │
    ├── Out-of-sample: validate with NO parameter changes
    │   ├── Must show Sharpe > 1.0
    │   ├── Must show max drawdown < 5%
    │   └── Must be profitable after fees (0.52% round-trip)
    │
    ├── Roll forward: slide window by 1 month, repeat
    │
    └── Final: aggregate all out-of-sample windows for reported metrics
```

### Statistical Significance

- Minimum 200 trades in out-of-sample period for statistical validity
- Bootstrap confidence intervals on Sharpe ratio (95% CI must exclude 0)
- Compare against benchmarks: buy-and-hold BTC, random entry with same risk management
- Document all parameter choices and their sensitivity

### Model Governance

| Event | Action Required |
|---|---|
| Claude model update | Re-run walk-forward validation, compare to baseline |
| Strategy parameter change | Full out-of-sample re-validation before live deployment |
| New indicator added | Validate incrementally — must improve Sharpe without overfitting |
| Market regime shift | Circuit breaker triggers → manual review → parameter adjustment if needed |
| Drawdown > 3% | Review decision logs, identify if strategy or market-driven |

---

## 5. Incident Response

### Severity Levels

| Level | Definition | Example | Response |
|---|---|---|---|
| **S1 — Critical** | Loss exceeding limits or system failure during open position | Drawdown gate failed, agent kept trading | Immediate halt. Manual position close. Root cause analysis within 24h. |
| **S2 — High** | Unexpected behavior within limits | Agent placed trade that should have been blocked by regime filter | Halt agent. Review logs. Fix and re-validate before restart. |
| **S3 — Medium** | Degraded performance | Win rate dropped below 40% over 50+ trades | Continue with monitoring. Schedule parameter review. |
| **S4 — Low** | Minor issue, no financial impact | Dashboard latency, stale metrics display | Log issue. Fix in next maintenance window. |

### Escalation Path

```
Automated (risk gates) → Agent self-halts
    ↓ (if gates fail)
Dashboard alert → Human reviews
    ↓ (if loss exceeds threshold)
Manual intervention → Kill process, close positions via Kraken directly
    ↓ (post-incident)
Root cause analysis → Decision log review → Fix → Re-validate → Restart
```

---

## 6. On-Chain Risk Enforcement (ERC-8004 + Risk Router)

### What the Smart Contract Enforces

The Risk Router contract (`RiskRouter.sol`) provides a **second, independent layer** of risk enforcement on-chain:

| Check | On-Chain Enforcement | Off-Chain Enforcement |
|---|---|---|
| Pair whitelist | Contract rejects non-whitelisted pairs | Config validation at startup |
| Position size | Contract rejects oversized intents | Risk gate checks before signing |
| Signer verification | EIP-712 signature must match registered agent wallet | Intent signed with agent's private key |

### Why Both Layers

- **Off-chain gates** are fast (sub-millisecond) and granular (7 independent checks)
- **On-chain enforcement** is immutable and publicly verifiable — even if the off-chain agent is compromised, the contract rejects invalid intents
- **Together** they form defense-in-depth: the on-chain layer is the backstop that proves constraints were honored

---

## 7. Supervision Dashboard

### Current Capabilities

- Real-time decision feed (every 10 seconds)
- Full decision context: signals, AI reasoning, risk gate results, execution outcome
- P&L chart with drawdown overlay
- Risk state indicators (gates active/triggered)
- Trade history with signed intent verification

### Planned (Post-Hackathon)

- Global pause button (one-click agent halt)
- Alert rules (email/Slack on S1/S2 events)
- Historical replay (deterministic mode for incident review)
- Multi-agent view (when registry supports multiple agents)

---

## 8. Mandate Capture (Post-Hackathon — Required for Product)

Before accepting any user, the product must capture explicit consent for:

| Parameter | User Configures | Default |
|---|---|---|
| Max daily loss | Dollar amount they're willing to lose per day | $5 |
| Max drawdown | Percentage of equity they'll accept as max loss | 5% |
| Trade size | BTC amount per trade | 0.001 |
| Paper/live mode | Whether real money is at risk | Paper |
| Trading pair | Which asset to trade | BTC/USD |
| Strategy mode | Momentum, hybrid, or AI-only | Momentum |

Mandate is captured in plain language, stored locally, and reflected in the agent's risk configuration. Changes require explicit user confirmation.

---

*This framework is designed for hackathon + early-product scope. Production deployment requires independent audit of risk controls and legal review of governance procedures.*
