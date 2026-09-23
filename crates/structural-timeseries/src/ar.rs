//! BOOM 0.9.16 ArPosteriorSampler and the bsts 0.9.11 AR factory.
//! Copyright Google LLC and Steven L. Scott; LGPL-2.1-or-later.
//! Reference behavior and corrected stationary-rejection-v1 remain distinct.
use crate::{
    initial::Initial,
    prior::{Prior, Statistics},
    regression::{factor, solve},
    state::Variance,
    Error,
};
use hirmos_causal_core::{
    arma_regression::unconstrain_stationary_univariate,
    nprandom::{Mt19937, NpRng},
};
use nalgebra::{DMatrix, DVector};
use std::num::NonZeroUsize;

#[derive(Clone, Debug)]
pub struct Coefficients(Vec<f64>);
impl Coefficients {
    pub fn new(values: Vec<f64>) -> Result<Self, Error> {
        if values.is_empty() {
            return Err(Error::Empty);
        }
        if values.iter().any(|x| !x.is_finite()) {
            return Err(Error::NonFinite);
        }
        // Reuse the existing inverse reflection-coefficient recursion. Finite
        // unconstrained coordinates represent the open stationary region.
        if unconstrain_stationary_univariate(&values)
            .iter()
            .any(|x| !x.is_finite())
        {
            return Err(Error::NonStationary);
        }
        Ok(Self(values))
    }
    pub fn values(&self) -> &[f64] {
        &self.0
    }
    pub fn order(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proposals {
    JointThenCoordinate(NonZeroUsize),
    Coordinate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    Reference(Proposals),
    /// Exact rejection from the joint Gaussian conditional. Exhaustion fails
    /// transactionally; it never substitutes the defective coordinate kernel.
    StationaryRejectionV1,
}

/// Keep sampler identity with the coefficients it produced, not in a parallel
/// metadata vector that could drift out of alignment during result handling.
pub struct Draw {
    pub algorithm: Algorithm,
    pub coefficients: Coefficients,
}
impl Default for Proposals {
    fn default() -> Self {
        Self::JointThenCoordinate(NonZeroUsize::new(3).unwrap())
    }
}

/// The R factory starts at phi=0, variance=1 and N(0,I), irrespective of
/// AddAr's initial.state.prior and SdPrior.fixed/initial.value fields.
#[derive(Clone)]
pub struct Sampler {
    coefficients: Coefficients,
    variance: Variance,
    prior: Prior,
    algorithm: Algorithm,
    rng: NpRng,
    variance_rng: Mt19937,
}
impl Sampler {
    pub fn new(order: NonZeroUsize, prior: Prior, seed: u32) -> Self {
        Self {
            coefficients: Coefficients(vec![0.; order.get()]),
            variance: Variance::new(1.).unwrap(),
            prior,
            algorithm: Algorithm::Reference(Proposals::default()),
            rng: NpRng::seeded(seed as u64),
            variance_rng: Mt19937::seeded(seed.wrapping_add(1)),
        }
    }
    pub fn with_proposals(mut self, proposals: Proposals) -> Self {
        self.algorithm = Algorithm::Reference(proposals);
        self
    }
    pub fn with_algorithm(mut self, algorithm: Algorithm) -> Self {
        self.algorithm = algorithm;
        self
    }
    pub fn algorithm(&self) -> Algorithm {
        self.algorithm
    }
    pub fn snapshot(&self) -> Draw {
        Draw {
            algorithm: self.algorithm,
            coefficients: self.coefficients.clone(),
        }
    }
    pub fn with_coefficients(mut self, coefficients: Coefficients) -> Result<Self, Error> {
        if coefficients.order() != self.coefficients.order() {
            return Err(Error::Shape);
        }
        self.coefficients = coefficients;
        Ok(self)
    }
    pub fn coefficients(&self) -> &Coefficients {
        &self.coefficients
    }
    pub fn variance(&self) -> Variance {
        self.variance
    }
    pub fn step(&mut self, x: &DMatrix<f64>, y: &DVector<f64>) -> Result<(), Error> {
        let p = self.coefficients.order();
        if x.ncols() != p || x.nrows() != y.len() || y.is_empty() {
            return Err(Error::Shape);
        }
        if x.iter().chain(y.iter()).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let xtx = x.transpose() * x;
        let xty = x.transpose() * y;
        let chol = factor(&xtx)?;
        let mean = solve(&chol, DMatrix::from_column_slice(p, 1, xty.as_slice()))?
            .column(0)
            .into_owned();
        let covariance = solve(&chol, DMatrix::identity(p, p))? * self.variance.value();
        let covariance = (&covariance + covariance.transpose()) * 0.5;
        let conditional = Initial::new(mean, covariance)?;
        let mut next = self.clone();
        let attempts = match self.algorithm {
            Algorithm::Reference(Proposals::JointThenCoordinate(n)) => n.get(),
            Algorithm::Reference(Proposals::Coordinate) => 0,
            Algorithm::StationaryRejectionV1 => 100000,
        };
        let mut accepted = None;
        for _ in 0..attempts {
            if let Ok(phi) = Coefficients::new(conditional.draw(&mut next.rng).as_slice().to_vec())
            {
                accepted = Some(phi);
                break;
            }
        }
        next.coefficients = match accepted {
            Some(phi) => phi,
            None => match self.algorithm {
                Algorithm::Reference(_) => next.coordinate(&xtx, &xty)?,
                Algorithm::StationaryRejectionV1 => return Err(Error::SamplingLimit),
            },
        };
        let phi = DVector::from_column_slice(next.coefficients.values());
        // Preserve the reference's sufficient-statistic arithmetic.
        let squares = phi.dot(&(&xtx * &phi)) - 2. * phi.dot(&xty) + y.dot(y);
        next.variance = self
            .prior
            .conditional(Statistics::from_summary(y.len(), squares)?)?
            .draw(&mut next.variance_rng)?;
        *self = next;
        Ok(())
    }
    fn coordinate(
        &mut self,
        xtx: &DMatrix<f64>,
        xty: &DVector<f64>,
    ) -> Result<Coefficients, Error> {
        let mut phi = self.coefficients.values().to_vec();
        for i in 0..phi.len() {
            let initial = phi[i];
            let mut low = -1.;
            let mut high = 1.;
            let dot = phi
                .iter()
                .enumerate()
                .map(|(j, b)| b * xtx[(j, i)])
                .sum::<f64>();
            let precision = xtx[(i, i)];
            let mean = (xty[i] - (dot - phi[i] * precision)) / precision;
            // Deliberately lacks sigma^2, exactly as ArPosteriorSampler.cpp.
            let sd = (1. / precision).sqrt();
            let mut accepted = false;
            for _ in 0..10000 {
                let candidate = truncated(mean, sd, low, high, &mut self.rng)?;
                phi[i] = candidate;
                if Coefficients::new(phi.clone()).is_ok() {
                    accepted = true;
                    break;
                }
                if candidate > initial {
                    high = candidate;
                } else {
                    low = candidate;
                }
            }
            if !accepted {
                return Err(Error::SamplingLimit);
            }
        }
        Coefficients::new(phi)
    }
}

/// Normal/uniform rejection envelopes give the same truncated distribution
/// without a new CDF approximation. Bounded retries fail transactionally.
pub(crate) fn truncated(
    mean: f64,
    sd: f64,
    low: f64,
    high: f64,
    rng: &mut impl crate::random::NormalDraw,
) -> Result<f64, Error> {
    if !mean.is_finite() || !sd.is_finite() || sd <= 0. || low >= high {
        return Err(Error::InvalidTruncation);
    }
    let mode = mean.clamp(low, high);
    let centered = low < mean && mean < high && (high - low) / sd > 0.5;
    for _ in 0..100000 {
        if centered {
            let value = mean + sd * rng.standard_normal();
            if value >= low && value <= high {
                return Ok(value);
            }
        } else {
            let value = low + (high - low) * rng.next_f64();
            let log_ratio =
                -0.5 * ((value - mean) / sd).powi(2) + 0.5 * ((mode - mean) / sd).powi(2);
            if rng.next_f64().ln() <= log_ratio {
                return Ok(value);
            }
        }
    }
    Err(Error::SamplingLimit)
}
