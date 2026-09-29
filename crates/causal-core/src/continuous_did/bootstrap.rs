//! ptetools 1.0.1 mboot2. Unlike did::mboot, signs use base R sample().
use crate::continuous_did::r_rng::{standard_normal_quantile, RRng};

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    Iterations,
    Alpha,
    DegenerateScale { column: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bands {
    Pointwise,
    Simultaneous,
}

#[derive(Debug)]
pub struct Bootstrap {
    pub draws: Vec<Vec<f64>>,
    pub standard_errors: Vec<f64>,
    pub raw_critical: f64,
    pub critical: f64,
    /// The reference issues a warning when imposing this floor. Never hide it.
    pub pointwise_floor_applied: bool,
}

/// Retain state across successive calls, as R's global RNG does.
pub struct Stream(RRng);
impl Stream {
    pub fn new(seed: u32) -> Self {
        Self(RRng::new(seed))
    }

    pub fn run(
        &mut self,
        influence: &[Vec<f64>],
        iterations: usize,
        alpha: f64,
    ) -> Result<Bootstrap, Error> {
        let n = influence.len();
        let k = influence.first().map_or(0, Vec::len);
        if n == 0 || k == 0 || influence.iter().any(|r| r.len() != k) {
            return Err(Error::Shape);
        }
        if iterations == 0 {
            return Err(Error::Iterations);
        }
        if !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 || 1.0 - alpha / 2.0 == 1.0 {
            return Err(Error::Alpha);
        }
        if influence.iter().flatten().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let root_n = (n as f64).sqrt();
        let mut draws = vec![vec![0.0; k]; iterations];
        for draw in &mut draws {
            for row in influence {
                let sign = if self.0.sample_index(2) == 0 {
                    -1.0
                } else {
                    1.0
                };
                for (value, x) in draw.iter_mut().zip(row) {
                    *value += sign * x;
                }
            }
            for value in draw {
                *value = *value / n as f64 * root_n;
            }
        }
        if draws.iter().flatten().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let normal_iqr = standard_normal_quantile(0.75) - standard_normal_quantile(0.25);
        let mut standard_errors = Vec::with_capacity(k);
        for j in 0..k {
            let mut column: Vec<_> = draws.iter().map(|r| r[j]).collect();
            column.sort_by(f64::total_cmp);
            let se = (quantile(&column, 0.75) - quantile(&column, 0.25)) / normal_iqr / root_n;
            // Undefined oracle bands are a refusal, not a replacement interval.
            if se <= 0.0 || !se.is_finite() {
                return Err(Error::DegenerateScale { column: j });
            }
            standard_errors.push(se);
        }
        let mut maxima: Vec<f64> = draws
            .iter()
            .map(|row| {
                row.iter()
                    .zip(&standard_errors)
                    .map(|(x, se)| (x / se).abs())
                    .fold(0.0, f64::max)
                    / root_n
            })
            .collect();
        maxima.sort_by(f64::total_cmp);
        let raw_critical = quantile(&maxima, 1.0 - alpha);
        if !raw_critical.is_finite() {
            return Err(Error::NonFinite);
        }
        let floor = standard_normal_quantile(1.0 - alpha / 2.0);
        Ok(Bootstrap {
            draws,
            standard_errors,
            raw_critical,
            critical: raw_critical.max(floor),
            pointwise_floor_applied: raw_critical < floor,
        })
    }
}

// R quantile(type=1), not the interpolated quantile used for dose-grid defaults.
fn quantile(sorted: &[f64], probability: f64) -> f64 {
    sorted[((sorted.len() as f64 * probability).ceil() as usize)
        .saturating_sub(1)
        .min(sorted.len() - 1)]
}
