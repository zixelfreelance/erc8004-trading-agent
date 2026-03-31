# AI Trading Agents (LabLab × Surge × Kraken) — rules & eligibility research

**Event (typical public framing):** online hackathon, **Mar 30 – Apr 12, 2026**, ~**$55,000** pool (verify on live page).  
**Primary URLs:** [AI Trading Agents — LabLab](https://lablab.ai/ai-hackathons/ai-trading-agents) · [Hackathon guidelines — LabLab](https://lablab.ai/blog/hackathon-guidelines)  
**Companion in this repo:** [`lablab-ai-trading-agents-technology-access.md`](lablab-ai-trading-agents-technology-access.md) (technology requirements) · [`lablab-ai-trading-agents-essentials.md`](lablab-ai-trading-agents-essentials.md) (no-fluff brief).  
**Assembled:** 2026-03-31 from organizer-facing copy + community research notes.

## 0. Hackathon timeline (captured detail)

| Milestone | When |
|-----------|------|
| **Start** | **March 30, 2026** (exact clock on LabLab / kickoff — confirm in dashboard) |
| **End** | **April 12, 2026**, **7:30 PM Iran Standard Time (IRST, UTC+3:30)** |

**Note:** LabLab may also show a countdown in **your** profile timezone; treat the **official submission deadline** in the platform as final if it differs from this note. **Approx. UTC** for the stated end: Apr 12, 2026 **16:00 UTC** (7:30 PM IRST, no Iranian DST in recent years).

### How to read this file

| Tag | Meaning |
|-----|---------|
| **Published** | Stated explicitly on LabLab/Surge/Kraken materials we are treating as first-party for this note. |
| **TBA** | Not found in public copy at research time — confirm in **Discord**, **dashboard**, or **organizer email**. |
| **Inference** | Plausible from LabLab’s *general* patterns or other events — **not** a substitute for this event’s form fields. |
| **Reconcile** | Two public narratives disagree; you must verify live. |

**Security:** If you see **shared login credentials** for Surge or any platform in chat or old screenshots, **do not paste them into git**. Use **your team profile**, **official Discord**, or **password reset** flows. This document **never** stores gate passwords.

---

## 1. Judging criteria and weights

### 1.1 ERC-8004 / “on-chain trust” challenge (Surge-oriented framing)

**Published (from event “Technology & Access” / challenge copy summarized on LabLab):**

- Emphasis on **risk-adjusted** performance, **drawdown** discipline, and **validation quality** (artifacts, registries, trust signals) rather than optimizing on **raw PnL alone** for this arm of the hackathon.
- Leaderboard narrative includes metrics such as **PnL**, **Sharpe**, **drawdown**, **validation score** (exact formula **TBA**).

**TBA:**

- Numeric **weights** (e.g. 40% / 30% / 30%).
- Whether any **subjective** judging layer exists on top of quantitative leaderboard data for this arm.
- Final **tie-break** rules.

### 1.2 Kraken challenge

**Published (from LabLab partner/challenge copy):**

- **Trading performance:** ranking tied to **net PnL** (realized + unrealized) over the competition window, with **Kraken CLI** as execution layer and **read-only API key** supplied for verification.
- **Social engagement:** separate scoring from **quantitative** public metrics (e.g. impressions, likes, shares, reach) on linked accounts — described as **non-subjective** at the metric layer.

**TBA:**

- Exact **weighting** between PnL vs social score (if combined for a single “Kraken” winner table).
- Anti-gaming / eligibility rules for social metrics.
- **Audit** mechanics (Kraken/lablab) beyond high-level descriptions.

### 1.3 “Hackathon judging pillars” (generic LabLab)

The [general hackathon guidelines](https://lablab.ai/blog/hackathon-guidelines) describe how LabLab events are *typically* evaluated (problem/solution, innovation, technical depth, completeness, presentation). Treat that as **background culture**, not **numeric scoring** for this event unless the **AI Trading Agents** submission UI or brief says it applies with weights.

---

## 2. Submission requirements (what you must deliver)

### 2.1 Explicit / strongly indicated (event + platform)

- A **working** AI trading/financial **agent** (trade, risk, yield, or protection) aligned to the challenge you enter.
- **ERC-8004 path:** identity registration, registries, signed intents, **Risk Router** / vault flow per organizer infra.
- **Kraken path:** **Kraken CLI** integration; **read-only** key for leaderboard where required.
- **Surge prize eligibility:** team/project registration on **early.surge.xyz** (stated on LabLab as required for prizes) and **build-in-public** activity there as described by organizers.

### 2.2 Standard LabLab submission shape (Inference)

Most LabLab hacks expect, at submission time:

- Public **repo** (or equivalent) + README + run instructions.
- **Demo video** (often **3–5 minutes** on similar events — **TBA** exact cap for this one).
- Completed **team** and **submission form** fields inside LabLab.

**TBA for this event:** word limits, required slide deck, mandatory live URL vs video-only, etc. — check the **dashboard after enrollment**.

---

## 3. One hackathon vs two “tracks”

**Published alignment with [`lablab-ai-trading-agents-technology-access.md`](lablab-ai-trading-agents-technology-access.md):**

- Copy describes **two equal challenges**; teams may compete in **one or both**; **combined** submissions are encouraged where integration makes sense.

**Reconcile (your two research notes):**

- A **minimal public landing** sometimes shows only the headline (“ERC-8004 or Kraken CLI”) without a full **track table**.
- Deeper on-page sections (and partner sections) treat **Kraken** and **ERC-8004** as **different success metrics** (PnL/social vs risk-adjusted/validation/vault).

**Practical read:** Think in terms of **two challenge specifications** that can be **merged in one product**. Do **not** assume shared judging math across them until the **official brief** confirms.

---

## 4. “Hackathon Capital Sandbox” / vault / risk router

**Published (conceptual):**

- ERC-8004 flow includes a **Hackathon Capital Vault** (funded **sub-account**, often **test** capital) and execution through a **whitelisted Risk Router** with limits (size, leverage, markets, loss caps).
- Performance is intended to be **measurable on-chain** in stablecoin terms for that challenge narrative.

**TBA:**

- Contract addresses, chain IDs, faucet procedures, support SLAs.
- How **Kraken** paper/live accounts relate to the same “sandbox” wording when copy blends ecosystems.

**Action:** Treat vault/router as **organizer-distributed** (Discord, Surge dashboard, private Notion) rather than something fully specified on the public marketing page alone.

---

## 5. early.surge.xyz — registration and access

**Published (high level):**

- **Project registration** on Surge’s early platform is tied to **prize eligibility** and **build-in-public** expectations in LabLab copy.

**TBA:**

- KYC, residency, entity type, team size caps, tax/withholding — if any, they will appear in **Surge terms** or **payout flows**, not in a one-line LabLab blurb.

**Security note:** Shared “demo” credentials circulated in the wild are a **liability**. Use official onboarding; **rotate** any password that was ever posted publicly.

---

## 6. Prizes: denomination, Surge vs Kraken, vesting

**Published (high level):**

- LabLab mentions a **~$55,000** pool **allocated via Surge and Kraken** (exact split and denomination must be read from the **current** event page and partner blocks).
- Separate Surge-oriented prize copy (also captured in this repo under `early-surge-*.md`) discusses **$SURGE** grants, **Streamflow** vesting, multisig expectations, and timelines — treat that as **pattern** for Surge token prizes but **re-read** the **AI Trading Agents** prize section for this edition.

**TBA until you see the official prize/legal module:**

- Whether any prizes are **USDC**, **fiat equivalent**, or **token-only**.
- Clawbacks, KYC for payout, geographic restrictions.

---

## 7. What wins (patterns, not guarantees)

**Illustrative parallels (Inference):**

- LabLab × Arc “agentic commerce” winner stories (e.g. agentic banking, escrow, signal infra) reward **credible economic loops** and **on-chain hooks**, not slide-only ideas — see organizer announcements for **Arc** as **analogy only**.
- For **this** hackathon, optimize **per challenge you choose**:
  - **Kraken:** competitive **PnL** within rules + durable **social proof** if that bucket is in play.
  - **ERC-8004:** **risk-aware** behavior, **clean validation story**, and **transparent** measurement.

---

## 8. Open questions checklist (paste into Discord / support)

Use this as a copy-paste prompt for organizers:

1. Are **Kraken** and **ERC-8004** judged on **fully separate leaderboards**, and can one repo win **both**?  
2. Exact **formula** for ERC-8004 leaderboard (Sharpe window, drawdown definition, validation score inputs).  
3. Exact **social engagement** metric sources and **anti-abuse** rules for Kraken.  
4. Required **submission artifacts** (video length, repo, deck, live URL) for **this** event ID.  
5. **Vault/router** deployment package (addresses, networks, test funds).  
6. **Prize** type (token vs cash), **vesting**, **eligibility** (KYC, geography), and **tax** docs.  
7. Relationship between **LabLab team**, **Surge project**, and **Kraken** verification accounts.

---

## 9. Contradiction log (for your own sanity)

| Topic | Shallow public page risk | Deeper partner sections |
|-------|--------------------------|-------------------------|
| Winning metric | “Trading agents” generically | **PnL** vs **risk-adjusted** split by challenge |
| Tracks | Looks like one blur | **Two challenges** with different requirements |
| Submission | Unclear fields | LabLab **usually** still wants repo + video |

When in doubt, **the deepest on-page LabLab section + submission dashboard** beat a hero banner or third-party recap.

---

*This file is research notes for builders, not legal advice. Verify everything on live organizer pages before you rely on it for eligibility or tax planning.*
