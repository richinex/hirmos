//! Column-major translation of LAPACK 3.12.1 `DPOTRF`, `DPOTRF2`, and `DPOTRS`.

use core::fmt;

const BLOCK_SIZE: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Triangle {
    Upper,
    Lower,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CholeskyError {
    InvalidLeadingDimension,
    MatrixTooShort,
    RightHandSideTooShort,
}

impl fmt::Display for CholeskyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLeadingDimension => {
                "the leading dimension is smaller than the matrix order"
            }
            Self::MatrixTooShort => "the Cholesky matrix buffer is too short",
            Self::RightHandSideTooShort => "the right-hand-side buffer is too short",
        })
    }
}

impl std::error::Error for CholeskyError {}

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

fn dpotrf2(triangle: Triangle, order: usize, a: &mut [f64], ld: usize, base: usize) -> usize {
    if order == 0 {
        return 0;
    }
    if order == 1 {
        let diagonal = a[base];
        if diagonal <= 0.0 || diagonal.is_nan() {
            return 1;
        }
        a[base] = diagonal.sqrt();
        return 0;
    }

    let first = order / 2;
    let second = order - first;
    let info = dpotrf2(triangle, first, a, ld, base);
    if info != 0 {
        return info;
    }

    match triangle {
        Triangle::Upper => {
            let right = base + first * ld;
            for column in 0..second {
                for row in 0..first {
                    let mut value = a[at(right, row, column, ld)];
                    for inner in 0..row {
                        value -= a[at(base, inner, row, ld)] * a[at(right, inner, column, ld)];
                    }
                    a[at(right, row, column, ld)] = value / a[at(base, row, row, ld)];
                }
            }
            let trailing = at(base, first, first, ld);
            for column in 0..second {
                for row in 0..=column {
                    let mut value = a[at(trailing, row, column, ld)];
                    for inner in 0..first {
                        value -= a[at(right, inner, row, ld)] * a[at(right, inner, column, ld)];
                    }
                    a[at(trailing, row, column, ld)] = value;
                }
            }
            let info = dpotrf2(triangle, second, a, ld, trailing);
            if info == 0 {
                0
            } else {
                info + first
            }
        }
        Triangle::Lower => {
            let lower = base + first;
            for row in 0..second {
                for column in 0..first {
                    let mut value = a[at(lower, row, column, ld)];
                    for inner in 0..column {
                        value -= a[at(lower, row, inner, ld)] * a[at(base, column, inner, ld)];
                    }
                    a[at(lower, row, column, ld)] = value / a[at(base, column, column, ld)];
                }
            }
            let trailing = at(base, first, first, ld);
            for column in 0..second {
                for row in column..second {
                    let mut value = a[at(trailing, row, column, ld)];
                    for inner in 0..first {
                        value -= a[at(lower, row, inner, ld)] * a[at(lower, column, inner, ld)];
                    }
                    a[at(trailing, row, column, ld)] = value;
                }
            }
            let info = dpotrf2(triangle, second, a, ld, trailing);
            if info == 0 {
                0
            } else {
                info + first
            }
        }
    }
}

/// Factor a symmetric positive-definite matrix, returning LAPACK's non-negative `INFO`.
pub fn dpotrf(
    triangle: Triangle,
    order: usize,
    a: &mut [f64],
    ld: usize,
) -> Result<usize, CholeskyError> {
    if ld < order.max(1) {
        return Err(CholeskyError::InvalidLeadingDimension);
    }
    if a.len() < matrix_len(order, order, ld) {
        return Err(CholeskyError::MatrixTooShort);
    }
    if order == 0 {
        return Ok(0);
    }
    if BLOCK_SIZE <= 1 || BLOCK_SIZE >= order {
        return Ok(dpotrf2(triangle, order, a, ld, 0));
    }

    for start in (0..order).step_by(BLOCK_SIZE) {
        let width = BLOCK_SIZE.min(order - start);
        match triangle {
            Triangle::Upper => {
                // DSYRK('U','T'): update the current diagonal block.
                for column in start..start + width {
                    for row in start..=column {
                        let mut value = a[at(0, row, column, ld)];
                        for inner in 0..start {
                            value -= a[at(0, inner, row, ld)] * a[at(0, inner, column, ld)];
                        }
                        a[at(0, row, column, ld)] = value;
                    }
                }
                let info = dpotrf2(triangle, width, a, ld, at(0, start, start, ld));
                if info != 0 {
                    return Ok(info + start);
                }

                // DGEMM followed by DTRSM computes the current block row.
                for column in start + width..order {
                    for row in start..start + width {
                        let mut value = a[at(0, row, column, ld)];
                        for inner in 0..start {
                            value -= a[at(0, inner, row, ld)] * a[at(0, inner, column, ld)];
                        }
                        for inner in start..row {
                            value -= a[at(0, inner, row, ld)] * a[at(0, inner, column, ld)];
                        }
                        a[at(0, row, column, ld)] = value / a[at(0, row, row, ld)];
                    }
                }
            }
            Triangle::Lower => {
                // DSYRK('L','N'): update the current diagonal block.
                for column in start..start + width {
                    for row in column..start + width {
                        let mut value = a[at(0, row, column, ld)];
                        for inner in 0..start {
                            value -= a[at(0, row, inner, ld)] * a[at(0, column, inner, ld)];
                        }
                        a[at(0, row, column, ld)] = value;
                    }
                }
                let info = dpotrf2(triangle, width, a, ld, at(0, start, start, ld));
                if info != 0 {
                    return Ok(info + start);
                }

                // DGEMM followed by DTRSM computes the current block column.
                for column in start..start + width {
                    for row in start + width..order {
                        let mut value = a[at(0, row, column, ld)];
                        for inner in 0..start {
                            value -= a[at(0, row, inner, ld)] * a[at(0, column, inner, ld)];
                        }
                        for inner in start..column {
                            value -= a[at(0, row, inner, ld)] * a[at(0, column, inner, ld)];
                        }
                        a[at(0, row, column, ld)] = value / a[at(0, column, column, ld)];
                    }
                }
            }
        }
    }
    Ok(0)
}

/// Solve a positive-definite system from `DPOTRF` factors.
pub fn dpotrs(
    triangle: Triangle,
    order: usize,
    nrhs: usize,
    factor: &[f64],
    ld: usize,
    b: &mut [f64],
    ldb: usize,
) -> Result<(), CholeskyError> {
    if ld < order.max(1) || ldb < order.max(1) {
        return Err(CholeskyError::InvalidLeadingDimension);
    }
    if factor.len() < matrix_len(order, order, ld) {
        return Err(CholeskyError::MatrixTooShort);
    }
    if b.len() < matrix_len(order, nrhs, ldb) {
        return Err(CholeskyError::RightHandSideTooShort);
    }

    for column in 0..nrhs {
        match triangle {
            Triangle::Upper => {
                for row in 0..order {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in 0..row {
                        value -= factor[at(0, inner, row, ld)] * b[at(0, inner, column, ldb)];
                    }
                    b[at(0, row, column, ldb)] = value / factor[at(0, row, row, ld)];
                }
                for row in (0..order).rev() {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in row + 1..order {
                        value -= factor[at(0, row, inner, ld)] * b[at(0, inner, column, ldb)];
                    }
                    b[at(0, row, column, ldb)] = value / factor[at(0, row, row, ld)];
                }
            }
            Triangle::Lower => {
                for row in 0..order {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in 0..row {
                        value -= factor[at(0, row, inner, ld)] * b[at(0, inner, column, ldb)];
                    }
                    b[at(0, row, column, ldb)] = value / factor[at(0, row, row, ld)];
                }
                for row in (0..order).rev() {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in row + 1..order {
                        value -= factor[at(0, inner, row, ld)] * b[at(0, inner, column, ldb)];
                    }
                    b[at(0, row, column, ldb)] = value / factor[at(0, row, row, ld)];
                }
            }
        }
    }
    Ok(())
}
