// Copyright (c) 2024 GRF Contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
// R causal_forest orthogonalization and staged tuning.
use super::{
    forest::{self, Forest, Options},
    regression,
    sampling::Clusters,
    tree::{Data, Honesty},
    tuning,
};
pub struct Model {
    pub forest: Forest,
    pub outcome_hat: Vec<f64>,
    pub treatment_hat: Vec<f64>,
    pub tuning: Option<Tuning>,
}
pub struct Tuning {
    pub outcome: Option<tuning::Output>,
    pub treatment: Option<tuning::Output>,
    pub causal: tuning::Output,
}

/// Scheduling seam. The numerical tuning algorithm remains `tuning::tune`.
/// `treatment = None` denotes regression; otherwise columns are residualized.
pub type Tuner<'a> = dyn FnMut(&[Vec<f64>], usize, Option<usize>, Option<usize>, &Clusters,
    &Options, &tuning::Config, bool) -> Result<tuning::Output, &'static str> + 'a;

pub fn fit_tuned_with(
    x: &[Vec<f64>], y: &[f64], w: &[f64], weights: Option<&[f64]>, clusters: &Clusters,
    options: &Options, y_hat: Option<&[f64]>, w_hat: Option<&[f64]>, stabilize: bool,
    nuisance_prune: bool, config: &tuning::Config, tuner: &mut Tuner<'_>,
) -> Result<Model, &'static str> {
    fit_internal(x, y, w, weights, clusters, options, y_hat, w_hat, stabilize,
        nuisance_prune, Some(config), Some(tuner))
}

/// Tune the nuisance regressions and the residualized causal forest as R does.
/// Nuisance regressions retain their own upstream tuning budgets, even when
/// the caller changes the causal stage's budget.
pub fn fit_tuned(
    x: &[Vec<f64>],
    y: &[f64],
    w: &[f64],
    weights: Option<&[f64]>,
    clusters: &Clusters,
    options: &Options,
    y_hat: Option<&[f64]>,
    w_hat: Option<&[f64]>,
    stabilize: bool,
    nuisance_prune: bool,
    config: &tuning::Config,
) -> Result<Model, &'static str> {
    fit_internal(
        x,
        y,
        w,
        weights,
        clusters,
        options,
        y_hat,
        w_hat,
        stabilize,
        nuisance_prune,
        Some(config),
        None,
    )
}
/// Auxiliary conditional-variance forest used by get_scores for continuous W.
pub fn continuous_debiasing(
    x: &[Vec<f64>],
    w: &[f64],
    w_hat: &[f64],
    weights: Option<&[f64]>,
    clusters: &Clusters,
    seed: u32,
    batches: usize,
    mode: forest::SeedMode,
    trees: u32,
) -> Result<Vec<f64>, &'static str> {
    if w.len() != w_hat.len() || w.is_empty() {
        return Err("Invalid treatment residuals.");
    }
    let mut columns = x.to_vec();
    columns.push(w.iter().zip(w_hat).map(|(w, p)| (w - p).powi(2)).collect());
    let weight = weights.map(|v| {
        columns.push(v.to_vec());
        columns.len() - 1
    });
    let options = Options {
        trees,
        group_size: 1,
        sample_fraction: 0.5,
        seed,
        seed_mode: mode,
        batches,
        tree: super::tree::Options {
            mtry: ((x.len() as f64).sqrt() + 20.0).ceil().min(x.len() as f64) as u32,
            min_node_size: 5,
            honesty: Honesty::Enabled {
                fraction: 0.5,
                prune: true,
            },
            alpha: 0.05,
            imbalance_penalty: 0.0,
        },
    };
    let variance =
        regression::train(&columns, x.len(), weight, clusters, &options)?.predict(x, true)?;
    variance
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            let v = v.ok_or("Conditional variance prediction is unavailable.")?;
            if v <= 0.0 {
                return Err("Conditional treatment variance must be positive.");
            }
            Ok((w[i] - w_hat[i]) / v)
        })
        .collect()
}
pub fn fit(
    x: &[Vec<f64>],
    y: &[f64],
    w: &[f64],
    weights: Option<&[f64]>,
    clusters: &Clusters,
    options: &Options,
    y_hat: Option<&[f64]>,
    w_hat: Option<&[f64]>,
) -> Result<Model, &'static str> {
    fit_with_stabilization(x, y, w, weights, clusters, options, y_hat, w_hat, true)
}
pub fn fit_with_stabilization(
    x: &[Vec<f64>],
    y: &[f64],
    w: &[f64],
    weights: Option<&[f64]>,
    clusters: &Clusters,
    options: &Options,
    y_hat: Option<&[f64]>,
    w_hat: Option<&[f64]>,
    stabilize: bool,
) -> Result<Model, &'static str> {
    let prune = match options.tree.honesty {
        Honesty::Disabled => true,
        Honesty::Enabled { prune, .. } => prune,
    };
    fit_with_nuisance_pruning(
        x, y, w, weights, clusters, options, y_hat, w_hat, stabilize, prune,
    )
}
/// R keeps honesty.prune.leaves for the nuisance forests even when honesty is
/// disabled for the causal forest. Preserve that otherwise independent option.
pub fn fit_with_nuisance_pruning(
    x: &[Vec<f64>],
    y: &[f64],
    w: &[f64],
    weights: Option<&[f64]>,
    clusters: &Clusters,
    options: &Options,
    y_hat: Option<&[f64]>,
    w_hat: Option<&[f64]>,
    stabilize: bool,
    nuisance_prune: bool,
) -> Result<Model, &'static str> {
    fit_internal(
        x,
        y,
        w,
        weights,
        clusters,
        options,
        y_hat,
        w_hat,
        stabilize,
        nuisance_prune,
        None,
        None,
    )
}

fn fit_internal(
    x: &[Vec<f64>],
    y: &[f64],
    w: &[f64],
    weights: Option<&[f64]>,
    clusters: &Clusters,
    options: &Options,
    y_hat: Option<&[f64]>,
    w_hat: Option<&[f64]>,
    stabilize: bool,
    nuisance_prune: bool,
    config: Option<&tuning::Config>,
    mut tuner: Option<&mut Tuner<'_>>,
) -> Result<Model, &'static str> {
    let n = y.len();
    if n == 0
        || x.is_empty()
        || w.len() != n
        || x.iter().any(|c| c.len() != n)
        || weights.is_some_and(|v| v.len() != n)
        || y.iter().chain(w).any(|v| !v.is_finite())
    {
        return Err("Invalid causal forest observations.");
    }
    let mut nuisance = options.clone();
    nuisance.trees = (options.trees / 4).max(50);
    nuisance.group_size = 1;
    nuisance.tree.min_node_size = 5;
    nuisance.tree.honesty = Honesty::Enabled {
        fraction: 0.5,
        prune: nuisance_prune,
    };
    let mut estimate = |target: &[f64],
                    given: Option<&[f64]>|
     -> Result<(Vec<f64>, Option<tuning::Output>), &'static str> {
        if let Some(given) = given {
            if (given.len() != 1 && given.len() != n) || given.iter().any(|v| !v.is_finite()) {
                return Err("Invalid supplied nuisance predictions.");
            }
            return Ok((
                if given.len() == 1 {
                    vec![given[0]; n]
                } else {
                    given.to_vec()
                },
                None,
            ));
        }
        let mut columns = x.to_vec();
        columns.push(target.to_vec());
        let weight = weights.map(|v| {
            columns.push(v.to_vec());
            columns.len() - 1
        });
        let tuned = config
            .map(|c| {
                let c = tuning::Config::regression_defaults(c.parameters.clone());
                if let Some(tuner) = tuner.as_deref_mut() {
                    return tuner(&columns, x.len(), None, weight, clusters, &nuisance, &c, stabilize);
                }
                tuning::tune(n, x.len(), &nuisance, &c.defaults(x.len()), &c, |o| {
                    tuning::regression_error(&columns, x.len(), weight, clusters, o)
                })
            })
            .transpose()?;
        let selected = tuned.as_ref().map_or(&nuisance, |t| &t.options);
        let f = regression::train(&columns, x.len(), weight, clusters, selected)?;
        let predictions = f
            .predict(x, true)?
            .into_iter()
            .map(|v| v.ok_or("Nuisance forest has an undefined OOB prediction."))
            .collect::<Result<Vec<_>, _>>()?;
        Ok((predictions, tuned))
    };
    let (outcome_hat, outcome_tuning) = estimate(y, y_hat)?;
    let (treatment_hat, treatment_tuning) = estimate(w, w_hat)?;
    let mut columns = x.to_vec();
    columns.push(y.iter().zip(&outcome_hat).map(|(a, b)| a - b).collect());
    columns.push(w.iter().zip(&treatment_hat).map(|(a, b)| a - b).collect());
    let weight = weights.map(|v| {
        columns.push(v.to_vec());
        columns.len() - 1
    });
    let data = Data {
        columns: &columns,
        outcome: x.len(),
        treatment: x.len() + 1,
        weight,
    };
    let tuned = config
        .map(|c| {
            let mut c = c.clone();
            if matches!(options.tree.honesty, Honesty::Disabled) {
                c.parameters.retain(|p| {
                    !matches!(
                        p,
                        tuning::Parameter::HonestyFraction | tuning::Parameter::HonestyPrune
                    )
                });
            }
            if let Some(tuner) = tuner.as_deref_mut() {
                return tuner(&columns, x.len(), Some(x.len() + 1), weight, clusters, options, &c, stabilize);
            }
            tuning::tune(n, x.len(), options, &c.defaults(x.len()), &c, |o| {
                tuning::causal_error(data, clusters, o, stabilize)
            })
        })
        .transpose()?;
    let selected = tuned.as_ref().map_or(options, |t| &t.options);
    let forest = forest::train_with_stabilization(data, clusters, selected, stabilize)?;
    Ok(Model {
        forest,
        outcome_hat,
        treatment_hat,
        tuning: tuned.map(|causal| Tuning {
            outcome: outcome_tuning,
            treatment: treatment_tuning,
            causal,
        }),
    })
}
