# Architecture Decision Records

This directory records significant decisions made in cryo-ocd, so contributors
and downstream forkers can understand *why* the project is the way it is.

An ADR is immutable once accepted. If a decision changes, add a new ADR that
supersedes the old one, and update the old one's **Status**.

| ADR | Title | Status |
|-----|-------|--------|
| [0001](./0001-fork-charter.md) | Fork charter & relationship to upstream | Accepted |
| [0002](./0002-dependency-modernization.md) | Modernize dependencies; target alloy 1.x | Accepted |
| [0003](./0003-retire-clap-cryo.md) | Retire the clap_cryo fork | Accepted |
| [0004](./0004-validation-strategy.md) | Validation strategy: cryo_test + self-hosted runner | Accepted |
| [0005](./0005-secret-handling.md) | Secret handling & .env convention | Accepted |
| [0006](./0006-op-stack-support.md) | OP Stack chain support | Proposed |
| [0007](./0007-multiple-rpc-providers.md) | Multiple RPC providers & graceful rate-limiting | Proposed |

New ADRs use [`template.md`](./template.md).
