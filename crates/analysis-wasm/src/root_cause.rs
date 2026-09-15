//! Validate root-cause requests and preserve the numerical core's output meanings.

use hirmos_causal_core::{
    gcm::{
        anomaly::attribute_anomalies,
        bootstrap::{confidence_intervals, fit_and_compute, ConfidenceLevel, SubsetFraction},
        distribution_change::distribution_change,
        model::Graph,
        shapley::{DistributionExecution, Execution, ProcessBatches},
    },
    nprandom::Mt19937,
    numpy_reduce::numpy_mean,
};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, num::NonZeroUsize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    names: Vec<String>,
    edges: Vec<(usize, usize)>,
    rows: NonZeroUsize,
    target: usize,
    repetitions: NonZeroUsize,
    upper_quantile: f64,
    fraction: f64,
    random: Random,
    query: Query,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum Random {
    Seed { seed: u32 },
    Resume { state: RandomState },
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct RandomState {
    pub(crate) keys: Vec<u32>,
    pub(crate) position: usize,
    pub(crate) normal: Option<f64>,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum Query {
    Anomaly { samples: NonZeroUsize },
    Change { rows: NonZeroUsize, samples: NonZeroUsize, execution: ChangeExecution },
    Intervention { rows: NonZeroUsize, order: Vec<usize>, shifts: Vec<Shift> },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Shift { node: usize, amount: f64 }

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum ChangeExecution {
    IndependentJobs,
    RecordedBatches { repetitions: Vec<Vec<Vec<Vec<bool>>>> },
}

enum ChangePlan {
    IndependentJobs,
    RecordedBatches(Vec<ProcessBatches>),
}

impl ChangePlan {
    fn at(&self, index: usize) -> DistributionExecution<'_> {
        match self {
            Self::IndependentJobs => DistributionExecution::IndependentJobs,
            Self::RecordedBatches(plans) => DistributionExecution::RecordedBatches(&plans[index]),
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum Outcome {
    Anomaly { nodes: Vec<usize>, summary: Summary },
    Change { nodes: Vec<usize>, summary: Summary },
    Intervention { target: usize, nodes: Vec<usize>, observed_mean: f64, summary: Summary },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Summary {
    estimates: Vec<f64>,
    bounds: Vec<[f64; 2]>,
    quantiles: [f64; 2],
    replicates: Vec<Vec<f64>>,
    optimizer_status: u8,
}

#[derive(Serialize)]
pub(crate) struct Evidence {
    outcome: Outcome,
    random: RandomState,
}

/// Values contain two consecutive column-major matrices: fitting data, then query data.
pub(crate) fn run(
    request: Request,
    values: &[f64],
    progress: &impl Fn(&'static str, usize, usize),
) -> Result<Evidence, String> {
    let Request { names, edges, rows, target, repetitions, upper_quantile, fraction, random, query } = request;
    let graph = Graph::new(names, &edges).map_err(|_| "Choose an acyclic graph with distinct variable names.".to_owned())?;
    if target >= graph.names().len() { return Err("The target must be a graph variable.".into()); }
    let level = ConfidenceLevel::new(upper_quantile).ok_or("The upper percentile must be between 0.5 and 1.")?;
    let subset = SubsetFraction::new(fraction).ok_or("The fitting fraction must be greater than zero and at most one.")?;
    let query_rows = match &query {
        Query::Anomaly { .. } => 1,
        Query::Change { rows, .. } | Query::Intervention { rows, .. } => rows.get(),
    };
    let total_rows = rows.get().checked_add(query_rows).ok_or("The matrix dimensions are too large.")?;
    let columns = graph.names().len();
    super::matrix::validate_dense_matrix("Root-cause analysis", values, total_rows, columns)?;
    let split = rows.get() * columns;
    let baseline = DMatrix::from_column_slice(rows.get(), columns, &values[..split]);
    let observed = DMatrix::from_column_slice(query_rows, columns, &values[split..]);
    if (rows.get() as f64 * fraction) < 1.0 { return Err("The fitting fraction leaves no observations.".into()); }
    if matches!(query, Query::Change { .. }) && (query_rows as f64 * fraction) < 1.0 {
        return Err("The comparison fitting fraction leaves no observations.".into());
    }
    let (mut rng, mut cache) = match random {
        Random::Seed { seed } => (Mt19937::seeded(seed), None),
        Random::Resume { state } => {
            if state.normal.is_some_and(|value| !value.is_finite()) { return Err("The saved random state is invalid.".into()); }
            (Mt19937::from_state(&state.keys, state.position).ok_or("The saved random state is invalid.")?, state.normal)
        }
    };
    let mut nodes = graph.ancestors(target).map_err(|_| "The target is not in the graph.")?;
    if matches!(query, Query::Anomaly { .. }) {
        nodes = graph.order().iter().copied().filter(|node| nodes.contains(node)).collect();
    }
    let mut shifts = BTreeMap::new();
    let mut change_plan = ChangePlan::IndependentJobs;
    match &query {
        Query::Intervention { shifts: changes, order, .. } => {
            let mut ordered = order.clone();
            ordered.sort_unstable();
            if ordered != (0..columns).collect::<Vec<_>>() { return Err("The summary order must contain every graph variable once.".into()); }
            nodes = order.clone();
            if changes.is_empty() { return Err("Specify at least one shift.".into()); }
            for shift in changes {
                if shift.node >= columns || !shift.amount.is_finite() || shifts.insert(shift.node, shift.amount).is_some() {
                    return Err("Each shift must name a distinct graph variable and a finite amount.".into());
                }
            }
        }
        Query::Change { execution: ChangeExecution::RecordedBatches { repetitions: batches }, .. } => {
            if batches.len() != repetitions.get() { return Err("Supply one recorded batch plan per repetition.".into()); }
            let mut plans = Vec::with_capacity(batches.len());
            for batch in batches {
                if batch.iter().flatten().any(|coalition| coalition.len() != nodes.len()) {
                    return Err("Recorded coalitions must match the target and its ancestors.".into());
                }
                plans.push(ProcessBatches::new(batch.clone()).map_err(|_| "The recorded batch plan is invalid.")?);
            }
            change_plan = ChangePlan::RecordedBatches(plans);
        }
        _ => {}
    }
    if matches!(query, Query::Change { .. }) { nodes.sort_by(|&a, &b| graph.names()[a].cmp(&graph.names()[b])); }
    let mut completed = 0;
    progress("Root-cause analysis", 0, repetitions.get());
    let estimate = confidence_intervals(repetitions, level, Execution::Serial, &mut rng, &mut cache, |rng, cache| {
        let values = match &query {
            Query::Anomaly { samples } => fit_and_compute(&graph, &baseline, subset, rng, cache, |model, rng, cache| {
                let attribution = attribute_anomalies(model, target, &observed, *samples, Execution::Isolated, rng, cache)
                    .map_err(|error| format!("Anomaly attribution failed: {error:?}"))?;
                align(&nodes, &attribution.nodes, &attribution.values[0])
            }).map_err(|error| format!("Anomaly fitting failed: {error:?}"))?,
            Query::Intervention { .. } => fit_and_compute(&graph, &baseline, subset, rng, cache, |model, rng, _| {
                let predictions = model.interventional_samples(&observed, &shifts, rng)?;
                Ok::<_, hirmos_causal_core::gcm::model::ModelError>(nodes.iter().map(|&node| numpy_mean(predictions.column(node).as_slice())).collect())
            }).map_err(|error| format!("Intervention fitting failed: {error:?}"))?,
            Query::Change { samples, .. } => {
                let old_order = rng.permutation(baseline.nrows());
                let new_order = rng.permutation(observed.nrows());
                let old = DMatrix::from_fn((baseline.nrows() as f64 * fraction) as usize, columns, |r, c| baseline[(old_order[r], c)]);
                let new = DMatrix::from_fn((observed.nrows() as f64 * fraction) as usize, columns, |r, c| observed[(new_order[r], c)]);
                let change = distribution_change(&graph, &old, &new, target, *samples, change_plan.at(completed), rng, cache)
                    .map_err(|error| format!("Distribution comparison failed: {error:?}"))?;
                align(&nodes, &change.attribution.nodes, &change.attribution.contributions)?
            }
        };
        completed += 1;
        progress("Root-cause analysis", completed, repetitions.get());
        Ok::<_, String>(values)
    }).map_err(|error| format!("Root-cause estimation failed: {error:?}"))?;
    let replicates = estimate.replicates().values();
    let summary = Summary {
        estimates: estimate.summary().optimizer.x.clone(), bounds: estimate.bounds().to_vec(),
        quantiles: [1.0 - upper_quantile, upper_quantile],
        replicates: (0..replicates.nrows()).map(|r| (0..replicates.ncols()).map(|c| replicates[(r, c)]).collect()).collect(),
        optimizer_status: estimate.summary().optimizer.warnflag,
    };
    let outcome = match query {
        Query::Anomaly { .. } => Outcome::Anomaly { nodes, summary },
        Query::Change { .. } => Outcome::Change { nodes, summary },
        Query::Intervention { .. } => Outcome::Intervention { target, nodes, observed_mean: numpy_mean(observed.column(target).as_slice()), summary },
    };
    Ok(Evidence { outcome, random: RandomState { keys: rng.state().0.to_vec(), position: rng.state().1, normal: cache } })
}

fn align(nodes: &[usize], reported: &[usize], values: &[f64]) -> Result<Vec<f64>, String> {
    nodes.iter().map(|node| reported.iter().position(|candidate| candidate == node)
        .and_then(|index| values.get(index).copied()).ok_or_else(|| "An attribution is missing a graph variable.".to_owned())).collect()
}
