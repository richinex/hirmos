//! DoWhy v0.11's linear two-stage front-door estimator.
//!
//! The reference implementation fits two OLS models and multiplies their intervention
//! contrasts:
//!
//! 1. mediator on treatment and the first-stage adjustment variables;
//! 2. outcome on mediator and the second-stage adjustment variables.
//!
//! Confidence intervals reproduce DoWhy's generic basic bootstrap, including its legacy
//! NumPy `RandomState` sampling stream and integer order-statistic convention. This estimator
//! is valid only after the graph has identified the requested effect by the front-door
//! criterion; it does not perform identification itself.

use crate::backdoor::{combinations, Dag};
use crate::dowhy_bootstrap::{basic_interval, resample_rows, BootstrapError, DowhyBootstrap};
use crate::nprandom::Mt19937;
use nalgebra::{DMatrix, DVector};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Numeric inputs for a singleton-treatment, singleton-mediator front-door estimate.
///
/// Adjustment matrices are row-major conceptually (`n` observations by `p` variables), as
/// represented by `nalgebra::DMatrix`. Categorical variables must be encoded before crossing
/// this numerical boundary, matching the dummy-variable expansion performed by DoWhy.
#[derive(Clone, Copy, Debug)]
pub struct FrontdoorInput<'a> {
    pub treatment: &'a [f64],
    pub mediator: &'a [f64],
    pub outcome: &'a [f64],
    pub first_stage_adjustment: Option<&'a DMatrix<f64>>,
    pub second_stage_adjustment: Option<&'a DMatrix<f64>>,
}

pub type FrontdoorBootstrap = DowhyBootstrap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrontdoorOptions {
    pub control_value: f64,
    pub treatment_value: f64,
    pub bootstrap: Option<FrontdoorBootstrap>,
}

impl Default for FrontdoorOptions {
    fn default() -> Self {
        Self {
            control_value: 0.0,
            treatment_value: 1.0,
            bootstrap: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrontdoorResult {
    pub first_stage_params: Vec<f64>,
    pub second_stage_params: Vec<f64>,
    pub first_stage_effect: f64,
    pub second_stage_effect: f64,
    pub ate: f64,
    pub confidence_interval: Option<[f64; 2]>,
    pub bootstrap_estimates: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrontdoorError {
    EmptyData,
    LengthMismatch {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
    NonFiniteValue {
        field: &'static str,
        row: usize,
    },
    InvalidIntervention,
    InvalidBootstrapSimulations,
    InvalidBootstrapSampleFraction,
    InvalidConfidenceLevel,
    InsufficientBootstrapSample {
        sample_size: usize,
        parameters: usize,
    },
    LeastSquaresFailure,
}

impl Display for FrontdoorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyData => write!(f, "front-door estimation requires at least one row"),
            Self::LengthMismatch {
                field,
                expected,
                actual,
            } => write!(f, "{field} has {actual} rows; expected {expected} rows"),
            Self::NonFiniteValue { field, row } => {
                write!(f, "{field} contains a non-finite value at row {row}")
            }
            Self::InvalidIntervention => {
                write!(
                    f,
                    "control and treatment values must be finite and distinct"
                )
            }
            Self::InvalidBootstrapSimulations => {
                write!(f, "bootstrap simulations must be greater than zero")
            }
            Self::InvalidBootstrapSampleFraction => write!(
                f,
                "bootstrap sample-size fraction must be finite and greater than zero"
            ),
            Self::InvalidConfidenceLevel => {
                write!(f, "confidence level must be strictly between zero and one")
            }
            Self::InsufficientBootstrapSample {
                sample_size,
                parameters,
            } => write!(
                f,
                "bootstrap sample has {sample_size} rows for a {parameters}-parameter stage"
            ),
            Self::LeastSquaresFailure => write!(f, "least-squares decomposition failed"),
        }
    }
}

impl Error for FrontdoorError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrontdoorIdentificationError {
    EndpointOutOfRange,
    SameEndpoint,
    UnobservedOutOfRange,
    UnobservedEndpoint,
}

impl Display for FrontdoorIdentificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndpointOutOfRange => write!(f, "front-door endpoints must be graph nodes"),
            Self::SameEndpoint => write!(f, "front-door treatment and outcome must be distinct"),
            Self::UnobservedOutOfRange => {
                write!(f, "every unobserved node must belong to the graph")
            }
            Self::UnobservedEndpoint => {
                write!(f, "front-door treatment and outcome must be observed")
            }
        }
    }
}

impl Error for FrontdoorIdentificationError {}

/// DoWhy v0.11's three-condition front-door-set search.
///
/// The returned set intercepts every directed treatment-to-outcome path, has no open back-door
/// path from treatment to the set without adjustment, and has every mediator-to-outcome back-door
/// path blocked by the treatment. Only observed descendants are eligible. DoWhy continues through
/// larger candidate-set sizes after finding a valid smaller set; this routine preserves that search
/// behavior.
pub fn identify_frontdoor_set(
    dag: &Dag,
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
) -> Result<Option<Vec<usize>>, FrontdoorIdentificationError> {
    if treatment >= dag.n || outcome >= dag.n {
        return Err(FrontdoorIdentificationError::EndpointOutOfRange);
    }
    if treatment == outcome {
        return Err(FrontdoorIdentificationError::SameEndpoint);
    }
    if unobserved.iter().any(|&node| node >= dag.n) {
        return Err(FrontdoorIdentificationError::UnobservedOutOfRange);
    }
    if unobserved.contains(&treatment) || unobserved.contains(&outcome) {
        return Err(FrontdoorIdentificationError::UnobservedEndpoint);
    }

    let unobserved: BTreeSet<usize> = unobserved.iter().copied().collect();
    let outcome_descendants = dag.descendants(outcome);
    let eligible = dag
        .descendants(treatment)
        .into_iter()
        .filter(|node| *node != outcome)
        .filter(|node| !outcome_descendants.contains(node))
        .filter(|node| !unobserved.contains(node))
        .collect::<Vec<_>>();
    let treatment_intervened = dag.without_incoming(treatment);
    let treatment_backdoor = dag.backdoor_graph(treatment);
    let empty = BTreeSet::new();
    let mut selected = None;

    for size in 1..=eligible.len() {
        for candidate in combinations(&eligible, size) {
            let candidate_set: BTreeSet<usize> = candidate.iter().copied().collect();
            let intercepts_paths =
                treatment_intervened.d_separated(treatment, outcome, &candidate_set);
            if !intercepts_paths {
                continue;
            }
            let treatment_mediator_unconfounded = candidate
                .iter()
                .all(|&mediator| treatment_backdoor.d_separated(treatment, mediator, &empty));
            if !treatment_mediator_unconfounded {
                continue;
            }
            let mediator_backdoor = dag.without_outgoing_set(&candidate_set);
            let treatment_adjustment = BTreeSet::from([treatment]);
            let mediator_outcome_blocked = candidate.iter().all(|&mediator| {
                mediator_backdoor.d_separated(mediator, outcome, &treatment_adjustment)
            });
            if mediator_outcome_blocked {
                selected = Some(candidate);
                break;
            }
        }
    }
    Ok(selected)
}

fn check_vector(name: &'static str, values: &[f64], n: usize) -> Result<(), FrontdoorError> {
    if values.len() != n {
        return Err(FrontdoorError::LengthMismatch {
            field: name,
            expected: n,
            actual: values.len(),
        });
    }
    if let Some(row) = values.iter().position(|value| !value.is_finite()) {
        return Err(FrontdoorError::NonFiniteValue { field: name, row });
    }
    Ok(())
}

fn check_adjustment(
    name: &'static str,
    values: Option<&DMatrix<f64>>,
    n: usize,
) -> Result<(), FrontdoorError> {
    let Some(values) = values else {
        return Ok(());
    };
    if values.nrows() != n {
        return Err(FrontdoorError::LengthMismatch {
            field: name,
            expected: n,
            actual: values.nrows(),
        });
    }
    for row in 0..values.nrows() {
        for column in 0..values.ncols() {
            if !values[(row, column)].is_finite() {
                return Err(FrontdoorError::NonFiniteValue { field: name, row });
            }
        }
    }
    Ok(())
}

fn validate(input: FrontdoorInput<'_>, options: FrontdoorOptions) -> Result<(), FrontdoorError> {
    let n = input.treatment.len();
    if n == 0 {
        return Err(FrontdoorError::EmptyData);
    }
    check_vector("treatment", input.treatment, n)?;
    check_vector("mediator", input.mediator, n)?;
    check_vector("outcome", input.outcome, n)?;
    check_adjustment(
        "first-stage adjustment matrix",
        input.first_stage_adjustment,
        n,
    )?;
    check_adjustment(
        "second-stage adjustment matrix",
        input.second_stage_adjustment,
        n,
    )?;
    if !options.control_value.is_finite()
        || !options.treatment_value.is_finite()
        || options.control_value == options.treatment_value
    {
        return Err(FrontdoorError::InvalidIntervention);
    }
    if let Some(bootstrap) = options.bootstrap {
        bootstrap.validate().map_err(|error| match error {
            BootstrapError::InvalidSimulations => FrontdoorError::InvalidBootstrapSimulations,
            BootstrapError::InvalidSampleFraction => FrontdoorError::InvalidBootstrapSampleFraction,
            BootstrapError::InvalidConfidenceLevel => FrontdoorError::InvalidConfidenceLevel,
        })?;
        let sample_size = bootstrap.sample_size(n);
        let first_parameters = 2 + input.first_stage_adjustment.map_or(0, DMatrix::ncols);
        let second_parameters = 2 + input.second_stage_adjustment.map_or(0, DMatrix::ncols);
        let parameters = first_parameters.max(second_parameters);
        if sample_size < parameters {
            return Err(FrontdoorError::InsufficientBootstrapSample {
                sample_size,
                parameters,
            });
        }
    }
    Ok(())
}

fn design(
    focal: &[f64],
    adjustment: Option<&DMatrix<f64>>,
    rows: Option<&[usize]>,
) -> DMatrix<f64> {
    let n = rows.map_or(focal.len(), <[usize]>::len);
    let adjustment_columns = adjustment.map_or(0, DMatrix::ncols);
    DMatrix::from_fn(n, 2 + adjustment_columns, |output_row, column| {
        let source_row = rows.map_or(output_row, |rows| rows[output_row]);
        match column {
            0 => 1.0,
            1 => focal[source_row],
            _ => adjustment.expect("adjustment column")[(source_row, column - 2)],
        }
    })
}

fn response(values: &[f64], rows: Option<&[usize]>) -> DVector<f64> {
    match rows {
        Some(rows) => DVector::from_iterator(rows.len(), rows.iter().map(|&row| values[row])),
        None => DVector::from_column_slice(values),
    }
}

/// Statsmodels OLS uses a Moore-Penrose pseudo-inverse with a `1e-15` singular-value cutoff.
fn fit_ols(x: DMatrix<f64>, y: DVector<f64>) -> Result<Vec<f64>, FrontdoorError> {
    crate::ols::Ols::try_fit(&x, &y)
        .map(|fit| fit.params.iter().copied().collect())
        .map_err(|_| FrontdoorError::LeastSquaresFailure)
}

fn fit_once(
    input: FrontdoorInput<'_>,
    rows: Option<&[usize]>,
    intervention_difference: f64,
) -> Result<(Vec<f64>, Vec<f64>, f64, f64, f64), FrontdoorError> {
    let first_stage_params = fit_ols(
        design(input.treatment, input.first_stage_adjustment, rows),
        response(input.mediator, rows),
    )?;
    let second_stage_params = fit_ols(
        design(input.mediator, input.second_stage_adjustment, rows),
        response(input.outcome, rows),
    )?;
    let first_stage_effect = first_stage_params[1] * intervention_difference;
    let second_stage_effect = second_stage_params[1] * intervention_difference;
    let ate = first_stage_effect * second_stage_effect;
    Ok((
        first_stage_params,
        second_stage_params,
        first_stage_effect,
        second_stage_effect,
        ate,
    ))
}

/// Estimate the linear front-door effect, reporting bootstrap progress as `(completed, total)`.
pub fn frontdoor_two_stage_with_progress<F>(
    input: FrontdoorInput<'_>,
    options: FrontdoorOptions,
    mut progress: F,
) -> Result<FrontdoorResult, FrontdoorError>
where
    F: FnMut(usize, usize),
{
    validate(input, options)?;
    let difference = options.treatment_value - options.control_value;
    let (first_params, second_params, first_effect, second_effect, ate) =
        fit_once(input, None, difference)?;

    let Some(bootstrap) = options.bootstrap else {
        return Ok(FrontdoorResult {
            first_stage_params: first_params,
            second_stage_params: second_params,
            first_stage_effect: first_effect,
            second_stage_effect: second_effect,
            ate,
            confidence_interval: None,
            bootstrap_estimates: Vec::new(),
        });
    };

    let n = input.treatment.len();
    let sample_size = bootstrap.sample_size(n);
    let mut rng = Mt19937::seeded(bootstrap.seed);
    let mut estimates = Vec::with_capacity(bootstrap.simulations);
    progress(0, bootstrap.simulations);
    for completed in 0..bootstrap.simulations {
        let rows = resample_rows(&mut rng, n, sample_size);
        estimates.push(fit_once(input, Some(&rows), difference)?.4);
        progress(completed + 1, bootstrap.simulations);
    }
    let confidence_interval = basic_interval(ate, &estimates, bootstrap.confidence_level);

    Ok(FrontdoorResult {
        first_stage_params: first_params,
        second_stage_params: second_params,
        first_stage_effect: first_effect,
        second_stage_effect: second_effect,
        ate,
        confidence_interval: Some(confidence_interval),
        bootstrap_estimates: estimates,
    })
}

pub fn frontdoor_two_stage(
    input: FrontdoorInput<'_>,
    options: FrontdoorOptions,
) -> Result<FrontdoorResult, FrontdoorError> {
    frontdoor_two_stage_with_progress(input, options, |_, _| {})
}
