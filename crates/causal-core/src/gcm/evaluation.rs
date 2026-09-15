//! Cross-validated numerical mechanism performance, without optional baseline models.

use super::{
    independence::kernel_test,
    metrics::{
        empirical_crps, marginal_kl, prediction_metrics, variance, MetricError, PredictionMetrics,
    },
    model::{FittedModel, Graph, ModelError},
    shapley::Execution,
};
use crate::{dml::kfold_shuffled, nprandom::Mt19937, numpy_reduce::numpy_mean};
use nalgebra::DMatrix;
use std::num::NonZeroUsize;

pub enum NodePerformance {
    Root {
        node: usize,
        kl_divergence: f64,
    },
    Conditional {
        node: usize,
        crps: f64,
        metrics: PredictionMetrics,
    },
}

#[derive(Debug)]
pub enum EvaluationError {
    Folds,
    Model(ModelError),
    Metric(MetricError),
    Independence(crate::kci::KciError),
    Falsification(super::falsification::FalsificationError),
}

/// All four diagnostics requested by the notebook's default evaluation.
pub struct Evaluation {
    pub mechanisms: Vec<NodePerformance>,
    pub invertibility: Vec<InvertibilityCheck>,
    pub overall_divergence: f64,
    pub graph: super::falsification::Falsification,
}

/// Evaluate in source order without restoring random states between phases.
pub fn evaluate(
    model: &FittedModel,
    data: &DMatrix<f64>,
    execution: Execution,
    rng: &mut Mt19937,
    cache: &mut Option<f64>,
) -> Result<Evaluation, EvaluationError> {
    let mechanisms = mechanism_performance(
        model.graph(),
        data,
        NonZeroUsize::new(5).unwrap(),
        execution,
        rng,
        cache,
    )?;
    let invertibility = invertibility(model, data, execution, rng, cache)?;
    let overall_divergence = overall_divergence(model, data, rng, cache)?;
    let graph = super::falsification::evaluate(
        model.graph(),
        data,
        NonZeroUsize::new(50).unwrap(),
        execution,
        rng,
        cache,
    )
    .map_err(EvaluationError::Falsification)?;
    Ok(Evaluation {
        mechanisms,
        invertibility,
        overall_divergence,
        graph,
    })
}

/// Five folds in the notebook; jobs are seeded in topological order.
pub fn mechanism_performance(
    graph: &Graph,
    data: &DMatrix<f64>,
    folds: NonZeroUsize,
    execution: Execution,
    rng: &mut Mt19937,
    cache: &mut Option<f64>,
) -> Result<Vec<NodePerformance>, EvaluationError> {
    if folds.get() < 2 || folds.get() > data.nrows() {
        return Err(EvaluationError::Folds);
    }
    graph.validate(data).map_err(EvaluationError::Model)?;
    let seeds: Vec<_> = graph
        .order()
        .iter()
        .map(|_| rng.randint(i32::MAX as u64) as u32)
        .collect();
    let distribution_rng = rng.clone();
    let distribution_cache = *cache;
    let mut result = Vec::new();
    for (&node, seed) in graph.order().iter().zip(seeds) {
        let performance = match execution {
            Execution::Serial => {
                *rng = Mt19937::seeded(seed);
                *cache = None;
                node_performance(graph, data, node, folds.get(), rng, cache, None)
            }
            Execution::Isolated => node_performance(
                graph,
                data,
                node,
                folds.get(),
                &mut Mt19937::seeded(seed),
                &mut None,
                Some((
                    &mut distribution_rng.clone(),
                    &mut distribution_cache.clone(),
                )),
            ),
        }?;
        result.push(performance);
    }
    Ok(result)
}

pub struct InvertibilityCheck {
    pub node: usize,
    pub p_value: f64,
    pub rejected: bool,
}

/// Five subsets capped at 2,000; merge within-node p-values by BH, then Bonferroni across nodes.
pub fn invertibility(
    model: &FittedModel,
    data: &DMatrix<f64>,
    execution: Execution,
    rng: &mut Mt19937,
    cache: &mut Option<f64>,
) -> Result<Vec<InvertibilityCheck>, EvaluationError> {
    model
        .graph()
        .validate(data)
        .map_err(EvaluationError::Model)?;
    if data.nrows() == 0 {
        return Err(EvaluationError::Metric(MetricError::Empty));
    }
    let count = data.nrows().min(2000);
    let runs = if data.nrows() <= 2000 { 1 } else { 5 };
    let mut checks = Vec::new();
    for node in 0..model.graph().names().len() {
        let parents = model.graph().parents(node).unwrap();
        if parents.is_empty() {
            continue;
        }
        let mut p_values = Vec::new();
        for _ in 0..runs {
            let order = rng.permutation(data.nrows());
            let subset = DMatrix::from_fn(count, data.ncols(), |r, c| data[(order[r], c)]);
            let noise = model
                .noise_from_data(&subset)
                .map_err(EvaluationError::Model)?;
            let conditioning =
                DMatrix::from_fn(count, parents.len(), |r, c| subset[(r, parents[c])]);
            let result = kernel_test(
                &noise.columns(node, 1).into_owned(),
                &conditioning,
                None,
                2000.try_into().unwrap(),
                execution,
                rng,
                cache,
            )
            .map_err(EvaluationError::Independence)?;
            p_values.push(result.p_value);
        }
        let merged = crate::pcmciplus::bh_values(&p_values)
            .into_iter()
            .fold(f64::INFINITY, f64::min);
        checks.push(InvertibilityCheck {
            node,
            p_value: merged,
            rejected: false,
        });
    }
    let count = checks.len() as f64;
    for check in &mut checks {
        check.rejected = check.p_value <= 0.05 / count;
        check.p_value = (check.p_value * count).min(1.0);
    }
    Ok(checks)
}

pub fn overall_divergence(
    model: &FittedModel,
    data: &DMatrix<f64>,
    rng: &mut Mt19937,
    cache: &mut Option<f64>,
) -> Result<f64, EvaluationError> {
    model
        .graph()
        .validate(data)
        .map_err(EvaluationError::Model)?;
    let samples = model
        .draw_samples(data.nrows(), rng, cache)
        .map_err(EvaluationError::Model)?;
    let divergences: Result<Vec<_>, _> = (0..data.ncols())
        .map(|c| marginal_kl(samples.column(c).as_slice(), data.column(c).as_slice()))
        .collect();
    Ok(numpy_mean(&divergences.map_err(EvaluationError::Metric)?))
}

fn node_performance(
    graph: &Graph,
    data: &DMatrix<f64>,
    node: usize,
    folds: usize,
    rng: &mut Mt19937,
    cache: &mut Option<f64>,
    mut root_state: Option<(&mut Mt19937, &mut Option<f64>)>,
) -> Result<NodePerformance, EvaluationError> {
    let parents = graph.parents(node).unwrap();
    let mut kl = Vec::new();
    let mut crps = Vec::new();
    let mut mse = Vec::new();
    let mut nmse = Vec::new();
    let mut r2 = Vec::new();
    for (train, test) in kfold_shuffled(data.nrows(), folds, rng) {
        let training = DMatrix::from_fn(train.len(), data.ncols(), |r, c| data[(train[r], c)]);
        let mechanism =
            FittedModel::fit_mechanism(graph, node, &training).map_err(EvaluationError::Model)?;
        let observed: Vec<_> = test.iter().map(|&r| data[(r, node)]).collect();
        if parents.is_empty() {
            let samples = match root_state.as_mut() {
                Some((rng, cache)) => mechanism.draw_noise(test.len(), rng, cache),
                None => mechanism.draw_noise(test.len(), rng, cache),
            };
            kl.push(marginal_kl(samples.as_slice(), &observed).map_err(EvaluationError::Metric)?);
            continue;
        }
        let parent_values = DMatrix::from_fn(test.len(), parents.len(), |r, c| {
            data[(test[r], parents[c])]
        });
        let standard_deviation = variance(&observed).map_err(EvaluationError::Metric)?.sqrt();
        let scale = if standard_deviation == 0.0 {
            1.0
        } else {
            standard_deviation
        };
        let mut scores = Vec::new();
        for (row, &outcome) in observed.iter().enumerate() {
            let repeated = DMatrix::from_fn(100, parents.len(), |_, c| parent_values[(row, c)]);
            let samples =
                (mechanism.predict(&repeated) + mechanism.draw_noise(100, rng, cache)) / scale;
            scores.push(
                empirical_crps(samples.as_slice(), outcome / scale)
                    .map_err(EvaluationError::Metric)?,
            );
        }
        crps.push(numpy_mean(&scores));
        let metrics = prediction_metrics(&observed, mechanism.predict(&parent_values).as_slice())
            .map_err(EvaluationError::Metric)?;
        mse.push(metrics.mse);
        nmse.push(metrics.nmse);
        r2.push(metrics.r2);
    }
    if parents.is_empty() {
        return Ok(NodePerformance::Root {
            node,
            kl_divergence: numpy_mean(&kl),
        });
    }
    Ok(NodePerformance::Conditional {
        node,
        crps: numpy_mean(&crps),
        metrics: PredictionMetrics {
            mse: numpy_mean(&mse),
            nmse: numpy_mean(&nmse),
            r2: numpy_mean(&r2),
        },
    })
}
