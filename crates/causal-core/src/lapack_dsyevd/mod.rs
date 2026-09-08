//! LAPACK 3.12.1 `DSYEVD` closure, retaining column-major storage.
//!
//! The generated routines are mechanical Rust translations of the vendored
//! CLAPACK f2c sources. Their control flow is checked against the corresponding
//! LAPACK 3.12.1 Fortran sources before they enter the public driver.

use core::fmt;

#[allow(unused_parens)]
mod generated {
    pub mod daxpy;
    pub mod dger;
    pub mod dlae2;
    pub mod dlaed0;
    pub mod dlaed1;
    pub mod dlaed2;
    pub mod dlaed3;
    pub mod dlaed4;
    pub mod dlaed5;
    pub mod dlaed7;
    pub mod dlaed8;
    pub mod dlaed9;
    pub mod dlaeda;
    pub mod dlaev2;
    pub mod dlaisnan;
    pub mod dlansy;
    pub mod dlarf;
    pub mod dlarfb;
    pub mod dlarfg;
    pub mod dlarft;
    pub mod dlatrd;
    pub mod dorm2l;
    pub mod dorm2r;
    pub mod dormql;
    pub mod dormqr;
    pub mod dormtr;
    pub mod dstedc;
    pub mod dsteqr;
    pub mod dsterf;
    pub mod dsyevd;
    pub mod dsymv;
    pub mod dsyr2;
    pub mod dsyr2k;
    pub mod dsytd2;
    pub mod dsytrd;
    pub mod dtrmm;
    pub mod dtrmv;
    pub mod iladlc;
    pub mod iladlr;
}

type FInt = core::ffi::c_long;
type FChar = core::ffi::c_char;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EigenJob {
    Values,
    Vectors,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Triangle {
    Upper,
    Lower,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DsyevdError {
    InvalidLeadingDimension,
    MatrixTooShort,
    EigenvaluesTooShort,
    WorkspaceOverflow,
    InvalidArgument(usize),
}

impl fmt::Display for DsyevdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLeadingDimension => {
                formatter.write_str("the leading dimension is smaller than the matrix order")
            }
            Self::MatrixTooShort => formatter.write_str("the symmetric matrix buffer is too short"),
            Self::EigenvaluesTooShort => {
                formatter.write_str("the eigenvalue output buffer is too short")
            }
            Self::WorkspaceOverflow => formatter.write_str("the DSYEVD workspace size overflowed"),
            Self::InvalidArgument(argument) => {
                write!(formatter, "DSYEVD rejected argument {argument}")
            }
        }
    }
}

impl std::error::Error for DsyevdError {}

fn matrix_len(order: usize, ld: usize) -> usize {
    if order == 0 {
        0
    } else {
        order + (order - 1) * ld
    }
}

pub fn dsyevd_workspace(job: EigenJob, order: usize) -> Result<(usize, usize), DsyevdError> {
    if order <= 1 {
        return Ok((1, 1));
    }
    let reduction = 2usize
        .checked_mul(order)
        .and_then(|value| value.checked_add(32usize.checked_mul(order)?))
        .ok_or(DsyevdError::WorkspaceOverflow)?;
    match job {
        EigenJob::Values => Ok(((2 * order + 1).max(reduction), 1)),
        EigenJob::Vectors => {
            let square = order
                .checked_mul(order)
                .and_then(|value| value.checked_mul(2))
                .ok_or(DsyevdError::WorkspaceOverflow)?;
            let work = 1usize
                .checked_add(
                    6usize
                        .checked_mul(order)
                        .ok_or(DsyevdError::WorkspaceOverflow)?,
                )
                .and_then(|value| value.checked_add(square))
                .ok_or(DsyevdError::WorkspaceOverflow)?;
            let integers = 3usize
                .checked_add(
                    5usize
                        .checked_mul(order)
                        .ok_or(DsyevdError::WorkspaceOverflow)?,
                )
                .ok_or(DsyevdError::WorkspaceOverflow)?;
            Ok((work.max(reduction), integers))
        }
    }
}

/// Compute all eigenvalues and, optionally, eigenvectors of a symmetric matrix.
///
/// The selected triangle is read from `a`. When eigenvectors are requested,
/// the columns of `a` are replaced by the orthonormal eigenvectors.
pub fn dsyevd(
    job: EigenJob,
    triangle: Triangle,
    order: usize,
    a: &mut [f64],
    ld: usize,
    eigenvalues: &mut [f64],
) -> Result<usize, DsyevdError> {
    if ld < order.max(1) {
        return Err(DsyevdError::InvalidLeadingDimension);
    }
    if a.len() < matrix_len(order, ld) {
        return Err(DsyevdError::MatrixTooShort);
    }
    if eigenvalues.len() < order {
        return Err(DsyevdError::EigenvaluesTooShort);
    }
    if order == 0 {
        return Ok(0);
    }

    let (lwork, liwork) = dsyevd_workspace(job, order)?;
    let mut work = vec![0.0; lwork];
    let mut iwork = vec![0 as FInt; liwork];
    let mut jobz = match job {
        EigenJob::Values => b'N' as FChar,
        EigenJob::Vectors => b'V' as FChar,
    };
    let mut uplo = match triangle {
        Triangle::Upper => b'U' as FChar,
        Triangle::Lower => b'L' as FChar,
    };
    let mut n = FInt::try_from(order).map_err(|_| DsyevdError::WorkspaceOverflow)?;
    let mut lda = FInt::try_from(ld).map_err(|_| DsyevdError::WorkspaceOverflow)?;
    let mut lwork = FInt::try_from(lwork).map_err(|_| DsyevdError::WorkspaceOverflow)?;
    let mut liwork = FInt::try_from(liwork).map_err(|_| DsyevdError::WorkspaceOverflow)?;
    let mut info = 0 as FInt;

    unsafe {
        generated::dsyevd::dsyevd_closure_dsyevd_(
            &mut jobz,
            &mut uplo,
            &mut n,
            a.as_mut_ptr(),
            &mut lda,
            eigenvalues.as_mut_ptr(),
            work.as_mut_ptr(),
            &mut lwork,
            iwork.as_mut_ptr(),
            &mut liwork,
            &mut info,
        );
    }
    if info < 0 {
        Err(DsyevdError::InvalidArgument((-info) as usize))
    } else {
        Ok(info as usize)
    }
}

/// f2c fixed-length character concatenation used to build ILAENV option text.
#[no_mangle]
unsafe extern "C" fn dsyevd_closure_s_cat(
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
