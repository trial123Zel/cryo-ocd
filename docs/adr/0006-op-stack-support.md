# ADR-0006: OP Stack chain support

- **Status:** Proposed
- **Date:** 2026-05-18

## Context

cryo's documentation describes it as a tool for "any EVM chain." In practice,
cryo-ocd cannot extract data from OP Stack chains (Optimism, Base, and the
wider Superchain) at all.

This was found while verifying P4-6 (#48), originally scoped as a medium task —
"add the OP Stack L1-fee receipt columns (`l1_fee`, `l1_gas_used`,
`l1_gas_price`, `l1_fee_scalar`)." Running cryo against Base mainnet failed at
the first step, fetching a block:

> Failed to get block: deserialization error: data did not match any variant
> of untagged enum `BlockTransactions`

Every OP Stack block contains, as its first transaction, a **deposit
transaction** (EIP-2718 type `0x7e`) — the system transaction that records L1
block attributes, carrying OP-specific fields (`sourceHash`, `mint`,
`depositReceiptVersion`). cryo types transactions with alloy's Ethereum
`TxEnvelope`, which has no `0x7e` variant. alloy deserialises a block's
transaction list all-or-nothing, so that single unrecognised transaction fails
the whole block — and cryo never gets past it.

The consequence is broad: it is not only the L1-fee receipt fields that are
missing — **no** dataset (blocks, transactions, logs, traces, …) can be
collected from an OP Stack chain. P4-6's original framing assumed cryo could
already read OP Stack data and merely needed extra columns. It cannot.

## Decision

OP Stack support is an **epic**, not a medium task. P4-6 (#48) is re-scoped
accordingly.

The proposed approach is to make cryo's type layer OP-aware via
[`op-alloy`](https://github.com/alloy-rs/op-alloy) — alloy's OP Stack
companion, which provides `OpTxEnvelope` (including the deposit variant) and
the OP block and receipt types (the L1-fee fields among them). The concrete
integration strategy is to be settled in a sub-issue when the epic is
scheduled; two broad options:

1. **Generic `AnyNetwork`.** Retype cryo's provider to alloy's `AnyNetwork`,
   whose block/transaction/receipt types deserialise permissively and capture
   chain-specific fields (the deposit transaction, the OP receipt fields) in
   catch-all `other` maps. One code path for every chain; chain-specific fields
   are weakly typed.
2. **OP-typed path.** Detect the chain and use `op-alloy`'s typed OP envelope
   and receipt on OP Stack chains. Strongly typed, but requires either a
   parallel code path or a network-generic refactor of the provider layer.

Either is invasive — it touches the provider and every block/transaction/
receipt type usage, on a scale comparable to the alloy 2.x migration (Phase 1).
Hence: an epic, with its own sub-issue breakdown.

This ADR is **Proposed**, not Accepted: it records the analysis and the
intended direction so they are not lost, but OP Stack support is **sidelined**
for now and no implementation work is scheduled. When the epic is taken up the
ADR is refined — the chosen approach filled in — and moved to Accepted.

## Consequences

- Until the epic is done, cryo-ocd **does not support OP Stack chains**, and
  documentation that implies otherwise (the README lists Optimism among
  supported chains) is inaccurate and should be corrected.
- The P4-6 receipt-column work — the four `l1_*` columns and a raw-receipt
  fetch path — is implemented on a parked branch
  (`feat/48-op-stack-receipt-fields`). It builds and its unit test passes, but
  it is inert: cryo never reaches receipt handling on an OP chain. The branch
  is a starting point for the epic's receipt-column piece.
- `op-alloy` becomes a new dependency to track and keep aligned with `alloy`.
- Non-OP EVM chains (Ethereum L1, Polygon PoS, BNB Smart Chain, …) are
  unaffected. Other non-standard transaction models — Arbitrum Nitro's, for
  example — are outside this ADR's scope and would need their own assessment.
- The cost is the epic's size; the benefit is that a large and growing share
  of EVM activity (the Superchain) becomes reachable.
