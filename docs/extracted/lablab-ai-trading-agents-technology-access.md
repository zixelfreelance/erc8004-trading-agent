# AI Trading Agents hackathon — Technology & Access (extracted)

**Source:** LabLab event copy (“Technology & Access”); align URLs with the **live** hackathon page.  
**Captured:** 2026-03-31 (user paste)  
**Related:** [`lablab-ai-trading-agents-rules-research.md`](lablab-ai-trading-agents-rules-research.md) — judging, submission TBA, Surge eligibility (no secrets in repo).

This hackathon has **two equal challenges**. Teams may enter **one or both**. Requirements below are **per challenge** unless noted.

---

## Kraken challenge — required technology

- **Kraken CLI:** zero-dependency **Rust** binary with built-in **MCP** server and full **Kraken API** connectivity.
- **Kraken API key (read-only)** linked to the trading account — for **leaderboard verification**; **no withdrawal** access required.

### How to get started (Kraken)

1. Install Kraken CLI from the **official repository**.
2. Configure API access and connect it to your agent.
3. Build your agent: autonomous workflow where AI analyzes market signals and executes trades **programmatically through Kraken CLI**.
4. Share progress publicly as part of the **Social Engagement** challenge.

### Resources (Kraken)

| Resource | URL (use live page if this drifts) |
|----------|--------------------------------------|
| Kraken CLI GitHub | [github.com/krakenfx/kraken-cli](https://github.com/krakenfx/kraken-cli) |
| Kraken CLI announcement | [Kraken blog: Announcing the Kraken CLI](https://blog.kraken.com/news/industry-news/announcing-the-kraken-cli) |

---

## ERC-8004 challenge — required technology

- **ERC-8004 registries** for **Agent Identity**, **Reputation**, and **Validation**. Deploy on an **L2** or **testnet** (mainnet optional).
- **EIP-712** typed data signatures for **trade intents** and **attestations**.
- **EIP-1271** support for smart-contract wallets; **EIP-155** chain-id binding.
- **DEX execution** via a **whitelisted Risk Router** contract (hackathon-provided — e.g. Uniswap-style routers).

### How to access the infrastructure (ERC-8004)

1. **Register your agent:** Mint and register an **Agent Identity (ERC-721)** pointing to **Agent Registration JSON**, capabilities, endpoints, and agent wallet.
2. **Claim sandbox capital:** Funded **sub-account** in the **Hackathon Capital Vault** (test funds by default; optional small real-capital pool for finals).
3. **Execute via Risk Router:** Agents submit signed **TradeIntents**. Router enforces position size limits, max leverage, whitelisted markets, daily loss limits.
4. **Record trust signals:** Trades and checkpoints emit on-chain events; validators post scores to the **Validation Registry**; PnL and validator scores feed **reputation**.
5. **Leaderboard:** lablab.ai publishes leaderboard: **PnL**, **Sharpe**, **drawdown**, **validation score**.

### Resources (ERC-8004 / specs)

Use the **official hackathon links** for authoritative URLs. Typical labels from the event page:

- **EIP-8004** specification (event copy uses this label alongside ERC-8004).
- ERC-8004 **forum** discussion.
- **Developer walkthrough** (Solidity + JS examples).
- **Optional curated resources**.

---

## Optional enhancements (either or both challenges)

Not required; can strengthen a submission:

- **TEE-backed** attestations or verifiable execution proofs (e.g. TEE oracle; zkML validation).
- **Off-chain indexers** or **subgraphs** for discovery dashboards and leaderboards.
- **Portfolio risk** modules (limits, circuit breakers, stop-loss) **enforced on-chain**.

---

## Local workspace — curated ERC-8004 links

Unofficial **awesome list** (community-maintained): clone with `gh repo clone sudeepb02/awesome-erc8004` → **`awesome-erc8004/`** at repo root. This parent repo **gitignores** that folder so its history stays separate; refresh with `git -C awesome-erc8004 pull`.

---

*Reference only. Requirements and links may change on the organizer’s site.*
