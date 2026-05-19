# RPC Data Sources

`cryo` collects all of its data from an Ethereum-compatible node over
[JSON-RPC](https://ethereum.org/en/developers/docs/apis/json-rpc/).

## Specifying a node

Provide the node endpoint with `--rpc <url>`, or set the `ETH_RPC_URL`
environment variable:

```sh
cryo blocks -b 18000000:18000010 --rpc http://localhost:8545

ETH_RPC_URL=http://localhost:8545 cryo blocks -b 18000000:18000010
```

If [MESC](https://github.com/paradigmxyz/mesc) is configured, `cryo` uses it to
resolve endpoints, and `--rpc` may instead be an endpoint name or a chain id.

## Connection transports

`cryo` selects the connection transport from the endpoint's URL scheme — there
is no separate flag:

| Endpoint form | Transport |
|---------------|-----------|
| `http://…`, `https://…` | HTTP |
| `ws://…`, `wss://…` | WebSocket |
| a filesystem path ending in `.ipc` | IPC (local socket) |
| a bare `host:port` with no scheme | assumed to be HTTP |

```sh
# HTTP
cryo blocks -b 18000000:18000010 --rpc http://localhost:8545

# WebSocket
cryo blocks -b 18000000:18000010 --rpc ws://localhost:8546

# IPC
cryo blocks -b 18000000:18000010 --rpc /path/to/reth.ipc
```

All transports expose the same datasets and honour the same retry and
rate-limiting options. Endpoints that require JWT authentication are not yet
supported — that work is tracked in
[#107](https://github.com/trial123Zel/cryo-ocd/issues/107).

## Supported chains

`cryo` works with EVM chains that use standard Ethereum block and transaction
formats — Ethereum, Polygon PoS, BNB Smart Chain, Avalanche C-Chain, and
similar. OP Stack chains (Optimism, Base) are not yet supported; see
[ADR-0006](../../docs/adr/0006-op-stack-support.md).
