# Builds the cryo CLI as a container image.
#
# Multi-stage: a Rust builder compiles the `cryo` binary, then a slim Debian
# runtime carries just the binary and the TLS trust store.
#
#   docker build -t cryo .
#   docker run --rm -v "$PWD:/data" -e ETH_RPC_URL cryo blocks -b 18000000:18000010

# --- builder -----------------------------------------------------------------
# Pinned to the bookworm suite so the builder's glibc matches the
# debian:bookworm-slim runtime below. A newer suite here (the unsuffixed
# `-slim` tag now resolves to trixie) links cryo against a glibc the runtime
# image does not provide.
FROM rust:1.95.0-slim-bookworm AS builder

# The release workflow passes the version (from `git describe`) as a build
# arg; the build context excludes .git, so without it cryo's build script
# falls back to the Cargo.toml version. See crates/freeze/build.rs.
ARG CRYO_VERSION
ENV CRYO_VERSION=${CRYO_VERSION}

# The rustls crypto backends (aws-lc-sys, ring) build C/assembly sources and
# need CMake and Perl in addition to the C compiler the rust image ships.
RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake perl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY . .

# --locked builds against the committed Cargo.lock; -p cryo_cli skips the
# Python extension crate, which is not part of the CLI image.
RUN cargo build --release --locked --package cryo_cli

# --- runtime -----------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

# cryo reaches RPC endpoints over HTTPS; rustls validates against the system
# trust store, so ca-certificates must be present.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir /data

COPY --from=builder /build/target/release/cryo /usr/local/bin/cryo

# Runs as root: cryo is a user-run CLI tool, and root keeps bind-mounted
# output directories writable without host-side permission setup (#96).
WORKDIR /data
ENTRYPOINT ["cryo"]
