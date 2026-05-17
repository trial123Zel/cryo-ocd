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
- The startup schema summary now lists event-decoded columns
  (`event__<name>`, with their ABI types) when an `--event-signature`
  is supplied — previously only a dataset's static columns were shown.
  From the event-decoding checklist in paradigmxyz/cryo#56. (#80)
- `transaction_count` column on the `blocks` dataset — the number of
  transactions in the block. Opt-in via `--include-columns
  transaction_count` (or `--columns all`). Re-implemented from upstream
  PR paradigmxyz/cryo#223 (@phqb). (#83)
- `blob_gas_used` and `excess_blob_gas` columns on the `blocks` dataset
  — the EIP-4844 header fields, populated for post-Dencun blocks. Opt-in
  via `--include-columns`. Adopted from upstream PR paradigmxyz/cryo#181
  (@peyha). (#84)
- `raw` column on the `transactions` dataset — the EIP-2718-encoded
  ("raw") transaction, as a binary column. Opt-in via `--include-columns
  raw`. Re-implemented from upstream PR paradigmxyz/cryo#180 (@0xMelkor).
  (#85)
- `deploy_address` column on the `transactions` dataset — the contract
  address created by a contract-deployment transaction, taken from the
  transaction receipt. Opt-in via `--include-columns deploy_address`;
  requesting it causes receipts to be fetched. Re-implemented from
  upstream PRs paradigmxyz/cryo#215 (@sslivkoff) and #189
  (@LatentSpaceExplorer). (#86)
- Prebuilt `cryo` release binaries and a `Dockerfile`. A new `release`
  workflow builds the CLI for Linux (x86_64), Windows (x86_64), and
  macOS (arm64) on every `v*` tag and publishes the archives as a
  GitHub Release; a multi-stage `Dockerfile` builds a slim runtime
  image. Re-implemented from upstream PR paradigmxyz/cryo#40
  (@distributedstatemachine). (#57)

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
- The CI `cryo_test data diff` job now performs a real column-by-column
  comparison of each pull request's `cryo` output against `main`,
  replacing the earlier build-and-connectivity smoke check (completes
  ADR-0004's validation gate). (#74)

### Removed

- `lzo` parquet compression, which polars 0.53 no longer supports
  (uncompressed/snappy/lz4/gzip/brotli/zstd are unaffected).

### Fixed

- The "install from source" instructions in `README.md` cloned the
  upstream `paradigmxyz/cryo` repository instead of this fork
  (`trial123Zel/cryo-ocd`), for both the CLI and the Python package. (#87)
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
- The collection summary no longer panics with "attempt to divide by
  zero" when `--align` leaves no chunks to collect. The errored/skipped/
  collected percentages now resolve to 0% instead of dividing by a zero
  chunk count. Reported by @Cybourgeoisie in paradigmxyz/cryo#150 and
  @0xhanh in paradigmxyz/cryo#125. (#72)
- The `contracts` dataset's `init_code_hash` and `code_hash` columns
  were swapped: `init_code_hash` held the keccak256 of the deployed
  code and `code_hash` the keccak256 of the init code. Each column now
  hashes its correct input. Cross-referenced paradigmxyz/cryo#249. (#73)
- The `cryo` Python package now declares its runtime dependencies:
  `polars` (required — cryo returns polars DataFrames across the
  pyo3-polars FFI boundary) and a `pandas` extra
  (`pip install cryo[pandas]`, pulling `pandas` and `pyarrow`) for
  `output_format="pandas"`. The package previously declared no
  dependencies, so a fresh `pip install cryo` failed at runtime with
  `ModuleNotFoundError`. Reported by @Evan-Kim2028 in
  paradigmxyz/cryo#137. (#76)
- `erc20_transfers` and `erc721_transfers` no longer panic when given
  `--from-address` or `--to-address`. The address was built into a
  log-topic filter at the wrong width (`B256::from_slice` on a 20-byte
  address); it is now left-padded into the 32-byte topic word that an
  indexed address occupies. All five datasets in the upstream request
  (`transactions`, `traces`, `native_transfers`, `erc20_transfers`,
  `erc721_transfers`) now honour the address filters. Reported by
  @sslivkoff in paradigmxyz/cryo#97. (#78)

[Unreleased]: https://github.com/trial123Zel/cryo-ocd/commits/main
