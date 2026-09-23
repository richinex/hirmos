//! BOOM 0.9.16 PoissonDataImputer, original mixture anchors and large-count limit.
//! Source copyright Steven L. Scott / Google LLC, LGPL-2.1-or-later.
//! Owned imputers retain the source's query-ordered interpolated/refitted table.
use crate::{poisson_mixture::Table, poisson_table::ANCHORS, Error};
use hirmos_causal_core::nprandom::NpRng;

#[cfg(test)]
mod tests {
    #[test]
    fn every_original_anchor_produces_valid_gaussian_data() {
        let mut rng = hirmos_causal_core::nprandom::NpRng::seeded(73);
        for &(count, _, _, _) in super::ANCHORS {
            let observation = super::Observation::new(count, 1.).unwrap();
            for _ in 0..20 {
                let draw = super::impute(observation, (count as f64).ln(), &mut rng).unwrap();
                let (mean, variance) = draw.gaussian().unwrap();
                assert!(mean.is_finite() && variance.is_finite() && variance > 0.);
                assert!(draw.internal.is_some());
            }
        }
    }

    #[test]
    fn every_original_mixture_anchor_matches_the_docker_export() {
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../fixtures/poisson-table.json")).unwrap();
        let values = fixture["values"].as_array().unwrap();
        let mut position = 0;
        assert_eq!(super::ANCHORS.len(), 244);
        for &(count, means, sds, weights) in super::ANCHORS {
            assert_eq!(count as f64, values[position].as_f64().unwrap());
            assert_eq!(means.len() as f64, values[position + 1].as_f64().unwrap());
            position += 2;
            for array in [weights, sds, means] {
                for &value in array {
                    assert_eq!(value, values[position].as_f64().unwrap());
                    position += 1;
                }
            }
        }
        assert_eq!(position, values.len());
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Observation {
    count: u32,
    exposure: f64,
}
impl Observation {
    pub fn count(self) -> u32 {
        self.count
    }
    pub fn exposure(self) -> f64 {
        self.exposure
    }
    pub fn new(count: u32, exposure: f64) -> Result<Self, Error> {
        if count > i32::MAX as u32 || !exposure.is_finite() || exposure <= 0. {
            return Err(Error::InvalidScale);
        }
        Ok(Self { count, exposure })
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Latent {
    pub negative_log_time: f64,
    pub mean: f64,
    pub precision: f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Augmented {
    pub internal: Option<Latent>,
    pub external: Latent,
}
impl Augmented {
    /// StateSpacePoissonPosteriorSampler's combined Gaussian pseudo-observation.
    pub fn gaussian(self) -> Result<(f64, f64), Error> {
        let mut precision = self.external.precision;
        let mut numerator = (self.external.negative_log_time - self.external.mean) * precision;
        if let Some(value) = self.internal {
            precision += value.precision;
            numerator += (value.negative_log_time - value.mean) * value.precision;
        }
        let mean = numerator / precision;
        let variance = 1. / precision;
        if !mean.is_finite() || !variance.is_finite() || variance <= 0. {
            return Err(Error::NonFinite);
        }
        Ok((mean, variance))
    }
}
fn anchor(count: u32) -> Result<usize, Error> {
    let p = ANCHORS.partition_point(|r| r.0 < count);
    if ANCHORS.get(p).is_some_and(|r| r.0 == count) {
        Ok(p)
    } else {
        Err(Error::MissingMixture(count))
    }
}
fn unmix(
    residual: f64,
    count: u32,
    table: Option<&Table>,
    rng: &mut NpRng,
) -> Result<(f64, f64), Error> {
    if count >= 30000 {
        return Ok((-(count as f64).ln(), 1. / count as f64));
    }
    let (means, sds, weights) = match table {
        Some(table) => {
            let mixture = table.get(count).ok_or(Error::MissingMixture(count))?;
            (mixture.means(), mixture.sds(), mixture.weights())
        }
        None => {
            let (_, m, s, w) = ANCHORS[anchor(count)?];
            (m, s, w)
        }
    };
    let mut probabilities: Vec<_> = means
        .iter()
        .zip(sds)
        .zip(weights)
        .map(|((m, s), w)| w.ln() - s.ln() - 0.5 * ((residual - m) / s).powi(2))
        .collect();
    let largest = probabilities
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    if !largest.is_finite() {
        return Err(Error::NonFinite);
    }
    let mut sum = 0.;
    for p in &mut probabilities {
        *p = (*p - largest).exp();
        sum += *p;
    }
    let target = rng.next_f64() * sum;
    let mut total = 0.;
    for (j, p) in probabilities.iter().enumerate() {
        total += p;
        if target < total {
            return Ok((means[j], sds[j] * sds[j]));
        }
    }
    Err(Error::InvalidProbability)
}
/// A complete failed augmentation leaves the caller's RNG untouched.
pub fn impute(
    observation: Observation,
    log_rate: f64,
    rng: &mut NpRng,
) -> Result<Augmented, Error> {
    if observation.count > 0 && observation.count < 30000 && anchor(observation.count).is_err() {
        return Imputer::default().impute(observation, log_rate, rng);
    }
    impute_with(observation, log_rate, None, rng)
}

/// A chain owns its fitted mixtures. There is no process-wide mutable cache.
#[derive(Clone, Default)]
pub struct Imputer {
    table: Table,
}
impl Imputer {
    pub fn table(&self) -> &Table {
        &self.table
    }
    pub fn impute(
        &mut self,
        observation: Observation,
        log_rate: f64,
        rng: &mut NpRng,
    ) -> Result<Augmented, Error> {
        if !log_rate.is_finite() {
            return Err(Error::NonFinite);
        }
        let count = observation.count;
        if count > 0 && count < 30000 && self.table.get(count).is_none() {
            let mut next = self.table.clone();
            next.ensure(count)?;
            let draw = impute_with(observation, log_rate, Some(&next), rng)?;
            self.table = next;
            Ok(draw)
        } else {
            impute_with(observation, log_rate, Some(&self.table), rng)
        }
    }
}

fn impute_with(
    observation: Observation,
    log_rate: f64,
    table: Option<&Table>,
    rng: &mut NpRng,
) -> Result<Augmented, Error> {
    if !log_rate.is_finite() {
        return Err(Error::NonFinite);
    }
    let mut next = rng.clone();
    // Beta(n,1) inverse CDF. Same distribution as BOOM rbeta, different stream.
    let final_internal = if observation.count > 0 {
        observation.exposure * (next.next_f64().ln() / observation.count as f64).exp()
    } else {
        0.
    };
    let delta = observation.exposure - final_internal;
    let z = if log_rate.abs() < 600. {
        -(delta - (-next.next_f64()).ln_1p() / log_rate.exp()).ln()
    } else {
        let gumbel = -(-next.next_f64().ln()).ln();
        if delta > 0. {
            let a = delta.ln();
            let b = -gumbel - log_rate;
            let hi = a.max(b);
            let lo = a.min(b);
            -(hi + (lo - hi).exp().ln_1p())
        } else {
            log_rate + gumbel
        }
    };
    if !z.is_finite() {
        return Err(Error::NonFinite);
    }
    let (mean, variance) = unmix(z - log_rate, 1, table, &mut next)?;
    let external = Latent {
        negative_log_time: z,
        mean,
        precision: 1. / variance,
    };
    let internal = if observation.count > 0 {
        let z = -final_internal.ln();
        if !z.is_finite() {
            return Err(Error::NonFinite);
        }
        let (mean, variance) = unmix(z - log_rate, observation.count, table, &mut next)?;
        Some(Latent {
            negative_log_time: z,
            mean,
            precision: 1. / variance,
        })
    } else {
        None
    };
    let output = Augmented { internal, external };
    output.gaussian()?;
    *rng = next;
    Ok(output)
}
