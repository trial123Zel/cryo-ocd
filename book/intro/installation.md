# Installation

cryo-ocd is not published to crates.io or PyPI. Install the `cryo` CLI one of
three ways.

## Prebuilt binary (recommended)

Download the archive for your platform from the
[latest release](https://github.com/trial123Zel/cryo-ocd/releases), extract it,
and move the `cryo` binary onto your `PATH`:

| Platform | Archive |
| :- | :- |
| Linux, x86_64 | `cryo-<version>-x86_64-unknown-linux-gnu.tar.gz` |
| macOS, Apple Silicon | `cryo-<version>-aarch64-apple-darwin.tar.gz` |
| Windows, x86_64 | `cryo-<version>-x86_64-pc-windows-msvc.zip` |

The binary is self-contained — no toolchain is required to run it.

## Build from source

```
git clone https://github.com/trial123Zel/cryo-ocd
cd cryo-ocd
cargo install --path ./crates/cli
```

Building from source requires a Rust toolchain ([rustup](https://rustup.rs/))
and a C compiler — cryo's TLS dependencies build C sources. Ensure
`~/.cargo/bin` is on your `PATH`.

## Docker

```
git clone https://github.com/trial123Zel/cryo-ocd
cd cryo-ocd
docker build -t cryo .
docker run --rm -v "$PWD:/data" -e ETH_RPC_URL cryo blocks -b 18000000:18000010
```

The image is built from the repository's `Dockerfile`; it is not published to a
registry. Mount a host directory at `/data` to collect output onto the host,
and supply the RPC endpoint via `ETH_RPC_URL` (or `--rpc`).

## Python package

To use cryo from Python, see
[Cryo Python → Installation](../cryo_python/installation.md).
