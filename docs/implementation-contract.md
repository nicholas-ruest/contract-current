# Implementation contract

Build browser-to-storage vertical slices. Public APIs use validated types and explicit errors. The application calls real AWS Cedar, RuVector and RVF adapters. The frontend displays real backend results and loading, empty, denial, recovery and success states.

A counted crate owns a distinct invariant or boundary, exposes typed behavior, has a caller or executable role and is tested. No service-owned database, committed secret, mocked first-party browser route, policy bypass, force push or implementation directly on main is permitted.

The verifier runs Rust quality, locked tests, cargo-deny, frontend quality/build, OpenAPI drift, middleware/failure tests, three Playwright journeys, restart retrieval, benchmarks, ablations and container builds. CI resolves evidence markers to github.sha and fails closed.
