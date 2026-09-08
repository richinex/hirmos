//! Identity- and log-link count time-series regression following `tscount::tsglm`.
//!
//! The conditional mean contains transformed past observations, past linear predictors, and
//! optional regressors. Mean parameters are estimated by Poisson quasi-maximum likelihood;
//! negative-binomial overdispersion is then estimated by tscount's Pearson root. This separation
//! is intentional and differs from fitting a static negative-binomial regression to lag columns.

use nalgebra::{DMatrix, DVector};

#[derive(Clone, Debug, PartialEq)]
pub struct IngarchSpecification {
    pub link: IngarchLink,
    pub past_observation_lags: Vec<usize>,
    pub past_mean_lags: Vec<usize>,
    /// `true` reproduces tscount's external-covariate correction in the past-mean recursion.
    pub external_regressors: Vec<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IngarchLink {
    /// The INGARCH recursion is directly on the conditional-count mean.
    Identity,
    /// The recursion is on the log conditional mean, using `log(y + 1)` for past counts.
    Log,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InterventionSchedule {
    Point,
    Persistent,
    Decaying { delta: f64 },
}

#[derive(Clone, Debug, PartialEq)]
pub enum IngarchError {
    EmptySeries,
    InvalidCount { row: usize, value: f64 },
    InvalidRegressorShape,
    InvalidLag,
    InvalidExternalRegressors,
    NonFiniteRegressor { row: usize, column: usize },
    IdentityRequiresNonNegativeRegressor { row: usize, column: usize },
    NonStationaryInitialization,
    InvalidSchedule,
    InvalidDetectionCandidates,
    SingularDetectionInformation,
    OptimizerFailed,
    DispersionNotEstimable,
}

#[derive(Clone, Debug)]
pub struct IngarchInterventionCandidate {
    /// Zero-based reference point in the input series. `tscount` reports this value plus one.
    pub reference_point: usize,
    pub score_statistic: f64,
}

#[derive(Clone, Debug)]
pub struct IngarchInterventionDetection {
    pub null_fit: IngarchFit,
    pub candidates: Vec<IngarchInterventionCandidate>,
    pub strongest_reference_point: usize,
    pub delta: f64,
}

#[derive(Clone, Debug)]
pub struct IngarchFit {
    pub specification: IngarchSpecification,
    /// intercept, past-observation coefficients, past-mean coefficients, regressor coefficients.
    pub parameters: Vec<f64>,
    pub start_parameters: Vec<f64>,
    pub fitted_means: Vec<f64>,
    pub linear_predictors: Vec<f64>,
    pub residuals: Vec<f64>,
    /// Full negative-binomial log likelihood evaluated at the quasi-likelihood mean fit.
    pub log_likelihood: f64,
    /// Negative-binomial size, `1 / dispersion`.
    pub size: f64,
    /// tscount's `sigmasq` overdispersion coefficient.
    pub dispersion: f64,
    pub score: Vec<f64>,
    pub iterations: usize,
    pub function_evaluations: usize,
    pub gradient_evaluations: usize,
}

#[derive(Clone, Debug)]
struct Recursion {
    nu: Vec<f64>,
    mean: Vec<f64>,
    derivatives: Vec<f64>,
    n_parameters: usize,
}

struct VmminResult {
    parameters: Vec<f64>,
    value: f64,
    gradient: Vec<f64>,
    iterations: usize,
    function_evaluations: usize,
    gradient_evaluations: usize,
    converged: bool,
}

fn identity_constraints(parameters: &[f64], dynamic_parameters: usize) -> Vec<f64> {
    // `tsglm` sets `slackvar = 0` for its default `final.control = list()` before it expands the
    // constrained-optimisation defaults. `epsilon = 1e-6` affects only the projected start.
    const SLACK: f64 = 1e-6;
    let mut values = Vec::with_capacity(parameters.len() + 1);
    values.push(parameters[0] - SLACK);
    values.extend_from_slice(&parameters[1..]);
    values.push(1.0 - SLACK - parameters[1..1 + dynamic_parameters].iter().sum::<f64>());
    values
}

fn identity_barrier(
    parameters: &[f64],
    previous: &[f64],
    dynamic_parameters: usize,
) -> Option<(f64, Vec<f64>)> {
    let gi = identity_constraints(parameters, dynamic_parameters);
    let gi_previous = identity_constraints(previous, dynamic_parameters);
    if gi.iter().any(|value| *value < 0.0) {
        return None;
    }
    let mut barrier = 0.0;
    let mut derivative = vec![0.0; parameters.len()];
    for index in 0..parameters.len() {
        barrier += gi_previous[index] * gi[index].ln() - parameters[index];
        derivative[index] += gi_previous[index] / gi[index] - 1.0;
    }
    let last = parameters.len();
    let sum = parameters[1..1 + dynamic_parameters].iter().sum::<f64>();
    barrier += gi_previous[last] * gi[last].ln() + sum;
    for value in &mut derivative[1..1 + dynamic_parameters] {
        *value += 1.0 - gi_previous[last] / gi[last];
    }
    barrier.is_finite().then_some((barrier, derivative))
}

/// R's `constrOptim` logarithmic-barrier outer loop with its default controls and BFGS inner loop.
fn constrained_identity_vmmin<F, G>(
    log_likelihood: F,
    score: G,
    start: &[f64],
    dynamic_parameters: usize,
) -> VmminResult
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
{
    const MU: f64 = -1e-4; // `fnscale = -1` changes constrOptim's default sign.
    const OUTER_EPS: f64 = 1e-5;
    let mut theta = start.to_vec();
    let mut objective = log_likelihood(&theta);
    let initial_barrier = identity_barrier(&theta, &theta, dynamic_parameters)
        .map(|value| value.0)
        .unwrap_or(f64::NEG_INFINITY);
    let mut barrier_value = objective - MU * initial_barrier;
    let mut total_function_evaluations = 0;
    let mut total_gradient_evaluations = 0;
    let mut last = None;

    for outer in 0..100 {
        let objective_previous = objective;
        let barrier_previous = barrier_value;
        let theta_previous = theta.clone();
        let inner_objective = |candidate: &[f64]| {
            let Some((barrier, _)) =
                identity_barrier(candidate, &theta_previous, dynamic_parameters)
            else {
                return f64::INFINITY;
            };
            -log_likelihood(candidate) + MU * barrier
        };
        let inner_gradient = |candidate: &[f64]| {
            let Some((_, barrier_gradient)) =
                identity_barrier(candidate, &theta_previous, dynamic_parameters)
            else {
                return vec![f64::NAN; candidate.len()];
            };
            score(candidate)
                .into_iter()
                .zip(barrier_gradient)
                .map(|(value, barrier)| -value + MU * barrier)
                .collect()
        };
        let inner = vmmin(inner_objective, inner_gradient, &theta_previous, 100, 1e-11);
        let candidate_barrier_value = -inner.value;
        let candidate = inner.parameters.clone();
        let candidate_objective = log_likelihood(&candidate);
        let converged = candidate_barrier_value.is_finite()
            && barrier_previous.is_finite()
            && (candidate_barrier_value - barrier_previous).abs()
                < (0.001 + candidate_barrier_value.abs()) * OUTER_EPS;

        if !converged {
            total_function_evaluations += inner.function_evaluations;
            total_gradient_evaluations += inner.gradient_evaluations;
        }
        let stopped_for_direction = candidate_objective < objective_previous;
        theta = candidate;
        objective = candidate_objective;
        barrier_value = candidate_barrier_value;
        last = Some((inner, outer + 1, converged));
        if converged || stopped_for_direction {
            break;
        }
    }

    let (inner, outer_iterations, outer_converged) = last.expect("one barrier iteration");
    VmminResult {
        parameters: theta.clone(),
        value: -objective,
        gradient: score(&theta).into_iter().map(|value| -value).collect(),
        iterations: outer_iterations,
        // `optim` includes the constrained wrapper's initial objective evaluation in this count.
        function_evaluations: total_function_evaluations + 1,
        gradient_evaluations: total_gradient_evaluations,
        converged: inner.converged && outer_converged,
    }
}

/// R's `vmmin` BFGS path from `src/appl/optim.c`, at the controls used by tscount.
fn vmmin<F, G>(
    objective: F,
    gradient: G,
    start: &[f64],
    max_iterations: usize,
    relative_tolerance: f64,
) -> VmminResult
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
{
    const STEP_REDUCTION: f64 = 0.2;
    const ACCEPTANCE_TOLERANCE: f64 = 0.0001;
    const RELATIVE_TEST: f64 = 10.0;

    let n = start.len();
    let mut parameters = start.to_vec();
    let mut minimum = objective(&parameters);
    let mut function_evaluations = 1;
    let mut grad = gradient(&parameters);
    let mut gradient_evaluations = 1;
    let mut iterations = 1;
    let mut last_restart = gradient_evaluations;
    let mut count;
    let mut inverse_hessian = vec![vec![0.0; n]; n];
    let mut direction = vec![0.0; n];
    let mut old_parameters = vec![0.0; n];
    let mut old_gradient = vec![0.0; n];

    loop {
        if last_restart == gradient_evaluations {
            for (i, row) in inverse_hessian.iter_mut().enumerate() {
                row.fill(0.0);
                row[i] = 1.0;
            }
        }
        old_parameters.copy_from_slice(&parameters);
        old_gradient.copy_from_slice(&grad);
        let mut gradient_projection = 0.0;
        for i in 0..n {
            direction[i] = -(0..n).map(|j| inverse_hessian[i][j] * grad[j]).sum::<f64>();
            gradient_projection += direction[i] * grad[i];
        }

        if gradient_projection < 0.0 {
            let mut step = 1.0;
            let mut value = minimum;
            loop {
                count = 0;
                for i in 0..n {
                    parameters[i] = old_parameters[i] + step * direction[i];
                    if RELATIVE_TEST + old_parameters[i] == RELATIVE_TEST + parameters[i] {
                        count += 1;
                    }
                }
                if count == n {
                    break;
                }
                value = objective(&parameters);
                function_evaluations += 1;
                if value.is_finite()
                    && value <= minimum + gradient_projection * step * ACCEPTANCE_TOLERANCE
                {
                    break;
                }
                step *= STEP_REDUCTION;
            }

            let enough =
                (value - minimum).abs() > relative_tolerance * (minimum.abs() + relative_tolerance);
            if !enough {
                count = n;
                minimum = value;
            }
            if count < n {
                minimum = value;
                grad = gradient(&parameters);
                gradient_evaluations += 1;
                iterations += 1;
                let mut d1 = 0.0;
                for i in 0..n {
                    direction[i] *= step;
                    old_gradient[i] = grad[i] - old_gradient[i];
                    d1 += direction[i] * old_gradient[i];
                }
                if d1 > 0.0 {
                    let mut product = vec![0.0; n];
                    let mut d2 = 0.0;
                    for i in 0..n {
                        product[i] = (0..n)
                            .map(|j| inverse_hessian[i][j] * old_gradient[j])
                            .sum();
                        d2 += product[i] * old_gradient[i];
                    }
                    d2 = 1.0 + d2 / d1;
                    for i in 0..n {
                        for j in 0..=i {
                            let update = (d2 * direction[i] * direction[j]
                                - product[i] * direction[j]
                                - direction[i] * product[j])
                                / d1;
                            inverse_hessian[i][j] += update;
                            inverse_hessian[j][i] = inverse_hessian[i][j];
                        }
                    }
                } else {
                    last_restart = gradient_evaluations;
                }
            } else if last_restart < gradient_evaluations {
                count = 0;
                last_restart = gradient_evaluations;
            }
        } else {
            count = if last_restart == gradient_evaluations {
                n
            } else {
                0
            };
            if count == 0 {
                last_restart = gradient_evaluations;
            }
        }

        if iterations >= max_iterations {
            break;
        }
        if gradient_evaluations - last_restart > 2 * n {
            last_restart = gradient_evaluations;
        }
        if count == n && last_restart == gradient_evaluations {
            break;
        }
    }

    VmminResult {
        parameters,
        value: minimum,
        gradient: grad,
        iterations,
        function_evaluations,
        gradient_evaluations,
        converged: iterations < max_iterations,
    }
}

fn validate(y: &[f64], x: &[Vec<f64>], spec: &IngarchSpecification) -> Result<usize, IngarchError> {
    if y.is_empty() {
        return Err(IngarchError::EmptySeries);
    }
    for (row, value) in y.iter().copied().enumerate() {
        if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
            return Err(IngarchError::InvalidCount { row, value });
        }
    }
    if spec
        .past_observation_lags
        .iter()
        .chain(&spec.past_mean_lags)
        .any(|&lag| lag == 0)
    {
        return Err(IngarchError::InvalidLag);
    }
    if x.len() != y.len() {
        return Err(IngarchError::InvalidRegressorShape);
    }
    let r = x.first().map_or(spec.external_regressors.len(), Vec::len);
    if spec.external_regressors.len() != r || x.iter().any(|row| row.len() != r) {
        return Err(IngarchError::InvalidExternalRegressors);
    }
    for (row, values) in x.iter().enumerate() {
        for (column, value) in values.iter().enumerate() {
            if !value.is_finite() {
                return Err(IngarchError::NonFiniteRegressor { row, column });
            }
            if spec.link == IngarchLink::Identity && *value < 0.0 {
                return Err(IngarchError::IdentityRequiresNonNegativeRegressor { row, column });
            }
        }
    }
    Ok(r)
}

fn recurse(
    y: &[f64],
    x: &[Vec<f64>],
    spec: &IngarchSpecification,
    parameters: &[f64],
    with_derivatives: bool,
) -> Result<Recursion, IngarchError> {
    let p = spec.past_observation_lags.len();
    let q = spec.past_mean_lags.len();
    let r = spec.external_regressors.len();
    let k = 1 + p + q + r;
    if parameters.len() != k {
        return Err(IngarchError::InvalidRegressorShape);
    }
    let obs = &parameters[1..1 + p];
    let means = &parameters[1 + p..1 + p + q];
    let regressors = &parameters[1 + p + q..];
    let denominator = 1.0 - obs.iter().sum::<f64>() - means.iter().sum::<f64>();
    if !denominator.is_finite() || denominator.abs() <= 1e-14 {
        return Err(IngarchError::NonStationaryInitialization);
    }
    let stationary = parameters[0] / denominator;
    if !stationary.is_finite() {
        return Err(IngarchError::NonStationaryInitialization);
    }

    let p_max = spec
        .past_observation_lags
        .iter()
        .copied()
        .max()
        .unwrap_or(0);
    let q_max = spec.past_mean_lags.iter().copied().max().unwrap_or(0);
    let mut z = vec![stationary; p_max];
    z.extend(y.iter().map(|value| match spec.link {
        IngarchLink::Identity => *value,
        IngarchLink::Log => value.ln_1p(),
    }));
    let mut nu_all = vec![stationary; q_max + y.len()];
    let mut derivatives = vec![0.0; (q_max + y.len()) * k];
    if with_derivatives {
        for t in 0..q_max {
            derivatives[t * k] = 1.0 / denominator;
            for parameter in 1..1 + p + q {
                derivatives[t * k + parameter] = parameters[0] / denominator.powi(2);
            }
        }
    }

    for t in 0..y.len() {
        let mut value = parameters[0];
        for (coefficient, lag) in obs.iter().zip(&spec.past_observation_lags) {
            value += coefficient * z[p_max + t - lag];
        }
        for (coefficient, lag) in means.iter().zip(&spec.past_mean_lags) {
            value += coefficient * nu_all[q_max + t - lag];
        }
        for column in 0..r {
            value += regressors[column] * x[t][column];
        }
        for (coefficient, lag) in means.iter().zip(&spec.past_mean_lags) {
            if t >= *lag {
                for column in 0..r {
                    if spec.external_regressors[column] {
                        value -= coefficient * regressors[column] * x[t - lag][column];
                    }
                }
            }
        }
        nu_all[q_max + t] = value;

        if with_derivatives {
            let row = (q_max + t) * k;
            derivatives[row] = 1.0;
            for (coefficient, lag) in means.iter().zip(&spec.past_mean_lags) {
                let previous = (q_max + t - lag) * k;
                for parameter in 0..k {
                    derivatives[row + parameter] += coefficient * derivatives[previous + parameter];
                }
            }
            for (index, lag) in spec.past_observation_lags.iter().enumerate() {
                derivatives[row + 1 + index] += z[p_max + t - lag];
            }
            for (index, lag) in spec.past_mean_lags.iter().enumerate() {
                let parameter = 1 + p + index;
                derivatives[row + parameter] += nu_all[q_max + t - lag];
                if t >= *lag {
                    for column in 0..r {
                        if spec.external_regressors[column] {
                            derivatives[row + parameter] -= regressors[column] * x[t - lag][column];
                        }
                    }
                }
            }
            for column in 0..r {
                let parameter = 1 + p + q + column;
                derivatives[row + parameter] += x[t][column];
                if spec.external_regressors[column] {
                    for (coefficient, lag) in means.iter().zip(&spec.past_mean_lags) {
                        if t >= *lag {
                            derivatives[row + parameter] -= coefficient * x[t - lag][column];
                        }
                    }
                }
            }
        }
    }

    let nu = nu_all[q_max..].to_vec();
    let mean: Vec<f64> = nu
        .iter()
        .map(|value| match spec.link {
            IngarchLink::Identity => *value,
            IngarchLink::Log => value.exp(),
        })
        .collect();
    if mean.iter().any(|value| !value.is_finite() || *value <= 0.0) {
        return Err(IngarchError::NonStationaryInitialization);
    }
    Ok(Recursion {
        nu,
        mean,
        derivatives: if with_derivatives {
            derivatives[q_max * k..].to_vec()
        } else {
            Vec::new()
        },
        n_parameters: k,
    })
}

fn quasi_log_likelihood(y: &[f64], recursion: &Recursion) -> f64 {
    y.iter()
        .zip(&recursion.mean)
        .map(|(&observation, &mean)| observation * mean.ln() - mean)
        .sum()
}

fn score(y: &[f64], recursion: &Recursion, link: IngarchLink) -> Vec<f64> {
    let mut result = vec![0.0; recursion.n_parameters];
    for (row, (&observation, &mean)) in y.iter().zip(&recursion.mean).enumerate() {
        for parameter in 0..recursion.n_parameters {
            let partial_mean = recursion.derivatives[row * recursion.n_parameters + parameter]
                * match link {
                    IngarchLink::Identity => 1.0,
                    IngarchLink::Log => mean,
                };
            result[parameter] += (observation / mean - 1.0) * partial_mean;
        }
    }
    result
}

fn intervention_covariate(n: usize, reference_point: usize, delta: f64) -> Vec<f64> {
    (0..n)
        .map(|time| {
            if time < reference_point {
                0.0
            } else {
                delta.powi((time - reference_point) as i32)
            }
        })
        .collect()
}

fn intervention_score_statistic(
    y: &[f64],
    x: &[Vec<f64>],
    null_fit: &IngarchFit,
    reference_point: usize,
    delta: f64,
) -> Result<f64, IngarchError> {
    let covariate = intervention_covariate(y.len(), reference_point, delta);
    let extended_x: Vec<Vec<f64>> = x
        .iter()
        .zip(covariate)
        .map(|(row, value)| {
            let mut extended = row.clone();
            extended.push(value);
            extended
        })
        .collect();
    let mut specification = null_fit.specification.clone();
    specification.external_regressors.push(false);
    let mut parameters = null_fit.parameters.clone();
    parameters.push(0.0);
    let recursion = recurse(y, &extended_x, &specification, &parameters, true)?;
    let parameter_count = parameters.len();
    let nuisance_count = parameter_count - 1;
    let mut total_score = DVector::zeros(parameter_count);
    let mut information = DMatrix::zeros(parameter_count, parameter_count);
    let mut corrected = DMatrix::zeros(parameter_count, parameter_count);

    // `from` in `tsglm.loglik` controls where the recursion is recomputed; it does not truncate the
    // likelihood. Earlier nuisance derivatives come from the cached null recursion, while the new
    // intervention derivative is zero before its candidate date. A full recomputation has exactly
    // those values, so all observations remain in the score and information sums.
    for row in 0..y.len() {
        let mean = recursion.mean[row];
        let nu = recursion.nu[row];
        let mut partial_mean = DVector::zeros(parameter_count);
        for parameter in 0..parameter_count {
            partial_mean[parameter] = recursion.derivatives[row * parameter_count + parameter]
                * match specification.link {
                    IngarchLink::Identity => 1.0,
                    IngarchLink::Log => mean,
                };
        }
        total_score += (y[row] / mean - 1.0) * &partial_mean;
        let outer = &partial_mean * partial_mean.transpose();
        information += (1.0 / mean) * &outer;
        // This deliberately follows `interv_detect.tsglm`, including its use of the linear
        // predictor `nu` rather than the fitted mean in the corrected information weight.
        corrected += (1.0 / nu + null_fit.dispersion) * outer;
    }

    let g11 = information
        .view((0, 0), (nuisance_count, nuisance_count))
        .into_owned();
    let g12 = information
        .view((0, nuisance_count), (nuisance_count, 1))
        .into_owned();
    let g21 = information
        .view((nuisance_count, 0), (1, nuisance_count))
        .into_owned();
    let g1_11 = corrected
        .view((0, 0), (nuisance_count, nuisance_count))
        .into_owned();
    let g1_12 = corrected
        .view((0, nuisance_count), (nuisance_count, 1))
        .into_owned();
    let g1_21 = corrected
        .view((nuisance_count, 0), (1, nuisance_count))
        .into_owned();
    let g1_22 = corrected[(nuisance_count, nuisance_count)];
    let g11_inverse = crate::linalg::inverse_positive_definite_upper(&g11)
        .map_err(|_| IngarchError::SingularDetectionInformation)?;
    let sigma =
        g1_22 - (&g21 * &g11_inverse * &g1_12)[(0, 0)] - (&g1_21 * &g11_inverse * &g12)[(0, 0)]
            + (&g21 * &g11_inverse * &g1_11 * &g11_inverse * &g12)[(0, 0)];
    if !sigma.is_finite() || sigma <= 0.0 {
        return Err(IngarchError::SingularDetectionInformation);
    }
    let intervention_score = total_score[nuisance_count];
    Ok(intervention_score * intervention_score / sigma)
}

/// Scan a known intervention shape over unknown candidate dates, matching
/// `tscount::interv_detect(..., B = NULL, info = "score", est_interv = FALSE)`.
///
/// The result intentionally has no p-value: tscount obtains one only through an optional
/// parametric bootstrap over the maximum statistic. This function is a model diagnostic, not a
/// causal intervention estimate.
pub fn detect_negative_binomial_intervention(
    y: &[f64],
    x: &[Vec<f64>],
    specification: IngarchSpecification,
    candidate_reference_points: &[usize],
    delta: f64,
) -> Result<IngarchInterventionDetection, IngarchError> {
    if candidate_reference_points.is_empty()
        || candidate_reference_points
            .iter()
            .any(|point| *point >= y.len())
        || !(0.0..=1.0).contains(&delta)
        || !delta.is_finite()
    {
        return Err(IngarchError::InvalidDetectionCandidates);
    }
    let mut points = candidate_reference_points.to_vec();
    points.sort_unstable();
    points.dedup();
    let null_fit = fit_negative_binomial_ingarch(y, x, specification)?;
    let mut candidates = Vec::with_capacity(points.len());
    for reference_point in points {
        candidates.push(IngarchInterventionCandidate {
            reference_point,
            score_statistic: intervention_score_statistic(y, x, &null_fit, reference_point, delta)?,
        });
    }
    // R's `which.max` returns the first maximum, and the candidates are sorted above as in tscount.
    let strongest = candidates
        .iter()
        .max_by(|left, right| {
            left.score_statistic
                .total_cmp(&right.score_statistic)
                .then_with(|| right.reference_point.cmp(&left.reference_point))
        })
        .expect("the candidate list is non-empty");
    let strongest_reference_point = strongest.reference_point;
    Ok(IngarchInterventionDetection {
        null_fit,
        candidates,
        strongest_reference_point,
        delta,
    })
}

fn pearson_size(y: &[f64], mean: &[f64], parameter_count: usize) -> Result<f64, IngarchError> {
    let target = y.len() as f64 - parameter_count as f64;
    let function = |size: f64| {
        y.iter()
            .zip(mean)
            .map(|(&observation, &mu)| {
                let residual = observation - mu;
                residual * residual / (mu * (1.0 + mu / size))
            })
            .sum::<f64>()
            - target
    };
    let mut a = 0.0;
    let mut b = 1e100;
    let mut fa = function(a);
    let fb_initial = function(b);
    if !fa.is_finite() || !fb_initial.is_finite() || fa.signum() == fb_initial.signum() {
        return Err(IngarchError::DispersionNotEstimable);
    }
    // R's R_zeroin2, used by uniroot with its default tolerance and iteration limit.
    let tolerance = f64::EPSILON.powf(0.25);
    let mut fb = fb_initial;
    let mut c = a;
    let mut fc = fa;
    for _ in 0..=1000 {
        let previous_step = b - a;
        if fc.abs() < fb.abs() {
            a = b;
            b = c;
            c = a;
            fa = fb;
            fb = fc;
            fc = fa;
        }
        let actual_tolerance = 2.0 * f64::EPSILON * b.abs() + tolerance / 2.0;
        let mut new_step = (c - b) / 2.0;
        if new_step.abs() <= actual_tolerance || fb == 0.0 {
            return Ok(b);
        }
        if previous_step.abs() >= actual_tolerance && fa.abs() > fb.abs() {
            let cb = c - b;
            let (mut p, mut q) = if a == c {
                let t1 = fb / fa;
                (cb * t1, 1.0 - t1)
            } else {
                let q0 = fa / fc;
                let t1 = fb / fc;
                let t2 = fb / fa;
                (
                    t2 * (cb * q0 * (q0 - t1) - (b - a) * (t1 - 1.0)),
                    (q0 - 1.0) * (t1 - 1.0) * (t2 - 1.0),
                )
            };
            if p > 0.0 {
                q = -q;
            } else {
                p = -p;
            }
            if p < 0.75 * cb * q - (actual_tolerance * q).abs() / 2.0
                && p < (previous_step * q / 2.0).abs()
            {
                new_step = p / q;
            }
        }
        if new_step.abs() < actual_tolerance {
            new_step = if new_step > 0.0 {
                actual_tolerance
            } else {
                -actual_tolerance
            };
        }
        a = b;
        fa = fb;
        b += new_step;
        fb = function(b);
        if (fb > 0.0 && fc > 0.0) || (fb < 0.0 && fc < 0.0) {
            c = a;
            fc = fa;
        }
    }
    Ok(b)
}

fn negative_binomial_log_likelihood(y: &[f64], mean: &[f64], size: f64) -> f64 {
    y.iter()
        .zip(mean)
        .map(|(&observation, &mu)| {
            spec_math::cephes64::lgam(observation + size)
                - spec_math::cephes64::lgam(size)
                - spec_math::cephes64::lgam(observation + 1.0)
                + size * (size / (size + mu)).ln()
                + observation * (mu / (size + mu)).ln()
        })
        .sum()
}

/// Fit tscount's identity- or log-link negative-binomial count time-series model.
pub fn fit_negative_binomial_ingarch(
    y: &[f64],
    x: &[Vec<f64>],
    specification: IngarchSpecification,
) -> Result<IngarchFit, IngarchError> {
    let r = validate(y, x, &specification)?;
    let k = 1 + specification.past_observation_lags.len() + specification.past_mean_lags.len() + r;
    let mut start = vec![0.0; k];
    start[0] = y
        .iter()
        .map(|value| match specification.link {
            IngarchLink::Identity => *value,
            IngarchLink::Log => value.ln_1p(),
        })
        .sum::<f64>()
        / y.len() as f64;
    if specification.link == IngarchLink::Identity {
        const EPSILON: f64 = 1e-6;
        const SLACK: f64 = 1e-6;
        for parameter in &mut start[1..] {
            *parameter = parameter.max(EPSILON);
        }
        let dynamic =
            specification.past_observation_lags.len() + specification.past_mean_lags.len();
        let total = start[1..1 + dynamic].iter().sum::<f64>();
        if total > 1.0 - EPSILON - SLACK {
            let shrinkage = (1.0 - SLACK - EPSILON) / total;
            for parameter in &mut start[1..1 + dynamic] {
                *parameter *= shrinkage;
            }
        }
        start[0] = start[0].max(SLACK + EPSILON);
    }
    let objective = |parameters: &[f64]| match recurse(y, x, &specification, parameters, false) {
        Ok(recursion) => -quasi_log_likelihood(y, &recursion),
        Err(_) => f64::INFINITY,
    };
    let gradient = |parameters: &[f64]| match recurse(y, x, &specification, parameters, true) {
        Ok(recursion) => score(y, &recursion, specification.link)
            .into_iter()
            .map(|value| -value)
            .collect(),
        Err(_) => vec![f64::NAN; k],
    };
    let optimized = match specification.link {
        IngarchLink::Identity => {
            let log_likelihood = |parameters: &[f64]| -objective(parameters);
            let likelihood_score = |parameters: &[f64]| {
                gradient(parameters)
                    .into_iter()
                    .map(|value| -value)
                    .collect()
            };
            constrained_identity_vmmin(
                log_likelihood,
                likelihood_score,
                &start,
                specification.past_observation_lags.len() + specification.past_mean_lags.len(),
            )
        }
        IngarchLink::Log => vmmin(objective, gradient, &start, 1000, 1e-13),
    };
    if optimized.parameters.iter().any(|value| !value.is_finite())
        || !optimized.value.is_finite()
        || optimized.gradient.iter().any(|value| !value.is_finite())
        || !optimized.converged
    {
        return Err(IngarchError::OptimizerFailed);
    }
    let recursion = recurse(y, x, &specification, &optimized.parameters, true)?;
    let final_score = score(y, &recursion, specification.link);
    let size = pearson_size(y, &recursion.mean, k)?;
    let log_likelihood = negative_binomial_log_likelihood(y, &recursion.mean, size);
    let residuals = y
        .iter()
        .zip(&recursion.mean)
        .map(|(&observation, &mean)| observation - mean)
        .collect();
    Ok(IngarchFit {
        specification,
        parameters: optimized.parameters,
        start_parameters: start,
        fitted_means: recursion.mean,
        linear_predictors: recursion.nu,
        residuals,
        log_likelihood,
        size,
        dispersion: 1.0 / size,
        score: final_score,
        iterations: optimized.iterations,
        function_evaluations: optimized.function_evaluations,
        gradient_evaluations: optimized.gradient_evaluations,
    })
}

impl IngarchFit {
    /// Conditional-mean forecast. Unknown future observations are replaced recursively by their
    /// conditional means, matching `predict.tsglm(..., level=0)`.
    pub fn forecast_mean(
        &self,
        y: &[f64],
        x: &[Vec<f64>],
        future_x: &[Vec<f64>],
    ) -> Result<Vec<f64>, IngarchError> {
        validate(y, x, &self.specification)?;
        let r = self.specification.external_regressors.len();
        if future_x.is_empty() || future_x.iter().any(|row| row.len() != r) {
            return Err(IngarchError::InvalidRegressorShape);
        }
        for (row, values) in future_x.iter().enumerate() {
            for (column, value) in values.iter().enumerate() {
                if !value.is_finite() {
                    return Err(IngarchError::NonFiniteRegressor {
                        row: y.len() + row,
                        column,
                    });
                }
                if self.specification.link == IngarchLink::Identity && *value < 0.0 {
                    return Err(IngarchError::IdentityRequiresNonNegativeRegressor {
                        row: y.len() + row,
                        column,
                    });
                }
            }
        }
        let fitted = recurse(y, x, &self.specification, &self.parameters, false)?;
        let p = self.specification.past_observation_lags.len();
        let q = self.specification.past_mean_lags.len();
        let obs_parameters = &self.parameters[1..1 + p];
        let mean_parameters = &self.parameters[1 + p..1 + p + q];
        let reg_parameters = &self.parameters[1 + p + q..];
        let mut extended_y = y.to_vec();
        let mut extended_nu = fitted.nu;
        let mut extended_x = x.to_vec();
        extended_x.extend_from_slice(future_x);
        let n = y.len();
        let mut result = Vec::with_capacity(future_x.len());
        for step in 0..future_x.len() {
            let t = n + step;
            let mut nu = self.parameters[0];
            for (coefficient, lag) in obs_parameters
                .iter()
                .zip(&self.specification.past_observation_lags)
            {
                nu += coefficient
                    * match self.specification.link {
                        IngarchLink::Identity => extended_y[t - lag],
                        IngarchLink::Log => extended_y[t - lag].ln_1p(),
                    };
            }
            for (coefficient, lag) in mean_parameters
                .iter()
                .zip(&self.specification.past_mean_lags)
            {
                nu += coefficient * extended_nu[t - lag];
            }
            for column in 0..r {
                nu += reg_parameters[column] * extended_x[t][column];
            }
            for (coefficient, lag) in mean_parameters
                .iter()
                .zip(&self.specification.past_mean_lags)
            {
                for column in 0..r {
                    if self.specification.external_regressors[column] {
                        nu -= coefficient * reg_parameters[column] * extended_x[t - lag][column];
                    }
                }
            }
            let mean = match self.specification.link {
                IngarchLink::Identity => nu,
                IngarchLink::Log => nu.exp(),
            };
            if !mean.is_finite() || mean <= 0.0 {
                return Err(IngarchError::NonStationaryInitialization);
            }
            extended_nu.push(nu);
            extended_y.push(mean);
            result.push(mean);
        }
        Ok(result)
    }
}

/// Build one future-regressor scenario without modifying the historical design.
pub fn intervention_regressors(
    baseline: &[Vec<f64>],
    regressor: usize,
    intervention_value: f64,
    schedule: InterventionSchedule,
) -> Result<Vec<Vec<f64>>, IngarchError> {
    if baseline.is_empty()
        || !intervention_value.is_finite()
        || baseline.iter().any(|row| regressor >= row.len())
    {
        return Err(IngarchError::InvalidSchedule);
    }
    let delta = match schedule {
        InterventionSchedule::Decaying { delta } if !(0.0..=1.0).contains(&delta) => {
            return Err(IngarchError::InvalidSchedule)
        }
        InterventionSchedule::Decaying { delta } => delta,
        _ => 0.0,
    };
    let baseline_value = baseline[0][regressor];
    let change = intervention_value - baseline_value;
    let mut result = baseline.to_vec();
    for (step, row) in result.iter_mut().enumerate() {
        row[regressor] = match schedule {
            InterventionSchedule::Point => {
                if step == 0 {
                    intervention_value
                } else {
                    baseline[step][regressor]
                }
            }
            InterventionSchedule::Persistent => intervention_value,
            InterventionSchedule::Decaying { .. } => {
                baseline[step][regressor] + change * delta.powi(step as i32)
            }
        };
    }
    Ok(result)
}
