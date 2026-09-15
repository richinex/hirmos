//! DoWhy v0.14 replicate execution, percentile bounds and subset refitting.

use super::{
    model::{FittedModel, Graph, ModelError},
    shapley::Execution,
};
use crate::{causal_effects::numpy_percentile, nprandom::Mt19937};
use nalgebra::DMatrix;
use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConfidenceLevel(f64);

impl ConfidenceLevel {
    pub fn new(value: f64) -> Option<Self> {
        (value.is_finite() && (0.5..=1.0).contains(&value)).then_some(Self(value))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SubsetFraction(f64);

impl SubsetFraction {
    pub fn new(value: f64) -> Option<Self> {
        (value.is_finite() && value > 0.0 && value <= 1.0).then_some(Self(value))
    }
}

#[derive(Debug, PartialEq)]
pub enum BootstrapError<E> {
    EmptyOutput,
    OutputShape,
    NonFinite,
    Evaluation(E),
    Summary(super::summary::SummaryError),
}

#[derive(Debug)]
pub enum RefitError<E> {
    EmptyTrainingSubset,
    Model(ModelError),
    Evaluation(E),
}

/// A nonempty, rectangular collection of finite estimates, in execution order.
#[derive(Clone, Debug)]
pub struct Replicates {
    values: DMatrix<f64>,
}

impl Replicates {
    pub fn values(&self) -> &DMatrix<f64> {
        &self.values
    }

    /// v0.14 uses [1-level, level], not the equal-tailed [(1-level)/2, ...].
    pub fn percentile_bounds(&self, level: ConfidenceLevel) -> Vec<[f64; 2]> {
        (0..self.values.ncols())
            .map(|column| {
                let values = self.values.column(column);
                [
                    numpy_percentile(values.as_slice(), 1.0 - level.0),
                    numpy_percentile(values.as_slice(), level.0),
                ]
            })
            .collect()
    }
}

/// Source summary, marginal bounds and the raw values used to calculate them.
pub struct ConfidenceEstimate {
    replicates: Replicates,
    summary: super::summary::GeometricMedian,
    bounds: Vec<[f64;2]>,
    level: ConfidenceLevel,
}

impl ConfidenceEstimate {
    pub fn replicates(&self)->&Replicates {&self.replicates}
    pub fn summary(&self)->&super::summary::GeometricMedian {&self.summary}
    pub fn bounds(&self)->&[[f64;2]] {&self.bounds}
    pub fn level(&self)->ConfidenceLevel {self.level}
}

pub fn confidence_intervals<F,E>(count:NonZeroUsize,level:ConfidenceLevel,execution:Execution,
    rng:&mut Mt19937,cache:&mut Option<f64>,function:F)->Result<ConfidenceEstimate,BootstrapError<E>>
where F:FnMut(&mut Mt19937,&mut Option<f64>)->Result<Vec<f64>,E> {
    let replicates=collect(count,execution,rng,cache,function)?;
    let summary=super::summary::geometric_median(replicates.values()).map_err(BootstrapError::Summary)?;
    let bounds=replicates.percentile_bounds(level);
    Ok(ConfidenceEstimate {replicates,summary,bounds,level})
}

/// Generate all job seeds before evaluating any replicate, as joblib does.
pub fn collect<F, E>(
    count: NonZeroUsize,
    execution: Execution,
    rng: &mut Mt19937,
    normal_cache: &mut Option<f64>,
    mut function: F,
) -> Result<Replicates, BootstrapError<E>>
where
    F: FnMut(&mut Mt19937, &mut Option<f64>) -> Result<Vec<f64>, E>,
{
    let seeds: Vec<_> = (0..count.get())
        .map(|_| rng.randint(i32::MAX as u64) as u32)
        .collect();
    let mut rows: Vec<Vec<f64>> = Vec::with_capacity(count.get());
    for seed in seeds {
        let values = match execution {
            Execution::Serial => {
                *rng = Mt19937::seeded(seed);
                *normal_cache = None;
                function(rng, normal_cache)
            }
            Execution::Isolated => function(&mut Mt19937::seeded(seed), &mut None),
        }
        .map_err(BootstrapError::Evaluation)?;
        if values.is_empty() {
            return Err(BootstrapError::EmptyOutput);
        }
        if rows.first().is_some_and(|row| row.len() != values.len()) {
            return Err(BootstrapError::OutputShape);
        }
        if values.iter().any(|v| !v.is_finite()) {
            return Err(BootstrapError::NonFinite);
        }
        rows.push(values);
    }
    Ok(Replicates {
        values: DMatrix::from_fn(rows.len(), rows[0].len(), |r, c| rows[r][c]),
    })
}

/// Refit assigned mechanisms on a fresh subset without replacement, then query.
pub fn fit_and_compute<F, T, E>(
    graph: &Graph,
    data: &DMatrix<f64>,
    fraction: SubsetFraction,
    rng: &mut Mt19937,
    normal_cache: &mut Option<f64>,
    function: F,
) -> Result<T, RefitError<E>>
where
    F: FnOnce(&FittedModel, &mut Mt19937, &mut Option<f64>) -> Result<T, E>,
{
    let count = (data.nrows() as f64 * fraction.0) as usize;
    if count == 0 {
        return Err(RefitError::EmptyTrainingSubset);
    }
    let order = rng.permutation(data.nrows());
    let subset = DMatrix::from_fn(count, data.ncols(), |r, c| data[(order[r], c)]);
    let model = FittedModel::fit(graph.clone(), &subset).map_err(RefitError::Model)?;
    function(&model, rng, normal_cache).map_err(RefitError::Evaluation)
}
