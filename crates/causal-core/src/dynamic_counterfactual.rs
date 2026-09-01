//! Time-series abduction, intervention, and replay for additive linear structural equations.
//!
//! The window replay follows Tigramite's internal `CounterfactualTimeseries`: lagged history before
//! the replay window remains factual, contemporaneous equations are evaluated in causal order, and
//! the same exogenous innovations are reused in the factual and intervened worlds.  Tigramite's
//! helper assumes that the SEM is known.  This module also fits that SEM from an observed stationary
//! time series, which is the additional layer needed by Hirmos.

use std::collections::{BTreeSet, HashSet};
use std::fmt;

use nalgebra::{DMatrix, DVector};

use crate::sklearn_linear::{
    fit_sklearn_linear_regression, SklearnLinearError, SKLEARN_LINEAR_TOLERANCE,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DynamicParentSpec {
    pub variable: usize,
    /// Zero is contemporaneous; a positive value refers to the past.
    pub lag: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DynamicParent {
    pub variable: usize,
    pub lag: usize,
    pub coefficient: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicEquation {
    pub intercept: f64,
    pub parents: Vec<DynamicParent>,
    pub residual_scale: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InterventionTiming {
    /// Replace the treatment equation at one observed time point.
    Point { time: usize },
    /// Replace the treatment equation at the start and every later replayed time point.
    Persistent { start: usize },
}

impl InterventionTiming {
    pub fn start(self) -> usize {
        match self {
            Self::Point { time } => time,
            Self::Persistent { start } => start,
        }
    }

    fn applies(self, time: usize) -> bool {
        match self {
            Self::Point { time: point } => time == point,
            Self::Persistent { start } => time >= start,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicCounterfactualPath {
    pub start: usize,
    pub factual: Vec<f64>,
    pub low: Vec<f64>,
    pub high: Vec<f64>,
    /// `high - low` at each replayed time point.
    pub contrast: Vec<f64>,
    pub average_contrast: f64,
    pub cumulative_contrast: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicLinearScm {
    /// One equation per variable, indexed by variable position.
    pub equations: Vec<DynamicEquation>,
    /// Causal evaluation order for contemporaneous arrows.
    pub order: Vec<usize>,
    pub max_lag: usize,
    /// Number of complete lagged rows used by `fit`; zero for a supplied SEM.
    pub fitted_observations: usize,
    /// One entry per equation. Supplied SEMs have no fit diagnostics.
    pub fit_diagnostics: Vec<Option<DynamicEquationFitDiagnostics>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicEquationFitDiagnostics {
    pub predictors: usize,
    pub numerical_rank: usize,
    pub singular_values: Vec<f64>,
    pub singular_value_cutoff: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DynamicScmError {
    EmptyModel,
    Shape,
    NonFinite,
    ParentOutOfRange {
        child: usize,
        parent: usize,
    },
    DuplicateParent {
        child: usize,
        parent: usize,
        lag: usize,
    },
    ContemporaneousCycle,
    InsufficientRows {
        rows: usize,
        max_lag: usize,
    },
    EmptyReferencePoints,
    InvalidReferencePoint {
        time: usize,
        rows: usize,
        max_lag: usize,
    },
    LinearRegression {
        variable: usize,
        error: SklearnLinearError,
    },
    VariableOutOfRange {
        variable: usize,
    },
    LagOutOfRange {
        lag: usize,
        max_lag: usize,
    },
    InvalidWindow {
        start: usize,
        steps: usize,
        rows: usize,
    },
    MissingObservedHistory {
        start: usize,
        max_lag: usize,
    },
}

impl fmt::Display for DynamicScmError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyModel => formatter.write_str("a dynamic SCM needs at least one variable"),
            Self::Shape => {
                formatter.write_str("time-series columns must have the same non-zero length")
            }
            Self::NonFinite => formatter.write_str("dynamic SCM inputs must be finite"),
            Self::ParentOutOfRange { child, parent } => write!(
                formatter,
                "parent {parent} of variable {child} is outside the model"
            ),
            Self::DuplicateParent { child, parent, lag } => write!(
                formatter,
                "variable {child} repeats parent {parent} at lag {lag}"
            ),
            Self::ContemporaneousCycle => {
                formatter.write_str("contemporaneous dynamic-SCM arrows must be acyclic")
            }
            Self::InsufficientRows { rows, max_lag } => write!(
                formatter,
                "{rows} rows do not leave a complete observation after lag {max_lag}"
            ),
            Self::EmptyReferencePoints => {
                formatter.write_str("dynamic SCM fitting needs at least one reference point")
            }
            Self::InvalidReferencePoint {
                time,
                rows,
                max_lag,
            } => write!(
                formatter,
                "reference point {time} must be at least lag {max_lag} and below {rows} rows"
            ),
            Self::LinearRegression { variable, error } => write!(
                formatter,
                "linear regression for variable {variable} failed: {error}"
            ),
            Self::VariableOutOfRange { variable } => {
                write!(formatter, "variable {variable} is outside the model")
            }
            Self::LagOutOfRange { lag, max_lag } => {
                write!(
                    formatter,
                    "lag {lag} exceeds the model maximum lag {max_lag}"
                )
            }
            Self::InvalidWindow { start, steps, rows } => write!(
                formatter,
                "the replay window starting at {start} for {steps} steps exceeds {rows} rows"
            ),
            Self::MissingObservedHistory { start, max_lag } => write!(
                formatter,
                "a fitted replay starting at {start} needs observed history through lag {max_lag}"
            ),
        }
    }
}

impl std::error::Error for DynamicScmError {}

impl DynamicLinearScm {
    /// Construct a supplied additive linear SEM. Exogenous innovations enter with coefficient one.
    pub fn from_equations(equations: Vec<DynamicEquation>) -> Result<Self, DynamicScmError> {
        let (order, max_lag) = validate_equations(&equations)?;
        Ok(Self {
            fit_diagnostics: vec![None; equations.len()],
            equations,
            order,
            max_lag,
            fitted_observations: 0,
        })
    }

    /// Fit one least-squares equation per variable on the graph-declared time-indexed parents.
    pub fn fit(
        data: &[Vec<f64>],
        parents: &[Vec<DynamicParentSpec>],
    ) -> Result<Self, DynamicScmError> {
        let (variables, rows) = validate_data(data)?;
        if parents.len() != variables {
            return Err(DynamicScmError::Shape);
        }
        let provisional: Vec<DynamicEquation> = parents
            .iter()
            .map(|specs| DynamicEquation {
                intercept: 0.0,
                parents: specs
                    .iter()
                    .map(|parent| DynamicParent {
                        variable: parent.variable,
                        lag: parent.lag,
                        coefficient: 0.0,
                    })
                    .collect(),
                residual_scale: 0.0,
            })
            .collect();
        let (order, max_lag) = validate_equations(&provisional)?;
        if rows <= max_lag {
            return Err(DynamicScmError::InsufficientRows { rows, max_lag });
        }
        let reference_points: Vec<usize> = (max_lag..rows).collect();
        Self::fit_validated(data, parents, variables, order, max_lag, &reference_points)
    }

    /// Fit on selected complete reference points. Repeated points are permitted for bootstrap
    /// samples; every lagged predictor is read relative to its original reference point, so block
    /// boundaries never create artificial temporal neighbours.
    pub fn fit_at_reference_points(
        data: &[Vec<f64>],
        parents: &[Vec<DynamicParentSpec>],
        reference_points: &[usize],
    ) -> Result<Self, DynamicScmError> {
        let (variables, rows) = validate_data(data)?;
        if parents.len() != variables {
            return Err(DynamicScmError::Shape);
        }
        let provisional: Vec<DynamicEquation> = parents
            .iter()
            .map(|specs| DynamicEquation {
                intercept: 0.0,
                parents: specs
                    .iter()
                    .map(|parent| DynamicParent {
                        variable: parent.variable,
                        lag: parent.lag,
                        coefficient: 0.0,
                    })
                    .collect(),
                residual_scale: 0.0,
            })
            .collect();
        let (order, max_lag) = validate_equations(&provisional)?;
        if reference_points.is_empty() {
            return Err(DynamicScmError::EmptyReferencePoints);
        }
        if let Some(&time) = reference_points
            .iter()
            .find(|&&time| time < max_lag || time >= rows)
        {
            return Err(DynamicScmError::InvalidReferencePoint {
                time,
                rows,
                max_lag,
            });
        }
        Self::fit_validated(data, parents, variables, order, max_lag, reference_points)
    }

    fn fit_validated(
        data: &[Vec<f64>],
        parents: &[Vec<DynamicParentSpec>],
        variables: usize,
        order: Vec<usize>,
        max_lag: usize,
        reference_points: &[usize],
    ) -> Result<Self, DynamicScmError> {
        let nobs = reference_points.len();
        let mut equations = Vec::with_capacity(variables);
        let mut fit_diagnostics = Vec::with_capacity(variables);
        for (child, specs) in parents.iter().enumerate() {
            let mut design = DMatrix::<f64>::zeros(nobs, specs.len());
            let mut outcome = DVector::<f64>::zeros(nobs);
            for (sample, &time) in reference_points.iter().enumerate() {
                outcome[sample] = data[child][time];
                for (column, parent) in specs.iter().enumerate() {
                    design[(sample, column)] = data[parent.variable][time - parent.lag];
                }
            }
            let fit = fit_sklearn_linear_regression(&design, &outcome, SKLEARN_LINEAR_TOLERANCE)
                .map_err(|error| DynamicScmError::LinearRegression {
                    variable: child,
                    error,
                })?;
            fit_diagnostics.push(Some(DynamicEquationFitDiagnostics {
                predictors: specs.len(),
                numerical_rank: fit.rank,
                singular_values: fit.singular_values.as_slice().to_vec(),
                singular_value_cutoff: fit.singular_value_cutoff,
            }));
            equations.push(DynamicEquation {
                intercept: fit.intercept,
                parents: specs
                    .iter()
                    .enumerate()
                    .map(|(index, parent)| DynamicParent {
                        variable: parent.variable,
                        lag: parent.lag,
                        coefficient: fit.coefficients[index],
                    })
                    .collect(),
                residual_scale: fit.residual_scale,
            });
        }
        Ok(Self {
            equations,
            order,
            max_lag,
            fitted_observations: nobs,
            fit_diagnostics,
        })
    }

    /// Generate the factual series from supplied additive innovations, using Tigramite's zero
    /// default before the beginning of a series.
    pub fn generate(&self, innovations: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, DynamicScmError> {
        let (variables, rows) = validate_data(innovations)?;
        if variables != self.equations.len() {
            return Err(DynamicScmError::Shape);
        }
        let mut values = vec![vec![0.0; rows]; variables];
        for time in 0..rows {
            for &child in &self.order {
                values[child][time] = self.mean_at(child, time, &values) + innovations[child][time];
            }
        }
        Ok(values)
    }

    /// Recover the additive innovation at every observed time point. Before-sample lagged values use
    /// the same zero default as Tigramite's continuous toy variables.
    pub fn abduct_innovations(&self, data: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, DynamicScmError> {
        let (variables, rows) = validate_data(data)?;
        if variables != self.equations.len() {
            return Err(DynamicScmError::Shape);
        }
        let mut innovations = vec![vec![0.0; rows]; variables];
        for time in 0..rows {
            for &child in &self.order {
                innovations[child][time] = data[child][time] - self.mean_at(child, time, data);
            }
        }
        Ok(innovations)
    }

    /// Reproduce Tigramite's known-SEM window replay for a point intervention on a source at a
    /// specified lag relative to each reference time. The output has one value for every complete
    /// reference window, from `max_lag` through the end of the series.
    pub fn point_intervention_windows(
        &self,
        data: &[Vec<f64>],
        treatment: usize,
        treatment_lag: usize,
        outcome: usize,
        value: f64,
    ) -> Result<Vec<f64>, DynamicScmError> {
        self.validate_variables(treatment, outcome)?;
        if treatment_lag > self.max_lag {
            return Err(DynamicScmError::LagOutOfRange {
                lag: treatment_lag,
                max_lag: self.max_lag,
            });
        }
        if !value.is_finite() {
            return Err(DynamicScmError::NonFinite);
        }
        let (_, rows) = validate_data(data)?;
        if rows <= self.max_lag {
            return Err(DynamicScmError::InsufficientRows {
                rows,
                max_lag: self.max_lag,
            });
        }
        let innovations = self.abduct_innovations(data)?;
        let mut outputs = Vec::with_capacity(rows - self.max_lag);
        for reference in self.max_lag..rows {
            // Tigramite evaluates every delta in the local window with the innovation at the
            // reference point. Values whose lag reaches before the computed part of the window are
            // read from the factual series at that lag relative to the reference point.
            let mut window = vec![vec![0.0; self.equations.len()]; self.max_lag + 1];
            for delta in 0..=self.max_lag {
                for &child in &self.order {
                    if child == treatment && self.max_lag - delta == treatment_lag {
                        window[delta][child] = value;
                        continue;
                    }
                    let equation = &self.equations[child];
                    let mut current = equation.intercept + innovations[child][reference];
                    for parent in &equation.parents {
                        let parent_value = if delta < parent.lag {
                            data[parent.variable][reference - parent.lag]
                        } else {
                            window[delta - parent.lag][parent.variable]
                        };
                        current += parent.coefficient * parent_value;
                    }
                    window[delta][child] = current;
                }
            }
            outputs.push(window[self.max_lag][outcome]);
        }
        Ok(outputs)
    }

    /// Abduct the observed innovations, replace the treatment equation, and replay both worlds.
    /// `steps` includes the intervention time itself.
    pub fn counterfactual_pair(
        &self,
        data: &[Vec<f64>],
        treatment: usize,
        outcome: usize,
        timing: InterventionTiming,
        steps: usize,
        low: f64,
        high: f64,
    ) -> Result<DynamicCounterfactualPath, DynamicScmError> {
        self.validate_variables(treatment, outcome)?;
        let (_, rows) = validate_data(data)?;
        let start = timing.start();
        if start < self.max_lag {
            return Err(DynamicScmError::MissingObservedHistory {
                start,
                max_lag: self.max_lag,
            });
        }
        if steps == 0 || start.checked_add(steps).is_none() || start + steps > rows {
            return Err(DynamicScmError::InvalidWindow { start, steps, rows });
        }
        if !low.is_finite() || !high.is_finite() {
            return Err(DynamicScmError::NonFinite);
        }
        let innovations = self.abduct_innovations(data)?;
        let end = start + steps - 1;
        let low_world = self.replay_range(data, &innovations, start, end, treatment, low, timing);
        let high_world = self.replay_range(data, &innovations, start, end, treatment, high, timing);
        let factual = data[outcome][start..=end].to_vec();
        let low_path = low_world[outcome][start..=end].to_vec();
        let high_path = high_world[outcome][start..=end].to_vec();
        let contrast: Vec<f64> = high_path
            .iter()
            .zip(&low_path)
            .map(|(upper, lower)| upper - lower)
            .collect();
        let cumulative_contrast = contrast.iter().sum::<f64>();
        Ok(DynamicCounterfactualPath {
            start,
            factual,
            low: low_path,
            high: high_path,
            average_contrast: cumulative_contrast / steps as f64,
            cumulative_contrast,
            contrast,
        })
    }

    fn validate_variables(&self, treatment: usize, outcome: usize) -> Result<(), DynamicScmError> {
        for variable in [treatment, outcome] {
            if variable >= self.equations.len() {
                return Err(DynamicScmError::VariableOutOfRange { variable });
            }
        }
        Ok(())
    }

    fn mean_at(&self, child: usize, time: usize, values: &[Vec<f64>]) -> f64 {
        let equation = &self.equations[child];
        equation
            .parents
            .iter()
            .fold(equation.intercept, |mean, parent| {
                let value = if time < parent.lag {
                    0.0
                } else {
                    values[parent.variable][time - parent.lag]
                };
                mean + parent.coefficient * value
            })
    }

    fn replay_range(
        &self,
        factual: &[Vec<f64>],
        innovations: &[Vec<f64>],
        recompute_start: usize,
        end: usize,
        treatment: usize,
        value: f64,
        timing: InterventionTiming,
    ) -> Vec<Vec<f64>> {
        let mut replay = factual.to_vec();
        for time in recompute_start..=end {
            for &child in &self.order {
                replay[child][time] = if child == treatment && timing.applies(time) {
                    value
                } else {
                    self.mean_at(child, time, &replay) + innovations[child][time]
                };
            }
        }
        replay
    }
}

fn validate_data(data: &[Vec<f64>]) -> Result<(usize, usize), DynamicScmError> {
    let Some(first) = data.first() else {
        return Err(DynamicScmError::Shape);
    };
    if first.is_empty() || data.iter().any(|column| column.len() != first.len()) {
        return Err(DynamicScmError::Shape);
    }
    if data.iter().flatten().any(|value| !value.is_finite()) {
        return Err(DynamicScmError::NonFinite);
    }
    Ok((data.len(), first.len()))
}

fn validate_equations(
    equations: &[DynamicEquation],
) -> Result<(Vec<usize>, usize), DynamicScmError> {
    if equations.is_empty() {
        return Err(DynamicScmError::EmptyModel);
    }
    let variables = equations.len();
    let mut adjacency = vec![Vec::new(); variables];
    let mut indegree = vec![0usize; variables];
    let mut max_lag = 0usize;
    for (child, equation) in equations.iter().enumerate() {
        if !equation.intercept.is_finite()
            || !equation.residual_scale.is_finite()
            || equation.residual_scale < 0.0
        {
            return Err(DynamicScmError::NonFinite);
        }
        let mut seen = HashSet::new();
        for parent in &equation.parents {
            if parent.variable >= variables {
                return Err(DynamicScmError::ParentOutOfRange {
                    child,
                    parent: parent.variable,
                });
            }
            if !parent.coefficient.is_finite() {
                return Err(DynamicScmError::NonFinite);
            }
            if !seen.insert((parent.variable, parent.lag)) {
                return Err(DynamicScmError::DuplicateParent {
                    child,
                    parent: parent.variable,
                    lag: parent.lag,
                });
            }
            max_lag = max_lag.max(parent.lag);
            if parent.lag == 0 {
                adjacency[parent.variable].push(child);
                indegree[child] += 1;
            }
        }
    }
    let mut ready: BTreeSet<usize> = indegree
        .iter()
        .enumerate()
        .filter_map(|(node, degree)| (*degree == 0).then_some(node))
        .collect();
    let mut order = Vec::with_capacity(variables);
    while let Some(node) = ready.pop_first() {
        order.push(node);
        for &child in &adjacency[node] {
            indegree[child] -= 1;
            if indegree[child] == 0 {
                ready.insert(child);
            }
        }
    }
    if order.len() != variables {
        return Err(DynamicScmError::ContemporaneousCycle);
    }
    Ok((order, max_lag))
}
