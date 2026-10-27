# Architecture

## Contexts
Contract Evolution owns OpenAPI snapshots and semantic diffs; Consumer Topology owns transitive dependency graphs. Consumer Registry owns identity and trust zones. Impact Intelligence owns compatibility observations and ReCIRC risk. Governed Migration owns waves, risk budgets and rollback order. Governance owns AWS Cedar decisions. Evidence owns append-only decision state. Intelligence/Evidence owns RuVector recall, RVF durability and audit. Delivery owns HTTP, middleware and React.

## Runtime path
cc-cli -> cc-api -> cc-auth/cc-middleware -> cc-orchestrator -> contract/schema-diff/consumer/impact-graph/compatibility/calibration/migration-plan/wave-optimizer/risk-budget/decision/policy/recall -> cedar/ruvector/embedding -> repository/audit/rvf.

## Crate map
Every crate has a typed API, caller or executable role, design link and focused or journey evidence.

| # | Crate | Responsibility | Context | Caller | Acceptance |
|---:|---|---|---|---|---|
| 1 | cc-types | shared value objects and DTOs | Shared Kernel | all domain crates | focused test + journey |
| 2 | cc-errors | typed errors | Shared Kernel | API/application | focused test + journey |
| 3 | cc-config | validated runtime configuration | Platform | cc-cli | focused test + journey |
| 4 | cc-contract | typed OpenAPI revision aggregate | API Revision Graph | cc-orchestrator | focused test + journey |
| 5 | cc-schema-diff | semantic field and operation compatibility changes | Contract Evolution | cc-impact-graph | focused test + journey |
| 6 | cc-impact-graph | dependency-safe consumer-impact DAG | API Revision Graph | cc-orchestrator | focused test + journey |
| 7 | cc-consumer | consumer identity and trust zone | Consumer Registry | cc-migration-plan | focused test + journey |
| 8 | cc-compatibility | contextual compatibility evidence | Capability Assurance | cc-calibration | focused test + journey |
| 9 | cc-calibration | input-adaptive conformal risk bound | Capability Assurance | cc-migration-plan | focused test + journey |
| 10 | cc-migration-plan | multi-role migration plan formation | Migration Plan Formation | cc-orchestrator | focused test + journey |
| 11 | cc-wave-optimizer | constrained utility search | Migration Plan Formation | cc-migration-plan | focused test + journey |
| 12 | cc-risk-budget | hard cost/concurrency limits | Migration Plan Formation | cc-wave-optimizer | focused test + journey |
| 13 | cc-decision | least-authority migration decision | Contract Lifecycle | cc-orchestrator | focused test + journey |
| 14 | cc-policy | typed policy decision | Governance | cc-orchestrator | focused test + journey |
| 15 | cc-cedar | AWS Cedar adapter | Governance | cc-policy | focused test + journey |
| 16 | cc-embedding | contract/consumer embedding | Intelligence | cc-recall | focused test + journey |
| 17 | cc-ruvector | RuVector adapter | Intelligence | cc-recall | focused test + journey |
| 18 | cc-recall | analogue migration plan recall | Intelligence | cc-orchestrator | focused test + journey |
| 19 | cc-rvf | RVF append/reopen/read | Evidence | cc-repository | focused test + journey |
| 20 | cc-audit | hash-chained evidence | Evidence | cc-repository | focused test + journey |
| 21 | cc-repository | single persistence boundary | Evidence | cc-orchestrator | focused test + journey |
| 22 | cc-auth | prototype principal | Delivery | cc-api | focused test + journey |
| 23 | cc-middleware | idempotency/correlation | Delivery | cc-api | focused test + journey |
| 24 | cc-orchestrator | contract-evolution use cases | Application | cc-api/cc-cli | focused test + journey |
| 25 | cc-api | versioned Axum API | Delivery | cc-cli | focused test + journey |
| 26 | cc-seed | synthetic workload | Evaluation | cc-cli | focused test + journey |
| 27 | cc-evaluation | benchmarks/ablations | Evaluation | cc-cli | focused test + journey |
| 28 | cc-cli | service executable | Delivery | operator | focused test + journey |

## Topology
React/Vite proxies to stateless Axum. Only the API owns upstream adapters. RVF is the sole durable store. Browser code has no credentials. Production Cloud Run requires separately authorized shared RuVector/Cloud SQL.

## Failure order
Auth and validation precede orchestration. Budgets and calibration precede policy. AWS Cedar denial precedes vector or RVF writes. Append failure prevents success. Timeouts and shutdown bound execution.
