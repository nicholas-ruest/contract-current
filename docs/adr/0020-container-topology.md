# ADR-0020: Container topology

Status: Accepted for the 2026-10-26 prototype.

## Context
The real browser-to-storage path needs an explicit decision, ownership and failure contract. Structural presence alone cannot satisfy this decision.

## Decision
Build independent API/web containers with health checks.

## Consequences
The owning crate exposes typed behavior, has a real caller and returns an explicit error on failure. This increases focused-test and traceability obligations while preventing silent fallback.

## Verification
Compose build.
