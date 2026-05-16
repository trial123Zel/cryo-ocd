# ADR-0002: Modernize dependencies; target alloy 2.x

- **Status:** Accepted
- **Date:** 2026-05-15
- **Amended:** 2026-05-16 — alloy released 2.0 before Phase 1 began, so the
  migration targeted alloy 2.x; Phase 1 landed as a structured commit
  sequence on `phase-1-deps`, reviewed via a single PR (#63).

## Context

cryo-ocd inherited a dependency set that is several major versions behind:

- `alloy` 0.6.4 — the Ethereum library; `alloy` is now on the 2.x line.
- `polars` 0.38.3 — the dataframe engine.
- `pyo3` 0.20 with `pyo3-polars` / `pyo3-asyncio` — the Python binding stack.
- `syn` 1.x — used by the `cryo_to_df` procedural-macro crate.

Old dependencies block bug fixes (upstream issue #239 links a node-compatibility
bug to the old `alloy`), prevent adoption of newer features, and will eventually
break the build as registries yank old crates. Two upstream pull requests
already attempt this work: #240 (`alloy` 1.0.9) and #244 (`alloy` 1.0.23 +
`polars` 0.50 + `syn` 2.0).

## Decision

Modernize the dependency stack as the keystone Phase 1 of the roadmap:

- Target the current `alloy` 2.x line; upgrade `polars`, the `pyo3` stack, and
  `syn` to current versions.
- Use upstream PR #244 as the primary reference and PR #240 as a cross-check;
  re-implement rather than merge wholesale, validating against a live node.
- A dependency migration cannot compile in intermediate states, so the work
  lands on a `phase-1-deps` integration branch as a structured sequence of
  commits, reviewed via a single `phase-1-deps` -> `main` pull request that
  merges only when the whole branch is green.
- Retiring the `clap_cryo` fork is related but independent; it is covered by
  ADR-0003 and can proceed in parallel.

## Consequences

- Phase 1 must complete before most other phases, since many conflicting
  feature PRs and correctness fixes rebase onto the modernized tree.
- Some upstream-reported bugs are expected to be resolved incidentally by the
  upgrade and should be re-verified before separate work is done.
- The public API of the `cryo_freeze` crate and the Python bindings may change;
  such changes are recorded in `CHANGELOG.md`.
