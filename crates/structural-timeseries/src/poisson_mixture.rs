//! BOOM 0.9.16 NormalMixtureApproximation and its mutable Poisson table.
//! Copyright Steven L. Scott / Google LLC, LGPL-2.1-or-later.
use crate::{
    brent,
    numerics::{self, IntegrationStatus},
    poisson_table::ANCHORS,
    Error,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Origin {
    Original,
    Interpolated {
        kl: f64,
        integration: [IntegrationStatus; 2],
    },
    Refitted {
        kl: f64,
        evaluations: usize,
        evaluation_limit_reached: bool,
        integration: [IntegrationStatus; 2],
    },
}
#[derive(Clone, Debug)]
pub struct Mixture {
    means: Vec<f64>,
    sds: Vec<f64>,
    weights: Vec<f64>,
    origin: Origin,
}
impl Mixture {
    fn new(means: Vec<f64>, sds: Vec<f64>, mut weights: Vec<f64>) -> Result<Self, Error> {
        let n = means.len();
        if n == 0 || sds.len() != n || weights.len() != n {
            return Err(Error::Shape);
        }
        if means
            .iter()
            .chain(&sds)
            .chain(&weights)
            .any(|x| !x.is_finite())
        {
            return Err(Error::NonFinite);
        }
        if sds.iter().any(|x| *x <= 0.) || weights.iter().any(|x| *x <= 0. || *x > 1.) {
            return Err(Error::InvalidScale);
        }
        let sum = weights.iter().sum::<f64>();
        if (sum - 1.).abs() > 1e-6 {
            if (sum - 1.).abs() >= 0.001 {
                return Err(Error::InvalidProbability);
            }
            for w in &mut weights {
                *w /= sum;
            }
        }
        let mut order: Vec<_> = (0..n).collect();
        order.sort_by(|&i, &j| means[i].total_cmp(&means[j]));
        Ok(Self {
            means: order.iter().map(|&i| means[i]).collect(),
            sds: order.iter().map(|&i| sds[i]).collect(),
            weights: order.iter().map(|&i| weights[i]).collect(),
            origin: Origin::Original,
        })
    }
    pub fn means(&self) -> &[f64] {
        &self.means
    }
    pub fn sds(&self) -> &[f64] {
        &self.sds
    }
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    pub fn origin(&self) -> &Origin {
        &self.origin
    }
    pub fn log_density(&self, x: f64) -> f64 {
        let logs: Vec<_> = self
            .means
            .iter()
            .zip(&self.sds)
            .zip(&self.weights)
            .map(|((m, s), w)| {
                let z = (x - m) / s;
                w.ln() - (0.918938533204672741780329736406 + 0.5 * z * z + s.ln())
            })
            .collect();
        let largest = logs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        if largest == f64::NEG_INFINITY {
            return largest;
        }
        largest + logs.iter().map(|v| (v - largest).exp()).sum::<f64>().ln()
    }
    fn theta(theta: &[f64]) -> Result<Self, Error> {
        let n = (theta.len() + 1) / 3;
        if n == 0 || theta.len() != 3 * n - 1 {
            return Err(Error::Shape);
        }
        let mut weights = vec![1.];
        weights.extend(theta[2 * n..].iter().map(|v| v.exp()));
        let sum = weights.iter().sum::<f64>();
        for w in &mut weights {
            *w /= sum;
        }
        Self::new(
            theta[..n].to_vec(),
            theta[n..2 * n].iter().map(|v| v.exp()).collect(),
            weights,
        )
    }
}

struct Distance {
    count: f64,
    log_gamma: f64,
    mode: f64,
    lo: f64,
    hi: f64,
}
impl Distance {
    fn new(count: u32, interpolation: bool) -> Result<Self, Error> {
        let count = count as f64;
        let log_gamma = spec_math::cephes64::lgam(count);
        let logf = |x: f64| -count * x - (-x).exp() - log_gamma;
        let (mode, source_mode_value) = brent::minimize(|x| -logf(x), 0., 1., 1e-5)?;
        let mut lo = mode - 1.;
        let mut hi = mode + 1.;
        // The preserved interpolation method calls C logf (float log) here.
        // Its fitting constructor instead calls the supplied target. Retain
        // that distinction, and the source Brent maximum_value sign.
        let mut flo = if interpolation {
            (lo as f32).ln() as f64
        } else {
            logf(lo)
        };
        let mut fhi = if interpolation {
            (hi as f32).ln() as f64
        } else {
            logf(hi)
        };
        while source_mode_value - flo < 30. {
            lo -= 1.;
            flo = logf(lo);
        }
        while source_mode_value - fhi < 30. {
            hi += 1.;
            fhi = logf(hi);
        }
        Ok(Self {
            count,
            log_gamma,
            mode,
            lo,
            hi,
        })
    }
    fn evaluate(&self, mixture: &Mixture) -> Result<(f64, [IntegrationStatus; 2]), Error> {
        let f = |x: f64| {
            let logp = -self.count * x - (-x).exp() - self.log_gamma;
            logp.exp() * (logp - mixture.log_density(x))
        };
        let left = numerics::integrate(f, self.lo, self.mode)?;
        let right = numerics::integrate(f, self.mode, self.hi)?;
        Ok((left.value + right.value, [left.status, right.status]))
    }
}

#[derive(Clone)]
pub struct Table {
    entries: Vec<(u32, Arc<Mixture>)>,
}
impl Default for Table {
    fn default() -> Self {
        Self {
            entries: ANCHORS
                .iter()
                .map(|&(n, m, s, w)| {
                    (
                        n,
                        Arc::new(Mixture {
                            means: m.to_vec(),
                            sds: s.to_vec(),
                            weights: w.to_vec(),
                            origin: Origin::Original,
                        }),
                    )
                })
                .collect(),
        }
    }
}
impl Table {
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn get(&self, count: u32) -> Option<&Mixture> {
        let p = self.entries.partition_point(|r| r.0 < count);
        self.entries
            .get(p)
            .filter(|r| r.0 == count)
            .map(|r| r.1.as_ref())
    }
    /// Insert only a fully computed source approximation. Query order is
    /// meaningful: the next request uses the current neighboring entries.
    pub fn ensure(&mut self, count: u32) -> Result<&Mixture, Error> {
        if count == 0 || count > 30000 {
            return Err(Error::InvalidScale);
        }
        let position = self.entries.partition_point(|r| r.0 < count);
        if self.entries[position].0 == count {
            return Ok(self.entries[position].1.as_ref());
        }
        let (lo, a) = &self.entries[position - 1];
        let (hi, b) = &self.entries[position];
        let n = a.means.len();
        let fraction = (count - lo) as f64 / (hi - lo) as f64;
        if n == b.means.len() {
            let blend = |a: &[f64], b: &[f64]| {
                a.iter()
                    .zip(b)
                    .map(|(a, b)| (1. - fraction) * a + fraction * b)
                    .collect()
            };
            let mut mixture = Mixture::new(
                blend(&a.means, &b.means),
                blend(&a.sds, &b.sds),
                blend(&a.weights, &b.weights),
            )?;
            let (kl, integration) = Distance::new(count, true)?.evaluate(&mixture)?;
            if kl < 1e-5 {
                mixture.origin = Origin::Interpolated { kl, integration };
                self.entries.insert(position, (count, Arc::new(mixture)));
                return Ok(self.get(count).unwrap());
            }
        }
        let nu = count as f64;
        // Preserve the source's slightly different initial sigma expressions.
        let sigma = if n == b.means.len() {
            (1. / nu).sqrt()
        } else {
            1. / nu.sqrt()
        };
        let mut initial = vec![-nu.ln(); n];
        initial.extend(vec![sigma.ln(); n]);
        initial.extend(vec![0.; n - 1]);
        let distance = Distance::new(count, false)?;
        let fit = numerics::minimize(
            |theta| {
                Mixture::theta(theta)
                    .and_then(|m| distance.evaluate(&m))
                    .map(|v| v.0)
                    .unwrap_or(f64::NAN)
            },
            &initial,
            0.5 / nu.sqrt(),
            1e-6,
            20000,
        )?;
        let mut mixture = Mixture::theta(&fit.point)?;
        let (kl, integration) = distance.evaluate(&mixture)?;
        mixture.origin = Origin::Refitted {
            kl,
            evaluations: fit.evaluations,
            evaluation_limit_reached: fit.evaluations >= 20000,
            integration,
        };
        self.entries.insert(position, (count, Arc::new(mixture)));
        Ok(self.get(count).unwrap())
    }
}
