#!/usr/bin/env bash
set -euo pipefail

echo "Building contracts..."
cargo build --target wasm32-unknown-unknown --release

echo "Deploying contracts to local devnet..."
# TODO: Add soroban CLI deploy commands
# soroban contract deploy --wasm target/wasm32-unknown-unknown/release/credit_token.wasm --network local

echo "Done."
