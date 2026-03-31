# Kraken CLI announcement — extracted notes

**Source:** [Announcing the Kraken CLI: the best crypto trading tool built for AI agents](https://blog.kraken.com/news/industry-news/announcing-the-kraken-cli) (Kraken Blog, Industry News)  
**Extracted:** 2026-03-31 (workspace reference; verify live copy on Kraken’s site for legal/compliance use)

## Prominent risk notice (page)

> Don’t invest unless you’re prepared to lose all the money you invest. This is a high-risk investment and you should not expect to be protected if something goes wrong. [Take 2 minutes to learn more](https://www.kraken.com/legal/uk/disclaimer)

## TL;DR (as stated by Kraken)

- **Kraken CLI** — open-source, **single-binary** execution engine; **direct, native** access to crypto markets for AI agents and developers.
- **Scope:** spot, futures, staking, subaccount transfers, WebSocket streaming; **134 commands**; **zero-dependency Rust** binary; **NDJSON** output oriented to machines.
- **MCP:** built-in **Model Context Protocol** server — `kraken mcp`; positioned for agentic tools (e.g. Claude Code, Codex, Cursor) and terminal-native agent environments (e.g. OpenCode, OpenClaw); avoids custom API wrappers, nonce handling, HMAC signing from the agent side.
- **Paper trading:** local engine vs **live public ticker**; simulated balances; limit/market orders; unrealized P&L **offline**.

## Narrative: why CLI vs raw API

- Raw exchange APIs force agents/scripts to handle nonces, **HMAC-SHA512** signing, retries — error-prone and token-heavy.
- CLI abstracts connectivity; Rust, zero-dependency; focus on **trading logic**.

## Key features (summary)

| Area | Summary |
|------|---------|
| MCP | `kraken mcp` — self-describing plugin; schemas for commands |
| Paper | Local state + live ticker; no capital at risk for tests |
| Shell | `-o json` / NDJSON across commands; pipe to `jq` / LLM context |
| Surface | Spot, futures, staking/earn, subaccounts, WebSocket |
| Resilience | Spot counter/decay; futures token-bucket rate limiting |
| Install | One-line curl installer (see below) |

## One-line install (from article)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/krakenfx/kraken-cli/releases/latest/download/kraken-cli-installer.sh | sh
```

## Primary links

- **GitHub:** [github.com/krakenfx/kraken-cli](https://github.com/krakenfx/kraken-cli)
- **Risk disclosure** (derivatives / leverage): referenced in article; see Kraken’s current **Risk Disclosure** on their site.

## Footer disclaimer (abridged)

Materials are general information only; not investment advice; not a recommendation to buy/sell/stake/hold; crypto can be unregulated; loss of funds possible; tax may apply — seek independent advice.

## Local workspace clone

This repo keeps a sibling checkout at **`kraken-cli/`** (from `gh repo clone krakenfx/kraken-cli`). It is **gitignored** here so `hack01` does not embed upstream’s history. Update the tool with `git -C kraken-cli pull` from the workspace root.

---

*This file is a project reference extraction, not legal or investment advice.*
