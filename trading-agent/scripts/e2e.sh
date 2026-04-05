#!/usr/bin/env bash
# Simple e2e smoke tests — curl endpoints, check shapes.
# Usage:
#   ./scripts/e2e.sh              # test local (builds + starts agent)
#   ./scripts/e2e.sh deployed     # test deployed infra (Render + Vercel + Sepolia)
set -uo pipefail

PASS=0; FAIL=0
RENDER_URL="${RENDER_URL:-https://trading-agent-95p9.onrender.com}"
VERCEL_URL="${VERCEL_URL:-https://erc8004-trading-agent.vercel.app}"
SEPOLIA_RPC="${SEPOLIA_RPC:-https://rpc.sepolia.org}"

check() {
  local name="$1"; shift
  if "$@" >/dev/null 2>&1; then
    echo "  PASS  $name"; ((PASS++))
  else
    echo "  FAIL  $name"; ((FAIL++))
  fi
}

json_has_key() {
  local url="$1" key="$2"
  curl -sf "$url" | grep -q "\"$key\""
}

http_ok() { curl -sf -o /dev/null "$1"; }

has_code() {
  local addr="$1"
  local resp
  resp=$(curl -sf -X POST "$SEPOLIA_RPC" \
    -H "Content-Type: application/json" \
    -d "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getCode\",\"params\":[\"$addr\",\"latest\"],\"id\":1}")
  echo "$resp" | grep -qv '"0x"'
}

# ── Deployed tests ──────────────────────────────────────────────
test_deployed() {
  echo "=== e2e: deployed targets ==="
  echo "  Render: $RENDER_URL"
  echo "  Vercel: $VERCEL_URL"
  echo ""

  check "GET /metrics shape"               json_has_key "$RENDER_URL/metrics" "ticks"
  check "GET /logs returns array"           bash -c "curl -sf '$RENDER_URL/logs' | grep -q '^\['"
  check "GET /agent-card has name"          json_has_key "$RENDER_URL/.well-known/agent-card.json" "name"
  check "GET /decision-schema non-empty"    bash -c "curl -sf '$RENDER_URL/decision-schema' | grep -q '{'"
  check "CORS header present"              bash -c "curl -sI '$RENDER_URL/metrics' | grep -iq 'access-control'"
  check "Vercel dashboard HTTP 200"         http_ok "$VERCEL_URL"
  check "Sepolia IdentityRegistry"          has_code "0xc83F0B94E7969Cc2265aB0A187Ba0F2e6A5B9554"
  check "Sepolia ReputationRegistry"        has_code "0x40dB57F7D848457289CEda81F39df15C4203D576"
  check "Sepolia RiskRouter"                has_code "0xCbC5DFeD364b6D65233DfA6edCcb95088F8f189B"
}

# ── Local tests ─────────────────────────────────────────────────
test_local() {
  echo "=== e2e: local (demo mode) ==="
  local PORT=13030

  echo "  Building release binary..."
  cargo build --release 2>/dev/null

  echo "  Starting agent on port $PORT..."
  AGENT_DEMO_MODE=true AGENT_HTTP_PORT=$PORT AGENT_INTERVAL_SECS=1 \
    AGENT_EXECUTION_MODE=paper AGENT_PAIR=BTCUSD AGENT_VOLUME=0.001 \
    CHAIN_RPC_URL="" PINATA_API_KEY="" \
    ./target/release/trading-agent &
  local PID=$!
  trap "kill $PID 2>/dev/null" EXIT

  # Wait for agent to start
  for i in $(seq 1 15); do
    curl -sf "http://localhost:$PORT/metrics" >/dev/null 2>&1 && break
    sleep 1
  done

  local URL="http://localhost:$PORT"
  check "GET /metrics shape"               json_has_key "$URL/metrics" "ticks"
  check "GET /logs returns array"           bash -c "curl -sf '$URL/logs' | grep -q '^\['"
  check "GET /agent-card has name"          json_has_key "$URL/.well-known/agent-card.json" "name"
  check "GET /decision-schema non-empty"    bash -c "curl -sf '$URL/decision-schema' | grep -q '{'"
  check "CORS header present"              bash -c "curl -sI '$URL/metrics' | grep -iq 'access-control'"

  # Wait for ticks to accumulate
  sleep 12
  check "Ticks accumulate"                 bash -c "curl -sf '$URL/metrics' | grep -oP '\"ticks\":\\K[0-9]+' | awk '\$1 > 1'"
  check "Zero errors in demo"              bash -c "curl -sf '$URL/metrics' | grep -q '\"errors\":0'"

  kill $PID 2>/dev/null
  trap - EXIT
}

# ── Main ────────────────────────────────────────────────────────
case "${1:-local}" in
  deployed) test_deployed ;;
  local)    test_local ;;
  all)      test_local; echo ""; test_deployed ;;
  *)        echo "Usage: $0 [local|deployed|all]"; exit 1 ;;
esac

echo ""
echo "=== $PASS passed, $FAIL failed ==="
exit $FAIL
