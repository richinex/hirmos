//! Reference double-precision BLAS and small LAPACK helpers used by DGELSD.
//!
//! The executable structure in this file follows LAPACK 3.12.1's reference
//! Fortran sources under `reference/lapack-3.12.1`.  Matrices are flat,
//! column-major arrays and leading dimensions have their Fortran meaning.
//! CLAPACK 3.2.1 was consulted only to check mechanical index conversion.

use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Transpose {
    None,
    Transpose,
}

impl Transpose {
    pub(crate) fn from_char(value: char) -> Option<Self> {
        if lsame(value, 'N') {
            Some(Self::None)
        } else if lsame(value, 'T') || lsame(value, 'C') {
            Some(Self::Transpose)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Side {
    Left,
    Right,
}

impl Side {
    pub(crate) fn from_char(value: char) -> Option<Self> {
        if lsame(value, 'L') {
            Some(Self::Left)
        } else if lsame(value, 'R') {
            Some(Self::Right)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Uplo {
    Upper,
    Lower,
}

impl Uplo {
    pub(crate) fn from_char(value: char) -> Option<Self> {
        if lsame(value, 'U') {
            Some(Self::Upper)
        } else if lsame(value, 'L') {
            Some(Self::Lower)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Diag {
    Unit,
    NonUnit,
}

impl Diag {
    pub(crate) fn from_char(value: char) -> Option<Self> {
        if lsame(value, 'U') {
            Some(Self::Unit)
        } else if lsame(value, 'N') {
            Some(Self::NonUnit)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BlasError {
    InvalidArgument {
        routine: &'static str,
        argument: usize,
    },
    BufferTooShort {
        routine: &'static str,
        buffer: &'static str,
        required: usize,
        actual: usize,
    },
}

impl fmt::Display for BlasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidArgument { routine, argument } => {
                write!(f, "{routine}: invalid argument {argument}")
            }
            Self::BufferTooShort {
                routine,
                buffer,
                required,
                actual,
            } => write!(
                f,
                "{routine}: {buffer} needs {required} elements, received {actual}"
            ),
        }
    }
}

impl std::error::Error for BlasError {}

#[inline]
fn matrix_index(row: usize, column: usize, leading_dimension: usize) -> usize {
    row + column * leading_dimension
}

#[inline]
fn vector_start(n: usize, increment: isize) -> isize {
    if increment < 0 {
        (n.saturating_sub(1) as isize) * -increment
    } else {
        0
    }
}

#[inline]
fn vector_required_len(n: usize, increment: isize) -> usize {
    if n == 0 {
        0
    } else {
        1 + (n - 1) * increment.unsigned_abs()
    }
}

#[inline]
fn matrix_required_len(rows: usize, columns: usize, leading_dimension: usize) -> usize {
    if rows == 0 || columns == 0 {
        0
    } else {
        (columns - 1) * leading_dimension + rows
    }
}

#[inline]
fn require_buffer(
    routine: &'static str,
    buffer: &'static str,
    actual: usize,
    required: usize,
) -> Result<(), BlasError> {
    if actual < required {
        Err(BlasError::BufferTooShort {
            routine,
            buffer,
            required,
            actual,
        })
    } else {
        Ok(())
    }
}

/// Case-insensitive single-character comparison (`LSAME`).
pub(crate) fn lsame(ca: char, cb: char) -> bool {
    ca == cb || ca.to_ascii_uppercase() == cb.to_ascii_uppercase()
}

/// `DAXPY`: `y <- da*x + y`.
pub(crate) fn daxpy(n: usize, da: f64, dx: &[f64], incx: isize, dy: &mut [f64], incy: isize) {
    if n == 0 || da == 0.0 {
        return;
    }
    assert!(dx.len() >= vector_required_len(n, incx));
    assert!(dy.len() >= vector_required_len(n, incy));

    if incx == 1 && incy == 1 {
        let m = n % 4;
        for i in 0..m {
            dy[i] += da * dx[i];
        }
        if n < 4 {
            return;
        }
        for i in (m..n).step_by(4) {
            dy[i] += da * dx[i];
            dy[i + 1] += da * dx[i + 1];
            dy[i + 2] += da * dx[i + 2];
            dy[i + 3] += da * dx[i + 3];
        }
    } else {
        let mut ix = vector_start(n, incx);
        let mut iy = vector_start(n, incy);
        for _ in 0..n {
            dy[iy as usize] += da * dx[ix as usize];
            ix += incx;
            iy += incy;
        }
    }
}

/// `DCOPY`: copy the incremented vector `dx` into `dy`.
pub(crate) fn dcopy(n: usize, dx: &[f64], incx: isize, dy: &mut [f64], incy: isize) {
    if n == 0 {
        return;
    }
    assert!(dx.len() >= vector_required_len(n, incx));
    assert!(dy.len() >= vector_required_len(n, incy));

    if incx == 1 && incy == 1 {
        let m = n % 7;
        for i in 0..m {
            dy[i] = dx[i];
        }
        if n < 7 {
            return;
        }
        for i in (m..n).step_by(7) {
            dy[i] = dx[i];
            dy[i + 1] = dx[i + 1];
            dy[i + 2] = dx[i + 2];
            dy[i + 3] = dx[i + 3];
            dy[i + 4] = dx[i + 4];
            dy[i + 5] = dx[i + 5];
            dy[i + 6] = dx[i + 6];
        }
    } else {
        let mut ix = vector_start(n, incx);
        let mut iy = vector_start(n, incy);
        for _ in 0..n {
            dy[iy as usize] = dx[ix as usize];
            ix += incx;
            iy += incy;
        }
    }
}

/// `DDOT`: dot product of two incremented vectors.
pub(crate) fn ddot(n: usize, dx: &[f64], incx: isize, dy: &[f64], incy: isize) -> f64 {
    let mut dtemp = 0.0;
    if n == 0 {
        return dtemp;
    }
    assert!(dx.len() >= vector_required_len(n, incx));
    assert!(dy.len() >= vector_required_len(n, incy));

    if incx == 1 && incy == 1 {
        let m = n % 5;
        for i in 0..m {
            dtemp += dx[i] * dy[i];
        }
        if n < 5 {
            return dtemp;
        }
        for i in (m..n).step_by(5) {
            dtemp = dtemp
                + dx[i] * dy[i]
                + dx[i + 1] * dy[i + 1]
                + dx[i + 2] * dy[i + 2]
                + dx[i + 3] * dy[i + 3]
                + dx[i + 4] * dy[i + 4];
        }
    } else {
        let mut ix = vector_start(n, incx);
        let mut iy = vector_start(n, incy);
        for _ in 0..n {
            dtemp += dx[ix as usize] * dy[iy as usize];
            ix += incx;
            iy += incy;
        }
    }
    dtemp
}

// Blue scaling constants shared by the current DNRM2 and DLASSQ translations.
const TSML: f64 = 1.491_668_146_240_041_3e-154; // 2**-511
const TBIG: f64 = 1.997_919_072_202_235e146; // 2**486
const SSML: f64 = 4.498_913_794_543_196_4e161; // 2**537
const SBIG: f64 = 1.111_379_374_742_538_7e-162; // 2**-538

/// `DNRM2`: safely scaled Euclidean norm from reference BLAS 3.12.1.
pub(crate) fn dnrm2(n: usize, x: &[f64], incx: isize) -> f64 {
    if n == 0 {
        return 0.0;
    }
    assert!(x.len() >= vector_required_len(n, incx));

    let mut notbig = true;
    let mut asml = 0.0;
    let mut amed = 0.0;
    let mut abig = 0.0;
    let mut ix = vector_start(n, incx);
    for _ in 0..n {
        let ax = x[ix as usize].abs();
        if ax > TBIG {
            let scaled = ax * SBIG;
            abig += scaled * scaled;
            notbig = false;
        } else if ax < TSML {
            if notbig {
                let scaled = ax * SSML;
                asml += scaled * scaled;
            }
        } else {
            amed += ax * ax;
        }
        ix += incx;
    }

    let (scale, sumsq);
    if abig > 0.0 {
        if amed > 0.0 || amed > f64::MAX || amed.is_nan() {
            abig += (amed * SBIG) * SBIG;
        }
        scale = 1.0 / SBIG;
        sumsq = abig;
    } else if asml > 0.0 {
        if amed > 0.0 || amed > f64::MAX || amed.is_nan() {
            amed = amed.sqrt();
            asml = asml.sqrt() / SSML;
            let (ymin, ymax) = if asml > amed {
                (amed, asml)
            } else {
                (asml, amed)
            };
            scale = 1.0;
            let ratio = ymin / ymax;
            sumsq = (ymax * ymax) * (1.0 + ratio * ratio);
        } else {
            scale = 1.0 / SSML;
            sumsq = asml;
        }
    } else {
        scale = 1.0;
        sumsq = amed;
    }
    scale * sumsq.sqrt()
}

/// `DROT`: apply a real plane rotation.
pub(crate) fn drot(
    n: usize,
    dx: &mut [f64],
    incx: isize,
    dy: &mut [f64],
    incy: isize,
    c: f64,
    s: f64,
) {
    if n == 0 {
        return;
    }
    assert!(dx.len() >= vector_required_len(n, incx));
    assert!(dy.len() >= vector_required_len(n, incy));

    let mut ix = if incx == 1 && incy == 1 {
        0
    } else {
        vector_start(n, incx)
    };
    let mut iy = if incx == 1 && incy == 1 {
        0
    } else {
        vector_start(n, incy)
    };
    for _ in 0..n {
        let xold = dx[ix as usize];
        let dtemp = c * xold + s * dy[iy as usize];
        dy[iy as usize] = c * dy[iy as usize] - s * xold;
        dx[ix as usize] = dtemp;
        ix += incx;
        iy += incy;
    }
}

/// `DSCAL`: scale an incremented vector. As in reference BLAS, non-positive
/// increments are a quick return.
pub(crate) fn dscal(n: usize, da: f64, dx: &mut [f64], incx: isize) {
    if n == 0 || incx <= 0 || da == 1.0 {
        return;
    }
    assert!(dx.len() >= vector_required_len(n, incx));

    if incx == 1 {
        let m = n % 5;
        for value in &mut dx[..m] {
            *value = da * *value;
        }
        if n < 5 {
            return;
        }
        for i in (m..n).step_by(5) {
            dx[i] = da * dx[i];
            dx[i + 1] = da * dx[i + 1];
            dx[i + 2] = da * dx[i + 2];
            dx[i + 3] = da * dx[i + 3];
            dx[i + 4] = da * dx[i + 4];
        }
    } else {
        let mut i = 0;
        for _ in 0..n {
            dx[i] = da * dx[i];
            i += incx as usize;
        }
    }
}

/// `DSWAP`: interchange two incremented vectors.
pub(crate) fn dswap(n: usize, dx: &mut [f64], incx: isize, dy: &mut [f64], incy: isize) {
    if n == 0 {
        return;
    }
    assert!(dx.len() >= vector_required_len(n, incx));
    assert!(dy.len() >= vector_required_len(n, incy));

    if incx == 1 && incy == 1 {
        let m = n % 3;
        for i in 0..m {
            core::mem::swap(&mut dx[i], &mut dy[i]);
        }
        if n < 3 {
            return;
        }
        for i in (m..n).step_by(3) {
            core::mem::swap(&mut dx[i], &mut dy[i]);
            core::mem::swap(&mut dx[i + 1], &mut dy[i + 1]);
            core::mem::swap(&mut dx[i + 2], &mut dy[i + 2]);
        }
    } else {
        let mut ix = vector_start(n, incx);
        let mut iy = vector_start(n, incy);
        for _ in 0..n {
            core::mem::swap(&mut dx[ix as usize], &mut dy[iy as usize]);
            ix += incx;
            iy += incy;
        }
    }
}

/// `IDAMAX`: one-based index in the logical vector of the first maximum.
/// Returns the reference BLAS sentinel zero for an invalid vector.
pub(crate) fn idamax(n: usize, dx: &[f64], incx: isize) -> usize {
    if n < 1 || incx <= 0 {
        return 0;
    }
    assert!(dx.len() >= vector_required_len(n, incx));
    if n == 1 {
        return 1;
    }

    let mut result = 0;
    let mut dmax = dx[0].abs();
    if incx == 1 {
        for (i, value) in dx.iter().take(n).enumerate().skip(1) {
            if value.abs() > dmax {
                result = i;
                dmax = value.abs();
            }
        }
    } else {
        let mut ix = incx as usize;
        for i in 1..n {
            if dx[ix].abs() > dmax {
                result = i;
                dmax = dx[ix].abs();
            }
            ix += incx as usize;
        }
    }
    result + 1
}

/// Rust-native zero-based form of [`idamax`].
pub(crate) fn idamax_index(n: usize, dx: &[f64], incx: isize) -> Option<usize> {
    idamax(n, dx, incx).checked_sub(1)
}

/// `DGEMV`: `y <- alpha*op(A)*x + beta*y`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dgemv(
    trans: Transpose,
    m: usize,
    n: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    x: &[f64],
    incx: isize,
    beta: f64,
    y: &mut [f64],
    incy: isize,
) -> Result<(), BlasError> {
    if lda < m.max(1) {
        return Err(BlasError::InvalidArgument {
            routine: "DGEMV",
            argument: 6,
        });
    }
    if incx == 0 {
        return Err(BlasError::InvalidArgument {
            routine: "DGEMV",
            argument: 8,
        });
    }
    if incy == 0 {
        return Err(BlasError::InvalidArgument {
            routine: "DGEMV",
            argument: 11,
        });
    }
    if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
        return Ok(());
    }

    let (lenx, leny) = match trans {
        Transpose::None => (n, m),
        Transpose::Transpose => (m, n),
    };
    require_buffer("DGEMV", "Y", y.len(), vector_required_len(leny, incy))?;
    if alpha != 0.0 {
        require_buffer("DGEMV", "A", a.len(), matrix_required_len(m, n, lda))?;
        require_buffer("DGEMV", "X", x.len(), vector_required_len(lenx, incx))?;
    }

    let kx = vector_start(lenx, incx);
    let ky = vector_start(leny, incy);
    if beta != 1.0 {
        let mut iy = ky;
        for _ in 0..leny {
            if beta == 0.0 {
                y[iy as usize] = 0.0;
            } else {
                y[iy as usize] = beta * y[iy as usize];
            }
            iy += incy;
        }
    }
    if alpha == 0.0 {
        return Ok(());
    }

    match trans {
        Transpose::None => {
            let mut jx = kx;
            if incy == 1 {
                for j in 0..n {
                    let temp = alpha * x[jx as usize];
                    for i in 0..m {
                        y[i] += temp * a[matrix_index(i, j, lda)];
                    }
                    jx += incx;
                }
            } else {
                for j in 0..n {
                    let temp = alpha * x[jx as usize];
                    let mut iy = ky;
                    for i in 0..m {
                        y[iy as usize] += temp * a[matrix_index(i, j, lda)];
                        iy += incy;
                    }
                    jx += incx;
                }
            }
        }
        Transpose::Transpose => {
            let mut jy = ky;
            if incx == 1 {
                for j in 0..n {
                    let mut temp = 0.0;
                    for i in 0..m {
                        temp += a[matrix_index(i, j, lda)] * x[i];
                    }
                    y[jy as usize] += alpha * temp;
                    jy += incy;
                }
            } else {
                for j in 0..n {
                    let mut temp = 0.0;
                    let mut ix = kx;
                    for i in 0..m {
                        temp += a[matrix_index(i, j, lda)] * x[ix as usize];
                        ix += incx;
                    }
                    y[jy as usize] += alpha * temp;
                    jy += incy;
                }
            }
        }
    }
    Ok(())
}

/// `DGER`: `A <- alpha*x*y**T + A`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dger(
    m: usize,
    n: usize,
    alpha: f64,
    x: &[f64],
    incx: isize,
    y: &[f64],
    incy: isize,
    a: &mut [f64],
    lda: usize,
) -> Result<(), BlasError> {
    if incx == 0 {
        return Err(BlasError::InvalidArgument {
            routine: "DGER",
            argument: 5,
        });
    }
    if incy == 0 {
        return Err(BlasError::InvalidArgument {
            routine: "DGER",
            argument: 7,
        });
    }
    if lda < m.max(1) {
        return Err(BlasError::InvalidArgument {
            routine: "DGER",
            argument: 9,
        });
    }
    if m == 0 || n == 0 || alpha == 0.0 {
        return Ok(());
    }
    require_buffer("DGER", "X", x.len(), vector_required_len(m, incx))?;
    require_buffer("DGER", "Y", y.len(), vector_required_len(n, incy))?;
    require_buffer("DGER", "A", a.len(), matrix_required_len(m, n, lda))?;

    let mut jy = vector_start(n, incy);
    if incx == 1 {
        for j in 0..n {
            if y[jy as usize] != 0.0 {
                let temp = alpha * y[jy as usize];
                for i in 0..m {
                    let ai = matrix_index(i, j, lda);
                    a[ai] += x[i] * temp;
                }
            }
            jy += incy;
        }
    } else {
        let kx = vector_start(m, incx);
        for j in 0..n {
            if y[jy as usize] != 0.0 {
                let temp = alpha * y[jy as usize];
                let mut ix = kx;
                for i in 0..m {
                    let ai = matrix_index(i, j, lda);
                    a[ai] += x[ix as usize] * temp;
                    ix += incx;
                }
            }
            jy += incy;
        }
    }
    Ok(())
}

/// `DGEMM`: `C <- alpha*op(A)*op(B) + beta*C`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dgemm(
    transa: Transpose,
    transb: Transpose,
    m: usize,
    n: usize,
    k: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    beta: f64,
    c: &mut [f64],
    ldc: usize,
) -> Result<(), BlasError> {
    let nota = transa == Transpose::None;
    let notb = transb == Transpose::None;
    let nrowa = if nota { m } else { k };
    let nrowb = if notb { k } else { n };
    if lda < nrowa.max(1) {
        return Err(BlasError::InvalidArgument {
            routine: "DGEMM",
            argument: 8,
        });
    }
    if ldb < nrowb.max(1) {
        return Err(BlasError::InvalidArgument {
            routine: "DGEMM",
            argument: 10,
        });
    }
    if ldc < m.max(1) {
        return Err(BlasError::InvalidArgument {
            routine: "DGEMM",
            argument: 13,
        });
    }
    if m == 0 || n == 0 || ((alpha == 0.0 || k == 0) && beta == 1.0) {
        return Ok(());
    }

    let acols = if nota { k } else { m };
    let bcols = if notb { n } else { k };
    if alpha != 0.0 && k != 0 {
        require_buffer(
            "DGEMM",
            "A",
            a.len(),
            matrix_required_len(nrowa, acols, lda),
        )?;
        require_buffer(
            "DGEMM",
            "B",
            b.len(),
            matrix_required_len(nrowb, bcols, ldb),
        )?;
    }
    require_buffer("DGEMM", "C", c.len(), matrix_required_len(m, n, ldc))?;

    if alpha == 0.0 {
        for j in 0..n {
            for i in 0..m {
                let ci = matrix_index(i, j, ldc);
                if beta == 0.0 {
                    c[ci] = 0.0;
                } else {
                    c[ci] = beta * c[ci];
                }
            }
        }
        return Ok(());
    }

    match (transa, transb) {
        (Transpose::None, Transpose::None) => {
            for j in 0..n {
                for i in 0..m {
                    let ci = matrix_index(i, j, ldc);
                    if beta == 0.0 {
                        c[ci] = 0.0;
                    } else if beta != 1.0 {
                        c[ci] = beta * c[ci];
                    }
                }
                for l in 0..k {
                    let temp = alpha * b[matrix_index(l, j, ldb)];
                    for i in 0..m {
                        let ci = matrix_index(i, j, ldc);
                        c[ci] += temp * a[matrix_index(i, l, lda)];
                    }
                }
            }
        }
        (Transpose::Transpose, Transpose::None) => {
            for j in 0..n {
                for i in 0..m {
                    let mut temp = 0.0;
                    for l in 0..k {
                        temp += a[matrix_index(l, i, lda)] * b[matrix_index(l, j, ldb)];
                    }
                    let ci = matrix_index(i, j, ldc);
                    c[ci] = if beta == 0.0 {
                        alpha * temp
                    } else {
                        alpha * temp + beta * c[ci]
                    };
                }
            }
        }
        (Transpose::None, Transpose::Transpose) => {
            for j in 0..n {
                for i in 0..m {
                    let ci = matrix_index(i, j, ldc);
                    if beta == 0.0 {
                        c[ci] = 0.0;
                    } else if beta != 1.0 {
                        c[ci] = beta * c[ci];
                    }
                }
                for l in 0..k {
                    let temp = alpha * b[matrix_index(j, l, ldb)];
                    for i in 0..m {
                        let ci = matrix_index(i, j, ldc);
                        c[ci] += temp * a[matrix_index(i, l, lda)];
                    }
                }
            }
        }
        (Transpose::Transpose, Transpose::Transpose) => {
            for j in 0..n {
                for i in 0..m {
                    let mut temp = 0.0;
                    for l in 0..k {
                        temp += a[matrix_index(l, i, lda)] * b[matrix_index(j, l, ldb)];
                    }
                    let ci = matrix_index(i, j, ldc);
                    c[ci] = if beta == 0.0 {
                        alpha * temp
                    } else {
                        alpha * temp + beta * c[ci]
                    };
                }
            }
        }
    }
    Ok(())
}

/// `DTRMM`: multiply a matrix in place by a triangular matrix.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dtrmm(
    side: Side,
    uplo: Uplo,
    transa: Transpose,
    diag: Diag,
    m: usize,
    n: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    b: &mut [f64],
    ldb: usize,
) -> Result<(), BlasError> {
    let lside = side == Side::Left;
    let nrowa = if lside { m } else { n };
    let nounit = diag == Diag::NonUnit;
    let upper = uplo == Uplo::Upper;
    if lda < nrowa.max(1) {
        return Err(BlasError::InvalidArgument {
            routine: "DTRMM",
            argument: 9,
        });
    }
    if ldb < m.max(1) {
        return Err(BlasError::InvalidArgument {
            routine: "DTRMM",
            argument: 11,
        });
    }
    if m == 0 || n == 0 {
        return Ok(());
    }
    require_buffer("DTRMM", "B", b.len(), matrix_required_len(m, n, ldb))?;
    if alpha != 0.0 {
        require_buffer(
            "DTRMM",
            "A",
            a.len(),
            matrix_required_len(nrowa, nrowa, lda),
        )?;
    }

    if alpha == 0.0 {
        for j in 0..n {
            for i in 0..m {
                b[matrix_index(i, j, ldb)] = 0.0;
            }
        }
        return Ok(());
    }

    if lside {
        if transa == Transpose::None {
            // B <- alpha*A*B.
            if upper {
                for j in 0..n {
                    for kk in 0..m {
                        let bkj = matrix_index(kk, j, ldb);
                        if b[bkj] != 0.0 {
                            let temp = alpha * b[bkj];
                            for i in 0..kk {
                                let bij = matrix_index(i, j, ldb);
                                b[bij] += temp * a[matrix_index(i, kk, lda)];
                            }
                            if nounit {
                                b[bkj] = temp * a[matrix_index(kk, kk, lda)];
                            } else {
                                b[bkj] = temp;
                            }
                        }
                    }
                }
            } else {
                for j in 0..n {
                    for kk in (0..m).rev() {
                        let bkj = matrix_index(kk, j, ldb);
                        if b[bkj] != 0.0 {
                            let temp = alpha * b[bkj];
                            b[bkj] = temp;
                            if nounit {
                                b[bkj] *= a[matrix_index(kk, kk, lda)];
                            }
                            for i in kk + 1..m {
                                let bij = matrix_index(i, j, ldb);
                                b[bij] += temp * a[matrix_index(i, kk, lda)];
                            }
                        }
                    }
                }
            }
        } else if upper {
            // B <- alpha*A**T*B, A upper.
            for j in 0..n {
                for i in (0..m).rev() {
                    let mut temp = b[matrix_index(i, j, ldb)];
                    if nounit {
                        temp *= a[matrix_index(i, i, lda)];
                    }
                    for kk in 0..i {
                        temp += a[matrix_index(kk, i, lda)] * b[matrix_index(kk, j, ldb)];
                    }
                    b[matrix_index(i, j, ldb)] = alpha * temp;
                }
            }
        } else {
            // B <- alpha*A**T*B, A lower.
            for j in 0..n {
                for i in 0..m {
                    let mut temp = b[matrix_index(i, j, ldb)];
                    if nounit {
                        temp *= a[matrix_index(i, i, lda)];
                    }
                    for kk in i + 1..m {
                        temp += a[matrix_index(kk, i, lda)] * b[matrix_index(kk, j, ldb)];
                    }
                    b[matrix_index(i, j, ldb)] = alpha * temp;
                }
            }
        }
    } else if transa == Transpose::None {
        // B <- alpha*B*A.
        if upper {
            for j in (0..n).rev() {
                let mut temp = alpha;
                if nounit {
                    temp *= a[matrix_index(j, j, lda)];
                }
                for i in 0..m {
                    let bij = matrix_index(i, j, ldb);
                    b[bij] = temp * b[bij];
                }
                for kk in 0..j {
                    let akj = a[matrix_index(kk, j, lda)];
                    if akj != 0.0 {
                        temp = alpha * akj;
                        for i in 0..m {
                            let bij = matrix_index(i, j, ldb);
                            b[bij] += temp * b[matrix_index(i, kk, ldb)];
                        }
                    }
                }
            }
        } else {
            for j in 0..n {
                let mut temp = alpha;
                if nounit {
                    temp *= a[matrix_index(j, j, lda)];
                }
                for i in 0..m {
                    let bij = matrix_index(i, j, ldb);
                    b[bij] = temp * b[bij];
                }
                for kk in j + 1..n {
                    let akj = a[matrix_index(kk, j, lda)];
                    if akj != 0.0 {
                        temp = alpha * akj;
                        for i in 0..m {
                            let bij = matrix_index(i, j, ldb);
                            b[bij] += temp * b[matrix_index(i, kk, ldb)];
                        }
                    }
                }
            }
        }
    } else if upper {
        // B <- alpha*B*A**T, A upper.
        for kk in 0..n {
            for j in 0..kk {
                let ajk = a[matrix_index(j, kk, lda)];
                if ajk != 0.0 {
                    let temp = alpha * ajk;
                    for i in 0..m {
                        let bij = matrix_index(i, j, ldb);
                        b[bij] += temp * b[matrix_index(i, kk, ldb)];
                    }
                }
            }
            let mut temp = alpha;
            if nounit {
                temp *= a[matrix_index(kk, kk, lda)];
            }
            if temp != 1.0 {
                for i in 0..m {
                    let bik = matrix_index(i, kk, ldb);
                    b[bik] = temp * b[bik];
                }
            }
        }
    } else {
        // B <- alpha*B*A**T, A lower.
        for kk in (0..n).rev() {
            for j in kk + 1..n {
                let ajk = a[matrix_index(j, kk, lda)];
                if ajk != 0.0 {
                    let temp = alpha * ajk;
                    for i in 0..m {
                        let bij = matrix_index(i, j, ldb);
                        b[bij] += temp * b[matrix_index(i, kk, ldb)];
                    }
                }
            }
            let mut temp = alpha;
            if nounit {
                temp *= a[matrix_index(kk, kk, lda)];
            }
            if temp != 1.0 {
                for i in 0..m {
                    let bik = matrix_index(i, kk, ldb);
                    b[bik] = temp * b[bik];
                }
            }
        }
    }
    Ok(())
}

/// `DLAISNAN`: compare two values for unordered inequality.
#[inline(never)]
pub(crate) fn dlaisnan(din1: f64, din2: f64) -> bool {
    din1 != din2
}

/// `DISNAN`: detect NaN using the reference helper's calling structure.
pub(crate) fn disnan(din: f64) -> bool {
    dlaisnan(din, din)
}

/// Machine parameters returned by `DLAMCH`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MachineParameter {
    Epsilon,
    SafeMinimum,
    Base,
    Precision,
    MantissaDigits,
    Rounding,
    MinimumExponent,
    UnderflowThreshold,
    MaximumExponent,
    OverflowThreshold,
}

impl MachineParameter {
    pub(crate) fn from_char(value: char) -> Option<Self> {
        match value.to_ascii_uppercase() {
            'E' => Some(Self::Epsilon),
            'S' => Some(Self::SafeMinimum),
            'B' => Some(Self::Base),
            'P' => Some(Self::Precision),
            'N' => Some(Self::MantissaDigits),
            'R' => Some(Self::Rounding),
            'M' => Some(Self::MinimumExponent),
            'U' => Some(Self::UnderflowThreshold),
            'L' => Some(Self::MaximumExponent),
            'O' => Some(Self::OverflowThreshold),
            _ => None,
        }
    }
}

/// Typed form of `DLAMCH` for IEEE-754 `f64`, matching the Fortran
/// intrinsics used by LAPACK 3.12.1.
pub(crate) fn dlamch_parameter(parameter: MachineParameter) -> f64 {
    let eps = f64::EPSILON * 0.5;
    match parameter {
        MachineParameter::Epsilon => eps,
        MachineParameter::SafeMinimum => {
            let small = 1.0 / f64::MAX;
            if small >= f64::MIN_POSITIVE {
                small * (1.0 + eps)
            } else {
                f64::MIN_POSITIVE
            }
        }
        MachineParameter::Base => 2.0,
        MachineParameter::Precision => eps * 2.0,
        MachineParameter::MantissaDigits => 53.0,
        MachineParameter::Rounding => 1.0,
        // Fortran MINEXPONENT(0d0) and MAXEXPONENT(0d0), respectively.
        MachineParameter::MinimumExponent => -1021.0,
        MachineParameter::UnderflowThreshold => f64::MIN_POSITIVE,
        MachineParameter::MaximumExponent => 1024.0,
        MachineParameter::OverflowThreshold => f64::MAX,
    }
}

/// Character-compatible form of `DLAMCH`; unrecognized selectors return zero.
pub(crate) fn dlamch(cmach: char) -> f64 {
    MachineParameter::from_char(cmach)
        .map(dlamch_parameter)
        .unwrap_or(0.0)
}

/// `DLAMC3`: retain the reference utility's explicit addition boundary.
#[inline(never)]
pub(crate) fn dlamc3(a: f64, b: f64) -> f64 {
    a + b
}

/// `DLAPY2`: robust `sqrt(x**2 + y**2)` with LAPACK NaN propagation.
pub(crate) fn dlapy2(x: f64, y: f64) -> f64 {
    let x_is_nan = disnan(x);
    let y_is_nan = disnan(y);
    // The reference makes two assignments in this order, so Y wins when
    // both operands are NaNs (including its payload/sign bits).
    if y_is_nan {
        return y;
    }
    if x_is_nan {
        return x;
    }
    let xabs = x.abs();
    let yabs = y.abs();
    let w = xabs.max(yabs);
    let z = xabs.min(yabs);
    if z == 0.0 || w > dlamch('O') {
        w
    } else {
        let ratio = z / w;
        w * (1.0 + ratio * ratio).sqrt()
    }
}

/// `DLASSQ`: update a scaled sum-of-squares pair using LAPACK 3.12.1's
/// three-accumulator algorithm.
pub(crate) fn dlassq(n: usize, x: &[f64], incx: isize, scale: &mut f64, sumsq: &mut f64) {
    if scale.is_nan() || sumsq.is_nan() {
        return;
    }
    if *sumsq == 0.0 {
        *scale = 1.0;
    }
    if *scale == 0.0 {
        *scale = 1.0;
        *sumsq = 0.0;
    }
    if n == 0 {
        return;
    }
    assert!(x.len() >= vector_required_len(n, incx));

    let mut notbig = true;
    let mut asml = 0.0;
    let mut amed = 0.0;
    let mut abig = 0.0;
    let mut ix = vector_start(n, incx);
    for _ in 0..n {
        let ax = x[ix as usize].abs();
        if ax > TBIG {
            let scaled = ax * SBIG;
            abig += scaled * scaled;
            notbig = false;
        } else if ax < TSML {
            if notbig {
                let scaled = ax * SSML;
                asml += scaled * scaled;
            }
        } else {
            amed += ax * ax;
        }
        ix += incx;
    }

    if *sumsq > 0.0 {
        let ax = *scale * sumsq.sqrt();
        if ax > TBIG {
            if *scale > 1.0 {
                *scale *= SBIG;
                abig += *scale * (*scale * *sumsq);
            } else {
                abig += *scale * (*scale * (SBIG * (SBIG * *sumsq)));
            }
        } else if ax < TSML {
            if notbig {
                if *scale < 1.0 {
                    *scale *= SSML;
                    asml += *scale * (*scale * *sumsq);
                } else {
                    asml += *scale * (*scale * (SSML * (SSML * *sumsq)));
                }
            }
        } else {
            amed += *scale * (*scale * *sumsq);
        }
    }

    if abig > 0.0 {
        if amed > 0.0 || amed.is_nan() {
            abig += (amed * SBIG) * SBIG;
        }
        *scale = 1.0 / SBIG;
        *sumsq = abig;
    } else if asml > 0.0 {
        if amed > 0.0 || amed.is_nan() {
            amed = amed.sqrt();
            asml = asml.sqrt() / SSML;
            let (ymin, ymax) = if asml > amed {
                (amed, asml)
            } else {
                (asml, amed)
            };
            *scale = 1.0;
            let ratio = ymin / ymax;
            *sumsq = (ymax * ymax) * (1.0 + ratio * ratio);
        } else {
            *scale = 1.0 / SSML;
            *sumsq = asml;
        }
    } else {
        *scale = 1.0;
        *sumsq = amed;
    }
}

/// `ILADLC`: one-based index of the last non-zero column, or zero.
pub(crate) fn iladlc(m: usize, n: usize, a: &[f64], lda: usize) -> usize {
    if n == 0 || m == 0 {
        return 0;
    }
    assert!(lda >= m.max(1));
    assert!(a.len() >= lda * n);
    if a[matrix_index(0, n - 1, lda)] != 0.0 || a[matrix_index(m - 1, n - 1, lda)] != 0.0 {
        return n;
    }
    for j in (0..n).rev() {
        for i in 0..m {
            if a[matrix_index(i, j, lda)] != 0.0 {
                return j + 1;
            }
        }
    }
    0
}

/// `ILADLR`: one-based index of the last non-zero row, or zero.
pub(crate) fn iladlr(m: usize, n: usize, a: &[f64], lda: usize) -> usize {
    if m == 0 || n == 0 {
        return 0;
    }
    assert!(lda >= m.max(1));
    assert!(a.len() >= lda * n);
    if a[matrix_index(m - 1, 0, lda)] != 0.0 || a[matrix_index(m - 1, n - 1, lda)] != 0.0 {
        return m;
    }
    let mut last = 0;
    for j in 0..n {
        let mut i = m;
        while i >= 1 && a[matrix_index(i - 1, j, lda)] == 0.0 {
            i -= 1;
        }
        last = last.max(i);
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: &[f64], expected: &[f64]) {
        assert_eq!(actual.len(), expected.len());
        for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
            let tolerance = 4.0e-14 * (1.0 + expected.abs());
            assert!(
                (actual - expected).abs() <= tolerance,
                "element {index}: actual={actual:?}, expected={expected:?}, tolerance={tolerance:?}"
            );
        }
    }

    #[test]
    fn option_types_parse_reference_letters() {
        assert_eq!(Transpose::from_char('n'), Some(Transpose::None));
        assert_eq!(Transpose::from_char('C'), Some(Transpose::Transpose));
        assert_eq!(Side::from_char('r'), Some(Side::Right));
        assert_eq!(Uplo::from_char('u'), Some(Uplo::Upper));
        assert_eq!(Diag::from_char('N'), Some(Diag::NonUnit));
        assert_eq!(Diag::from_char('x'), None);
        assert!(lsame('a', 'A'));
        assert!(!lsame('A', 'B'));
    }

    #[test]
    fn level_one_unit_and_signed_stride_paths() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let mut y = [10.0; 9];
        daxpy(9, 2.0, &x, 1, &mut y, 1);
        assert_eq!(y, [12.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0, 26.0, 28.0]);

        let mut scattered = [0.0; 5];
        dcopy(3, &x[..5], -2, &mut scattered, 2);
        assert_eq!(scattered, [5.0, 0.0, 3.0, 0.0, 1.0]);
        assert_eq!(ddot(3, &x[..5], -2, &scattered, 2), 35.0);

        // INC=0 is meaningful in reference DAXPY/DDOT even though it is not
        // a valid BLAS vector increment at level 2 and 3.
        let mut repeated = [1.0];
        daxpy(3, 2.0, &[4.0], 0, &mut repeated, 0);
        assert_eq!(repeated, [25.0]);
        assert_eq!(ddot(3, &[2.0], 0, &[5.0], 0), 30.0);

        let mut sx = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        dscal(3, -2.0, &mut sx, 2);
        assert_eq!(sx, [-2.0, 2.0, -6.0, 4.0, -10.0, 6.0]);
        let unchanged = sx;
        dscal(3, 8.0, &mut sx, -1);
        assert_eq!(sx, unchanged);

        let mut a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let mut b = [6.0, 7.0, 8.0, 9.0, 10.0];
        dswap(3, &mut a, -2, &mut b, 2);
        assert_eq!(a, [10.0, 2.0, 8.0, 4.0, 6.0]);
        assert_eq!(b, [5.0, 7.0, 3.0, 9.0, 1.0]);

        let mut rx = [1.0, 0.0, 2.0, 0.0, 3.0];
        let mut ry = [4.0, 0.0, 5.0, 0.0, 6.0];
        drot(3, &mut rx, -2, &mut ry, 2, 0.0, 1.0);
        assert_eq!(rx, [6.0, 0.0, 5.0, 0.0, 4.0]);
        assert_eq!(ry, [-3.0, 0.0, -2.0, 0.0, -1.0]);

        assert_eq!(idamax(4, &[1.0, -7.0, 7.0, 2.0], 1), 2);
        assert_eq!(idamax(3, &[1.0, 99.0, -8.0, 99.0, 4.0], 2), 2);
        assert_eq!(idamax(2, &[1.0, 2.0], 0), 0);
        assert_eq!(idamax_index(4, &[1.0, -7.0, 7.0, 2.0], 1), Some(1));
    }

    #[test]
    fn norms_handle_extreme_values_and_existing_sums() {
        assert_eq!(dnrm2(2, &[3.0, 4.0], 1), 5.0);
        assert_eq!(dnrm2(2, &[3.0e300, 4.0e300], 1), 5.0e300);
        assert!((dnrm2(2, &[3.0e-300, 4.0e-300], 1) / 5.0e-300 - 1.0).abs() < 3.0e-16);
        assert_eq!(dnrm2(3, &[2.0], 0), 12.0_f64.sqrt());
        assert!(dnrm2(2, &[1.0, f64::NAN], 1).is_nan());
        assert!(dnrm2(2, &[1.0, f64::INFINITY], 1).is_infinite());

        let mut scale = 2.0;
        let mut sumsq = 9.0;
        dlassq(2, &[3.0, 4.0], 1, &mut scale, &mut sumsq);
        assert_eq!(scale * sumsq.sqrt(), 61.0_f64.sqrt());

        let mut scale = 0.0;
        let mut sumsq = 123.0;
        dlassq(2, &[3.0e300, 4.0e300], 1, &mut scale, &mut sumsq);
        assert_eq!(scale * sumsq.sqrt(), 5.0e300);

        let mut scale = f64::NAN;
        let mut sumsq = 1.0;
        dlassq(1, &[2.0], 1, &mut scale, &mut sumsq);
        assert!(scale.is_nan());
        assert_eq!(sumsq, 1.0);
    }

    #[test]
    fn dgemv_matches_both_reference_branches_and_strides() {
        // 3-by-2 with LDA=4: columns [1,2,3] and [4,5,6].
        let a = [1.0, 2.0, 3.0, 99.0, 4.0, 5.0, 6.0, 99.0];
        let mut y = [10.0, 20.0, 30.0];
        dgemv(
            Transpose::None,
            3,
            2,
            2.0,
            &a,
            4,
            &[7.0, 8.0],
            1,
            -1.0,
            &mut y,
            1,
        )
        .unwrap();
        assert_eq!(y, [68.0, 88.0, 108.0]);

        let mut yt = [1.0, 99.0, 2.0];
        dgemv(
            Transpose::Transpose,
            3,
            2,
            0.5,
            &a,
            4,
            &[9.0, 99.0, 8.0, 99.0, 7.0],
            -2,
            2.0,
            &mut yt,
            -2,
        )
        .unwrap();
        assert_eq!(yt, [63.0, 99.0, 29.0]);
    }

    #[test]
    fn dger_matches_rank_one_update_with_negative_strides() {
        let mut a = [1.0, 2.0, 99.0, 3.0, 4.0, 99.0];
        dger(
            2,
            2,
            2.0,
            &[5.0, 99.0, 6.0],
            -2,
            &[7.0, 99.0, 8.0],
            -2,
            &mut a,
            3,
        )
        .unwrap();
        assert_eq!(a, [97.0, 82.0, 99.0, 87.0, 74.0, 99.0]);
    }

    fn stored_matrix(rows: usize, columns: usize, ld: usize, seed: f64) -> Vec<f64> {
        let mut result = vec![-999.0; ld * columns];
        for j in 0..columns {
            for i in 0..rows {
                result[matrix_index(i, j, ld)] = seed + (i + 1) as f64 + 0.25 * (j + 1) as f64;
            }
        }
        result
    }

    #[test]
    fn dgemm_matches_all_transposition_combinations() {
        let (m, n, k) = (2, 3, 4);
        for transa in [Transpose::None, Transpose::Transpose] {
            for transb in [Transpose::None, Transpose::Transpose] {
                let (arows, acols) = if transa == Transpose::None {
                    (m, k)
                } else {
                    (k, m)
                };
                let (brows, bcols) = if transb == Transpose::None {
                    (k, n)
                } else {
                    (n, k)
                };
                let lda = arows + 1;
                let ldb = brows + 1;
                let ldc = m + 1;
                let a = stored_matrix(arows, acols, lda, 0.5);
                let b = stored_matrix(brows, bcols, ldb, -0.25);
                let mut c = stored_matrix(m, n, ldc, 2.0);
                let original = c.clone();
                let mut expected = original.clone();
                for j in 0..n {
                    for i in 0..m {
                        let mut total = 0.0;
                        for l in 0..k {
                            let av = if transa == Transpose::None {
                                a[matrix_index(i, l, lda)]
                            } else {
                                a[matrix_index(l, i, lda)]
                            };
                            let bv = if transb == Transpose::None {
                                b[matrix_index(l, j, ldb)]
                            } else {
                                b[matrix_index(j, l, ldb)]
                            };
                            total += av * bv;
                        }
                        let ci = matrix_index(i, j, ldc);
                        expected[ci] = 1.25 * total - 0.5 * original[ci];
                    }
                }
                dgemm(
                    transa, transb, m, n, k, 1.25, &a, lda, &b, ldb, -0.5, &mut c, ldc,
                )
                .unwrap();
                for j in 0..n {
                    assert_close(
                        &c[matrix_index(0, j, ldc)..matrix_index(0, j, ldc) + m],
                        &expected[matrix_index(0, j, ldc)..matrix_index(0, j, ldc) + m],
                    );
                }
            }
        }
    }

    fn triangular_value(
        a: &[f64],
        lda: usize,
        uplo: Uplo,
        diag: Diag,
        row: usize,
        column: usize,
    ) -> f64 {
        if row == column && diag == Diag::Unit {
            1.0
        } else if (uplo == Uplo::Upper && row <= column) || (uplo == Uplo::Lower && row >= column) {
            a[matrix_index(row, column, lda)]
        } else {
            0.0
        }
    }

    #[test]
    fn dtrmm_matches_all_sides_triangles_transposes_and_diagonals() {
        let (m, n) = (3, 2);
        for side in [Side::Left, Side::Right] {
            let order = if side == Side::Left { m } else { n };
            let lda = order + 1;
            // Both halves are populated so tests catch reading the wrong one.
            let a = stored_matrix(order, order, lda, -0.75);
            for uplo in [Uplo::Upper, Uplo::Lower] {
                for trans in [Transpose::None, Transpose::Transpose] {
                    for diag in [Diag::Unit, Diag::NonUnit] {
                        let ldb = m + 1;
                        let mut b = stored_matrix(m, n, ldb, 1.5);
                        let original = b.clone();
                        let mut expected = b.clone();
                        for j in 0..n {
                            for i in 0..m {
                                let mut total = 0.0;
                                if side == Side::Left {
                                    for q in 0..m {
                                        let av = if trans == Transpose::None {
                                            triangular_value(&a, lda, uplo, diag, i, q)
                                        } else {
                                            triangular_value(&a, lda, uplo, diag, q, i)
                                        };
                                        total += av * original[matrix_index(q, j, ldb)];
                                    }
                                } else {
                                    for q in 0..n {
                                        let av = if trans == Transpose::None {
                                            triangular_value(&a, lda, uplo, diag, q, j)
                                        } else {
                                            triangular_value(&a, lda, uplo, diag, j, q)
                                        };
                                        total += original[matrix_index(i, q, ldb)] * av;
                                    }
                                }
                                expected[matrix_index(i, j, ldb)] = -1.25 * total;
                            }
                        }
                        dtrmm(side, uplo, trans, diag, m, n, -1.25, &a, lda, &mut b, ldb).unwrap();
                        for j in 0..n {
                            assert_close(
                                &b[matrix_index(0, j, ldb)..matrix_index(0, j, ldb) + m],
                                &expected[matrix_index(0, j, ldb)..matrix_index(0, j, ldb) + m],
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn scalar_helpers_match_ieee_double_parameters() {
        assert!(disnan(f64::NAN));
        assert!(!disnan(f64::INFINITY));
        assert!(dlaisnan(f64::NAN, f64::NAN));
        assert_eq!(dlamch('E'), f64::EPSILON * 0.5);
        assert_eq!(dlamch('P'), f64::EPSILON);
        assert_eq!(dlamch('S'), f64::MIN_POSITIVE);
        assert_eq!(dlamch('B'), 2.0);
        assert_eq!(dlamch('N'), 53.0);
        assert_eq!(dlamch('M'), -1021.0);
        assert_eq!(dlamch('L'), 1024.0);
        assert_eq!(dlamch('O'), f64::MAX);
        assert_eq!(dlamch('?'), 0.0);
        assert_eq!(dlamc3(1.25, 2.5), 3.75);
        assert_eq!(dlapy2(3.0, 4.0), 5.0);
        assert_eq!(dlapy2(3.0e300, 4.0e300), 5.0e300);
        assert!(dlapy2(f64::NAN, 1.0).is_nan());
        let nan_x = f64::from_bits(0x7ff8_0000_0000_0011);
        let nan_y = f64::from_bits(0xfff8_0000_0000_0022);
        assert_eq!(dlapy2(nan_x, nan_y).to_bits(), nan_y.to_bits());
        assert_eq!(dlapy2(f64::INFINITY, 1.0), f64::INFINITY);
    }

    #[test]
    fn iladlc_and_iladlr_scan_column_major_storage() {
        let lda = 4;
        let a = [
            0.0, 0.0, 0.0, 99.0, // column 1
            1.0, 0.0, 0.0, 99.0, // column 2
            0.0, 0.0, 2.0, 99.0, // column 3
            0.0, 0.0, 0.0, 99.0, // column 4
        ];
        assert_eq!(iladlc(3, 4, &a, lda), 3);
        assert_eq!(iladlr(3, 4, &a, lda), 3);
        assert_eq!(iladlc(0, 4, &[], 1), 0);
        assert_eq!(iladlr(3, 0, &[], 3), 0);
    }

    #[test]
    fn level_two_and_three_report_bad_dimensions_without_touching_buffers() {
        let mut empty = [];
        assert_eq!(
            dgemv(
                Transpose::None,
                1,
                1,
                1.0,
                &[],
                0,
                &[],
                1,
                0.0,
                &mut empty,
                1
            ),
            Err(BlasError::InvalidArgument {
                routine: "DGEMV",
                argument: 6
            })
        );
        assert!(matches!(
            dgemm(
                Transpose::None,
                Transpose::None,
                1,
                1,
                1,
                1.0,
                &[],
                1,
                &[],
                1,
                0.0,
                &mut empty,
                1
            ),
            Err(BlasError::BufferTooShort {
                routine: "DGEMM",
                buffer: "A",
                ..
            })
        ));

        let mut y = [3.0, 4.0];
        dgemv(Transpose::None, 2, 3, 0.0, &[], 2, &[], 1, -2.0, &mut y, 1).unwrap();
        assert_eq!(y, [-6.0, -8.0]);

        let mut c = [1.0, 2.0, 3.0, 4.0];
        dgemm(
            Transpose::None,
            Transpose::None,
            2,
            2,
            0,
            7.0,
            &[],
            2,
            &[],
            1,
            3.0,
            &mut c,
            2,
        )
        .unwrap();
        assert_eq!(c, [3.0, 6.0, 9.0, 12.0]);

        let mut b = [f64::NAN, 2.0];
        dtrmm(
            Side::Left,
            Uplo::Upper,
            Transpose::None,
            Diag::NonUnit,
            2,
            1,
            0.0,
            &[],
            2,
            &mut b,
            2,
        )
        .unwrap();
        assert_eq!(b, [0.0, 0.0]);
    }

    #[test]
    fn level_two_and_three_accept_pointer_offset_matrix_views() {
        // Each matrix ends at its last addressed element: the unused padding
        // after the final column is intentionally absent, as for `&A[i,j]`
        // pointers passed throughout LAPACK.
        let a = [1.0, 2.0, 99.0, 3.0, 4.0]; // 2x2, lda=3
        let mut y = [0.0, 0.0];
        dgemv(
            Transpose::None,
            2,
            2,
            1.0,
            &a,
            3,
            &[5.0, 6.0],
            1,
            0.0,
            &mut y,
            1,
        )
        .unwrap();
        assert_eq!(y, [23.0, 34.0]);

        let mut rank_one = a;
        dger(2, 2, 1.0, &[1.0, 2.0], 1, &[3.0, 4.0], 1, &mut rank_one, 3).unwrap();
        assert_eq!(rank_one, [4.0, 8.0, 99.0, 7.0, 12.0]);

        let mut product = [0.0, 0.0, 99.0, 0.0, 0.0];
        dgemm(
            Transpose::None,
            Transpose::None,
            2,
            2,
            2,
            1.0,
            &a,
            3,
            &a,
            3,
            0.0,
            &mut product,
            3,
        )
        .unwrap();
        assert_eq!(product, [7.0, 10.0, 99.0, 15.0, 22.0]);

        let triangular = [1.0, 88.0, 99.0, 2.0, 3.0];
        let mut rhs = [5.0, 6.0, 99.0, 7.0, 8.0];
        dtrmm(
            Side::Left,
            Uplo::Upper,
            Transpose::None,
            Diag::NonUnit,
            2,
            2,
            1.0,
            &triangular,
            3,
            &mut rhs,
            3,
        )
        .unwrap();
        assert_eq!(rhs, [17.0, 18.0, 99.0, 23.0, 24.0]);
    }
}
