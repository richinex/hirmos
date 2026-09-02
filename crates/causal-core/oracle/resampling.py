"""Generate the pandas 2.3.3 oracle for the supported Hirmos downsampling lane."""

from __future__ import annotations

import json
from calendar import monthrange
from pathlib import Path

import pandas as pd


ROOT = Path(__file__).resolve().parent
OUT = ROOT / "fixtures" / "resampling.json"


def expected_days(label: pd.Timestamp, target: str) -> int:
    return 7 if target == "weekly" else monthrange(label.year, label.month)[1]


def run_case(
    timestamps: pd.DatetimeIndex,
    columns: dict[str, list[float]],
    aggregations: dict[str, str],
    target: str,
    incomplete: str,
) -> dict[str, object]:
    frame = pd.DataFrame(columns, index=timestamps)
    rule = "W-MON" if target == "weekly" else "MS"
    grouped = frame.resample(rule, closed="left", label="left")
    result = grouped.agg(aggregations)
    counts = grouped.size()
    complete = pd.Series(
        [count == expected_days(label, target) for label, count in counts.items()],
        index=counts.index,
    )
    retained = complete if incomplete == "drop" else pd.Series(True, index=complete.index)
    result = result.loc[retained]

    kept_source_rows = int(counts.loc[retained].sum())
    return {
        "target": target,
        "incomplete": incomplete,
        "timestamps_ms": [int(stamp.value // 1_000_000) for stamp in timestamps],
        "columns": list(columns),
        "aggregations": [aggregations[name] for name in columns],
        "values": [value for name in columns for value in columns[name]],
        "expected": {
            "timestamps_ms": [int(stamp.value // 1_000_000) for stamp in result.index],
            "values": [float(value) for name in columns for value in result[name]],
            "source_rows": len(frame),
            "output_rows": len(result),
            "incomplete_bins": int((~complete).sum()),
            "bins_dropped": int((~retained).sum()),
            "source_rows_dropped": len(frame) - kept_source_rows,
        },
    }


def main() -> None:
    days = pd.date_range("2026-01-01 12:30:00+00:00", periods=14, freq="D")
    week = [7.0, 1.0, 5.0, 3.0, 9.0, 2.0, 4.0, 8.0, 6.0, 10.0, 12.0, 11.0, 13.0, 14.0]
    columns = {
        "mean": week,
        "sum": [1e16, 1.0, -1e16, 3.0, 9.0, 2.0, 4.0, *week[7:]],
        "median": week,
        "min": week,
        "max": week,
        "first": week,
        "last": week,
    }
    aggregations = {name: name for name in columns}

    leap = pd.date_range("2024-02-01", periods=30, freq="D", tz="UTC")
    leap_columns = {"sum": [float(value) for value in range(1, 31)]}
    leap_aggregations = {"sum": "sum"}

    cases = {
        "pandas_version": pd.__version__,
        "pandas_revision": "9c8bc3e55188c8aff37207a74f1dd144980b8874",
        "source_contract": {
            "weekly": "DataFrame.resample('W-MON', closed='left', label='left').agg(...) ",
            "monthly": "DataFrame.resample('MS', closed='left', label='left').agg(...) ",
        },
        "cases": [
            run_case(days, columns, aggregations, "weekly", "keep"),
            run_case(days, columns, aggregations, "weekly", "drop"),
            run_case(leap, leap_columns, leap_aggregations, "monthly", "keep"),
            run_case(leap, leap_columns, leap_aggregations, "monthly", "drop"),
        ],
    }
    OUT.write_text(json.dumps(cases, indent=2, allow_nan=False) + "\n")


if __name__ == "__main__":
    main()
