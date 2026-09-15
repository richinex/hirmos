//! EconML bootstrap inference for binary T-learners at fixed query rows.

use super::{fit_tlearner, TLearnerError};
use crate::{causal_effects::numpy_percentile, nprandom::Mt19937};
use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug)]
pub enum IntervalMethod {
    Percentile,
    Pivot,
    Normal,
}

#[derive(Clone, Copy, Debug)]
pub struct Confidence(f64);

impl Confidence {
    pub fn new(value: f64) -> Option<Self> {
        (value.is_finite() && value > 0.0 && value < 1.0).then_some(Self(value))
    }
}

pub struct Forest {
    pub trees: NonZeroUsize,
    pub min_leaf: NonZeroUsize,
    pub seed: u32,
}

pub struct Bootstrap {
    pub samples: NonZeroUsize,
    pub seed: u32,
    pub confidence: Confidence,
    pub method: IntervalMethod,
}

#[derive(Debug)]
pub enum Error {
    InvalidData,
    Fit(TLearnerError),
    Resample { index: usize, error: TLearnerError },
}

pub struct Estimate {
    pub effects: Vec<f64>,
    pub intervals: Vec<[f64; 2]>,
    pub standard_errors: Vec<f64>,
    pub replicates: Vec<Vec<f64>>,
    pub average: Average,
}

/// EconML's population summary uses a conservative standard-error bound.
pub struct Average {
    pub effect: f64,
    pub standard_error_bound: f64,
    pub interval: [f64; 2],
}

/// Draw the same row-major indices as np.random.choice(n, size=(samples, n)).
pub fn sample_indices(rows: NonZeroUsize, samples: NonZeroUsize, seed: u32) -> Vec<Vec<usize>> {
    let mut rng = Mt19937::seeded(seed);
    (0..samples.get())
        .map(|_| {
            (0..rows.get())
                .map(|_| rng.randint(rows.get() as u64) as usize)
                .collect()
        })
        .collect()
}

pub fn fit(
    x: &[Vec<f64>],
    y: &[f64],
    treatment: &[f64],
    query: &[Vec<f64>],
    forest: &Forest,
    bootstrap: &Bootstrap,
) -> Result<Estimate, Error> {
    let rows = NonZeroUsize::new(x.len()).ok_or(Error::InvalidData)?;
    let width = x[0].len();
    if width == 0
        || y.len() != x.len()
        || treatment.len() != x.len()
        || query.is_empty()
        || x.iter()
            .chain(query)
            .any(|row| row.len() != width || row.iter().any(|v| !v.is_finite()))
        || y.iter().chain(treatment).any(|v| !v.is_finite())
    {
        return Err(Error::InvalidData);
    }
    let fitted = fit_tlearner(
        x,
        y,
        treatment,
        forest.trees.get(),
        forest.min_leaf.get(),
        forest.seed,
    )
    .map_err(Error::Fit)?;
    let effects = fitted.effect(query);
    let mut rng = Mt19937::seeded(bootstrap.seed);
    // Retain predictions, not all resampled forests: each fit can be released immediately.
    let mut replicates = Vec::with_capacity(bootstrap.samples.get());
    for index in 0..bootstrap.samples.get() {
        // The forests use their own fixed seeds, so sampling can stream without changing the sequence.
        let sampled: Vec<_> = (0..rows.get())
            .map(|_| rng.randint(rows.get() as u64) as usize)
            .collect();
        let bx: Vec<_> = sampled.iter().map(|&i| x[i].clone()).collect();
        let by: Vec<_> = sampled.iter().map(|&i| y[i]).collect();
        let bt: Vec<_> = sampled.iter().map(|&i| treatment[i]).collect();
        let fitted = fit_tlearner(
            &bx,
            &by,
            &bt,
            forest.trees.get(),
            forest.min_leaf.get(),
            forest.seed,
        )
        .map_err(|error| Error::Resample { index, error })?;
        replicates.push(fitted.effect(query));
    }
    summarize(effects, replicates, bootstrap.confidence, bootstrap.method)
}

/// Summarize fixed-query effects, rejecting incomplete or nonfinite replicate records.
pub fn summarize(
    effects: Vec<f64>,
    replicates: Vec<Vec<f64>>,
    confidence: Confidence,
    method: IntervalMethod,
) -> Result<Estimate, Error> {
    if effects.is_empty()
        || replicates.is_empty()
        || effects.iter().any(|v| !v.is_finite())
        || replicates
            .iter()
            .any(|row| row.len() != effects.len() || row.iter().any(|v| !v.is_finite()))
    {
        return Err(Error::InvalidData);
    }
    let mut intervals = Vec::with_capacity(effects.len());
    let mut standard_errors = Vec::with_capacity(effects.len());
    let tail = (1.0 - confidence.0) / 2.0;
    for (column, &effect) in effects.iter().enumerate() {
        let values: Vec<_> = replicates.iter().map(|row| row[column]).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let se =
            (values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64).sqrt();
        let bounds = match method {
            IntervalMethod::Percentile => [
                numpy_percentile(&values, tail),
                numpy_percentile(&values, 1.0 - tail),
            ],
            IntervalMethod::Pivot => [
                2.0 * effect - numpy_percentile(&values, 1.0 - tail),
                2.0 * effect - numpy_percentile(&values, tail),
            ],
            IntervalMethod::Normal => [
                effect - spec_math::cephes64::ndtri(1.0 - tail) * se,
                effect - spec_math::cephes64::ndtri(tail) * se,
            ],
        };
        if !se.is_finite() || bounds.iter().any(|value| !value.is_finite()) {
            return Err(Error::InvalidData);
        }
        intervals.push(bounds);
        standard_errors.push(se);
    }
    let effect = effects.iter().sum::<f64>() / effects.len() as f64;
    let standard_error_bound = (standard_errors.iter().map(|se| se * se).sum::<f64>()
        / standard_errors.len() as f64)
        .sqrt();
    let average = Average {
        effect,
        standard_error_bound,
        interval: [
            effect + spec_math::cephes64::ndtri(tail) * standard_error_bound,
            effect + spec_math::cephes64::ndtri(1.0 - tail) * standard_error_bound,
        ],
    };
    if !effect.is_finite()
        || !standard_error_bound.is_finite()
        || average.interval.iter().any(|value| !value.is_finite())
    {
        return Err(Error::InvalidData);
    }
    Ok(Estimate {
        effects,
        intervals,
        standard_errors,
        replicates,
        average,
    })
}
