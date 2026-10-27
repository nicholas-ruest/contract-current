#!/usr/bin/env bash
set -euo pipefail
crates=$(find crates -mindepth 1 -maxdepth 1 -type d|wc -l);adrs=$(find docs/adr -name '[0-9][0-9][0-9][0-9]-*.md'|wc -l);ddds=$(find docs/ddd -name '[0-9][0-9]-*.md'|wc -l);test "$crates" -ge 20;test "$adrs" -eq 25;test "$ddds" -eq 12;echo "STRUCTURAL_PASS crates=$crates adrs=$adrs ddds=$ddds"
