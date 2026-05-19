# cryo-ocd Roadmap

cryo-ocd revives and modernizes
[`paradigmxyz/cryo`](https://github.com/paradigmxyz/cryo) by methodically working
through the upstream issue and pull-request backlog. This document is the plan
of record.

## How to read this

- Work is grouped into **phases**, each tracked as a GitHub **milestone**.
- Every task has an ID — `P<phase>-<n>` for phase work, `H-<n>` for
  housekeeping — and one tracking issue in this repo.
- Each phase below is a single table with the same columns. Full scope,
  acceptance criteria, and validation method live in each task's linked issue.
- Only worthwhile community work is imported; the upstream tracker is not
  mirrored or adjudicated — see [ADR-0001](./docs/adr/0001-fork-charter.md).

### Legend

**Status** — ✅ Done · 🔄 In progress · ⬜ Not started · ⏸️ Deferred (blocked on infrastructure, awaiting a future phase)

- **Issue** — the cryo-ocd issue tracking the task. GitHub shows it green when
  open, purple when closed.
- **Credit** — upstream
  [`paradigmxyz/cryo`](https://github.com/paradigmxyz/cryo) issues and PRs,
  cited as `cryo#NN`, with the original authors credited by handle per
  [ADR-0001](./docs/adr/0001-fork-charter.md).
- **★** — an **epic**: gets its own ADR and sub-issue breakdown when its
  milestone opens.

The merge gate for every code task: `cargo build`, `cargo test`,
`clippy -D warnings`, `cargo fmt`, a secret scan, and the `LICENSE-*` guard all
green; the `cryo_test` data diff runs as an advisory check.

## Phase overview

| Phase | Theme | Status | Tasks done |
|-------|-------|--------|------------|
| 0 — Foundation & Governance | Scaffolding, governance, CI, secret handling | ✅ Done | 10 / 10 |
| 1 — Dependency Modernization | alloy 2.x, polars, pyo3, syn 2; retire clap_cryo | ✅ Done | 11 / 11 |
| 2 — Quick Bug Fixes | Small, well-understood fixes with reference patches | ✅ Done | 9 / 9 |
| 3 — Correctness Bugs | Data-correctness bugs needing node reproduction | ✅ Done | 6 / 6 |
| 4 — Features & Enhancements | In-scope feature work from the community backlog | 🔄 In progress | 12 / 19 |
| Housekeeping | Low-risk maintenance, batched across phases | ✅ Done | 6 / 6 |
| Deferred | Items blocked on infrastructure, awaiting a future phase | ⏸️ Deferred | 0 / 2 |

**Status snapshot (2026-05-19).** Phases 0–2 are complete with their milestones
closed, and Phase 3's correctness fixes are all merged. Phase 4 is in progress:
**`v0.5.0` has shipped** — cryo-ocd's second release, after the `v0.4.0` debut —
with prebuilt CLI binaries (x86-64 Linux and Windows, ARM64 macOS) published by
the `release` workflow. That workflow also builds and smoke-tests the Docker
image, but does not publish it to a registry. Two tasks — `P3-2` and `P4-12` —
are **deferred**: both need an Erigon archive node to verify, and are parked for
a future phase (see [Deferred](#deferred)). 54 of 63 tracked tasks are complete
overall.

---

## Phase 0 — Foundation & Governance

Scaffolding, governance, CI, and secret handling. Branch protection and
merge-on-green switch on at the end of the phase, so `P0-9` is the first PR
through the full pipeline.

| Task | Status | Summary | Issue | Credit |
|------|--------|---------|-------|--------|
| P0-1 | ✅ Done | Governance & licensing docs — `NOTICE`, README fork banner, `ACKNOWLEDGEMENTS.md`, `CONTRIBUTING.md`, `CHANGELOG.md`; `LICENSE-*` left byte-for-byte unchanged | #1 | — |
| P0-2 | ✅ Done | Architecture Decision Records — `docs/adr/` with a template and ADR-0001…0005 | #2 | — |
| P0-3 | ✅ Done | Issue & PR templates — implementation-task form, refreshed bug/feature templates | #3 | — |
| P0-4 | ✅ Done | Label namespaces and Phase 0–4 milestones; GitHub Issues enabled | #4 | — |
| P0-5 | ✅ Done | Secret handling & `.env` convention — `.env.example`, `.gitignore`, CI secret-scanning | #5 | — |
| P0-6 | ✅ Done | CI overhaul & runner lockdown — build/test, clippy, fmt, secret scan, `LICENSE-*` guard, gated `cryo_test` job | #6 | — |
| P0-7 | ✅ Done | This roadmap document | #7 | — |
| P0-8 | ✅ Done | `cryo_test` portability fix — drop the hard-coded path/commit, read `RETH_RPC_URL` from `.env` | #8 | — |
| P0-9 | ✅ Done | Branch protection + warm-up PR — enable merge-on-green, shaken down by the `H-3` typo batch | #9 | — |
| P0-10 | ✅ Done | Import the triaged backlog as one GitHub issue per roadmap task | #10 | — |

---

## Phase 1 — Dependency Modernization

The keystone. A dependency migration cannot compile in intermediate states, so
`P1-1`–`P1-7` landed as one ordered commit sequence on a `phase-1-deps`
integration branch behind a single review PR (`P1-10`); `P1-8` is independent.
Primary reference: upstream PR `cryo#244` (@clouds56); cross-check `cryo#240`
(@mattsse). See [ADR-0002](./docs/adr/0002-dependency-modernization.md).

| Task | Status | Summary | Issue | Credit |
|------|--------|---------|-------|--------|
| P1-0 | ✅ Done | Golden baseline capture — archive `cryo_test` output from `main` across all datatypes as the regression baseline | #16 | — |
| P1-1 | ✅ Done | alloy 2.x — provider/source layer (`types/sources.rs`, `types/rpc_params.rs`) | #17 | — |
| P1-2 | ✅ Done | alloy 2.x — primitive & RPC-type fixes across the ~40 dataset files | #18 | — |
| P1-3 | ✅ Done | alloy 2.x — trace-type datasets (`traces`, `trace_calls`, `contracts`, `*_diffs`, `vm_traces`, `geth_*`) | #19 | — |
| P1-4 | ✅ Done | alloy 2.x — `to_df` macro & python crate | #20 | — |
| P1-5 | ✅ Done | polars 0.38 → current — schema/series API migration | #21 | — |
| P1-6 | ✅ Done | syn 1 → 2 in the `to_df` procedural-macro crate | #22 | — |
| P1-7 | ✅ Done | pyo3 / pyo3-polars / pyo3-asyncio upgrade | #23 | — |
| P1-8 | ✅ Done | Retire the clap_cryo fork for mainline clap — see [ADR-0003](./docs/adr/0003-retire-clap-cryo.md) | #24 | — |
| P1-9 | ✅ Done | Cargo.lock hygiene — dedup crates, resolve yanked deps | #25 | `cryo#225`, `cryo#217` |
| P1-10 | ✅ Done | Full validation & version bump — all-datatype `cryo_test` diff vs the `P1-0` baseline | #26 | `cryo#239` |

---

## Phase 2 — Quick Bug Fixes

Small, well-understood fixes — each one PR with a regression test and a
`cryo_test` diff for the affected datatype. Several upstream contributors
submitted overlapping patches; cryo-ocd re-implements from the issue,
validates, and credits the original reporter.

| Task | Status | Summary | Issue | Credit |
|------|--------|---------|-------|--------|
| P2-1 | ✅ Done | `erc20_transfers` matched the Approval signature instead of Transfer | #27 | `cryo#231`, `cryo#233` (@ChadRosseau) |
| P2-2 | ✅ Done | `erc721_transfers` contract column misnamed `erc20` | #28 | `cryo#230`, `cryo#232` (@ChadRosseau) |
| P2-3 | ✅ Done | `geth_state_diffs` wrong `to_value` when post-state is absent | #29 | `cryo#245`, `cryo#251` |
| P2-4 | ✅ Done | "could not generate FixedBytes column" on an empty result | #30 | `cryo#238`, `cryo#254` |
| P2-5 | ✅ Done | Excluding a `default_sort` column zeroed the output | #31 | `cryo#221`, `cryo#253` |
| P2-6 | ✅ Done | Reorg-buffer dropped the partial tail chunk | #32 | `cryo#193`, `cryo#255` |
| P2-7 | ✅ Done | u32 overflow on large trace gas values | #33 | `cryo#173`, `cryo#256` |
| P2-8 | ✅ Done | Divide-by-zero in collection summaries with `--align` | #34 | `cryo#150`, `cryo#125`, `cryo#258` |
| P2-9 | ✅ Done | `contracts` `init_code_hash` / `code_hash` swapped | #35 | `cryo#249` |

---

## Phase 3 — Correctness Bugs

Data-correctness bugs that need live-node reproduction: reproduce, root-cause,
fix, validate.

| Task | Status | Summary | Issue | Credit |
|------|--------|---------|-------|--------|
| P3-1 | ✅ Done | `balances` returned pre-execution balances (silent corruption) | #36 | `cryo#154` |
| P3-3 | ✅ Done | reth `contracts` from block 0 always failed | #38 | `cryo#151` |
| P3-4 | ✅ Done | `--txs` empty-result / empty-chunk panic | #39 | `cryo#49`, `cryo#26` |
| P3-5 | ✅ Done | `polars-arrow` compile failure | #40 | `cryo#63` |
| P3-6 | ✅ Done | Poetry build fails when `GIT_DESCRIPTION` is unset | #41 | `cryo#61` |
| P3-7 | ✅ Done | pip install did not require `pyarrow` | #42 | `cryo#137` |

Phase 3's seventh task, `P3-2`, has been **deferred** — it can only be verified
on an Erigon archive node. See [Deferred](#deferred).

---

## Phase 4 — Features & Enhancements

In-scope enhancements from the community backlog. Epics (★) get their own ADR
and sub-issue breakdown when their milestone opens.

| Task | Status | Summary | Issue | Credit |
|------|--------|---------|-------|--------|
| P4-1 | ✅ Done | EIP-4844 block fields (`blob_gas_used`, `excess_blob_gas`) | #43 | `cryo#181` (@peyha) |
| P4-2 | ✅ Done | Transaction deployed-contract-address column | #44 | `cryo#215` (@sslivkoff), `cryo#189` (@LatentSpaceExplorer) |
| P4-3 | ✅ Done | Raw (EIP-2718-encoded) transaction column | #45 | `cryo#180` (@0xMelkor) |
| P4-4 | ✅ Done | `--to`/`--from-address` filtering for more datasets | #46 | `cryo#97` |
| P4-5 | ✅ Done | `transaction_count` column on the blocks dataset | #47 | `cryo#223` |
| P4-6 ★ | ⬜ Not started | OP Stack chain support (epic) | #48 | `cryo#155`; [ADR-0006](./docs/adr/0006-op-stack-support.md) |
| P4-7 | ✅ Done | WebSocket & IPC connection support | #49 | `cryo#65` |
| P4-8 ★ | ⬜ Not started | `--function-signature` filtering + calldata decoding for `txs` | #50 | `cryo#140`, `cryo#145` (@cool-mestorf), `cryo#149` (@DoTheBestToGetTheBest) |
| P4-9 | ✅ Done | Event decoding — u256 handling, empty-result datatypes, schema-summary display | #51 | `cryo#56`, `cryo#184` |
| P4-10 ★ | ⬜ Not started | Multiple RPC providers + rate-limiting (epic) | #52 | `cryo#132`, `cryo#5`; [ADR-0007](./docs/adr/0007-multiple-rpc-providers.md) |
| P4-11 | ✅ Done | Incremental dataset consolidation | #53 | `cryo#29` (@banteg) |
| P4-13 ★ | ⬜ Not started | Cloud/S3 sink via a generalized `Sink` trait | #55 | `cryo#47`, `cryo#92` (@sslivkoff) |
| P4-14 ★ | ⬜ Not started | Direct Reth DB access, bypassing JSON-RPC | #56 | `cryo#3`, `cryo#156`, `cryo#163` |
| P4-15 | ✅ Done | Release binaries via CI + Dockerfile | #57 | `cryo#229`, `cryo#40` (@distributedstatemachine) |
| P4-16 | ✅ Done | Python docs, API docstrings, install-instruction fixes | #58 | `cryo#205`, `cryo#178`, `cryo#186` (@peyha), `cryo#169` (@0xstubbs) |
| P4-17 | ⬜ Not started | Array & tuple support in log/event decoding (candidate epic) | #79 | `cryo#184` |
| P4-18 | ✅ Done | Progress bar for the Python CLI | #101 | `cryo#178` |
| P4-19 | ⬜ Not started | mdBook (`book/`) fork-staleness review | #100 | — |
| P4-20 | ✅ Done | JWT authentication for RPC connections | #107 | `cryo#65` |

`P4-6` was re-scoped from "OP Stack receipt fields" to an epic during
verification: cryo cannot deserialise OP Stack blocks at all — the per-block
deposit transaction (type `0x7e`) is absent from alloy's Ethereum types. Full
OP Stack support needs `op-alloy` integration; see
[ADR-0006](./docs/adr/0006-op-stack-support.md). Sidelined for now.

`P4-10` was likewise re-scoped to an epic: multi-provider routing and adaptive
rate-limiting are large, and — unlike most tasks — their ideal behaviour is
highly specific to each user's setup (one local node, several paid providers,
or a single throttled free tier all want different things). The design is
captured in [ADR-0007](./docs/adr/0007-multiple-rpc-providers.md); sidelined
until there is user demand to anchor the trade-offs.

`P4-17` was broken out of `P4-9` during scope review: array/tuple decoding is
design-heavy — nested type representation and the interaction with cryo's u256
multi-representation — and may warrant its own ADR if pursued as an epic.

`P4-18` and `P4-19` were split out of `P4-16` during the docs review — the
Python-CLI progress bar, and a fork-staleness pass over the rest of the mdBook
(`book/`) beyond the install and Python pages already refreshed there. `P4-16`'s
notebook-example sub-task was dropped as out of scope.

`P4-12` (`erigon_getHeaderByNumber` fast-path) has been **deferred** — it needs
an Erigon node to test. See [Deferred](#deferred).

`P4-20` was split out of the `P4-7` review: WebSocket/IPC transport and JWT
authentication are separate concerns, so JWT was filed as its own task.

---

## Deferred

Two tasks are deferred to a future, not-yet-scheduled phase. Both can only be
verified against an **Erigon archive node**, and the project's CI runner uses
Reth — so neither can be tested today. They will be scheduled once an Erigon
archive node is available.

| Task | Status | Summary | Issue | Credit |
|------|--------|---------|-------|--------|
| P3-2 | ⏸️ Deferred | `address_appearances` fails on Erigon-3 archive nodes | #37 | `cryo#224` |
| P4-12 | ⏸️ Deferred | `erigon_getHeaderByNumber` batch fast-path | #54 | `cryo#35` |

`P3-2` is very likely already fixed by the alloy 2.x migration (Phase 1) but
needs confirmation on a real Erigon-3 node. `P4-12` is an Erigon-specific
performance path that cannot be exercised — or benchmarked — without one. Both
keep their original phase IDs; only their scheduling has moved.

---

## Housekeeping

Low-risk maintenance, batched alongside the phases.

| Task | Status | Summary | Issue | Credit |
|------|--------|---------|-------|--------|
| H-1 | ✅ Done | Bump and full-SHA-pin GitHub Actions | #59 | `cryo#241` (@PixelPil0t1), `cryo#242` (@Daulox92) |
| H-2 | ✅ Done | Modernize `std::io::Error` construction in `build.rs` | #14 | `cryo#234` (@strmfos) |
| H-3 | ✅ Done | Typo / docs batch — the Phase 0 warm-up PR | #12 | `cryo#220` (@sunxunle), `cryo#226` (@Hopium21), `cryo#235` (@0xAlexKorn), `cryo#237` (@solanaXpeter) |
| H-4 | ✅ Done | Full `clippy` pass and Rust toolchain pin | #15 | `cryo#246` (@bh2smith) |
| H-5 | ✅ Done | Repo-maintenance refresh ahead of the release candidate | #87 | — |
| H-6 | ✅ Done | Bump GitHub Actions off the retiring Node 20 runtime | #93 | — |

---

Task IDs map to GitHub issues in this repo (the **Issue** column). Upstream
references (`cryo#NN`) point to
[`paradigmxyz/cryo`](https://github.com/paradigmxyz/cryo); adopted work
preserves original authorship per [ADR-0001](./docs/adr/0001-fork-charter.md).
