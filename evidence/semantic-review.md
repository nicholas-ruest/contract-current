# Semantic evidence review

Candidate: `56e3c1766d90d149e384a67b8eaa2f6a4ef0bc3d`

Review method: direct inspection of every workspace manifest and Rust source file; `cargo metadata`/dependency edges; focused tests; the composed `cc-orchestrator -> cc-api -> cc-cli` path; real AWS Cedar, RuVector and RVF adapter tests; Playwright and restart evidence. A directory count was not accepted as evidence.

## Result: PASS

All 28 members expose typed behavior or a justified executable role, appear on a real caller path, and have focused or whole-system evidence. None is empty, generated-only, mock-only or an unused adapter. The lowest-line-count members are role-justified: `cc-cli` is the service entrypoint, `cc-seed` owns deterministic registry fixtures entering the real orchestrator, and `cc-recall` is the typed boundary coupling embeddings to RuVector search. The remaining domain crates implement validation, DAG ordering, compatibility coverage, calibration/abstention, consumer migration plan formation, optimization, budgets, contract hashing/scopes, policy, append-only RVF evidence, repository recovery, authentication, correlation, orchestration and HTTP delivery.

## Call-path findings

- Primary: React control -> POST /v1/changes -> auth/correlation -> contract validation/semantic schema diff/impact graph -> compatibility/ReCIRC calibration -> migration-plan/wave-optimizer/risk-budget -> AWS Cedar -> RuVector recall -> RVF audit -> rendered consumer migration plan.
- Denial: the same path reaches AWS Cedar before RuVector insert or RVF append; browser evidence proves no durable row appears.
- Recovery: a real process termination/restart reopens RVF and returns the exact saved migration decision ID.
- Evaluation: the ablation executable calls actual AWS Cedar, RuVector and RVF APIs and demonstrates the missing capability for each removed ingredient.

## Counts inspected

28 workspace crates; 27 focused Rust tests plus the executable-role crate; 25 decision ADRs; 12 DDD documents; 6 browser journeys across desktop/mobile; one restart process journey. Exact commands and artifact hashes are recorded in the prototype manifest generated for this candidate.
