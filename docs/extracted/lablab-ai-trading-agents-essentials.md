# AI Trading Agents — essentials (no-fluff brief)

**One-sentence core:** Build an **AI agent** that **interacts with money or simulated money**, **makes decisions**, and **shows it behaves safely and transparently**.

**See also:** [`lablab-ai-trading-agents-rules-research.md`](lablab-ai-trading-agents-rules-research.md) (published vs TBA) · [`lablab-ai-trading-agents-technology-access.md`](lablab-ai-trading-agents-technology-access.md) (technical requirements).

---

## What organizers are really testing

1. **Capital** — Not a chatbot: a **financial actor** (trade, risk, yield, protection).
2. **Autonomy** — Reads data, decides, executes without hand-holding each step.
3. **Provability** — Logs, validation artifacts, inspectable behavior. **This layer matters most** for “trustless” framing.

---

## Tracks (choose or combine)

| Option | You build | Judged on (per published story) |
|--------|-----------|----------------------------------|
| **Kraken** | AI trading agent via **Kraken CLI** (data + execution + PnL) | **Net PnL** + **quantitative social engagement** |
| **ERC-8004** | On-chain **identity** + signed intents + sandbox/vault-style execution | **Risk-adjusted** performance, **drawdown**, **validation / trust** signals |
| **Combined** | Explicitly encouraged — e.g. CLI execution + ERC-8004 logging/validation | Meet both barometers; don’t ship two half-systems |

---

## Submission surface (typical LabLab + this event)

- **Basic:** title, short/long description, tags  
- **Presentation:** demo video, slides, cover image  
- **Product:** public **GitHub** repo, **working demo URL**  
- **Mandatory for Surge prizes:** project registered on **early.surge.xyz** + build-in-public expectations as stated by organizers  

*(Exact form fields — confirm in dashboard.)*

---

## Minimum technical loop (all four must exist)

| Stage | Requirement |
|-------|----------------|
| **Input** | Real market data (e.g. via Kraken CLI) |
| **Decision** | Agent chooses action (buy / sell / hold / size) |
| **Action** | Executes ( **paper is fine** where rules allow ) |
| **Output** | **Logs** + trace of outcomes (PnL, positions, risk state) |

Missing any leg → weak submission.

---

## What tends to differentiate winners

**Common submissions:** naked bots, indicator glue, pretty dashboards only.

**Stronger submissions:** **discipline + clarity** — not just “smart.”

- **Risk control** — limits, stops, max loss, caps  
- **Transparency** — why a decision happened (structured, not only prose)  
- **Consistency** — repeatable policy, not one-off luck  
- **Measurable** — PnL *or* risk-adjusted metrics, aligned to the track you claim  

**Reframe:** You are not shipping “a strategy”; you are shipping **a system others could trust with capital** (even if sandboxed).

---

## Validity checklist (all YES)

- Fetches real market data  
- Decides automatically  
- Executes (paper or live per rules)  
- Tracks results (PnL / exposure)  
- Exposes reasoning or structured logs  
- Enforces explicit risk rules  

---

## Simplest winning interpretation (single line)

**AI trading agent that decides freely but is bounded by a strict risk system, with transparent logs and measurable outcomes.**

---

## Suggested build order

1. Closed **agent loop** (data → decision → execute → log)  
2. **Risk** layer (non-negotiable)  
3. **Presentation** (demo + repo + metrics people can see in 3 minutes)  
4. **Track-specific polish** (social / ERC-8004 artifacts)  

---

*Opinion brief for pair planning — not official rules. Verify eligibility and fields on LabLab and Surge.*
