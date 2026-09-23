//! BOOM 0.9.16 NonzeroMeanAr1Sampler and Ar1Suf equations.
//! Copyright Google LLC 2018 and Steven L. Scott; LGPL-2.1-or-later.
//! Sources are preserved in vendor. RNGs and variance/truncated-normal draws
//! reuse the existing port; updates commit only after a complete sweep.
use crate::{
    defaults::Scale,
    prior::{Prior, Statistics},
    state::{Normal, Variance},
    Error,
};
use hirmos_causal_core::nprandom::{Mt19937, NpRng};

#[derive(Clone, Copy, Debug)]
pub enum Support {
    Real,
    Positive,
    Stationary,
    PositiveStationary,
}
impl Support {
    fn contains(self, phi: f64) -> bool {
        phi.is_finite()
            && match self {
                Self::Real => true,
                Self::Positive => phi >= 0.,
                Self::Stationary => phi >= -1. && phi <= 1.,
                Self::PositiveStationary => phi >= 0. && phi <= 1.,
            }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Parameters {
    mean: f64,
    phi: f64,
    sigma: Scale,
}
impl Parameters {
    pub fn new(mean: f64, phi: f64, sigma: Scale) -> Result<Self, Error> {
        if !mean.is_finite() || !phi.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(Self { mean, phi, sigma })
    }
    pub fn mean(self) -> f64 {
        self.mean
    }
    pub fn phi(self) -> f64 {
        self.phi
    }
    pub fn variance(self) -> Variance {
        self.sigma.variance()
    }
}

#[derive(Clone)]
pub struct Sampler {
    mean_prior: Normal,
    phi_prior: Normal,
    variance_prior: Prior,
    support: Support,
    parameters: Parameters,
    rng: NpRng,
    variance_rng: Mt19937,
}
impl Sampler {
    pub fn new(
        mean_prior: Normal,
        phi_prior: Normal,
        variance_prior: Prior,
        support: Support,
        initial: Parameters,
        seed: u32,
    ) -> Result<Self, Error> {
        if mean_prior.variance().value() <= 0. || phi_prior.variance().value() <= 0. {
            return Err(Error::InvalidScale);
        }
        if !support.contains(initial.phi) {
            return Err(Error::InvalidTruncation);
        }
        Ok(Self {
            mean_prior,
            phi_prior,
            variance_prior,
            support,
            parameters: initial,
            rng: NpRng::seeded(seed as u64),
            variance_rng: Mt19937::seeded(seed.wrapping_add(1)),
        })
    }
    pub fn parameters(&self) -> Parameters {
        self.parameters
    }
    pub fn step(&mut self, slopes: &[f64]) -> Result<(), Error> {
        let stats = Sufficient::new(slopes)?;
        let mut next = self.clone();
        let p = next.parameters;
        let q = p.variance().value();
        let a = 1. - p.phi;
        let precision = (1. + (stats.n - 1.) * a * a) / q + 1. / next.mean_prior.variance().value();
        let mean = ((a * (stats.current_sum() - p.phi * stats.lag_sum()) + stats.first) / q
            + next.mean_prior.mean() / next.mean_prior.variance().value())
            / precision;
        let mu = mean + (1. / precision).sqrt() * next.rng.standard_normal();
        let lag_squares =
            stats.lag_squares() - 2. * stats.lag_sum() * mu + (stats.n - 1.) * mu * mu;
        let cross =
            stats.cross - mu * (stats.current_sum() + stats.lag_sum()) + (stats.n - 1.) * mu * mu;
        let precision = lag_squares / q + 1. / next.phi_prior.variance().value();
        let mean =
            (cross / q + next.phi_prior.mean() / next.phi_prior.variance().value()) / precision;
        let sd = (1. / precision).sqrt();
        if !mu.is_finite() || !mean.is_finite() || !sd.is_finite() || sd <= 0. {
            return Err(Error::InvalidScale);
        }
        let phi = match next.support {
            Support::Stationary => crate::ar::truncated(mean, sd, -1., 1., &mut next.rng)?,
            Support::PositiveStationary => crate::ar::truncated(mean, sd, 0., 1., &mut next.rng)?,
            Support::Real => mean + sd * next.rng.standard_normal(),
            Support::Positive => {
                let mut accepted = None;
                for _ in 0..100000 {
                    let value = mean + sd * next.rng.standard_normal();
                    if value >= 0. {
                        accepted = Some(value);
                        break;
                    }
                }
                accepted.ok_or(Error::SamplingLimit)?
            }
        };
        let a = 1. - phi;
        let remaining_squares = stats.squares
            - stats.first.powi(2)
            - 2. * phi * stats.cross
            - 2. * a * mu * stats.current_sum()
            + phi * phi * stats.lag_squares()
            + 2. * phi * a * mu * stats.lag_sum()
            + (stats.n - 1.) * (mu * a).powi(2);
        let squares = (stats.first - mu).powi(2) + remaining_squares;
        let variance = next
            .variance_prior
            .conditional(Statistics::from_summary(slopes.len(), squares)?)?
            .draw(&mut next.variance_rng)?;
        next.parameters = Parameters::new(mu, phi, Scale::new(variance.value().sqrt())?)?;
        *self = next;
        Ok(())
    }
}

struct Sufficient {
    first: f64,
    last: f64,
    sum: f64,
    squares: f64,
    cross: f64,
    n: f64,
}
impl Sufficient {
    fn new(values: &[f64]) -> Result<Self, Error> {
        if values.is_empty() {
            return Err(Error::Empty);
        }
        if values.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        Ok(Self {
            first: values[0],
            last: values[values.len() - 1],
            sum: values.iter().sum(),
            squares: values.iter().map(|v| v * v).sum(),
            cross: values.windows(2).map(|v| v[0] * v[1]).sum(),
            n: values.len() as f64,
        })
    }
    fn current_sum(&self) -> f64 {
        self.sum - self.first
    }
    fn lag_sum(&self) -> f64 {
        self.sum - self.last
    }
    fn lag_squares(&self) -> f64 {
        self.squares - self.last * self.last
    }
}
