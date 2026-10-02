//! Single-unit ridge augmentation from augsynth 0.2.0 commit 7e700722.
//! Baseline weights, conditional augmentation, CV and time-jackknife+ are
//! separate stages. Covariates and unit demeaning are not implemented here.
use crate::lapack_lu::{dgetrf, dgetrs, Transpose};
use nalgebra::{DMatrix, DVector};

#[derive(Debug, PartialEq)]
pub enum Error {
    Empty,
    Shape,
    NonFinite,
    InvalidLambda,
    InvalidWeights,
    Singular,
    Optimizer,
    InvalidBoundary,
    InvalidConfidence,
}
pub struct Augmentation {
    pub weights: Vec<f64>,
    pub adjustment: Vec<f64>,
}

/// Input donor rows and target are already centred against donor means and
/// transformed by the configured V matrix. No nonnegativity constraint is
/// applied to augmented weights.
pub fn augment(
    donors: &DMatrix<f64>,
    target: &[f64],
    baseline: &[f64],
    lambda: f64,
) -> Result<Augmentation, Error> {
    let (n, p) = donors.shape();
    if n == 0 || p == 0 {
        return Err(Error::Empty);
    }
    if target.len() != p || baseline.len() != n {
        return Err(Error::Shape);
    }
    if donors
        .iter()
        .chain(target)
        .chain(baseline)
        .any(|x| !x.is_finite())
    {
        return Err(Error::NonFinite);
    }
    if !lambda.is_finite() || lambda <= 0. {
        return Err(Error::InvalidLambda);
    }
    if baseline.iter().any(|x| *x < -1e-7) || (baseline.iter().sum::<f64>() - 1.).abs() > 1e-7 {
        return Err(Error::InvalidWeights);
    }
    let residual = DVector::from_column_slice(target)
        - donors.transpose() * DVector::from_column_slice(baseline);
    let mut gram = (donors.transpose() * donors).as_slice().to_vec();
    for j in 0..p {
        gram[j * p + j] += lambda;
    }
    let mut pivots = vec![0; p];
    if dgetrf(p, p, &mut gram, p, &mut pivots).map_err(|_| Error::Singular)? != 0 {
        return Err(Error::Singular);
    }
    let mut solution = residual.as_slice().to_vec();
    dgetrs(Transpose::None, p, 1, &gram, p, &pivots, &mut solution, p)
        .map_err(|_| Error::Singular)?;
    let adjustment = donors * DVector::from_column_slice(&solution);
    let weights: Vec<_> = baseline
        .iter()
        .zip(adjustment.iter())
        .map(|(s, r)| s + r)
        .collect();
    if weights.iter().any(|x| !x.is_finite()) {
        return Err(Error::NonFinite);
    }
    Ok(Augmentation {
        weights,
        adjustment: adjustment.as_slice().to_vec(),
    })
}

/// Source grid contains n_lambda+1 entries. R's seq(0:n_lambda) uses the
/// length of that vector, so the subsequent subtraction produces 0:n_lambda.
pub fn lambda_grid(maximum: f64, ratio: f64, steps: usize) -> Result<Vec<f64>, Error> {
    if !maximum.is_finite()
        || maximum <= 0.
        || !ratio.is_finite()
        || ratio <= 0.
        || ratio >= 1.
        || steps == 0
    {
        return Err(Error::InvalidLambda);
    }
    let scale = ratio.powf(1. / steps as f64);
    let grid: Vec<_> = (0..=steps)
        .map(|i| maximum * scale.powf(i as f64))
        .collect();
    if grid.iter().any(|x| !x.is_finite() || *x <= 0.) {
        return Err(Error::InvalidLambda);
    }
    Ok(grid)
}
#[derive(Clone, Copy)]
pub enum Selection {
    MinimumError,
    OneStandardError,
}
pub fn select_lambda(
    grid: &[f64],
    errors: &[f64],
    standard_errors: &[f64],
    rule: Selection,
) -> Result<f64, Error> {
    if grid.is_empty() {
        return Err(Error::Empty);
    }
    if errors.len() != grid.len() || standard_errors.len() != grid.len() {
        return Err(Error::Shape);
    }
    if grid.iter().any(|x| !x.is_finite() || *x <= 0.)
        || errors
            .iter()
            .chain(standard_errors)
            .any(|x| !x.is_finite() || *x < 0.)
    {
        return Err(Error::NonFinite);
    }
    let mut best = 0;
    for i in 1..grid.len() {
        if errors[i] < errors[best] {
            best = i;
        }
    }
    Ok(match rule {
        Selection::MinimumError => grid[best],
        Selection::OneStandardError => grid
            .iter()
            .zip(errors)
            .filter(|(_, e)| **e <= errors[best] + standard_errors[best])
            .map(|(g, _)| *g)
            .fold(0., f64::max),
    })
}

#[derive(Clone, Copy)]
pub struct Tuning {
    pub maximum: Option<f64>,
    pub minimum_ratio: f64,
    pub steps: usize,
    pub holdout_length: usize,
    pub selection: Selection,
}
impl Default for Tuning {
    fn default() -> Self {
        Self {
            maximum: None,
            minimum_ratio: 1e-8,
            steps: 20,
            holdout_length: 1,
            selection: Selection::OneStandardError,
        }
    }
}
#[derive(Clone, Copy)]
pub enum Regularization {
    Fixed(f64),
    CrossValidation(Tuning),
}
pub struct CvEvidence {
    pub candidates: Vec<f64>,
    pub errors: Vec<f64>,
    pub standard_errors: Vec<f64>,
}
pub struct Fit {
    pub weights: Vec<f64>,
    pub baseline: Vec<f64>,
    pub lambda: f64,
    pub tuning: Option<CvEvidence>,
    pub prediction: Vec<f64>,
    pub effects: Vec<f64>,
}

fn baseline(donors: &DMatrix<f64>, target: &[f64]) -> Result<Vec<f64>, Error> {
    // Reuse the translated OSQP solver with the oracle's auxiliary-variable
    // formulation and settings. No donor Gram matrix, fallback or clipping.
    let (n, t) = donors.shape();
    let variables = n + t;
    let mut p = vec![vec![0.; variables]; variables];
    let mut q = vec![0.; variables];
    for j in 0..t {
        p[n + j][n + j] = 1.;
        q[n + j] = -target[j];
    }
    let mut a = vec![vec![0.; variables]; 1 + n + t];
    let mut lower = vec![0.; 1 + n + t];
    let mut upper = vec![0.; 1 + n + t];
    for i in 0..n {
        a[0][i] = 1.;
        a[1 + i][i] = 1.;
        upper[1 + i] = 1.;
    }
    lower[0] = 1.;
    upper[0] = 1.;
    for j in 0..t {
        for i in 0..n {
            a[1 + n + j][i] = -donors[(i, j)];
        }
        a[1 + n + j][n + j] = 1.;
    }
    let result = osqp_reference_port::solve_with(
        &p,
        &q,
        &a,
        &lower,
        &upper,
        osqp_reference_port::Settings::Augsynth,
    )
    .map_err(|_| Error::Optimizer)?;
    if result.status != 1 {
        return Err(Error::Optimizer);
    }
    Ok(result.x[..n].to_vec())
}

fn tune(donors: &DMatrix<f64>, target: &[f64], spec: Tuning) -> Result<(f64, CvEvidence), Error> {
    let (n, p) = donors.shape();
    if spec.holdout_length == 0 || spec.holdout_length >= p || p - spec.holdout_length < 2 {
        return Err(Error::InvalidBoundary);
    }
    let maximum = match spec.maximum {
        Some(v) => v,
        None => {
            let mut buffer = donors.as_slice().to_vec();
            let (info, svd) = crate::lapack_dgesdd::dgesdd(
                crate::lapack_dgesdd::SvdJob::None,
                n,
                p,
                &mut buffer,
                n,
            )
            .map_err(|_| Error::Singular)?;
            if info != 0 {
                return Err(Error::Singular);
            }
            svd.singular_values[0].powi(2)
        }
    };
    let grid = lambda_grid(maximum, spec.minimum_ratio, spec.steps)?;
    let folds = p - spec.holdout_length;
    let mut fold_errors = DMatrix::<f64>::zeros(folds, grid.len());
    // The pinned package evaluates p-holdout_length folds, not p-h+1.
    for i in 0..folds {
        let retained: Vec<_> = (0..p)
            .filter(|j| *j < i || *j >= i + spec.holdout_length)
            .collect();
        let train = DMatrix::from_fn(n, retained.len(), |r, c| donors[(r, retained[c])]);
        let train_target: Vec<_> = retained.iter().map(|j| target[*j]).collect();
        let syn = baseline(&train, &train_target)?;
        for (j, &lambda) in grid.iter().enumerate() {
            let w = augment(&train, &train_target, &syn, lambda)?.weights;
            fold_errors[(i, j)] = (i..i + spec.holdout_length)
                .map(|t| {
                    let pred: f64 = (0..n).map(|r| donors[(r, t)] * w[r]).sum();
                    (target[t] - pred).powi(2)
                })
                .sum();
        }
    }
    let errors: Vec<f64> = (0..grid.len())
        .map(|j| fold_errors.column(j).sum() / folds as f64)
        .collect();
    let standard_errors: Vec<f64> = (0..grid.len())
        .map(|j| {
            (fold_errors
                .column(j)
                .iter()
                .map(|e| (e - errors[j]).powi(2))
                .sum::<f64>()
                / ((folds - 1) * folds) as f64)
                .sqrt()
        })
        .collect();
    let lambda = select_lambda(&grid, &errors, &standard_errors, spec.selection)?;
    Ok((
        lambda,
        CvEvidence {
            candidates: grid,
            errors,
            standard_errors,
        },
    ))
}

/// Complete single-treated-unit, outcome-history-only fit. The columns are
/// ordered periods; pre_periods splits the pre-treatment and post-treatment
/// blocks. Auxiliary covariates and unit demeaning are not enabled here.
pub fn fit(
    donors: &DMatrix<f64>,
    target: &[f64],
    pre_periods: usize,
    regularization: Regularization,
) -> Result<Fit, Error> {
    let (n, t) = donors.shape();
    if n < 2 || t == 0 {
        return Err(Error::Empty);
    }
    if target.len() != t {
        return Err(Error::Shape);
    }
    if pre_periods == 0 || pre_periods >= t {
        return Err(Error::InvalidBoundary);
    }
    if donors.iter().chain(target).any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let means: Vec<f64> = (0..pre_periods)
        .map(|j| donors.column(j).sum() / n as f64)
        .collect();
    let centered = DMatrix::from_fn(n, pre_periods, |i, j| donors[(i, j)] - means[j]);
    let centered_target: Vec<_> = (0..pre_periods).map(|j| target[j] - means[j]).collect();
    let syn = baseline(&centered, &centered_target)?;
    let (lambda, tuning) = match regularization {
        Regularization::Fixed(v) => (v, None),
        Regularization::CrossValidation(s) => {
            let (v, e) = tune(&centered, &centered_target, s)?;
            (v, Some(e))
        }
    };
    let weights = augment(&centered, &centered_target, &syn, lambda)?.weights;
    let prediction: Vec<f64> = (0..t)
        .map(|j| (0..n).map(|i| weights[i] * donors[(i, j)]).sum())
        .collect();
    let effects = target.iter().zip(&prediction).map(|(y, p)| y - p).collect();
    Ok(Fit {
        weights,
        baseline: syn,
        lambda,
        tuning,
        prediction,
        effects,
    })
}

#[derive(Clone, Copy)]
pub enum JackknifeRule {
    Plus,
    Conservative,
}
pub struct JackknifeInterval {
    /// Post-treatment periods followed by their average, as in the oracle.
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
    pub heldout_errors: Vec<f64>,
}
fn quantile(values: &[f64], probability: f64) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let position = (sorted.len() - 1) as f64 * probability;
    let i = position.floor() as usize;
    sorted[i] + (position - i as f64) * (sorted[position.ceil() as usize] - sorted[i])
}
pub fn jackknife_plus(
    donors: &DMatrix<f64>,
    target: &[f64],
    pre_periods: usize,
    regularization: Regularization,
    alpha: f64,
    rule: JackknifeRule,
) -> Result<JackknifeInterval, Error> {
    if !alpha.is_finite() || alpha <= 0. || alpha >= 1. {
        return Err(Error::InvalidConfidence);
    }
    // Validate the original specification before constructing deletion fits.
    let validated = fit(donors, target, pre_periods, regularization)?;
    if pre_periods < 2 {
        return Err(Error::InvalidBoundary);
    }
    let (n, t) = donors.shape();
    let post = t - pre_periods;
    let mut predictions = DMatrix::zeros(pre_periods, post + 1);
    let mut errors = Vec::with_capacity(pre_periods);
    for deleted in 0..pre_periods {
        let order: Vec<_> = (0..pre_periods)
            .filter(|j| *j != deleted)
            .chain(std::iter::once(deleted))
            .chain(pre_periods..t)
            .collect();
        let new_donors = DMatrix::from_fn(n, t, |i, j| donors[(i, order[j])]);
        let new_target: Vec<_> = order.iter().map(|j| target[*j]).collect();
        // augsynth stores the selected lambda in extra_args before inference.
        // Deletion fits hold it fixed; they do not rerun hyperparameter tuning.
        let deleted_fit = fit(
            &new_donors,
            &new_target,
            pre_periods - 1,
            Regularization::Fixed(validated.lambda),
        )?;
        errors.push(target[deleted] - deleted_fit.prediction[pre_periods - 1]);
        for j in 0..post {
            predictions[(deleted, j)] = deleted_fit.prediction[pre_periods + j];
        }
        predictions[(deleted, post)] =
            (0..post).map(|j| predictions[(deleted, j)]).sum::<f64>() / post as f64;
    }
    let observed: Vec<_> = target[pre_periods..]
        .iter()
        .copied()
        .chain(std::iter::once(
            target[pre_periods..].iter().sum::<f64>() / post as f64,
        ))
        .collect();
    let absolute: Vec<_> = errors.iter().map(|v| v.abs()).collect();
    let mut lower = vec![];
    let mut upper = vec![];
    for j in 0..=post {
        let estimates: Vec<_> = predictions.column(j).iter().copied().collect();
        let (lo, hi) = match rule {
            JackknifeRule::Plus => {
                let lows: Vec<_> = estimates
                    .iter()
                    .zip(&absolute)
                    .map(|(p, e)| p - e)
                    .collect();
                let highs: Vec<_> = estimates
                    .iter()
                    .zip(&absolute)
                    .map(|(p, e)| p + e)
                    .collect();
                (
                    quantile(&lows, alpha / 2.),
                    quantile(&highs, 1. - alpha / 2.),
                )
            }
            JackknifeRule::Conservative => {
                let q = quantile(&absolute, 1. - alpha);
                (
                    estimates.iter().copied().fold(f64::INFINITY, f64::min) - q,
                    estimates.iter().copied().fold(f64::NEG_INFINITY, f64::max) + q,
                )
            }
        };
        lower.push(observed[j] - hi);
        upper.push(observed[j] - lo);
    }
    Ok(JackknifeInterval {
        lower,
        upper,
        heldout_errors: errors,
    })
}
