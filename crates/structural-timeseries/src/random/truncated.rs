//! BOOM 0.9.16 trun_gamma.cpp and BoundedAdaptiveRejectionSampler.cpp.
//! Copyright Google LLC and Steven L. Scott; LGPL-2.1-or-later.
use super::Random;
use crate::Error;

impl Random {
    /// BOOM's shape/rate parameterization, including its five-step slice branch.
    pub fn truncated_gamma(&mut self, shape: f64, rate: f64, cut: f64) -> Result<f64, Error> {
        if !shape.is_finite()
            || !rate.is_finite()
            || !cut.is_finite()
            || shape <= 0.
            || rate <= 0.
            || cut <= 0.
        {
            return Err(Error::InvalidScale);
        }
        let density = |x: f64| (shape - 1.) * x.ln() - rate * x;
        let derivative = |x: f64| (shape - 1.) / x - rate;
        if cut <= (shape - 1.) / rate {
            for _ in 0..1_000_000 {
                let value = self.gamma(shape, 1. / rate)?;
                if value >= cut {
                    return Ok(value);
                }
            }
            return Err(Error::SamplingLimit);
        }
        if shape <= 1. {
            let mut x = cut;
            for _ in 0..5 {
                let log_slice = density(x) - self.exponential();
                let mut hi = x;
                let mut f = density(hi) - log_slice;
                let mut slope = derivative(hi);
                let mut attempts = 0;
                while f > f64::EPSILON.sqrt() {
                    hi -= f / slope;
                    f = density(hi) - log_slice;
                    slope = derivative(cut);
                    attempts += 1;
                    if attempts > 1000 {
                        return Err(Error::SamplingLimit);
                    }
                }
                x = cut + (hi - cut) * self.uniform();
                let mut trials = 0;
                while density(x) < log_slice {
                    hi = x;
                    x = cut + (hi - cut) * self.uniform();
                    trials += 1;
                    if trials > 1000 {
                        return Err(Error::SamplingLimit);
                    }
                }
            }
            return Ok(x);
        }
        if derivative(cut) >= 0. {
            return Err(Error::InvalidScale);
        }
        let mut points = vec![cut];
        let mut knots = vec![cut];
        for _ in 0..=1000 {
            let values: Vec<_> = points.iter().map(|&x| density(x)).collect();
            let slopes: Vec<_> = points.iter().map(|&x| derivative(x)).collect();
            knots.resize(points.len(), 0.);
            knots[0] = points[0];
            for i in 1..points.len() {
                knots[i] = if slopes[i] == slopes[i - 1] {
                    points[i - 1]
                } else {
                    ((values[i - 1] - slopes[i - 1] * points[i - 1])
                        - (values[i] - slopes[i] * points[i]))
                        / (slopes[i] - slopes[i - 1])
                };
            }
            let mut cdf = Vec::with_capacity(points.len());
            let mut last = 0.;
            for i in 0..points.len() {
                let d = slopes[i];
                let y = values[i] - values[0];
                let z = points[i];
                let first = if i + 1 == points.len() {
                    0.
                } else {
                    (1. / d) * (y - d * z + d * knots[i + 1]).exp()
                };
                let second = (1. / d) * (y - d * z + d * knots[i]).exp();
                last = last + first - second;
                if !last.is_finite() {
                    return Err(Error::NonFinite);
                }
                cdf.push(last);
            }
            let u = self.uniform() * last;
            let k = cdf.partition_point(|&v| v < u);
            if k == cdf.len() {
                return Err(Error::NonFinite);
            }
            let candidate = if k + 1 == cdf.len() {
                knots[k] + self.exponential() / (-slopes[k])
            } else {
                self.truncated_exponential(-slopes[k], knots[k], knots[k + 1])?
            };
            let hull = values[k] + slopes[k] * (candidate - points[k]);
            if hull - self.exponential() <= density(candidate) {
                return Ok(candidate);
            }
            // BOOM inserts using the knot positions, not the proposed points.
            let position = knots.partition_point(|&v| v < candidate);
            points.insert(position, candidate);
        }
        Err(Error::SamplingLimit)
    }

    pub(super) fn truncated_exponential(
        &mut self,
        rate: f64,
        lo: f64,
        hi: f64,
    ) -> Result<f64, Error> {
        if (hi - lo).abs() < 1e-7 {
            return Ok(lo);
        }
        if lo > hi {
            return Err(Error::InvalidScale);
        }
        let mut u = self.uniform();
        while u < f64::MIN_POSITIVE || u >= 1. {
            u = self.uniform();
        }
        let a = u.ln() - rate * hi;
        let b = (1. - u).ln() - rate * lo;
        let maximum = a.max(b);
        Ok((maximum + (a.min(b) - maximum).exp().ln_1p()) / -rate)
    }
}
