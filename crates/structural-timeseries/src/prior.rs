//! Gaussian variance conditionals from BOOM GenericGaussianVarianceSampler.
//! Copyright Google LLC and Steven L. Scott; LGPL-2.1-or-later.
//! Sampling reuses the project's inverse-gamma machinery, not BOOM's RNG.
use crate::{defaults::Scale, state::Variance, Error};
use hirmos_causal_core::nprandom::Mt19937;

#[derive(Clone, Copy, Debug)]
pub enum Limit {
    Unbounded,
    At(Scale),
    Zero,
}

#[derive(Clone, Copy, Debug)]
pub struct Prior {
    evidence: Evidence,
    limit: Limit,
}
#[derive(Clone, Copy, Debug)]
enum Evidence {
    Proper { guess: Scale, df: f64 },
    ScaleInvariant,
}
impl Prior {
    /// BOOM ChisqModel(0, sigma) contributes zero shape and rate. It is
    /// improper until data supply a positive count and sum of squares.
    pub fn scale_invariant(limit: Limit) -> Self {
        Self {
            evidence: Evidence::ScaleInvariant,
            limit,
        }
    }
    pub(crate) fn with_ceiling(mut self, ceiling: Scale) -> Self {
        self.limit = Limit::At(ceiling);
        self
    }
    pub fn new(guess: Scale, df: f64, limit: Limit) -> Result<Self, Error> {
        if !df.is_finite() || df <= 0.0 {
            return Err(Error::InvalidScale);
        }
        let mass = df * guess.variance().value();
        if !mass.is_finite() || mass <= 0.0 {
            return Err(Error::InvalidScale);
        }
        Ok(Self {
            evidence: Evidence::Proper { guess, df },
            limit,
        })
    }
    pub fn conditional(self, stats: Statistics) -> Result<Conditional, Error> {
        let (df, mass) = match self.evidence {
            Evidence::Proper { guess, df } => (df, df * guess.variance().value()),
            Evidence::ScaleInvariant => (0.0, 0.0),
        };
        let shape = (df + stats.count as f64) / 2.0;
        let scale = (mass + stats.squares) / 2.0;
        if !shape.is_finite() || !scale.is_finite() {
            return Err(Error::NonFinite);
        }
        if shape <= 0.0 || scale <= 0.0 {
            return Err(Error::ImproperConditional);
        }
        Ok(Conditional {
            shape,
            scale,
            limit: self.limit,
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Statistics {
    count: usize,
    squares: f64,
}
impl Statistics {
    pub fn from_summary(count: usize, squares: f64) -> Result<Self, Error> {
        if !squares.is_finite() || squares < 0.0 {
            return Err(Error::InvalidScale);
        }
        Ok(Self { count, squares })
    }
    pub fn add(&mut self, residual: f64) -> Result<(), Error> {
        let squares = self.squares + residual * residual;
        if !residual.is_finite() || !squares.is_finite() {
            return Err(Error::NonFinite);
        }
        self.count = self.count.checked_add(1).ok_or(Error::Shape)?;
        self.squares = squares;
        Ok(())
    }
    pub fn count(self) -> usize {
        self.count
    }
    pub fn sum_squares(self) -> f64 {
        self.squares
    }
}

pub struct Conditional {
    shape: f64,
    scale: f64,
    limit: Limit,
}
impl Conditional {
    pub fn shape(&self) -> f64 {
        self.shape
    }
    pub fn scale(&self) -> f64 {
        self.scale
    }
    /// Inverse CDF of the variance distribution, including genuine truncation.
    pub fn quantile(&self, probability: f64) -> Result<Variance, Error> {
        if !probability.is_finite() || probability <= 0.0 || probability >= 1.0 {
            return Err(Error::InvalidProbability);
        }
        let mass = match self.limit {
            Limit::Zero => return Variance::new(0.0),
            Limit::Unbounded => 1.0,
            Limit::At(sd) => {
                spec_math::cephes64::igamc(self.shape, self.scale / sd.variance().value())
            }
        };
        let probability = probability * mass;
        if probability <= 0.0 || !probability.is_finite() {
            return Err(Error::TailUnderflow);
        }
        let precision = spec_math::cephes64::igamci(self.shape, probability);
        let value = self.scale / precision;
        if !value.is_finite() || value <= 0.0 {
            return Err(Error::TailUnderflow);
        }
        Variance::new(value)
    }
    pub fn draw(&self, rng: &mut Mt19937) -> Result<Variance, Error> {
        if matches!(self.limit, Limit::Unbounded) {
            let value = hirmos_causal_core::ucm::invgamma_rvs(self.shape, self.scale, rng);
            if !value.is_finite() || value <= 0.0 {
                return Err(Error::TailUnderflow);
            }
            return Variance::new(value);
        }
        // MT produces [0,1). Exclude zero, which is not a finite IG quantile.
        let probability = loop {
            let p = rng.next_f64();
            if p > 0.0 {
                break p;
            }
        };
        match self.quantile(probability) {
            Err(Error::TailUnderflow) => self.draw_tail(rng),
            result => result,
        }
    }
    /// Exponential rejection envelope for gamma precision above its mode.
    /// For shape > 1 use the log-density tangent at the cutoff, as in BOOM's
    /// adaptive rejection approach. For shape <= 1, the gamma power factor
    /// decreases, so an exponential of rate one bounds the remaining tail.
    fn draw_tail(&self, rng: &mut Mt19937) -> Result<Variance, Error> {
        let Limit::At(sd) = self.limit else {
            return Err(Error::TailUnderflow);
        };
        let cut = self.scale / sd.variance().value();
        let power = self.shape - 1.0;
        if !cut.is_finite() || cut <= power.max(0.0) {
            return Err(Error::TailUnderflow);
        }
        let rate = if power > 0.0 { 1.0 - power / cut } else { 1.0 };
        for _ in 0..10000 {
            let u = rng.next_f64();
            let v = rng.next_f64();
            if u == 0.0 || v == 0.0 {
                continue;
            }
            let excess = -u.ln() / rate;
            let ratio = excess / cut;
            let log_accept = if power > 0.0 {
                power * (ratio.ln_1p() - ratio)
            } else {
                power * ratio.ln_1p()
            };
            if v.ln() <= log_accept {
                let value = self.scale / (cut + excess);
                if value > 0.0 && value.is_finite() && value <= sd.variance().value() {
                    return Variance::new(value);
                }
                return Err(Error::TailUnderflow);
            }
        }
        Err(Error::SamplingLimit)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Update {
    Fixed(Variance),
    Sample { prior: Prior, initial: Scale },
}
impl Update {
    pub fn initial(self) -> Variance {
        match self {
            Self::Fixed(value) => value,
            Self::Sample { initial, .. } => initial.variance(),
        }
    }
    pub fn draw(self, stats: Statistics, rng: &mut Mt19937) -> Result<Variance, Error> {
        match self {
            Self::Fixed(value) => Ok(value),
            Self::Sample { prior, .. } => prior.conditional(stats)?.draw(rng),
        }
    }
}
