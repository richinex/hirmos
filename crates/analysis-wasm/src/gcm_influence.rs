//! Variance contributions and incoming-arrow strengths have separate output meanings.

use hirmos_causal_core::gcm::{
    additive::Output,
    influence::{approximate_icc, generated_arrow_strengths, Batch},
    model::{AssignedNode, Assignment, Graph},
    selection::Kind,
    shapley::{Execution, Method},
};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use crate::root_cause::{Random, RandomState};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    names: Vec<String>,
    edges: Vec<(usize, usize)>,
    rows: NonZeroUsize,
    target: usize,
    random: Random,
    query: Query,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
enum Query {
    Intrinsic { training: NonZeroUsize, randomization: NonZeroUsize, baseline: NonZeroUsize },
    Arrows { conditional: NonZeroUsize, max_runs: NonZeroUsize, tolerance: f64 },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(crate) enum Outcome {
    Intrinsic { nodes: Vec<usize>, values: Vec<f64> },
    Arrows { nodes: Vec<usize>, values: Vec<f64> },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum Predictor { Linear, Quadratic, Boosted }
impl From<Kind> for Predictor {
    fn from(value: Kind) -> Self {
        match value { Kind::Linear => Self::Linear, Kind::Quadratic => Self::Quadratic, Kind::Boosted => Self::Boosted }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum Noise { Continuous, Discrete }

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum Mechanism {
    Empirical { node: usize },
    Additive { node: usize, predictor: Predictor, noise: Noise },
}

#[derive(Serialize)]
pub(crate) struct Evidence {
    outcome: Outcome,
    mechanisms: Vec<Mechanism>,
    random: RandomState,
}

pub(crate) fn run(request: Request, values: &[f64], progress: &impl Fn(&'static str, usize, usize)) -> Result<Evidence, String> {
    let Request { names, edges, rows, target, random, query } = request;
    let graph = Graph::new(names, &edges).map_err(|_| "Choose an acyclic graph with distinct variable names.")?;
    let parents = graph.parents(target).ok_or("Choose a target in the graph.")?.to_vec();
    if rows.get() < 5 { return Err("Automatic selection requires at least five observations.".into()); }
    match &query {
        Query::Intrinsic { training, .. } if training.get() < 5 || training.get() > 200_000 => return Err("Use between 5 and 200,000 training samples.".into()),
        Query::Arrows { tolerance, .. } if !tolerance.is_finite() || *tolerance < 0.0 => return Err("The convergence tolerance must be finite and nonnegative.".into()),
        Query::Arrows { .. } if parents.is_empty() => return Err("Arrow strength requires a target with parents.".into()),
        _ => {}
    }
    super::matrix::validate_dense_matrix("Causal influence", values, rows.get(), graph.names().len())?;
    let data = DMatrix::from_column_slice(rows.get(), graph.names().len(), values);
    let (mut rng, mut cache) = random.restore()?;
    progress("Selecting conditional models", 0, 3);
    let assignment = Assignment::select(graph, &data, Execution::Isolated, &mut rng)
        .map_err(|error| format!("Conditional model selection failed: {error:?}"))?;
    let mechanisms = assignment.nodes().iter().enumerate().map(|(node, mechanism)| match mechanism {
        AssignedNode::Empirical => Mechanism::Empirical { node },
        AssignedNode::Additive { selection, output } => Mechanism::Additive { node, predictor: selection.best().into(), noise: match output { Output::Continuous => Noise::Continuous, Output::Discrete => Noise::Discrete } },
    }).collect();
    progress("Fitting conditional models", 1, 3);
    let model = assignment.fit(&data, &mut rng).map_err(|error| format!("Model fitting failed: {error:?}"))?;
    progress("Calculating causal influence", 2, 3);
    let outcome = match query {
        Query::Intrinsic { training, randomization, baseline } => {
            let result = approximate_icc(&model, target, training, randomization, baseline, Batch::All, Method::Auto, Execution::Isolated, &mut rng, &mut cache)
                .map_err(|error| format!("Variance attribution failed: {error:?}"))?.result;
            Outcome::Intrinsic { nodes: result.nodes, values: result.contribution.values }
        }
        Query::Arrows { conditional, max_runs, tolerance } => {
            let values = generated_arrow_strengths(&model, target, conditional, max_runs.get(), tolerance, Execution::Isolated, &mut rng, &mut cache)
                .map_err(|error| format!("Arrow strength calculation failed: {error:?}"))?;
            Outcome::Arrows { nodes: parents, values }
        }
    };
    progress("Causal influence complete", 3, 3);
    Ok(Evidence { outcome, mechanisms, random: RandomState { keys: rng.state().0.to_vec(), position: rng.state().1, normal: cache } })
}
