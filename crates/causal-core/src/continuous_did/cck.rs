//! Univariate npiv regression (X == W), cubic quantile splines.
//! Reference: npiv 0.1.3. Shared DGESDD, spline and R RNG implementations.
use crate::continuous_did::{
    lapack_dgesdd::{dgesdd, SvdJob},
    quantile::type_seven_quantile,
    r_rng::RRng,
    spline::Basis,
};
use nalgebra::{DMatrix, DVector};

#[derive(Debug)]
pub enum Error {
    Input,
    Spline(crate::continuous_did::spline::Error),
    Svd,
    Numerical,
    Panel(crate::continuous_did::panel::Error),
    Binary(crate::staggered_did::Error),
    Bootstrap(crate::continuous_did::bootstrap::Error),
}

fn inverse(a: &DMatrix<f64>) -> Result<DMatrix<f64>, Error> {
    let mut values = a.as_slice().to_vec();
    let (info, s) = dgesdd(SvdJob::Some, a.nrows(), a.ncols(), &mut values, a.nrows())
        .map_err(|_| Error::Svd)?;
    if info != 0 {
        return Err(Error::Svd);
    }
    let u = DMatrix::from_column_slice(
        s.left_rows,
        s.left_columns,
        &s.left_vectors.ok_or(Error::Svd)?,
    );
    let vt = DMatrix::from_column_slice(
        s.right_rows,
        s.right_columns,
        &s.right_vectors_transposed.ok_or(Error::Svd)?,
    );
    let cutoff = f64::EPSILON.sqrt() * s.singular_values[0];
    // MASS::ginv default cutoff, not NumPy's default tolerance.
    Ok(vt.transpose()
        * DMatrix::from_fn(s.singular_values.len(), a.nrows(), |i, j| {
            if s.singular_values[i] > cutoff {
                u[(j, i)] / s.singular_values[i]
            } else {
                0.0
            }
        }))
}

fn basis(
    x: &[f64],
    eval: &[f64],
    segments: usize,
    derivative: bool,
) -> Result<DMatrix<f64>, Error> {
    basis_adjusted(x, eval, segments, derivative, true)
}
fn basis_adjusted(
    x: &[f64],
    eval: &[f64],
    segments: usize,
    derivative: bool,
    adjust: bool,
) -> Result<DMatrix<f64>, Error> {
    let mut sorted = x.to_vec();
    sorted.sort_by(f64::total_cmp);
    let range = sorted[sorted.len() - 1] - sorted[0];
    let knots: Vec<_> = (0..=segments)
        .map(|i| {
            type_seven_quantile(&sorted, i as f64 / segments as f64)
                + if adjust {
                    i as f64 / segments as f64 * 1e-10 * range
                } else {
                    0.0
                }
        })
        .collect();
    let b = Basis::new(3, knots[1..segments].to_vec(), [knots[0], knots[segments]])
        .map_err(Error::Spline)?;
    let mut rows = Vec::new();
    for v in eval {
        rows.extend(
            if derivative {
                b.derivative(*v, true)
            } else {
                b.evaluate(*v, true)
            }
            .map_err(Error::Spline)?,
        );
    }
    Ok(DMatrix::from_row_slice(eval.len(), segments + 3, &rows))
}

struct Model {
    h: DVector<f64>,
    deriv: DVector<f64>,
    se: DVector<f64>,
    dse: DVector<f64>,
    beta: DVector<f64>,
    residual: DVector<f64>,
    influence: DMatrix<f64>,
    dinfluence: DMatrix<f64>,
}
fn model(x: &[f64], y: &[f64], eval: &[f64], segments: usize) -> Result<Model, Error> {
    let b = basis(x, x, segments, false)?;
    let e = basis(x, eval, segments, false)?;
    let d = basis(x, eval, segments, true)?;
    let gram = b.transpose() * &b;
    let projected = &gram * inverse(&gram)? * b.transpose();
    let tmp = inverse(&(&projected * &b))? * projected;
    let y = DVector::from_column_slice(y);
    let beta = &tmp * &y;
    let residual = y - &b * &beta;
    let weighted = DMatrix::from_fn(tmp.nrows(), tmp.ncols(), |i, j| tmp[(i, j)] * residual[j]);
    let covariance = &weighted * weighted.transpose();
    let se = |design: &DMatrix<f64>| {
        let product = design * &covariance;
        DVector::from_iterator(
            design.nrows(),
            (0..design.nrows()).map(|i| {
                (0..design.ncols())
                    .map(|j| product[(i, j)] * design[(i, j)])
                    .sum::<f64>()
                    .abs()
                    .sqrt()
            }),
        )
    };
    Ok(Model {
        h: &e * &beta,
        deriv: &d * &beta,
        se: se(&e),
        dse: se(&d),
        beta,
        residual,
        influence: e * &weighted,
        dinfluence: d * &weighted,
    })
}

fn quantile5(values: &[f64], p: f64) -> f64 {
    let mut x = values.to_vec();
    x.sort_by(f64::total_cmp);
    let h = x.len() as f64 * p + 0.5;
    if h <= 1.0 {
        x[0]
    } else if h >= x.len() as f64 {
        x[x.len() - 1]
    } else {
        let j = h.floor() as usize;
        let f = h - j as f64;
        (1.0 - f) * x[j - 1] + f * x[j]
    }
}
fn maximum(influence: &DMatrix<f64>, se: &DVector<f64>, z: &DVector<f64>) -> f64 {
    let draw = influence * z;
    draw.iter()
        .zip(se)
        .map(|(v, s)| (v / s.max(f64::EPSILON)).abs())
        .fold(0.0, f64::max)
}

pub struct Fit {
    pub segments: usize,
    pub h: Vec<f64>,
    pub derivative: Vec<f64>,
    pub se: Vec<f64>,
    pub derivative_se: Vec<f64>,
    pub coefficients: Vec<f64>,
    pub residuals: Vec<f64>,
    pub critical: f64,
    pub derivative_critical: f64,
}

/// Fixed dimension or data-driven CCK dimension selection; no IV instruments.
pub fn fit(
    x: &[f64],
    y: &[f64],
    grid: &[f64],
    segments: Option<usize>,
    iterations: usize,
    alpha: f64,
    seed: u32,
) -> Result<Fit, Error> {
    if x.len() != y.len()
        || x.len() < 8
        || grid.is_empty()
        || x.iter().chain(y).chain(grid).any(|v| !v.is_finite())
        || iterations == 0
        || !(0.0..1.0).contains(&alpha)
        || alpha == 0.0
        || segments == Some(0)
        || iterations
            .checked_mul(x.len())
            .and_then(|n| n.checked_mul(8))
            .is_none()
        || segments.is_some_and(|s| {
            s.checked_add(3)
                .and_then(|p| p.checked_mul(p.max(x.len()).max(grid.len())))
                .and_then(|n| n.checked_mul(8))
                .is_none()
        })
    {
        return Err(Error::Input);
    }
    let low = x.iter().copied().fold(f64::INFINITY, f64::min);
    let high = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !(high - low).is_finite() {
        return Err(Error::Numerical);
    }
    let mut rng = RRng::new(seed);
    let normals: Vec<_> = (0..iterations)
        .map(|_| DVector::from_iterator(x.len(), (0..x.len()).map(|_| rng.normal())))
        .collect();
    let (chosen, candidates, theta) = if let Some(s) = segments {
        (s, vec![s], 0.0)
    } else {
        choose(x, y, grid, &normals)?
    };
    let fitted = model(x, y, x, chosen)?;
    let boot_candidates: Vec<_> = if segments.is_some() {
        vec![chosen]
    } else if candidates.len() > 2 {
        let max = chosen.max(candidates[candidates.len() - 3]);
        candidates.iter().copied().filter(|s| *s <= max).collect()
    } else {
        candidates
    };
    let mut maxima = vec![0.0_f64; iterations];
    let mut dmaxima = maxima.clone();
    for s in boot_candidates {
        let m = model(x, y, x, s)?;
        for (i, z) in normals.iter().enumerate() {
            maxima[i] = maxima[i].max(maximum(&m.influence, &m.se, z));
            dmaxima[i] = dmaxima[i].max(maximum(&m.dinfluence, &m.dse, z));
        }
    }
    let penalty = if segments.is_none() {
        ((chosen + 3) as f64).ln().ln().max(0.0) * theta
    } else {
        0.0
    };
    let critical = quantile5(&maxima, 1.0 - alpha) + penalty;
    let derivative_critical = quantile5(&dmaxima, 1.0 - alpha) + penalty;
    if fitted
        .h
        .iter()
        .chain(fitted.se.iter())
        .chain(fitted.deriv.iter())
        .chain(fitted.dse.iter())
        .any(|v| !v.is_finite())
        || !critical.is_finite()
        || !derivative_critical.is_finite()
    {
        return Err(Error::Numerical);
    }
    Ok(Fit {
        segments: chosen,
        h: fitted.h.as_slice().to_vec(),
        derivative: fitted.deriv.as_slice().to_vec(),
        se: fitted.se.as_slice().to_vec(),
        derivative_se: fitted.dse.as_slice().to_vec(),
        coefficients: fitted.beta.as_slice().to_vec(),
        residuals: fitted.residual.as_slice().to_vec(),
        critical,
        derivative_critical,
    })
}

fn choose(
    x: &[f64],
    y: &[f64],
    grid: &[f64],
    normals: &[DVector<f64>],
) -> Result<(usize, Vec<usize>, f64), Error> {
    let n = x.len() as f64;
    let lmax = n.log2().floor().max(3.0) as usize;
    let mut tests = Vec::new();
    for i in 0..lmax {
        let dimension = (1usize << i) + 3;
        let value = if i <= 1 || tests[i - 2] <= 10.0 * n.sqrt() {
            let s = (0.1 * n.ln()).powi(4).max(1.0);
            dimension as f64 * (dimension as f64).ln().sqrt() * (0.1 * n.ln()).powi(4).max(1.0 / s)
        } else {
            tests[i - 1]
        };
        tests.push(value);
    }
    let count = tests
        .windows(2)
        .position(|w| w[0] <= 10.0 * n.sqrt() && 10.0 * n.sqrt() < w[1])
        .map_or(lmax, |i| i + 1);
    if count < 2 {
        return Err(Error::Input);
    }
    let candidates: Vec<_> = (0..count).map(|i| 1usize << i).collect();
    let maxdim = (candidates[count - 1] + 3) as f64;
    let alpha = (maxdim.ln() / maxdim).sqrt().min(0.5);
    let models: Vec<_> = candidates
        .iter()
        .map(|s| model(x, y, grid, *s))
        .collect::<Result<_, _>>()?;
    let mut tests = Vec::new();
    let mut maxima = vec![0.0_f64; normals.len()];
    for j in 1..count {
        for i in 0..j {
            let a = &models[i];
            let b = &models[j];
            let difference = &a.influence - &b.influence;
            let se = DVector::from_iterator(
                grid.len(),
                (0..grid.len()).map(|r| {
                    let cov = a.influence.row(r).dot(&b.influence.row(r));
                    (a.se[r].powi(2) + b.se[r].powi(2) - 2.0 * cov).sqrt()
                }),
            );
            if se.iter().any(|s| !s.is_finite()) {
                return Err(Error::Numerical);
            }
            let value =
                a.h.iter()
                    .zip(&b.h)
                    .zip(&se)
                    .map(|((a, b), s)| ((a - b) / s.max(f64::EPSILON)).abs())
                    .fold(0.0, f64::max);
            tests.push((i, j, value));
            for (k, z) in normals.iter().enumerate() {
                maxima[k] = maxima[k].max(maximum(&difference, &se, z));
            }
        }
    }
    let theta = quantile5(&maxima, 1.0 - alpha);
    let chosen = (0..count - 1)
        .find(|i| {
            tests
                .iter()
                .filter(|(a, _, _)| a == i)
                .all(|(_, _, v)| *v <= 1.1 * theta)
        })
        .unwrap_or(count - 1);
    Ok((candidates[chosen.min(count - 2)], candidates, theta))
}

pub struct Impact {
    pub dose: Vec<f64>,
    pub level: Vec<f64>,
    pub slope: Vec<f64>,
    pub level_se: Vec<f64>,
    pub slope_se: Vec<f64>,
    pub level_critical: f64,
    pub slope_critical: f64,
    pub overall_level: f64,
    pub overall_level_se: f64,
    pub overall_level_influence: Vec<f64>,
    pub overall_slope: f64,
    pub overall_slope_se: f64,
    pub overall_slope_influence: Vec<f64>,
    /// Upstream assigns att.d_crit.val twice, leaving the slope band uniform.
    pub upstream_mixed_pointwise: bool,
}

/// contdid 0.1.1 CCK wrapper. Exactly two periods and one adoption cohort.
/// Numerical parity retains its fixed 999 npiv draws and default 0.05 alpha.
pub fn impact(
    rows: &[crate::continuous_did::panel::Row],
    iterations: usize,
    alpha: f64,
    bands: crate::continuous_did::bootstrap::Bands,
    seed: u32,
) -> Result<Impact, Error> {
    use crate::continuous_did::panel::{Adoption, Period};
    use std::collections::{BTreeMap, BTreeSet};
    if iterations == 0 || !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
        return Err(Error::Input);
    }
    let observations: Vec<_> = rows.iter().map(|r| r.observation).collect();
    let panel = crate::staggered_did::Panel::new(&observations).map_err(Error::Binary)?;
    let doses = crate::continuous_did::panel::dose_by_unit(rows).map_err(Error::Panel)?;
    let times: Vec<Period> = observations
        .iter()
        .map(|r| r.period)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if times.len() != 2
        || observations
            .iter()
            .any(|r| matches!(r.adoption,Adoption::At(g) if g!=times[1]))
    {
        return Err(Error::Input);
    }
    let outcomes: BTreeMap<_, _> = observations
        .iter()
        .map(|r| ((r.unit, r.period), r.outcome))
        .collect();
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut control = Vec::new();
    for u in panel.units() {
        let dy = outcomes[&(u, times[1])] - outcomes[&(u, times[0])];
        if doses[&u] > 0.0 {
            x.push(doses[&u]);
            y.push(dy);
        } else {
            control.push(dy);
        }
    }
    if x.len() < 8 || control.is_empty() {
        return Err(Error::Input);
    }
    let baseline = control.iter().sum::<f64>() / control.len() as f64;
    for value in &mut y {
        *value -= baseline;
    }
    let min = x.iter().copied().fold(f64::INFINITY, f64::min);
    let max = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let grid: Vec<_> = (0..50)
        .map(|i| min + (max - min) * i as f64 / 49.0)
        .collect();
    let fitted = fit(&x, &y, &grid, None, 999, 0.05, seed)?;
    let b = basis_adjusted(&x, &x, fitted.segments, false, false)?;
    let d = basis_adjusted(&x, &x, fitted.segments, true, false)?;
    let n = x.len();
    let inv = inverse(&(b.transpose() * &b / n as f64))?;
    let mean_derivative = DVector::from_iterator(
        d.ncols(),
        (0..d.ncols()).map(|j| d.column(j).sum() / n as f64),
    );
    let correction = b * inv * mean_derivative;
    let overall_slope = fitted.derivative.iter().sum::<f64>() / n as f64;
    let overall_slope_influence: Vec<_> = (0..n)
        .map(|i| fitted.derivative[i] - overall_slope + (y[i] - fitted.h[i]) * correction[i])
        .collect();
    let mean = overall_slope_influence.iter().sum::<f64>() / n as f64;
    let overall_slope_se = (overall_slope_influence
        .iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>()
        / (n - 1) as f64
        / n as f64)
        .sqrt();
    let binary = crate::staggered_did::cell(
        &panel,
        times[1],
        times[1],
        crate::staggered_did::Controls::NotYetTreated,
        crate::staggered_did::Baseline::Varying,
    )
    .map_err(Error::Binary)?;
    let crate::staggered_did::Cell::Estimated(binary) = binary else {
        return Err(Error::Input);
    };
    // Adaptive npiv uses with_preserve_seed throughout. The binary summary
    // therefore starts at the original seed. For one cohort the group and
    // dynamic scores are identical; preserve all seven calls made by pte.
    let column: Vec<_> = binary.influence.iter().map(|v| vec![*v]).collect();
    let mut stream = crate::continuous_did::bootstrap::Stream::new(seed);
    let mut overall_level_se = 0.0;
    for (i, b) in [
        1000, iterations, iterations, iterations, iterations, iterations, iterations,
    ]
    .into_iter()
    .enumerate()
    {
        let result = stream.run(&column, b, alpha).map_err(Error::Bootstrap)?;
        if i == 3 {
            overall_level_se = result.standard_errors[0];
        }
    }
    let mut order: Vec<_> = (0..n).collect();
    order.sort_by(|i, j| x[*i].total_cmp(&x[*j]));
    let sorted = |values: &[f64]| order.iter().map(|i| values[*i]).collect();
    Ok(Impact {
        dose: sorted(&x),
        level: sorted(&fitted.h),
        slope: sorted(&fitted.derivative),
        level_se: sorted(&fitted.se),
        slope_se: sorted(&fitted.derivative_se),
        level_critical: if bands == crate::continuous_did::bootstrap::Bands::Pointwise {
            crate::continuous_did::r_rng::standard_normal_quantile(1.0 - alpha / 2.0)
        } else {
            fitted.critical
        },
        slope_critical: fitted.derivative_critical,
        overall_level: binary.att,
        overall_level_se,
        overall_level_influence: binary.influence,
        overall_slope,
        overall_slope_se,
        overall_slope_influence,
        upstream_mixed_pointwise: bands == crate::continuous_did::bootstrap::Bands::Pointwise,
    })
}
