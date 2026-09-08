//! LAPACK `DGESDD` divide-and-conquer singular-value decomposition.

use core::fmt;

#[allow(unused_parens)]
mod generated {
    pub mod dbdsdc;
    pub mod dgebd2;
    pub mod dgebrd;
    pub mod dgelq2;
    pub mod dgelqf;
    pub mod dgeqr2;
    pub mod dgeqrf;
    pub mod dgesdd;
    pub mod dlabrd;
    pub mod dlange;
    pub mod dlarfp;
    pub mod dlasd0;
    pub mod dlasd1;
    pub mod dlasd2;
    pub mod dlasd3;
    pub mod dorg2r;
    pub mod dorgbr;
    pub mod dorgl2;
    pub mod dorglq;
    pub mod dorgqr;
    pub mod dormbr;
    pub mod dorml2;
    pub mod dormlq;
}

type FInt = core::ffi::c_long;
type FChar = core::ffi::c_char;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum SvdJob {
    All,
    Some,
    Overwrite,
    None,
}

impl SvdJob {
    fn character(self) -> FChar {
        match self {
            Self::All => b'A' as FChar,
            Self::Some => b'S' as FChar,
            Self::Overwrite => b'O' as FChar,
            Self::None => b'N' as FChar,
        }
    }
}

#[derive(Debug)]
pub struct DgesddOutput {
    pub singular_values: Vec<f64>,
    pub left_vectors: Option<Vec<f64>>,
    pub left_rows: usize,
    pub left_columns: usize,
    pub right_vectors_transposed: Option<Vec<f64>>,
    pub right_rows: usize,
    pub right_columns: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DgesddError {
    InvalidLeadingDimension,
    MatrixTooShort,
    DimensionOverflow,
    InvalidArgument(usize),
}

impl fmt::Display for DgesddError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLeadingDimension => {
                formatter.write_str("the leading dimension is smaller than the matrix row count")
            }
            Self::MatrixTooShort => formatter.write_str("the SVD matrix buffer is too short"),
            Self::DimensionOverflow => formatter.write_str("the DGESDD dimensions overflowed"),
            Self::InvalidArgument(argument) => {
                write!(formatter, "DGESDD rejected argument {argument}")
            }
        }
    }
}

impl std::error::Error for DgesddError {}

fn matrix_len(rows: usize, columns: usize, ld: usize) -> Option<usize> {
    if rows == 0 || columns == 0 {
        Some(0)
    } else {
        (columns - 1).checked_mul(ld)?.checked_add(rows)
    }
}

struct Buffers {
    u: Vec<f64>,
    ldu: usize,
    vt: Vec<f64>,
    ldvt: usize,
}

fn buffers(job: SvdJob, rows: usize, columns: usize) -> Result<Buffers, DgesddError> {
    let order = rows.min(columns);
    let (u_rows, u_columns, vt_rows, vt_columns) = match job {
        SvdJob::All => (rows, rows, columns, columns),
        SvdJob::Some => (rows, order, order, columns),
        SvdJob::Overwrite if rows < columns => (rows, rows, 1, 1),
        SvdJob::Overwrite => (1, 1, columns, columns),
        SvdJob::None => (1, 1, 1, 1),
    };
    let ldu = u_rows.max(1);
    let ldvt = vt_rows.max(1);
    let u_len = matrix_len(u_rows, u_columns, ldu).ok_or(DgesddError::DimensionOverflow)?;
    let vt_len = matrix_len(vt_rows, vt_columns, ldvt).ok_or(DgesddError::DimensionOverflow)?;
    Ok(Buffers {
        u: vec![0.0; u_len.max(1)],
        ldu,
        vt: vec![0.0; vt_len.max(1)],
        ldvt,
    })
}

unsafe fn raw_call(
    job: SvdJob,
    rows: usize,
    columns: usize,
    a: &mut [f64],
    lda: usize,
    singular_values: &mut [f64],
    buffers: &mut Buffers,
    work: &mut [f64],
    lwork_value: isize,
    integers: &mut [FInt],
) -> Result<FInt, DgesddError> {
    let mut jobz = job.character();
    let mut m = FInt::try_from(rows).map_err(|_| DgesddError::DimensionOverflow)?;
    let mut n = FInt::try_from(columns).map_err(|_| DgesddError::DimensionOverflow)?;
    let mut lda = FInt::try_from(lda).map_err(|_| DgesddError::DimensionOverflow)?;
    let mut ldu = FInt::try_from(buffers.ldu).map_err(|_| DgesddError::DimensionOverflow)?;
    let mut ldvt = FInt::try_from(buffers.ldvt).map_err(|_| DgesddError::DimensionOverflow)?;
    let mut lwork = FInt::try_from(lwork_value).map_err(|_| DgesddError::DimensionOverflow)?;
    let mut info = 0 as FInt;
    generated::dgesdd::dgesdd_closure_dgesdd_(
        &mut jobz,
        &mut m,
        &mut n,
        a.as_mut_ptr(),
        &mut lda,
        singular_values.as_mut_ptr(),
        buffers.u.as_mut_ptr(),
        &mut ldu,
        buffers.vt.as_mut_ptr(),
        &mut ldvt,
        work.as_mut_ptr(),
        &mut lwork,
        integers.as_mut_ptr(),
        &mut info,
    );
    Ok(info)
}

/// Return the source driver's optimal real workspace for these dimensions.
pub fn dgesdd_workspace(job: SvdJob, rows: usize, columns: usize) -> Result<usize, DgesddError> {
    let lda = rows.max(1);
    let matrix_size =
        matrix_len(rows.max(1), columns.max(1), lda).ok_or(DgesddError::DimensionOverflow)?;
    let mut a = vec![0.0; matrix_size.max(1)];
    let mut singular_values = vec![0.0; rows.min(columns).max(1)];
    let mut vectors = buffers(job, rows, columns)?;
    let mut work = vec![0.0; 1];
    let mut integers = vec![0 as FInt; rows.min(columns).saturating_mul(8).max(1)];
    let info = unsafe {
        raw_call(
            job,
            rows,
            columns,
            &mut a,
            lda,
            &mut singular_values,
            &mut vectors,
            &mut work,
            -1,
            &mut integers,
        )?
    };
    if info < 0 {
        return Err(DgesddError::InvalidArgument((-info) as usize));
    }
    if !work[0].is_finite() || work[0] < 1.0 || work[0] > usize::MAX as f64 {
        return Err(DgesddError::DimensionOverflow);
    }
    Ok(work[0] as usize)
}

/// Compute a full, economy, overwrite, or values-only SVD in column-major form.
pub fn dgesdd(
    job: SvdJob,
    rows: usize,
    columns: usize,
    a: &mut [f64],
    lda: usize,
) -> Result<(usize, DgesddOutput), DgesddError> {
    if lda < rows.max(1) {
        return Err(DgesddError::InvalidLeadingDimension);
    }
    let required = matrix_len(rows, columns, lda).ok_or(DgesddError::DimensionOverflow)?;
    if a.len() < required {
        return Err(DgesddError::MatrixTooShort);
    }
    if rows == 0 || columns == 0 {
        return Ok((
            0,
            DgesddOutput {
                singular_values: Vec::new(),
                left_vectors: None,
                left_rows: 0,
                left_columns: 0,
                right_vectors_transposed: None,
                right_rows: 0,
                right_columns: 0,
            },
        ));
    }

    let order = rows.min(columns);
    let mut singular_values = vec![0.0; order];
    let mut vectors = buffers(job, rows, columns)?;
    let lwork = dgesdd_workspace(job, rows, columns)?;
    let mut work = vec![0.0; lwork];
    let mut integers = vec![0 as FInt; order.checked_mul(8).ok_or(DgesddError::DimensionOverflow)?];
    let info = unsafe {
        raw_call(
            job,
            rows,
            columns,
            a,
            lda,
            &mut singular_values,
            &mut vectors,
            &mut work,
            isize::try_from(lwork).map_err(|_| DgesddError::DimensionOverflow)?,
            &mut integers,
        )?
    };
    if info < 0 {
        return Err(DgesddError::InvalidArgument((-info) as usize));
    }

    let (left_vectors, left_rows, left_columns) = match job {
        SvdJob::All => (Some(vectors.u), rows, rows),
        SvdJob::Some => (Some(vectors.u), rows, order),
        SvdJob::Overwrite if rows < columns => (Some(vectors.u), rows, rows),
        _ => (None, 0, 0),
    };
    let (right_vectors_transposed, right_rows, right_columns) = match job {
        SvdJob::All => (Some(vectors.vt), columns, columns),
        SvdJob::Some => (Some(vectors.vt), order, columns),
        SvdJob::Overwrite if rows >= columns => (Some(vectors.vt), columns, columns),
        _ => (None, 0, 0),
    };
    Ok((
        info as usize,
        DgesddOutput {
            singular_values,
            left_vectors,
            left_rows,
            left_columns,
            right_vectors_transposed,
            right_rows,
            right_columns,
        },
    ))
}
