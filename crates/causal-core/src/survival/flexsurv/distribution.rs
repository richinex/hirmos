//! Built-in probability distributions used by flexsurv 2.3.2.
//!
//! Parameterizations follow `reference/flexsurv-2.3.2/R/distributions.R` and
//! the package's C++ density/CDF implementations.

use std::f64::consts::{PI, SQRT_2};

use spec_math::{Beta as SpecBeta, Gamma as SpecGamma};

use crate::survival::r_rng::RRng;

#[derive(Clone, Copy, Debug, PartialEq)]
enum DistributionKind {
    Exponential {
        rate: f64,
    },
    Weibull {
        shape: f64,
        scale: f64,
    },
    WeibullPh {
        shape: f64,
        scale: f64,
    },
    LogNormal {
        mean_log: f64,
        sd_log: f64,
    },
    Gamma {
        shape: f64,
        rate: f64,
    },
    Gompertz {
        shape: f64,
        rate: f64,
    },
    LogLogistic {
        shape: f64,
        scale: f64,
    },
    GeneralizedGamma {
        mu: f64,
        sigma: f64,
        q: f64,
    },
    GeneralizedGammaOriginal {
        shape: f64,
        scale: f64,
        k: f64,
    },
    GeneralizedF {
        mu: f64,
        sigma: f64,
        q: f64,
        p: f64,
    },
    GeneralizedFOriginal {
        mu: f64,
        sigma: f64,
        first_shape: f64,
        second_shape: f64,
    },
}

/// A source-compatible flexsurv distribution whose parameters are valid by
/// construction.
///
/// The representation is intentionally opaque.  Callers must use one of the
/// fallible named constructors, so a negative rate or zero scale cannot enter
/// the likelihood and wait to fail during optimization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexSurvDistribution(DistributionKind);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DistributionError {
    NonFiniteParameter,
    NonPositiveParameter(&'static str),
    NegativeParameter(&'static str),
    NegativeTime,
    ProbabilityOutsideUnitInterval,
    QuantileDoesNotExist,
    InvalidRestrictedMeanWindow,
    NumericalIntegrationFailed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DistributionValue {
    pub density: f64,
    pub cdf: f64,
    pub survival: f64,
    pub hazard: f64,
    pub cumulative_hazard: f64,
}

fn require_finite(parameters: &[f64]) -> Result<(), DistributionError> {
    if parameters.iter().all(|value| value.is_finite()) {
        Ok(())
    } else {
        Err(DistributionError::NonFiniteParameter)
    }
}

fn require_positive(name: &'static str, value: f64) -> Result<(), DistributionError> {
    if value > 0.0 {
        Ok(())
    } else {
        Err(DistributionError::NonPositiveParameter(name))
    }
}

fn normal_cdf(value: f64) -> f64 {
    0.5 * libm::erfc(-value / SQRT_2)
}

fn log_normal_log_density(time: f64, mean_log: f64, sd_log: f64) -> f64 {
    if time == 0.0 {
        return f64::NEG_INFINITY;
    }
    let z = (time.ln() - mean_log) / sd_log;
    -0.5 * z * z - time.ln() - sd_log.ln() - 0.5 * (2.0 * PI).ln()
}

impl FlexSurvDistribution {
    pub(crate) fn sample_r(self, rng: &mut RRng) -> Result<f64, DistributionError> {
        let value = match self.0 {
            DistributionKind::Exponential { rate } => rng.exponential() / rate,
            DistributionKind::Weibull { shape, scale } => {
                scale * (-rng.uniform().ln()).powf(1.0 / shape)
            }
            DistributionKind::WeibullPh { shape, scale } => {
                (-rng.uniform().ln() / scale).powf(1.0 / shape)
            }
            DistributionKind::LogNormal { mean_log, sd_log } => {
                (mean_log + sd_log * rng.normal()).exp()
            }
            DistributionKind::Gompertz { shape, rate } => {
                Self::gompertz(shape, rate)?.quantile(rng.uniform())?
            }
            DistributionKind::LogLogistic { shape, scale } => {
                let probability = rng.uniform();
                scale * (probability / (1.0 - probability)).powf(1.0 / shape)
            }
            DistributionKind::Gamma { shape, rate } => rng.gamma(shape, 1.0 / rate),
            DistributionKind::GeneralizedGamma { mu, sigma, q } => {
                if q == 0.0 {
                    (mu + sigma * rng.normal()).exp()
                } else {
                    let q_squared = q * q;
                    let gamma = rng.gamma(1.0 / q_squared, 1.0);
                    (mu + sigma * (q_squared * gamma).ln() / q).exp()
                }
            }
            DistributionKind::GeneralizedGammaOriginal { shape, scale, k } => {
                scale * (rng.gamma(k, 1.0).ln() / shape).exp()
            }
            DistributionKind::GeneralizedF { mu, sigma, q, p } => {
                if p == 0.0 {
                    return Self::generalized_gamma(mu, sigma, q)?.sample_r(rng);
                }
                let common = q * q + 2.0 * p;
                let delta = common.sqrt();
                let first_shape = 2.0 / (common + q * delta);
                let second_shape = 2.0 / (common - q * delta);
                let f_sample = rng.f_ratio(2.0 * first_shape, 2.0 * second_shape);
                (mu + sigma * f_sample.ln() / delta).exp()
            }
            DistributionKind::GeneralizedFOriginal {
                mu,
                sigma,
                first_shape,
                second_shape,
            } => {
                let f_sample = rng.f_ratio(2.0 * first_shape, 2.0 * second_shape);
                (mu + sigma * f_sample.ln()).exp()
            }
        };
        Ok(value)
    }

    pub fn exponential(rate: f64) -> Result<Self, DistributionError> {
        require_finite(&[rate])?;
        require_positive("rate", rate)?;
        Ok(Self(DistributionKind::Exponential { rate }))
    }

    pub fn weibull(shape: f64, scale: f64) -> Result<Self, DistributionError> {
        require_finite(&[shape, scale])?;
        require_positive("shape", shape)?;
        require_positive("scale", scale)?;
        Ok(Self(DistributionKind::Weibull { shape, scale }))
    }

    pub fn weibull_ph(shape: f64, scale: f64) -> Result<Self, DistributionError> {
        require_finite(&[shape, scale])?;
        require_positive("shape", shape)?;
        require_positive("scale", scale)?;
        Ok(Self(DistributionKind::WeibullPh { shape, scale }))
    }

    pub fn log_normal(mean_log: f64, sd_log: f64) -> Result<Self, DistributionError> {
        require_finite(&[mean_log, sd_log])?;
        require_positive("sd_log", sd_log)?;
        Ok(Self(DistributionKind::LogNormal { mean_log, sd_log }))
    }

    pub fn gamma(shape: f64, rate: f64) -> Result<Self, DistributionError> {
        require_finite(&[shape, rate])?;
        require_positive("shape", shape)?;
        require_positive("rate", rate)?;
        Ok(Self(DistributionKind::Gamma { shape, rate }))
    }

    pub fn gompertz(shape: f64, rate: f64) -> Result<Self, DistributionError> {
        require_finite(&[shape, rate])?;
        require_positive("rate", rate)?;
        Ok(Self(DistributionKind::Gompertz { shape, rate }))
    }

    pub fn log_logistic(shape: f64, scale: f64) -> Result<Self, DistributionError> {
        require_finite(&[shape, scale])?;
        require_positive("shape", shape)?;
        require_positive("scale", scale)?;
        Ok(Self(DistributionKind::LogLogistic { shape, scale }))
    }

    pub fn generalized_gamma(mu: f64, sigma: f64, q: f64) -> Result<Self, DistributionError> {
        require_finite(&[mu, sigma, q])?;
        require_positive("sigma", sigma)?;
        Ok(Self(DistributionKind::GeneralizedGamma { mu, sigma, q }))
    }

    pub fn generalized_gamma_original(
        shape: f64,
        scale: f64,
        k: f64,
    ) -> Result<Self, DistributionError> {
        require_finite(&[shape, scale, k])?;
        require_positive("shape", shape)?;
        require_positive("scale", scale)?;
        require_positive("k", k)?;
        Ok(Self(DistributionKind::GeneralizedGammaOriginal {
            shape,
            scale,
            k,
        }))
    }

    pub fn generalized_f(mu: f64, sigma: f64, q: f64, p: f64) -> Result<Self, DistributionError> {
        require_finite(&[mu, sigma, q, p])?;
        require_positive("sigma", sigma)?;
        if p < 0.0 {
            return Err(DistributionError::NegativeParameter("p"));
        }
        Ok(Self(DistributionKind::GeneralizedF { mu, sigma, q, p }))
    }

    pub fn generalized_f_original(
        mu: f64,
        sigma: f64,
        first_shape: f64,
        second_shape: f64,
    ) -> Result<Self, DistributionError> {
        require_finite(&[mu, sigma, first_shape, second_shape])?;
        require_positive("sigma", sigma)?;
        require_positive("s1", first_shape)?;
        require_positive("s2", second_shape)?;
        Ok(Self(DistributionKind::GeneralizedFOriginal {
            mu,
            sigma,
            first_shape,
            second_shape,
        }))
    }

    pub fn log_density(self, time: f64) -> Result<f64, DistributionError> {
        if time < 0.0 {
            return Ok(f64::NEG_INFINITY);
        }

        let value = match self.0 {
            DistributionKind::Exponential { rate } => rate.ln() - rate * time,
            DistributionKind::Weibull { shape, scale } => {
                shape.ln() - scale.ln() + (shape - 1.0) * (time / scale).ln()
                    - (time / scale).powf(shape)
            }
            DistributionKind::WeibullPh { shape, scale } => {
                shape.ln() + scale.ln() + (shape - 1.0) * time.ln() - scale * time.powf(shape)
            }
            DistributionKind::LogNormal { mean_log, sd_log } => {
                log_normal_log_density(time, mean_log, sd_log)
            }
            DistributionKind::Gamma { shape, rate } => {
                shape * rate.ln() + (shape - 1.0) * time.ln()
                    - rate * time
                    - shape.lgamma()
            }
            DistributionKind::Gompertz { shape, rate } => {
                let scaled = shape * time;
                let shift = if scaled == 0.0 {
                    time
                } else {
                    time * scaled.exp_m1() / scaled
                };
                rate.ln() + scaled - rate * shift
            }
            DistributionKind::LogLogistic { shape, scale } => {
                shape.ln() - scale.ln() + (shape - 1.0) * (time / scale).ln()
                    - 2.0 * (1.0 + (time / scale).powf(shape)).ln()
            }
            DistributionKind::GeneralizedGamma { mu, sigma, q } => {
                if q == 0.0 {
                    log_normal_log_density(time, mu, sigma)
                } else {
                    let w = (time.ln() - mu) / sigma;
                    let inverse_q_squared = 1.0 / (q * q);
                    -(sigma * time).ln()
                        + q.abs().ln() * (1.0 - 2.0 * inverse_q_squared)
                        + inverse_q_squared * (q * w - (q * w).exp())
                        - inverse_q_squared.lgamma()
                }
            }
            DistributionKind::GeneralizedGammaOriginal { shape, scale, k } => {
                shape.ln() - k.lgamma() + (shape * k - 1.0) * time.ln()
                    - shape * k * scale.ln()
                    - (time / scale).powf(shape)
            }
            DistributionKind::GeneralizedF { mu, sigma, q, p } => {
                if p == 0.0 {
                    return Self::generalized_gamma(mu, sigma, q)?.log_density(time);
                }
                let common = q * q + 2.0 * p;
                let delta = common.sqrt();
                let first_shape = 2.0 / (common + q * delta);
                let second_shape = 2.0 / (common - q * delta);
                let exponential_w = time.powf(delta / sigma) * (-mu * delta / sigma).exp();
                delta.ln()
                    + first_shape / sigma * delta * (time.ln() - mu)
                    + first_shape * (first_shape.ln() - second_shape.ln())
                    - (sigma * time).ln()
                    - (first_shape + second_shape)
                        * (1.0 + first_shape * exponential_w / second_shape).ln()
                    - first_shape.lbeta(second_shape)
            }
            DistributionKind::GeneralizedFOriginal {
                mu,
                sigma,
                first_shape,
                second_shape,
            } => {
                let w = (time.ln() - mu) / sigma;
                let exponential_w = time.powf(1.0 / sigma) * (-mu / sigma).exp();
                -(sigma * time).ln() + first_shape * (first_shape.ln() + w - second_shape.ln())
                    - (first_shape + second_shape)
                        * (1.0 + first_shape * exponential_w / second_shape).ln()
                    - first_shape.lbeta(second_shape)
            }
        };
        Ok(value)
    }

    pub fn cdf(self, time: f64) -> Result<f64, DistributionError> {
        if time < 0.0 {
            return Ok(0.0);
        }

        let value = match self.0 {
            DistributionKind::Exponential { rate } => -(-rate * time).exp_m1(),
            DistributionKind::Weibull { shape, scale } => -(-(time / scale).powf(shape)).exp_m1(),
            DistributionKind::WeibullPh { shape, scale } => -(-(scale * time.powf(shape))).exp_m1(),
            DistributionKind::LogNormal { mean_log, sd_log } => {
                if time == 0.0 {
                    0.0
                } else {
                    normal_cdf((time.ln() - mean_log) / sd_log)
                }
            }
            DistributionKind::Gamma { shape, rate } => shape.igamma(rate * time),
            DistributionKind::Gompertz { shape, rate } => {
                let coefficient = if shape == 0.0 {
                    -rate * time
                } else if time.is_infinite() && shape < 0.0 {
                    rate / shape
                } else {
                    -rate * (shape * time).exp_m1() / shape
                };
                -coefficient.exp_m1()
            }
            DistributionKind::LogLogistic { shape, scale } => {
                let power = (time / scale).powf(shape);
                power / (1.0 + power)
            }
            DistributionKind::GeneralizedGamma { mu, sigma, q } => {
                if q == 0.0 {
                    if time == 0.0 {
                        0.0
                    } else {
                        normal_cdf((time.ln() - mu) / sigma)
                    }
                } else {
                    let w = (time.ln() - mu) / sigma;
                    let shape = 1.0 / (q * q);
                    let point = (q * w).exp() * shape;
                    if q > 0.0 {
                        shape.igamma(point)
                    } else {
                        shape.igammac(point)
                    }
                }
            }
            DistributionKind::GeneralizedGammaOriginal { shape, scale, k } => {
                if time == 0.0 {
                    0.0
                } else {
                    k.igamma((time / scale).powf(shape))
                }
            }
            DistributionKind::GeneralizedF { mu, sigma, q, p } => {
                if p == 0.0 {
                    return Self::generalized_gamma(mu, sigma, q)?.cdf(time);
                }
                let common = q * q + 2.0 * p;
                let delta = common.sqrt();
                let first_shape = 2.0 / (common + q * delta);
                let second_shape = 2.0 / (common - q * delta);
                let exponential_w = time.powf(delta / sigma) * (-mu * delta / sigma).exp();
                let denominator = second_shape + first_shape * exponential_w;
                let complement_point = second_shape / denominator;
                if complement_point > 0.99 {
                    let point = first_shape * exponential_w / denominator;
                    point.ibeta(first_shape, second_shape)
                } else {
                    1.0 - complement_point.ibeta(second_shape, first_shape)
                }
            }
            DistributionKind::GeneralizedFOriginal {
                mu,
                sigma,
                first_shape,
                second_shape,
            } => {
                if time == 0.0 {
                    0.0
                } else {
                    let w = (time.ln() - mu) / sigma;
                    let exponential_w = w.exp();
                    let complement_point =
                        second_shape / (second_shape + first_shape * exponential_w);
                    1.0 - complement_point.ibeta(second_shape, first_shape)
                }
            }
        };
        Ok(value)
    }

    pub fn survival(self, time: f64) -> Result<f64, DistributionError> {
        if time < 0.0 {
            return Ok(1.0);
        }
        let value = match self.0 {
            DistributionKind::Exponential { rate } => (-rate * time).exp(),
            DistributionKind::Weibull { shape, scale } => (-(time / scale).powf(shape)).exp(),
            DistributionKind::WeibullPh { shape, scale } => (-scale * time.powf(shape)).exp(),
            DistributionKind::Gompertz { shape, rate } => {
                if shape == 0.0 {
                    (-rate * time).exp()
                } else if time.is_infinite() && shape < 0.0 {
                    (rate / shape).exp()
                } else {
                    (-rate * (shape * time).exp_m1() / shape).exp()
                }
            }
            DistributionKind::LogLogistic { shape, scale } => {
                1.0 / (1.0 + (time / scale).powf(shape))
            }
            DistributionKind::LogNormal { mean_log, sd_log } => {
                if time == 0.0 {
                    1.0
                } else {
                    0.5 * libm::erfc((time.ln() - mean_log) / (sd_log * SQRT_2))
                }
            }
            DistributionKind::Gamma { shape, rate } => shape.igammac(rate * time),
            DistributionKind::GeneralizedGamma { mu, sigma, q } => {
                if q == 0.0 {
                    if time == 0.0 {
                        1.0
                    } else {
                        0.5 * libm::erfc((time.ln() - mu) / (sigma * SQRT_2))
                    }
                } else {
                    let w = (time.ln() - mu) / sigma;
                    let shape = 1.0 / (q * q);
                    let point = (q * w).exp() * shape;
                    if q > 0.0 {
                        shape.igammac(point)
                    } else {
                        shape.igamma(point)
                    }
                }
            }
            _ => 1.0 - self.cdf(time)?,
        };
        Ok(value)
    }

    pub fn evaluate(self, time: f64) -> Result<DistributionValue, DistributionError> {
        if time < 0.0 {
            return Err(DistributionError::NegativeTime);
        }
        let log_density = self.log_density(time)?;
        let density = log_density.exp();
        let cdf = self.cdf(time)?;
        let survival = self.survival(time)?;
        let log_survival = survival.ln();
        Ok(DistributionValue {
            density,
            cdf,
            survival,
            hazard: (log_density - log_survival).exp(),
            cumulative_hazard: -log_survival,
        })
    }

    pub fn quantile(self, probability: f64) -> Result<f64, DistributionError> {
        if !(0.0..=1.0).contains(&probability) || !probability.is_finite() {
            return Err(DistributionError::ProbabilityOutsideUnitInterval);
        }
        if probability == 0.0 {
            return Ok(0.0);
        }
        if probability == 1.0 {
            if let DistributionKind::Gompertz { shape, .. } = self.0 {
                if shape < 0.0 {
                    return Err(DistributionError::QuantileDoesNotExist);
                }
            }
            return Ok(f64::INFINITY);
        }

        let value = match self.0 {
            DistributionKind::Exponential { rate } => -(-probability).ln_1p() / rate,
            DistributionKind::Weibull { shape, scale } => {
                scale * (-(-probability).ln_1p()).powf(1.0 / shape)
            }
            DistributionKind::WeibullPh { shape, scale } => {
                (-(-probability).ln_1p() / scale).powf(1.0 / shape)
            }
            DistributionKind::LogNormal { mean_log, sd_log } => {
                (mean_log + sd_log * spec_math::cephes64::ndtri(probability)).exp()
            }
            DistributionKind::Gamma { shape, rate } => shape.igamma_inv(probability) / rate,
            DistributionKind::Gompertz { shape, rate } => {
                if shape == 0.0 {
                    -(-probability).ln_1p() / rate
                } else {
                    let asymptote = 1.0 - (rate / shape).exp();
                    if shape < 0.0 && probability > asymptote {
                        f64::INFINITY
                    } else {
                        (-(-probability).ln_1p() * shape / rate).ln_1p() / shape
                    }
                }
            }
            DistributionKind::LogLogistic { shape, scale } => {
                scale * (probability / (1.0 - probability)).powf(1.0 / shape)
            }
            DistributionKind::GeneralizedGamma { mu, sigma, q } => {
                if q == 0.0 {
                    (mu + sigma * spec_math::cephes64::ndtri(probability)).exp()
                } else {
                    let adjusted = if q < 0.0 {
                        1.0 - probability
                    } else {
                        probability
                    };
                    let gamma_quantile = (1.0 / (q * q)).igamma_inv(adjusted);
                    (mu + sigma * ((q * q * gamma_quantile).ln() / q)).exp()
                }
            }
            DistributionKind::GeneralizedGammaOriginal { shape, scale, k } => {
                scale * k.igamma_inv(probability).powf(1.0 / shape)
            }
            DistributionKind::GeneralizedF { mu, sigma, q, p } => {
                if p == 0.0 {
                    return Self::generalized_gamma(mu, sigma, q)?.quantile(probability);
                }
                let common = q * q + 2.0 * p;
                let delta = common.sqrt();
                let first_shape = 2.0 / (common + q * delta);
                let second_shape = 2.0 / (common - q * delta);
                let beta_quantile = probability.ibeta_inv(first_shape, second_shape);
                let f_quantile =
                    second_shape * beta_quantile / (first_shape * (1.0 - beta_quantile));
                (mu + sigma * f_quantile.ln() / delta).exp()
            }
            DistributionKind::GeneralizedFOriginal {
                mu,
                sigma,
                first_shape,
                second_shape,
            } => {
                let beta_quantile = probability.ibeta_inv(first_shape, second_shape);
                let f_quantile =
                    second_shape * beta_quantile / (first_shape * (1.0 - beta_quantile));
                (mu + sigma * f_quantile.ln()).exp()
            }
        };
        Ok(value)
    }

    pub fn mean(self) -> Result<f64, DistributionError> {
        let value = match self.0 {
            DistributionKind::Exponential { rate } => 1.0 / rate,
            DistributionKind::Weibull { shape, scale } => {
                scale * SpecGamma::gamma(&(1.0 + 1.0 / shape))
            }
            DistributionKind::WeibullPh { shape, scale } => {
                scale.powf(-1.0 / shape) * SpecGamma::gamma(&(1.0 + 1.0 / shape))
            }
            DistributionKind::LogNormal { mean_log, sd_log } => {
                (mean_log + 0.5 * sd_log * sd_log).exp()
            }
            DistributionKind::Gamma { shape, rate } => shape / rate,
            DistributionKind::LogLogistic { shape, scale } if shape > 1.0 => {
                let angle = PI / shape;
                scale * angle / angle.sin()
            }
            DistributionKind::LogLogistic { .. } => f64::NAN,
            DistributionKind::GeneralizedGammaOriginal { shape, scale, k } => {
                scale * (puruspe::ln_gamma(k + 1.0 / shape) - puruspe::ln_gamma(k)).exp()
            }
            DistributionKind::GeneralizedFOriginal {
                mu,
                sigma,
                first_shape,
                second_shape,
            } if second_shape > sigma => {
                let log_moment = mu
                    + sigma * (second_shape / first_shape).ln()
                    + (first_shape + sigma).lbeta(second_shape - sigma)
                    - first_shape.lbeta(second_shape);
                log_moment.exp()
            }
            DistributionKind::GeneralizedFOriginal { .. } => f64::INFINITY,
            DistributionKind::Gompertz { shape, .. } if shape < 0.0 => f64::INFINITY,
            _ => self.restricted_mean(0.0, f64::INFINITY)?,
        };
        Ok(value)
    }

    pub fn restricted_mean(self, start: f64, end: f64) -> Result<f64, DistributionError> {
        if !start.is_finite() || start < 0.0 || end.is_nan() || end < start {
            return Err(DistributionError::InvalidRestrictedMeanWindow);
        }
        if start == end {
            return Ok(0.0);
        }

        if let DistributionKind::Exponential { rate } = self.0 {
            let width = end - start;
            return Ok(if width.is_infinite() {
                1.0 / rate
            } else {
                -(-rate * width).exp_m1() / rate
            });
        }

        let start_survival = self.survival(start)?;
        if start_survival == 0.0 {
            return Ok(0.0);
        }
        let integrand = |time: f64| self.survival(time).unwrap_or(f64::NAN) / start_survival;
        let value = if end.is_finite() {
            adaptive_simpson(&integrand, start, end, 1e-9, 22)
        } else {
            let transformed = |u: f64| {
                if u == 1.0 {
                    0.0
                } else {
                    let one_minus = 1.0 - u;
                    integrand(start + u / one_minus) / (one_minus * one_minus)
                }
            };
            adaptive_simpson(&transformed, 0.0, 1.0, 1e-9, 24)
        };
        if value.is_finite() {
            Ok(value)
        } else {
            Err(DistributionError::NumericalIntegrationFailed)
        }
    }
}

pub(crate) fn adaptive_simpson<F>(
    function: &F,
    lower: f64,
    upper: f64,
    tolerance: f64,
    depth: u32,
) -> f64
where
    F: Fn(f64) -> f64,
{
    fn recurse<F>(
        function: &F,
        lower: f64,
        upper: f64,
        tolerance: f64,
        whole: f64,
        depth: u32,
    ) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let middle = 0.5 * (lower + upper);
        let left_middle = 0.5 * (lower + middle);
        let right_middle = 0.5 * (middle + upper);
        let left = (middle - lower)
            * (function(lower) + 4.0 * function(left_middle) + function(middle))
            / 6.0;
        let right = (upper - middle)
            * (function(middle) + 4.0 * function(right_middle) + function(upper))
            / 6.0;
        let delta = left + right - whole;
        if depth == 0 || delta.abs() <= 15.0 * tolerance {
            left + right + delta / 15.0
        } else {
            recurse(function, lower, middle, tolerance / 2.0, left, depth - 1)
                + recurse(function, middle, upper, tolerance / 2.0, right, depth - 1)
        }
    }

    let middle = 0.5 * (lower + upper);
    let whole =
        (upper - lower) * (function(lower) + 4.0 * function(middle) + function(upper)) / 6.0;
    recurse(function, lower, upper, tolerance, whole, depth)
}

#[cfg(test)]
mod random_sampler_tests {
    use serde::Deserialize;

    use super::FlexSurvDistribution;
    use crate::survival::r_rng::RRng;

    #[derive(Deserialize)]
    struct SampleCase {
        values: Vec<f64>,
        next_uniforms: Vec<f64>,
    }

    #[derive(Deserialize)]
    struct Samples {
        exponential: SampleCase,
        weibull: SampleCase,
        weibull_ph: SampleCase,
        log_normal: SampleCase,
        gamma_small: SampleCase,
        gamma_mid: SampleCase,
        gamma_middle_coefficients: SampleCase,
        gamma_large: SampleCase,
        gompertz_zero: SampleCase,
        gompertz: SampleCase,
        log_logistic: SampleCase,
        generalized_gamma_zero: SampleCase,
        generalized_gamma_positive: SampleCase,
        generalized_gamma_negative: SampleCase,
        generalized_gamma_original: SampleCase,
        generalized_f_zero: SampleCase,
        generalized_f: SampleCase,
        generalized_f_original: SampleCase,
    }

    #[derive(Deserialize)]
    struct Oracle {
        r_version: String,
        rng_kind: String,
        seed: u32,
        draws_per_case: usize,
        samples: Samples,
        mixed_seed: u32,
        mixed: SampleCase,
    }

    fn oracle() -> Oracle {
        serde_json::from_str(include_str!(
            "../../../oracle/fixtures/flexsurv_random_samplers.json"
        ))
        .unwrap()
    }

    fn assert_close(actual: f64, expected: f64) {
        let tolerance = 3.0e-14 * expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual:.17e}, expected={expected:.17e}, tolerance={tolerance:.3e}"
        );
    }

    fn assert_case(seed: u32, draws: usize, distribution: FlexSurvDistribution, case: &SampleCase) {
        assert_eq!(case.values.len(), draws);
        let mut rng = RRng::new(seed);
        for &expected in &case.values {
            assert_close(distribution.sample_r(&mut rng).unwrap(), expected);
        }
        for &expected in &case.next_uniforms {
            assert_eq!(
                rng.uniform(),
                expected,
                "sampler consumed a different RNG path"
            );
        }
    }

    #[test]
    fn every_flexsurv_distribution_matches_r_4_4_1_values_and_draw_order() {
        let fixture = oracle();
        assert_eq!(fixture.r_version, "4.4.1");
        assert_eq!(fixture.rng_kind, "Mersenne-Twister/Inversion/Rejection");
        let seed = fixture.seed;
        let draws = fixture.draws_per_case;
        let samples = &fixture.samples;

        assert_case(
            seed,
            draws,
            FlexSurvDistribution::exponential(0.7).unwrap(),
            &samples.exponential,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::weibull(1.3, 2.2).unwrap(),
            &samples.weibull,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::weibull_ph(1.3, 0.4).unwrap(),
            &samples.weibull_ph,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::log_normal(0.2, 0.8).unwrap(),
            &samples.log_normal,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::gamma(0.37, 1.9).unwrap(),
            &samples.gamma_small,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::gamma(2.75, 1.9).unwrap(),
            &samples.gamma_mid,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::gamma(5.5, 1.9).unwrap(),
            &samples.gamma_middle_coefficients,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::gamma(25.0, 1.9).unwrap(),
            &samples.gamma_large,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::gompertz(0.0, 1.1).unwrap(),
            &samples.gompertz_zero,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::gompertz(0.3, 1.1).unwrap(),
            &samples.gompertz,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::log_logistic(1.6, 2.3).unwrap(),
            &samples.log_logistic,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::generalized_gamma(0.2, 0.8, 0.0).unwrap(),
            &samples.generalized_gamma_zero,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::generalized_gamma(0.2, 0.8, 0.7).unwrap(),
            &samples.generalized_gamma_positive,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::generalized_gamma(0.2, 0.8, -1.3).unwrap(),
            &samples.generalized_gamma_negative,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::generalized_gamma_original(1.7, 2.4, 0.45).unwrap(),
            &samples.generalized_gamma_original,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::generalized_f(0.2, 0.8, 0.7, 0.0).unwrap(),
            &samples.generalized_f_zero,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::generalized_f(0.2, 0.8, -0.4, 0.8).unwrap(),
            &samples.generalized_f,
        );
        assert_case(
            seed,
            draws,
            FlexSurvDistribution::generalized_f_original(0.2, 0.8, 1.4, 2.2).unwrap(),
            &samples.generalized_f_original,
        );
    }

    #[test]
    fn mixed_family_sequence_preserves_r_draw_order() {
        let fixture = oracle();
        let distributions = [
            FlexSurvDistribution::exponential(0.7).unwrap(),
            FlexSurvDistribution::gamma(0.37, 1.9).unwrap(),
            FlexSurvDistribution::generalized_gamma(0.2, 0.8, -1.3).unwrap(),
            FlexSurvDistribution::generalized_f(0.2, 0.8, -0.4, 0.8).unwrap(),
            FlexSurvDistribution::generalized_gamma_original(1.7, 2.4, 0.45).unwrap(),
            FlexSurvDistribution::generalized_f_original(0.2, 0.8, 1.4, 2.2).unwrap(),
        ];
        assert_eq!(distributions.len(), fixture.mixed.values.len());
        let mut rng = RRng::new(fixture.mixed_seed);
        for (distribution, &expected) in distributions.iter().zip(&fixture.mixed.values) {
            assert_close(distribution.sample_r(&mut rng).unwrap(), expected);
        }
        for &expected in &fixture.mixed.next_uniforms {
            assert_eq!(
                rng.uniform(),
                expected,
                "mixed sequence consumed a different RNG path"
            );
        }
    }
}
