#!/usr/bin/env bash
# Local source verification without modifying Git history.
set -euo pipefail
cd "$(dirname "$0")/.."
git --version
cargo --version
cargo check --locked --all-targets
cargo test --locked --all-targets
echo "PRYSM source checks passed locally; GitHub Actions release gate is separate."
