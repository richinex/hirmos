//! R survival's unpenalised, right-censored Breslow fit with clustered sandwich
//! covariance. Source: coxfit6.c, coxscore2.c, residuals.coxph.R and coxph.R.
//! This is a separate contract from the lifelines Efron fitter. It currently
//! accepts unit weights, no strata/entry, and a full-rank design only.

use super::frailty::{aeq_surv, cholesky2, chsolve2};
use super::{Event, RightCensoredData};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug)]
pub struct ClusteredBreslowOptions {
    pub max_iterations: usize,
    pub timefix: bool,
}

impl Default for ClusteredBreslowOptions {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            timefix: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClusteredBreslowError {
    MissingClusters,
    UnsupportedEntry,
    UnsupportedStrata,
    UnsupportedWeights,
    InvalidIterations,
    SingularInformation,
    NonFiniteFit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClusteredConvergence {
    Converged,
    ConvergedDuringHalving,
    IterationLimit,
}

#[derive(Clone, Debug)]
pub struct ClusteredBreslowFit {
    pub coefficients: Vec<f64>,
    /// Row-major sandwich covariance, not the inverse information matrix.
    pub covariance: Vec<f64>,
    pub naive_covariance: Vec<f64>,
    pub means: Vec<f64>,
    pub linear_predictors: Vec<f64>,
    pub score_residuals: Vec<f64>,
    pub log_likelihood: [f64; 2],
    pub score_test: f64,
    pub robust_score_test: f64,
    pub wald_test: f64,
    pub iterations: usize,
    pub convergence: ClusteredConvergence,
    pub clusters: usize,
    /// The event times used by the fit, after optional near-tie correction.
    pub times: Vec<f64>,
    pub concordance: Option<f64>,
}

const EPS: f64 = 1e-9;

fn factor(matrix: &mut [f64], p: usize) -> Result<(), ClusteredBreslowError> {
    cholesky2(matrix, p, f64::EPSILON.powf(0.75));
    if (0..p).any(|j| matrix[j * p + j] <= 0.0) {
        return Err(ClusteredBreslowError::SingularInformation);
    }
    Ok(())
}

/// survival's chinv2; factorisation and solve reuse the existing survival port.
fn invert(matrix: &mut [f64], p: usize) {
    for i in 0..p {
        matrix[i * p + i] = 1.0 / matrix[i * p + i];
        for j in i + 1..p {
            matrix[j * p + i] = -matrix[j * p + i];
            for k in 0..i {
                matrix[j * p + k] += matrix[j * p + i] * matrix[i * p + k];
            }
        }
    }
    for i in 0..p {
        for j in i + 1..p {
            let temp = matrix[j * p + i] * matrix[j * p + j];
            matrix[i * p + j] = temp;
            for k in i..j {
                matrix[i * p + k] += temp * matrix[j * p + k];
            }
        }
    }
    for i in 0..p {
        for j in i + 1..p {
            matrix[j * p + i] = matrix[i * p + j];
        }
    }
}

struct Evaluation {
    likelihood: f64,
    score: Vec<f64>,
    information: Vec<f64>,
}

fn evaluate(
    data: &RightCensoredData,
    times: &[f64],
    order: &[usize],
    x: &[f64],
    beta: &[f64],
) -> Evaluation {
    let p = beta.len();
    let mut out = Evaluation {
        likelihood: 0.0,
        score: vec![0.0; p],
        information: vec![0.0; p * p],
    };
    let mut a = vec![0.0; p];
    let mut a2 = vec![0.0; p];
    let mut c = vec![0.0; p * p];
    let mut c2 = vec![0.0; p * p];
    let mut denom = 0.0;
    let mut end = order.len();
    while end > 0 {
        let time = times[order[end - 1]];
        let mut deaths = 0.0;
        let mut event_risk = 0.0;
        while end > 0 && times[order[end - 1]] == time {
            end -= 1;
            let row = order[end];
            let values = &x[row * p..(row + 1) * p];
            let lp = (0..p).map(|j| beta[j] * values[j]).sum::<f64>();
            let risk = lp.exp();
            let observed = data.events()[row] == Event::Observed;
            if observed {
                deaths += 1.0;
                event_risk += risk;
                out.likelihood += lp;
                for j in 0..p {
                    out.score[j] += values[j];
                    a2[j] += risk * values[j];
                    for k in 0..=j {
                        c2[j * p + k] += risk * values[j] * values[k];
                    }
                }
            } else {
                denom += risk;
                for j in 0..p {
                    a[j] += risk * values[j];
                    for k in 0..=j {
                        c[j * p + k] += risk * values[j] * values[k];
                    }
                }
            }
        }
        if deaths == 0.0 {
            continue;
        }
        denom += event_risk;
        out.likelihood -= deaths * denom.ln();
        for j in 0..p {
            a[j] += a2[j];
            let mean = a[j] / denom;
            out.score[j] -= deaths * mean;
            for k in 0..=j {
                c[j * p + k] += c2[j * p + k];
                out.information[k * p + j] += deaths * (c[j * p + k] - mean * a[k]) / denom;
            }
        }
        a2.fill(0.0);
        c2.fill(0.0);
    }
    out
}

/// coxscore2's reverse-time cumulative sums: O(n*p), including tied censoring.
fn residuals(data: &RightCensoredData, times: &[f64], lp: &[f64]) -> Vec<f64> {
    let p = data.columns();
    let mut order = (0..data.rows()).collect::<Vec<_>>();
    order.sort_by(|&a, &b| {
        times[a]
            .total_cmp(&times[b])
            .then(data.events()[b].cmp(&data.events()[a]))
    });
    let mut scores = vec![0.0; data.rows() * p];
    let mut a = vec![0.0; p];
    let mut xhaz = vec![0.0; p];
    let mut denom = 0.0;
    let mut cumhaz = 0.0;
    let mut end = order.len();
    while end > 0 {
        let last = end;
        let time = times[order[end - 1]];
        let mut deaths = 0.0;
        while end > 0 && times[order[end - 1]] == time {
            end -= 1;
            let row = order[end];
            let risk = lp[row].exp();
            denom += risk;
            let x = data.covariates().row(row);
            for j in 0..p {
                scores[row * p + j] = risk * (x[j] * cumhaz - xhaz[j]);
                a[j] += risk * x[j];
            }
            deaths += f64::from(data.events()[row] == Event::Observed);
        }
        if deaths == 0.0 {
            continue;
        }
        let hazard = deaths / denom;
        cumhaz += hazard;
        for j in 0..p {
            let mean = a[j] / denom;
            xhaz[j] += mean * hazard;
            for &row in &order[end..last] {
                if data.events()[row] == Event::Observed {
                    scores[row * p + j] += data.covariates().row(row)[j] - mean;
                }
            }
        }
    }
    for &row in order.iter().rev() {
        for j in 0..p {
            scores[row * p + j] +=
                lp[row].exp() * (xhaz[j] - data.covariates().row(row)[j] * cumhaz);
        }
    }
    scores
}

fn collapse(
    scores: &[f64],
    groups: &[usize],
    p: usize,
    covariance: Option<&[f64]>,
) -> Vec<Vec<f64>> {
    let mut result = BTreeMap::<usize, Vec<f64>>::new();
    for (row, &group) in groups.iter().enumerate() {
        let total = result.entry(group).or_insert_with(|| vec![0.0; p]);
        for j in 0..p {
            let value = match covariance {
                Some(v) => (0..p).map(|k| scores[row * p + k] * v[k * p + j]).sum(),
                None => scores[row * p + j],
            };
            total[j] += value;
        }
    }
    result.into_values().collect()
}

fn crossproduct(rows: &[Vec<f64>], p: usize) -> Vec<f64> {
    let mut result = vec![0.0; p * p];
    for row in rows {
        for j in 0..p {
            for k in 0..p {
                result[j * p + k] += row[j] * row[k];
            }
        }
    }
    result
}

fn quadratic(mut covariance: Vec<f64>, values: &[f64]) -> f64 {
    let p = values.len();
    cholesky2(&mut covariance, p, f64::EPSILON.powf(0.75));
    let mut solved = values.to_vec();
    chsolve2(&covariance, p, &mut solved);
    values.iter().zip(solved).map(|(a, b)| a * b).sum()
}

pub fn fit_clustered_breslow(
    data: &RightCensoredData,
    options: ClusteredBreslowOptions,
) -> Result<ClusteredBreslowFit, ClusteredBreslowError> {
    let groups = data
        .clusters()
        .ok_or(ClusteredBreslowError::MissingClusters)?;
    if data.entries().is_some() {
        return Err(ClusteredBreslowError::UnsupportedEntry);
    }
    if data.strata().is_some() {
        return Err(ClusteredBreslowError::UnsupportedStrata);
    }
    if data.weights().iter().any(|&w| w != 1.0) {
        return Err(ClusteredBreslowError::UnsupportedWeights);
    }
    if options.max_iterations < 2 {
        return Err(ClusteredBreslowError::InvalidIterations);
    }
    let times = if options.timefix {
        aeq_surv(data.durations())
    } else {
        data.durations().to_vec()
    };
    let p = data.columns();
    let n = data.rows();
    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
    let mut means = vec![0.0; p];
    let mut scales = vec![1.0; p];
    let mut x = data.covariates().values().to_vec();
    for j in 0..p {
        if order
            .iter()
            .all(|&row| [-1.0, 0.0, 1.0].contains(&x[row * p + j]))
        {
            continue;
        }
        means[j] = order.iter().map(|&row| x[row * p + j]).sum::<f64>() / n as f64;
        for row in 0..n {
            x[row * p + j] -= means[j];
        }
        let deviation = order.iter().map(|&row| x[row * p + j].abs()).sum::<f64>();
        if deviation > 0.0 {
            scales[j] = n as f64 / deviation;
        }
        for row in 0..n {
            x[row * p + j] *= scales[j];
        }
    }
    let mut beta = vec![0.0; p];
    let mut current = evaluate(data, &times, &order, &x, &beta);
    if !current.likelihood.is_finite() {
        return Err(ClusteredBreslowError::NonFiniteFit);
    }
    let initial = current.likelihood;
    factor(&mut current.information, p)?;
    let mut step = current.score.clone();
    chsolve2(&current.information, p, &mut step);
    let score_test = current.score.iter().zip(&step).map(|(a, b)| a * b).sum();
    let mut next = step;
    let mut halving = 0.0;
    let mut iterations = 0;
    let mut convergence = ClusteredConvergence::IterationLimit;
    for iteration in 1..=options.max_iterations {
        iterations = iteration;
        let mut trial = evaluate(data, &times, &order, &x, &next);
        let finite = trial.likelihood.is_finite()
            && trial
                .score
                .iter()
                .chain(&trial.information)
                .all(|v| v.is_finite());
        if !finite
            || trial.likelihood < current.likelihood
                && (1.0 - current.likelihood / trial.likelihood).abs() > EPS
        {
            halving += 1.0;
            for j in 0..p {
                next[j] = (next[j] + halving * beta[j]) / (halving + 1.0);
            }
            continue;
        }
        factor(&mut trial.information, p)?;
        if (1.0 - current.likelihood / trial.likelihood).abs() <= EPS {
            beta = next;
            current = trial;
            convergence = if halving > 0.0 {
                ClusteredConvergence::ConvergedDuringHalving
            } else {
                ClusteredConvergence::Converged
            };
            break;
        }
        halving = 0.0;
        beta.clone_from(&next);
        let mut step = trial.score.clone();
        chsolve2(&trial.information, p, &mut step);
        for j in 0..p {
            next[j] += step[j];
        }
        current = trial;
    }
    if convergence == ClusteredConvergence::IterationLimit {
        iterations += 1;
        // coxfit6's exhausted-iteration exit re-evaluates before chinv2 without
        // another factorisation. Retain that output and its nonconvergence status.
        current = evaluate(data, &times, &order, &x, &beta);
    }
    invert(&mut current.information, p);
    for j in 0..p {
        beta[j] *= scales[j];
        for k in 0..p {
            current.information[j * p + k] *= scales[j] * scales[k];
        }
    }
    let center = beta.iter().zip(&means).map(|(a, b)| a * b).sum::<f64>();
    let lp = (0..n)
        .map(|row| {
            data.covariates()
                .row(row)
                .iter()
                .zip(&beta)
                .map(|(a, b)| a * b)
                .sum::<f64>()
                - center
        })
        .collect::<Vec<_>>();
    let scores = residuals(data, &times, &lp);
    let influences = collapse(&scores, groups, p, Some(&current.information));
    let covariance = crossproduct(&influences, p);
    let null_scores = residuals(data, &times, &vec![0.0; n]);
    let null_groups = collapse(&null_scores, groups, p, None);
    let total = (0..p)
        .map(|j| null_groups.iter().map(|r| r[j]).sum())
        .collect::<Vec<_>>();
    let robust_score_test = quadratic(crossproduct(&null_groups, p), &total);
    let wald_test = quadratic(covariance.clone(), &beta);
    let predictions = lp.iter().map(|value| -value).collect::<Vec<_>>();
    let concordance = super::concordance::prediction_concordance_index(
        &times, data.events(), &predictions, None,
    ).map(|value| value.value());
    Ok(ClusteredBreslowFit {
        coefficients: beta,
        covariance,
        naive_covariance: current.information,
        means,
        linear_predictors: lp,
        score_residuals: scores,
        log_likelihood: [initial, current.likelihood],
        score_test,
        robust_score_test,
        wald_test,
        iterations,
        convergence,
        clusters: influences.len(),
        times,
        concordance,
    })
}
