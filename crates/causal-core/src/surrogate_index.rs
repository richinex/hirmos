//! Two-sample linear surrogate index and normalized treatment contrast.
//! Independently implemented from equations (6.1) and (7.1) of Athey et al.
//! Reuses shared OLS and unpenalized logit kernels. Bootstrap inference refits
//! both samples using explicit, stratified resampling plans.
use crate::ols::Ols;
use nalgebra::{DMatrix, DVector};

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    InvalidTreatment,
    MissingArm,
    InvalidPropensity,
    RankDeficient,
    Fit,
    PropensityDidNotConverge,
    InvalidResample,
    InvalidBound,
}

pub struct Estimate {
    pub coefficients: Vec<f64>,
    pub predictions: Vec<f64>,
    pub effect: f64,
}

/// Gaussian validation regression, matching the replication's default glm
/// family. These tests require the primary outcome in the experimental sample.
pub struct ValidationCoefficient {
    pub estimate: f64,
    pub standard_error: f64,
    pub t_statistic: f64,
    pub residual_degrees_of_freedom: usize,
}

pub enum ValidationDiagnostic {
    Estimated(ValidationCoefficient),
    PerfectFit { estimate: f64, residual_degrees_of_freedom: usize },
}

pub struct ValidationRegressions {
    pub surrogacy: ValidationDiagnostic,
    pub comparability: ValidationDiagnostic,
}

pub struct OutcomePeriod {
    pub control_mean: f64,
    pub treated_mean: f64,
    pub contrast: f64,
    pub cumulative_contrast: f64,
}

pub struct ExperimentalOutcomePath {
    pub periods: Vec<OutcomePeriod>,
    /// Regression of each participant's across-period mean on treatment.
    /// Classical independent-participant inference, not a quarter-level SE.
    pub participant_benchmark: ValidationCoefficient,
    /// SD of quarterly contrasts divided by sqrt(number of quarters).
    /// Included solely to reproduce the source's plot benchmark calculation;
    /// it does not account for dependence between repeated quarters.
    pub quarter_contrast_standard_error: Option<f64>,
}

/// Outcomes are rows=participants, columns=chronologically ordered periods.
/// All values remain on the supplied scale; percentages belong to presentation.
/// Requires complete observations, avoiding unequal denominator drift by period.
pub fn experimental_outcome_path(
    outcomes: &DMatrix<f64>,
    w: &[f64],
) -> Result<ExperimentalOutcomePath, Error> {
    let periods = outcome_periods(outcomes, w)?;
    let y = DVector::from_iterator(
        outcomes.nrows(),
        (0..outcomes.nrows())
            .map(|i| outcomes.row(i).iter().sum::<f64>() / outcomes.ncols() as f64),
    );
    let x = DMatrix::from_fn(w.len(), 2, |i, j| if j == 0 { 1. } else { w[i] });
    let participant_benchmark = validation_indicator(&x, &y)?;
    let n = periods.len() as f64;
    let quarter_contrast_standard_error = if periods.len() > 1 {
        let mean = periods.last().unwrap().cumulative_contrast;
        let se = (periods
            .iter()
            .map(|p| (p.contrast - mean).powi(2))
            .sum::<f64>()
            / (n * (n - 1.)))
            .sqrt();
        if !se.is_finite() {
            return Err(Error::NonFinite);
        }
        Some(se)
    } else {
        None
    };
    Ok(ExperimentalOutcomePath {
        periods,
        participant_benchmark,
        quarter_contrast_standard_error,
    })
}

fn outcome_periods(outcomes: &DMatrix<f64>, w: &[f64]) -> Result<Vec<OutcomePeriod>, Error> {
    if outcomes.ncols() == 0 || outcomes.nrows() != w.len() || w.len() < 3 {
        return Err(Error::Shape);
    }
    if outcomes.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    if w.iter().any(|&v| v != 0. && v != 1.) {
        return Err(Error::InvalidTreatment);
    }
    let n1 = w.iter().filter(|&&v| v == 1.).count();
    let n0 = w.len() - n1;
    if n1 == 0 || n0 == 0 {
        return Err(Error::MissingArm);
    }
    let mut cumulative = 0.;
    let mut periods = Vec::with_capacity(outcomes.ncols());
    for t in 0..outcomes.ncols() {
        let mut sums = [0.; 2];
        for i in 0..outcomes.nrows() {
            sums[w[i] as usize] += outcomes[(i, t)];
        }
        let control_mean = sums[0] / n0 as f64;
        let treated_mean = sums[1] / n1 as f64;
        let contrast = treated_mean - control_mean;
        cumulative += contrast;
        if !cumulative.is_finite() {
            return Err(Error::NonFinite);
        }
        periods.push(OutcomePeriod {
            control_mean,
            treated_mean,
            contrast,
            cumulative_contrast: cumulative / (t + 1) as f64,
        });
    }
    Ok(periods)
}

/// One bootstrap distribution per cumulative horizon. Whole participant rows
/// are resampled within treatment arms, retaining cross-period dependence.
pub fn bootstrap_naive(
    outcomes: &DMatrix<f64>,
    w: &[f64],
    plans: &[Vec<usize>],
) -> Result<Vec<Bootstrap>, Error> {
    outcome_periods(outcomes, w)?;
    if plans.len() < 2 {
        return Err(Error::InvalidResample);
    }
    let treated = w.iter().filter(|&&v| v == 1.).count();
    let mut effects = vec![Vec::with_capacity(plans.len()); outcomes.ncols()];
    for indices in plans {
        if indices.len() != w.len() || indices.iter().any(|&i| i >= w.len()) {
            return Err(Error::InvalidResample);
        }
        let rw: Vec<_> = indices.iter().map(|&i| w[i]).collect();
        if rw.iter().filter(|&&v| v == 1.).count() != treated {
            return Err(Error::InvalidResample);
        }
        for (target, p) in effects
            .iter_mut()
            .zip(outcome_periods(&select_rows(outcomes, indices), &rw)?)
        {
            target.push(p.cumulative_contrast);
        }
    }
    effects.into_iter().map(summarize_bootstrap).collect()
}

pub fn validation_regressions(
    o: &DMatrix<f64>,
    oy: &DVector<f64>,
    e: &DMatrix<f64>,
    ey: &DVector<f64>,
    w: &[f64],
) -> Result<ValidationRegressions, Error> {
    if o.ncols() == 0
        || o.ncols() != e.ncols()
        || o.nrows() == 0
        || e.nrows() == 0
        || oy.len() != o.nrows()
        || ey.len() != e.nrows()
        || w.len() != e.nrows()
    {
        return Err(Error::Shape);
    }
    if w.iter().any(|&v| v != 0. && v != 1.) {
        return Err(Error::InvalidTreatment);
    }
    if !w.contains(&0.) || !w.contains(&1.) {
        return Err(Error::MissingArm);
    }
    let experimental = DMatrix::from_fn(e.nrows(), e.ncols() + 1, |i, j| {
        if j == e.ncols() {
            w[i]
        } else {
            e[(i, j)]
        }
    });
    let combined = DMatrix::from_fn(e.nrows() + o.nrows(), e.ncols() + 1, |i, j| {
        if j == e.ncols() {
            if i < e.nrows() {
                1.
            } else {
                0.
            }
        } else if i < e.nrows() {
            e[(i, j)]
        } else {
            o[(i - e.nrows(), j)]
        }
    });
    let outcome = DVector::from_iterator(ey.len() + oy.len(), ey.iter().chain(oy.iter()).copied());
    Ok(ValidationRegressions {
        surrogacy: validation_diagnostic(&experimental, ey, true)?,
        comparability: validation_diagnostic(&combined, &outcome, true)?,
    })
}

fn validation_indicator(x: &DMatrix<f64>, y: &DVector<f64>) -> Result<ValidationCoefficient, Error> {
    match validation_diagnostic(x, y, false)? {
        ValidationDiagnostic::Estimated(value) => Ok(value),
        ValidationDiagnostic::PerfectFit { .. } => Err(Error::Fit),
    }
}

fn validation_diagnostic(x: &DMatrix<f64>, y: &DVector<f64>, detect_perfect_fit: bool) -> Result<ValidationDiagnostic, Error> {
    if x.iter().chain(y.iter()).any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let fit = Ols::try_fit(x, y).map_err(|_| Error::Fit)?;
    if fit.rank != x.ncols() {
        return Err(Error::RankDeficient);
    }
    if fit.nobs <= fit.rank {
        return Err(Error::Fit);
    }
    let j = x.ncols() - 1;
    let df = fit.nobs - fit.rank;
    // Scale-aware backward-error check, not a coefficient or p-value threshold.
    // Includes the design/parameter product so rescaling does not change the decision.
    let reconstruction_scale = y.norm() + x.norm() * fit.params.norm();
    if detect_perfect_fit && fit.ssr.sqrt() <= 64. * f64::EPSILON * reconstruction_scale {
        return Ok(ValidationDiagnostic::PerfectFit { estimate: fit.params[j], residual_degrees_of_freedom: df });
    }
    let se = (fit.ssr / df as f64 * fit.xtx_inverse()[(j, j)]).sqrt();
    let estimate = fit.params[j];
    let t = estimate / se;
    if se <= 0. || !se.is_finite() || !t.is_finite() {
        return Err(Error::Fit);
    }
    Ok(ValidationDiagnostic::Estimated(ValidationCoefficient {
        estimate,
        standard_error: se,
        t_statistic: t,
        residual_degrees_of_freedom: df,
    }))
}

#[derive(Clone, Copy)]
pub struct BiasNuisance {
    pub index: f64,
    pub surrogate: f64,
    pub propensity: f64,
}

pub enum BiasRestriction {
    BinaryWithoutSurrogacy,
    BinaryWithoutComparability,
    BoundedDirectEffect { maximum: f64 },
    BoundedSampleDifference { maximum: f64 },
}

/// Bounds on tau minus the surrogate estimand, not effect confidence intervals.
pub struct BiasBounds {
    pub lower: f64,
    pub upper: f64,
}

pub struct FittedBiasBounds {
    pub bounds: BiasBounds,
    pub index_predictions: Vec<f64>,
    pub surrogate_probabilities: Vec<f64>,
    pub propensity_probabilities: Vec<f64>,
}

/// Fit nuisance models for Section 5.2 on the supplied outcome scale and design.
/// No implicit division by horizon or selection of surrogate columns is applied.
/// The Table 9 replication specification must therefore supply its divided
/// outcome explicitly; it is not silently substituted for the paper's outcome.
pub fn fit_bias_bounds(
    o: &DMatrix<f64>,
    y: &DVector<f64>,
    e: &DMatrix<f64>,
    w: &[f64],
    baseline: Propensity<'_>,
    restriction: BiasRestriction,
) -> Result<FittedBiasBounds, Error> {
    if matches!(
        restriction,
        BiasRestriction::BinaryWithoutSurrogacy | BiasRestriction::BinaryWithoutComparability
    ) && y.iter().any(|v| !v.is_finite() || !(0. ..=1.).contains(v))
    {
        return Err(Error::InvalidBound);
    }
    let fitted = fit_index(o, y, e, w, baseline)?;
    // Predict from the linear predictor, not the upstream experimental-score
    // return, which applies a second logistic transform to probabilities.
    let surrogate = logit_predict(e, w, e)?;
    let rows: Vec<_> = (0..e.nrows())
        .map(|i| BiasNuisance {
            index: fitted.index.predictions[i],
            surrogate: surrogate[i],
            propensity: fitted.propensity[i],
        })
        .collect();
    let bounds = bias_bounds(&rows, restriction)?;
    Ok(FittedBiasBounds {
        bounds,
        index_predictions: fitted.index.predictions,
        surrogate_probabilities: surrogate,
        propensity_probabilities: fitted.propensity,
    })
}

/// Empirical forms of Section 5.2, Lemmas 1 and 2, evaluated on E rows.
/// Binary bounds reject index predictions outside [0,1]; no silent clipping.
pub fn bias_bounds(
    rows: &[BiasNuisance],
    restriction: BiasRestriction,
) -> Result<BiasBounds, Error> {
    if rows.is_empty() {
        return Err(Error::Shape);
    }
    let maximum = match restriction {
        BiasRestriction::BoundedDirectEffect { maximum }
        | BiasRestriction::BoundedSampleDifference { maximum } => {
            if !maximum.is_finite() || maximum < 0. {
                return Err(Error::InvalidBound);
            }
            maximum
        }
        _ => 0.,
    };
    let mut lower = 0.;
    let mut upper = 0.;
    for row in rows {
        let (mu, r, p) = (row.index, row.surrogate, row.propensity);
        if !r.is_finite() || !(0. ..=1.).contains(&r) || !p.is_finite() || p <= 0. || p >= 1. {
            return Err(Error::InvalidPropensity);
        }
        let denom = p * (1. - p);
        let (lo, hi) = match restriction {
            BiasRestriction::BinaryWithoutSurrogacy
            | BiasRestriction::BinaryWithoutComparability => {
                if !mu.is_finite() || !(0. ..=1.).contains(&mu) {
                    return Err(Error::InvalidBound);
                }
                if matches!(restriction, BiasRestriction::BinaryWithoutSurrogacy) {
                    // Algebraically cancel r*(1-r) before division, including endpoints.
                    (
                        -((1. - mu) * (1. - r)).min(mu * r) / denom,
                        (mu * (1. - r)).min((1. - mu) * r) / denom,
                    )
                } else {
                    (
                        ((if r < p { 1. } else { 0. }) - mu) * (r - p) / denom,
                        ((if r > p { 1. } else { 0. }) - mu) * (r - p) / denom,
                    )
                }
            }
            BiasRestriction::BoundedDirectEffect { .. } => {
                let width = maximum * r * (1. - r) / denom;
                (-width, width)
            }
            BiasRestriction::BoundedSampleDifference { .. } => {
                let width = maximum * (r - p).abs() / denom;
                (-width, width)
            }
        };
        lower += lo;
        upper += hi;
    }
    lower /= rows.len() as f64;
    upper /= rows.len() as f64;
    if !lower.is_finite() || !upper.is_finite() {
        return Err(Error::NonFinite);
    }
    Ok(BiasBounds { lower, upper })
}

/// Treatment prevalence for the randomized contrast, or an unpenalized model of
/// treatment using baseline covariates only. The caller supplies the intercept.
pub enum Propensity<'a> {
    TreatmentPrevalence,
    BaselineLogit(&'a DMatrix<f64>),
}

pub struct FittedIndex {
    pub index: Estimate,
    pub propensity: Vec<f64>,
    pub propensity_iterations: Option<usize>,
}

pub fn fit_index(
    o: &DMatrix<f64>,
    y: &DVector<f64>,
    e: &DMatrix<f64>,
    treatment: &[f64],
    model: Propensity<'_>,
) -> Result<FittedIndex, Error> {
    if treatment.len() != e.nrows() || treatment.is_empty() {
        return Err(Error::Shape);
    }
    if treatment.iter().any(|&w| w != 0. && w != 1.) {
        return Err(Error::InvalidTreatment);
    }
    if !treatment.contains(&0.) || !treatment.contains(&1.) {
        return Err(Error::MissingArm);
    }
    let (propensity, iterations) = match model {
        Propensity::TreatmentPrevalence => (
            vec![treatment.iter().sum::<f64>() / treatment.len() as f64; treatment.len()],
            None,
        ),
        Propensity::BaselineLogit(x) => {
            if x.nrows() != e.nrows() || x.ncols() == 0 {
                return Err(Error::Shape);
            }
            // Reuse the shared solver but disable its numerical ridge. This is
            // an unpenalized likelihood, not sklearn's regularized default.
            let design: Vec<Vec<f64>> = (0..x.nrows())
                .map(|i| x.row(i).iter().copied().collect())
                .collect();
            let fit = crate::logit::fit(
                &design,
                treatment,
                crate::logit::Settings {
                    max_iter: 100,
                    tolerance: 1e-10,
                    ridge_factor: 0.,
                },
            )
            .map_err(|_| Error::Fit)?;
            if !fit.termination.converged() || fit.params.iter().any(|v| !v.is_finite()) {
                return Err(Error::PropensityDidNotConverge);
            }
            (fit.predict_probability(&design), Some(fit.iterations))
        }
    };
    let index = linear_index(o, y, e, treatment, &propensity)?;
    Ok(FittedIndex {
        index,
        propensity,
        propensity_iterations: iterations,
    })
}

/// Explicit indices permit exact R resampling replay without claiming identical RNGs.
#[derive(Debug, PartialEq)]
pub struct Resample {
    pub experimental: Vec<usize>,
    pub observational: Vec<usize>,
}

/// Reproducible application resampling, not R's RNG stream. Exact oracle replay
/// uses recorded plans instead. Experimental arm sizes are preserved.
/// Participant-only bootstrap plans, stratified by experimental treatment arm.
pub fn experimental_resampling_plan(
    treatment: &[f64],
    repetitions: usize,
    seed: u64,
) -> Result<Vec<Vec<usize>>, Error> {
    if repetitions < 2 {
        return Err(Error::InvalidResample);
    }
    if treatment.iter().any(|&w| w != 0. && w != 1.) {
        return Err(Error::InvalidTreatment);
    }
    let treated: Vec<_> = (0..treatment.len())
        .filter(|&i| treatment[i] == 1.)
        .collect();
    let controls: Vec<_> = (0..treatment.len())
        .filter(|&i| treatment[i] == 0.)
        .collect();
    if treated.is_empty() || controls.is_empty() {
        return Err(Error::MissingArm);
    }
    let mut rng = crate::nprandom::NpRng::seeded(seed);
    Ok((0..repetitions)
        .map(|_| draw_experimental_rows(&treated, &controls, &mut rng))
        .collect())
}

fn draw_experimental_rows(
    treated: &[usize],
    controls: &[usize],
    rng: &mut crate::nprandom::NpRng,
) -> Vec<usize> {
    let mut rows: Vec<_> = treated
        .iter()
        .map(|_| treated[rng.bounded_uint64(treated.len() as u64) as usize])
        .collect();
    rows.extend(
        controls
            .iter()
            .map(|_| controls[rng.bounded_uint64(controls.len() as u64) as usize]),
    );
    rows
}

pub fn resampling_plan(
    treatment: &[f64],
    observational_rows: usize,
    repetitions: usize,
    seed: u64,
) -> Result<Vec<Resample>, Error> {
    if observational_rows == 0 || repetitions < 2 {
        return Err(Error::InvalidResample);
    }
    if treatment.iter().any(|&w| w != 0. && w != 1.) {
        return Err(Error::InvalidTreatment);
    }
    let treated: Vec<_> = (0..treatment.len())
        .filter(|&i| treatment[i] == 1.)
        .collect();
    let controls: Vec<_> = (0..treatment.len())
        .filter(|&i| treatment[i] == 0.)
        .collect();
    if treated.is_empty() || controls.is_empty() {
        return Err(Error::MissingArm);
    }
    let mut rng = crate::nprandom::NpRng::seeded(seed);
    Ok((0..repetitions)
        .map(|_| {
            let experimental = draw_experimental_rows(&treated, &controls, &mut rng);
            Resample {
                experimental,
                observational: (0..observational_rows)
                    .map(|_| rng.bounded_uint64(observational_rows as u64) as usize)
                    .collect(),
            }
        })
        .collect())
}

pub struct ScoreEstimate {
    pub effect: f64,
    pub surrogate: Vec<f64>,
    pub sampling: Vec<f64>,
    pub propensity: Vec<f64>,
}

fn logit_predict(
    train: &DMatrix<f64>,
    w: &[f64],
    target: &DMatrix<f64>,
) -> Result<Vec<f64>, Error> {
    if train.ncols() != target.ncols()
        || train.nrows() != w.len()
        || train.ncols() == 0
        || target.nrows() == 0
    {
        return Err(Error::Shape);
    }
    if train.iter().chain(target.iter()).any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let rows = |x: &DMatrix<f64>| {
        (0..x.nrows())
            .map(|i| x.row(i).iter().copied().collect())
            .collect::<Vec<Vec<f64>>>()
    };
    let fit = crate::logit::fit(
        &rows(train),
        w,
        crate::logit::Settings {
            max_iter: 100,
            tolerance: 1e-10,
            ridge_factor: 0.,
        },
    )
    .map_err(|_| Error::Fit)?;
    if !fit.termination.converged() || fit.params.iter().any(|v| !v.is_finite()) {
        return Err(Error::PropensityDidNotConverge);
    }
    let p = fit.predict_probability(&rows(target));
    if p.iter().any(|&v| !v.is_finite() || v <= 0. || v >= 1.) {
        return Err(Error::InvalidPropensity);
    }
    Ok(p)
}

/// Score weighting in equations 6.2–6.3. Designs include an intercept.
/// Baseline designs are supplied as a pair, or omitted together.
pub fn fit_score(
    o: &DMatrix<f64>,
    y: &DVector<f64>,
    e: &DMatrix<f64>,
    w: &[f64],
    baseline: Option<(&DMatrix<f64>, &DMatrix<f64>)>,
) -> Result<ScoreEstimate, Error> {
    if o.nrows() != y.len() || o.nrows() == 0 || e.nrows() != w.len() || o.ncols() != e.ncols() {
        return Err(Error::Shape);
    }
    if y.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    if w.iter().any(|&v| v != 0. && v != 1.) {
        return Err(Error::InvalidTreatment);
    }
    if !w.contains(&0.) || !w.contains(&1.) {
        return Err(Error::MissingArm);
    }
    let surrogate = logit_predict(e, w, o)?;
    let combined = DMatrix::from_fn(e.nrows() + o.nrows(), o.ncols(), |i, j| {
        if i < e.nrows() {
            e[(i, j)]
        } else {
            o[(i - e.nrows(), j)]
        }
    });
    let membership: Vec<_> = (0..combined.nrows())
        .map(|i| if i < e.nrows() { 1. } else { 0. })
        .collect();
    let sampling = logit_predict(&combined, &membership, o)?;
    let prop = match baseline {
        None => vec![w.iter().sum::<f64>() / w.len() as f64; o.nrows()],
        Some((be, bo)) => {
            if be.nrows() != e.nrows() || bo.nrows() != o.nrows() {
                return Err(Error::Shape);
            }
            logit_predict(be, w, bo)?
        }
    };
    let q = e.nrows() as f64 / combined.nrows() as f64;
    let mut totals = [0.; 2];
    let mut values = [0.; 2];
    for i in 0..o.nrows() {
        let common = sampling[i] * (1. - q) / ((1. - sampling[i]) * q);
        let weights = [
            (1. - surrogate[i]) * common / (1. - prop[i]),
            surrogate[i] * common / prop[i],
        ];
        for arm in 0..2 {
            totals[arm] += weights[arm];
            values[arm] += weights[arm] * y[i];
        }
    }
    let effect = values[1] / totals[1] - values[0] / totals[0];
    if !effect.is_finite() || totals.iter().chain(values.iter()).any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    Ok(ScoreEstimate {
        effect,
        surrogate,
        sampling,
        propensity: prop,
    })
}

/// Nuisance values at one experimental observation. These are predictions, not
/// observed individual treatment effects.
#[derive(Clone, Copy)]
pub struct ExperimentalNuisance {
    pub treatment: f64,
    pub propensity: f64,
    pub index: f64,
    pub arm_one: f64,
    pub arm_zero: f64,
}

#[derive(Clone, Copy)]
pub struct ObservationalNuisance {
    pub outcome: f64,
    pub index: f64,
    pub propensity: f64,
    pub surrogate: f64,
    pub sampling: f64,
}

pub struct InfluenceEstimate {
    pub effect: f64,
    pub experimental_term: f64,
    pub observational_term: f64,
}

/// Empirical average of the uncentered score in equations 4.4 and 6.4.
/// The printed 6.4 sum omits the 1/N factor; averaging follows 4.5 and the
/// replication's sum/n. No normalization of individual arm weights is applied.
/// No variance formula is inferred from treating fitted nuisance values as known.
pub fn influence_from_nuisances(
    experimental: &[ExperimentalNuisance],
    observational: &[ObservationalNuisance],
) -> Result<InfluenceEstimate, Error> {
    if experimental.is_empty() || observational.is_empty() {
        return Err(Error::Shape);
    }
    let probability = |p: f64| p.is_finite() && p > 0. && p < 1.;
    let mut arms = [false; 2];
    let mut first = 0.;
    for r in experimental {
        if r.treatment != 0. && r.treatment != 1. {
            return Err(Error::InvalidTreatment);
        }
        arms[r.treatment as usize] = true;
        if !probability(r.propensity) {
            return Err(Error::InvalidPropensity);
        }
        if [r.index, r.arm_one, r.arm_zero]
            .iter()
            .any(|v| !v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        first += r.arm_one - r.arm_zero + r.treatment * (r.index - r.arm_one) / r.propensity
            - (1. - r.treatment) * (r.index - r.arm_zero) / (1. - r.propensity);
    }
    if !arms[0] || !arms[1] {
        return Err(Error::MissingArm);
    }
    let mut correction = 0.;
    for r in observational {
        if ![r.propensity, r.surrogate, r.sampling]
            .iter()
            .all(|&p| probability(p))
        {
            return Err(Error::InvalidPropensity);
        }
        if !r.outcome.is_finite() || !r.index.is_finite() {
            return Err(Error::NonFinite);
        }
        correction +=
            r.sampling / (1. - r.sampling) * (r.outcome - r.index) * (r.surrogate - r.propensity)
                / (r.propensity * (1. - r.propensity));
    }
    let experimental_term = first / experimental.len() as f64;
    let observational_term = correction / experimental.len() as f64;
    let effect = experimental_term + observational_term;
    if !effect.is_finite() {
        return Err(Error::NonFinite);
    }
    Ok(InfluenceEstimate {
        effect,
        experimental_term,
        observational_term,
    })
}

/// Linear outcome index, additive arm regression on baseline covariates, and
/// unpenalized logistic score models. Baseline designs include an intercept and
/// are ordered experimental, observational. This follows the paper's score,
/// not the replication function's scalar-index substitution.
pub fn fit_influence(
    o: &DMatrix<f64>,
    y: &DVector<f64>,
    e: &DMatrix<f64>,
    w: &[f64],
    baseline: Option<(&DMatrix<f64>, &DMatrix<f64>)>,
) -> Result<InfluenceEstimate, Error> {
    let fitted = fit_index(
        o,
        y,
        e,
        w,
        match baseline {
            Some((be, _)) => Propensity::BaselineLogit(be),
            None => Propensity::TreatmentPrevalence,
        },
    )?;
    let score = fit_score(o, y, e, w, baseline)?;
    let intercept = DMatrix::from_element(e.nrows(), 1, 1.);
    let be = baseline.map(|(be, _)| be).unwrap_or(&intercept);
    let design = DMatrix::from_fn(e.nrows(), be.ncols() + 1, |i, j| {
        if j == be.ncols() {
            w[i]
        } else {
            be[(i, j)]
        }
    });
    let arm_fit = Ols::try_fit(
        &design,
        &DVector::from_vec(fitted.index.predictions.clone()),
    )
    .map_err(|_| Error::Fit)?;
    if arm_fit.rank != design.ncols() {
        return Err(Error::RankDeficient);
    }
    let zero = be * arm_fit.params.rows(0, be.ncols());
    let contrast = arm_fit.params[be.ncols()];
    let mu_o = o * DVector::from_vec(fitted.index.coefficients);
    let en: Vec<_> = (0..e.nrows())
        .map(|i| ExperimentalNuisance {
            treatment: w[i],
            propensity: fitted.propensity[i],
            index: fitted.index.predictions[i],
            arm_zero: zero[i],
            arm_one: zero[i] + contrast,
        })
        .collect();
    let on: Vec<_> = (0..o.nrows())
        .map(|i| ObservationalNuisance {
            outcome: y[i],
            index: mu_o[i],
            propensity: score.propensity[i],
            surrogate: score.surrogate[i],
            sampling: score.sampling[i],
        })
        .collect();
    influence_from_nuisances(&en, &on)
}
pub struct Bootstrap {
    pub effects: Vec<f64>,
    pub standard_error: f64,
}

pub fn bootstrap_index(
    o: &DMatrix<f64>,
    y: &DVector<f64>,
    e: &DMatrix<f64>,
    treatment: &[f64],
    model: Propensity<'_>,
    resamples: &[Resample],
) -> Result<Bootstrap, Error> {
    // Validate the base fit even when a resample happens to omit an invalid row.
    fit_index(
        o,
        y,
        e,
        treatment,
        match &model {
            Propensity::TreatmentPrevalence => Propensity::TreatmentPrevalence,
            Propensity::BaselineLogit(x) => Propensity::BaselineLogit(x),
        },
    )?;
    bootstrap_refits(o, y, e, treatment, resamples, |ro, ry, re, w, sample| {
        let rx = match &model {
            Propensity::TreatmentPrevalence => None,
            Propensity::BaselineLogit(x) => Some(select_rows(x, &sample.experimental)),
        };
        Ok(fit_index(
            ro,
            ry,
            re,
            w,
            match &rx {
                Some(x) => Propensity::BaselineLogit(x),
                None => Propensity::TreatmentPrevalence,
            },
        )?
        .index
        .effect)
    })
}

/// Two-sample bootstrap with all three score models refitted on each draw.
/// Baseline designs are ordered experimental, observational.
pub fn bootstrap_score(
    o: &DMatrix<f64>,
    y: &DVector<f64>,
    e: &DMatrix<f64>,
    treatment: &[f64],
    baseline: Option<(&DMatrix<f64>, &DMatrix<f64>)>,
    resamples: &[Resample],
) -> Result<Bootstrap, Error> {
    fit_score(o, y, e, treatment, baseline)?;
    bootstrap_refits(o, y, e, treatment, resamples, |ro, ry, re, w, sample| {
        let rb = baseline.map(|(be, bo)| {
            (
                select_rows(be, &sample.experimental),
                select_rows(bo, &sample.observational),
            )
        });
        Ok(fit_score(ro, ry, re, w, rb.as_ref().map(|(be, bo)| (be, bo)))?.effect)
    })
}

/// Every draw refits outcome, arm, propensity, surrogate and sampling models.
pub fn bootstrap_influence(
    o: &DMatrix<f64>,
    y: &DVector<f64>,
    e: &DMatrix<f64>,
    treatment: &[f64],
    baseline: Option<(&DMatrix<f64>, &DMatrix<f64>)>,
    resamples: &[Resample],
) -> Result<Bootstrap, Error> {
    fit_influence(o, y, e, treatment, baseline)?;
    bootstrap_refits(o, y, e, treatment, resamples, |ro, ry, re, w, sample| {
        let rb = baseline.map(|(be, bo)| {
            (
                select_rows(be, &sample.experimental),
                select_rows(bo, &sample.observational),
            )
        });
        Ok(fit_influence(ro, ry, re, w, rb.as_ref().map(|(be, bo)| (be, bo)))?.effect)
    })
}

fn select_rows(x: &DMatrix<f64>, rows: &[usize]) -> DMatrix<f64> {
    DMatrix::from_fn(rows.len(), x.ncols(), |i, j| x[(rows[i], j)])
}

fn bootstrap_refits(
    o: &DMatrix<f64>,
    y: &DVector<f64>,
    e: &DMatrix<f64>,
    treatment: &[f64],
    resamples: &[Resample],
    refit: impl Fn(&DMatrix<f64>, &DVector<f64>, &DMatrix<f64>, &[f64], &Resample) -> Result<f64, Error>,
) -> Result<Bootstrap, Error> {
    if resamples.len() < 2 {
        return Err(Error::InvalidResample);
    }
    let treated = treatment.iter().filter(|&&w| w == 1.).count();
    let mut effects = Vec::with_capacity(resamples.len());
    for sample in resamples {
        if sample.experimental.len() != e.nrows()
            || sample.observational.len() != o.nrows()
            || sample.experimental.iter().any(|&i| i >= e.nrows())
            || sample.observational.iter().any(|&i| i >= o.nrows())
        {
            return Err(Error::InvalidResample);
        }
        let w: Vec<_> = sample.experimental.iter().map(|&i| treatment[i]).collect();
        if w.iter().filter(|&&v| v == 1.).count() != treated {
            return Err(Error::InvalidResample);
        }
        let ro = select_rows(o, &sample.observational);
        let re = select_rows(e, &sample.experimental);
        let ry = DVector::from_iterator(
            sample.observational.len(),
            sample.observational.iter().map(|&i| y[i]),
        );
        effects.push(refit(&ro, &ry, &re, &w, sample)?);
    }
    summarize_bootstrap(effects)
}

/// Summarize an explicitly supplied, complete collection of bootstrap estimates.
/// Missing or failed draws are errors; callers must not silently discard them.
pub fn summarize_bootstrap(effects: Vec<f64>) -> Result<Bootstrap, Error> {
    if effects.len() < 2 {
        return Err(Error::InvalidResample);
    }
    let mean = effects.iter().sum::<f64>() / effects.len() as f64;
    let standard_error = (effects.iter().map(|v| (v - mean).powi(2)).sum::<f64>()
        / (effects.len() - 1) as f64)
        .sqrt();
    if !standard_error.is_finite() {
        return Err(Error::NonFinite);
    }
    Ok(Bootstrap {
        effects,
        standard_error,
    })
}

/// Both designs must contain the same columns, including any intercept.
/// Surrogates and baseline covariates belong in the index design, not treatment.
/// No second-stage-only standard error is reported as full two-sample inference.
pub fn linear_index(
    observational: &DMatrix<f64>,
    outcome: &DVector<f64>,
    experimental: &DMatrix<f64>,
    treatment: &[f64],
    propensity: &[f64],
) -> Result<Estimate, Error> {
    let n = experimental.nrows();
    if n == 0
        || observational.nrows() == 0
        || observational.ncols() == 0
        || observational.ncols() != experimental.ncols()
        || outcome.len() != observational.nrows()
        || treatment.len() != n
        || propensity.len() != n
    {
        return Err(Error::Shape);
    }
    if observational
        .iter()
        .chain(experimental.iter())
        .chain(outcome.iter())
        .any(|x| !x.is_finite())
    {
        return Err(Error::NonFinite);
    }
    if treatment.iter().any(|&x| x != 0.0 && x != 1.0) {
        return Err(Error::InvalidTreatment);
    }
    if !treatment.contains(&0.0) || !treatment.contains(&1.0) {
        return Err(Error::MissingArm);
    }
    if propensity
        .iter()
        .any(|&p| !p.is_finite() || p <= 0.0 || p >= 1.0)
    {
        return Err(Error::InvalidPropensity);
    }
    let fit = Ols::try_fit(observational, outcome).map_err(|_| Error::Fit)?;
    // R's lm alias handling differs from minimum-norm prediction. Do not silently substitute it.
    if fit.rank != observational.ncols() {
        return Err(Error::RankDeficient);
    }
    let predictions = experimental * &fit.params;
    let mut total = [0.0; 2];
    let mut weighted = [0.0; 2];
    for i in 0..n {
        let arm = treatment[i] as usize;
        let weight = 1.0
            / if arm == 1 {
                propensity[i]
            } else {
                1.0 - propensity[i]
            };
        total[arm] += weight;
        weighted[arm] += weight * predictions[i];
    }
    let effect = weighted[1] / total[1] - weighted[0] / total[0];
    if !effect.is_finite()
        || total
            .iter()
            .chain(weighted.iter())
            .chain(predictions.iter())
            .any(|x| !x.is_finite())
    {
        return Err(Error::NonFinite);
    }
    Ok(Estimate {
        coefficients: fit.params.as_slice().to_vec(),
        predictions: predictions.as_slice().to_vec(),
        effect,
    })
}
