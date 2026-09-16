//! Numeric variance attribution kernels from DoWhy 0.14 influence.py.
//!
//! These accept fitted prediction functions; automatic mechanism and predictor selection
//! are separate from these calculations.

use super::{
    metrics::variance,
    shapley::{estimate, Execution, Method, ShapleyError},
};
use crate::{nprandom::Mt19937, numpy_reduce::numpy_mean};
use nalgebra::{DMatrix, DVector};
use std::num::NonZeroUsize;

#[derive(Clone, Copy)]
pub enum Batch {
    All,
    Rows(NonZeroUsize),
}

/// Signed Shapley contributions to target variance, not outcome mean differences.
pub struct VarianceContributions {
    pub values: Vec<f64>,
}

pub struct IntrinsicContribution {
    pub nodes: Vec<usize>,
    pub contribution: VarianceContributions,
}

pub struct ApproximateContribution {
    pub result: IntrinsicContribution,
    pub selection: super::selection::Selection,
}

/// Default numeric ICC: select and fit a predictor of the target from ancestor noise.
pub fn approximate_icc(model:&super::model::FittedModel,target:usize,training:NonZeroUsize,
    randomization:NonZeroUsize,baseline:NonZeroUsize,batch:Batch,method:Method,execution:Execution,
    rng:&mut Mt19937,cache:&mut Option<f64>)->Result<ApproximateContribution,ShapleyError> {
    let (model,mapping)=model.ancestor_model(target).map_err(|_|ShapleyError::OutputShape)?;
    let target=mapping.iter().position(|&n|n==target).ok_or(ShapleyError::OutputShape)?;
    let order=model.ancestor_order(target).map_err(|_|ShapleyError::OutputShape)?;
    let (data,noise)=model.sample_ancestors(target,training.get(),rng,cache).map_err(|_|ShapleyError::NonFinite)?;
    let input=DMatrix::from_fn(training.get(),order.len(),|r,c|noise[(r,order[c])]);
    let observed=data.column(target).into_owned();
    let selection=super::selection::select(&input,&observed,execution,rng).map_err(|_|ShapleyError::NonFinite)?;
    if matches!(execution,Execution::Serial) {*cache=None;}
    let predictor=selection.best().fit(&input,&observed,rng).map_err(|_|ShapleyError::NonFinite)?;
    let count=randomization.get().checked_add(baseline.get()).ok_or(ShapleyError::Capacity)?;
    let (_,noise)=model.sample_ancestors(target,count,rng,cache).map_err(|_|ShapleyError::NonFinite)?;
    let random_noise=DMatrix::from_fn(randomization.get(),order.len(),|r,c|noise[(r,order[c])]);
    let baseline_noise=DMatrix::from_fn(baseline.get(),order.len(),|r,c|noise[(r+randomization.get(),order[c])]);
    let contribution=variance_contributions(&random_noise,&baseline_noise,batch,method,execution,rng,
        |x|predictor.predict(x).map_err(|_|ShapleyError::NonFinite))?;
    Ok(ApproximateContribution {result:IntrinsicContribution {nodes:order.iter().map(|&n|mapping[n]).collect(),contribution},selection})
}

/// Generate the source-default parent sample before evaluating incoming arrows.
pub fn generated_arrow_strengths(model:&super::model::FittedModel,target:usize,conditional:NonZeroUsize,
    max_runs:usize,tolerance:f64,execution:Execution,rng:&mut Mt19937,cache:&mut Option<f64>)
    ->Result<Vec<f64>,ShapleyError> {
    let (model,mapping)=model.ancestor_model(target).map_err(|_|ShapleyError::OutputShape)?;
    let target=mapping.iter().position(|&n|n==target).ok_or(ShapleyError::OutputShape)?;
    let parents=model.graph().parents(target).ok_or(ShapleyError::OutputShape)?;
    if parents.is_empty() {return Err(ShapleyError::OutputShape);}
    let count=conditional.get().checked_mul(20).ok_or(ShapleyError::Capacity)?;
    let data=model.draw_samples(count,rng,cache).map_err(|_|ShapleyError::NonFinite)?;
    let inputs=DMatrix::from_fn(count,parents.len(),|r,c|data[(r,parents[c])]);
    let result=arrow_strengths(&model,target,&inputs,conditional,max_runs,tolerance,execution,rng)?;
    if matches!(execution,Execution::Serial) {*cache=None;}
    Ok(result)
}

/// Numeric `intrinsic_causal_influence(prediction_model="exact")`.
/// The approximate/default predictor is deliberately not substituted with this mode.
pub fn exact_icc(model:&super::model::FittedModel,target:usize,training:NonZeroUsize,
    randomization:NonZeroUsize,baseline:NonZeroUsize,batch:Batch,method:Method,execution:Execution,
    rng:&mut Mt19937,cache:&mut Option<f64>)->Result<IntrinsicContribution,ShapleyError> {
    let columns=model.graph().names().len();
    let (model,mapping)=model.ancestor_model(target).map_err(|_|ShapleyError::OutputShape)?;
    // v0.14 exact prediction uses the original graph with ancestor-only noise,
    // raising KeyError if unrelated nodes are present. Do not fabricate a result.
    if mapping.len()!=columns {return Err(ShapleyError::OutputShape);}
    let target=mapping.iter().position(|&n|n==target).ok_or(ShapleyError::OutputShape)?;
    // The upstream exact mode still makes the initial training draw.
    model.sample_ancestors(target,training.get(),rng,cache).map_err(|_|ShapleyError::NonFinite)?;
    let count=randomization.get().checked_add(baseline.get()).ok_or(ShapleyError::Capacity)?;
    let (_,noise)=model.sample_ancestors(target,count,rng,cache).map_err(|_|ShapleyError::NonFinite)?;
    let order=model.ancestor_order(target).map_err(|_|ShapleyError::OutputShape)?;
    let random_noise=DMatrix::from_fn(randomization.get(),order.len(),|r,c|noise[(r,order[c])]);
    let baseline_noise=DMatrix::from_fn(baseline.get(),order.len(),|r,c|noise[(r+randomization.get(),order[c])]);
    let contribution=variance_contributions(&random_noise,&baseline_noise,batch,method,execution,rng,|input| {
        let mut noise=DMatrix::zeros(input.nrows(),order.len());
        for (c,&node) in order.iter().enumerate() {noise.set_column(node,&input.column(c));}
        model.data_from_noise(&noise).map(|data|data.column(target).into_owned()).map_err(|_|ShapleyError::NonFinite)
    })?;
    Ok(IntrinsicContribution {nodes:order.iter().map(|&n|mapping[n]).collect(),contribution})
}

/// Numeric `arrow_strength_of_model`, one parent per reported strength.
pub fn arrow_strengths(model:&super::model::FittedModel,target:usize,parents:&DMatrix<f64>,
    conditional:NonZeroUsize,max_runs:usize,tolerance:f64,execution:Execution,rng:&mut Mt19937)
    ->Result<Vec<f64>,ShapleyError> {
    validate(parents)?;
    if model.graph().parents(target).map(|p|p.len())!=Some(parents.ncols()) {return Err(ShapleyError::OutputShape);}
    if !tolerance.is_finite() || tolerance<0.0 {return Err(ShapleyError::NonFinite);}
    let seeds:Vec<_>=(0..parents.ncols()).map(|_|rng.randint(i32::MAX as u64) as u32).collect();
    seeds.into_iter().enumerate().map(|(column,seed)| {
        let mut isolated=Mt19937::seeded(seed);
        let job=match execution {Execution::Serial=>{*rng=Mt19937::seeded(seed); &mut *rng},Execution::Isolated=>&mut isolated};
        direct_strength(parents,&[column],conditional,max_runs,tolerance,job,|x,rng|
            model.conditional_samples(target,x,rng).map_err(|_|ShapleyError::NonFinite))
    }).collect()
}

fn validate(samples: &DMatrix<f64>) -> Result<(), ShapleyError> {
    if samples.nrows() == 0 || samples.ncols() == 0 {
        return Err(ShapleyError::OutputShape);
    }
    if samples.iter().any(|v| !v.is_finite()) {
        return Err(ShapleyError::NonFinite);
    }
    Ok(())
}

fn predict_checked(
    samples: &DMatrix<f64>,
    predict: &mut impl FnMut(&DMatrix<f64>) -> Result<DVector<f64>, ShapleyError>,
) -> Result<DVector<f64>, ShapleyError> {
    let values = predict(samples)?;
    if values.len() != samples.nrows() {
        return Err(ShapleyError::OutputShape);
    }
    if values.iter().any(|v| !v.is_finite()) {
        return Err(ShapleyError::NonFinite);
    }
    Ok(values)
}

/// `_estimate_iccs` for a scalar numeric target and its default variance attribution.
pub fn variance_contributions(
    noise: &DMatrix<f64>,
    baseline: &DMatrix<f64>,
    batch: Batch,
    method: Method,
    execution: Execution,
    rng: &mut Mt19937,
    mut predict: impl FnMut(&DMatrix<f64>) -> Result<DVector<f64>, ShapleyError>,
) -> Result<VarianceContributions, ShapleyError> {
    validate(noise)?;
    validate(baseline)?;
    if noise.ncols() != baseline.ncols() {
        return Err(ShapleyError::OutputShape);
    }
    let target = predict_checked(baseline, &mut predict)?;
    let batch_size = match batch {
        Batch::All => baseline.nrows(),
        Batch::Rows(n) => n.get().min(baseline.nrows()),
    };
    let size = batch_size
        .checked_mul(noise.nrows())
        .ok_or(ShapleyError::Capacity)?;
    size.checked_mul(noise.ncols())
        .ok_or(ShapleyError::Capacity)?;
    let mut inputs = DMatrix::zeros(size, noise.ncols());
    let mut values = DVector::zeros(baseline.nrows());
    let result = estimate(
        NonZeroUsize::new(noise.ncols()).unwrap(),
        method,
        execution,
        rng,
        |subset, rng| {
            if subset.iter().all(|&v| v) {
                values.copy_from(&target);
            } else if subset.iter().all(|&v| !v) {
                values.fill(numpy_mean(predict_checked(noise, &mut predict)?.as_slice()));
            } else {
                // Preserve the joint permutation, even though averaging is invariant in exact arithmetic.
                let order = rng.permutation(noise.nrows());
                for offset in (0..baseline.nrows()).step_by(batch_size) {
                    let count = batch_size.min(baseline.nrows() - offset);
                    for c in 0..noise.ncols() {
                        for b in 0..count {
                            for r in 0..noise.nrows() {
                                inputs[(b * noise.nrows() + r, c)] = if subset[c] {
                                    baseline[(offset + b, c)]
                                } else {
                                    noise[(order[r], c)]
                                };
                            }
                        }
                    }
                    let predictions = if count == batch_size {
                        predict_checked(&inputs, &mut predict)?
                    } else {
                        predict_checked(
                            &inputs.rows(0, count * noise.nrows()).into_owned(),
                            &mut predict,
                        )?
                    };
                    for b in 0..count {
                        values[offset + b] = numpy_mean(
                            &predictions.as_slice()[b * noise.nrows()..(b + 1) * noise.nrows()],
                        );
                    }
                }
            }
            let value = variance(values.as_slice()).map_err(|_| ShapleyError::NonFinite)?;
            Ok(vec![value.max(0.0)])
        },
    )?;
    Ok(VarianceContributions {
        values: result.into_iter().next().ok_or(ShapleyError::OutputShape)?,
    })
}

/// `_estimate_direct_strength` for the numeric default: randomized minus conditional variance.
/// The source checks the zero-based run index against max_runs after evaluating that run.
pub fn direct_strength(
    samples: &DMatrix<f64>,
    subset: &[usize],
    conditional: NonZeroUsize,
    max_runs: usize,
    tolerance: f64,
    rng: &mut Mt19937,
    mut draw: impl FnMut(&DMatrix<f64>, &mut Mt19937) -> Result<DVector<f64>, ShapleyError>,
) -> Result<f64, ShapleyError> {
    validate(samples)?;
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(ShapleyError::NonFinite);
    }
    if subset.iter().any(|&c| c >= samples.ncols()) {
        return Err(ShapleyError::OutputShape);
    }
    let count = conditional.get().min(samples.nrows());
    let mut inputs = DMatrix::zeros(count, samples.ncols());
    let mut sum = 0.0;
    let mut average = 0.0;
    let mut converged = 0;
    for run in 0..samples.nrows() {
        for c in 0..samples.ncols() {
            inputs.column_mut(c).fill(samples[(run, c)]);
        }
        let order = rng.permutation(samples.nrows());
        let old = predict_checked(&inputs, &mut |x| draw(x, rng))?;
        for &c in subset {
            for r in 0..count {
                inputs[(r, c)] = samples[(order[r], c)];
            }
        }
        let new = predict_checked(&inputs, &mut |x| draw(x, rng))?;
        let previous = average;
        sum += variance(new.as_slice()).map_err(|_| ShapleyError::NonFinite)?
            - variance(old.as_slice()).map_err(|_| ShapleyError::NonFinite)?;
        average = sum / (run + 1) as f64;
        if run >= max_runs {
            break;
        }
        if run == 0 {
            continue;
        }
        let stable = if previous == 0.0 {
            average == 0.0
        } else {
            (1.0 - average / previous).abs() < tolerance
        };
        converged = if stable { converged + 1 } else { 0 };
        if converged >= 3 {
            break;
        }
    }
    if !average.is_finite() {
        return Err(ShapleyError::NonFinite);
    }
    Ok(average)
}
