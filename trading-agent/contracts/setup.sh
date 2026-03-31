#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

# Install Foundry if not available
if ! command -v forge &> /dev/null; then
    echo "Installing Foundry..."
    curl -L https://foundry.paradigm.xyz | bash
    source "$HOME/.bashrc" 2>/dev/null || source "$HOME/.zshrc" 2>/dev/null || true
    foundryup
fi

echo "Forge version: $(forge --version)"

# Initialize forge (installs forge-std into lib/)
forge init --no-commit --force .

# Install OpenZeppelin contracts v5
forge install OpenZeppelin/openzeppelin-contracts --no-commit

# Build
forge build

# Test
forge test -vvv
