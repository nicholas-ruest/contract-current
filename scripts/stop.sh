#!/usr/bin/env bash
set -euo pipefail
for f in /tmp/contract-current-backend.pid /tmp/contract-current-frontend.pid;do if test -f "$f";then kill "$(cat "$f")" 2>/dev/null || true;rm -f "$f";fi;done
