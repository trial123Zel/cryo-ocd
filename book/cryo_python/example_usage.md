# Example Usage

These examples assume an RPC endpoint. Pass it as `rpc=...`, or set the
`ETH_RPC_URL` environment variable and omit the argument.

## Collect a dataset into a DataFrame

```python
import cryo

df = cryo.collect('blocks', blocks=['18000000:18000100'])
```

## Choose the output format

`collect` returns a polars DataFrame by default. Other formats:

```python
# pandas DataFrame (needs the pandas extra)
pdf = cryo.collect('transactions', blocks=['18000000:18000010'],
                    output_format='pandas')

# list of per-row dicts
rows = cryo.collect('logs', blocks=['18000000:18000010'],
                     output_format='list')
```

## Write data to files

`freeze` writes files instead of returning data — the same output the CLI
produces:

```python
cryo.freeze('logs', blocks=['18000000:18000100'],
            output_dir='./data', file_format='parquet')
```

## Filter and shape the query

Keyword arguments mirror the CLI flags:

```python
df = cryo.collect(
    'logs',
    blocks=['18000000:18000100'],
    contract=['0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'],  # USDC
    include_columns=['transaction_hash'],
)
```

## Async usage

Inside an event loop, use the awaitable variants:

```python
df = await cryo.async_collect('blocks', blocks=['18000000:18000100'])
```
