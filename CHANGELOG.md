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
- `rust-toolchain.toml` pinning the Rust toolchain (1.95.0) so the `stable`
  channel advancing cannot silently break CI. (#15)

### Changed

- Dependency modernization (Phase 1): migrated the workspace off
  long-unmaintained versions, with no change to cryo's data output
  (verified column-by-column against a pre-migration baseline):
  `alloy` 0.6.4 → 2.0, `polars` 0.38.3 → 0.53, `pyo3` 0.20 → 0.27 and
  `pyo3-polars` 0.12 → 0.26, `syn` 1 → 2 in `cryo_to_df`, `clap_cryo`
  (an unmaintained clap fork) → mainline `clap` (see ADR-0003), and
  `pyo3-asyncio` (abandoned) → its successor `pyo3-async-runtimes`.
- GitHub Actions workflows pinned to full commit SHAs and bumped to
  current versions, for supply-chain safety. Adopts the intent of
  upstream PRs paradigmxyz/cryo#241 (@PixelPil0t1) and #242
  (@Daulox92). (#59)
- Renamed the `erc721_transfers` contract-address column from `erc20`
  to `erc721`; the name had been copied verbatim from the
  `erc20_transfers` dataset. Reported by @ChadRosseau in
  paradigmxyz/cryo#230. (#65)

### Removed

- `lzo` parquet compression, which polars 0.53 no longer supports
  (uncompressed/snappy/lz4/gzip/brotli/zstd are unaffected).

### Fixed

- Corrected typos in collection error messages (`partitions.rs`, `sources.rs`),
  the documentation summary (`book/SUMMARY.md`), and a block-chunk test
  (`blocks.rs`). Adopted from upstream PRs paradigmxyz/cryo#220 (@sunxunle),
  #226 (@Hopium21), and #237 (@solanaXpeter). (#12)
- Fixed the `clippy::io_other_error` lint failure on Rust 1.95 by using
  `std::io::Error::other` in `crates/freeze/build.rs`. Adopted from upstream PR
  paradigmxyz/cryo#234 (@strmfos). (#14)
- Resolved all remaining `clippy` lints across the workspace under Rust 1.95
  (`needless_return`, `useless_vec`, `len_zero`, `unnecessary_unwrap`,
  `double_ended_iterator_last`). (#15)
- cryo's dataframe sort is now stable, so output row order is deterministic
  and reproducible run-to-run; polars 0.53's default sort is unstable.
- `erc20_transfers` collected by transaction (`--txs`) returned ERC-20
  Approval logs instead of Transfer logs: the by-transaction filter
  matched the wrong event signature hash. Reported by @ChadRosseau in
  paradigmxyz/cryo#231. (#64)
- `geth_state_diffs` reported a `to_value` of zero for any balance,
  nonce, or code that a transaction touched but left unchanged: geth's
  diff-mode tracer omits unchanged fields from its `post` state. Such
  fields now correctly carry their pre-transaction value, while
  genuinely self-destructed accounts still resolve to zero. Reported by
  @phqb in paradigmxyz/cryo#245. (#66)
- Querying `logs` with an `--event-signature` containing a `bytes32`
  (FixedBytes) parameter no longer fails with "could not generate
  FixedBytes column" when the query matches no rows; an empty result
  now produces an empty dataset. Reported by @mempirate in
  paradigmxyz/cryo#238. (#67)
- Excluding a column that is part of a dataset's `default_sort` (for
  example `create_index` on `contracts`) no longer zeroes the output.
  Sorting now skips any sort column not present in the result. Reported
  by @dreaded369 in paradigmxyz/cryo#221. (#68)
- `--reorg-buffer` no longer drops the partial tail chunk when `latest`
  is the range end. The chunk straddling the reorg-safe ceiling is now
  trimmed to it instead of being discarded, so the max collected block
  is no longer rounded down to the last full chunk boundary. Reported
  by @BowTiedDevil in paradigmxyz/cryo#193. (#69)
- `traces` and `trace_calls` no longer truncate gas values above
  `u32::MAX`. The `action_gas` and `result_gas_used` columns are now
  `u64`; on chains such as BSC where trace gas exceeds `u32::MAX`, the
  previous `as u32` cast silently truncated the value. Reported by
  @shouc in paradigmxyz/cryo#173. (#71)

[Unreleased]: https://github.com/trial123Zel/cryo-ocd/commits/main
