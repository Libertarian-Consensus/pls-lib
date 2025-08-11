# Baseline Functional Context (PR1)

Objective
- Establish a precise functional baseline for `pls-bitcoin` (TypeScript) to support a future port.

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

Run Report (latest)
- Date: 2025-08-11
- Runner: Vitest v1.6.1 (node)
- Results:
  - Test Files: 4 passed (4)
  - Tests: 18 passed (18)
  - Duration: ~0.8s (local)
