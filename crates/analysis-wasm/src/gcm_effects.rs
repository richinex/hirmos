//! Forward intervention effects; separate from analyses requiring invertible noise models.
use hirmos_causal_core::{
    causal_effects::numpy_percentile,
    gcm::{bootstrap::{confidence_intervals, ConfidenceLevel}, generative::{Forest, Intervention, MechanismSpec, Model},
        model::Graph, shapley::Execution},
    nprandom::Mt19937, numpy_reduce::numpy_mean,
};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, num::NonZeroUsize};

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum Mechanism { Empirical, Regression, Classifier { classes: NonZeroUsize } }
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum Grouping { None, Quantiles { column: usize, groups: usize, scope: Scope } }
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum Scope { All, LowerBound { minimum: f64 } }
impl Scope {
    fn contains(&self, value: f64) -> bool {
        match self { Self::All => true, Self::LowerBound { minimum } => value >= *minimum }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    names: Vec<String>, edges: Vec<(usize, usize)>, rows: NonZeroUsize,
    treatment: usize, outcome: usize, mechanisms: Vec<Mechanism>,
    trees: NonZeroUsize, min_leaf: NonZeroUsize, fit_seed: u32, simulation_seed: u32,
    repetitions: NonZeroUsize, upper_quantile: f64, grouping: Grouping,
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum Groups { None, Quantiles { column: usize, cutpoints: Vec<f64>, counts: Vec<usize>, scope: Scope, excluded: usize } }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum Status { Converged, IterationLimit, PrecisionLoss }
#[derive(Serialize)]
struct Optimizer { status: Status, iterations: usize }
#[derive(Serialize)]
pub(crate) struct Evidence {
    estimates: Vec<f64>, bounds: Vec<[f64; 2]>, quantiles: [f64; 2],
    replicates: Vec<Vec<f64>>, optimizer: Optimizer, grouping: Groups,
}

pub(crate) fn run(request: Request, values: &[f64], progress: &impl Fn(&'static str, usize, usize)) -> Result<Evidence, String> {
    let columns = request.names.len();
    let rows = request.rows.get();
    super::matrix::validate_dense_matrix("Intervention effects", values, rows, columns)?;
    if !(2..=64).contains(&columns) || request.treatment >= columns || request.outcome >= columns
        || request.treatment == request.outcome || request.mechanisms.len() != columns
        || request.trees.get() > 1000 || request.repetitions.get() > 1000 {
        return Err("Choose valid variables, models and sample counts.".into());
    }
    let level = ConfidenceLevel::new(request.upper_quantile).ok_or("Choose percentiles between 50 and 100.")?;
    if request.upper_quantile <= 0.5 || request.upper_quantile >= 1. { return Err("Choose percentiles between 50 and 100.".into()); }
    let graph = Graph::new(request.names.clone(), &request.edges).map_err(|_| "The graph must be acyclic with distinct variable names.")?;
    let data = DMatrix::from_column_slice(rows, columns, values);
    if (0..rows).any(|r| !matches!(data[(r,request.treatment)], 0. | 1.)) {
        return Err("The treatment must be coded 0 and 1.".into());
    }
    let grouping = match request.grouping {
        Grouping::None => Groups::None,
        Grouping::Quantiles { column, groups, scope } => {
            if column >= columns || column == request.treatment || column == request.outcome
                || !(2..=10).contains(&groups) || request.edges.iter().any(|&(_, child)| child == column) {
                return Err("Choose a grouping variable without parents, distinct from treatment and outcome.".into());
            }
            let values: Vec<_> = data.column(column).iter().copied().collect();
            if matches!(&scope, Scope::LowerBound { minimum } if !minimum.is_finite()) {
                return Err("The group lower bound must be finite.".into());
            }
            let cutpoints: Vec<_> = (1..groups).map(|g| numpy_percentile(&values, g as f64 / groups as f64)).collect();
            if cutpoints.windows(2).any(|pair| pair[0] >= pair[1]) { return Err("Repeated quantiles leave empty groups. Choose fewer groups.".into()); }
            let mut counts = vec![0; groups];
            let mut excluded = 0;
            for value in values {
                if scope.contains(value) { counts[cutpoints.partition_point(|&edge| edge <= value)] += 1; }
                else { excluded += 1; }
            }
            if counts.contains(&0) { return Err("The grouping leaves an empty group. Choose fewer groups.".into()); }
            Groups::Quantiles { column, cutpoints, counts, scope, excluded }
        }
    };
    let width = match &grouping { Groups::None => 1, Groups::Quantiles { counts, .. } => 1 + counts.len() };
    let specs = request.mechanisms.into_iter().map(|mechanism| {
        let forest = Forest { trees: request.trees, min_leaf: request.min_leaf };
        match mechanism {
            Mechanism::Empirical => MechanismSpec::Empirical,
            Mechanism::Regression => MechanismSpec::Regression(forest),
            Mechanism::Classifier { classes } => MechanismSpec::Classifier { forest, classes },
        }
    }).collect();
    progress("Fitting variable models", 0, request.repetitions.get());
    let model = Model::fit(graph, &data, specs, &mut Mt19937::seeded(request.fit_seed))
        .map_err(|error| match error {
            hirmos_causal_core::gcm::generative::Error::UnsupportedEncoding { node } =>
                format!("{} has categorical parents requiring an unsupported encoding. Review their model assignments.", request.names[node]),
            _ => "The variable models could not be fitted. Check category codes, finite values and model assignments.".to_string(),
        })?;
    let intervention = BTreeMap::from([(request.treatment, Intervention::uniform(vec![0., 1.]).unwrap())]);
    let mut completed = 0;
    let result = confidence_intervals(request.repetitions, level, Execution::Serial,
        &mut Mt19937::seeded(request.simulation_seed), &mut None, |rng, _| {
            let sample = model.interventional_samples(&data, &intervention, rng)
                .map_err(|_| "The intervention could not be simulated.".to_string())?;
            let mut control = vec![Vec::new(); width];
            let mut treated = vec![Vec::new(); width];
            for r in 0..rows {
                let arm = if sample[(r,request.treatment)] == 1. { &mut treated } else { &mut control };
                arm[0].push(sample[(r,request.outcome)]);
                if let Groups::Quantiles { column, cutpoints, scope, .. } = &grouping {
                    if !scope.contains(sample[(r,*column)]) { continue; }
                    let group = cutpoints.partition_point(|&edge| edge <= sample[(r,*column)]) + 1;
                    arm[group].push(sample[(r,request.outcome)]);
                }
            }
            if control.iter().chain(&treated).any(Vec::is_empty) {
                return Err("A simulated group contains only one treatment setting. Use larger groups.".to_string());
            }
            completed += 1;
            progress("Simulating intervention effects", completed, request.repetitions.get());
            Ok((0..width).map(|i| numpy_mean(&treated[i]) - numpy_mean(&control[i])).collect())
        }).map_err(|error| match error {
            hirmos_causal_core::gcm::bootstrap::BootstrapError::Evaluation(detail) => detail,
            hirmos_causal_core::gcm::bootstrap::BootstrapError::EmptyOutput => "The simulation returned no estimates.".to_string(),
            hirmos_causal_core::gcm::bootstrap::BootstrapError::OutputShape => "The simulations returned inconsistent estimate counts.".to_string(),
            hirmos_causal_core::gcm::bootstrap::BootstrapError::NonFinite |
            hirmos_causal_core::gcm::bootstrap::BootstrapError::Summary(_) => "The simulation estimates could not be summarized as finite values.".to_string(),
        })?;
    let summary = &result.summary().optimizer;
    let status = match summary.warnflag {
        0 => Status::Converged, 1 => Status::IterationLimit, 2 => Status::PrecisionLoss,
        _ => return Err("The summary optimizer returned an unknown status.".into()),
    };
    let samples = result.replicates().values();
    Ok(Evidence { estimates: summary.x.clone(), bounds: result.bounds().to_vec(),
        quantiles: [1. - request.upper_quantile, request.upper_quantile],
        replicates: (0..samples.nrows()).map(|r| (0..samples.ncols()).map(|c| samples[(r,c)]).collect()).collect(),
        optimizer: Optimizer { status, iterations: summary.nit }, grouping })
}
