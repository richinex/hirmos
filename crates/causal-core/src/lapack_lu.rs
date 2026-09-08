//! Column-major translation of LAPACK 3.12.1 `DGETRF`, `DGETRF2`, and `DGETRS`.

use core::fmt;

const BLOCK_SIZE: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Transpose {
    None,
    Transpose,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LuError {
    InvalidLeadingDimension,
    MatrixTooShort,
    PivotTooShort,
    RightHandSideTooShort,
}

impl fmt::Display for LuError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLeadingDimension => {
                "the leading dimension is smaller than the matrix order"
            }
            Self::MatrixTooShort => "the LU matrix buffer is too short",
            Self::PivotTooShort => "the pivot buffer is too short",
            Self::RightHandSideTooShort => "the right-hand-side buffer is too short",
        })
    }
}

impl std::error::Error for LuError {}

#[inline]
fn at(base: usize, row: usize, column: usize, ld: usize) -> usize {
    base + row + column * ld
}

fn matrix_len(rows: usize, columns: usize, ld: usize) -> usize {
    if rows == 0 || columns == 0 {
        0
    } else {
        rows + (columns - 1) * ld
    }
}

fn swap_rows(a: &mut [f64], base: usize, columns: usize, ld: usize, left: usize, right: usize) {
    if left == right {
        return;
    }
    for column in 0..columns {
        a.swap(at(base, left, column, ld), at(base, right, column, ld));
    }
}

fn apply_pivots(
    a: &mut [f64],
    base: usize,
    columns: usize,
    ld: usize,
    first: usize,
    last: usize,
    pivots: &[usize],
    reverse: bool,
) {
    if reverse {
        for row in (first..last).rev() {
            swap_rows(a, base, columns, ld, row, pivots[row] - 1);
        }
    } else {
        for row in first..last {
            swap_rows(a, base, columns, ld, row, pivots[row] - 1);
        }
    }
}

fn dgetrf2(
    rows: usize,
    columns: usize,
    a: &mut [f64],
    ld: usize,
    base: usize,
    pivots: &mut [usize],
) -> usize {
    if rows == 0 || columns == 0 {
        return 0;
    }
    if rows == 1 {
        pivots[0] = 1;
        return usize::from(a[base] == 0.0);
    }
    if columns == 1 {
        let mut pivot = 0;
        for row in 1..rows {
            if a[base + row].abs() > a[base + pivot].abs() {
                pivot = row;
            }
        }
        pivots[0] = pivot + 1;
        if a[base + pivot] == 0.0 {
            return 1;
        }
        a.swap(base, base + pivot);
        let diagonal = a[base];
        if diagonal.abs() >= f64::MIN_POSITIVE {
            let reciprocal = 1.0 / diagonal;
            for row in 1..rows {
                a[base + row] *= reciprocal;
            }
        } else {
            for row in 1..rows {
                a[base + row] /= diagonal;
            }
        }
        return 0;
    }

    let first_columns = rows.min(columns) / 2;
    let second_columns = columns - first_columns;
    let mut info = dgetrf2(
        rows,
        first_columns,
        a,
        ld,
        base,
        &mut pivots[..first_columns],
    );

    let right_base = base + first_columns * ld;
    apply_pivots(
        a,
        right_base,
        second_columns,
        ld,
        0,
        first_columns,
        &pivots[..first_columns],
        false,
    );

    // DTRSM('L','L','N','U'): A12 <- inv(L11) A12.
    for column in 0..second_columns {
        for row in 0..first_columns {
            let mut value = a[at(right_base, row, column, ld)];
            for inner in 0..row {
                value -= a[at(base, row, inner, ld)] * a[at(right_base, inner, column, ld)];
            }
            a[at(right_base, row, column, ld)] = value;
        }
    }

    // DGEMM('N','N'): A22 <- A22 - A21 A12.
    let lower_rows = rows - first_columns;
    let lower_base = base + first_columns;
    let trailing_base = lower_base + first_columns * ld;
    for column in 0..second_columns {
        for row in 0..lower_rows {
            let mut value = a[at(trailing_base, row, column, ld)];
            for inner in 0..first_columns {
                value -= a[at(lower_base, row, inner, ld)] * a[at(right_base, inner, column, ld)];
            }
            a[at(trailing_base, row, column, ld)] = value;
        }
    }

    let lower_order = lower_rows.min(second_columns);
    let second_info = dgetrf2(
        lower_rows,
        second_columns,
        a,
        ld,
        trailing_base,
        &mut pivots[first_columns..first_columns + lower_order],
    );
    if info == 0 && second_info > 0 {
        info = second_info + first_columns;
    }
    for pivot in &mut pivots[first_columns..first_columns + lower_order] {
        *pivot += first_columns;
    }
    for row in first_columns..rows.min(columns) {
        swap_rows(a, base, first_columns, ld, row, pivots[row] - 1);
    }
    info
}

/// Factor `A = P L U`, returning LAPACK's non-negative `INFO` value.
pub fn dgetrf(
    rows: usize,
    columns: usize,
    a: &mut [f64],
    ld: usize,
    pivots: &mut [usize],
) -> Result<usize, LuError> {
    if ld < rows.max(1) {
        return Err(LuError::InvalidLeadingDimension);
    }
    if a.len() < matrix_len(rows, columns, ld) {
        return Err(LuError::MatrixTooShort);
    }
    let order = rows.min(columns);
    if pivots.len() < order {
        return Err(LuError::PivotTooShort);
    }
    if order == 0 {
        return Ok(0);
    }
    if BLOCK_SIZE <= 1 || BLOCK_SIZE >= order {
        return Ok(dgetrf2(rows, columns, a, ld, 0, pivots));
    }

    let mut info = 0;
    for start in (0..order).step_by(BLOCK_SIZE) {
        let width = BLOCK_SIZE.min(order - start);
        let panel_base = at(0, start, start, ld);
        let panel_order = (rows - start).min(width);
        let panel_info = dgetrf2(
            rows - start,
            width,
            a,
            ld,
            panel_base,
            &mut pivots[start..start + panel_order],
        );
        if info == 0 && panel_info > 0 {
            info = panel_info + start;
        }
        for pivot in &mut pivots[start..start + panel_order] {
            *pivot += start;
        }

        for row in start..start + width {
            swap_rows(a, 0, start, ld, row, pivots[row] - 1);
        }
        if start + width >= columns {
            continue;
        }
        let right_base = (start + width) * ld;
        for row in start..start + width {
            swap_rows(
                a,
                right_base,
                columns - start - width,
                ld,
                row,
                pivots[row] - 1,
            );
        }

        for column in start + width..columns {
            for row in start..start + width {
                let mut value = a[at(0, row, column, ld)];
                for inner in start..row {
                    value -= a[at(0, row, inner, ld)] * a[at(0, inner, column, ld)];
                }
                a[at(0, row, column, ld)] = value;
            }
        }
        if start + width < rows {
            for column in start + width..columns {
                for row in start + width..rows {
                    let mut value = a[at(0, row, column, ld)];
                    for inner in start..start + width {
                        value -= a[at(0, row, inner, ld)] * a[at(0, inner, column, ld)];
                    }
                    a[at(0, row, column, ld)] = value;
                }
            }
        }
    }
    Ok(info)
}

/// Solve `A X = B` or `A^T X = B` from `DGETRF` factors.
pub fn dgetrs(
    transpose: Transpose,
    order: usize,
    nrhs: usize,
    lu: &[f64],
    ld: usize,
    pivots: &[usize],
    b: &mut [f64],
    ldb: usize,
) -> Result<(), LuError> {
    if ld < order.max(1) || ldb < order.max(1) {
        return Err(LuError::InvalidLeadingDimension);
    }
    if lu.len() < matrix_len(order, order, ld) {
        return Err(LuError::MatrixTooShort);
    }
    if pivots.len() < order {
        return Err(LuError::PivotTooShort);
    }
    if b.len() < matrix_len(order, nrhs, ldb) {
        return Err(LuError::RightHandSideTooShort);
    }

    match transpose {
        Transpose::None => {
            apply_pivots(b, 0, nrhs, ldb, 0, order, pivots, false);
            for column in 0..nrhs {
                for row in 0..order {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in 0..row {
                        value -= lu[at(0, row, inner, ld)] * b[at(0, inner, column, ldb)];
                    }
                    b[at(0, row, column, ldb)] = value;
                }
                for row in (0..order).rev() {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in row + 1..order {
                        value -= lu[at(0, row, inner, ld)] * b[at(0, inner, column, ldb)];
                    }
                    b[at(0, row, column, ldb)] = value / lu[at(0, row, row, ld)];
                }
            }
        }
        Transpose::Transpose => {
            for column in 0..nrhs {
                for row in 0..order {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in 0..row {
                        value -= lu[at(0, inner, row, ld)] * b[at(0, inner, column, ldb)];
                    }
                    b[at(0, row, column, ldb)] = value / lu[at(0, row, row, ld)];
                }
                for row in (0..order).rev() {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in row + 1..order {
                        value -= lu[at(0, inner, row, ld)] * b[at(0, inner, column, ldb)];
                    }
                    b[at(0, row, column, ldb)] = value;
                }
            }
            apply_pivots(b, 0, nrhs, ldb, 0, order, pivots, true);
        }
    }
    Ok(())
}
