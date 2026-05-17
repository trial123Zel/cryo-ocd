# ADR-0004: Validation strategy — cryo_test + self-hosted runner

- **Status:** Accepted
- **Date:** 2026-05-15
- **Amended:** 2026-05-16 — the CI `cryo_test` job compares each pull request's
  `cryo` build against a freshly built `main` build (a live pull-request-vs-
  `main` diff), rather than against a frozen golden snapshot. `main` is itself
  gated by this job, so it stays known-good, and there is no frozen baseline to
  regenerate when cryo's output changes on purpose. The pre-Phase-1 `P1-0`
  capture is kept as a historical reference. See issue #70.

## Context

Most of cryo's open bugs are data-correctness issues: the code compiles and runs
but produces wrong values. Compilation and unit tests cannot catch these. The
repository already contains `cryo_test`, a harness that runs cryo and diffs the
resulting Parquet output column-by-column between two builds.

Real validation requires querying a real node. The project maintainer runs a
synced Reth full node on a private LAN. GitHub-hosted cloud runners cannot reach
it, and this is a public repository, so any CI that touches the node must not be
executable by untrusted contributors.

## Decision

- `cryo_test` is the data-correctness gate. Each code change is validated by
  diffing its Parquet output against a known-good baseline.
- A **self-hosted GitHub Actions runner** on the maintainer's network runs the
  `cryo_test` job; cloud runners continue to run build/test/clippy/fmt.
- **Lockdown** (secrets are covered by ADR-0005):
  - The repository requires approval for all outside-contributor workflow runs.
  - The self-hosted job runs only when the actor is the maintainer account and
    the pull-request head is a branch in this repository (not a fork).
  - The node RPC URL lives only in a local `.env` on the runner host; it is
    never stored on GitHub, not even as an encrypted secret.
- A golden baseline of `cryo_test` output is captured from `main` before the
  Phase 1 migration and used as the regression reference.

## Consequences

- The self-hosted runner and the maintainer's node are temporary,
  maintainer-specific infrastructure. When the project becomes broadly useful to
  the public, this infrastructure is expected to be removed, and downstream
  users will establish their own validation harness.
- Until then, full data validation depends on maintainer availability; cloud
  jobs still gate every pull request independently.
- Contributors without node access can still pass build/test/clippy/fmt; the
  data-validation result is supplied by the maintainer's runner.
