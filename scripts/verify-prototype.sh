#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR=/tmp/contract-current-target
trap './scripts/stop.sh' EXIT
mkdir -p evidence/prototype
run(){ echo "RUN $*"; "$@"; }
run ./scripts/check-design.sh
run cargo fmt --all -- --check
run cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
run cargo test --locked --workspace --all-features
run cargo deny check
run bash -lc 'cd frontend && npm audit'
run bash -lc 'cd frontend && npm run typecheck && npm run lint && npm test && npm run build'
run ./scripts/check-contract.sh
run ./scripts/benchmark.sh
run ./scripts/reset.sh
run ./scripts/start.sh
run ./scripts/restart-journey.sh
run bash -lc 'cd frontend && npx playwright test'
echo PROTOTYPE_PASS
