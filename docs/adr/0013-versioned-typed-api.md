# ADR-0013: Versioned typed API

Status: Accepted for the 2026-10-26 prototype.

## Context
The real browser-to-storage path needs an explicit decision, ownership and failure contract. Structural presence alone cannot satisfy this decision.

## Decision
Expose health/readiness and v1 contract/audit endpoints with explicit errors.

## Consequences
The owning crate exposes typed behavior, has a real caller and returns an explicit error on failure. This increases focused-test and traceability obligations while preventing silent fallback.

## Verification
OpenAPI.
