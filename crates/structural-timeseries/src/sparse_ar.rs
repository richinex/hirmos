//! Fixed-variance spike/slab kernel used by BOOM's ArSpikeSlabSampler.
//! Source: BOOM 0.9.16 SpikeSlabSampler.cpp, LGPL-2.1-or-later,
//! copyright Google LLC and Steven L. Scott. This is not BregVsSampler:
//! the slab is unscaled and the variance is conditioned on, not integrated out.
//! Stationarity is deliberately absent from the reference inclusion weights.
use crate::{
    defaults::Scale,
    regression::{factor, solve, FlipSweep, Inclusion, Slab},
    Error,
};
use hirmos_causal_core::nprandom::NpRng;
use nalgebra::{DMatrix, DVector};
mod sampler;
pub use sampler::{Coefficients, Draw, Sampler, Support, UpdatePath};

/// Parameters at the R SpikeSlabArPrior adapter boundary. Truncation and
/// model-selection policy remain explicit sampler settings, not prior fields.
pub struct RPrior {
    pub mean: Vec<f64>,
    pub sd: Vec<Scale>,
    pub inclusion: Vec<Inclusion>,
    pub response_sd: Scale,
    pub expected_r2: f64,
    pub df: f64,
    pub sigma_upper: f64,
}
impl RPrior {
    pub fn build(self) -> Result<UnscaledSlab, Error> {
        use crate::prior::{Limit, Prior};
        if !self.expected_r2.is_finite() || self.expected_r2 <= 0.0 || self.expected_r2 >= 1.0 {
            return Err(Error::InvalidProbability);
        }
        if !self.df.is_finite() || self.df < 0.0 || self.sigma_upper.is_nan() {
            return Err(Error::InvalidScale);
        }
        let limit = if self.sigma_upper <= 0.0 || self.sigma_upper == f64::INFINITY {
            Limit::Unbounded
        } else {
            Limit::At(Scale::new(self.sigma_upper)?)
        };
        let guess = Scale::new((1.0 - self.expected_r2).sqrt() * self.response_sd.value())?;
        let residual = if self.df == 0.0 {
            Prior::scale_invariant(limit)
        } else {
            Prior::new(guess, self.df, limit)?
        };
        // Reuse Gaussian dimension/finite/SPD validation and the same LAPACK.
        let precision = DMatrix::from_diagonal(&DVector::from_iterator(
            self.sd.len(),
            self.sd.iter().map(|s| 1.0 / s.value().powi(2)),
        ));
        let slab = Slab::with_prior(
            DVector::from_vec(self.mean),
            precision,
            self.inclusion,
            residual,
        )?;
        Ok(UnscaledSlab::new(slab))
    }
}

/// Separate interpretation of validated Gaussian prior parameters. Sharing
/// parameter validation does not share the conjugate regression likelihood.
#[derive(Clone)]
pub struct UnscaledSlab(Slab);
impl UnscaledSlab {
    pub fn new(parameters: Slab) -> Self {
        Self(parameters)
    }
    pub fn parameters(&self) -> &Slab {
        &self.0
    }

    /// bsts::SpikeSlabArPrior defaults for an explicitly supplied response SD.
    /// AddAutoAr's data-derived SD warning/fallback belongs to its adapter.
    pub fn r_default(order: std::num::NonZeroUsize, response_sd: Scale) -> Result<Self, Error> {
        let n = order.get();
        let mut probabilities = Vec::with_capacity(n);
        let mut precision = DMatrix::zeros(n, n);
        for j in 0..n {
            let discount = 0.8_f64.powf(j as f64);
            probabilities.push(Inclusion::new(0.8 * discount)?);
            precision[(j, j)] = 1.0 / (0.5 * discount).powi(2);
        }
        Ok(Self(Slab::new(
            DVector::zeros(n),
            precision,
            probabilities,
            Scale::new(response_sd.value() * 0.5_f64.sqrt())?,
            1.0,
        )?))
    }
}

/// A data snapshot, so all candidate models use the same sufficient statistics.
#[derive(Clone)]
pub struct Kernel {
    xtx: DMatrix<f64>,
    xty: DVector<f64>,
    prior: UnscaledSlab,
}

pub struct Conditional {
    mean: DVector<f64>,
    precision: DMatrix<f64>,
    factor: DMatrix<f64>,
    indices: Vec<usize>,
    dimension: usize,
    log_weight: f64,
}
impl Conditional {
    pub fn mean(&self) -> &DVector<f64> {
        &self.mean
    }
    pub fn precision(&self) -> &DMatrix<f64> {
        &self.precision
    }
    pub fn log_weight(&self) -> f64 {
        self.log_weight
    }
    pub fn draw(&self, rng: &mut NpRng) -> Result<Vec<f64>, Error> {
        if !self.log_weight.is_finite() {
            return Err(Error::InvalidProbability);
        }
        // Solve L' z = epsilon using the already factored precision. No second
        // inverse or covariance factorization, and no diagonal jitter.
        let n = self.indices.len();
        let mut z: Vec<_> = (0..n).map(|_| rng.standard_normal()).collect();
        for i in (0..n).rev() {
            for j in i + 1..n {
                z[i] -= self.factor[(j, i)] * z[j];
            }
            z[i] /= self.factor[(i, i)];
        }
        let mut coefficients = vec![0.0; self.dimension];
        for (i, index) in self.indices.iter().enumerate() {
            coefficients[*index] = self.mean[i] + z[i];
        }
        if coefficients.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        Ok(coefficients)
    }
}

#[derive(Clone, Copy)]
pub enum Selection {
    Fixed,
    Sweep(FlipSweep),
}

impl Kernel {
    /// The same BOOM SpikeSlabSampler is used by Student observation models.
    /// Keep the weighted accumulation, rather than recomputing sqrt(W)X.
    pub fn from_weighted(
        stats: &crate::weighted::Statistics,
        prior: UnscaledSlab,
    ) -> Result<Self, Error> {
        if stats.rows() == 0 || stats.xty().len() != prior.0.mean().len() {
            return Err(Error::Shape);
        }
        Ok(Self {
            xtx: stats.xtx().clone(),
            xty: stats.xty().clone(),
            prior,
        })
    }
    pub fn new(x: &DMatrix<f64>, y: &DVector<f64>, prior: UnscaledSlab) -> Result<Self, Error> {
        if x.nrows() == 0 || x.nrows() != y.len() || x.ncols() != prior.0.mean().len() {
            return Err(Error::Shape);
        }
        if x.iter().chain(y.iter()).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let xtx = x.transpose() * x;
        let xty = x.transpose() * y;
        if xtx.iter().chain(xty.iter()).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        Ok(Self { xtx, xty, prior })
    }

    pub fn conditional(&self, included: &[bool], variance: f64) -> Result<Conditional, Error> {
        if included.len() != self.xty.len() {
            return Err(Error::Shape);
        }
        if !variance.is_finite() || variance <= 0.0 {
            return Err(Error::InvalidScale);
        }
        let prior = &self.prior.0;
        let log_prior: f64 = prior
            .inclusion_probabilities()
            .iter()
            .zip(included)
            .map(|(p, yes)| if *yes { p.ln() } else { (-p).ln_1p() })
            .sum();
        let indices: Vec<_> = included
            .iter()
            .enumerate()
            .filter_map(|(i, yes)| yes.then_some(i))
            .collect();
        let n = indices.len();
        let mu = DVector::from_fn(n, |i, _| prior.mean()[indices[i]]);
        let base = DMatrix::from_fn(n, n, |i, j| prior.precision()[(indices[i], indices[j])]);
        let precision =
            &base + DMatrix::from_fn(n, n, |i, j| self.xtx[(indices[i], indices[j])] / variance);
        let rhs = &base * &mu + DVector::from_fn(n, |i, _| self.xty[indices[i]] / variance);
        let chol = factor(&precision)?;
        let mean = solve(&chol, DMatrix::from_column_slice(n, 1, rhs.as_slice()))?
            .column(0)
            .into_owned();
        let logdet =
            |chol: &DMatrix<f64>| 2.0 * chol.diagonal().iter().map(|v| v.ln()).sum::<f64>();
        let log_weight = log_prior
            + 0.5
                * (logdet(&factor(&base)?) - logdet(&chol) - mu.dot(&(&base * &mu))
                    + rhs.dot(&mean));
        if mean.iter().chain(precision.iter()).any(|v| !v.is_finite())
            || (log_prior.is_finite() && !log_weight.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(Conditional {
            mean,
            precision,
            factor: chol,
            indices,
            dimension: included.len(),
            log_weight,
        })
    }

    /// Commit the mask and RNG together only after a complete successful sweep.
    pub fn select(
        &self,
        included: &mut Vec<bool>,
        variance: f64,
        selection: Selection,
        rng: &mut NpRng,
    ) -> Result<(), Error> {
        let mut current = self.conditional(included, variance)?;
        let sweep = match selection {
            Selection::Fixed => {
                return if current.log_weight.is_finite() {
                    Ok(())
                } else {
                    Err(Error::InvalidProbability)
                }
            }
            Selection::Sweep(sweep) => sweep,
        };
        let mut next = included.clone();
        if !current.log_weight.is_finite() {
            for (yes, p) in next.iter_mut().zip(self.prior.0.inclusion_probabilities()) {
                if p == 0.0 {
                    *yes = false;
                }
                if p == 1.0 {
                    *yes = true;
                }
            }
            current = self.conditional(&next, variance)?;
        }
        if !current.log_weight.is_finite() {
            return Err(Error::InvalidProbability);
        }
        let mut next_rng = rng.clone();
        let mut order: Vec<_> = (0..next.len()).collect();
        next_rng.shuffle(&mut order);
        let count = match sweep {
            FlipSweep::All => order.len(),
            FlipSweep::AtMost(n) => n.get().min(order.len()),
        };
        for index in order.into_iter().take(count) {
            next[index] = !next[index];
            let candidate = self.conditional(&next, variance)?;
            if next_rng.next_f64().ln() <= candidate.log_weight - current.log_weight {
                current = candidate;
            } else {
                next[index] = !next[index];
            }
        }
        *included = next;
        *rng = next_rng;
        Ok(())
    }
}
