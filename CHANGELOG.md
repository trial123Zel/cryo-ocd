# Changelog

All notable changes to cryo-ocd are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

cryo-ocd is a community-maintained fork of
[`paradigmxyz/cryo`](https://github.com/paradigmxyz/cryo). Changes made upstream
prior to the fork point are not duplicated here; this log records changes made
within cryo-ocd. Each entry credits the original author where the change
originates from an upstream contribution.

## [Unreleased]

### Added

- Project governance and contributor documentation: `NOTICE`,
  `ACKNOWLEDGEMENTS.md`, `CHANGELOG.md`, `ROADMAP.md`, and Architecture Decision
  Records under `docs/adr/`.
- Secret-handling convention: `.env.example` template and a CI secret-scanning
  step. No API keys, RPC URLs, or IP addresses are stored in the repository.
- Self-hosted CI job for `cryo_test` data validation against a live node.
- Refreshed issue and pull-request templates, including an implementation-task
  template.

### Fixed

- Corrected typos in collection error messages (`partitions.rs`, `sources.rs`),
  the documentation summary (`book/SUMMARY.md`), and a block-chunk test
  (`blocks.rs`). Adopted from upstream PRs paradigmxyz/cryo#220 (@sunxunle),
  #226 (@Hopium21), and #237 (@solanaXpeter). (#12)

[Unreleased]: https://github.com/trial123Zel/cryo-ocd/commits/main
