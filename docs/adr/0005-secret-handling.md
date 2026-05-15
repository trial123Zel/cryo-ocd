# ADR-0005: Secret handling & .env convention

- **Status:** Accepted
- **Date:** 2026-05-15

## Context

cryo-ocd is a public repository. The project interacts with private
infrastructure — an RPC node on a private network — and with configuration that
must never become public: API keys, WAN/LAN IP addresses, RPC URLs, and other
PII.

A scan of the working tree and the full git history at fork time found no such
values committed, and `.gitignore` already ignores `*.env`. The goal is to keep
it that way.

## Decision

- **No secrets in the repository.** API keys, RPC URLs, IP addresses, and PII
  are never committed — not to files, not to workflow definitions, and not to
  GitHub-stored secrets where it can be avoided.
- **All environment-specific values go in a local `.env`**, which is
  git-ignored. A committed `.env.example` documents every variable with
  placeholder values only.
- The node RPC URL used by CI lives in a local `.env` on the self-hosted runner
  host (see ADR-0004); it is read at job time and never transmitted to GitHub.
- **CI runs a secret scanner** on every push; commits containing secret-like
  values are rejected.
- The `LICENSE-*` files are guarded by CI against modification (see ADR-0001).

## Consequences

- Contributors must copy `.env.example` to `.env` and fill in their own values.
- A planted-secret test is used to confirm the scanner fails closed.
- Environment-variable names (e.g. `RETH_RPC_URL`, `ETH_RPC_URL`) are part of
  the project's public interface and are documented in `.env.example`.
