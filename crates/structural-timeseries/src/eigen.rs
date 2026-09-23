//! Safe adapter to the DSYEVD symbol already compiled by hirmos-causal-core.
//! No second eigensolver or translated LAPACK closure is compiled here.
use crate::Error;
use nalgebra::{DMatrix, DVector};
use std::ffi::{c_char, c_int, c_long};
extern "C" {
    fn dsyevd_closure_dsyevd_(
        job: *mut c_char,
        triangle: *mut c_char,
        n: *mut c_long,
        a: *mut f64,
        lda: *mut c_long,
        values: *mut f64,
        work: *mut f64,
        lwork: *mut c_long,
        iwork: *mut c_long,
        liwork: *mut c_long,
        info: *mut c_long,
    ) -> c_int;
}
pub(crate) fn symmetric(matrix: &DMatrix<f64>) -> Result<(DVector<f64>, DMatrix<f64>), Error> {
    if matrix.nrows() == 0 || !matrix.is_square() {
        return Err(Error::Shape);
    }
    if matrix.iter().any(|x| !x.is_finite()) {
        return Err(Error::NonFinite);
    }
    let mut n = c_long::try_from(matrix.nrows()).map_err(|_| Error::Shape)?;
    let mut lda = n;
    let mut vectors = matrix.clone();
    let mut values = DVector::zeros(matrix.nrows());
    let mut job = b'V' as c_char;
    let mut triangle = b'L' as c_char;
    let mut info = 0;
    let mut lwork = -1;
    let mut liwork = -1;
    let mut work_size = [0.];
    let mut integer_size = [0];
    // Both buffers are one element long, as required by LAPACK's query mode.
    unsafe {
        dsyevd_closure_dsyevd_(
            &mut job,
            &mut triangle,
            &mut n,
            vectors.as_mut_ptr(),
            &mut lda,
            values.as_mut_ptr(),
            work_size.as_mut_ptr(),
            &mut lwork,
            integer_size.as_mut_ptr(),
            &mut liwork,
            &mut info,
        );
    }
    if info != 0 || !work_size[0].is_finite() || work_size[0] < 1. || integer_size[0] < 1 {
        return Err(Error::Shape);
    }
    if work_size[0] >= c_long::MAX as f64 {
        return Err(Error::Shape);
    }
    lwork = work_size[0] as c_long;
    liwork = integer_size[0];
    let mut work = vec![0.; usize::try_from(lwork).map_err(|_| Error::Shape)?];
    let mut integers = vec![0; usize::try_from(liwork).map_err(|_| Error::Shape)?];
    // Shape and queried workspaces satisfy the translated driver's contract.
    unsafe {
        dsyevd_closure_dsyevd_(
            &mut job,
            &mut triangle,
            &mut n,
            vectors.as_mut_ptr(),
            &mut lda,
            values.as_mut_ptr(),
            work.as_mut_ptr(),
            &mut lwork,
            integers.as_mut_ptr(),
            &mut liwork,
            &mut info,
        );
    }
    if info != 0 {
        return Err(Error::Singular);
    }
    if values.iter().chain(vectors.iter()).any(|x| !x.is_finite()) {
        return Err(Error::NonFinite);
    }
    Ok((values, vectors))
}
