# Strykr PRISM — product / marketing page extraction

**Product:** PRISM API — built by **Strykr**  
**API base (examples):** `https://api.prismapi.ai`  
**Captured:** 2026-03-31 (user paste; **verify** copy, limits, and URLs on [PRISM / Strykr live sites](https://api.prismapi.ai) and official docs)

## Positioning

- **One API to resolve every financial asset** — canonical data layer for **AI agents**.
- Resolve ticker, contract, or symbol to a **single source of truth** across **crypto, equities, DeFi** (and TradFi in API grouping).

## Claimed metrics (marketing)

| Metric | Value (as stated) |
|--------|-------------------|
| P50 latency | **&lt;50ms** (hero); example resolve **~9ms** |
| Assets | **5,000+** |
| Users / developers | **15,000+** |
| Venues | **50+** (example response lists Binance, Coinbase, …) |

## Example: resolve (hero)

```bash
curl https://api.prismapi.ai/resolve/BTC
```

Response shape (illustrative):

- `object`: resolved_asset  
- `symbol`, `name`, `price_usd`, `confidence`, `latency_ms`, `venues` (array)

*Note:* LabLab hackathon materials also referenced `curl -H "X-API-Key: YOUR_KEY" https://api.prismapi.ai/resolve/BTC` — production use typically requires an API key header; do **not** commit real keys.

## Resolution engine (features)

| Capability | Example |
|------------|---------|
| Fuzzy match | `BTCON` → `BTC` |
| Rebrand tracked | `MATIC` → `POL` |
| Collision resolved | `COIN?ctx=stock` vs crypto `COIN` |
| Contract resolved | `0x2260…` → `WBTC` |
| Venue mapped | `NVDA` → venues (NASDAQ, NYSE, …) |
| Family grouped | `BTC?expand` → WBTC, cbBTC, tBTC, … |

## 4-layer resolution (as described)

1. **Cache** — hot path, **&lt;1ms**  
2. **Deterministic match** — exact + alias, **&lt;5ms**  
3. **Priority rules** — collision + rebrand, **&lt;20ms**  
4. **LLM fallback** — fuzzy resolution, **&lt;200ms**

## Example: query-style resolve

`GET /v1/resolve?q=BTCON&context=crypto` → `canonical`, `confidence`, `method` (e.g. `fuzzy_match`), `latency_ms`.

## Architecture narrative

“Eyes before the hands”: agents → **PRISM** → trade routing (CEX / DEX / TradFi). *(Marketing diagram claims may use placeholder zeros for counters on static pages.)*

## API surface (high level)

| Group | Count (stated) | Examples |
|-------|----------------|----------|
| Resolution | 12 | `GET /v1/resolve`, `POST /v1/resolve/batch`, `GET /v1/family/{symbol}`, `GET /v1/venues/{symbol}` |
| Crypto | 35+ | `.../crypto/price/{symbol}`, `markets`, `ohlcv`, `metadata` |
| TradFi | 30+ | `.../tradfi/quote`, `historical`, `financials`, `screener` |
| DEX | 25+ | `pairs/{address}`, `tokens/trending`, `pools/{chain}`, `POST .../dex/search` |

**Agent-native:** MCP server, structured JSON, confidence scores; example tool call `resolve_asset("BTCON")`.

## Integration paths

- **REST** (curl with `X-API-Key: prism_sk_...`)  
- **Python SDK**  
- **MCP Server** for agents  

## Pricing tiers (as pasted — confirm live)

| Tier | Price | Limits (stated) |
|------|--------|-----------------|
| Free | $0/mo | 1 QPS burst, **1,000 requests/day**, core endpoints |
| Starter | $19.99/mo | 3 QPS burst, 5,000/day, all endpoints |
| Dev | $49.99/mo | 10 QPS burst, 50,000/day, priority support |
| Pro | $199.99/mo | 30 QPS burst, 500,000/day, dedicated support |
| Enterprise | Custom | Custom QPS, infra, SLAs |

*Free tier:* “No credit card”, **1,000 free requests/day** (also stated elsewhere as dashboard copy).

## Footer / related

- Links typically include: API Docs, Glossary, Asset Registry, Strykr, Terms, Privacy — **resolve on live site**.

## Hackathon context (LabLab)

Organizers may offer PRISM credits / promo code (e.g. **LABLAB**) and note subscription/cancellation — follow **current** hackathon and vendor terms; LabLab may disclaim third-party credit delivery.

---

*Reference only. Not affiliated with Strykr or PRISM. Pricing and limits change; use official docs for integration.*
