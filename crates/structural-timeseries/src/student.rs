//! Student trend innovation conditionals from BOOM 0.9.16
//! StudentLocalLinearTrend.cpp / StudentLocalLinearTrendPosteriorSampler.cpp.
//! Copyright Google LLC / Steven L. Scott; LGPL-2.1-or-later.
use crate::{
    defaults::Scale,
    prior::{Prior, Statistics},
    slice,
    state::Variance,
    Error,
};
use crate::{
    gaussian,
    state::{Component, Normal, System},
};
use hirmos_causal_core::nprandom::{Mt19937, NpRng};
use nalgebra::DVector;

#[derive(Clone, Copy, Debug)]
pub struct UniformTail {
    lower: f64,
    upper: f64,
}
impl UniformTail {
    pub fn new(lower: f64, upper: f64) -> Result<Self, Error> {
        if !lower.is_finite() || !upper.is_finite() || lower <= 0. || upper <= lower {
            return Err(Error::InvalidScale);
        }
        Ok(Self { lower, upper })
    }
    pub(crate) fn log_density(self, nu: f64) -> f64 {
        if nu >= self.lower && nu <= self.upper {
            -(self.upper - self.lower).ln()
        } else {
            f64::NEG_INFINITY
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Parameters {
    variance: Variance,
    nu: f64,
}
impl Parameters {
    pub fn new(scale: Scale, nu: f64) -> Result<Self, Error> {
        if !nu.is_finite() || nu <= 0. {
            return Err(Error::InvalidScale);
        }
        Ok(Self {
            variance: scale.variance(),
            nu,
        })
    }
    pub fn variance(self) -> Variance {
        self.variance
    }
    pub fn nu(self) -> f64 {
        self.nu
    }
}
#[derive(Clone, Debug)]
pub struct Observation {
    residuals: Vec<f64>,
    weights: Vec<f64>,
    scale: Statistics,
    sum_weights: f64,
    sum_log_weights: f64,
}
impl Observation {
    /// The scale sufficient statistic uses PREVIOUS weights. The tail update
    /// uses NEW weights. This order is deliberate in the preserved source.
    pub fn new(residuals: Vec<f64>, old_weights: &[f64], weights: Vec<f64>) -> Result<Self, Error> {
        if residuals.len() != weights.len() || residuals.len() != old_weights.len() {
            return Err(Error::Shape);
        }
        if residuals
            .iter()
            .chain(old_weights)
            .chain(&weights)
            .any(|v| !v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        if old_weights.iter().chain(&weights).any(|v| *v <= 0.) {
            return Err(Error::InvalidScale);
        }
        let squares = residuals
            .iter()
            .zip(old_weights)
            .map(|(r, w)| r * r * w)
            .sum();
        let scale = Statistics::from_summary(residuals.len(), squares)?;
        let sum_weights = weights.iter().sum::<f64>();
        let sum_log_weights = weights.iter().map(|v| v.ln()).sum::<f64>();
        if !sum_weights.is_finite() || !sum_log_weights.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(Self {
            residuals,
            weights,
            scale,
            sum_weights,
            sum_log_weights,
        })
    }
    pub fn scale_statistics(&self) -> Statistics {
        self.scale
    }
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    pub fn log_tail(&self, nu: f64, prior: UniformTail, parameters: Parameters) -> f64 {
        let p = prior.log_density(nu);
        if !p.is_finite() {
            return p;
        }
        let half = nu / 2.;
        if parameters.nu > 10. {
            // Collapsed Student density, rather than latent gamma density.
            self.residuals.iter().fold(p, |sum, r| {
                sum + spec_math::cephes64::lgam((nu + 1.) / 2.)
                    - spec_math::cephes64::lgam(half)
                    - 0.5 * (nu * std::f64::consts::PI * parameters.variance.value()).ln()
                    - (nu + 1.) / 2. * (r * r / (nu * parameters.variance.value())).ln_1p()
            })
        } else {
            p + self.weights.len() as f64 * (half * half.ln() - spec_math::cephes64::lgam(half))
                + (half - 1.) * self.sum_log_weights
                - half * self.sum_weights
        }
    }
}

#[derive(Clone)]
pub struct Innovation {
    parameters: Parameters,
    prior: Prior,
    tail: UniformTail,
    weights: Vec<f64>,
    gamma: Mt19937,
    slice: NpRng,
}
impl Innovation {
    pub fn new(
        parameters: Parameters,
        prior: Prior,
        tail: UniformTail,
        transitions: usize,
        seed: u32,
    ) -> Result<Self, Error> {
        if !tail.log_density(parameters.nu).is_finite() {
            return Err(Error::InvalidScale);
        }
        Ok(Self {
            parameters,
            prior,
            tail,
            weights: vec![1.; transitions],
            gamma: Mt19937::seeded(seed),
            slice: NpRng::seeded(seed as u64 + 1),
        })
    }
    pub fn parameters(&self) -> Parameters {
        self.parameters
    }
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    pub fn observe(&mut self, residuals: &[f64]) -> Result<Observation, Error> {
        if residuals.len() != self.weights.len() {
            return Err(Error::Shape);
        }
        if residuals.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let mut next = self.gamma.clone();
        let mut weights = Vec::with_capacity(residuals.len());
        for r in residuals {
            let shape = (1. + self.parameters.nu) / 2.;
            let rate = (self.parameters.nu + r * r / self.parameters.variance.value()) / 2.;
            if !shape.is_finite() || !rate.is_finite() || rate <= 0. {
                return Err(Error::NonFinite);
            }
            let weight = 1. / hirmos_causal_core::ucm::invgamma_rvs(shape, rate, &mut next);
            if !weight.is_finite() || weight <= 0. {
                return Err(Error::TailUnderflow);
            }
            weights.push(weight);
        }
        let observation = Observation::new(residuals.to_vec(), &self.weights, weights.clone())?;
        self.weights = weights;
        self.gamma = next;
        Ok(observation)
    }
    pub fn draw(&mut self, observation: &Observation) -> Result<(), Error> {
        if observation.weights.len() != self.weights.len() {
            return Err(Error::Shape);
        }
        let mut next = self.clone();
        next.parameters.variance = next
            .prior
            .conditional(observation.scale)?
            .draw(&mut next.gamma)?;
        if next.parameters.variance.value() <= 0. {
            return Err(Error::InvalidScale);
        }
        next.parameters.nu = slice::positive(next.parameters.nu, &mut next.slice, |nu| {
            observation.log_tail(nu, next.tail, next.parameters)
        })?;
        *self = next;
        Ok(())
    }
    pub fn step(&mut self, residuals: &[f64]) -> Result<(), Error> {
        let mut next = self.clone();
        let observation = next.observe(residuals)?;
        next.draw(&observation)?;
        *self = next;
        Ok(())
    }
}

/// Gaussian observation model with one Student local-linear trend.
/// Latent-weight observations are retained between sweeps, matching BOOM's
/// impute-state -> observe-state -> parameter-draw ordering.
#[derive(Clone)]
pub struct Trend {
    level: Innovation,
    slope: Innovation,
    initial_level: Normal,
    initial_slope: Normal,
    observed: [Observation; 2],
    observation_prior: Prior,
    observation_variance: Variance,
    y: Vec<Option<f64>>,
    states: Vec<DVector<f64>>,
    rng: NpRng,
    gamma: Mt19937,
}
pub struct TrendDraw {
    pub level: Parameters,
    pub slope: Parameters,
    pub observation_variance: Variance,
    pub states: Vec<Vec<f64>>,
    pub level_weights: Vec<f64>,
    pub slope_weights: Vec<f64>,
}
fn trend_system(
    level: &Innovation,
    slope: &Innovation,
    initial_level: Normal,
    initial_slope: Normal,
) -> Result<System, Error> {
    if level.weights.len() != slope.weights.len() {
        return Err(Error::Shape);
    }
    let mut noise = level
        .weights
        .iter()
        .zip(&slope.weights)
        .map(|(l, s)| {
            Ok(vec![
                Variance::new(level.parameters.variance.value() / l)?,
                Variance::new(slope.parameters.variance.value() / s)?,
            ])
        })
        .collect::<Result<Vec<_>, Error>>()?;
    // BOOM reserves one extra weight (one) after the final observed transition.
    noise.push(vec![level.parameters.variance, slope.parameters.variance]);
    System::new(vec![Component::Trend {
        level: level.parameters.variance,
        slope: slope.parameters.variance,
        initial_level,
        initial_slope,
    }])?
    .with_innovations(noise)
}
fn observe_trend(
    level: &mut Innovation,
    slope: &mut Innovation,
    states: &[DVector<f64>],
) -> Result<[Observation; 2], Error> {
    let levels: Vec<_> = states
        .windows(2)
        .map(|w| w[1][0] - (w[0][0] + w[0][1]))
        .collect();
    let slopes: Vec<_> = states.windows(2).map(|w| w[1][1] - w[0][1]).collect();
    Ok([level.observe(&levels)?, slope.observe(&slopes)?])
}
impl Trend {
    pub fn new(
        mut level: Innovation,
        mut slope: Innovation,
        initial_level: Normal,
        initial_slope: Normal,
        observation_prior: Prior,
        y: Vec<Option<f64>>,
        seed: u32,
    ) -> Result<Self, Error> {
        if y.len() < 2 || !y.iter().any(Option::is_some) {
            return Err(Error::Empty);
        }
        if level.weights.len() != y.len() - 1 || slope.weights.len() != y.len() - 1 {
            return Err(Error::Shape);
        }
        let mut rng = NpRng::seeded(seed as u64);
        let system = trend_system(&level, &slope, initial_level, initial_slope)?;
        let observation_variance = Variance::new(1.)?;
        let states = gaussian::draw_states(&system, observation_variance, &y, &mut rng)?;
        let observed = observe_trend(&mut level, &mut slope, &states)?;
        Ok(Self {
            level,
            slope,
            initial_level,
            initial_slope,
            observed,
            observation_prior,
            observation_variance,
            y,
            states,
            rng,
            gamma: Mt19937::seeded(seed.wrapping_add(1)),
        })
    }
    pub fn step(&mut self) -> Result<TrendDraw, Error> {
        let mut next = self.clone();
        let mut stats = Statistics::default();
        for (y, state) in next.y.iter().zip(&next.states) {
            if let Some(y) = y {
                stats.add(y - state[0])?;
            }
        }
        next.observation_variance = next
            .observation_prior
            .conditional(stats)?
            .draw(&mut next.gamma)?;
        next.level.draw(&next.observed[0])?;
        next.slope.draw(&next.observed[1])?;
        let system = trend_system(
            &next.level,
            &next.slope,
            next.initial_level,
            next.initial_slope,
        )?;
        next.states =
            gaussian::draw_states(&system, next.observation_variance, &next.y, &mut next.rng)?;
        next.observed = observe_trend(&mut next.level, &mut next.slope, &next.states)?;
        let draw = TrendDraw {
            level: next.level.parameters,
            slope: next.slope.parameters,
            observation_variance: next.observation_variance,
            states: next.states.iter().map(|v| v.as_slice().to_vec()).collect(),
            level_weights: next.level.weights.clone(),
            slope_weights: next.slope.weights.clone(),
        };
        *self = next;
        Ok(draw)
    }
    /// Marginal Student innovations are drawn afresh, not extrapolated latent weights.
    pub fn forecast(&self, horizon: usize, seed: u32) -> Result<Vec<f64>, Error> {
        let mut rng = NpRng::seeded(seed as u64);
        let mut gamma = Mt19937::seeded(seed.wrapping_add(1));
        let mut state = self.states.last().ok_or(Error::Empty)?.clone();
        let mut result = Vec::with_capacity(horizon);
        for _ in 0..horizon {
            let mut error = [0.; 2];
            for (j, p) in [self.level.parameters, self.slope.parameters]
                .iter()
                .enumerate()
            {
                let mixing =
                    hirmos_causal_core::ucm::invgamma_rvs(p.nu / 2., p.nu / 2., &mut gamma);
                if !mixing.is_finite() || mixing <= 0. {
                    return Err(Error::TailUnderflow);
                }
                error[j] = (p.variance.value() * mixing).sqrt() * rng.standard_normal();
            }
            state[0] += state[1] + error[0];
            state[1] += error[1];
            let y = state[0] + self.observation_variance.value().sqrt() * rng.standard_normal();
            if !y.is_finite() || state.iter().any(|v| !v.is_finite()) {
                return Err(Error::NonFinite);
            }
            result.push(y);
        }
        Ok(result)
    }
}
