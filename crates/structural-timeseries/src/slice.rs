//! Positive-domain branches of BOOM 0.9.16 ScalarSliceSampler.cpp.
//! Copyright 2007 Steven L. Scott / 2018 Google LLC; LGPL-2.1-or-later.
//! Student trend constructs a fresh sampler for each degrees-of-freedom draw.
use crate::Error;
use hirmos_causal_core::nprandom::NpRng;

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Unimodal,
    General,
}

/// Lower-bounded BOOM sampler. Regression retains this object across sweeps;
/// Student trend instead constructs a fresh unimodal sampler for every draw.
#[derive(Clone, Debug)]
pub struct Positive {
    width: f64,
    shape: Shape,
}
impl Positive {
    pub fn new(shape: Shape) -> Self {
        Self { width: 1., shape }
    }

    pub fn draw(
        &mut self,
        value: f64,
        rng: &mut NpRng,
        log_density: impl Fn(f64) -> f64,
    ) -> Result<f64, Error> {
        let mut sampler = self.clone();
        let mut next = rng.clone();
        let result = sampler.draw_inner(value, &mut next, log_density)?;
        *self = sampler;
        *rng = next;
        Ok(result)
    }

    fn draw_inner(
        &mut self,
        value: f64,
        rng: &mut NpRng,
        log_density: impl Fn(f64) -> f64,
    ) -> Result<f64, Error> {
        if !value.is_finite() || value <= 0. {
            return Err(Error::InvalidScale);
        }
        let initial = log_density(value);
        if !initial.is_finite() {
            return Err(Error::NonFinite);
        }
        let height = initial + (1. - rng.next_f64()).ln();
        if !height.is_finite() {
            return Err(Error::NonFinite);
        }
        let checked = |x| {
            let p = log_density(x);
            if p.is_nan() || p == f64::INFINITY {
                Err(Error::NonFinite)
            } else {
                Ok(p)
            }
        };
        let mut lo = 0.;
        let mut hi = value + self.width;
        if !hi.is_finite() || hi <= value {
            return Err(Error::InvalidScale);
        }
        let mut bracketed = false;
        for _ in 0..=100 {
            // Preserve short-circuit consumption of the extra doubling coin.
            let expand = checked(hi)? >= height
                || (matches!(self.shape, Shape::General) && rng.next_f64() > 0.5);
            if !expand {
                bracketed = true;
                break;
            }
            hi = value + 2. * (hi - value);
            if !hi.is_finite() {
                return Err(Error::NonFinite);
            }
        }
        if !bracketed {
            return Err(Error::SamplingLimit);
        }
        for _ in 0..=100 {
            let candidate = lo + (hi - lo) * rng.next_f64();
            if checked(candidate)? >= height {
                return if candidate > 0. {
                    Ok(candidate)
                } else {
                    Err(Error::InvalidScale)
                };
            }
            if candidate > value {
                hi = candidate;
            } else {
                lo = candidate;
            }
            self.width = hi - lo;
            if !self.width.is_finite() || self.width <= 0. {
                return Err(Error::InvalidScale);
            }
        }
        Err(Error::SamplingLimit)
    }
}

/// Starts with width one, lower bound zero, doubles the upper bracket, then
/// contracts about the current value. Failed draws do not consume caller RNG.
pub fn positive(
    value: f64,
    rng: &mut NpRng,
    log_density: impl Fn(f64) -> f64,
) -> Result<f64, Error> {
    Positive::new(Shape::Unimodal).draw(value, rng, log_density)
}
