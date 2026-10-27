#!/usr/bin/env bash
set -euo pipefail
./scripts/stop.sh
rm -rf data evidence/prototype/*.log evidence/prototype/*.png frontend/test-results frontend/playwright-report
mkdir -p data evidence/prototype
