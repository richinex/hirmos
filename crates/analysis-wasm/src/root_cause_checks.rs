//! Default v0.14 model evaluation, in its original random-state order.
use hirmos_causal_core::{gcm::{evaluation::{self, NodePerformance}, falsification::{self, Validation, Verdict}, model::{FittedModel, Graph}, shapley::Execution}, nprandom::Mt19937};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request { names: Vec<String>, edges: Vec<(usize, usize)>, rows: NonZeroUsize, seed: u32, #[serde(default)] scope: Scope }

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Scope { #[default] Full, Fitted }

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum Performance {
    Root { node: usize, kl_divergence: f64 },
    Conditional { node: usize, crps: f64, mse: f64, nmse: f64, r2: f64 },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Invertibility { node: usize, p_value: f64, rejected: bool }

#[derive(Serialize)]
struct Implication { x: usize, y: usize, given: Vec<usize>, p: f64 }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GraphCheck { order: Vec<usize>, implications: Vec<Implication>, lmc_violations: usize, tpa_violations: usize }

impl From<Validation> for GraphCheck {
    fn from(value: Validation) -> Self {
        Self { order: value.order, implications: value.implications.into_iter().zip(value.p_values).map(|(test, p)| Implication { x: test.x, y: test.y, given: test.given, p }).collect(), lmc_violations: value.lmc_violations, tpa_violations: value.tpa_violations }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum Decision { NoImplications, NotInformative, Retained, Rejected }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FullEvidence {
    mechanisms: Vec<Performance>, invertibility: Vec<Invertibility>, overall_divergence: f64,
    given: GraphCheck, permutations: Vec<GraphCheck>, p_value_lmc: f64, p_value_tpa: f64, verdict: Decision,
    random: super::root_cause::RandomState,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FittedEvidence {
    scope: FittedScope,
    mechanisms: Vec<Performance>, invertibility: Vec<Invertibility>, overall_divergence: f64,
    random: super::root_cause::RandomState,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum FittedScope { Fitted }

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum Evidence { Full(FullEvidence), Fitted(FittedEvidence) }

pub(crate) fn run(request: Request, values: &[f64], progress: &impl Fn(&'static str, usize, usize)) -> Result<Evidence, String> {
    let graph = Graph::new(request.names, &request.edges).map_err(|error| format!("Invalid model graph: {error:?}"))?;
    super::matrix::validate_dense_matrix("Root-cause model checks", values, request.rows.get(), graph.names().len())?;
    let data = DMatrix::from_column_slice(request.rows.get(), graph.names().len(), values);
    let model = FittedModel::fit(graph, &data).map_err(|error| format!("Model fitting failed: {error:?}"))?;
    let mut rng = Mt19937::seeded(request.seed);
    let mut cache = None;
    let stages = match request.scope { Scope::Full => 4, Scope::Fitted => 3 };
    progress("Model performance", 0, stages);
    let mechanisms = evaluation::mechanism_performance(model.graph(), &data, NonZeroUsize::new(5).unwrap(), Execution::Isolated, &mut rng, &mut cache).map_err(|error| format!("Model performance failed: {error:?}"))?;
    progress("Noise independence", 1, stages);
    let invertibility = evaluation::invertibility(&model, &data, Execution::Isolated, &mut rng, &mut cache).map_err(|error| format!("Noise independence failed: {error:?}"))?;
    progress("Generated distribution", 2, stages);
    let overall_divergence = evaluation::overall_divergence(&model, &data, &mut rng, &mut cache).map_err(|error| format!("Distribution check failed: {error:?}"))?;
    let mechanisms = mechanisms.into_iter().map(|entry| match entry {
            NodePerformance::Root { node, kl_divergence } => Performance::Root { node, kl_divergence },
            NodePerformance::Conditional { node, crps, metrics } => Performance::Conditional { node, crps, mse: metrics.mse, nmse: metrics.nmse, r2: metrics.r2 },
        }).collect();
    let invertibility = invertibility.into_iter().map(|entry| Invertibility { node: entry.node, p_value: entry.p_value, rejected: entry.rejected }).collect();
    if matches!(request.scope, Scope::Fitted) {
        progress("Fitted-model checks complete", stages, stages);
        return Ok(Evidence::Fitted(FittedEvidence { scope: FittedScope::Fitted, mechanisms, invertibility, overall_divergence,
            random: super::root_cause::RandomState { keys: rng.state().0.to_vec(), position: rng.state().1, normal: cache } }));
    }
    progress("Graph checks", 3, stages);
    let checked = falsification::evaluate(model.graph(), &data, NonZeroUsize::new(50).unwrap(), Execution::Isolated, &mut rng, &mut cache).map_err(|error| format!("Graph checks failed: {error:?}"))?;
    progress("Model checks complete", stages, stages);
    Ok(Evidence::Full(FullEvidence {
        mechanisms, invertibility,
        overall_divergence, given: checked.given.into(), permutations: checked.permutations.into_iter().map(Into::into).collect(),
        p_value_lmc: checked.p_value_lmc, p_value_tpa: checked.p_value_tpa,
        verdict: match checked.verdict { Verdict::NoImplications => Decision::NoImplications, Verdict::NotInformative => Decision::NotInformative, Verdict::Retained => Decision::Retained, Verdict::Rejected => Decision::Rejected },
        random: super::root_cause::RandomState { keys: rng.state().0.to_vec(), position: rng.state().1, normal: cache },
    }))
}
