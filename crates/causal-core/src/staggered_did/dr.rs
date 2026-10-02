//! DRDID 1.3.0 traditional panel score and nuisance-estimation corrections.
use crate::{lapack_cholesky as chol, lapack_lu as lu};
use nalgebra::DMatrix;

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    MissingGroup,
    Weight,
    Probability,
    Singular,
    Numerical,
    Overlap,
}

pub struct Sample {
    x: DMatrix<f64>,
    change: Vec<f64>,
    treated: Vec<bool>,
    weights: Vec<f64>,
}

impl Sample {
    /// The design includes its intercept, matching DRDID's matrix interface.
    pub fn new(
        x: DMatrix<f64>,
        change: Vec<f64>,
        treated: Vec<bool>,
        weights: Vec<f64>,
    ) -> Result<Self, Error> {
        let n = x.nrows();
        if n == 0 || x.ncols() == 0 || change.len() != n || treated.len() != n || weights.len() != n
        {
            return Err(Error::Shape);
        }
        if x.iter()
            .chain(&change)
            .chain(&weights)
            .any(|v| !v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        if !treated.contains(&true) || !treated.contains(&false) {
            return Err(Error::MissingGroup);
        }
        let mean = weights.iter().sum::<f64>() / n as f64;
        if weights.iter().any(|w| *w < 0.0) || mean <= 0.0 || !mean.is_finite() {
            return Err(Error::Weight);
        }
        let weights: Vec<_> = weights.iter().map(|w| w / mean).collect();
        for group in [true, false] {
            if treated
                .iter()
                .zip(&weights)
                .filter(|(d, _)| **d == group)
                .map(|(_, w)| w)
                .sum::<f64>()
                <= 0.0
            {
                return Err(Error::Weight);
            }
        }
        Ok(Self {
            x,
            change,
            treated,
            weights,
        })
    }
}

#[derive(Debug)]
pub struct Score {
    pub att: f64,
    pub se: f64,
    pub influence: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FitStatus {
    Converged { iterations: usize },
    IterationLimit,
}

pub struct Fit {
    pub score: Score,
    pub propensity: Vec<f64>,
    pub outcome: Vec<f64>,
    pub coefficients: Vec<f64>,
    pub status: FitStatus,
}

/// Binomial IRLS with the reference initialization, deviance criterion and
/// step halving. Positive definite systems use the existing LAPACK solve;
/// the control outcome regression uses the shared DGELSD least-squares path.
fn propensity_fit(sample: &Sample) -> Result<(Vec<f64>, Vec<f64>, FitStatus), Error> {
    let n = sample.x.nrows();
    let p = sample.x.ncols();
    let y: Vec<_> = sample
        .treated
        .iter()
        .map(|d| if *d { 1.0 } else { 0.0 })
        .collect();
    let mut mu: Vec<_> = (0..n)
        .map(|i| (sample.weights[i] * y[i] + 0.5) / (sample.weights[i] + 1.0))
        .collect();
    let mut eta: Vec<_> = mu.iter().map(|m| (m / (1.0 - m)).ln()).collect();
    let deviance = |mu: &[f64]| -> f64 {
        (0..n)
            .map(|i| {
                -2.0 * sample.weights[i]
                    * if y[i] == 1.0 {
                        mu[i].ln()
                    } else {
                        (1.0 - mu[i]).ln()
                    }
            })
            .sum()
    };
    let mut dev = deviance(&mu);
    let mut coefficients = vec![0.0; p];
    let mut status = FitStatus::IterationLimit;
    let predict = |b: &[f64]| -> (Vec<f64>, Vec<f64>) {
        let eta: Vec<_> = (0..n)
            .map(|i| (0..p).map(|j| sample.x[(i, j)] * b[j]).sum::<f64>())
            .collect();
        let mu = eta
            .iter()
            .map(|v| {
                let e = if *v < -30.0 {
                    f64::EPSILON
                } else if *v > 30.0 {
                    1.0 / f64::EPSILON
                } else {
                    v.exp()
                };
                e / (1.0 + e)
            })
            .collect();
        (eta, mu)
    };
    for iteration in 0..100 {
        let old = coefficients.clone();
        let old_dev = dev;
        let mut cross = DMatrix::zeros(p, p);
        let mut rhs = DMatrix::zeros(p, 1);
        for i in 0..n {
            let e = eta[i].exp();
            let derivative = if eta[i].abs() > 30.0 {
                f64::EPSILON
            } else {
                e / (1.0 + e).powi(2)
            };
            let w = sample.weights[i] * derivative.powi(2) / (mu[i] * (1.0 - mu[i]));
            let z = eta[i] + (y[i] - mu[i]) / derivative;
            for j in 0..p {
                rhs[(j, 0)] += w * sample.x[(i, j)] * z;
                for k in 0..p {
                    cross[(j, k)] += w * sample.x[(i, j)] * sample.x[(i, k)];
                }
            }
        }
        coefficients = (inverse(&cross, true)? * rhs)
            .column(0)
            .iter()
            .copied()
            .collect();
        (eta, mu) = predict(&coefficients);
        dev = deviance(&mu);
        if iteration > 0 && (dev - old_dev) / (0.1 + dev.abs()) >= 1e-8 {
            for _ in 0..100 {
                if (dev - old_dev) / (0.1 + dev.abs()) < -1e-8 {
                    break;
                }
                for j in 0..p {
                    coefficients[j] = 0.5 * (coefficients[j] + old[j]);
                }
                (eta, mu) = predict(&coefficients);
                dev = deviance(&mu);
            }
        }
        if !dev.is_finite() {
            return Err(Error::Numerical);
        }
        if (dev - old_dev).abs() / (0.1 + dev.abs()) < 1e-8 {
            status = FitStatus::Converged {
                iterations: iteration + 1,
            };
            break;
        }
    }
    Ok((mu, coefficients, status))
}

/// did performs these unweighted checks before DRDID's weighted fit.
pub fn panel_guard(sample: &Sample) -> Result<(), Error> {
    panel_guard_for(sample, super::AdjustmentMethod::DoublyRobust)
}

pub fn panel_guard_for(sample: &Sample, method: super::AdjustmentMethod) -> Result<(), Error> {
    if method != super::AdjustmentMethod::OutcomeRegression {
        let unweighted = Sample::new(
            sample.x.clone(),
            sample.change.clone(),
            sample.treated.clone(),
            vec![1.0; sample.x.nrows()],
        )?;
        let (propensity, _, _) = propensity_fit(&unweighted)?;
        if propensity.iter().any(|p| *p >= 0.999) {
            return Err(Error::Overlap);
        }
    }
    if method != super::AdjustmentMethod::InverseProbability {
        let x = DMatrix::from_fn(sample.x.nrows(), sample.x.ncols(), |i, j| {
            if sample.treated[i] {
                0.0
            } else {
                sample.x[(i, j)]
            }
        });
        inverse(&(x.transpose() * x), false)?;
    }
    Ok(())
}

pub fn fit(sample: &Sample, trim: f64) -> Result<Fit, Error> {
    let (mu, coefficients, status) = propensity_fit(sample)?;
    let (outcome, _) = outcome_fit(sample)?;
    let score = score(sample, &mu, &outcome, trim)?;
    Ok(Fit {
        score,
        propensity: mu,
        outcome,
        coefficients,
        status,
    })
}

fn outcome_fit(sample: &Sample) -> Result<(Vec<f64>, Vec<f64>), Error> {
    let n = sample.x.nrows();
    let p = sample.x.ncols();
    let controls: Vec<_> = (0..n).filter(|i| !sample.treated[*i]).collect();
    let x = DMatrix::from_fn(controls.len(), p, |i, j| {
        sample.weights[controls[i]].sqrt() * sample.x[(controls[i], j)]
    });
    let target = DMatrix::from_fn(controls.len(), 1, |i, _| {
        sample.weights[controls[i]].sqrt() * sample.change[controls[i]]
    });
    let reg = crate::least_squares::solve(&x, &target, f64::EPSILON * p as f64)
        .map_err(|_| Error::Numerical)?;
    if reg.rank < p {
        return Err(Error::Singular);
    }
    // Refine the same least-squares solution against its residual. Covariates
    // on very different scales (such as earnings and binary indicators) can
    // lose low-order digits in the initial SVD back substitution.
    let residual = &target - &x * &reg.coefficients;
    let correction = crate::least_squares::solve(&x, &residual, f64::EPSILON * p as f64)
        .map_err(|_| Error::Numerical)?;
    let coefficients = reg.coefficients + correction.coefficients;
    let outcome: Vec<_> = (&sample.x * &coefficients)
        .column(0)
        .iter()
        .copied()
        .collect();
    Ok((
        outcome,
        coefficients.column(0).iter().copied().collect(),
    ))
}

/// DRDID's two IPW estimators have distinct finite-sample denominators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpwNormalization {
    Abadie,
    Hajek,
}

pub struct IpwFit {
    pub score: Score,
    pub propensity: Vec<f64>,
    pub coefficients: Vec<f64>,
    pub status: FitStatus,
}

pub struct OutcomeRegressionFit {
    pub score: Score,
    pub outcome: Vec<f64>,
    pub coefficients: Vec<f64>,
}

fn score_from_influence(att: f64, influence: Vec<f64>) -> Result<Score, Error> {
    let n = influence.len() as f64;
    let mean = influence.iter().sum::<f64>() / n;
    let se = influence
        .iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>()
        .sqrt()
        / n;
    if !att.is_finite() || !se.is_finite() || influence.iter().any(|v| !v.is_finite()) {
        return Err(Error::Numerical);
    }
    Ok(Score { att, se, influence })
}

/// DRDID 1.3.0 ipw_did_panel / std_ipw_did_panel, including propensity fitting effects.
pub fn fit_ipw(
    sample: &Sample,
    trim: f64,
    normalization: IpwNormalization,
) -> Result<IpwFit, Error> {
    let (propensity, coefficients, status) = propensity_fit(sample)?;
    let score = ipw_score(sample, &propensity, trim, normalization)?;
    Ok(IpwFit {
        score,
        propensity,
        coefficients,
        status,
    })
}

pub fn ipw_score(
    sample: &Sample,
    propensity: &[f64],
    trim: f64,
    normalization: IpwNormalization,
) -> Result<Score, Error> {
    let n = sample.x.nrows();
    let nf = n as f64;
    let p = sample.x.ncols();
    if propensity.len() != n {
        return Err(Error::Shape);
    }
    if !trim.is_finite()
        || trim <= 0.0
        || trim > 1.0
        || propensity
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.0 || *v >= 1.0)
    {
        return Err(Error::Probability);
    }
    let ps: Vec<_> = propensity.iter().map(|v| v.min(1.0 - 1e-6)).collect();
    let wt: Vec<_> = (0..n)
        .map(|i| {
            if sample.treated[i] {
                sample.weights[i]
            } else {
                0.0
            }
        })
        .collect();
    let wc: Vec<_> = (0..n)
        .map(|i| {
            if !sample.treated[i] && ps[i] < trim {
                sample.weights[i] * ps[i] / (1.0 - ps[i])
            } else {
                0.0
            }
        })
        .collect();
    let mt = wt.iter().sum::<f64>() / nf;
    let mc = wc.iter().sum::<f64>() / nf;
    if mt <= 0.0 || (normalization == IpwNormalization::Hajek && mc <= 0.0) {
        return Err(Error::MissingGroup);
    }
    let denominator = match normalization {
        IpwNormalization::Abadie => mt,
        IpwNormalization::Hajek => mc,
    };
    let et = (0..n).map(|i| wt[i] * sample.change[i]).sum::<f64>() / nf / mt;
    let ec = (0..n).map(|i| wc[i] * sample.change[i]).sum::<f64>() / nf / denominator;
    let att = et - ec;
    let mut hessian = DMatrix::zeros(p, p);
    let mut moment = vec![0.0; p];
    for i in 0..n {
        let centered = match normalization {
            IpwNormalization::Abadie => sample.change[i],
            IpwNormalization::Hajek => sample.change[i] - ec,
        };
        for j in 0..p {
            moment[j] += wc[i] * centered * sample.x[(i, j)] / nf;
            for k in 0..p {
                hessian[(j, k)] +=
                    sample.weights[i] * ps[i] * (1.0 - ps[i]) * sample.x[(i, j)] * sample.x[(i, k)];
            }
        }
    }
    let h_inv = inverse(&hessian, true)? * nf;
    let influence = (0..n)
        .map(|i| {
            let d = f64::from(sample.treated[i]);
            let adjustment = (0..p)
                .map(|j| {
                    let linear = (0..p)
                        .map(|k| sample.weights[i] * (d - ps[i]) * sample.x[(i, k)] * h_inv[(k, j)])
                        .sum::<f64>();
                    linear * moment[j]
                })
                .sum::<f64>();
            match normalization {
                IpwNormalization::Abadie => {
                    (wt[i] * sample.change[i] - wc[i] * sample.change[i] - adjustment - wt[i] * att)
                        / mt
                }
                IpwNormalization::Hajek => {
                    wt[i] * (sample.change[i] - et) / mt
                        - (wc[i] * (sample.change[i] - ec) + adjustment) / mc
                }
            }
        })
        .collect();
    score_from_influence(att, influence)
}

/// DRDID 1.3.0 reg_did_panel. Only comparison-unit changes fit the outcome model.
/// No propensity model or positivity guard is fitted for this method.
pub fn fit_outcome_regression(sample: &Sample) -> Result<OutcomeRegressionFit, Error> {
    let (outcome, coefficients) = outcome_fit(sample)?;
    let n = sample.x.nrows();
    let nf = n as f64;
    let p = sample.x.ncols();
    let wt: Vec<_> = (0..n)
        .map(|i| {
            if sample.treated[i] {
                sample.weights[i]
            } else {
                0.0
            }
        })
        .collect();
    let mt = wt.iter().sum::<f64>() / nf;
    let residual: Vec<_> = (0..n).map(|i| sample.change[i] - outcome[i]).collect();
    let att = (0..n).map(|i| wt[i] * residual[i]).sum::<f64>() / nf / mt;
    let mut cross = DMatrix::zeros(p, p);
    let mut moment = vec![0.0; p];
    for i in 0..n {
        let control = if sample.treated[i] {
            0.0
        } else {
            sample.weights[i]
        };
        for j in 0..p {
            moment[j] += wt[i] * sample.x[(i, j)] / nf;
            for k in 0..p {
                cross[(j, k)] += control * sample.x[(i, j)] * sample.x[(i, k)] / nf;
            }
        }
    }
    let inv = inverse(&cross, false)?;
    let influence = (0..n)
        .map(|i| {
            let control = if sample.treated[i] {
                0.0
            } else {
                sample.weights[i]
            };
            let correction = (0..p)
                .map(|j| {
                    (0..p)
                        .map(|k| control * residual[i] * sample.x[(i, k)] * inv[(k, j)])
                        .sum::<f64>()
                        * moment[j]
                })
                .sum::<f64>();
            (wt[i] * (residual[i] - att) - correction) / mt
        })
        .collect();
    let score = score_from_influence(att, influence)?;
    Ok(OutcomeRegressionFit {
        score,
        outcome,
        coefficients,
    })
}

fn inverse(a: &DMatrix<f64>, positive: bool) -> Result<DMatrix<f64>, Error> {
    let p = a.nrows();
    let mut factor = a.as_slice().to_vec();
    let mut result = DMatrix::<f64>::identity(p, p);
    if positive {
        let info =
            chol::dpotrf(chol::Triangle::Upper, p, &mut factor, p).map_err(|_| Error::Shape)?;
        if info != 0 {
            return Err(Error::Singular);
        }
        chol::dpotrs(
            chol::Triangle::Upper,
            p,
            p,
            &factor,
            p,
            result.as_mut_slice(),
            p,
        )
        .map_err(|_| Error::Shape)?;
    } else {
        let mut pivots = vec![0; p];
        let info = lu::dgetrf(p, p, &mut factor, p, &mut pivots).map_err(|_| Error::Shape)?;
        if info != 0 {
            return Err(Error::Singular);
        }
        lu::dgetrs(
            lu::Transpose::None,
            p,
            p,
            &factor,
            p,
            &pivots,
            result.as_mut_slice(),
            p,
        )
        .map_err(|_| Error::Shape)?;
    }
    // Same one-norm singularity threshold as the reference rcond guard.
    let norm = |m: &DMatrix<f64>| {
        (0..p)
            .map(|j| (0..p).map(|i| m[(i, j)].abs()).sum::<f64>())
            .fold(0.0, f64::max)
    };
    if 1.0 / (norm(a) * norm(&result)) < f64::EPSILON {
        return Err(Error::Singular);
    }
    if result.iter().any(|v| !v.is_finite()) {
        return Err(Error::Numerical);
    }
    Ok(result)
}

/// Conditional replay boundary: supplied nuisance predictions must be from the
/// same sample and design. This isolates score parity from optimizer parity.
pub fn score(
    sample: &Sample,
    propensity: &[f64],
    outcome: &[f64],
    trim: f64,
) -> Result<Score, Error> {
    let n = sample.x.nrows();
    let p = sample.x.ncols();
    let nf = n as f64;
    if propensity.len() != n || outcome.len() != n {
        return Err(Error::Shape);
    }
    if !trim.is_finite()
        || trim <= 0.0
        || trim > 1.0
        || propensity
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.0 || *v >= 1.0)
    {
        return Err(Error::Probability);
    }
    if outcome.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let ps: Vec<_> = propensity.iter().map(|v| v.min(1.0 - 1e-6)).collect();
    let residual: Vec<_> = sample
        .change
        .iter()
        .zip(outcome)
        .map(|(y, m)| y - m)
        .collect();
    let wt: Vec<_> = (0..n)
        .map(|i| {
            if sample.treated[i] {
                sample.weights[i]
            } else {
                0.0
            }
        })
        .collect();
    let wc: Vec<_> = (0..n)
        .map(|i| {
            if !sample.treated[i] && ps[i] < trim {
                sample.weights[i] * ps[i] / (1.0 - ps[i])
            } else {
                0.0
            }
        })
        .collect();
    let mt = wt.iter().sum::<f64>() / nf;
    let mc = wc.iter().sum::<f64>() / nf;
    if mt <= 0.0 || mc <= 0.0 {
        return Err(Error::MissingGroup);
    }
    let et = wt.iter().zip(&residual).map(|(w, r)| w * r).sum::<f64>() / nf / mt;
    let ec = wc.iter().zip(&residual).map(|(w, r)| w * r).sum::<f64>() / nf / mc;
    let mut xx = DMatrix::zeros(p, p);
    let mut hessian = DMatrix::zeros(p, p);
    let mut m1 = vec![0.0; p];
    let mut m2 = vec![0.0; p];
    let mut m3 = vec![0.0; p];
    for i in 0..n {
        let w = sample.weights[i];
        let c = if sample.treated[i] { 0.0 } else { w };
        for j in 0..p {
            let x = sample.x[(i, j)];
            m1[j] += wt[i] * x;
            m2[j] += wc[i] * (residual[i] - ec) * x;
            m3[j] += wc[i] * x;
            for k in 0..p {
                xx[(j, k)] += c * x * sample.x[(i, k)];
                hessian[(j, k)] += ps[i] * (1.0 - ps[i]) * w * x * sample.x[(i, k)];
            }
        }
    }
    xx /= nf;
    for moment in [&mut m1, &mut m2, &mut m3] {
        moment.iter_mut().for_each(|value| *value /= nf);
    }
    let xx_inv = inverse(&xx, false)?;
    let h_inv = inverse(&hessian, true)? * nf;
    let mut influence = vec![0.0; n];
    for i in 0..n {
        let d = if sample.treated[i] { 1.0 } else { 0.0 };
        let w = sample.weights[i];
        let mut ols_t = 0.0;
        let mut ps_c = 0.0;
        let mut ols_c = 0.0;
        for j in 0..p {
            let ols = (0..p)
                .map(|k| w * (1.0 - d) * residual[i] * sample.x[(i, k)] * xx_inv[(k, j)])
                .sum::<f64>();
            let prop = (0..p)
                .map(|k| w * (d - ps[i]) * sample.x[(i, k)] * h_inv[(k, j)])
                .sum::<f64>();
            ols_t += ols * m1[j];
            ps_c += prop * m2[j];
            ols_c += ols * m3[j];
        }
        influence[i] = (wt[i] * (residual[i] - et) - ols_t) / mt
            - (wc[i] * (residual[i] - ec) + ps_c - ols_c) / mc;
    }
    let att = et - ec;
    let mean = influence.iter().sum::<f64>() / nf;
    let se = influence
        .iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>()
        .sqrt()
        / nf;
    if !att.is_finite() || !se.is_finite() {
        return Err(Error::Numerical);
    }
    Ok(Score { att, se, influence })
}
