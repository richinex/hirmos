//! Cross-sectional ATE weighting with explicit finite-sample normalization.
//! Propensity fitting is separate so the same fitted scores feed diagnostics and estimation.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Normalization {
    /// Facure chapter 5: potential-outcome totals divided by the full sample size.
    HorvitzThompson,
    /// Separate weighted means, as in DoWhy 0.14's weighting estimator.
    Hajek,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeightScale {
    InverseProbability,
    Stabilized,
}

/// Population over which the binary treatment contrast is averaged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Ate,
    Att,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Specification {
    pub normalization: Normalization,
    pub scale: WeightScale,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    EmptySample,
    LengthMismatch,
    NonFiniteOutcome { row: usize },
    InvalidProbability { row: usize },
    MissingArm { treated: bool },
    NumericalOverflow,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FitError {
    EmptySample,
    LengthMismatch,
    NoColumns,
    NonFiniteDesign { row: usize, column: usize },
    MissingArm { treated: bool },
    StepSolveFailed { iteration: usize },
}

/// Whether the design already holds its constant column. patsy emits one and sklearn fits its
/// own, so the two differ and the choice is explicit rather than inferred from the model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intercept {
    Present,
    Absent,
}

/// Facure's chapter 5 fits the propensity twice: statsmodels for In[4] onwards, sklearn from In[11].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TreatmentModel {
    /// statsmodels `Logit`, which counts the intercept among the design columns.
    Newton(crate::logit::Settings),
    /// sklearn `LogisticRegression(penalty=None)`, which fits an intercept of its own.
    Lbfgsb { max_iter: usize },
}

#[derive(Clone, Debug)]
pub enum FittedModel {
    Newton {
        /// One per design column, in column order.
        params: Vec<f64>,
        iterations: usize,
        termination: crate::logit::Termination,
    },
    Lbfgsb {
        coefficients: Vec<f64>,
        intercept: f64,
        termination: crate::lbfgsb::LbfgsbTermination,
    },
}

impl FittedModel {
    pub fn converged(&self) -> bool {
        match self {
            Self::Newton { termination, .. } => termination.converged(),
            Self::Lbfgsb { termination, .. } => termination.converged(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scores {
    /// P(treated | design), in source row order.
    pub propensity: Vec<f64>,
    pub fitted: FittedModel,
}

#[derive(Clone, Debug)]
pub struct Bootstrap {
    pub lower: f64,
    pub upper: f64,
    /// One estimate per round, in round order.
    pub estimates: Vec<f64>,
    /// Rounds whose treatment model stopped before converging.
    pub unconverged: Vec<usize>,
}

/// Draws rows by NumPy's legacy masked-rejection path, and refits the model in every round.
pub fn bootstrap_interval(
    design: &[Vec<f64>],
    outcome: &[f64],
    treated: &[bool],
    intercept: Intercept,
    estimand: Estimand,
    rounds: usize,
    seed: u32,
    model: TreatmentModel,
    lower_percentile: f64,
    upper_percentile: f64,
) -> Result<Bootstrap, EstimateError> {
    let n = outcome.len();
    if n == 0 {
        return Err(EstimateError::Weighting(Error::EmptySample));
    }
    if treated.len() != n || design.len() != n {
        return Err(EstimateError::Weighting(Error::LengthMismatch));
    }
    let mut rng = crate::nprandom::Mt19937::seeded(seed);
    let mut estimates = Vec::with_capacity(rounds);
    let mut unconverged = Vec::new();
    for round in 0..rounds {
        let rows = crate::dowhy_bootstrap::resample_rows(&mut rng, n, n);
        let resampled_design: Vec<Vec<f64>> = rows.iter().map(|&row| design[row].clone()).collect();
        let resampled_treated: Vec<bool> = rows.iter().map(|&row| treated[row]).collect();
        let resampled_outcome: Vec<f64> = rows.iter().map(|&row| outcome[row]).collect();
        let (effect, fitted) = match estimand {
            Estimand::InverseProbability(specification) => {
                let scores = fit(&resampled_design, intercept, &resampled_treated, model)
                    .map_err(EstimateError::Fit)?;
                let estimate = ate(
                    &resampled_outcome,
                    &resampled_treated,
                    &scores.propensity,
                    specification,
                )
                .map_err(EstimateError::Weighting)?;
                (estimate.effect, scores.fitted)
            }
            Estimand::DoublyRobust => {
                let estimate = doubly_robust(
                    &resampled_design,
                    &resampled_outcome,
                    &resampled_treated,
                    intercept,
                    model,
                )?;
                (estimate.effect, estimate.fitted)
            }
            Estimand::InverseProbabilityAtt(specification) => {
                let scores = fit(&resampled_design, intercept, &resampled_treated, model)
                    .map_err(EstimateError::Fit)?;
                let estimate = weighted_effect(&resampled_outcome, &resampled_treated,
                    &scores.propensity, specification, Target::Att).map_err(EstimateError::Weighting)?;
                (estimate.effect, scores.fitted)
            }
            Estimand::DoublyRobustAtt => {
                let estimate = doubly_robust_target(&resampled_design, &resampled_outcome,
                    &resampled_treated, intercept, model, Target::Att)?;
                (estimate.effect, estimate.fitted)
            }
        };
        if !fitted.converged() {
            unconverged.push(round);
        }
        estimates.push(effect);
    }
    if estimates.is_empty() {
        return Err(EstimateError::Weighting(Error::EmptySample));
    }
    Ok(Bootstrap {
        lower: crate::causal_effects::numpy_percentile(&estimates, lower_percentile),
        upper: crate::causal_effects::numpy_percentile(&estimates, upper_percentile),
        estimates,
        unconverged,
    })
}

#[derive(Clone, Debug)]
pub struct Matched {
    pub effect: f64,
    /// The opposite-arm outcome matched to each row, in source row order.
    pub matches: Vec<f64>,
}

/// Averages over every row, so the estimand is the ATE, not the ATT.
pub fn match_on_score(
    outcome: &[f64],
    treated: &[bool],
    propensity: &[f64],
) -> Result<Matched, Error> {
    match_on_score_target(outcome, treated, propensity, Target::Ate)
}

pub fn match_on_score_target(
    outcome: &[f64], treated: &[bool], propensity: &[f64], target: Target,
) -> Result<Matched, Error> {
    let n = outcome.len();
    if n == 0 {
        return Err(Error::EmptySample);
    }
    if treated.len() != n || propensity.len() != n {
        return Err(Error::LengthMismatch);
    }
    let treated_rows = treated.iter().filter(|&&t| t).count();
    if treated_rows == 0 {
        return Err(Error::MissingArm { treated: true });
    }
    if treated_rows == n {
        return Err(Error::MissingArm { treated: false });
    }
    for row in 0..n {
        if !outcome[row].is_finite() {
            return Err(Error::NonFiniteOutcome { row });
        }
        let p = propensity[row];
        if !p.is_finite() || p <= 0. || p >= 1. {
            return Err(Error::InvalidProbability { row });
        }
    }
    // Each arm holds the candidates for the other.
    let arm = |want: bool| -> (Vec<Vec<f64>>, Vec<f64>) {
        (0..n)
            .filter(|&row| treated[row] == want)
            .map(|row| (vec![propensity[row]], outcome[row]))
            .unzip()
    };
    let (control_scores, control_outcomes) = arm(false);
    let (treated_scores, treated_outcomes) = arm(true);
    let matches: Vec<f64> = (0..n)
        .map(|row| {
            let (scores, outcomes) = if treated[row] {
                (&control_scores, &control_outcomes)
            } else {
                (&treated_scores, &treated_outcomes)
            };
            crate::causal_effects::k_neighbors_predict(scores, outcomes, 1, &[propensity[row]])
        })
        .collect();
    let effect = (0..n)
        .filter(|&row| target == Target::Ate || treated[row])
        .map(|row| {
            let sign = if treated[row] { 1. } else { -1. };
            sign * (outcome[row] - matches[row])
        })
        .sum::<f64>()
        / if target == Target::Att { treated_rows as f64 } else { n as f64 };
    if !effect.is_finite() {
        return Err(Error::NumericalOverflow);
    }
    Ok(Matched { effect, matches })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Estimand {
    InverseProbability(Specification),
    DoublyRobust,
    InverseProbabilityAtt(Specification),
    DoublyRobustAtt,
}

#[derive(Clone, Debug)]
pub struct DoublyRobust {
    pub effect: f64,
    pub treated_term: f64,
    pub control_term: f64,
    pub fitted: FittedModel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EstimateError {
    Fit(FitError),
    Weighting(Error),
    OutcomeModel { treated: bool },
}

pub fn doubly_robust(
    design: &[Vec<f64>],
    outcome: &[f64],
    treated: &[bool],
    intercept: Intercept,
    model: TreatmentModel,
) -> Result<DoublyRobust, EstimateError> {
    doubly_robust_target(design, outcome, treated, intercept, model, Target::Ate)
}

pub fn doubly_robust_target(
    design: &[Vec<f64>], outcome: &[f64], treated: &[bool],
    intercept: Intercept, model: TreatmentModel, target: Target,
) -> Result<DoublyRobust, EstimateError> {
    let n = design.len();
    if outcome.len() != n || treated.len() != n {
        return Err(EstimateError::Weighting(Error::LengthMismatch));
    }
    let scores = fit(design, intercept, treated, model).map_err(EstimateError::Fit)?;
    for row in 0..n {
        if !outcome[row].is_finite() {
            return Err(EstimateError::Weighting(Error::NonFiniteOutcome { row }));
        }
        let p = scores.propensity[row];
        if !p.is_finite() || p <= 0. || p >= 1. {
            return Err(EstimateError::Weighting(Error::InvalidProbability { row }));
        }
    }

    let columns = design[0].len();
    let full = nalgebra::DMatrix::from_fn(n, columns, |row, column| design[row][column]);
    let arm_prediction = |want: bool| -> Result<nalgebra::DVector<f64>, EstimateError> {
        let rows: Vec<usize> = (0..n).filter(|&row| treated[row] == want).collect();
        let x = nalgebra::DMatrix::from_fn(rows.len(), columns, |row, column| {
            design[rows[row]][column]
        });
        let y = nalgebra::DVector::from_iterator(rows.len(), rows.iter().map(|&row| outcome[row]));
        let fitted = crate::sklearn_linear::fit_sklearn_linear_regression(
            &x,
            &y,
            crate::sklearn_linear::SKLEARN_LINEAR_TOLERANCE,
        )
        .map_err(|_| EstimateError::OutcomeModel { treated: want })?;
        Ok(crate::sklearn_linear::predict_fortran(
            &full,
            &fitted.coefficients,
            fitted.intercept,
        ))
    };
    let control_hat = arm_prediction(false)?;
    if target == Target::Att {
        let count = treated.iter().filter(|&&t| t).count() as f64;
        let treated_term = outcome.iter().zip(treated).filter(|(_, t)| **t)
            .map(|(y, _)| *y).sum::<f64>() / count;
        // ATTE score: D(Y-m0(X)) - (1-D)e(X)/(1-e(X))(Y-m0(X)), divided by sum D.
        let control_term = (0..n).map(|row| if treated[row] { control_hat[row] }
            else { scores.propensity[row] / (1. - scores.propensity[row])
                * (outcome[row] - control_hat[row]) }).sum::<f64>() / count;
        let effect = treated_term - control_term;
        if !effect.is_finite() { return Err(EstimateError::Weighting(Error::NumericalOverflow)); }
        return Ok(DoublyRobust { effect, treated_term, control_term, fitted: scores.fitted });
    }
    let treated_hat = arm_prediction(true)?;

    let mut treated_values = Vec::with_capacity(n);
    let mut control_values = Vec::with_capacity(n);
    for row in 0..n {
        let t = f64::from(treated[row]);
        let p = scores.propensity[row];
        treated_values.push(t * (outcome[row] - treated_hat[row]) / p + treated_hat[row]);
        control_values.push((1. - t) * (outcome[row] - control_hat[row]) / (1. - p) + control_hat[row]);
    }
    let treated_term = crate::numpy_reduce::numpy_mean(&treated_values);
    let control_term = crate::numpy_reduce::numpy_mean(&control_values);
    let effect = treated_term - control_term;
    if !effect.is_finite() {
        return Err(EstimateError::Weighting(Error::NumericalOverflow));
    }
    Ok(DoublyRobust {
        effect,
        treated_term,
        control_term,
        fitted: scores.fitted,
    })
}

#[derive(Clone, Copy, Debug)]
pub struct ScoreAdjusted {
    pub effect: f64,
    pub standard_error: f64,
}

/// The outcome regressed on the treatment and the fitted score.
pub fn adjusted_by_score(
    outcome: &[f64],
    treated: &[bool],
    propensity: &[f64],
) -> Result<ScoreAdjusted, Error> {
    let n = outcome.len();
    if n == 0 {
        return Err(Error::EmptySample);
    }
    if treated.len() != n || propensity.len() != n {
        return Err(Error::LengthMismatch);
    }
    let treated_rows = treated.iter().filter(|&&t| t).count();
    if treated_rows == 0 {
        return Err(Error::MissingArm { treated: true });
    }
    if treated_rows == n {
        return Err(Error::MissingArm { treated: false });
    }
    for row in 0..n {
        if !outcome[row].is_finite() {
            return Err(Error::NonFiniteOutcome { row });
        }
        let p = propensity[row];
        if !p.is_finite() || p <= 0. || p >= 1. {
            return Err(Error::InvalidProbability { row });
        }
    }
    let design = nalgebra::DMatrix::from_fn(n, 3, |row, column| match column {
        0 => 1.,
        1 => f64::from(treated[row]),
        _ => propensity[row],
    });
    let y = nalgebra::DVector::from_iterator(n, outcome.iter().copied());
    let fit = crate::ols::Ols::fit(&design, &y);
    let effect = fit.params[1];
    let standard_error = effect / fit.tvalues()[1];
    if !effect.is_finite() || !standard_error.is_finite() {
        return Err(Error::NumericalOverflow);
    }
    Ok(ScoreAdjusted {
        effect,
        standard_error,
    })
}

fn first_non_finite(design: &[Vec<f64>]) -> Option<(usize, usize)> {
    design.iter().enumerate().find_map(|(row, values)| {
        values
            .iter()
            .position(|value| !value.is_finite())
            .map(|column| (row, column))
    })
}

/// Encoding is not done here: one variable can be a number in one model and indicators in another.
pub fn fit(
    design: &[Vec<f64>],
    intercept: Intercept,
    treated: &[bool],
    model: TreatmentModel,
) -> Result<Scores, FitError> {
    let n = design.len();
    if n == 0 {
        return Err(FitError::EmptySample);
    }
    if treated.len() != n {
        return Err(FitError::LengthMismatch);
    }
    let columns = design[0].len();
    if columns == 0 {
        return Err(FitError::NoColumns);
    }
    if design.iter().any(|values| values.len() != columns) {
        return Err(FitError::LengthMismatch);
    }
    if let Some((row, column)) = first_non_finite(design) {
        return Err(FitError::NonFiniteDesign { row, column });
    }
    let treated_rows = treated.iter().filter(|&&t| t).count();
    if treated_rows == 0 {
        return Err(FitError::MissingArm { treated: true });
    }
    if treated_rows == n {
        return Err(FitError::MissingArm { treated: false });
    }
    let labels: Vec<f64> = treated.iter().map(|&t| f64::from(t)).collect();
    // The Newton fit reads the constant from the design; the L-BFGS-B fit holds its own apart.
    let prepared: Vec<Vec<f64>> = match (model, intercept) {
        (TreatmentModel::Newton(_), Intercept::Absent) => design
            .iter()
            .map(|row| std::iter::once(1.0).chain(row.iter().copied()).collect())
            .collect(),
        _ => design.to_vec(),
    };
    let design = prepared.as_slice();
    match model {
        TreatmentModel::Newton(settings) => {
            let fit = crate::logit::fit(design, &labels, settings).map_err(|cause| match cause {
                crate::logit::LogitError::StepSolveFailed { iteration } => {
                    FitError::StepSolveFailed { iteration }
                }
                crate::logit::LogitError::EmptySample => FitError::EmptySample,
                crate::logit::LogitError::NoColumns => FitError::NoColumns,
                crate::logit::LogitError::RowMismatch => FitError::LengthMismatch,
                crate::logit::LogitError::NonFiniteValue => first_non_finite(design)
                    .map_or(FitError::LengthMismatch, |(row, column)| {
                        FitError::NonFiniteDesign { row, column }
                    }),
            })?;
            Ok(Scores {
                propensity: fit.predict_probability(design),
                fitted: FittedModel::Newton {
                    params: fit.params,
                    iterations: fit.iterations,
                    termination: fit.termination,
                },
            })
        }
        TreatmentModel::Lbfgsb { max_iter } => {
            let fit = crate::logistic::fit_unpenalized(design, &labels, max_iter);
            Ok(Scores {
                propensity: fit.predict_probability(design),
                fitted: FittedModel::Lbfgsb {
                    coefficients: fit.coef,
                    intercept: fit.intercept,
                    termination: fit.termination,
                },
            })
        }
    }
}

#[derive(Debug)]
pub struct Estimate {
    pub specification: Specification,
    pub effect: f64,
    pub treated_mean: f64,
    pub control_mean: f64,
    /// Weights in original row order. No automatic clipping or row removal.
    pub weights: Vec<f64>,
    pub treated_weight_sum: f64,
    pub control_weight_sum: f64,
    pub treated_rows: usize,
    pub control_rows: usize,
}

pub fn ate(
    outcome: &[f64],
    treated: &[bool],
    propensity: &[f64],
    specification: Specification,
) -> Result<Estimate, Error> {
    weighted_effect(outcome, treated, propensity, specification, Target::Ate)
}

pub fn weighted_effect(
    outcome: &[f64], treated: &[bool], propensity: &[f64],
    specification: Specification, target: Target,
) -> Result<Estimate, Error> {
    let n = outcome.len();
    if n == 0 {
        return Err(Error::EmptySample);
    }
    if treated.len() != n || propensity.len() != n {
        return Err(Error::LengthMismatch);
    }
    let treated_rows = treated.iter().filter(|&&t| t).count();
    let control_rows = n - treated_rows;
    if treated_rows == 0 {
        return Err(Error::MissingArm { treated: true });
    }
    if control_rows == 0 {
        return Err(Error::MissingArm { treated: false });
    }
    let prevalence = treated_rows as f64 / n as f64;
    let (treated_scale, control_scale) = match specification.scale {
        WeightScale::InverseProbability => (1., 1.),
        WeightScale::Stabilized => (prevalence, 1. - prevalence),
    };
    let mut weights = Vec::with_capacity(n);
    let (mut treated_sum, mut control_sum, mut treated_weight_sum, mut control_weight_sum) =
        (0., 0., 0., 0.);
    for row in 0..n {
        if !outcome[row].is_finite() {
            return Err(Error::NonFiniteOutcome { row });
        }
        let p = propensity[row];
        if !p.is_finite() || p <= 0. || p >= 1. {
            return Err(Error::InvalidProbability { row });
        }
        let w = if treated[row] {
            if target == Target::Att { treated_scale } else { treated_scale / p }
        } else {
            control_scale * if target == Target::Att { p / (1. - p) } else { 1. / (1. - p) }
        };
        weights.push(w);
        if treated[row] {
            treated_sum += w * outcome[row];
            treated_weight_sum += w;
        } else {
            control_sum += w * outcome[row];
            control_weight_sum += w;
        }
    }
    let (treated_denominator, control_denominator) = match specification.normalization {
        Normalization::HorvitzThompson => {
            let count = if target == Target::Att { treated_rows } else { n } as f64;
            (count * treated_scale, count * control_scale)
        },
        Normalization::Hajek => (treated_weight_sum, control_weight_sum),
    };
    let treated_mean = treated_sum / treated_denominator;
    let control_mean = control_sum / control_denominator;
    let effect = treated_mean - control_mean;
    if [
        treated_sum,
        control_sum,
        treated_weight_sum,
        control_weight_sum,
        treated_mean,
        control_mean,
        effect,
    ]
    .iter()
    .any(|v| !v.is_finite())
    {
        return Err(Error::NumericalOverflow);
    }
    Ok(Estimate {
        specification,
        effect,
        treated_mean,
        control_mean,
        weights,
        treated_weight_sum,
        control_weight_sum,
        treated_rows,
        control_rows,
    })
}
