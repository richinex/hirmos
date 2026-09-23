//! View-independent fitting lifecycle. Only completed runs expose samples.
use crate::{
    fit::{Chain, Draw},
    Error,
};
use std::{num::NonZeroUsize, ops::ControlFlow};

#[derive(Clone, Copy)]
pub struct Schedule {
    warmup: usize,
    draws: NonZeroUsize,
    horizon: Option<NonZeroUsize>,
}
impl Schedule {
    pub fn new(
        warmup: usize,
        draws: NonZeroUsize,
        horizon: Option<NonZeroUsize>,
    ) -> Result<Self, Error> {
        warmup.checked_add(draws.get()).ok_or(Error::Shape)?;
        if let Some(horizon) = horizon {
            horizon.get().checked_mul(draws.get()).ok_or(Error::Shape)?;
        }
        Ok(Self {
            warmup,
            draws,
            horizon,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Progress {
    Warmup { completed: usize, total: usize },
    Sampling { completed: usize, total: usize },
}

pub struct Samples {
    draws: Vec<Draw>,
    forecasts: Option<Vec<Vec<f64>>>,
}
impl Samples {
    pub fn draws(&self) -> &[Draw] {
        &self.draws
    }
    pub fn forecasts(&self) -> Option<&[Vec<f64>]> {
        self.forecasts.as_deref()
    }
    pub fn forecast_summary(&self, coverage: f64) -> Result<Option<Vec<Summary>>, Error> {
        if !coverage.is_finite() || coverage <= 0.0 || coverage >= 1.0 {
            return Err(Error::InvalidProbability);
        }
        match &self.forecasts {
            None => Ok(None),
            Some(paths) => (0..paths[0].len())
                .map(|t| {
                    let values: Vec<f64> = paths.iter().map(|row| row[t]).collect();
                    summarize(&values, coverage)
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Some),
        }
    }
}

pub struct Summary {
    pub mean: f64,
    pub median: f64,
    pub lower: f64,
    pub upper: f64,
}
fn summarize(values: &[f64], coverage: f64) -> Result<Summary, Error> {
    if values.is_empty() {
        return Err(Error::Empty);
    }
    if values.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    if !mean.is_finite() {
        return Err(Error::NonFinite);
    }
    let quantile = hirmos_causal_core::negbin_nuts::quantile;
    Ok(Summary {
        mean,
        median: quantile(values, 0.5),
        lower: quantile(values, (1.0 - coverage) / 2.0),
        upper: quantile(values, (1.0 + coverage) / 2.0),
    })
}

/// Cancellation discards this owned run, including warmup and partial samples.
/// The observer belongs to the job owner, not a mounted UI component.
pub fn fit(
    chain: Chain,
    schedule: Schedule,
    prediction_seed: u64,
    observe: impl FnMut(Progress) -> ControlFlow<()>,
) -> Result<Samples, Error> {
    let prediction = match schedule.horizon {
        None => Prediction::None,
        Some(horizon) => {
            chain.forecast(horizon.get(), prediction_seed)?;
            Prediction::State(horizon)
        }
    };
    fit_inner(chain, schedule, prediction_seed, prediction, observe)
}

pub fn fit_with_predictors(
    chain: Chain,
    schedule: Schedule,
    prediction_seed: u64,
    predictors: nalgebra::DMatrix<f64>,
    observe: impl FnMut(Progress) -> ControlFlow<()>,
) -> Result<Samples, Error> {
    if schedule.horizon.map(NonZeroUsize::get) != Some(predictors.nrows()) {
        return Err(Error::Shape);
    }
    chain.forecast_with_predictors(&predictors, prediction_seed)?;
    fit_inner(
        chain,
        schedule,
        prediction_seed,
        Prediction::Regression(predictors),
        observe,
    )
}

enum Prediction {
    None,
    State(NonZeroUsize),
    Regression(nalgebra::DMatrix<f64>),
}

fn fit_inner(
    mut chain: Chain,
    schedule: Schedule,
    prediction_seed: u64,
    prediction: Prediction,
    mut observe: impl FnMut(Progress) -> ControlFlow<()>,
) -> Result<Samples, Error> {
    let mut draws = Vec::with_capacity(schedule.draws.get());
    let mut forecasts = schedule
        .horizon
        .map(|_| Vec::with_capacity(schedule.draws.get()));
    for i in 0..schedule.warmup {
        if observe(Progress::Warmup {
            completed: i,
            total: schedule.warmup,
        })
        .is_break()
        {
            return Err(Error::Cancelled);
        }
        chain.step()?;
    }
    for i in 0..schedule.draws.get() {
        if observe(Progress::Sampling {
            completed: i,
            total: schedule.draws.get(),
        })
        .is_break()
        {
            return Err(Error::Cancelled);
        }
        let draw = chain.step()?;
        if let Some(paths) = &mut forecasts {
            let seed = prediction_seed.wrapping_add(i as u64);
            match &prediction {
                Prediction::None => unreachable!("forecast storage follows the validated schedule"),
                Prediction::State(horizon) => paths.push(chain.forecast(horizon.get(), seed)?),
                Prediction::Regression(x) => paths.push(chain.forecast_with_predictors(x, seed)?),
            }
        }
        draws.push(draw);
    }
    Ok(Samples { draws, forecasts })
}
