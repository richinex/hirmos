//! Sharp local-linear RD: rdrobust 1.3.0's triangular/NN/mserd default path.
//! No fuzzy treatment, covariate adjustment or alternative bandwidth selector is implied.

use crate::lapack_cholesky::{dpotrf, dpotrs, Triangle};
use nalgebra::{DMatrix, DVector};

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    MissingSide,
    InsufficientSupport,
    Singular,
    InvalidBandwidth,
}

pub struct SharpData {
    left: Vec<(f64, f64)>,
    right: Vec<(f64, f64)>,
}

impl SharpData {
    pub fn new(x: &[f64], y: &[f64], cutoff: f64) -> Result<Self, Error> {
        if x.len() != y.len() || x.is_empty() {
            return Err(Error::Shape);
        }
        if !cutoff.is_finite() || x.iter().chain(y).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let mut rows: Vec<_> = x.iter().zip(y).map(|(x, y)| (x - cutoff, *y)).collect();
        if rows.iter().any(|r| !r.0.is_finite()) { return Err(Error::NonFinite); }
        rows.sort_by(|a, b| a.0.total_cmp(&b.0));
        let split = rows.partition_point(|row| row.0 < 0.0);
        if split == 0 || split == rows.len() {
            return Err(Error::MissingSide);
        }
        Ok(Self {
            left: rows[..split].to_vec(),
            right: rows[split..].to_vec(),
        })
    }
}

pub struct Bandwidth {
    pub estimation: f64,
    pub bias: f64,
}
pub struct Estimate {
    pub value: f64,
    pub standard_error: f64,
    pub interval: [f64; 2],
}
pub struct Fit {
    pub bandwidth: Bandwidth,
    pub conventional: Estimate,
    pub bias_corrected: Estimate,
    pub robust: Estimate,
    pub observations: [usize; 2],
    pub effective_observations: [usize; 2],
    pub left_coefficients: Vec<f64>,
    pub right_coefficients: Vec<f64>,
}

fn weight(x: f64, h: f64) -> f64 {
    (1.0 - (x / h).abs()).max(0.0) / h
}

/// Nearest-neighbor residuals include all tied neighbors, including ties at the boundary.
fn residuals(rows: &[(f64, f64)]) -> Result<Vec<f64>, Error> {
    let n = rows.len();
    if n < 2 {
        return Err(Error::InsufficientSupport);
    }
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let mut left = i;
        let mut right = i;
        while left > 0 && rows[left - 1].0 == rows[i].0 {
            left -= 1;
        }
        while right + 1 < n && rows[right + 1].0 == rows[i].0 {
            right += 1;
        }
        while right - left < 3.min(n - 1) {
            let dl = if left > 0 {
                rows[i].0 - rows[left - 1].0
            } else {
                f64::INFINITY
            };
            let dr = if right + 1 < n {
                rows[right + 1].0 - rows[i].0
            } else {
                f64::INFINITY
            };
            if dl <= dr && left > 0 {
                left -= 1;
                while left > 0 && rows[left - 1].0 == rows[left].0 {
                    left -= 1;
                }
            }
            if dr <= dl && right + 1 < n {
                right += 1;
                while right + 1 < n && rows[right + 1].0 == rows[right].0 {
                    right += 1;
                }
            }
        }
        let neighbors = (right - left) as f64;
        let others = rows[left..=right].iter().map(|r| r.1).sum::<f64>() - rows[i].1;
        result.push((neighbors / (neighbors + 1.0)).sqrt() * (rows[i].1 - others / neighbors));
    }
    Ok(result)
}

struct Polynomial {
    inverse: DMatrix<f64>,
    weighted: DMatrix<f64>,
    beta: DVector<f64>,
}

fn polynomial(rows: &[(f64, f64)], degree: usize, h: f64) -> Result<Polynomial, Error> {
    if !h.is_finite() || h <= 0.0 {
        return Err(Error::InvalidBandwidth);
    }
    let design = DMatrix::from_fn(rows.len(), degree + 1, |i, j| rows[i].0.powi(j as i32));
    let scaled = DMatrix::from_fn(rows.len(), degree + 1, |i, j| {
        design[(i, j)] * weight(rows[i].0, h).sqrt()
    });
    let k = degree + 1;
    let mut gram = scaled.transpose() * scaled;
    let info = dpotrf(Triangle::Lower, k, gram.as_mut_slice(), k).map_err(|_| Error::Singular)?;
    if info != 0 {
        return Err(Error::InsufficientSupport);
    }
    let mut inverse = DMatrix::identity(k, k);
    dpotrs(
        Triangle::Lower,
        k,
        k,
        gram.as_slice(),
        k,
        inverse.as_mut_slice(),
        k,
    )
    .map_err(|_| Error::Singular)?;
    let weighted = DMatrix::from_fn(rows.len(), degree + 1, |i, j| {
        design[(i, j)] * weight(rows[i].0, h)
    });
    let y = DVector::from_iterator(rows.len(), rows.iter().map(|r| r.1));
    let beta = &inverse * weighted.transpose() * y;
    Ok(Polynomial {
        inverse,
        weighted,
        beta,
    })
}

fn variance(
    inverse: &DMatrix<f64>,
    weighted: &DMatrix<f64>,
    residual: &[f64],
    derivative: usize,
) -> f64 {
    let influence = inverse.row(derivative) * weighted.transpose();
    influence
        .iter()
        .zip(residual)
        .map(|(w, r)| (w * r).powi(2))
        .sum()
}

struct Pilot {
    variance: f64,
    bias: f64,
    regularization: f64,
    rate: f64,
}

fn pilot(
    rows: &[(f64, f64)],
    order: usize,
    derivative: usize,
    bias_order: usize,
    variance_bandwidth: f64,
    bias_bandwidth: f64,
    regularize: bool,
) -> Result<Pilot, Error> {
    let vrows: Vec<_> = rows
        .iter()
        .copied()
        .filter(|r| weight(r.0, variance_bandwidth) > 0.0)
        .collect();
    let vfit = polynomial(&vrows, order, variance_bandwidth)?;
    let vv = variance(
        &vfit.inverse,
        &vfit.weighted,
        &residuals(&vrows)?,
        derivative,
    );
    let power = DVector::from_iterator(
        vrows.len(),
        vrows
            .iter()
            .map(|r| (r.0 / variance_bandwidth).powi((order + 1) as i32)),
    );
    let constant = (vfit.inverse * vfit.weighted.transpose() * power)[derivative]
        * variance_bandwidth.powi(derivative as i32);
    let brows: Vec<_> = rows
        .iter()
        .copied()
        .filter(|r| weight(r.0, bias_bandwidth) > 0.0)
        .collect();
    let bfit = polynomial(&brows, bias_order, bias_bandwidth)?;
    let regularization = if regularize {
        3.0 * constant.powi(2)
            * variance(
                &bfit.inverse,
                &bfit.weighted,
                &residuals(&brows)?,
                bias_order,
            )
    } else {
        0.0
    };
    let factor = (2 * (order + 1 - derivative)) as f64;
    Ok(Pilot {
        variance: (2 * derivative + 1) as f64
            * variance_bandwidth.powi((2 * derivative + 1) as i32)
            * vv,
        bias: factor.sqrt() * constant * bfit.beta[bias_order],
        regularization: factor * regularization,
        rate: 1.0 / (2 * order + 3) as f64,
    })
}

pub fn select_bandwidth(data: &SharpData) -> Result<Bandwidth, Error> {
    let xs: Vec<_> = data.left.iter().chain(&data.right).map(|r| r.0).collect();
    let n = xs.len();
    let mean = xs.iter().sum::<f64>() / n as f64;
    let sd = (xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt();
    let percentile = |p| crate::causal_effects::numpy_percentile(&xs, p);
    let mut unique = xs.clone();
    unique.dedup();
    let range_l = data.left[0].0.abs();
    let range_r = data.right.last().ok_or(Error::MissingSide)?.0;
    let maximum = range_l.max(range_r);
    let mut initial = (2.576
        * sd.min((percentile(0.75) - percentile(0.25)) / 1.349)
        * (unique.len() as f64).powf(-0.2))
    .min(maximum);
    let unique_side = |rows: &[(f64, f64)]| {
        let mut xs: Vec<_> = rows.iter().map(|r| r.0.abs()).collect();
        xs.sort_by(f64::total_cmp);
        xs.dedup();
        xs
    };
    let ul = unique_side(&data.left);
    let ur = unique_side(&data.right);
    let masspoints = 1.0 - ul.len() as f64 / data.left.len() as f64 >= 0.1
        || 1.0 - ur.len() as f64 / data.right.len() as f64 >= 0.1;
    let minimum = if masspoints {
        (ul[9.min(ul.len() - 1)] + 1e-8).max(ur[9.min(ur.len() - 1)] + 1e-8)
    } else {
        0.0
    };
    initial = initial.max(minimum);
    let combine = |l: Pilot, r: Pilot| {
        let value = ((l.variance + r.variance)
            / ((r.bias - l.bias).powi(2) + l.regularization + r.regularization))
            .powf(l.rate);
        if value.is_nan() || value <= 0.0 { Err(Error::InvalidBandwidth) } else { Ok(value.min(maximum)) }
    };
    let d = combine(
        pilot(&data.left, 3, 3, 4, initial, range_l, false)?,
        pilot(&data.right, 3, 3, 4, initial, range_r, false)?,
    )?
    .max(minimum);
    let b = combine(
        pilot(&data.left, 2, 2, 3, initial, d, true)?,
        pilot(&data.right, 2, 2, 3, initial, d, true)?,
    )?;
    let h = combine(
        pilot(&data.left, 1, 0, 2, initial, b, true)?,
        pilot(&data.right, 1, 0, 2, initial, b, true)?,
    )?;
    if !h.is_finite() || !b.is_finite() || h <= 0.0 || b <= 0.0 {
        return Err(Error::InvalidBandwidth);
    }
    Ok(Bandwidth {
        estimation: h,
        bias: b,
    })
}

struct Side {
    value: f64,
    corrected: f64,
    variance: f64,
    robust_variance: f64,
    coefficients: Vec<f64>,
    effective: usize,
}

fn side(rows: &[(f64, f64)], h: f64, b: f64) -> Result<Side, Error> {
    let rows: Vec<_> = rows
        .iter()
        .copied()
        .filter(|r| weight(r.0, h.max(b)) > 0.0)
        .collect();
    let p = polynomial(&rows, 1, h)?;
    let q = polynomial(&rows, 2, b)?;
    let moment = &p.weighted.transpose()
        * DVector::from_iterator(rows.len(), rows.iter().map(|r| r.0.powi(2)));
    let correction = moment * (q.inverse.row(2) * q.weighted.transpose());
    let corrected_weights = &p.weighted - correction.transpose();
    let corrected = (&p.inverse
        * corrected_weights.transpose()
        * DVector::from_iterator(rows.len(), rows.iter().map(|r| r.1)))[0];
    let residual = residuals(&rows)?;
    Ok(Side {
        value: p.beta[0],
        corrected,
        variance: variance(&p.inverse, &p.weighted, &residual, 0),
        robust_variance: variance(&p.inverse, &corrected_weights, &residual, 0),
        coefficients: p.beta.as_slice().to_vec(),
        effective: rows.iter().filter(|r| weight(r.0, h) > 0.0).count(),
    })
}

pub fn fit(data: &SharpData) -> Result<Fit, Error> {
    let bandwidth = select_bandwidth(data)?;
    let l = side(&data.left, bandwidth.estimation, bandwidth.bias)?;
    let r = side(&data.right, bandwidth.estimation, bandwidth.bias)?;
    let estimate = |value: f64, variance: f64| {
        let se = variance.sqrt();
        let z = spec_math::cephes64::ndtri(0.975);
        Estimate {
            value,
            standard_error: se,
            interval: [value - z * se, value + z * se],
        }
    };
    Ok(Fit {
        conventional: estimate(r.value - l.value, l.variance + r.variance),
        bias_corrected: estimate(r.corrected - l.corrected, l.variance + r.variance),
        robust: estimate(
            r.corrected - l.corrected,
            l.robust_variance + r.robust_variance,
        ),
        bandwidth,
        observations: [data.left.len(), data.right.len()],
        effective_observations: [l.effective, r.effective],
        left_coefficients: l.coefficients,
        right_coefficients: r.coefficients,
    })
}
