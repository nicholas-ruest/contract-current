# Prototype acceptance

## Journey 1 - issue a governed migration plan contract

- UI: home form, **Form governed migration plan**
- API/middleware: POST `/v1/changes`; bearer auth; validated JSON; generated correlation/idempotency key; 10-second timeout; structured error
- Application/crates: `cc-api -> cc-orchestrator -> cc-decision/cc-impact-graph -> cc-compatibility/cc-calibration -> cc-migration-plan/cc-wave-optimizer/cc-risk-budget -> cc-decision/cc-policy`
- Upstream/persistence: AWS Cedar policy evaluation, RuVector recall/insert, RVF append
- Visible evidence: issued status, risk, affected consumers and migration waves and decision digest; desktop/mobile screenshot and trace

## Journey 2 - deny without effect, then recover

- UI: submit a breaking change during a frozen window, submit, inspect alert, refresh audit, correct the window and retry
- API/middleware: 403 error retains correlation ID; authorization runs before RVF append
- Application/crates: policy anti-corruption boundary calls real AWS Cedar; repository is not invoked on denial
- Proof: Playwright records audit row count before/after denial and requires no change, then requires one new row after recovery

## Journey 3 - retrieve after refresh and service restart

- UI: create, reload page, refresh durable audit
- API/middleware: GET `/v1/changes` with bearer authorization
- Application/crates: `cc-repository -> cc-audit -> cc-rvf`
- Proof: Playwright validates refresh; `scripts/restart-journey.sh` creates a migration decision, terminates the actual Rust process, restarts it against the same RVF path, and retrieves the exact saved ID

No browser acceptance route is intercepted or mocked. Seeded consumers are deterministic and labelled synthetic. Test effects are isolated to `data/contract-current.rvf` and removed by `scripts/reset.sh`.
