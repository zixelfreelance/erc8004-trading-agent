#!/usr/bin/env bash
set -euo pipefail

# Usage: ./deploy.sh
# Required env vars:
#   DEPLOYER_PRIVATE_KEY  - private key with Sepolia ETH
#   RPC_URL              - Sepolia RPC (e.g. https://eth-sepolia.g.alchemy.com/v2/KEY)

if [ -z "${DEPLOYER_PRIVATE_KEY:-}" ]; then
    echo "Error: DEPLOYER_PRIVATE_KEY not set"
    exit 1
fi

if [ -z "${RPC_URL:-}" ]; then
    echo "Error: RPC_URL not set"
    exit 1
fi

echo "Deploying AgentIdentityRegistry to Sepolia..."
forge script script/Deploy.s.sol:DeployIdentityRegistry \
    --rpc-url "$RPC_URL" \
    --broadcast \
    --verify \
    -vvvv

echo "Done. Copy the deployed address to your .env as IDENTITY_REGISTRY=0x..."
