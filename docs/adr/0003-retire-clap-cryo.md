# ADR-0003: Retire the clap_cryo fork

- **Status:** Accepted
- **Date:** 2026-05-15

## Context

The CLI crate depends on `clap_cryo` (version `4.3.21-cryo`), a fork of the
`clap` argument parser published as `clap_cryo` / `clap_builder_cryo` /
`clap_derive_cryo`. The fork exists only to customize `--help` rendering. cryo's
actual usage is minimal: the `Parser` derive macro and a `builder::Styles`
value.

`clap_cryo` is itself unmaintained, pinned to a mid-2023 `clap` release. Keeping
it means no upstream fixes and a growing risk of version conflicts as the rest
of the dependency tree advances. Modern mainline `clap` 4.x supports
`builder::Styles` natively and offers `help_template` for help-text control.

## Decision

Migrate the CLI crate from `clap_cryo` to current mainline `clap`. Re-create the
help styling with `clap::builder::Styles` and `help_template`. Minor cosmetic
differences in `--help` output are acceptable; argument-parsing behavior must be
unchanged.

## Consequences

- One fewer unmaintained dependency; the CLI tracks mainline `clap`.
- The bespoke help layout may differ slightly; this is a deliberate, accepted
  trade-off recorded here.
- This work (roadmap task P1-8) is independent of the `alloy`/`polars` migration
  and can land on its own branch.
