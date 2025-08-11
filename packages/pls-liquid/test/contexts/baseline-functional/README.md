# Baseline Functional Context (PR2)

Objective
- Establish a precise functional baseline for `pls-liquid` (TypeScript) to support a future port.

Scope
- Deterministic tests (no network, no external services)
- Focus on current behavior: schemas, combinatorics, multisig generation
- Expose current limitations/bugs explicitly through assertions (no silent suppression)

Rules
- Start from `origin/main`, no local drift
- Keep tests isolated; use local fixtures only
- If current behavior rejects/throws, assert it explicitly and document it

Structure
- fixtures/: deterministic data used by tests
- unit/: unit tests for focused functions
- integration/: lightweight integration checks (e.g., module exports)
