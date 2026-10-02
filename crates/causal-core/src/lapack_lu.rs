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
    NonFiniteConditionEstimate,
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
            Self::NonFiniteConditionEstimate => {
                "the condition estimate exceeds the supported finite range"
            }
        })
    }
}

impl std::error::Error for LuError {}

/// DGECON's one-norm estimate using LAPACK 3.12.1 DLACN2's sign/cycling
/// iteration and the existing triangular solve. Row pivots are unnecessary:
/// they do not change the inverse one-norm. This finite-range route does not
/// implement DLATRS's extreme-exponent scaling; overflow is an explicit error.
/// Source: reference/lapack-3.12.1/SRC/{dgecon,dlacn2}.f, LAPACK BSD license.
pub fn reciprocal_condition_one_fused(
    order: usize,
    lu: &[f64],
    ld: usize,
    norm: f64,
) -> Result<f64, LuError> {
    if ld < order.max(1) {
        return Err(LuError::InvalidLeadingDimension);
    }
    if lu.len() < matrix_len(order, order, ld) {
        return Err(LuError::MatrixTooShort);
    }
    if !norm.is_finite() || norm < 0. {
        return Err(LuError::NonFiniteConditionEstimate);
    }
    if order == 0 {
        return Ok(1.);
    }
    if norm == 0. {
        return Ok(0.);
    }
    let identity: Vec<_> = (1..=order).collect();
    let apply = |x: &mut [f64], transpose| -> Result<(), LuError> {
        dgetrs_fused(transpose, order, 1, lu, ld, &identity, x, order)?;
        if x.iter().any(|v| !v.is_finite()) {
            return Err(LuError::NonFiniteConditionEstimate);
        }
        Ok(())
    };
    let asum = |x: &[f64]| x.iter().map(|v| v.abs()).sum::<f64>();
    let max_index = |x: &[f64]| {
        let mut j = 0;
        for i in 1..x.len() {
            if x[i].abs() > x[j].abs() {
                j = i;
            }
        }
        j
    };
    let sign = |v: f64| if v >= 0. { 1. } else { -1. };
    let mut x = vec![1. / order as f64; order];
    apply(&mut x, Transpose::None)?;
    let mut estimate = asum(&x);
    if order > 1 {
        let mut signs: Vec<_> = x.iter().map(|v| sign(*v)).collect();
        x.copy_from_slice(&signs);
        apply(&mut x, Transpose::Transpose)?;
        let mut j = max_index(&x);
        let mut iteration = 2;
        loop {
            x.fill(0.);
            x[j] = 1.;
            apply(&mut x, Transpose::None)?;
            let old = estimate;
            estimate = asum(&x);
            if x.iter().zip(&signs).all(|(v, s)| sign(*v) == *s) || estimate <= old {
                break;
            }
            for i in 0..order {
                signs[i] = sign(x[i]);
                x[i] = signs[i];
            }
            apply(&mut x, Transpose::Transpose)?;
            let last = j;
            j = max_index(&x);
            if x[last] == x[j].abs() || iteration >= 5 {
                break;
            }
            iteration += 1;
        }
        for (i, v) in x.iter_mut().enumerate() {
            *v = if i % 2 == 0 { 1. } else { -1. } * (1. + i as f64 / (order - 1) as f64);
        }
        apply(&mut x, Transpose::None)?;
        estimate = estimate.max(2. * (asum(&x) / (3 * order) as f64));
    }
    if !estimate.is_finite() {
        return Err(LuError::NonFiniteConditionEstimate);
    }
    Ok(if estimate == 0. {
        0.
    } else {
        (1. / estimate) / norm
    })
}

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

/// OpenBLAS 0.3.30 `lapack/getf2/getf2_k.c` small-system factorization.
/// Left-looking column updates differ from recursive reference DGETRF2.
/// Existing DGETRF callers keep their original recipe.
///
/// Copyright 2009, 2010 The University of Texas at Austin.
/// All rights reserved.
///
/// Redistribution and use in source and binary forms, with or without
/// modification, are permitted provided that the following conditions are met:
///
/// 1. Redistributions of source code must retain the above copyright notice,
///    this list of conditions and the following disclaimer.
/// 2. Redistributions in binary form must reproduce the above copyright notice,
///    this list of conditions and the following disclaimer in the documentation
///    and/or other materials provided with the distribution.
///
/// THIS SOFTWARE IS PROVIDED BY THE UNIVERSITY OF TEXAS AT AUSTIN ``AS IS'' AND
/// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
/// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
/// DISCLAIMED. IN NO EVENT SHALL THE UNIVERSITY OF TEXAS AT AUSTIN OR CONTRIBUTORS
/// BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
/// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
/// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
/// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
/// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
/// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
/// POSSIBILITY OF SUCH DAMAGE.
///
/// The views and conclusions contained in the software and documentation are
/// those of the authors and should not be interpreted as representing official
/// policies, either expressed or implied, of The University of Texas at Austin.
pub fn dgetf2_left_looking(
    rows: usize,
    columns: usize,
    a: &mut [f64],
    ld: usize,
    pivots: &mut [usize],
) -> Result<usize, LuError> {
    dgetf2_left_looking_impl(rows, columns, a, ld, pivots, false)
}

/// Ordered fused accumulation used by the pinned ARM OpenBLAS backend.
pub fn dgetf2_left_looking_fused(
    rows: usize,
    columns: usize,
    a: &mut [f64],
    ld: usize,
    pivots: &mut [usize],
) -> Result<usize, LuError> {
    dgetf2_left_looking_impl(rows, columns, a, ld, pivots, true)
}

fn dgetf2_left_looking_impl(
    rows: usize,
    columns: usize,
    a: &mut [f64],
    ld: usize,
    pivots: &mut [usize],
    fused_updates: bool,
) -> Result<usize, LuError> {
    use crate::lapack_dgelsd::blas::{ddot, dgemv, Transpose as BlasTranspose};
    if ld < rows.max(1) {
        return Err(LuError::InvalidLeadingDimension);
    }
    if a.len() < matrix_len(rows, columns, ld) {
        return Err(LuError::MatrixTooShort);
    }
    if pivots.len() < rows.min(columns) {
        return Err(LuError::PivotTooShort);
    }
    if rows == 0 || columns == 0 {
        return Ok(0);
    }
    let mut info = 0;
    for column in 0..columns {
        let len = column.min(rows);
        let mut b = a[column * ld..column * ld + rows].to_vec();
        for i in 0..len {
            b.swap(i, pivots[i] - 1);
        }
        for i in 1..len {
            let dot = if fused_updates {
                (0..i).fold(0., |sum, j| a[i + j * ld].mul_add(b[j], sum))
            } else {
                ddot(i, &a[i..], ld as isize, &b, 1)
            };
            b[i] -= dot;
        }
        if column < rows {
            // Reuse the existing source-translated GEMV rather than another
            // hand-written matrix product. The input and output are disjoint.
            let mut tail = b[column..].to_vec();
            if fused_updates {
                for j in 0..column {
                    for i in column..rows {
                        tail[i - column] = (-b[j]).mul_add(a[i + j * ld], tail[i - column]);
                    }
                }
            } else {
                dgemv(
                    BlasTranspose::None,
                    rows - column,
                    column,
                    -1.,
                    &a[column..],
                    ld,
                    &b,
                    1,
                    1.,
                    &mut tail,
                    1,
                )
                .map_err(|_| LuError::MatrixTooShort)?;
            }
            b[column..].copy_from_slice(&tail);
            let mut pivot = column;
            for i in column + 1..rows {
                if b[i].abs() > b[pivot].abs() {
                    pivot = i;
                }
            }
            pivots[column] = pivot + 1;
            let diagonal = b[pivot];
            if diagonal == 0. {
                if info == 0 {
                    info = column + 1;
                }
            } else if diagonal.abs() >= f64::MIN_POSITIVE {
                if pivot != column {
                    for c in 0..column {
                        a.swap(column + c * ld, pivot + c * ld);
                    }
                    b.swap(column, pivot);
                }
                let inverse = 1. / diagonal;
                for v in &mut b[column + 1..] {
                    *v *= inverse;
                }
            }
        }
        a[column * ld..column * ld + rows].copy_from_slice(&b);
    }
    Ok(info)
}

/// Factor `A = P L U`, returning LAPACK's non-negative `INFO` value.
/// Pinned OpenBLAS NEOVERSEN1 recursive panels, sharing the small GETF2 and
/// packed TRSM arithmetic. Source: OpenBLAS 0.3.30 lapack/getrf/getrf_single.c;
/// the University of Texas notice retained above applies to this translation.
pub fn dgetrf_fused(
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
    let blocking = ((order / 2 + 3) / 4 * 4).min(640);
    if blocking <= 8 {
        return dgetf2_left_looking_fused(rows, columns, a, ld, pivots);
    }
    let mut info = 0;
    for start in (0..order).step_by(blocking) {
        let width = blocking.min(order - start);
        let base = start + start * ld;
        let panel_info = dgetrf_fused(
            rows - start,
            width,
            &mut a[base..],
            ld,
            &mut pivots[start..start + width],
        )?;
        if info == 0 && panel_info > 0 {
            info = start + panel_info;
        }
        for pivot in &mut pivots[start..start + width] {
            *pivot += start;
        }
        let end = start + width;
        if end < columns {
            for row in start..end {
                let pivot = pivots[row] - 1;
                for column in end..columns {
                    a.swap(row + column * ld, pivot + column * ld);
                }
            }
            let lower: Vec<f64> = (0..width)
                .flat_map(|c| (0..width).map(move |r| (r, c)))
                .map(|(r, c)| a[start + r + (start + c) * ld])
                .collect();
            packed_trsm_fused(
                true,
                width,
                columns - end,
                &lower,
                width,
                &mut a[start + end * ld..],
                ld,
            );
            let m = rows - end;
            let n = columns - end;
            if m > 0 {
                let left: Vec<f64> = (0..width)
                    .flat_map(|c| (0..m).map(move |r| (r, c)))
                    .map(|(r, c)| a[end + r + (start + c) * ld])
                    .collect();
                let right: Vec<f64> = (0..n)
                    .flat_map(|c| (0..width).map(move |r| (r, c)))
                    .map(|(r, c)| a[start + r + (end + c) * ld])
                    .collect();
                let mut trailing: Vec<f64> = (0..n)
                    .flat_map(|c| (0..m).map(move |r| (r, c)))
                    .map(|(r, c)| a[end + r + (end + c) * ld])
                    .collect();
                use crate::lapack_dgelsd::blas::{dgemm_fused, Transpose as BlasTranspose};
                dgemm_fused(
                    BlasTranspose::None,
                    BlasTranspose::None,
                    m,
                    n,
                    width,
                    -1.,
                    &left,
                    m,
                    &right,
                    width,
                    1.,
                    &mut trailing,
                    m,
                )
                .expect("validated panel product dimensions");
                for c in 0..n {
                    for r in 0..m {
                        a[end + r + (end + c) * ld] = trailing[r + c * m];
                    }
                }
            }
        }
    }
    // OpenBLAS postpones later-panel permutations of earlier columns.
    for start in (0..order).step_by(blocking) {
        let end = (start + blocking).min(order);
        for row in end..order {
            for column in start..end {
                a.swap(row + column * ld, pivots[row] - 1 + column * ld);
            }
        }
    }
    Ok(info)
}

// NEOVERSEN1's eight-row packed solve, reused by GETRF and GETRS.
fn packed_trsm_fused(
    lower_unit: bool,
    order: usize,
    nrhs: usize,
    lu: &[f64],
    ld: usize,
    b: &mut [f64],
    ldb: usize,
) {
    let mut blocks = Vec::new();
    let mut first = 0;
    while order - first >= 8 {
        blocks.push((first, first + 8));
        first += 8;
    }
    for width in [4, 2, 1] {
        if order - first >= width {
            blocks.push((first, first + width));
            first += width;
        }
    }
    for column in 0..nrhs {
        let traversal: Vec<(usize, usize)> = if lower_unit {
            blocks.clone()
        } else {
            blocks.iter().rev().copied().collect()
        };
        for (start, end) in traversal {
            for row in start..end {
                let mut sum = 0.;
                let range = if lower_unit { 0..start } else { end..order };
                for inner in range {
                    sum = lu[row + inner * ld].mul_add(b[inner + column * ldb], sum);
                }
                b[row + column * ldb] -= sum;
            }
            if lower_unit {
                for row in start..end {
                    let value = b[row + column * ldb];
                    for target in row + 1..end {
                        b[target + column * ldb] =
                            (-value).mul_add(lu[target + row * ld], b[target + column * ldb]);
                    }
                }
            } else {
                for row in (start..end).rev() {
                    b[row + column * ldb] *= 1. / lu[row + row * ld];
                    let value = b[row + column * ldb];
                    for target in start..row {
                        b[target + column * ldb] =
                            (-value).mul_add(lu[target + row * ld], b[target + column * ldb]);
                    }
                }
            }
        }
    }
}

/// Reference LAPACK factorization. Existing callers retain their recipe.
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
    dgetrs_impl(transpose, order, nrhs, lu, ld, pivots, b, ldb, false)
}

/// Same DGETRS control flow with fused multiply-subtract updates, matching
/// the compiled ARM Fortran recipe in the pinned R Synth oracle. Other
/// numerical ports retain DGETRS's existing separate-rounding behaviour.
pub fn dgetrs_fused(
    transpose: Transpose,
    order: usize,
    nrhs: usize,
    lu: &[f64],
    ld: usize,
    pivots: &[usize],
    b: &mut [f64],
    ldb: usize,
) -> Result<(), LuError> {
    dgetrs_impl(transpose, order, nrhs, lu, ld, pivots, b, ldb, true)
}

fn dgetrs_impl(
    transpose: Transpose,
    order: usize,
    nrhs: usize,
    lu: &[f64],
    ld: usize,
    pivots: &[usize],
    b: &mut [f64],
    ldb: usize,
    fused_updates: bool,
) -> Result<(), LuError> {
    let subtract_product = |value: f64, left: f64, right: f64| {
        if fused_updates {
            (-left).mul_add(right, value)
        } else {
            value - left * right
        }
    };
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
            if fused_updates && nrhs > 1 {
                // Pinned NEOVERSEN1 packed TRSM: eight-row tiles followed by
                // four/two/one-row remainders. Between tiles the GEMM kernel
                // accumulates a dot product before subtracting it; within a
                // tile the generic TRSM kernel performs ordered fused updates.
                // Source: OpenBLAS 0.3.30 param.h, trsm_kernel_LT.c/LN.c.
                packed_trsm_fused(true, order, nrhs, lu, ld, b, ldb);
                packed_trsm_fused(false, order, nrhs, lu, ld, b, ldb);
                return Ok(());
            }
            for column in 0..nrhs {
                for row in 0..order {
                    let value = b[at(0, row, column, ldb)];
                    if value != 0.0 {
                        for target in row + 1..order {
                            b[at(0, target, column, ldb)] = subtract_product(
                                b[at(0, target, column, ldb)],
                                value,
                                lu[at(0, target, row, ld)],
                            );
                        }
                    }
                }
                // Reference BLAS DTRSM('L','U','N','N') updates columns in
                // descending order. An ascending row-wise dot product is
                // algebraically equivalent but has different rounding.
                for row in (0..order).rev() {
                    if b[at(0, row, column, ldb)] != 0.0 {
                        b[at(0, row, column, ldb)] /= lu[at(0, row, row, ld)];
                        let value = b[at(0, row, column, ldb)];
                        for target in 0..row {
                            b[at(0, target, column, ldb)] = subtract_product(
                                b[at(0, target, column, ldb)],
                                value,
                                lu[at(0, target, row, ld)],
                            );
                        }
                    }
                }
            }
        }
        Transpose::Transpose => {
            for column in 0..nrhs {
                for row in 0..order {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in 0..row {
                        value = subtract_product(
                            value,
                            lu[at(0, inner, row, ld)],
                            b[at(0, inner, column, ldb)],
                        );
                    }
                    b[at(0, row, column, ldb)] = value / lu[at(0, row, row, ld)];
                }
                for row in (0..order).rev() {
                    let mut value = b[at(0, row, column, ldb)];
                    for inner in row + 1..order {
                        value = subtract_product(
                            value,
                            lu[at(0, inner, row, ld)],
                            b[at(0, inner, column, ldb)],
                        );
                    }
                    b[at(0, row, column, ldb)] = value;
                }
            }
            apply_pivots(b, 0, nrhs, ldb, 0, order, pivots, true);
        }
    }
    Ok(())
}
