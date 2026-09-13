"""Run preparation scripts with nullable pandas inputs and Arrow transport."""

import contextlib
import io
import traceback

import numpy as np
import pandas as pd
import pyarrow as pa
import pyarrow.compute as pc
import pyarrow.ipc as ipc

# Installing PyArrow must not change the default string behavior of user scripts.
pd.options.mode.string_storage = "python"

_PANDAS_TYPES = {
    pa.int8(): pd.Int8Dtype(),
    pa.int16(): pd.Int16Dtype(),
    pa.int32(): pd.Int32Dtype(),
    pa.int64(): pd.Int64Dtype(),
    pa.uint8(): pd.UInt8Dtype(),
    pa.uint16(): pd.UInt16Dtype(),
    pa.uint32(): pd.UInt32Dtype(),
    pa.uint64(): pd.UInt64Dtype(),
    pa.float32(): pd.Float32Dtype(),
    pa.float64(): pd.Float64Dtype(),
    pa.bool_(): pd.BooleanDtype(),
    pa.string(): pd.StringDtype(storage="python"),
    pa.large_string(): pd.StringDtype(storage="python"),
}


def _pandas_type(dtype):
    return _PANDAS_TYPES.get(dtype, pd.ArrowDtype(dtype))


def _read_input(path):
    table = ipc.open_file(path).read_all()
    # DuckDB's lossless export uses the canonical bool8 extension. Convert that
    # representation through Arrow before choosing pandas' nullable boolean type.
    for index, field in enumerate(table.schema):
        if field.type == pa.bool8():
            values = pa.chunked_array([chunk.storage.cast(pa.bool_()) for chunk in table.column(index).chunks], type=pa.bool_())
            table = table.set_column(index, field.name, values)
    frame = table.to_pandas(types_mapper=_pandas_type)
    # Nullable pandas conversion treats valid NaNs as missing. Only these special
    # floating columns retain Arrow storage, which keeps NaN distinct from null.
    for index, field in enumerate(table.schema):
        if pa.types.is_floating(field.type) and pc.any(pc.is_nan(table.column(index))).as_py():
            frame[field.name] = table.column(index).to_pandas(types_mapper=pd.ArrowDtype)
    return frame


def _output_type(dtype, nested=False):
    """Reject types that the receiving engine would narrow or reinterpret."""
    if pa.types.is_decimal(dtype) and dtype.precision > 38:
        return "decimal precision exceeds DuckDB's 38-digit limit"
    if pa.types.is_duration(dtype):
        return "duration columns need an explicit numeric unit or interval representation"
    if pa.types.is_timestamp(dtype) and dtype.tz is not None and dtype.unit == "ns":
        return "timezone-aware nanosecond timestamps exceed DuckDB's microsecond precision"
    if pa.types.is_time64(dtype) and dtype.unit == "ns":
        return "nanosecond times exceed DuckDB's microsecond precision"
    if nested and pa.types.is_interval(dtype):
        return "nested interval columns need an explicit supported representation"
    if pa.types.is_union(dtype) or pa.types.is_run_end_encoded(dtype):
        return "this Arrow representation is not supported for Script results"
    if isinstance(dtype, pa.BaseExtensionType):
        if dtype.extension_name not in {"arrow.uuid", "arrow.bool8", "arrow.json", "arrow.opaque"}:
            return "this extension type is not supported for Script results"
        if dtype.extension_name == "arrow.opaque" and (dtype.vendor_name != "DuckDB" or dtype.type_name not in {"hugeint", "uhugeint"}):
            return "this database-specific type is not supported for Script results"
    if pa.types.is_nested(dtype):
        for index in range(dtype.num_fields):
            problem = _output_type(dtype.field(index).type, nested=True)
            if problem is not None:
                return problem
    return None


def _prepared_table(prepared):
    if isinstance(prepared, pd.Series):
        prepared = prepared.to_frame()
    if not isinstance(prepared, pd.DataFrame):
        raise TypeError(f"prepared must be a DataFrame, not {type(prepared).__name__}")
    if any(name is not None for name in prepared.index.names):
        prepared = prepared.reset_index()
    if isinstance(prepared.columns, pd.MultiIndex):
        prepared = prepared.set_axis(["_".join(str(part) for part in column if str(part)) for column in prepared.columns], axis=1)
    names = [str(column) for column in prepared.columns]
    if not names:
        raise ValueError("prepared has no columns")
    if any(not name or "\x00" in name for name in names):
        raise ValueError("prepared column names must be nonempty and contain no null characters")
    if len({name.lower() for name in names}) != len(names):
        raise ValueError("prepared column names must be distinct, including capitalization")
    prepared = prepared.set_axis(names, axis=1)
    try:
        table = pa.Table.from_pandas(prepared, preserve_index=False, nthreads=1, safe=True)
    except (pa.ArrowException, TypeError, ValueError) as error:
        raise ValueError(f"prepared could not be converted to a typed table: {error}. Give mixed or unsupported columns an explicit supported type.") from error
    for field in table.schema:
        problem = _output_type(field.type) or _interval_precision(table.column(field.name))
        if problem is not None:
            raise ValueError(f"Column '{field.name}' ({field.type}): {problem}. Convert that column explicitly in the script.")
    return table


def _interval_precision(column):
    if not pa.types.is_interval(column.type):
        return None
    if any(value is not None and value.nanoseconds % 1000 != 0 for value in column.to_pylist()):
        return "interval nanoseconds exceed DuckDB's microsecond precision"
    return None


def _hirmos_run(code, input_paths, output_path):
    printed = io.StringIO()
    try:
        inputs = [_read_input(path) for path in input_paths]
        namespace = {"inputs": inputs, "pd": pd, "np": np, "__name__": "__hirmos_script__"}
        with contextlib.redirect_stdout(printed):
            exec(compile(code, "<script>", "exec"), namespace)
        if "prepared" not in namespace:
            raise ValueError("the script did not assign prepared")
        table = _prepared_table(namespace["prepared"])
        with pa.OSFile(output_path, "wb") as sink:
            with ipc.new_stream(sink, table.schema) as writer:
                writer.write_table(table)
        return ["ok", table.num_rows, table.column_names, printed.getvalue()]
    except SyntaxError as error:
        return ["error", f"line {error.lineno}: SyntaxError: {error.msg}", printed.getvalue()]
    except Exception as error:
        frames = [frame for frame in traceback.extract_tb(error.__traceback__) if frame.filename == "<script>"]
        where = f"line {frames[-1].lineno}: " if frames else ""
        message = " ".join(str(error).split())
        return ["error", f"{where}{type(error).__name__}: {message}", printed.getvalue()]
