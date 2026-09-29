//! contdid::cont_did_acrt, for finite full-rank group-time samples.
//! Preserve the reference's separate implicit boundaries for training and grid bases.
use crate::continuous_did::{r_qr::Qr, spline::Basis};

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    InvalidDose,
    MissingGroup,
    InsufficientRows,
    RankDeficient,
    Spline(crate::continuous_did::spline::Error),
}

#[derive(Debug)]
pub struct Cell {
    pub coefficients: Vec<f64>,
    pub level: Vec<f64>,
    pub slope: Vec<f64>,
    pub average_level: f64,
    pub average_slope: f64,
    pub influence: Vec<f64>,
    pub bread: Vec<Vec<f64>>,
    pub scores: Vec<Vec<f64>>,
}

fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}
fn dot(x: &[f64], y: &[f64]) -> f64 {
    x.iter().zip(y).map(|(x, y)| x * y).sum()
}
fn bounds(x: &[f64]) -> [f64; 2] {
    [
        x.iter().copied().fold(f64::INFINITY, f64::min),
        x.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    ]
}
pub(crate) fn design(basis: &Basis, x: f64, derivative: bool) -> Result<Vec<f64>, Error> {
    let mut row = vec![if derivative { 0.0 } else { 1.0 }];
    row.extend(
        if derivative {
            basis.derivative(x, false)
        } else {
            basis.evaluate(x, false)
        }
        .map_err(Error::Spline)?,
    );
    Ok(row)
}

pub fn fit(
    dose: &[f64],
    change: &[f64],
    grid: &[f64],
    degree: usize,
    knots: Vec<f64>,
) -> Result<Cell, Error> {
    if dose.len() != change.len() || dose.is_empty() || grid.len() < 2 {
        return Err(Error::Shape);
    }
    if dose
        .iter()
        .chain(change)
        .chain(grid)
        .any(|x| !x.is_finite())
    {
        return Err(Error::NonFinite);
    }
    if dose.iter().any(|x| *x < 0.0) {
        return Err(Error::InvalidDose);
    }
    let indices: Vec<_> = dose
        .iter()
        .enumerate()
        .filter_map(|(i, d)| (*d > 0.0).then_some(i))
        .collect();
    let untreated: Vec<_> = dose
        .iter()
        .zip(change)
        .filter_map(|(d, y)| (*d == 0.0).then_some(*y))
        .collect();
    if indices.is_empty() || untreated.is_empty() {
        return Err(Error::MissingGroup);
    }
    let x: Vec<_> = indices.iter().map(|i| dose[*i]).collect();
    let y: Vec<_> = indices.iter().map(|i| change[*i]).collect();
    let training = Basis::new(degree, knots.clone(), bounds(&x)).map_err(Error::Spline)?;
    let evaluation = Basis::new(degree, knots, bounds(grid)).map_err(Error::Spline)?;
    let rows: Vec<_> = x
        .iter()
        .map(|x| design(&training, *x, false))
        .collect::<Result<_, _>>()?;
    let n = rows.len();
    let p = rows[0].len();
    if n <= p {
        return Err(Error::InsufficientRows);
    }
    let mut column_major = vec![0.0; n * p];
    for i in 0..n {
        for j in 0..p {
            column_major[i + j * n] = rows[i][j];
        }
    }
    let qr = Qr::rectangular(column_major, n, p, 1e-7);
    let coefficients = qr.solve(&y).ok_or(Error::RankDeficient)?;
    // Reuse the same QR factor for n*(X'X)^-1, without an n-by-n identity
    // matrix, a second factorization, or a numerical fallback.
    let mut bread = qr.inverse_crossproduct().ok_or(Error::RankDeficient)?;
    for row in &mut bread {
        for value in row {
            *value *= n as f64;
        }
    }
    let scores: Vec<Vec<f64>> = rows
        .iter()
        .zip(&y)
        .map(|(row, y)| {
            let residual = y - dot(row, &coefficients);
            row.iter().map(|x| x * residual).collect()
        })
        .collect();
    let derivatives: Vec<_> = x
        .iter()
        .map(|x| design(&training, *x, true))
        .collect::<Result<_, _>>()?;
    let slopes: Vec<_> = derivatives.iter().map(|r| dot(r, &coefficients)).collect();
    let average_slope = mean(&slopes);
    let mean_derivative: Vec<_> = (0..p)
        .map(|j| derivatives.iter().map(|r| r[j]).sum::<f64>() / n as f64)
        .collect();
    let projected: Vec<_> = bread.iter().map(|r| dot(r, &mean_derivative)).collect();
    let mut influence = vec![0.0; dose.len()];
    for (i, original) in indices.iter().enumerate() {
        influence[*original] = slopes[i] - average_slope + dot(&scores[i], &projected);
    }
    let baseline = mean(&untreated);
    let average_level =
        rows.iter().map(|r| dot(r, &coefficients)).sum::<f64>() / n as f64 - baseline;
    let level = grid
        .iter()
        .map(|x| Ok(dot(&design(&evaluation, *x, false)?, &coefficients) - baseline))
        .collect::<Result<_, Error>>()?;
    let slope = grid
        .iter()
        .map(|x| Ok(dot(&design(&evaluation, *x, true)?, &coefficients)))
        .collect::<Result<_, Error>>()?;
    Ok(Cell {
        coefficients,
        level,
        slope,
        average_level,
        average_slope,
        influence,
        bread,
        scores,
    })
}
