"""Executable missing-data oracle for Hirmos and the Rust transpile.

This module deliberately separates two operations with different scientific meanings:

* :func:`tigramite_case` asks the vendored/reference Tigramite implementation which
  lagged samples survive its missing-value and analysis-mask rules.
* The imputation helpers create explicit replacement values and return an imputed-cell
  bitmap.  They never alter the time axis and never fill a gap longer than ``max_gap``.

Run ``uv run python oracle/preprocessing.py`` to regenerate the Rust parity fixture.
"""

from __future__ import annotations

from dataclasses import dataclass
import json
from pathlib import Path
from typing import Literal, Sequence

import numpy as np
from tigramite import data_processing as pp


CutOff = Literal[
    "2xtau_max",
    "tau_max",
    "max_lag",
    "max_lag_or_tau_max",
    "2xtau_max_future",
]


@dataclass(frozen=True)
class ImputationResult:
    values: np.ndarray
    valid: np.ndarray
    imputed: np.ndarray

    @property
    def unresolved(self) -> np.ndarray:
        return ~self.valid


@dataclass(frozen=True)
class ParsedNullable:
    values: np.ndarray
    valid: np.ndarray
    recognized_missing: tuple[str, ...]


def parse_nullable_rows(
    rows: Sequence[Sequence[object]],
    *,
    missing_tokens: Sequence[str] = ("", "NA", "N/A", "NULL"),
    numeric_sentinel: float | None = None,
    confirm_numeric_sentinel: bool = False,
) -> ParsedNullable:
    """Parse tabular numeric text without ever turning a null into a valid zero.

    Text tokens are matched case-insensitively after trimming. A numeric sentinel is
    accepted only with explicit confirmation because it may be a legitimate observation.
    Invalid storage is normalized to zero, while ``valid`` remains authoritative.
    """
    if not rows or not rows[0] or any(len(row) != len(rows[0]) for row in rows):
        raise ValueError("rows must be a non-empty rectangular matrix")
    if numeric_sentinel is not None and not confirm_numeric_sentinel:
        raise ValueError("numeric_sentinel requires explicit confirmation")
    tokens = {str(token).strip().casefold() for token in missing_tokens}
    values = np.zeros((len(rows), len(rows[0])), dtype=float)
    valid = np.ones_like(values, dtype=bool)
    recognized: set[str] = set()
    for time, row in enumerate(rows):
        for variable, raw in enumerate(row):
            text = "" if raw is None else str(raw).strip()
            if text.casefold() in tokens:
                valid[time, variable] = False
                recognized.add(text)
                continue
            value = float(text)
            if np.isnan(value):
                valid[time, variable] = False
                recognized.add(text)
            elif numeric_sentinel is not None and value == numeric_sentinel:
                valid[time, variable] = False
                recognized.add(text)
            else:
                values[time, variable] = value
    return ParsedNullable(values, valid, tuple(sorted(recognized)))


def _matrix(values: Sequence[Sequence[float]], valid: Sequence[Sequence[bool]]):
    values_array = np.asarray(values, dtype=float)
    valid_array = np.asarray(valid, dtype=bool)
    if values_array.ndim != 2 or values_array.shape != valid_array.shape:
        raise ValueError("values and validity must be equally shaped 2-D matrices")
    if np.any(np.isnan(values_array) & valid_array):
        raise ValueError("a valid cell cannot contain NaN")
    return values_array.copy(), valid_array.copy()


def linear_interpolate(
    values: Sequence[Sequence[float]],
    valid: Sequence[Sequence[bool]],
    *,
    max_gap: int,
    coordinates: Sequence[float] | None = None,
) -> ImputationResult:
    """Fill bounded interior gaps using their actual time coordinates.

    Leading/trailing gaps and runs longer than ``max_gap`` remain invalid.  This is
    intentionally narrower than pandas' default and DuckDB ``fill``, both of which can
    extrapolate at an edge depending on their options.
    """
    if max_gap < 1:
        raise ValueError("max_gap must be positive")
    out, validity = _matrix(values, valid)
    times = np.arange(out.shape[0], dtype=float) if coordinates is None else np.asarray(coordinates, dtype=float)
    if times.shape != (out.shape[0],) or np.any(~np.isfinite(times)) or np.any(np.diff(times) <= 0):
        raise ValueError("coordinates must be finite, strictly increasing, and match the rows")
    imputed = np.zeros_like(validity)
    for column in range(out.shape[1]):
        cursor = 0
        while cursor < out.shape[0]:
            if validity[cursor, column]:
                cursor += 1
                continue
            start = cursor
            while cursor < out.shape[0] and not validity[cursor, column]:
                cursor += 1
            end = cursor
            if start == 0 or end == out.shape[0] or end - start > max_gap:
                continue
            left, right = start - 1, end
            width = times[right] - times[left]
            for row in range(start, end):
                weight = (times[row] - times[left]) / width
                out[row, column] = out[left, column] + weight * (out[right, column] - out[left, column])
                validity[row, column] = True
                imputed[row, column] = True
    return ImputationResult(out, validity, imputed)


def forward_fill(
    values: Sequence[Sequence[float]],
    valid: Sequence[Sequence[bool]],
    *,
    max_gap: int,
) -> ImputationResult:
    """Carry a prior state through a gap only when the entire run is within budget."""
    if max_gap < 1:
        raise ValueError("max_gap must be positive")
    out, validity = _matrix(values, valid)
    imputed = np.zeros_like(validity)
    for column in range(out.shape[1]):
        cursor = 0
        while cursor < out.shape[0]:
            if validity[cursor, column]:
                cursor += 1
                continue
            start = cursor
            while cursor < out.shape[0] and not validity[cursor, column]:
                cursor += 1
            end = cursor
            if start == 0 or end - start > max_gap:
                continue
            out[start:end, column] = out[start - 1, column]
            validity[start:end, column] = True
            imputed[start:end, column] = True
    return ImputationResult(out, validity, imputed)


def structural_zero(
    values: Sequence[Sequence[float]], valid: Sequence[Sequence[bool]]
) -> ImputationResult:
    """Replace every invalid value with zero after the caller confirms that meaning."""
    out, validity = _matrix(values, valid)
    imputed = ~validity
    out[imputed] = 0.0
    validity[imputed] = True
    return ImputationResult(out, validity, imputed)


def longest_complete_interval(
    valid: Sequence[Sequence[bool]], columns: Sequence[int] | None = None
) -> tuple[int, int] | None:
    """Return the earliest longest half-open interval complete in selected columns."""
    validity = np.asarray(valid, dtype=bool)
    if validity.ndim != 2:
        raise ValueError("validity must be a 2-D matrix")
    selected = np.arange(validity.shape[1]) if columns is None else np.asarray(columns, dtype=int)
    if selected.size == 0 or np.any(selected < 0) or np.any(selected >= validity.shape[1]):
        raise ValueError("at least one in-range column is required")
    complete = np.all(validity[:, selected], axis=1)
    best: tuple[int, int] | None = None
    cursor = 0
    while cursor < len(complete):
        if not complete[cursor]:
            cursor += 1
            continue
        start = cursor
        while cursor < len(complete) and complete[cursor]:
            cursor += 1
        candidate = (start, cursor)
        if best is None or candidate[1] - candidate[0] > best[1] - best[0]:
            best = candidate
    return best


def _base_data() -> np.ndarray:
    data = np.arange(18 * 4, dtype=float).reshape(18, 4)
    data[:, 0] /= 10.0
    data[:, 1] = 100.0 + data[:, 1]
    data[:, 2] = -20.0 + data[:, 2] / 3.0
    data[:, 3] = np.sin(np.arange(18) / 3.0)
    data[9, 0] = 0.0  # A legitimate zero retained beside sentinel-based missingness.
    return data


def tigramite_case(
    *,
    name: str,
    cutoff: CutOff,
    mask_type: str | None = None,
    propagate: bool = False,
    missing: bool = True,
    reference_points: Sequence[int] | None = None,
    extra_z: Sequence[tuple[int, int]] = (),
    include_mask: bool = True,
) -> dict:
    """Run one authoritative single-series ``DataFrame.construct_array`` case."""
    sentinel = -999.0
    data = _base_data()
    if missing:
        data[7, 2] = sentinel
        data[12, 1] = sentinel
    mask = np.zeros_like(data, dtype=bool)
    mask[9, 0] = True
    mask[11, 1] = True
    mask[6, 2] = True
    data_type = np.zeros_like(data, dtype=bool)
    data_type[:, 1] = True
    x = [(0, -1), (0, -1)]
    y = [(1, 0)]
    z = [(2, -2), (0, -1), (3, 0)]
    frame = pp.DataFrame(
        data,
        mask=mask if include_mask else None,
        missing_flag=sentinel if missing else None,
        data_type=data_type,
        reference_points=None if reference_points is None else list(reference_points),
        remove_missing_upto_maxlag=propagate,
    )
    array, xyz, cleaned, types = frame.construct_array(
        x,
        y,
        z,
        tau_max=3,
        extraZ=list(extra_z),
        mask_type=mask_type,
        cut_off=cutoff,
        return_cleaned_xyz=True,
    )
    return {
        "name": name,
        "values": data.tolist(),
        "missing_flag": sentinel if missing else None,
        "analysis_mask": mask.tolist() if include_mask else None,
        "data_type": data_type.tolist(),
        "x": x,
        "y": y,
        "z": z,
        "extra_z": list(extra_z),
        "tau_max": 3,
        "cutoff": cutoff,
        "mask_type": mask_type,
        "propagate": propagate,
        "reference_points": list(range(len(data))) if reference_points is None else list(reference_points),
        "array": array.tolist(),
        "xyz": xyz.tolist(),
        "cleaned": [[list(node) for node in group] for group in cleaned],
        "type_array": types.astype(int).tolist(),
        "retained_reference_points": frame.use_indices_dataset_dict[0].tolist(),
    }


def build_fixture() -> dict:
    cases = []
    for cutoff in ("2xtau_max", "tau_max", "max_lag", "max_lag_or_tau_max", "2xtau_max_future"):
        cases.append(tigramite_case(name=f"cutoff-{cutoff}", cutoff=cutoff))
    for mask_type in (None, "x", "y", "z", "xy", "xz", "yz", "xyz"):
        cases.append(
            tigramite_case(
                name=f"mask-{mask_type or 'none'}", cutoff="tau_max", mask_type=mask_type
            )
        )
    cases.extend(
        [
            tigramite_case(name="propagation-off", cutoff="tau_max", propagate=False),
            tigramite_case(name="propagation-on", cutoff="tau_max", propagate=True),
            tigramite_case(
                name="reference-window-extra-z",
                cutoff="tau_max",
                mask_type="xyz",
                reference_points=range(4, 16),
                extra_z=[(2, -1)],
            ),
            tigramite_case(name="no-missing-flag", cutoff="2xtau_max", missing=False),
            tigramite_case(
                name="no-analysis-mask", cutoff="tau_max", include_mask=False, mask_type=None
            ),
        ]
    )

    values = [[0.0, 10.0], [99.0, 11.0], [99.0, 12.0], [6.0, 13.0], [99.0, 14.0], [12.0, 15.0]]
    valid = [[True, True], [False, True], [False, True], [True, True], [False, True], [True, True]]
    coordinates = [0.0, 1.0, 2.5, 4.0, 7.0, 10.0]
    linear = linear_interpolate(values, valid, max_gap=2, coordinates=coordinates)
    forward = forward_fill(values, valid, max_gap=2)
    zero = structural_zero(values, valid)
    parsed = parse_nullable_rows(
        [["0", "NA"], [" 1.5 ", "-999"], ["NULL", "2"]],
        numeric_sentinel=-999.0,
        confirm_numeric_sentinel=True,
    )
    return {
        "tigramite": cases,
        "nullable_parsing": {
            "values": parsed.values.tolist(),
            "valid": parsed.valid.tolist(),
            "recognized_missing": list(parsed.recognized_missing),
        },
        "imputation": {
            "values": values,
            "valid": valid,
            "coordinates": coordinates,
            "max_gap": 2,
            "linear": {
                "values": linear.values.tolist(),
                "valid": linear.valid.tolist(),
                "imputed": linear.imputed.tolist(),
            },
            "forward_fill": {
                "values": forward.values.tolist(),
                "valid": forward.valid.tolist(),
                "imputed": forward.imputed.tolist(),
            },
            "structural_zero": {
                "values": zero.values.tolist(),
                "valid": zero.valid.tolist(),
                "imputed": zero.imputed.tolist(),
            },
            "complete_interval": longest_complete_interval(valid),
        },
    }


if __name__ == "__main__":
    fixture = build_fixture()
    output = Path(__file__).with_name("fixtures") / "missing_preprocessing.json"
    output.write_text(json.dumps(fixture, indent=1) + "\n")
    print(f"wrote {output} ({len(fixture['tigramite'])} Tigramite cases)")
