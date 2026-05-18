from __future__ import annotations

import typing

if typing.TYPE_CHECKING:
    from typing_extensions import Unpack

    from . import _spec


async def async_freeze(
    datatype: str | typing.Sequence[str],
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> None:
    """Asynchronously collect a dataset and write it to files on disk.

    Awaitable equivalent of :func:`freeze`; see it for the parameters.
    """

    from . import _cryo_rust  # type: ignore
    from . import _args

    if isinstance(datatype, str):
        datatypes = [datatype]
    elif isinstance(datatype, list):
        datatypes = datatype
    else:
        raise Exception('invalid format for datatype(s)')

    cli_args = _args.parse_cli_args(**kwargs)
    return await _cryo_rust._freeze(datatypes, **cli_args)  # type: ignore


def freeze(
    datatype: str | typing.Sequence[str],
    **kwargs: Unpack[_spec.CryoCliArgs],
) -> None:
    """Collect a dataset and write it to files on disk.

    ``datatype`` is a dataset name, or a sequence of names, to collect, such
    as ``"blocks"`` or ``["blocks", "transactions"]``. The keyword arguments
    mirror the ``cryo`` CLI flags, for example ``blocks``, ``rpc``,
    ``output_dir``, and ``file_format``, plus the convenience pair
    ``start_block`` / ``end_block``. See ``cryo._spec.CryoCliArgs`` for the
    full set.

    Returns ``None``; the data is written as files. Use :func:`collect` to
    return it in memory instead.
    """

    import asyncio

    coroutine = async_freeze(datatype, **kwargs)

    try:
        import concurrent.futures

        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        with concurrent.futures.ThreadPoolExecutor() as executor:
            future = executor.submit(loop.run_until_complete, coroutine)  # type: ignore
            return future.result()  # type: ignore
    except RuntimeError:
        return asyncio.run(coroutine)

