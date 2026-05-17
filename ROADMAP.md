# cryo-ocd Roadmap

cryo-ocd revives and modernizes
[`paradigmxyz/cryo`](https://github.com/paradigmxyz/cryo) by methodically working
through the upstream issue and pull-request backlog. This document is the plan
of record.

## How to read this

- Work is grouped into **phases**, tracked as GitHub **milestones**.
- Each **task** has an ID (`P1-3`, `H-2`) and becomes one GitHub **issue**.
- Each task notes its **size** (S/M/L) and **dependencies**, then describes the
  scope, **acceptance criteria**, a **validation** method, and **credit** for
  any adopted upstream work.
- Only worthwhile community work is imported; the upstream tracker is not
  mirrored or adjudicated — see [ADR-0001](./docs/adr/0001-fork-charter.md).

The merge gate for every code task: `cargo build` + `cargo test`,
`clippy -Dwarnings`, `cargo fmt`, and the `cryo_test` data diff — all green.

## Phase overview

| Phase | Milestone | Theme | Status |
|-------|-----------|-------|--------|
| 0 | Foundation & Governance | Scaffolding, governance, CI, secret handling | ✅ Complete |
| 1 | Dependency Modernization | alloy 2.x, polars, pyo3, syn 2; retire clap_cryo | ✅ Complete |
| 2 | Quick Bug Fixes | Small, well-understood fixes with reference patches | ✅ Complete |
| 3 | Correctness Bugs | Data-correctness bugs needing node reproduction | ✅ Complete |
| 4 | Features & Enhancements | In-scope feature work from the community backlog | 🔄 In progress |

Housekeeping tasks (`H-*`) are low-risk and batched alongside other phases.

**Status snapshot (2026-05-17).** Phases 0–3 are complete: the dependency
migration, the quick-fix batch, and the correctness-bug batch have all landed.
Two minor follow-ups remain open against their milestones — `#60` (migrate
`python_release.yml` to `upload-artifact` / `download-artifact` v4) and `#37` /
`P3-2` (`address_appearances` on Erigon-3 archive nodes, pending
re-verification). Phase 4 is in progress: `P4-1`–`P4-5` and `P4-9` are merged;
`P4-15` (release binaries + Dockerfile) is next. Per-task status is shown in the
Phase 4 table below — ✅ done, 🔄 in progress, blank = not started.

---

## Phase 0 — Foundation & Governance

Delivered as a scaffolding pull request plus repo-configuration actions. Branch
protection and the merge-on-green policy switch on at the end of the phase, so
`P0-9` is the first PR through the full pipeline.

**P0-1 — Governance & licensing docs.** *S.* `NOTICE`, README fork banner,
`ACKNOWLEDGEMENTS.md`, refreshed `CONTRIBUTING.md`, `CHANGELOG.md`. `LICENSE-*`
left byte-for-byte unchanged.

**P0-2 — Architecture Decision Records.** *S.* `docs/adr/` with a template and
ADR-0001…0005.

**P0-3 — Issue & PR templates.** *S.* Implementation-task issue form, refreshed
bug/feature templates, updated PR template.

**P0-4 — Labels & milestones.** *S.* Label namespaces and Phase 0–4 milestones;
GitHub Issues enabled. *(Completed during Phase 0 setup.)*

**P0-5 — Secret handling & .env convention.** *S.* `.env.example`, `.gitignore`
confirmation, CI secret-scanning.

**P0-6 — CI overhaul & runner lockdown.** *M.* Keep build/test, clippy, fmt; add
the self-hosted `cryo_test` job with actor + same-repo-branch gating; add a
secret scan and a `LICENSE-*` change guard.

**P0-7 — ROADMAP.md.** *S.* This document.

**P0-8 — cryo_test portability fix.** *S.* Remove the hard-coded path and build
commit from the `cryo_test` example scripts; read `RETH_RPC_URL` from `.env`.

**P0-9 — Branch protection + warm-up PR.** *S, depends P0-1…P0-8.* Enable branch
protection and merge-on-green; run the `H-3` typo batch as the pipeline
shakedown.

**P0-10 — Import triaged backlog as issues.** *M, depends P0-3.* Create one
GitHub issue per roadmap task below.

---

## Phase 1 — Dependency Modernization

The keystone. A dependency migration cannot compile in intermediate states, so
`P1-1`…`P1-7` land as a structured sequence of commits on a `phase-1-deps`
integration branch; the single `phase-1-deps` → `main` pull request (`P1-10`)
is the review point and the only one gated green by CI and the `cryo_test`
baseline diff. `P1-8` is independent. Primary reference: upstream PR #244
(`@clouds56`); cross-check: PR #240 (`@mattsse`). See
[ADR-0002](./docs/adr/0002-dependency-modernization.md).

**P1-0 — Golden baseline capture.** *S.* Archive `cryo_test` output from current
`main` across all datatypes as the regression baseline.

**P1-1 — alloy: provider/source layer.** *M, depends P1-0.* Migrate
`types/sources.rs`, `types/rpc_params.rs` to alloy 2.x.

**P1-2 — alloy: primitive & RPC-type fixes.** *M, depends P1-1.* The ~40 dataset
files that only use primitive/RPC types.

**P1-3 — alloy: trace-type datasets.** *L, depends P1-1.* `traces`,
`trace_calls`, `contracts`, `storage_diffs`, `code_diffs`, `balance_diffs`,
`vm_traces`, `geth_*`.

**P1-4 — alloy: to_df macro & python crate.** *M, depends P1-2/P1-3.*

**P1-5 — polars 0.38 → current.** *L.* Schema/series API; `with_series!` →
`with_column` rename.

**P1-6 — syn 1 → 2 in to_df.** *M.* The procedural-macro crate.

**P1-7 — pyo3 / pyo3-polars / pyo3-asyncio upgrade.** *M, depends P1-4.*

**P1-8 — Retire clap_cryo → mainline clap.** *M, independent.* See
[ADR-0003](./docs/adr/0003-retire-clap-cryo.md).

**P1-9 — Cargo.lock hygiene.** *S.* Dedup crates, resolve yanked deps.
Resurrects upstream PRs #225 and #217.

**P1-10 — Full validation & version bump.** *M, depends all.* `cryo_test`
all-datatype diff vs the `P1-0` baseline; update `CHANGELOG.md`; bump version;
merge `phase-1-deps` → `main`. Closes upstream issue #239.

---

## Phase 2 — Quick Bug Fixes

Each task is one small PR with a regression test and a `cryo_test` diff for the
affected datatype. Mostly parallelizable. Several upstream contributors
submitted overlapping patches; cryo-ocd re-implements from the issue, validates,
and credits the original reporter and any genuinely-used diff.

| Task | Fix | Files | Upstream / credit |
|------|-----|-------|-------------------|
| P2-1 | erc20_transfers uses the Approval signature instead of Transfer | `datasets/erc20_transfers.rs` | issue #231, PR #233 (@ChadRosseau) |
| P2-2 | erc721_transfers contract column misnamed `erc20` | `datasets/erc721_transfers.rs` | issue #230, PR #232 (@ChadRosseau) |
| P2-3 | geth_state_diffs wrong `to_value` when post-state absent | `multi_datasets/geth_state_diffs.rs` | issue #245, PR #251 |
| P2-4 | "could not generate FixedBytes column" on empty result | `to_df/src/lib.rs` | issue #238, PR #254 |
| P2-5 | excluding a `default_sort` column zeroes output | `types/dataframes/sort.rs` | issue #221, PR #253 |
| P2-6 | reorg-buffer drops the partial tail chunk | `cli/src/parse/blocks.rs` | issue #193, PR #255 |
| P2-7 | u32 overflow on large trace gas values | `datasets/traces.rs`, `trace_calls.rs` | issue #173, PR #256 |
| P2-8 | divide-by-zero in summaries with `--align` | `types/summaries.rs` | issues #150, #125, PR #258 |
| P2-9 | contracts `init_code_hash` / `code_hash` swapped | `datasets/contracts.rs` | PR #249 |

---

## Phase 3 — Correctness Bugs

Each task: reproduce against a live node, root-cause, fix, validate.
Parallelizable. `P3-2` and `P3-5` may be resolved incidentally by Phase 1 —
re-verify before doing separate work.

| Task | Bug | Upstream |
|------|-----|----------|
| P3-1 | `balances` returns pre-execution balances (silent corruption) | issue #154 |
| P3-2 | `address_appearances` fails on Erigon-3 archive nodes | issue #224 |
| P3-3 | reth `contracts` from block 0 always fails | issue #151 |
| P3-4 | `--txs` empty-result / empty-chunk panic | issues #49, #26 |
| P3-5 | `polars-arrow` compile failure | issue #63 |
| P3-6 | Poetry build fails (`GIT_DESCRIPTION` unset) | issue #61 |
| P3-7 | pip install does not require `pyarrow` | issue #137 |

---

## Phase 4 — Features & Enhancements

In-scope enhancements from the community backlog. The three epics (★) get their
own ADR and sub-issue breakdown when their milestone opens.

| Task | Feature | Upstream / credit | Status |
|------|---------|-------------------|--------|
| P4-1 | EIP-4844 block fields (`blob_gas_used`, `excess_blob_gas`) | PR #181 (@peyha) | ✅ Done |
| P4-2 | Transaction deploy/contract address column | PR #215 (@sslivkoff), #189 (@LatentSpaceExplorer) | ✅ Done |
| P4-3 | Raw transaction column | PR #180 (@0xMelkor) | ✅ Done |
| P4-4 | `--to/--from-address` filtering for more datasets | issue #97 | ✅ Done |
| P4-5 | `transaction_count` column on blocks | upstream PR #223 | ✅ Done |
| P4-6 | OP Stack receipt fields | issue #155 | |
| P4-7 | WebSocket support | issue #65 | |
| P4-8 ★ | `--function-signature` filtering + calldata decoding for `txs` | issue #140, PRs #145 (@cool-mestorf), #149 (@DoTheBestToGetTheBest) | |
| P4-9 | Event decoding: u256 handling, empty-result datatypes, schema-summary display | issues #56, #184 | ✅ Done |
| P4-10 | Multiple RPC providers + graceful rate-limiting | issues #132, #5 | |
| P4-11 | Incremental dataset consolidation | issue #29 | |
| P4-12 | `erigon_getHeaderByNumber` batch perf | issue #35 | |
| P4-13 ★ | Cloud/S3 sink via a generalized `Sink` trait | issue #47, PR #92 (@sslivkoff) | |
| P4-14 ★ | Direct Reth DB access, bypassing JSON-RPC | issues #3, #156, upstream PR #163 | |
| P4-15 | Release binaries via CI + Dockerfile | issue #229, PR #40 (@distributedstatemachine) | 🔄 Next |
| P4-16 | Python docs, docstrings, notebook example, progress bar | issues #205, #178, PRs #186 (@peyha), #169 (@0xstubbs) | |
| P4-17 | Array & tuple support in log/event decoding (candidate epic) | issue #79, upstream #184 | |

`P4-17` was broken out of `P4-9` during scope review: array/tuple decoding is
design-heavy — nested type representation and the interaction with cryo's u256
multi-representation — and may warrant its own ADR and sub-issue breakdown if
pursued as an epic. See [#79](https://github.com/trial123Zel/cryo-ocd/issues/79).

---

## Housekeeping

Low-risk, batched.

| Task | Work | Upstream / credit |
|------|------|-------------------|
| H-1 | CI action version bumps | PRs #241 (@PixelPil0t1), #242 (@Daulox92) |
| H-2 | Modernize `std::io::Error` construction in `build.rs` | PR #234 (@strmfos) |
| H-3 | Typo / docs batch — the Phase 0 warm-up PR | PRs #220 (@sunxunle), #226 (@Hopium21), #235 (@0xAlexKorn), #237 (@solanaXpeter) |
| H-4 | Full `clippy` pass (after Phase 1) | PR #246 (@bh2smith) |
| H-5 | Repo-maintenance refresh ahead of the release candidate | — |

---

*Upstream issue and PR numbers refer to
[`paradigmxyz/cryo`](https://github.com/paradigmxyz/cryo). Adopted work preserves
original authorship per [ADR-0001](./docs/adr/0001-fork-charter.md).*
