# ERC-8004: Building the Trustless Agent Layer of Ethereum — extraction

**Source:** [Medium — Siddhant (@savvysid)](https://medium.com/@savvysid/erc-8004-building-the-trustless-agent-layer-of-ethereum-0eec8b9ad112)  
**Published:** 2025-10-25 · ~8 min read  
**Extracted:** 2026-03-31 (structured notes; **full code and prose** — use the original post)

## Thesis

Autonomous agents (DeFi bots, oracles, agentic SaaS) need **discovery** and **trust** across boundaries: find agents, prove capabilities, justify correct outputs. **ERC-8004** proposes **three on-chain registries** — **Identity**, **Reputation**, **Validation** — linking agents to **off-chain** capability descriptions and validation (reputation, crypto-economic validators, **TEE** / **ZK**-style attestations). Article walks overview, Solidity/JS-style examples, and an **Oasis ROFL** (TEE) workflow sketch.

## What ERC-8004 provides (author summary)

| Registry | Role |
|----------|------|
| **Identity** | ERC-721-style handle (`tokenId` → `tokenURI`) → off-chain JSON (endpoints, wallets, trust models). |
| **Reputation** | API for **authorized** clients to post feedback (score, tags, optional URI + hash); composable **on-chain** signals, richer data **off-chain**. |
| **Validation** | Request/response flow; validators respond (e.g. **0–100**); supports re-execution, zkML, TEE attestations. |

## Article structure (for navigation)

1. **Identity Registry** — Simplified `IdentityRegistry` (OZ `ERC721URIStorage`): `register(tokenURI)`, optional `setMetadata` / `getMetadata`, events for indexers. Explains `tokenURI` as pointer to registration JSON (IPFS/HTTPS/DID).
2. **Reputation Registry** — `IReputationRegistry.giveFeedback(...)` sketch; **`feedbackAuth`** (EIP-191 / ERC-1271) to bind client and limit spam; `NewFeedback` events; off-chain aggregation.
3. **Validation Registry + TEE** — Request (`requestUri`, `requestHash`) / response pattern; ROFL-style attestation flow (work in TEE → post request → validator posts response). Notes **off-chain** attestation verification then **on-chain** result.
4. **Hardhat demo** — Deploy identity, `register`, parse `Registered` for `agentId` (production: compliant ERC-8004 registration JSON).
5. **Oasis ROFL** — Conceptual JS using a **hypothetical** `@oasisprotocol/rofl-js`-style API (`deploy`, `execute`, `attest`); ethers snippet posting `validationResponse` with IPFS attestation URI/hash; workflow steps; use cases (DeFi bots, sensitive analytics, marketplaces filtering on `supportedTrust: tee-attestation`).
6. **Agent Explorer + Reputation Aggregator** — Index `Registered`, fetch `tokenURI`, filters; subscribe to feedback, Sybil resistance, weighted scores.
7. **Risks / practices** — Sybil/spam (authorization + reviewer reputation/stake); MEV/`tokenURI` front-running; keep on-chain **compact**, proofs **off-chain**; TEE vs ZK tradeoffs.

## Conclusion (paraphrase)

Minimal **on-chain** primitives + **off-chain** indexers/aggregators + verifiable execution (ROFL/TEE, zkML) → practical **trustless agents**. Builders: start with identity + IPFS JSON, prototype ROFL attestations, simple reputation ingestion.

## References (from article)

- [EIP-8004 (eips.ethereum.org)](https://eips.ethereum.org/EIPS/eip-8004)
- [Ethereum Magicians — ERC-8004 discussion](https://ethereum-magicians.org/t/erc-8004-trustless-agents/25098)
- [Google A2A (agent interoperability)](https://developers.googleblog.com/en/a2a-a-new-era-of-agent-interoperability/)
- [Oasis ROFL docs](https://docs.oasis.io/build/rofl/)

## Caveats for builders

- Article **Solidity/JS** is **educational**; audit and match **current** EIP-8004 / deployment interfaces before production.
- ROFL JavaScript is described with a **hypothetical** SDK pattern — confirm against **current** Oasis SDKs and docs.
- This file is **not** a substitute for the EIP or official hackathon contracts (e.g. Risk Router).

---

*Reference only. Copyright remains with the author and Medium.*
