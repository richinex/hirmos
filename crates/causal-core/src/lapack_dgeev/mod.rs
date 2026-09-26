//! LAPACK 3.12.1 `DGEEV` closure for real nonsymmetric matrices.
//!
//! The generated routines are mechanical Rust translations of the vendored
//! CLAPACK f2c sources. Their control flow is checked against the corresponding
//! LAPACK 3.12.1 Fortran sources before they enter this typed driver.

use core::fmt;

#[allow(unused_parens, unused_variables, clippy::eq_op, clippy::approx_constant)]
mod generated {
    pub mod d_lg10;
    pub mod d_sign;
    pub mod dgebak;
    pub mod dgebal;
    pub mod dgeev;
    pub mod dgehd2;
    pub mod dgehrd;
    pub mod dhseqr;
    pub mod dlabad;
    pub mod dladiv;
    pub mod dlaexc;
    pub mod dlahqr;
    pub mod dlahr2;
    pub mod dlaln2;
    pub mod dlange;
    pub mod dlanv2;
    pub mod dlaqr0;
    pub mod dlaqr1;
    pub mod dlaqr2;
    pub mod dlaqr3;
    pub mod dlaqr4;
    pub mod dlaqr5;
    pub mod dlarf;
    pub mod dlarfb;
    pub mod dlarfg;
    pub mod dlarft;
    pub mod dlarfx;
    pub mod dlaset;
    pub mod dlassq;
    pub mod dlasy2;
    pub mod dorg2r;
    pub mod dorghr;
    pub mod dorgqr;
    pub mod dorm2r;
    pub mod dormhr;
    pub mod dormqr;
    pub mod dtrevc;
    pub mod dtrexc;
    pub mod i_nint;
    pub mod ieeeck;
    pub mod iladlc;
    pub mod iladlr;
    pub mod ilaenv;
    pub mod iparmq;
    pub mod s_cmp;
    pub mod s_copy;
}

type FInt = core::ffi::c_long;
type FChar = core::ffi::c_char;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Eigenvectors {
    None,
    Left,
    Right,
    Both,
}

impl Eigenvectors {
    fn left(self) -> bool {
        matches!(self, Self::Left | Self::Both)
    }

    fn right(self) -> bool {
        matches!(self, Self::Right | Self::Both)
    }
}

#[derive(Debug)]
pub struct DgeevOutput {
    pub real_eigenvalues: Vec<f64>,
    pub imaginary_eigenvalues: Vec<f64>,
    /// Column-major left eigenvectors in LAPACK's real encoding.
    #[allow(dead_code)]
    pub left_vectors: Option<Vec<f64>>,
    /// Column-major right eigenvectors in LAPACK's real encoding.
    pub right_vectors: Option<Vec<f64>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DgeevError {
    InvalidLeadingDimension,
    MatrixTooShort,
    DimensionOverflow,
    InvalidWorkspace,
    InvalidArgument(usize),
}

impl fmt::Display for DgeevError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLeadingDimension => {
                formatter.write_str("the leading dimension is smaller than the matrix order")
            }
            Self::MatrixTooShort => formatter.write_str("the DGEEV matrix buffer is too short"),
            Self::DimensionOverflow => formatter.write_str("the DGEEV dimensions overflowed"),
            Self::InvalidWorkspace => {
                formatter.write_str("DGEEV returned an invalid workspace size")
            }
            Self::InvalidArgument(argument) => {
                write!(formatter, "DGEEV rejected argument {argument}")
            }
        }
    }
}

impl std::error::Error for DgeevError {}

fn matrix_len(order: usize, ld: usize) -> Option<usize> {
    if order == 0 {
        Some(0)
    } else {
        (order - 1).checked_mul(ld)?.checked_add(order)
    }
}

struct Vectors {
    left: Vec<f64>,
    ld_left: usize,
    right: Vec<f64>,
    ld_right: usize,
}

impl Vectors {
    fn new(job: Eigenvectors, order: usize) -> Result<Self, DgeevError> {
        let square = order
            .checked_mul(order)
            .ok_or(DgeevError::DimensionOverflow)?;
        Ok(Self {
            left: vec![0.0; if job.left() { square } else { 1 }],
            ld_left: if job.left() { order.max(1) } else { 1 },
            right: vec![0.0; if job.right() { square } else { 1 }],
            ld_right: if job.right() { order.max(1) } else { 1 },
        })
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn raw_call(
    job: Eigenvectors,
    order: usize,
    a: &mut [f64],
    lda: usize,
    real: &mut [f64],
    imaginary: &mut [f64],
    vectors: &mut Vectors,
    work: &mut [f64],
    lwork_value: isize,
) -> Result<FInt, DgeevError> {
    let mut jobvl = if job.left() { b'V' } else { b'N' } as FChar;
    let mut jobvr = if job.right() { b'V' } else { b'N' } as FChar;
    let mut n = FInt::try_from(order).map_err(|_| DgeevError::DimensionOverflow)?;
    let mut lda = FInt::try_from(lda).map_err(|_| DgeevError::DimensionOverflow)?;
    let mut ldvl = FInt::try_from(vectors.ld_left).map_err(|_| DgeevError::DimensionOverflow)?;
    let mut ldvr = FInt::try_from(vectors.ld_right).map_err(|_| DgeevError::DimensionOverflow)?;
    let mut lwork = FInt::try_from(lwork_value).map_err(|_| DgeevError::DimensionOverflow)?;
    let mut info = 0 as FInt;

    generated::dgeev::dgeev_closure_dgeev_(
        &mut jobvl,
        &mut jobvr,
        &mut n,
        a.as_mut_ptr(),
        &mut lda,
        real.as_mut_ptr(),
        imaginary.as_mut_ptr(),
        vectors.left.as_mut_ptr(),
        &mut ldvl,
        vectors.right.as_mut_ptr(),
        &mut ldvr,
        work.as_mut_ptr(),
        &mut lwork,
        &mut info,
    );
    Ok(info)
}

/// Return the optimal real workspace reported by the source driver.
pub fn dgeev_workspace(job: Eigenvectors, order: usize) -> Result<usize, DgeevError> {
    let lda = order.max(1);
    let mut a = vec![
        0.0;
        order
            .checked_mul(order)
            .ok_or(DgeevError::DimensionOverflow)?
            .max(1)
    ];
    let mut real = vec![0.0; order.max(1)];
    let mut imaginary = vec![0.0; order.max(1)];
    let mut vectors = Vectors::new(job, order)?;
    let mut work = vec![0.0];
    let info = unsafe {
        raw_call(
            job,
            order,
            &mut a,
            lda,
            &mut real,
            &mut imaginary,
            &mut vectors,
            &mut work,
            -1,
        )?
    };
    if info < 0 {
        return Err(DgeevError::InvalidArgument((-info) as usize));
    }
    if !work[0].is_finite() || work[0] < 1.0 || work[0] > usize::MAX as f64 {
        return Err(DgeevError::InvalidWorkspace);
    }
    Ok(work[0] as usize)
}

/// Compute all eigenvalues and selected eigenvectors of a real general matrix.
///
/// Complex conjugate eigenpairs use LAPACK's real two-column encoding. If
/// `imaginary_eigenvalues[j] > 0`, the eigenvector for eigenvalue `j` is
/// `vectors[:, j] + i * vectors[:, j + 1]`; the next eigenvalue/vector is its
/// complex conjugate.
pub fn dgeev(
    job: Eigenvectors,
    order: usize,
    a: &mut [f64],
    lda: usize,
) -> Result<(usize, DgeevOutput), DgeevError> {
    if lda < order.max(1) {
        return Err(DgeevError::InvalidLeadingDimension);
    }
    let required = matrix_len(order, lda).ok_or(DgeevError::DimensionOverflow)?;
    if a.len() < required {
        return Err(DgeevError::MatrixTooShort);
    }
    if order == 0 {
        return Ok((
            0,
            DgeevOutput {
                real_eigenvalues: Vec::new(),
                imaginary_eigenvalues: Vec::new(),
                left_vectors: job.left().then(Vec::new),
                right_vectors: job.right().then(Vec::new),
            },
        ));
    }

    let mut real = vec![0.0; order];
    let mut imaginary = vec![0.0; order];
    let mut vectors = Vectors::new(job, order)?;
    let workspace = dgeev_workspace(job, order)?;
    let mut work = vec![0.0; workspace];
    let info = unsafe {
        raw_call(
            job,
            order,
            a,
            lda,
            &mut real,
            &mut imaginary,
            &mut vectors,
            &mut work,
            workspace as isize,
        )?
    };
    if info < 0 {
        return Err(DgeevError::InvalidArgument((-info) as usize));
    }

    Ok((
        info as usize,
        DgeevOutput {
            real_eigenvalues: real,
            imaginary_eigenvalues: imaginary,
            left_vectors: job.left().then_some(vectors.left),
            right_vectors: job.right().then_some(vectors.right),
        },
    ))
}

/// f2c fixed-length character concatenation used to build ILAENV option text.
#[no_mangle]
unsafe extern "C" fn dgeev_closure_s_cat(
    target: *mut FChar,
    sources: *mut *mut FChar,
    lengths: *mut FInt,
    count: *mut FInt,
    target_len: FInt,
) -> core::ffi::c_int {
    let mut written = 0usize;
    let limit = target_len.max(0) as usize;
    for index in 0..(*count).max(0) as usize {
        let source = *sources.add(index);
        let length = (*lengths.add(index)).max(0) as usize;
        for offset in 0..length.min(limit - written) {
            *target.add(written) = *source.add(offset);
            written += 1;
        }
        if written == limit {
            break;
        }
    }
    while written < limit {
        *target.add(written) = b' ' as FChar;
        written += 1;
    }
    0
}
