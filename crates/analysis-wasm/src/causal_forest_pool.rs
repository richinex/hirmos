//! Scheduler adapter: replay the canonical tuning decision using ordered worker scores.
//! An absent score is pending work; a computed `None` is an unavailable OOB error.
use hirmos_causal_core::grf::{forest, sampling::Clusters, tree, tuning};
use serde::{Deserialize, Serialize};
use crate::causal_forest::{self as fit, Configuration, Target};

// Wire representations belong to the adapter, not to the numerical core.
#[derive(Serialize, Deserialize)]
#[serde(remote="forest::SeedMode")]
enum SeedMode { Indexed, Legacy }
#[derive(Serialize, Deserialize)]
#[serde(remote="tree::Honesty")]
enum Honesty { Disabled, Enabled { fraction: f64, prune: bool } }
#[derive(Serialize, Deserialize)]
#[serde(remote="tree::Options", deny_unknown_fields)]
struct TreeOptions {
    mtry: u32, min_node_size: usize, #[serde(with="Honesty")] honesty: tree::Honesty,
    alpha: f64, imbalance_penalty: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(remote="forest::Options", deny_unknown_fields)]
struct ForestOptions {
    trees: u32, group_size: usize, sample_fraction: f64, seed: u32,
    #[serde(with="SeedMode")] seed_mode: forest::SeedMode, batches: usize,
    #[serde(with="TreeOptions")] tree: tree::Options,
}
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct Options(#[serde(with="ForestOptions")] forest::Options);

#[derive(Deserialize)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
pub struct Request {
    rows: usize, columns: usize, treatment: usize, outcome: usize, adjustment: Vec<usize>,
    target: Target, configuration: Configuration, column_names: Vec<String>,
    scores: Vec<Vec<Option<f64>>>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
pub struct Batch {
    stage: usize, offset: usize, columns: Vec<Vec<f64>>, outcome: usize,
    treatment: Option<usize>, weight: Option<usize>, labels: Vec<usize>, per_cluster: usize,
    stabilize: bool, options: Vec<Options>,
}
#[derive(Serialize)]
#[serde(tag="kind", rename_all="kebab-case")]
pub enum Reply { Pending { batch: Batch }, Complete { evidence: fit::Evidence } }

pub fn plan(values: &[f64], request: Request) -> Result<Reply, String> {
    if request.scores.iter().flatten().flatten().any(|v| !v.is_finite()) {
        return Err("A tuning score is not finite.".into());
    }
    let mut pending = None;
    let mut stage = 0;
    let mut tuner = |columns: &[Vec<f64>], outcome: usize, treatment: Option<usize>, weight: Option<usize>, clusters: &Clusters,
        base: &forest::Options, config: &tuning::Config, stabilize: bool| {
        let prior = request.scores.get(stage).map(Vec::as_slice).unwrap_or(&[]);
        let mut next = 0;
        let mut options = Vec::new();
        let result = tuning::tune(columns[0].len(), outcome, base, &config.defaults(outcome), config, |option| {
            let index = next; next += 1;
            match prior.get(index) {
                Some(score) => Ok(*score),
                None => { options.push(Options(option.clone())); Ok(None) },
            }
        });
        if !options.is_empty() {
            let (labels, per_cluster) = clusters.membership();
            pending = Some(Batch { stage, offset: prior.len(), columns: columns.to_vec(), outcome, treatment, weight,
                labels, per_cluster, stabilize, options });
            return Err("Tuning scores pending.");
        }
        if prior.len() != next { return Err("Unexpected extra tuning scores."); }
        stage += 1;
        result
    };
    let result = fit::fit_scheduled(values, request.rows, request.columns, request.treatment, request.outcome,
        &request.adjustment, request.target, request.configuration, &request.column_names, Some(&mut tuner));
    match pending {
        Some(batch) => Ok(Reply::Pending { batch }),
        None => result.map(|evidence| Reply::Complete { evidence }),
    }
}

pub fn score(batch: &Batch, index: usize) -> Result<Option<f64>, String> {
    let options = &batch.options.get(index).ok_or("Unknown tuning candidate.")?.0;
    let rows = batch.labels.len();
    if rows < 2 || batch.columns.is_empty() || batch.columns.iter().any(|c| c.len() != rows || c.iter().any(|v| !v.is_finite()))
        || batch.outcome >= batch.columns.len() || batch.weight.is_some_and(|c| c >= batch.columns.len())
        || batch.treatment.is_some_and(|c| c >= batch.columns.len() || c == batch.outcome) {
        return Err("Invalid tuning matrix or roles.".into());
    }
    let clusters = Clusters::new(&batch.labels, batch.per_cluster).map_err(str::to_owned)?;
    match batch.treatment {
        None => tuning::regression_error(&batch.columns, batch.outcome, batch.weight, &clusters, options),
        Some(treatment) => tuning::causal_error(tree::Data { columns: &batch.columns, outcome: batch.outcome, treatment, weight: batch.weight }, &clusters, options, batch.stabilize),
    }.map_err(str::to_owned)
}
