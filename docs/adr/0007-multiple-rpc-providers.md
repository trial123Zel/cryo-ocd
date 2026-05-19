# ADR-0007: Multiple RPC providers & graceful rate-limiting

- **Status:** Proposed
- **Date:** 2026-05-18

## Context

cryo connects to exactly one RPC endpoint. Two long-standing upstream feature
requests ask for more:

- **`paradigmxyz/cryo#132`** — accept several RPC URLs and spread work across
  them. With a single third-party endpoint, even carefully tuned concurrency
  saturates the provider's compute-units-per-second budget; aggregate
  throughput is capped at one provider's limit.
- **`paradigmxyz/cryo#5`** — handle rate-limit responses (HTTP 429) gracefully
  and automatically, with good defaults, instead of making users hand-tune
  concurrency knobs.

cryo-ocd tracks the pair as **P4-10** (#52).

The single-endpoint assumption is wired through the codebase:

- `crates/cli/src/args.rs` — `--rpc` is a single `Option<String>`;
  `parse_rpc_url` resolves exactly one URL (MESC → `--rpc` → `ETH_RPC_URL`).
- `crates/cli/src/parse/source.rs` — `parse_source` builds one `RpcClient`,
  hence one `DynProvider`.
- `crates/freeze/src/types/sources.rs` — the `Source` struct holds
  `provider: DynProvider` and `rpc_url: String` (both singular), a single
  `governor` rate limiter, and a single concurrency semaphore; `permit_request`
  gates every call through that one limiter; and ~20 methods call
  `self.provider.<method>()` directly.

So the change is broad — it reshapes `Source`, the `--rpc` argument, and the
rate-limit model. Some machinery exists already: alloy's `RetryBackoffLayer`
retries 429s with exponential backoff. But nothing *adapts* — the rate knobs
are fixed numbers the user must guess, and load cannot move between endpoints.

**Why this is being deferred.** Beyond its size, P4-10's "right" behaviour is
unusually **subjective to the end user's setup**:

- a single local node (the maintainer's own case) needs none of this — there
  are no rate limits to be graceful about, and no second endpoint to rotate to;
- a user fanning out across several paid providers wants weighted routing by
  each plan's budget;
- a user on one free, rate-limited tier wants aggressive adaptive backoff above
  all else.

No single default is correct for all three, and there is no concrete user
demand in cryo-ocd to anchor the trade-offs. The feature is also not needed for
the maintainer's own use case. P4-10 is therefore **re-scoped as an epic and
sidelined**; this ADR records the analysis and a proposed design so they are
not lost.

## Decision

P4-10 (#52) is **re-scoped from a medium task to an epic** and sidelined. No
implementation work is scheduled. This ADR is **Proposed**, not Accepted: it
commits the project to a direction, not a schedule. When the epic is taken up —
driven by real user demand — the ADR is refined and moved to Accepted, and
broken into the sub-issues below.

The proposed design has two layers.

### Layer A — multiple RPC providers

1. **Specifying endpoints.** Make `--rpc` repeatable
   (`--rpc URL1 --rpc URL2`), accept a comma-separated `ETH_RPC_URL`, and
   resolve MESC endpoint *groups* where MESC is configured. One endpoint stays
   the common case and behaves exactly as today.
2. **Reshaping `Source`.** Replace the single `provider` / `rpc_url` with a
   `Vec` of provider entries, each pairing a `DynProvider` with its own rate
   limiter, concurrency semaphore, and health state. Keep `permit_request` as
   the single seam — it already wraps every call — and extend it to also
   *select* an endpoint and hand back its provider, so each of the ~20 call
   sites changes by one line rather than the request layer being rewritten.
3. **Selection strategy.** Default to **least-outstanding-requests**: it
   balances heterogeneous providers without the user declaring weights, and
   skips endpoints currently in backoff. A primary-with-failover mode is a
   simpler opt-in.

### Layer B — graceful rate-limiting

1. **Detect** throttling: HTTP 429, `Retry-After` headers, and JSON-RPC error
   bodies mentioning rate / capacity / compute units (alloy's
   `RateLimitRetryPolicy` already classifies some of these).
2. **Adapt per endpoint** with an AIMD controller (additive-increase /
   multiplicative-decrease, as in TCP congestion control): on a throttle
   response, multiplicatively cut that endpoint's allowed rate; on sustained
   success, ease it back up toward a ceiling. This removes the need to guess
   `--requests-per-second`. Honour `Retry-After` when the provider sends it.
3. **Route around** throttled endpoints via the Layer-A selection strategy. The
   existing fixed knobs (`--requests-per-second`, `--compute-units-per-second`)
   become optional ceilings rather than required tuning.

### Sub-issue breakdown

1. Multi-value `--rpc` parsing and MESC endpoint groups.
2. Reshape `Source` to N providers; the endpoint-selection seam in
   `permit_request`; round-robin baseline.
3. Per-provider rate limiter and concurrency semaphore.
4. Adaptive AIMD backoff: 429 / `Retry-After` detection and per-endpoint
   throttling.
5. Health-aware selection and failover.
6. Documentation and multi-provider output metadata (`SourceLabels`).

### Alternatives considered

- **Do nothing** — keep relying on alloy's `RetryBackoffLayer`. Survives mild
  throttling; cannot aggregate throughput across providers. This is the status
  quo and the fallback if the epic is never scheduled.
- **MESC-only** — push all multi-endpoint configuration into MESC and add
  nothing to cryo's own surface. Lighter, but MESC adoption is low and it
  provides no adaptive behaviour.
- **Better defaults only** (a minimal reading of `paradigmxyz/cryo#5`) —
  retune the existing knobs' defaults without multi-provider support. Cheap,
  but only partially addresses #5 and not #132 at all.

## Consequences

- Until the epic is scheduled, cryo-ocd remains **single-endpoint**: one
  `--rpc`, with alloy's retry/backoff layer as the only rate-limit cushion.
  Users hitting third-party limits must still hand-tune `--requests-per-second`
  and `--max-concurrent-requests`.
- The reshape of `Source` (the `provider` field, `permit_request`, ~20 call
  sites) is invasive but mechanical; the adaptive-backoff controller is the
  genuinely design-heavy and subjective part. The sub-issue split lets the
  mechanical multi-provider work land independently of the heuristics.
- A new dependency may be needed for the AIMD controller, or it can be built on
  the `governor` limiter already in the tree.
- Issue #52 and the `ROADMAP.md` Phase 4 entry are marked as an epic (★).
