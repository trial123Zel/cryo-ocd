# Contributing to cryo-ocd

cryo-ocd is a community-maintained fork of
[`paradigmxyz/cryo`](https://github.com/paradigmxyz/cryo). Its goal is to
methodically work through the upstream issue and pull-request backlog and
modernize the codebase, so the tool stays useful to analysts.

Contributions are welcome. This guide explains how the project is run.

## How work is organized

- **[`ROADMAP.md`](./ROADMAP.md)** — the phased plan. Every task has an ID
  (`P1-3`, `H-2`, ...).
- **GitHub Issues** — one issue per task, grouped under phase **milestones**.
  Each issue states a Problem, Affected files, Acceptance criteria, a Validation
  method, and References & credit.
- **[`docs/adr/`](./docs/adr/)** — Architecture Decision Records capture *why*
  significant decisions were made.

Comment on an existing issue, or open one, before starting non-trivial work so
effort is not duplicated.

## Relationship to the upstream tracker

cryo-ocd does not mirror, close, or comment on the `paradigmxyz/cryo` issue
tracker. Worthwhile upstream issues and pull requests are *selectively imported*
here as new issues, with credit. Items judged out of scope are simply not
imported — no action is taken on the upstream tracker. See ADR-0001.

## Branching & commits

- Branch per issue: `fix/<issue>-short-desc`, `feat/...`, `chore/...`,
  `docs/...`.
- Use [Conventional Commits](https://www.conventionalcommits.org/): `fix:`,
  `feat:`, `chore:`, `docs:`, `perf:`, `refactor:`, `test:`.
- Keep commits logically grouped; squash noisy checkpoint commits.
- Reference the issue in the commit or PR (e.g. `Closes #42`).
- Keep pull requests small and single-purpose.

## Crediting adopted work

When a change originates from an upstream cryo contributor:

- Prefer cherry-picking the original commit so git preserves the `Author:`
  field.
- If re-implementing, add a `Co-Authored-By: Name <email>` trailer for the
  original author.
- Credit the original reporter/author in the PR description and link the
  upstream issue or PR number.

Preserving authorship of community contributions is a hard requirement, not a
courtesy.

## Secrets & private infrastructure

- **Never commit** API keys, RPC URLs, WAN/LAN IP addresses, or any other PII.
- All environment-specific values belong in a local `.env`, which is
  git-ignored. Copy [`.env.example`](./.env.example) and fill it in.
- CI runs a secret scanner; commits containing secrets are rejected.

## Code conventions

- `cargo fmt --all` must pass (nightly rustfmt; see `rustfmt.toml`).
- `cargo clippy --workspace --all-targets --all-features` must pass with
  `-D warnings`.
- Avoid panics (`panic!`, `todo!`, `unwrap()`, `expect()`) outside tests, build
  scripts, `lazy_static` blocks, and procedural macros.
- Add tests for bug fixes and features. Tests that use forking must contain
  `fork` in their name.

## The CI gate

A pull request is mergeable when CI is green:

1. `cargo build` + `cargo test` (workspace)
2. `cargo clippy` with `-D warnings`
3. `cargo fmt --check`
4. Secret scan
5. `cryo_test` data validation (self-hosted runner; see ADR-0004)

## Licensing

By contributing, you agree that your contributions are dual-licensed under the
MIT and Apache-2.0 licenses, consistent with the original project. The original
license files and copyright notices are retained unchanged.

## Code of Conduct

This project follows the
[Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
Report unacceptable behavior confidentially to the repository maintainer
through GitHub.
