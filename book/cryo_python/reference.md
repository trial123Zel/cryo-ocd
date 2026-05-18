# Reference

## Functions

cryo's Python package exposes four functions:

- `cryo.collect(datatype, output_format='polars', **kwargs)` — collect a
  dataset and return it in memory.
- `cryo.async_collect(...)` — awaitable form of `collect`.
- `cryo.freeze(datatype, **kwargs)` — collect a dataset and write it to
  files; returns `None`.
- `cryo.async_freeze(...)` — awaitable form of `freeze`.

`datatype` is a dataset name such as `'blocks'`, `'transactions'`, `'logs'`,
or `'traces'` — see the [dataset reference](../datasets/dataset_reference.md).
`freeze` also accepts a sequence of dataset names.

## Output formats

`collect` and `async_collect` take an `output_format`:

| `output_format` | Return type |
| --- | --- |
| `'polars'` (default) | `polars.DataFrame` |
| `'pandas'` | `pandas.DataFrame` (needs the `pandas` extra) |
| `'list'` | `list[dict]` — one dict per row |
| `'dict'` | `dict[str, list]` — one list per column |

## Keyword arguments

The keyword arguments mirror the `cryo` CLI flags — `blocks`, `rpc`,
`contract`, `include_columns`, `exclude_columns`, `output_dir`, and the rest.
As a convenience, `start_block` and `end_block` may be given in place of a
`blocks` range, and `file_format` (`'parquet'`, `'csv'`, or `'json'`) in place
of the individual format flags.

The full set is defined by the `CryoCliArgs` type in
[`crates/python/python/cryo/_spec.py`](https://github.com/trial123Zel/cryo-ocd/blob/main/crates/python/python/cryo/_spec.py);
each entry corresponds to a CLI flag documented in the
[CLI reference](../reference/interfaces/cli.md).
