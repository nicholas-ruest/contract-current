#!/usr/bin/env bash
set -euo pipefail
grep -q '/v1/changes' openapi.yaml
grep -q '/v1/changes' frontend/src/client.ts
echo CONTRACT_PASS
