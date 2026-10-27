#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR=/tmp/contract-current-target
auth=(-H 'Authorization: Bearer demo-token')
body='{"revision":"Restore this governed decision after a real service restart","change_class":"standard"}'
response=$(curl -fsS "${auth[@]}" -H 'Content-Type: application/json' -d "$body" http://127.0.0.1:18080/v1/changes)
id=$(printf '%s' "$response" | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>console.log(JSON.parse(s).id))')
kill "$(cat /tmp/contract-current-backend.pid)"
for _ in $(seq 1 30); do curl -fsS http://127.0.0.1:18080/readyz >/dev/null 2>&1 || break; sleep 1; done
CC_BIND=127.0.0.1:18080 CC_DATA=data/decisions.rvf CC_TOKEN=demo-token RUST_LOG=info cargo run -p cc-cli >>evidence/prototype/backend.log 2>&1 &
echo $! > /tmp/contract-current-backend.pid
for _ in $(seq 1 60); do curl -fsS http://127.0.0.1:18080/readyz >/dev/null 2>&1 && break; sleep 1; done
curl -fsS "${auth[@]}" http://127.0.0.1:18080/v1/changes | node -e 'let s="";const id=process.argv[1];process.stdin.on("data",d=>s+=d).on("end",()=>{const a=JSON.parse(s);if(!a.some(x=>x.id===id))process.exit(1);console.log("RESTART_RETRIEVAL_PASS "+id)})' "$id"
