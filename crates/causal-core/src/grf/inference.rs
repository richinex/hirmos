// Copyright (c) 2024 GRF Contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
// GRF get_scores / average_treatment_effect / forest_summary.
use crate::ols::Ols;
use crate::synthetic_control::t_cdf;
use nalgebra::{DMatrix, DVector};
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub enum Covariance {
    Hc0,
    Hc1,
    Hc2,
    Hc3,
}
pub struct LinearSummary {
    pub estimates: Vec<f64>,
    pub standard_errors: Vec<f64>,
    pub statistics: Vec<f64>,
    pub p_values: Vec<f64>,
    pub df: usize,
}
#[derive(Debug)]
pub struct Average {
    pub estimate: f64,
    pub standard_error: f64,
}
#[derive(Clone, Copy)]
pub enum Target {
    All,
    Treated,
    Control,
}
pub struct Observations<'a> {
    pub y: &'a [f64],
    pub w: &'a [f64],
    pub y_hat: &'a [f64],
    pub w_hat: &'a [f64],
    pub tau: &'a [f64],
}
impl Observations<'_> {
    fn check(&self) -> Result<usize, &'static str> {
        let n = self.y.len();
        if n < 2
            || [self.w, self.y_hat, self.w_hat, self.tau]
                .iter()
                .any(|v| v.len() != n)
            || [self.y, self.w, self.y_hat, self.w_hat, self.tau]
                .iter()
                .any(|v| v.iter().any(|x| !x.is_finite()))
        {
            return Err("Invalid inference observations.");
        }
        Ok(n)
    }
}
fn groups(labels: &[usize]) -> BTreeMap<usize, Vec<usize>> {
    let mut groups = BTreeMap::<usize, Vec<usize>>::new();
    for (i, &g) in labels.iter().enumerate() {
        groups.entry(g).or_default().push(i);
    }
    groups
}
pub fn observation_weights(
    labels: &[usize],
    weights: Option<&[f64]>,
    equalize: bool,
) -> Result<Vec<f64>, &'static str> {
    if labels.is_empty()
        || weights.is_some_and(|w| {
            w.len() != labels.len() || w.iter().any(|v| !v.is_finite() || *v < 0.0)
        })
        || equalize && weights.is_some()
    {
        return Err("Invalid observation or cluster weights.");
    }
    let mut result = if equalize {
        let sizes = groups(labels);
        labels
            .iter()
            .map(|g| 1.0 / sizes[g].len() as f64)
            .collect::<Vec<_>>()
    } else {
        weights
            .map(|v| v.to_vec())
            .unwrap_or(vec![1.0; labels.len()])
    };
    let mass: f64 = result.iter().sum();
    if mass <= 0.0 {
        return Err("Observation weights have no positive mass.");
    }
    for v in &mut result {
        *v /= mass;
    }
    Ok(result)
}
pub fn scores(
    data: &Observations<'_>,
    debiasing: Option<&[f64]>,
) -> Result<Vec<f64>, &'static str> {
    let n = data.check()?;
    if debiasing.is_some_and(|v| v.len() != n || v.iter().any(|x| !x.is_finite())) {
        return Err("Invalid debiasing weights.");
    }
    if debiasing.is_none() && data.w.iter().any(|v| *v != 0.0 && *v != 1.0) {
        return Err("Continuous treatment requires conditional-variance debiasing weights.");
    }
    (0..n)
        .map(|i| {
            let d = match debiasing {
                Some(v) => v[i],
                None => {
                    let p = data.w_hat[i];
                    if p <= 0.0 || p >= 1.0 {
                        return Err("Binary propensity must lie strictly between zero and one.");
                    }
                    (data.w[i] - p) / (p * (1.0 - p))
                }
            };
            let residual = data.y[i] - (data.y_hat[i] + data.tau[i] * (data.w[i] - data.w_hat[i]));
            let value = data.tau[i] + d * residual;
            if value.is_finite() {
                Ok(value)
            } else {
                Err("Non-finite doubly robust score.")
            }
        })
        .collect()
}
pub fn average_scores(
    scores: &[f64],
    weights: &[f64],
    labels: &[usize],
) -> Result<Average, &'static str> {
    if scores.len() < 2
        || weights.len() != scores.len()
        || labels.len() != scores.len()
        || scores.iter().any(|v| !v.is_finite())
        || weights.iter().any(|v| !v.is_finite() || *v < 0.0)
    {
        return Err("Invalid average-effect inputs.");
    }
    let mass: f64 = weights.iter().sum();
    if mass <= 0.0 {
        return Err("Average-effect sample has no positive weight.");
    }
    let estimate = scores.iter().zip(weights).map(|(v, w)| v * w).sum::<f64>() / mass;
    let mut square = 0.0;
    let mut count = 0;
    for rows in groups(labels).values() {
        let gmass: f64 = rows.iter().map(|&i| weights[i]).sum();
        if gmass > 0.0 {
            count += 1;
        }
        let s: f64 = rows
            .iter()
            .map(|&i| (scores[i] - estimate) * weights[i])
            .sum();
        square += s * s;
    }
    if count < 2 {
        return Err("Inference requires more than one positive-weight cluster.");
    }
    Ok(Average {
        estimate,
        standard_error: (square / (mass * mass) * count as f64 / (count - 1) as f64).sqrt(),
    })
}
pub fn aipw(
    data: &Observations<'_>,
    weights: &[f64],
    labels: &[usize],
    target: Target,
    debiasing: Option<&[f64]>,
) -> Result<Average, &'static str> {
    let n = data.check()?;
    if matches!(target, Target::All) {
        return average_scores(&scores(data, debiasing)?, weights, labels);
    }
    if weights.len() != n
        || labels.len() != n
        || weights.iter().any(|v| !v.is_finite() || *v < 0.0)
        || data.w.iter().any(|w| *w != 0.0 && *w != 1.0)
        || data.w_hat.iter().any(|p| *p <= 0.0 || *p >= 1.0)
    {
        return Err(
            "Treated/control inference requires binary treatment and interior propensities.",
        );
    }
    let arm = if matches!(target, Target::Treated) {
        1.0
    } else {
        0.0
    };
    let mass: f64 = weights.iter().sum();
    let arm_mass: f64 = (0..n)
        .filter(|&i| data.w[i] == arm)
        .map(|i| weights[i])
        .sum();
    if mass <= arm_mass || arm_mass <= 0.0 {
        return Err("Both treatment arms need positive weight.");
    }
    let raw = (0..n)
        .filter(|&i| data.w[i] == arm)
        .map(|i| data.tau[i] * weights[i])
        .sum::<f64>()
        / arm_mass;
    let raw_var = (0..n)
        .filter(|&i| data.w[i] == arm)
        .map(|i| weights[i].powi(2) * (data.tau[i] - raw).powi(2))
        .sum::<f64>()
        / arm_mass.powi(2);
    let gamma: Vec<_> = (0..n)
        .map(|i| {
            if data.w[i] == arm {
                1.0
            } else if arm == 1.0 {
                data.w_hat[i] / (1.0 - data.w_hat[i])
            } else {
                (1.0 - data.w_hat[i]) / data.w_hat[i]
            }
        })
        .collect();
    let mut norm = [0.0; 2];
    for i in 0..n {
        norm[data.w[i] as usize] += weights[i] * gamma[i];
    }
    if norm.iter().any(|v| *v <= 0.0) {
        return Err("Invalid treatment-arm normalization.");
    }
    let correction: Vec<_> = (0..n)
        .map(|i| {
            let fitted = if data.w[i] == 1.0 {
                data.y_hat[i] + (1.0 - data.w_hat[i]) * data.tau[i]
            } else {
                data.y_hat[i] - data.w_hat[i] * data.tau[i]
            };
            (2.0 * data.w[i] - 1.0) * gamma[i] / norm[data.w[i] as usize]
                * mass
                * (data.y[i] - fitted)
        })
        .collect();
    let adjustment = correction
        .iter()
        .zip(weights)
        .map(|(c, w)| c * w)
        .sum::<f64>()
        / mass;
    let mut sum = 0.0;
    let mut count = 0;
    for rows in groups(labels).values() {
        if rows.iter().any(|&i| weights[i] > 0.0) {
            count += 1;
        }
        let value = rows
            .iter()
            .map(|&i| correction[i] * weights[i])
            .sum::<f64>();
        sum += value * value;
    }
    if count < 2 {
        return Err("Inference requires more than one positive-weight cluster.");
    }
    Ok(Average {
        estimate: raw + adjustment,
        standard_error: (raw_var + sum / mass.powi(2) * count as f64 / (count - 1) as f64).sqrt(),
    })
}

/// Weighted lm + one-way sandwich::vcovCL. Reuses the existing LAPACK/SVD OLS.
pub fn linear_summary(
    x: &DMatrix<f64>,
    y: &[f64],
    weights: &[f64],
    labels: &[usize],
    covariance: Covariance,
    one_sided: bool,
) -> Result<LinearSummary, &'static str> {
    let (n, p) = (x.nrows(), x.ncols());
    if n <= p
        || p == 0
        || y.len() != n
        || weights.len() != n
        || labels.len() != n
        || x.iter().chain(y).any(|v| !v.is_finite())
        || weights.iter().any(|w| !w.is_finite() || *w <= 0.0)
    {
        return Err("Invalid regression inference dimensions or nonpositive weights.");
    }
    let wx = DMatrix::from_fn(n, p, |i, j| x[(i, j)] * weights[i].sqrt());
    let wy = DVector::from_iterator(n, y.iter().zip(weights).map(|(v, w)| v * w.sqrt()));
    let fit = Ols::try_fit(&wx, &wy).map_err(|_| "Inference regression failed.")?;
    if fit.rank < p {
        return Err("Inference design is rank deficient.");
    }
    let bread = fit.xtx_inverse();
    let residual = DVector::from_column_slice(y) - x * &fit.params;
    let clusters = groups(labels);
    let g = clusters.len();
    if g < 2 {
        return Err("At least two clusters are required.");
    }
    let mut meat = DMatrix::<f64>::zeros(p, p);
    for rows in clusters.values() {
        let m = rows.len();
        let xx = DMatrix::from_fn(m, p, |i, j| x[(rows[i], j)]);
        let mut rr = DVector::from_iterator(m, rows.iter().map(|&i| weights[i] * residual[i]));
        match covariance {
            Covariance::Hc0 | Covariance::Hc1 => {}
            Covariance::Hc3 => {
                let hat = &xx
                    * &bread
                    * xx.transpose()
                    * DMatrix::from_diagonal(&DVector::from_iterator(
                        m,
                        rows.iter().map(|&i| weights[i]),
                    ));
                rr = (DMatrix::<f64>::identity(m, m) - hat)
                    .lu()
                    .solve(&rr)
                    .ok_or("Singular cluster leverage correction.")?;
            }
            Covariance::Hc2 => {
                let z = DMatrix::from_fn(m, p, |i, j| xx[(i, j)] * weights[rows[i]].sqrt());
                let eigen = (DMatrix::<f64>::identity(m, m) - &z * &bread * z.transpose())
                    .symmetric_eigen();
                if eigen.eigenvalues.iter().any(|v| *v <= 0.0) {
                    return Err("Singular cluster leverage correction.");
                }
                let inverse = &eigen.eigenvectors
                    * DMatrix::from_diagonal(&eigen.eigenvalues.map(|v| 1.0 / v.sqrt()))
                    * eigen.eigenvectors.transpose();
                for i in 0..m {
                    rr[i] *= weights[rows[i]].sqrt();
                }
                rr = inverse * rr;
                for i in 0..m {
                    rr[i] /= weights[rows[i]].sqrt();
                }
            }
        }
        let score = xx.transpose() * rr;
        meat += &score * score.transpose();
    }
    let adjustment = match covariance {
        Covariance::Hc0 => g as f64 / (g - 1) as f64,
        Covariance::Hc1 => g as f64 / (g - 1) as f64 * (n - 1) as f64 / (n - p) as f64,
        _ => 1.0,
    };
    let cov = &bread * meat * &bread * adjustment;
    let se: Vec<_> = (0..p).map(|i| cov[(i, i)].sqrt()).collect();
    let statistics: Vec<_> = (0..p).map(|i| fit.params[i] / se[i]).collect();
    let p_values = statistics
        .iter()
        .map(|v| {
            if one_sided {
                t_cdf(-*v, (n - p) as f64)
            } else {
                2.0 * t_cdf(-v.abs(), (n - p) as f64)
            }
        })
        .collect();
    Ok(LinearSummary {
        estimates: fit.params.as_slice().to_vec(),
        standard_errors: se,
        statistics,
        p_values,
        df: n - p,
    })
}
pub fn calibration(
    data: &Observations<'_>,
    weights: &[f64],
    labels: &[usize],
    cov: Covariance,
) -> Result<LinearSummary, &'static str> {
    let n = data.check()?;
    if weights.len() != n {
        return Err("Invalid calibration weights.");
    }
    let mean = data
        .tau
        .iter()
        .zip(weights)
        .map(|(v, w)| v * w)
        .sum::<f64>()
        / weights.iter().sum::<f64>();
    let x = DMatrix::from_fn(n, 2, |i, j| {
        (data.w[i] - data.w_hat[i]) * if j == 0 { mean } else { data.tau[i] - mean }
    });
    let y: Vec<_> = data.y.iter().zip(data.y_hat).map(|(a, b)| a - b).collect();
    linear_summary(&x, &y, weights, labels, cov, true)
}
pub fn projection(
    scores: &[f64],
    covariates: &[Vec<f64>],
    weights: &[f64],
    labels: &[usize],
    cov: Covariance,
) -> Result<LinearSummary, &'static str> {
    if covariates.iter().any(|c| c.len() != scores.len()) {
        return Err("Invalid projection covariates.");
    }
    let x = DMatrix::from_fn(scores.len(), covariates.len() + 1, |i, j| {
        if j == 0 {
            1.0
        } else {
            covariates[j - 1][i]
        }
    });
    linear_summary(&x, scores, weights, labels, cov, false)
}
pub fn overlap(
    data: &Observations<'_>,
    weights: &[f64],
    labels: &[usize],
    clustered: bool,
) -> Result<Average, &'static str> {
    let n = data.check()?;
    let x = DMatrix::from_fn(n, 2, |i, j| {
        if j == 0 {
            1.0
        } else {
            data.w[i] - data.w_hat[i]
        }
    });
    let y: Vec<_> = data.y.iter().zip(data.y_hat).map(|(a, b)| a - b).collect();
    let summary = linear_summary(
        &x,
        &y,
        weights,
        labels,
        if clustered {
            Covariance::Hc1
        } else {
            Covariance::Hc3
        },
        false,
    )?;
    Ok(Average {
        estimate: summary.estimates[1],
        standard_error: summary.standard_errors[1],
    })
}

/// GRF's binary, unweighted, unclustered targeted update. The absence of weight
/// and cluster arguments is intentional: upstream rejects those TMLE variants.
pub fn tmle(data: &Observations<'_>, target: Target) -> Result<Average, &'static str> {
    let n = data.check()?;
    if data.w.iter().any(|w| *w != 0.0 && *w != 1.0)
        || data.w_hat.iter().any(|p| *p <= 0.0 || *p >= 1.0)
    {
        return Err("TMLE requires binary treatment and interior propensities.");
    }
    let arms: [Vec<usize>; 2] =
        std::array::from_fn(|arm| (0..n).filter(|&i| data.w[i] == arm as f64).collect());
    if arms.iter().any(|rows| rows.len() < 2) {
        return Err("TMLE requires at least two observations in each treatment arm.");
    }
    let residual = |i: usize| data.y[i] - data.y_hat[i] - (data.w[i] - data.w_hat[i]) * data.tau[i];
    let fluctuation = |arm: usize, a: &dyn Fn(usize) -> f64| {
        let rows = &arms[arm];
        let x = DMatrix::from_fn(rows.len(), 1, |i, _| a(rows[i]));
        let y: Vec<_> = rows.iter().map(|&i| residual(i)).collect();
        linear_summary(
            &x,
            &y,
            &vec![1.0; rows.len()],
            &(0..rows.len()).collect::<Vec<_>>(),
            Covariance::Hc3,
            false,
        )
    };
    let retained: Vec<_> = match target {
        Target::All => (0..n).collect(),
        Target::Treated => arms[1].clone(),
        Target::Control => arms[0].clone(),
    };
    let count = retained.len() as f64;
    let raw = retained.iter().map(|&i| data.tau[i]).sum::<f64>() / count;
    let raw_var = retained
        .iter()
        .map(|&i| (data.tau[i] - raw).powi(2))
        .sum::<f64>()
        / count.powi(2);
    let (correction, variance) = match target {
        Target::All => {
            let a0 = |i: usize| 1.0 / (1.0 - data.w_hat[i]);
            let a1 = |i: usize| 1.0 / data.w_hat[i];
            let f0 = fluctuation(0, &a0)?;
            let f1 = fluctuation(1, &a1)?;
            let c0 = (0..n).map(a0).sum::<f64>() / n as f64;
            let c1 = (0..n).map(a1).sum::<f64>() / n as f64;
            (
                f1.estimates[0] * c1 - f0.estimates[0] * c0,
                (f0.standard_errors[0] * c0).powi(2) + (f1.standard_errors[0] * c1).powi(2),
            )
        }
        Target::Treated | Target::Control => {
            let treated = matches!(target, Target::Treated);
            let a = |i: usize| {
                if treated {
                    data.w_hat[i] / (1.0 - data.w_hat[i])
                } else {
                    (1.0 - data.w_hat[i]) / data.w_hat[i]
                }
            };
            let f = fluctuation(if treated { 0 } else { 1 }, &a)?;
            let center = retained.iter().map(|&i| a(i)).sum::<f64>() / count;
            let mean_residual = retained.iter().map(|&i| residual(i)).sum::<f64>() / count;
            let residual_var = retained
                .iter()
                .map(|&i| (residual(i) - mean_residual).powi(2))
                .sum::<f64>()
                / (count * (count - 1.0));
            (
                (if treated { -1.0 } else { 1.0 }) * f.estimates[0] * center,
                (f.standard_errors[0] * center).powi(2) + residual_var,
            )
        }
    };
    Ok(Average {
        estimate: raw + correction,
        standard_error: (raw_var + variance).sqrt(),
    })
}

/// The upstream projection's overlap target drops effectively zero overlap
/// before computing scores and applies propensity overlap weights afterwards.
pub fn overlap_projection(
    data: &Observations<'_>,
    covariates: &[Vec<f64>],
    weights: &[f64],
    labels: &[usize],
    covariance: Covariance,
    debiasing: Option<&[f64]>,
) -> Result<LinearSummary, &'static str> {
    let n = data.check()?;
    if weights.len() != n
        || labels.len() != n
        || covariates.iter().any(|v| v.len() != n)
        || debiasing.is_some_and(|v| v.len() != n)
    {
        return Err("Invalid overlap projection dimensions.");
    }
    let rows: Vec<_> = (0..n)
        .filter(|&i| data.w_hat[i] * (1.0 - data.w_hat[i]) > f64::EPSILON)
        .collect();
    let select = |v: &[f64]| rows.iter().map(|&i| v[i]).collect::<Vec<_>>();
    let y = select(data.y);
    let w = select(data.w);
    let yh = select(data.y_hat);
    let wh = select(data.w_hat);
    let tau = select(data.tau);
    let selected = Observations {
        y: &y,
        w: &w,
        y_hat: &yh,
        w_hat: &wh,
        tau: &tau,
    };
    let d = debiasing.map(select);
    let scores = scores(&selected, d.as_deref())?;
    let x = covariates.iter().map(|v| select(v)).collect::<Vec<_>>();
    let weights = rows
        .iter()
        .map(|&i| weights[i] * data.w_hat[i] * (1.0 - data.w_hat[i]))
        .collect::<Vec<_>>();
    let labels = rows.iter().map(|&i| labels[i]).collect::<Vec<_>>();
    projection(&scores, &x, &weights, &labels, covariance)
}
