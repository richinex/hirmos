//! Spline regression on explicitly prepared outcome-change contrasts.
//! Matches the 201785 replication's curve IF calculation, not its feols vcov.
//! This conditional regression uncertainty does not propagate uncertainty from
//! estimating the comparison-group means used to construct the contrasts.
use crate::continuous_did::{r_qr::Qr, spline::Basis};

#[derive(Debug)]
pub enum Error {
    Input,
    Spline(crate::continuous_did::spline::Error),
    RankDeficient,
    Numerical,
}

#[derive(Debug, PartialEq)]
pub enum Uncertainty {
    /// Residual-based row influence; no cluster or first-stage correction.
    ConditionalRowInfluence,
}

pub struct Fit {
    pub coefficients: Vec<f64>,
    pub fitted: Vec<f64>,
    pub residuals: Vec<f64>,
    pub curve: Vec<f64>,
    pub standard_errors: Vec<f64>,
    pub uncertainty: Uncertainty,
    pub observations: usize,
}

/// Explicit knots and evaluation grid. Training boundaries are retained when
/// evaluating the curve; the caller chooses the display range separately.
pub fn fit(
    dose: &[f64],
    contrast: &[f64],
    grid: &[f64],
    degree: usize,
    knots: Vec<f64>,
) -> Result<Fit, Error> {
    let n = dose.len();
    let p = degree
        .checked_add(1)
        .and_then(|v| v.checked_add(knots.len()))
        .ok_or(Error::Input)?;
    if n != contrast.len()
        || n <= p
        || grid.is_empty()
        || dose
            .iter()
            .chain(contrast)
            .chain(grid)
            .any(|v| !v.is_finite())
    {
        return Err(Error::Input);
    }
    let boundaries = [
        dose.iter().copied().fold(f64::INFINITY, f64::min),
        dose.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    ];
    let basis = Basis::new(degree, knots, boundaries).map_err(Error::Spline)?;
    let design = dose
        .iter()
        .map(|x| basis.evaluate(*x, true))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Error::Spline)?;
    let mut column_major = vec![0.0; n.checked_mul(p).ok_or(Error::Input)?];
    for i in 0..n {
        for j in 0..p {
            column_major[i + j * n] = design[i][j];
        }
    }
    let qr = Qr::rectangular(column_major, n, p, 1e-7);
    let coefficients = qr.solve(contrast).ok_or(Error::RankDeficient)?;
    let inverse = qr.inverse_crossproduct().ok_or(Error::RankDeficient)?;
    let dot = |a: &[f64], b: &[f64]| a.iter().zip(b).map(|(a, b)| a * b).sum::<f64>();
    let fitted: Vec<_> = design.iter().map(|row| dot(row, &coefficients)).collect();
    let residuals: Vec<_> = contrast.iter().zip(&fitted).map(|(y, f)| y - f).collect();
    let mut curve = Vec::with_capacity(grid.len());
    let mut standard_errors = Vec::with_capacity(grid.len());
    for x in grid {
        let row = basis.evaluate(*x, true).map_err(Error::Spline)?;
        curve.push(dot(&row, &coefficients));
        let projection: Vec<_> = inverse.iter().map(|r| dot(r, &row)).collect();
        // Algebraically identical to mean(IF^2)/n, without an n-by-grid IF matrix.
        let variance = design
            .iter()
            .zip(&residuals)
            .map(|(r, e)| (e * dot(r, &projection)).powi(2))
            .sum::<f64>();
        standard_errors.push(variance.sqrt());
    }
    if curve
        .iter()
        .chain(&standard_errors)
        .chain(&coefficients)
        .chain(&residuals)
        .any(|v| !v.is_finite())
    {
        return Err(Error::Numerical);
    }
    Ok(Fit {
        coefficients,
        fitted,
        residuals,
        curve,
        standard_errors,
        uncertainty: Uncertainty::ConditionalRowInfluence,
        observations: n,
    })
}
