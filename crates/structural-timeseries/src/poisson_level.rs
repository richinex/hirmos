//! BOOM 0.9.16 local-level Poisson state-space sampler with regression.
//! Source copyright Google LLC / Steven L. Scott; LGPL-2.1-or-later.
//! Uses the existing Gaussian smoother, variance prior and spike/slab kernel.
use crate::{
    gaussian,
    poisson::{Imputer, Observation},
    poisson_regression::Draw as Regression,
    prior::{Statistics as Residuals, Update},
    sparse_ar::{Kernel, Selection, UnscaledSlab},
    state::{Component, Normal, System, Variance},
    weighted::Statistics,
    Error,
};
use hirmos_causal_core::nprandom::{Mt19937, NpRng};
use nalgebra::{DMatrix, DVector};
use rand_distr::Distribution;

#[derive(Clone, Debug)]
pub struct Draw {
    pub regression: Regression,
    pub level_variance: Variance,
    pub states: Vec<f64>,
}
#[derive(Clone)]
pub struct Chain {
    data: Vec<Option<Observation>>,
    x: DMatrix<f64>,
    rows: Vec<usize>,
    initial: Normal,
    update: Update,
    slab: UnscaledSlab,
    selection: Selection,
    draw: Draw,
    latent: Vec<Option<f64>>,
    noise: Vec<Variance>,
    statistics: Statistics,
    imputer: Imputer,
    rng: NpRng,
    gamma: Mt19937,
}
impl Chain {
    pub fn new(
        data: Vec<Option<Observation>>,
        x: DMatrix<f64>,
        initial: Normal,
        update: Update,
        slab: UnscaledSlab,
        selection: Selection,
        seed: u32,
    ) -> Result<Self, Error> {
        let rows: Vec<_> = data
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.map(|_| i))
            .collect();
        if rows.is_empty() {
            return Err(Error::Empty);
        }
        if x.nrows() != data.len() || x.ncols() != slab.parameters().mean().len() {
            return Err(Error::Shape);
        }
        if x.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let system = System::new(vec![Component::Level {
            innovation: update.initial(),
            initial,
        }])?;
        // Original augmented data start at zero with precision one. The first
        // state draw precedes non-state augmentation and parameter sampling.
        let latent: Vec<_> = data.iter().map(|v| v.map(|_| 0.)).collect();
        let noise: Vec<_> = data
            .iter()
            .map(|v| {
                Variance::new(if v.is_some() {
                    1.
                } else {
                    std::f64::consts::PI.powi(2) / 6.
                })
            })
            .collect::<Result<_, _>>()?;
        let mut rng = NpRng::seeded(seed as u64);
        let states = gaussian::draw_states_with_noise(&system, &noise, &latent, &mut rng)?
            .iter()
            .map(|v| v[0])
            .collect::<Vec<_>>();
        let observed_x = DMatrix::from_fn(rows.len(), x.ncols(), |i, j| x[(rows[i], j)]);
        let statistics = Statistics::new(
            &observed_x,
            &rows.iter().map(|&i| -states[i]).collect::<Vec<_>>(),
            &vec![1.; rows.len()],
        )?;
        let regression = Regression {
            coefficients: DVector::zeros(x.ncols()),
            included: slab
                .parameters()
                .inclusion_probabilities()
                .iter()
                .map(|p| *p > 0.)
                .collect(),
        };
        let mut chain = Self {
            data,
            x,
            rows,
            initial,
            update,
            slab,
            selection,
            draw: Draw {
                regression,
                level_variance: update.initial(),
                states,
            },
            latent,
            noise,
            statistics,
            imputer: Imputer::default(),
            rng,
            gamma: Mt19937::seeded(seed.wrapping_add(1)),
        };
        chain.augment()?;
        Ok(chain)
    }
    pub fn parameters(&self) -> &Draw {
        &self.draw
    }
    fn augment(&mut self) -> Result<(), Error> {
        let regression = &self.x * &self.draw.regression.coefficients;
        for &t in &self.rows {
            let augmented = self.imputer.impute(
                self.data[t].ok_or(Error::Empty)?,
                self.draw.states[t] + regression[t],
                &mut self.rng,
            )?;
            let (mean, variance) = augmented.gaussian()?;
            self.latent[t] = Some(mean);
            self.noise[t] = Variance::new(variance)?;
        }
        Ok(())
    }
    /// StateSpacePoissonPosteriorSampler uses the combined latent observation,
    /// unlike the two separately accumulated rows in standalone regression.
    fn statistics(&self) -> Result<Statistics, Error> {
        let x = DMatrix::from_fn(self.rows.len(), self.x.ncols(), |i, j| {
            self.x[(self.rows[i], j)]
        });
        let y = self
            .rows
            .iter()
            .map(|&i| {
                self.latent[i]
                    .ok_or(Error::Empty)
                    .map(|v| v - self.draw.states[i])
            })
            .collect::<Result<Vec<_>, _>>()?;
        let weights = self
            .rows
            .iter()
            .map(|&i| 1. / self.noise[i].value())
            .collect::<Vec<_>>();
        Statistics::new(&x, &y, &weights)
    }
    pub fn step(&mut self) -> Result<Draw, Error> {
        let mut next = self.clone();
        let kernel = Kernel::from_weighted(&next.statistics, next.slab.clone())?;
        kernel.select(
            &mut next.draw.regression.included,
            1.,
            next.selection,
            &mut next.rng,
        )?;
        next.draw.regression.coefficients = DVector::from_vec(
            kernel
                .conditional(&next.draw.regression.included, 1.)?
                .draw(&mut next.rng)?,
        );
        let mut residuals = Residuals::default();
        for pair in next.draw.states.windows(2) {
            residuals.add(pair[1] - pair[0])?;
        }
        next.draw.level_variance = next.update.draw(residuals, &mut next.gamma)?;
        next.augment()?;
        let regression = &next.x * &next.draw.regression.coefficients;
        let response = next
            .latent
            .iter()
            .enumerate()
            .map(|(i, v)| v.map(|v| v - regression[i]))
            .collect::<Vec<_>>();
        let system = System::new(vec![Component::Level {
            innovation: next.draw.level_variance,
            initial: next.initial,
        }])?;
        next.draw.states =
            gaussian::draw_states_with_noise(&system, &next.noise, &response, &mut next.rng)?
                .iter()
                .map(|v| v[0])
                .collect();
        next.statistics = next.statistics()?;
        let output = next.draw.clone();
        *self = next;
        Ok(output)
    }
    /// Independent predictive stream. Exposure is an offset on the count scale,
    /// not a predictor or a multiplier of the state innovation variance.
    pub fn forecast(
        &self,
        x: &DMatrix<f64>,
        exposure: &[f64],
        seed: u64,
    ) -> Result<Vec<f64>, Error> {
        self.snapshot()?.forecast(x, exposure, seed)
    }
    pub fn snapshot(&self) -> Result<Snapshot, Error> {
        Snapshot::new(
            self.draw.level_variance,
            *self.draw.states.last().ok_or(Error::Empty)?,
            self.draw.regression.coefficients.as_slice().to_vec(),
        )
    }
}

/// Complete local-level forecast state. Restoring one draw never depends on
/// a previous draw's inclusion mask or on retaining the fitted chain.
pub struct Snapshot {
    variance: Variance,
    state: f64,
    coefficients: DVector<f64>,
}
impl Snapshot {
    pub fn new(variance: Variance, state: f64, coefficients: Vec<f64>) -> Result<Self, Error> {
        if coefficients.is_empty() {
            return Err(Error::Empty);
        }
        if !state.is_finite() || coefficients.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        Ok(Self {
            variance,
            state,
            coefficients: DVector::from_vec(coefficients),
        })
    }
    pub fn replay(&self) -> crate::forecast::Replay {
        crate::forecast::Replay::ModelEquationsV1
    }
    pub fn forecast(
        &self,
        x: &DMatrix<f64>,
        exposure: &[f64],
        seed: u64,
    ) -> Result<Vec<f64>, Error> {
        if x.nrows() != exposure.len() || x.ncols() != self.coefficients.len() {
            return Err(Error::Shape);
        }
        if x.iter().chain(exposure).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        if exposure.iter().any(|v| *v < 0.) {
            return Err(Error::InvalidScale);
        }
        let mut rng = NpRng::seeded(seed);
        let mut state = self.state;
        let regression = x * &self.coefficients;
        let mut out = Vec::with_capacity(exposure.len());
        for (i, &exposure) in exposure.iter().enumerate() {
            state += self.variance.value().sqrt() * rng.standard_normal();
            let mean = exposure * (state + regression[i]).exp();
            out.push(sample_count(mean, &mut rng)?);
        }
        Ok(out)
    }
}

/// Adapt the existing PCG stream; no OS entropy or global/thread RNG.
struct Random<'a>(&'a mut NpRng);
impl rand_core::RngCore for Random<'_> {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u64() as u32
    }
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        for bytes in dst.chunks_mut(8) {
            bytes.copy_from_slice(&self.0.next_u64().to_le_bytes()[..bytes.len()]);
        }
    }
}
fn sample_count(mean: f64, rng: &mut NpRng) -> Result<f64, Error> {
    if !mean.is_finite() {
        return Err(Error::NonFinite);
    }
    if mean == 0. {
        return Ok(0.);
    }
    let distribution = rand_distr::Poisson::new(mean).map_err(|_| Error::InvalidScale)?;
    let value = distribution.sample(&mut Random(rng));
    if !value.is_finite() {
        return Err(Error::NonFinite);
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        defaults::Scale,
        regression::{FlipSweep, Inclusion, Slab},
    };
    fn chain() -> Chain {
        let slab = Slab::new(
            DVector::zeros(1),
            DMatrix::identity(1, 1),
            vec![Inclusion::new(0.).unwrap()],
            Scale::new(1.).unwrap(),
            1.,
        )
        .unwrap();
        Chain::new(
            vec![Some(Observation::new(2, 1.).unwrap()); 4],
            DMatrix::from_element(4, 1, 1.),
            Normal::new(0., Variance::new(1.).unwrap()).unwrap(),
            Update::Fixed(Variance::new(0.01).unwrap()),
            UnscaledSlab::new(slab),
            Selection::Sweep(FlipSweep::All),
            42,
        )
        .unwrap()
    }
    #[test]
    fn failed_state_sweep_preserves_parameters_mixtures_and_streams() {
        let mut a = chain();
        let mut b = a.clone();
        a.data[0] = Some(Observation::new(111, 1.).unwrap());
        b.data[0] = a.data[0];
        a.x[(3, 0)] = f64::NAN;
        assert!(matches!(a.step(), Err(Error::NonFinite)));
        assert_eq!(a.draw.states, b.draw.states);
        assert_eq!(a.draw.regression, b.draw.regression);
        assert_eq!(a.imputer.table().len(), b.imputer.table().len());
        a.x[(3, 0)] = 1.;
        for _ in 0..20 {
            let x = a.step().unwrap();
            let y = b.step().unwrap();
            assert_eq!(x.states, y.states);
            assert_eq!(x.regression, y.regression);
            assert_eq!(x.level_variance.value(), y.level_variance.value());
        }
    }
    #[test]
    fn count_library_matches_poisson_moments_and_cdf() {
        let mut rng = NpRng::seeded(57);
        for mean in [0.01_f64, 1., 11.9, 12., 100., 100000.] {
            let mut sum = 0.;
            let mut sum_sq = 0.;
            let mut below = 0.;
            let n = 50000;
            let cutoff = mean.floor();
            let expected_cdf = spec_math::cephes64::igamc(cutoff + 1., mean);
            for _ in 0..n {
                let value = sample_count(mean, &mut rng).unwrap();
                assert!(value >= 0. && value.fract() == 0.);
                sum += value;
                sum_sq += (value - mean).powi(2);
                if value <= cutoff {
                    below += 1.;
                }
            }
            let n = n as f64;
            assert!((sum / n - mean).abs() < 6. * (mean / n).sqrt());
            assert!((sum_sq / n - mean).abs() < 6. * ((mean + 2. * mean * mean) / n).sqrt());
            assert!(
                (below / n - expected_cdf).abs()
                    < 6. * (expected_cdf * (1. - expected_cdf) / n).sqrt() + 1. / n
            );
        }
        let mut control = rng.clone();
        assert_eq!(sample_count(-1., &mut rng), Err(Error::InvalidScale));
        assert_eq!(sample_count(f64::INFINITY, &mut rng), Err(Error::NonFinite));
        assert_eq!(rng.next_u64(), control.next_u64());
    }
}
