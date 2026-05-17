"""Regression tests for cryo_test.polars_utils."""

from __future__ import annotations

import polars as pl

from cryo_test.polars_utils import find_df_differences, find_row_differences


def test_row_count_difference_is_reported() -> None:
    # Regression (#75): a row-count mismatch was computed but never
    # appended to the differences list, so it was silently dropped.
    a = pl.DataFrame({'x': [1, 2, 3]})
    b = pl.DataFrame({'x': [1, 2]})
    diffs = find_row_differences({'a': a, 'b': b})
    assert len(diffs) == 1
    assert 'different number of rows' in diffs[0]


def test_equal_frames_report_no_difference() -> None:
    a = pl.DataFrame({'x': [1, 2, 3]})
    b = pl.DataFrame({'x': [1, 2, 3]})
    assert find_row_differences({'a': a, 'b': b}) == []


def test_value_difference_is_still_reported() -> None:
    a = pl.DataFrame({'x': [1, 2, 3]})
    b = pl.DataFrame({'x': [1, 2, 4]})
    diffs = find_row_differences({'a': a, 'b': b})
    assert len(diffs) == 1
    assert 'do not match' in diffs[0]


def test_find_df_differences_flags_row_count_mismatch() -> None:
    # the public entry point: matching schemas but differing row counts
    a = pl.DataFrame({'x': [1, 2, 3]})
    b = pl.DataFrame({'x': [1, 2]})
    diffs = find_df_differences({'a': a, 'b': b})
    assert any('different number of rows' in d for d in diffs)
