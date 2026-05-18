# Cryo Python

cryo can be used directly from Python. The package wraps the same Rust engine
as the CLI, so every dataset and option available on the command line is
available from Python.

There are four entry points:

| Function | Returns | Sync / async |
| --- | --- | --- |
| `cryo.collect` | the data, in memory | sync |
| `cryo.async_collect` | the data, in memory | async |
| `cryo.freeze` | `None` (writes files) | sync |
| `cryo.async_freeze` | `None` (writes files) | async |

`collect` returns a dataset for immediate analysis; `freeze` writes it to
parquet, csv, or json files, exactly as the CLI does. The `async_*` variants
are awaitable, for use inside an existing event loop.

```python
import cryo

# blocks 18,000,000-18,000,099 as a polars DataFrame
df = cryo.collect('blocks', blocks=['18000000:18000100'], rpc='http://...')
```

See [Installation](./installation.md) to set the package up,
[Example Usage](./example_usage.md) for more examples, and
[Reference](./reference.md) for the full argument list.
