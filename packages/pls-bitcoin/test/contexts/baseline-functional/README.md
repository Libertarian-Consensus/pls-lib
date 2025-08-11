# Baseline Functional Context (PR1)

Objective: Establish a precise functional baseline for `pls-bitcoin` (TypeScript) to support a future port (Rust/WASM) without introducing WASM in this PR.

Scope
- Deterministic tests (no network, no external services)
- Focus on current behavior: schemas, combinatorics, multisig generation
- Expose current limitations/bugs explicitly via tests (no silent suppression)

Out of Scope
- Any WASM-related code or directories (wasm/, pkg_wasm/)
- PSBT finalize/extract/sign end-to-end flows
- Performance benchmarks

Rules
- Start from `origin/main`, no local drift
- Keep tests isolated, data from local fixtures only
- If current behavior rejects/throws, assert it explicitly (document the behavior)

Structure
- fixtures/: deterministic data used by tests
- unit/: unit tests for focused functions
- integration/: lightweight integration checks (e.g., module exports)
