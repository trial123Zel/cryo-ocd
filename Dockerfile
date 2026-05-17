# Builds the cryo CLI as a container image.
#
# Multi-stage: a Rust builder compiles the `cryo` binary, then a slim Debian
# runtime carries just the binary and the TLS trust store.
#
#   docker build -t cryo .
#   docker run --rm -v "$PWD:/data" -e ETH_RPC_URL cryo blocks 18M:18.001M

# --- builder -----------------------------------------------------------------
FROM rust:1.95.0-slim AS builder

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
    && useradd --create-home --uid 10001 cryo \
    && mkdir /data \
    && chown cryo:cryo /data

COPY --from=builder /build/target/release/cryo /usr/local/bin/cryo

USER cryo
WORKDIR /data
ENTRYPOINT ["cryo"]
