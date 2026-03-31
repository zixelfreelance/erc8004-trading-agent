# ERC-8004: Trustless Agents — on-repo spec snapshot

**Canonical source (authoritative):** [EIP-8004](https://eips.ethereum.org/EIPS/eip-8004) (Draft, Standards Track: ERC)  
**Snapshot date:** 2026-03-31  
**Purpose:** Offline-oriented **digest** for hackathon work. **Do not** treat this file as the legal/spec text — diff against the live EIP before implementation.

---

## Metadata

| Field | Value |
|--------|--------|
| Title | ERC-8004: Trustless Agents — *Discover agents and establish trust through reputation and validation* |
| Authors | Marco De Rossi ([@MarcoMetaMask](https://github.com/MarcoMetaMask)), Davide Crapis ([@dcrapis](https://github.com/dcrapis)), Jordan Ellis, Erik Reppel |
| Created | 2025-08-13 |
| Discussion | [Ethereum Magicians — ERC-8004](https://ethereum-magicians.org/t/erc-8004-trustless-agents/25098) |
| Requires | [EIP-155](https://eips.ethereum.org/EIPS/eip-155), [EIP-712](https://eips.ethereum.org/EIPS/eip-712), [EIP-721](https://eips.ethereum.org/EIPS/eip-721), [EIP-1271](https://eips.ethereum.org/EIPS/eip-1271) |

---

## Abstract (essence)

Blockchains enable **discovering, choosing, and interacting** with **agents across organizations** without pre-existing trust — **open-ended agent economies**. Trust is **tiered** and **pluggable**: client feedback / reputation, stake-secured re-execution, **zkML**, **TEE** oracles, etc., scaled to task risk.

---

## Motivation (essence)

**MCP** exposes server capabilities; **A2A** covers auth, AgentCards, messaging, task lifecycle — but **not** full **discovery + trust** across untrusted parties. This ERC adds **three lightweight registries** (deploy on L2/mainnet, typically **per-chain singletons**):

1. **Identity** — ERC-721 + URI storage → agent registration file.  
2. **Reputation** — feedback signals; scoring on- and off-chain.  
3. **Validation** — hooks for independent validators (re-exec, zkML, TEE, judges).

**Payments** are **orthogonal**; examples mention **x402** enriching feedback signals, not defining settlement here.

---

## Global agent identifier

Each agent is identified by:

- **`agentRegistry`:** `{namespace}:{chainId}:{identityRegistry}` (e.g. `eip155:1:0x742…`)  
- **`agentId`:** ERC-721 `tokenId` (document uses *agentId* / *agentURI* synonymously with tokenId / tokenURI).

---

## Identity Registry — registration file (MUST shape)

`agentURI` MUST resolve to JSON with at least this structure (endpoints are extensible):

```json
{
  "type": "https://eips.ethereum.org/EIPS/eip-8004#registration-v1",
  "name": "myAgentName",
  "description": "A natural language description of the Agent, which MAY include what it does, how it works, pricing, and interaction methods",
  "image": "https://example.com/agentimage.png",
  "services": [
    { "name": "web", "endpoint": "https://web.agentxyz.com/" },
    {
      "name": "A2A",
      "endpoint": "https://agent.example/.well-known/agent-card.json",
      "version": "0.3.0"
    },
    {
      "name": "MCP",
      "endpoint": "https://mcp.agent.eth/",
      "version": "2025-06-18"
    },
    {
      "name": "OASF",
      "endpoint": "ipfs://{cid}",
      "version": "0.8",
      "skills": [],
      "domains": []
    },
    { "name": "ENS", "endpoint": "vitalik.eth", "version": "v1" },
    { "name": "DID", "endpoint": "did:method:foobar", "version": "v1" },
    { "name": "email", "endpoint": "mail@myagent.com" }
  ],
  "x402Support": false,
  "active": true,
  "registrations": [
    {
      "agentId": 22,
      "agentRegistry": "{namespace}:{chainId}:{identityRegistry}"
    }
  ],
  "supportedTrust": ["reputation", "crypto-economic", "tee-attestation"]
}
```

- Top-level `type`, `name`, `description`, `image` SHOULD stay compatible with ERC-721 consumers.  
- `supportedTrust` is OPTIONAL; if absent/empty, ERC is **discovery-only** (no trust advertisement).  
- **Optional** HTTPS domain verification: `https://{endpoint-domain}/.well-known/agent-registration.json` with matching `registrations` entry.

### Identity — notable on-chain behavior

- **`getMetadata` / `setMetadata`** — optional key/value bytes; event `MetadataSet`.  
- Reserved key **`agentWallet`** — not settable via normal metadata; set via **`setAgentWallet`** with **EIP-712** (EOA) or **EIP-1271** (contract wallet); cleared on NFT transfer.  
- **`register`** overloads (with/without URI, optional metadata array) → `Registered` (+ `Transfer`, metadata events).  
- **`setAgentURI`** → `URIUpdated`. On-chain JSON via **`data:application/json;base64,...`** encouraged when storing full file on-chain.

---

## Reputation Registry

- Initialized with **`initialize(identityRegistry)`**; **`getIdentityRegistry()`** view.  
- **`giveFeedback(agentId, value, valueDecimals, tag1, tag2, endpoint, feedbackURI, feedbackHash)`**  
  - `valueDecimals` ∈ [0, 18]; submitter MUST NOT be owner/operator of `agentId`.  
  - Several string/hash fields OPTIONAL; `feedbackHash` commits to off-chain file when not content-addressed.  
- **`NewFeedback`** event; **value, decimals, tags, `isRevoked`** stored on-chain; endpoint/URI/hash **emitted but not stored** (per draft).  
- Agent-clients SHOULD use **`agentWallet`** as `clientAddress` for aggregation.  
- **`revokeFeedback`**, **`appendResponse`** (+ events).  
- **Read:** `getSummary` (requires **non-empty `clientAddresses`** filter — Sybil note), `readFeedback`, `readAllFeedback`, `getResponseCount`, `getClients`, `getLastIndex`.

### Off-chain feedback file (illustrative)

Must include: `agentRegistry`, `agentId`, `clientAddress`, `createdAt`, `value`, `valueDecimals`. Optional: tags, endpoint, `mcp` / `a2a` / `oasf` blocks, **`proofOfPayment`** (x402-friendly), etc. See EIP for full template.

### Example `tag1` / `value` semantics (from EIP table)

Includes rows such as **starred** (0–100 quality), **reachable**, **ownerVerified**, **uptime**, **successRate**, **responseTime**, **revenues**, **tradingYield** (with `tag2` = day/week/month/year), etc.

---

## Validation Registry

- **`validationRequest(validatorAddress, agentId, requestURI, requestHash)`** — caller = owner/operator of `agentId`; `requestHash` commits to payload.  
- **`validationResponse(requestHash, response, responseURI, responseHash, tag)`** — caller = chosen validator; `response` ∈ [0, 100]; multiple responses per hash allowed (e.g. progressive tags).  
- Stores fields for **on-chain querying** (`requestHash`, validator, agentId, response, `responseHash`, `lastUpdate`, `tag`).  
- **Read:** `getValidationStatus`, `getSummary` (avg response), `getAgentValidations`, `getValidatorRequests`.  
- Validator **incentives / slashing** — **out of scope** of this registry.

---

## Rationale (bullets)

- **Flexible registration file** links MCP, A2A, wallets, ENS, DIDs — not one wire protocol.  
- **Feedback** aligns tags with A2A (tasks/skills) and MCP (tools/prompts) while keeping signal structure flexible.  
- **Gas:** clients need not be registered agents; frictionless feedback via **EIP-7702** mentioned.  
- **Indexing:** on-chain signals + IPFS-friendly URIs → subgraphs/indexers.  
- **Deployment:** **singletons per chain** expected; agents may operate cross-chain or multi-register.

---

## Security considerations (bullets)

- **Sybil / spam** on reputation — protocol standardizes **public schema**; expect **reviewer reputation**, filtering (protocol enables filtering by reviewer).  
- **Immutability** of pointers/hashes supports audit trails.  
- **Validator economics** live in separate protocols.  
- Registration authenticity ≠ **capability honesty** — reputation, validation, TEE attestations address that gap.

---

## Copyright

CC0 (per EIP site).

---

## Citation (abbrev.)

Marco De Rossi et al., *“ERC-8004: Trustless Agents [DRAFT],”* Ethereum Improvement Proposals, no. 8004, August 2025. [Online]. Available: `https://eips.ethereum.org/EIPS/eip-8004`

---

*End of snapshot — compare line-by-line with [EIP-8004](https://eips.ethereum.org/EIPS/eip-8004) before audits or deployments.*
