//! DoWhy v0.11's instrumental-variable identifier and estimator.
//!
//! `get_instruments` (dowhy/graph.py) keeps the parents of the treatment that, once the
//! treatment's incoming edges are cut, are neither ancestors of the outcome nor descendants of
//! such ancestors. `InstrumentalVariableEstimator` then takes one of three arithmetic routes:
//!
//! 1. one binary instrument: the Wald ratio of outcome and treatment group-mean differences;
//! 2. one continuous instrument: the ratio of sample covariances with the instrument;
//! 3. otherwise statsmodels' `IV2SLS` two-stage least squares, with the per-treatment
//!    coefficients summed.
//!
//! No intercept and no covariates enter any route, and the treatment and outcome are rounded to
//! single precision before two-stage least squares, exactly as the reference does. Confidence
//! intervals reproduce DoWhy's generic bootstrap. Identification is graphical only: it does not
//! test instrument strength, and the estimator does not test it either.

use crate::backdoor::Dag;
use crate::dowhy_bootstrap::{
    basic_interval, resample_rows, standard_error, BootstrapError, DowhyBootstrap,
};
use crate::nprandom::Mt19937;
use crate::numpy_reduce::{numpy_mean, numpy_sum};
use nalgebra::{DMatrix, DVector};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IvIdentificationError {
    EndpointOutOfRange,
    NoTreatment,
    NoOutcome,
    SharedEndpoint,
}

impl Display for IvIdentificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndpointOutOfRange => {
                write!(f, "instrument search endpoints must be graph nodes")
            }
            Self::NoTreatment => write!(f, "instrument search needs at least one treatment"),
            Self::NoOutcome => write!(f, "instrument search needs at least one outcome"),
            Self::SharedEndpoint => {
                write!(
                    f,
                    "instrument search treatments and outcomes must be distinct"
                )
            }
        }
    }
}

impl Error for IvIdentificationError {}

/// DoWhy v0.11's `get_instruments`, returned in ascending node order.
///
/// Parents are read from the original graph; ancestors and descendants from the graph with every
/// edge into a treatment removed. The exclusion step drops parents that are ancestors of an
/// outcome, and the as-if-random step drops parents that descend from such an ancestor. The
/// reference does not restrict the result to observed nodes; callers that need that restriction
/// apply it afterwards.
pub fn identify_instrument_set(
    dag: &Dag,
    treatments: &[usize],
    outcomes: &[usize],
) -> Result<Vec<usize>, IvIdentificationError> {
    if treatments.is_empty() {
        return Err(IvIdentificationError::NoTreatment);
    }
    if outcomes.is_empty() {
        return Err(IvIdentificationError::NoOutcome);
    }
    if treatments.iter().chain(outcomes).any(|&node| node >= dag.n) {
        return Err(IvIdentificationError::EndpointOutOfRange);
    }
    let treatment_set: BTreeSet<usize> = treatments.iter().copied().collect();
    if outcomes
        .iter()
        .any(|outcome| treatment_set.contains(outcome))
    {
        return Err(IvIdentificationError::SharedEndpoint);
    }

    let parents: BTreeSet<usize> = treatments
        .iter()
        .flat_map(|&treatment| dag.parents[treatment].iter().copied())
        .collect();
    let mut edges = Vec::new();
    for (parent, children) in dag.children.iter().enumerate() {
        for &child in children {
            if !treatment_set.contains(&child) {
                edges.push((parent, child));
            }
        }
    }
    let intervened = Dag::new(dag.n, &edges);
    let mut outcome_ancestors = BTreeSet::new();
    for &outcome in outcomes {
        let mut ancestors = intervened.ancestors_of(&[outcome]);
        ancestors.remove(&outcome);
        outcome_ancestors.extend(ancestors);
    }
    let candidates: BTreeSet<usize> = parents.difference(&outcome_ancestors).copied().collect();
    let ancestor_descendants: BTreeSet<usize> = outcome_ancestors
        .iter()
        .flat_map(|&ancestor| intervened.descendants(ancestor))
        .collect();
    Ok(candidates
        .difference(&ancestor_descendants)
        .copied()
        .collect())
}

/// Numeric inputs for an instrumental-variable estimate.
///
/// Matrices are `n` observations by `k` treatments and `n` by `m` instruments. Categorical
/// variables must be encoded before crossing this numerical boundary.
#[derive(Clone, Copy, Debug)]
pub struct IvInput<'a> {
    pub treatments: &'a DMatrix<f64>,
    pub instruments: &'a DMatrix<f64>,
    pub outcome: &'a [f64],
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IvOptions {
    pub bootstrap: Option<DowhyBootstrap>,
}

/// The arithmetic route DoWhy chose for the full sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IvEstimator {
    WaldRatio,
    CovarianceRatio,
    TwoStageLeastSquares,
}

impl Display for IvEstimator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WaldRatio => write!(f, "Wald ratio"),
            Self::CovarianceRatio => write!(f, "covariance ratio"),
            Self::TwoStageLeastSquares => write!(f, "two-stage least squares"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IvResult {
    pub estimator: IvEstimator,
    /// The effect of moving every treatment from 0 to 1; DoWhy ignores the requested
    /// intervention values for this estimator.
    pub estimate: f64,
    /// The per-treatment coefficients; a single entry equal to `estimate` on the two
    /// single-instrument routes.
    pub params: Vec<f64>,
    pub confidence_interval: Option<[f64; 2]>,
    pub standard_error: Option<f64>,
    pub bootstrap_estimates: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IvError {
    EmptyData,
    NoTreatment,
    NoInstruments,
    FewerInstrumentsThanTreatments {
        instruments: usize,
        treatments: usize,
    },
    LengthMismatch {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
    NonFiniteValue {
        field: &'static str,
        row: usize,
    },
    Bootstrap(BootstrapError),
    InsufficientBootstrapSample {
        sample_size: usize,
        required: usize,
    },
    /// The reference arithmetic produced NaN or infinity: an empty instrument group, a binary
    /// instrument not coded 0/1, or a treatment the instruments do not move.
    NonFiniteEstimate(IvEstimator),
    SingularSystem,
}

impl Display for IvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyData => {
                write!(f, "instrumental-variable estimation requires at least one row")
            }
            Self::NoTreatment => {
                write!(f, "instrumental-variable estimation requires a treatment column")
            }
            Self::NoInstruments => write!(
                f,
                "no valid instruments found; the instrumental-variable method is not applicable"
            ),
            Self::FewerInstrumentsThanTreatments {
                instruments,
                treatments,
            } => write!(
                f,
                "{instruments} instruments for {treatments} treatments; two-stage least squares requires at least as many instruments as treatments"
            ),
            Self::LengthMismatch {
                field,
                expected,
                actual,
            } => write!(f, "{field} has {actual} rows; expected {expected} rows"),
            Self::NonFiniteValue { field, row } => {
                write!(f, "{field} contains a non-finite value at row {row}")
            }
            Self::Bootstrap(error) => write!(f, "{error}"),
            Self::InsufficientBootstrapSample {
                sample_size,
                required,
            } => write!(
                f,
                "bootstrap sample has {sample_size} rows; the estimator requires {required}"
            ),
            Self::NonFiniteEstimate(estimator) => {
                write!(f, "the {estimator} estimate is not finite")
            }
            Self::SingularSystem => {
                write!(f, "two-stage least squares normal equations are singular")
            }
        }
    }
}

impl Error for IvError {}

fn check_matrix(name: &'static str, values: &DMatrix<f64>, n: usize) -> Result<(), IvError> {
    if values.nrows() != n {
        return Err(IvError::LengthMismatch {
            field: name,
            expected: n,
            actual: values.nrows(),
        });
    }
    for row in 0..values.nrows() {
        for column in 0..values.ncols() {
            if !values[(row, column)].is_finite() {
                return Err(IvError::NonFiniteValue { field: name, row });
            }
        }
    }
    Ok(())
}

fn validate(input: IvInput<'_>, options: IvOptions) -> Result<(), IvError> {
    let n = input.outcome.len();
    if n == 0 {
        return Err(IvError::EmptyData);
    }
    let treatments = input.treatments.ncols();
    let instruments = input.instruments.ncols();
    if treatments == 0 {
        return Err(IvError::NoTreatment);
    }
    // The reference refuses in this order inside `fit`, before it reads any values.
    if instruments == 0 {
        return Err(IvError::NoInstruments);
    }
    if instruments < treatments {
        return Err(IvError::FewerInstrumentsThanTreatments {
            instruments,
            treatments,
        });
    }
    check_matrix("treatments", input.treatments, n)?;
    check_matrix("instruments", input.instruments, n)?;
    if let Some(row) = input.outcome.iter().position(|value| !value.is_finite()) {
        return Err(IvError::NonFiniteValue {
            field: "outcome",
            row,
        });
    }
    if let Some(bootstrap) = options.bootstrap {
        bootstrap.validate().map_err(IvError::Bootstrap)?;
        let sample_size = bootstrap.sample_size(n);
        let required = instruments.max(2);
        if sample_size < required {
            return Err(IvError::InsufficientBootstrapSample {
                sample_size,
                required,
            });
        }
    }
    Ok(())
}

fn gather_column(values: &DMatrix<f64>, column: usize, rows: Option<&[usize]>) -> Vec<f64> {
    match rows {
        Some(rows) => rows.iter().map(|&row| values[(row, column)]).collect(),
        None => (0..values.nrows())
            .map(|row| values[(row, column)])
            .collect(),
    }
}

fn gather_vector(values: &[f64], rows: Option<&[usize]>) -> Vec<f64> {
    match rows {
        Some(rows) => rows.iter().map(|&row| values[row]).collect(),
        None => values.to_vec(),
    }
}

fn distinct_count(values: &[f64]) -> usize {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted.dedup();
    sorted.len()
}

fn group_mean(values: &[f64], instrument: &[f64], level: f64) -> f64 {
    let selected: Vec<f64> = values
        .iter()
        .zip(instrument)
        .filter(|(_, z)| **z == level)
        .map(|(value, _)| *value)
        .collect();
    numpy_mean(&selected)
}

/// The off-diagonal entry of `np.cov(a, b)`: sample covariance with `N - 1` in the denominator.
fn sample_covariance(a: &[f64], b: &[f64]) -> f64 {
    let a_mean = numpy_mean(a);
    let b_mean = numpy_mean(b);
    let products: Vec<f64> = a
        .iter()
        .zip(b)
        .map(|(a, b)| (a - a_mean) * (b - b_mean))
        .collect();
    numpy_sum(&products) * (1.0 / (a.len() - 1) as f64)
}

/// `np.dot(left.T, right)` with NumPy's summation order down each column pair.
fn cross_product(left: &DMatrix<f64>, right: &DMatrix<f64>) -> DMatrix<f64> {
    DMatrix::from_fn(left.ncols(), right.ncols(), |i, j| {
        let products: Vec<f64> = (0..left.nrows())
            .map(|row| left[(row, i)] * right[(row, j)])
            .collect();
        numpy_sum(&products)
    })
}

fn solve(system: DMatrix<f64>, right: DMatrix<f64>) -> Result<DMatrix<f64>, IvError> {
    system.lu().solve(&right).ok_or(IvError::SingularSystem)
}

/// `statsmodels.sandbox.regression.gmm.IV2SLS(endog, exog, instrument).fit().params`, on
/// treatment and outcome values already rounded to single precision.
fn two_stage_least_squares(
    treatments: &DMatrix<f64>,
    instruments: &DMatrix<f64>,
    outcome: &DVector<f64>,
) -> Result<Vec<f64>, IvError> {
    let first_stage = solve(
        cross_product(instruments, instruments),
        cross_product(instruments, treatments),
    )?;
    let fitted = DMatrix::from_fn(instruments.nrows(), treatments.ncols(), |row, column| {
        (0..instruments.ncols()).fold(0.0, |total, i| {
            total + instruments[(row, i)] * first_stage[(i, column)]
        })
    });
    let outcome = DMatrix::from_column_slice(outcome.len(), 1, outcome.as_slice());
    let params = solve(
        cross_product(&fitted, &fitted),
        cross_product(&fitted, &outcome),
    )?;
    Ok(params.iter().copied().collect())
}

fn single_precision(value: f64) -> f64 {
    (value as f32) as f64
}

fn estimate_once(
    input: IvInput<'_>,
    rows: Option<&[usize]>,
) -> Result<(IvEstimator, f64, Vec<f64>), IvError> {
    let outcome = gather_vector(input.outcome, rows);
    let single_route = input.instruments.ncols() == 1 && input.treatments.ncols() == 1;
    let (estimator, estimate, params) = if single_route {
        let instrument = gather_column(input.instruments, 0, rows);
        let treatment = gather_column(input.treatments, 0, rows);
        if distinct_count(&instrument) <= 2 {
            let numerator =
                group_mean(&outcome, &instrument, 1.0) - group_mean(&outcome, &instrument, 0.0);
            let denominator =
                group_mean(&treatment, &instrument, 1.0) - group_mean(&treatment, &instrument, 0.0);
            let estimate = numerator / denominator;
            (IvEstimator::WaldRatio, estimate, vec![estimate])
        } else {
            let estimate = sample_covariance(&outcome, &instrument)
                / sample_covariance(&treatment, &instrument);
            (IvEstimator::CovarianceRatio, estimate, vec![estimate])
        }
    } else {
        let n = outcome.len();
        let treatments = DMatrix::from_fn(n, input.treatments.ncols(), |row, column| {
            let source = rows.map_or(row, |rows| rows[row]);
            single_precision(input.treatments[(source, column)])
        });
        let instruments = DMatrix::from_fn(n, input.instruments.ncols(), |row, column| {
            let source = rows.map_or(row, |rows| rows[row]);
            input.instruments[(source, column)]
        });
        let outcome =
            DVector::from_iterator(n, outcome.iter().map(|&value| single_precision(value)));
        let params = two_stage_least_squares(&treatments, &instruments, &outcome)?;
        let estimate = params.iter().fold(0.0, |total, param| total + param);
        (IvEstimator::TwoStageLeastSquares, estimate, params)
    };
    if !estimate.is_finite() {
        return Err(IvError::NonFiniteEstimate(estimator));
    }
    Ok((estimator, estimate, params))
}

/// Estimate the instrumental-variable effect, reporting bootstrap progress as
/// `(completed, total)`.
pub fn instrumental_variable_with_progress<F>(
    input: IvInput<'_>,
    options: IvOptions,
    mut progress: F,
) -> Result<IvResult, IvError>
where
    F: FnMut(usize, usize),
{
    validate(input, options)?;
    let (estimator, estimate, params) = estimate_once(input, None)?;

    let Some(bootstrap) = options.bootstrap else {
        return Ok(IvResult {
            estimator,
            estimate,
            params,
            confidence_interval: None,
            standard_error: None,
            bootstrap_estimates: Vec::new(),
        });
    };

    let n = input.outcome.len();
    let sample_size = bootstrap.sample_size(n);
    let mut rng = Mt19937::seeded(bootstrap.seed);
    let mut estimates = Vec::with_capacity(bootstrap.simulations);
    progress(0, bootstrap.simulations);
    for completed in 0..bootstrap.simulations {
        let rows = resample_rows(&mut rng, n, sample_size);
        estimates.push(estimate_once(input, Some(&rows))?.1);
        progress(completed + 1, bootstrap.simulations);
    }
    let confidence_interval = basic_interval(estimate, &estimates, bootstrap.confidence_level);
    let standard_error = standard_error(&estimates);

    Ok(IvResult {
        estimator,
        estimate,
        params,
        confidence_interval: Some(confidence_interval),
        standard_error: Some(standard_error),
        bootstrap_estimates: estimates,
    })
}

pub fn instrumental_variable(input: IvInput<'_>, options: IvOptions) -> Result<IvResult, IvError> {
    instrumental_variable_with_progress(input, options, |_, _| {})
}
