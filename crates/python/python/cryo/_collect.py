from __future__ import annotations
import typing

if typing.TYPE_CHECKING:
    from typing_extensions import Unpack
    from typing import Any, Literal, TypeVar

    import pandas as pd
    import polars as pl
    from . import _spec

    ListOfDicts = list[dict[str, Any]]
    DictOfLists = dict[str, list[Any]]
    T = TypeVar('T', pl.DataFrame, pd.DataFrame, ListOfDicts, DictOfLists)


@typing.overload
async def async_collect(
    datatype: _spec.Datatype,
    output_format: Literal['polars'] = 'polars',
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> pl.DataFrame:
    ...


@typing.overload
async def async_collect(
    datatype: _spec.Datatype,
    output_format: Literal['pandas'],
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> pd.DataFrame:
    ...


@typing.overload
async def async_collect(
    datatype: _spec.Datatype,
    output_format: Literal['list'],
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> ListOfDicts:
    ...


@typing.overload
async def async_collect(
    datatype: _spec.Datatype,
    output_format: Literal['dict'],
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> DictOfLists:
    ...


async def async_collect(
    datatype: _spec.Datatype,
    output_format: _spec.PythonOutput = 'polars',
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> pl.DataFrame | pd.DataFrame | ListOfDicts | DictOfLists:
    """Asynchronously collect a dataset and return it in memory.

    Awaitable equivalent of :func:`collect`; see it for the parameters.
    """

    from . import _args

    # parse inputs
    cli_args = _args.parse_cli_args(**kwargs)

    # collect data, showing a progress bar unless verbose was disabled
    show_progress = not cli_args.get('no_verbose', False)
    result: pl.DataFrame = await _collect_in_chunks(datatype, cli_args, show_progress)

    # format output
    if output_format == 'polars':
        return result
    elif output_format == 'pandas':
        return result.to_pandas()
    elif output_format == 'list':
        return result.to_dicts()
    elif output_format == 'dict':
        return result.to_dict(as_series=False)
    else:
        raise Exception('unknown output format')


@typing.overload
def collect(
    datatype: _spec.Datatype,
    output_format: Literal['polars'] = 'polars',
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> pl.DataFrame:
    ...


@typing.overload
def collect(
    datatype: _spec.Datatype,
    output_format: Literal['pandas'],
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> pd.DataFrame:
    ...


@typing.overload
def collect(
    datatype: _spec.Datatype,
    output_format: Literal['list'],
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> ListOfDicts:
    ...


@typing.overload
def collect(
    datatype: _spec.Datatype,
    output_format: Literal['dict'],
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> DictOfLists:
    ...


def collect(
    datatype: _spec.Datatype,
    output_format: _spec.PythonOutput = 'polars',
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> pl.DataFrame | pd.DataFrame | ListOfDicts | DictOfLists:
    """Collect a dataset and return it in memory.

    ``datatype`` is the dataset to collect, such as ``"blocks"``,
    ``"transactions"``, or ``"logs"``. ``output_format`` selects the return
    type: ``"polars"`` (default; a ``polars.DataFrame``), ``"pandas"`` (a
    ``pandas.DataFrame``, which needs the ``pandas`` extra), ``"list"`` (a
    list of per-row dicts), or ``"dict"`` (a dict of column name to value
    list).

    The remaining keyword arguments mirror the ``cryo`` CLI flags, for
    example ``blocks``, ``rpc``, ``contract``, and ``include_columns``, plus
    the convenience pair ``start_block`` / ``end_block``. See
    ``cryo._spec.CryoCliArgs`` for the full set.

    A ``tqdm`` progress bar is shown while collecting an explicit block
    range; pass ``verbose=False`` to suppress it.

    Use :func:`freeze` to write the data to files instead, or
    :func:`async_collect` from within an async context.
    """

    import asyncio

    coroutine = async_collect(datatype, output_format=output_format, **kwargs)

    try:
        import concurrent.futures

        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        with concurrent.futures.ThreadPoolExecutor() as executor:
            future = executor.submit(loop.run_until_complete, coroutine)  # type: ignore
            result: T = future.result()  # type: ignore
    except RuntimeError:
        result = asyncio.run(coroutine)

    return result


# the cryo `_collect` binding collects a single partition, so each per-chunk
# call sets a chunk size that spans its whole sub-range
_PROGRESS_CHUNK_TARGET = 100  # sub-ranges to split a collection into
_SINGLE_PARTITION_CHUNK_SIZE = 20_000_000  # fallback chunk size for one call


async def _collect_in_chunks(
    datatype: _spec.Datatype,
    cli_args: _spec.CryoCliArgs,
    show_progress: bool,
) -> pl.DataFrame:
    """Collect a dataset, advancing a progress bar across sub-ranges.

    An explicit ``START:END`` block range is split into sub-ranges collected
    one after another so a ``tqdm`` bar can track progress. Any other block
    specification (open-ended ranges, every-nth syntax, parquet-file inputs,
    transaction-based collection) is collected in a single call without a bar.
    """
    from . import _cryo_rust  # type: ignore

    chunks = _plan_block_chunks(cli_args.get('blocks'))

    if not chunks:
        single = dict(cli_args)
        single['chunk_size'] = _SINGLE_PARTITION_CHUNK_SIZE
        return await _cryo_rust._collect(datatype, **single)

    total_blocks = sum(end - start for start, end in chunks)
    bar = _make_progress_bar(total_blocks, show_progress, datatype)
    frames = []
    try:
        for start, end in chunks:
            chunk_args = dict(cli_args)
            chunk_args['blocks'] = [str(start) + ':' + str(end)]
            chunk_args['chunk_size'] = max(1, end - start)
            chunk_args.pop('n_chunks', None)
            frames.append(await _cryo_rust._collect(datatype, **chunk_args))
            if bar is not None:
                bar.update(end - start)
    finally:
        if bar is not None:
            bar.close()

    import polars as pl

    return pl.concat(frames)


def _make_progress_bar(
    total: int,
    show_progress: bool,
    datatype: _spec.Datatype,
) -> Any:
    """Build a tqdm progress bar, or None if disabled or tqdm is unavailable."""
    if not show_progress:
        return None
    try:
        from tqdm.auto import tqdm
    except ImportError:
        return None
    return tqdm(
        total=total, desc='collecting ' + str(datatype), unit=' blocks', unit_scale=True
    )


def _plan_block_chunks(
    blocks: typing.Sequence[str] | None,
) -> list[tuple[int, int]] | None:
    """Split a plain ``START:END`` block range into end-exclusive sub-ranges.

    Returns ``None`` if ``blocks`` is not a single closed numeric range — open
    ranges, every-nth syntax and parquet-file inputs are left for a single
    collection call.
    """
    if not blocks or len(blocks) != 1:
        return None
    parts = blocks[0].split(':')
    if len(parts) != 2:
        return None
    start = _parse_block_number(parts[0])
    end = _parse_block_number(parts[1])
    if start is None or end is None or end <= start:
        return None

    total = end - start
    chunk = max(1, -(-total // _PROGRESS_CHUNK_TARGET))
    if total <= chunk:
        return None
    return [(b, min(b + chunk, end)) for b in range(start, end, chunk)]


def _parse_block_number(text: str) -> int | None:
    """Parse a cryo block number (supporting ``_`` and ``K`` / ``M`` / ``B``)."""
    text = text.strip().replace('_', '')
    if not text:
        return None
    multiplier = 1
    if text[-1] in 'kKmMbB':
        multiplier = {'k': 10**3, 'm': 10**6, 'b': 10**9}[text[-1].lower()]
        text = text[:-1]
    try:
        value = float(text) * multiplier
    except ValueError:
        return None
    return int(value) if value == int(value) else None
