# AI Disclosure — Proof-of-Trust Trading Agent

> Draft disclosure for users and regulators. Not legal advice — requires counsel review before production use.

---

## 1. AI Usage in This Product

### What AI Does

This product uses artificial intelligence (Anthropic Claude via the Agent Development Kit) as **one input** in a multi-stage trading decision pipeline. Specifically:

- **Signal generation:** AI analyzes technical indicators (RSI, MACD, Bollinger Bands, ATR, ADX), market regime, order book depth, and recent trade history to produce a directional signal (buy, sell, or hold) with a confidence score.
- **Reasoning:** AI provides human-readable rationale for each decision, including bull/bear arguments and risk assessment.
- **Refinement:** In hybrid mode, AI refines a deterministic momentum signal — it does not generate signals from scratch.

### What AI Does NOT Do

- AI **does not execute trades**. Execution is performed by the Kraken CLI after all risk gates pass.
- AI **cannot override risk limits**. The 7-layer risk gate system rejects any AI proposal that violates constraints, regardless of AI confidence.
- AI **does not have access to user funds**. The agent uses the user's Kraken API key with trade permissions only.
- AI **does not learn or adapt in real-time**. The model is stateless between decisions — no reinforcement learning or live fine-tuning.

---

## 2. Automation Boundaries

### Decision Pipeline

```
Market Data → Technical Indicators → AI Analysis → Signed Intent → Risk Gates → Execution
                                         ↑                              ↑
                                    AI proposes here            System enforces here
```

### Risk Controls (Cannot Be Overridden by AI)

| Control | Limit | Effect |
|---|---|---|
| Max Drawdown | 5% of peak equity | Agent halts all trading |
| Daily Loss Limit | $5 USD | Agent halts for remainder of day |
| Circuit Breaker | 3 consecutive losses | Agent pauses trading |
| Confidence Floor | 60% minimum | Low-confidence signals rejected |
| Fee Filter | 0.7% minimum edge | Unprofitable trades rejected |
| Position Limit | 1 open position | No pyramiding |
| ATR Trailing Stop | 1.5x ATR from entry | Mechanical exit |

### Human Override

- Users can pause or stop the agent at any time via the dashboard or by terminating the process
- Users configure all risk limits before the agent starts
- The agent does not modify its own risk configuration

---

## 3. Kraken's Role

- **Kraken is the execution venue**, not a partner or co-developer of this product
- Trades are placed via [Kraken CLI](https://blog.kraken.com/news/industry-news/announcing-the-kraken-cli), an open-source tool published by Kraken
- User funds remain in the user's Kraken account at all times
- This product is not endorsed, sponsored, or affiliated with Kraken (except as a hackathon participant)

---

## 4. No Custody Design

- This product **never holds, transfers, or controls user funds**
- The user provides their own Kraken API key with trade permissions
- The API key is stored locally on the user's machine, never transmitted to third parties
- The user can revoke API access at any time via their Kraken account settings

---

## 5. AI Limitations and Risk Factors

### Known Limitations

- **AI can be wrong.** The Claude model may misinterpret market signals, generate false confidence, or produce contradictory reasoning. This is why AI is treated as untrusted input with hard constraints.
- **Historical performance does not predict future results.** Walk-forward validation and backtests demonstrate strategy behavior under past conditions, not future ones.
- **Market regime changes.** The regime detection system (trending/ranging/transition) may fail to identify novel market conditions not present in training data.
- **Latency.** The 10-second decision loop and market order execution introduce slippage that may differ from backtested results.
- **Model changes.** Anthropic may update the Claude model, which could change AI behavior. Strategy performance should be re-validated after model updates.

### Risk Factors

- **Loss of capital.** Trading involves risk of loss. The max drawdown limit (5%) bounds but does not eliminate loss.
- **Exchange risk.** Kraken outages, API changes, or account restrictions could prevent trade execution or exit.
- **Smart contract risk.** On-chain components (ERC-8004 identity, Risk Router) may contain vulnerabilities. These are trust/audit components, not custody — no funds are at risk from contract bugs.
- **Operational risk.** Agent crashes, network interruptions, or configuration errors could result in missed exits or unintended positions.

---

## 6. Data Usage

- **Market data** is sourced from Kraken's public API (prices, order book, OHLC candles)
- **AI prompts** contain only market data and technical indicators — no personal user data is sent to Anthropic
- **Decision artifacts** (including AI reasoning) are logged locally and optionally anchored on-chain as hashes
- **No user data is sold, shared, or used for training**

---

## 7. Contact and Governance

- **Source code:** Open-source on GitHub (link)
- **Decision audit:** Every trade is logged with full context at `/logs` endpoint
- **On-chain identity:** Agent is registered on Base L2 via ERC-8004 with verifiable risk policy and performance history
- **Incident response:** (To be defined for production — currently hackathon/research scope)

---

*This disclosure is a draft for development purposes. Production deployment requires review by qualified legal counsel in the relevant jurisdiction(s).*
