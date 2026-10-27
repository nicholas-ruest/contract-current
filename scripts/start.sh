#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR=/tmp/contract-current-target
mkdir -p evidence/prototype data
CC_BIND=127.0.0.1:18080 CC_DATA="${CC_DATA:-data/decisions.rvf}" CC_TOKEN=demo-token RUST_LOG=info cargo run -p cc-cli >evidence/prototype/backend.log 2>&1 &
echo $! > /tmp/contract-current-backend.pid
(cd frontend && exec ./node_modules/.bin/vite --host 127.0.0.1 --port 5173) >evidence/prototype/frontend.log 2>&1 &
echo $! > /tmp/contract-current-frontend.pid
for _ in $(seq 1 90); do
  if curl -fsS http://127.0.0.1:18080/readyz >/dev/null 2>&1 && curl -fsS http://127.0.0.1:5173 >/dev/null 2>&1; then exit 0; fi
  sleep 1
done
echo readiness timeout >&2
exit 1
