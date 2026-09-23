//! Application adapter for a pre-intervention Gaussian structural time series.
//! Reuses the preserved BOOM component samplers; not the legacy TFP impact model.
use crate::{defaults, fit::{Chain, Term}, prior::{Limit, Prior}, state::{Harmonics, Normal, Season, Variance}, Error};
use hirmos_causal_core::nprandom::NpRng;
use nalgebra::DMatrix;
use std::ops::ControlFlow;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trend { Level, Linear, Semilocal }

#[derive(Clone, Debug)]
pub enum Seasonality {
    None,
    Seasonal(Season),
    Harmonic(Harmonics),
}
impl Seasonality {
    pub fn seasonal(count: usize, duration: usize) -> Result<Self, Error> {
        count.checked_mul(duration).ok_or(Error::Shape)?;
        Ok(Self::Seasonal(Season::new(count, duration)?))
    }
    pub fn harmonic(period: f64, pairs: usize) -> Result<Self, Error> {
        // Exclude aliased frequencies and the degenerate sine at Nyquist.
        if pairs == 0 || !period.is_finite() || 2.0 * pairs as f64 >= period {
            return Err(Error::InvalidSeason);
        }
        Ok(Self::Harmonic(Harmonics::new(period, &(1..=pairs).map(|k| k as f64).collect::<Vec<_>>())?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Component { Trend, Seasonal, Predictor(usize) }

pub struct Contribution {
    pub component: Component,
    /// Time-major latent contributions, on the original response scale.
    pub paths: Vec<Vec<f64>>,
}
pub struct Fit {
    pub means: Vec<f64>,
    /// Time-major posterior predictive observations, including observation noise.
    pub paths: Vec<Vec<f64>>,
    pub contributions: Vec<Contribution>,
    pub inclusion_probabilities: Vec<f64>,
}

/// Validated, owned inputs. Post-intervention responses are deliberately absent.
pub struct Plan {
    training: Vec<Option<f64>>,
    predictors: DMatrix<f64>,
    mean: f64,
    sd: f64,
    trend: Trend,
    seasonality: Seasonality,
    draws: usize,
    warmup: usize,
    seed: u32,
}
impl Plan {
    pub fn new(training: &[f64], predictors: DMatrix<f64>, trend: Trend,
        seasonality: Seasonality, draws: usize, warmup: usize, seed: u32) -> Result<Self, Error> {
        if training.len() < 3 || predictors.nrows() <= training.len() || draws < 2 {
            return Err(Error::Shape);
        }
        draws.checked_add(warmup).ok_or(Error::Shape)?;
        predictors.nrows().checked_mul(draws).ok_or(Error::Shape)?;
        if training.iter().chain(predictors.iter()).any(|v| !v.is_finite()) { return Err(Error::NonFinite); }
        let scale = |values: &[f64]| -> Result<(f64, f64), Error> {
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let sd = ((values.iter().map(|v| (v - mean).powi(2)).sum::<f64>()) / (values.len()-1) as f64).sqrt();
            defaults::Scale::new(sd)?;
            if !mean.is_finite() { return Err(Error::NonFinite); }
            Ok((mean, sd))
        };
        let (mean, sd) = scale(training)?;
        let mut predictors = predictors;
        for j in 0..predictors.ncols() {
            let pre: Vec<_> = (0..training.len()).map(|i| predictors[(i,j)]).collect();
            let (center, spread) = scale(&pre)?;
            for i in 0..predictors.nrows() { predictors[(i,j)] = (predictors[(i,j)] - center) / spread; }
        }
        let training: Vec<_> = training.iter().map(|v| Some((v-mean)/sd)).collect();
        if training.iter().flatten().chain(predictors.iter()).any(|v| !v.is_finite()) { return Err(Error::NonFinite); }
        Ok(Self { training, predictors, mean, sd, trend, seasonality, draws, warmup, seed })
    }

    fn chain(&self) -> Result<Chain, Error> {
        let y = &self.training;
        let trend = match self.trend {
            Trend::Level => defaults::level(y)?.into_term()?,
            Trend::Linear => defaults::trend(y)?.into_term()?,
            Trend::Semilocal => defaults::semilocal(y)?.into_term(self.seed.wrapping_add(3))?,
        };
        let mut terms = vec![trend];
        match &self.seasonality {
            Seasonality::None => (),
            Seasonality::Seasonal(season) => terms.push(defaults::seasonal(y, *season)?.into_term()?),
            Seasonality::Harmonic(cycle) => {
                // Explicit harmonic prior on standardized data, recorded as part of
                // this adapter's v1 specification. Not an inferred R default.
                let prior = Prior::new(defaults::Scale::new(0.01)?, 0.01, Limit::At(defaults::Scale::new(1.0)?))?;
                terms.push(Term::harmonic(cycle.clone(), prior, Normal::new(0., Variance::new(1.)?)?));
            }
        }
        if self.predictors.ncols() == 0 {
            Chain::new(terms, defaults::gaussian(y)?, y.clone(), self.seed)
        } else {
            let x = self.predictors.rows(0, y.len()).into_owned();
            let slab = defaults::bsts_regression(&x, y)?;
            Chain::with_regression(terms, slab, x, y.clone(), self.seed)
        }
    }
}

pub fn fit(plan: &Plan, mut observe: impl FnMut(crate::run::Progress) -> ControlFlow<()>) -> Result<Fit, Error> {
    let mut chain = plan.chain()?;
    let n = plan.predictors.nrows();
    let pre = plan.training.len();
    let mut means = vec![0.; n];
    let mut inclusions = vec![0usize; plan.predictors.ncols()];
    let mut paths = vec![Vec::with_capacity(plan.draws); n];
    let mut labels = vec![Component::Trend];
    if !matches!(plan.seasonality, Seasonality::None) { labels.push(Component::Seasonal); }
    labels.extend((0..plan.predictors.ncols()).map(Component::Predictor));
    let mut contributions: Vec<_> = labels.into_iter().map(|component| Contribution {
        component, paths: vec![Vec::with_capacity(plan.draws); n],
    }).collect();
    let trend_dimension = match plan.trend { Trend::Level => 1, Trend::Linear => 2, Trend::Semilocal => 3 };
    let mut observation_rng = NpRng::seeded((plan.seed as u64) ^ 0xd1b54a32d192ed03);
    for iteration in 0..plan.warmup+plan.draws {
        let progress = if iteration < plan.warmup {
            crate::run::Progress::Warmup { completed: iteration, total: plan.warmup }
        } else { crate::run::Progress::Sampling { completed: iteration-plan.warmup, total: plan.draws } };
        if observe(progress).is_break() { return Err(Error::Cancelled); }
        let draw = chain.step()?;
        if iteration < plan.warmup { continue; }
        if let Some(regression) = &draw.regression {
            for (count, included) in inclusions.iter_mut().zip(&regression.included) {
                *count += usize::from(*included);
            }
        }
        let beta = draw.regression.as_ref().map(|r| r.coefficients.as_slice()).unwrap_or(&[]);
        let offsets: Vec<_> = (pre..n).map(|t| beta.iter().enumerate().map(|(j,b)| plan.predictors[(t,j)]*b).sum()).collect();
        let future = chain.forecast_snapshot()?.path(&offsets,
            ((plan.seed as u64) ^ 0x9e3779b97f4a7c15).wrapping_add((iteration-plan.warmup) as u64))?;
        for t in 0..n {
            let state = if t < pre { &draw.states[t] } else { &future.states[t-pre] };
            let seasonal = match &plan.seasonality {
                Seasonality::None => 0.,
                Seasonality::Seasonal(_) => state[trend_dimension],
                Seasonality::Harmonic(cycle) => (0..cycle.dimension()).step_by(2).map(|j| state[trend_dimension+j]).sum(),
            };
            let regression: f64 = beta.iter().enumerate().map(|(j,b)| plan.predictors[(t,j)]*b).sum();
            let mean = (state[0]+seasonal+regression)*plan.sd+plan.mean;
            let prediction = if t < pre { mean + draw.observation_variance.sqrt()*plan.sd*observation_rng.standard_normal() }
                else { future.observations[t-pre]*plan.sd+plan.mean };
            if !mean.is_finite() || !prediction.is_finite() { return Err(Error::NonFinite); }
            means[t] += mean/plan.draws as f64;
            paths[t].push(prediction);
            for contribution in &mut contributions {
                let value = match contribution.component {
                    Component::Trend => state[0]*plan.sd+plan.mean,
                    Component::Seasonal => seasonal*plan.sd,
                    Component::Predictor(j) => plan.predictors[(t,j)]*beta[j]*plan.sd,
                };
                if !value.is_finite() { return Err(Error::NonFinite); }
                contribution.paths[t].push(value);
            }
        }
    }
    let inclusion_probabilities = inclusions.into_iter().map(|n| n as f64 / plan.draws as f64).collect();
    Ok(Fit { means, paths, contributions, inclusion_probabilities })
}
