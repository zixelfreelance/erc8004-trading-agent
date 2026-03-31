# ERC-8004: Trustless Agents — Ethereum Magicians thread (extraction)

**Source:** [ethereum-magicians.org — ERC-8004: Trustless Agents](https://ethereum-magicians.org/t/erc-8004-trustless-agents/25098)  
**Thread starter:** davidecrapis.eth · 2025-08-14  
**Extracted:** 2026-03-31 (notes; **full discussion** — read the live thread; pagination continues after first page)

## Opening post (summary)

ERC-8004 extends **Agent-to-Agent (A2A)** with a **trust layer** so participants can **discover, choose, and interact** with agents **across organizational boundaries** without prior trust.

- **Three on-chain registries:** Identity, Reputation, Validation — **lightweight**; application logic stays **off-chain**.
- **Standard / PR:** [ethereum/ERCs pull/1170](https://github.com/ethereum/ERCs/pull/1170) (verify current number on GitHub).
- **Process:** Public discussion; coordination with **Linux Foundation** and **A2A** ecosystem stakeholders.
- **Acknowledgments:** Listed contributors (Consensys, TensorBlock, Nethermind, Google, EF, Olas, etc.) — see thread OP.

## Themes raised in discussion (condensed)

| Topic | Summary |
|--------|---------|
| **On-chain composability** | Concern that design favors **off-chain** reads (events) over **contract-readable** validation/reputation; desire for primitives so other contracts can branch on validation results and decouple validation from enforcement (e.g. slashing). |
| **Reputation shape** | Ideas: multiple scores / providers; **aggregate** metrics for “who to work with”; counterpoint — **single global score** risks monopoly; trust as **contextual** (Alice→Bob ≠ Charlie→Bob); modularity and async oracles (e.g. CCIP-read) suggested. |
| **Reputation semantics** | “Reputons” / RFC 7071 mentioned; distinction between “credibility” vs **SLA/uptime**-style metrics. |
| **Validation interface** | Related work: escrow + pluggable validation, `checkObligation` **view** pattern; **EAS** for on-chain attestations; keep **core ERC small**, extend via optional interfaces. |
| **Escrow & payments** | Questions on how escrow / timelocks / staking fit the ERC; **protocol authors** (Marco-MetaMask, gpt3_eth): **payments not in scope** — stay unopinionated; optional **proof-of-payment** in off-chain schemas; interest in **A2A payment** extensions (e.g. **x402**) aligning with ERC-8004. |
| **ETH agent economy** | Community idea: agents demanding **ETH** for tasks as network effect — noted as broader than the ERC text. |
| **A2A / Agent Card hosting** | Tension: **well-known location** for Agent Card vs **URLs** / multi-agent per domain; **pcarranzav**: domain ownership verification hard (trusted party / zkTLS / multiple claims); preference for **registering Agent Card URL** in some suggestions. |
| **ERC-8001** | **KBryan:** coordination standard — synergy with 8004; **Marco-MetaMask:** 8001 ≈ consensus on attestations, **orthogonal** to 8004. |
| **Design rationale (author)** | Rating/reputation detail and validation **response** often **off-chain** or **event-only** because trust decisions use **aggregation**; gas and avoiding per-feedback txs; **open** to optional on-chain storage / bundlers. |
| **Concrete asks** | Expose data contracts can read: e.g. `getValidationResponse`-style **view** functions; optional on-chain **FeedbackData** struct / indexing patterns (EAS-indexer analogy). |
| **Author clarifications (Marco-MetaMask)** | Intent to clarify: **ERC-8004** makes **AgentCard at well-known location mandatory**; **one agent per domain** model (N-level subdomains / CI/CD); ERC to specify **types** more precisely over time; goal **singleton identity registry per chain** (if single-chain). |
| **gpt3_eth** | Do not embed **settlement** (credits, x402, etc.) in 8004; standardize **hooks** so feedback can **reference** payment proofs for indexers. |

## Why this thread matters for builders

- Clarifies **scope boundaries** (trust/discovery vs payments/settlement).
- Surfaces **ongoing debates**: on-chain readability vs gas, reputation aggregation, Agent Card **URL vs domain** rules.
- **PR + EIP** evolve — treat this file as **orientation**, not the latest spec text.

## Related links (from thread)

- [EIP-8004 on eips.ethereum.org](https://eips.ethereum.org/EIPS/eip-8004) (cross-check with current draft)
- [ERCs PR #1170](https://github.com/ethereum/ERCs/pull/1170) (verify still correct)
- [Google A2A announcement](https://developers.googleblog.com/en/a2a-a-new-era-of-agent-interoperability/) (cited in ecosystem context elsewhere)

---

*Reference only. Thread © contributors; excerpted for hackathon note-taking.*
