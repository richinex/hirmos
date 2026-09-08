//! Shared minimum-norm solve extracted from the sklearn dense regression port.
//! Callers own centering and the source-specific relative rank tolerance.
use nalgebra::{DMatrix, DVector};

pub(crate) struct Solution {
    pub coefficients: DMatrix<f64>,
    pub rank: usize,
    pub singular_values: DVector<f64>,
    pub cutoff: f64,
}

pub(crate) fn solve(
    a: &DMatrix<f64>,
    b: &DMatrix<f64>,
    tolerance: f64,
) -> Result<Solution, &'static str> {
    if a.nrows() != b.nrows() || a.nrows() == 0 {
        return Err("least-squares row mismatch or empty samples");
    }
    if !tolerance.is_finite()
        || (tolerance < 0.0 && tolerance != -1.0)
        || a.iter().chain(b.iter()).any(|v| !v.is_finite())
    {
        return Err("non-finite least-squares input or invalid tolerance");
    }
    if a.ncols() == 0 {
        return Ok(Solution {
            coefficients: DMatrix::zeros(0, b.ncols()),
            rank: 0,
            singular_values: DVector::zeros(0),
            cutoff: 0.0,
        });
    }

    // NumPy and SciPy route this source path through LAPACK DGELSD. Keep the
    // column-major matrices intact and give B LAPACK's required max(m, n)
    // leading dimension; its first n rows become the minimum-norm solution.
    let m = a.nrows();
    let n = a.ncols();
    let nrhs = b.ncols();
    let lda = m.max(1);
    let ldb = m.max(n).max(1);
    let mut design = a.as_slice().to_vec();
    let mut targets = vec![0.0; ldb * nrhs];
    for column in 0..nrhs {
        targets[column * ldb..column * ldb + m].copy_from_slice(b.column(column).as_slice());
    }

    let k = m.min(n);
    let mut singular = vec![0.0; k];
    let mut rank = 0;
    let workspace = crate::lapack_dgelsd::dgelsd_workspace(m, n, nrhs);
    let mut work = vec![0.0; workspace.optimal_real];
    let mut iwork = vec![0; workspace.minimum_integer];
    crate::lapack_dgelsd::dgelsd(
        m,
        n,
        nrhs,
        &mut design,
        lda,
        &mut targets,
        ldb,
        &mut singular,
        tolerance,
        &mut rank,
        &mut work,
        workspace.optimal_real as isize,
        &mut iwork,
    )
    .map_err(|_| "DGELSD failed to converge")?;

    let singular_values = DVector::from_vec(singular);
    let effective_tolerance = if tolerance < 0.0 {
        f64::EPSILON
    } else {
        tolerance
    };
    let cutoff = effective_tolerance * if k == 0 { 0.0 } else { singular_values[0] };
    let coefficients = DMatrix::from_fn(n, nrhs, |row, column| targets[row + column * ldb]);
    Ok(Solution {
        coefficients,
        rank,
        singular_values,
        cutoff,
    })
}
