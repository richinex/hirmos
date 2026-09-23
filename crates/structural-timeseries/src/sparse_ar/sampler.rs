//! BOOM 0.9.16 ArSpikeSlabSampler, LGPL-2.1-or-later.
//! Copyright Google LLC and Steven L. Scott. Reference fallback semantics are
//! preserved, including retaining coefficients after a caught fallback error.
use super::{Conditional, Kernel, Selection, UnscaledSlab};
use crate::{ar, prior::Statistics, regression::FlipSweep, state::Variance, Error};
use hirmos_causal_core::nprandom::{Mt19937, NpRng};
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Support {
    /// Reference truncation: inclusion probabilities ignore stationarity and
    /// a failed fallback can retain a nonstationary post-selection vector.
    ReferenceStationary,
    Unrestricted,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdatePath {
    Joint,
    Coordinate,
    RetainedAfterFallbackFailure,
}

/// Finite coefficients, not a promise of stationarity. The ordinary AR type
/// remains unchanged. Excluded lag positions are always exactly zero.
#[derive(Clone, Debug)]
pub struct Coefficients {
    values: Vec<f64>,
    included: Vec<bool>,
}
impl Coefficients {
    pub fn new(values: Vec<f64>, included: Vec<bool>) -> Result<Self, Error> {
        if values.is_empty() || values.len() != included.len() {
            return Err(Error::Shape);
        }
        if values.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        if values
            .iter()
            .zip(&included)
            .any(|(v, yes)| !yes && *v != 0.)
        {
            return Err(Error::InvalidProbability);
        }
        Ok(Self { values, included })
    }
    pub fn values(&self) -> &[f64] {
        &self.values
    }
    pub fn included(&self) -> &[bool] {
        &self.included
    }
    pub fn order(&self) -> usize {
        self.values.len()
    }
    pub fn is_stationary(&self) -> bool {
        stationary(&self.values)
    }
}
fn stationary(values: &[f64]) -> bool {
    values.is_empty() || ar::Coefficients::new(values.to_vec()).is_ok()
}

#[derive(Clone, Debug)]
pub struct Draw {
    pub support: Support,
    pub path: UpdatePath,
    pub coefficients: Coefficients,
}
#[derive(Clone)]
pub struct Sampler {
    prior: UnscaledSlab,
    coefficients: Coefficients,
    variance: Variance,
    support: Support,
    selection: Selection,
    path: UpdatePath,
    rng: NpRng,
    variance_rng: Mt19937,
}
impl Sampler {
    pub fn new(prior: UnscaledSlab, support: Support, seed: u32) -> Self {
        let p = prior.parameters().mean().len();
        Self {
            prior,
            coefficients: Coefficients::new(vec![0.; p], vec![true; p]).unwrap(),
            variance: Variance::new(1.).unwrap(),
            support,
            selection: Selection::Sweep(FlipSweep::All),
            path: UpdatePath::Joint,
            rng: NpRng::seeded(seed as u64),
            variance_rng: Mt19937::seeded(seed.wrapping_add(1)),
        }
    }
    pub fn with_selection(mut self, selection: Selection) -> Self {
        self.selection = selection;
        self
    }
    pub fn coefficients(&self) -> &Coefficients {
        &self.coefficients
    }
    pub fn variance(&self) -> Variance {
        self.variance
    }
    pub fn snapshot(&self) -> Draw {
        Draw {
            support: self.support,
            path: self.path,
            coefficients: self.coefficients.clone(),
        }
    }
    pub fn step(&mut self, x: &DMatrix<f64>, y: &DVector<f64>) -> Result<(), Error> {
        self.step_with_budget(x, y, 100)
    }
    fn step_with_budget(
        &mut self,
        x: &DMatrix<f64>,
        y: &DVector<f64>,
        budget: usize,
    ) -> Result<(), Error> {
        let kernel = Kernel::new(x, y, self.prior.clone())?;
        let mut next = self.clone();
        kernel.select(
            &mut next.coefficients.included,
            next.variance.value(),
            next.selection,
            &mut next.rng,
        )?;
        for (v, yes) in next
            .coefficients
            .values
            .iter_mut()
            .zip(&next.coefficients.included)
        {
            if !yes {
                *v = 0.;
            }
        }
        let conditional =
            kernel.conditional(next.coefficients.included(), next.variance.value())?;
        next.draw_coefficients(&conditional, budget)?;
        let phi = DVector::from_column_slice(next.coefficients.values());
        let squares = y.dot(y) - 2. * phi.dot(&kernel.xty) + phi.dot(&(&kernel.xtx * &phi));
        next.variance = next
            .prior
            .parameters()
            .residual_prior()
            .conditional(Statistics::from_summary(y.len(), squares)?)?
            .draw(&mut next.variance_rng)?;
        *self = next;
        Ok(())
    }
    fn draw_coefficients(
        &mut self,
        conditional: &Conditional,
        attempts: usize,
    ) -> Result<(), Error> {
        for _ in 0..attempts {
            let values = conditional.draw(&mut self.rng)?;
            if self.support == Support::Unrestricted || stationary(&values) {
                self.coefficients.values = values;
                self.path = UpdatePath::Joint;
                return Ok(());
            }
        }
        match self.coordinate(conditional) {
            Ok(values) => {
                self.coefficients.values = values;
                self.path = UpdatePath::Coordinate;
            }
            // BOOM catches the fallback error and retains the post-selection
            // coefficients. RNG consumption is retained, then variance updates.
            Err(_) => {
                self.path = UpdatePath::RetainedAfterFallbackFailure;
            }
        }
        Ok(())
    }
    fn coordinate(&mut self, c: &Conditional) -> Result<Vec<f64>, Error> {
        let mut phi: Vec<_> = c
            .indices
            .iter()
            .map(|i| self.coefficients.values[*i])
            .collect();
        if !self.coefficients.is_stationary() {
            // Preserve BOOM's compact-vector shrink and post-increment bound.
            let mut attempts = 0;
            loop {
                let old = attempts;
                attempts += 1;
                if old >= 20 || stationary(&phi) {
                    break;
                }
                for value in &mut phi {
                    *value *= 0.95;
                }
            }
            if attempts >= 20 {
                return Err(Error::NonStationary);
            }
        }
        for i in 0..phi.len() {
            if phi.len() == 1 {
                continue;
            }
            let q = c.precision[(i, i)];
            let mean = c.mean[i]
                - (0..phi.len())
                    .filter(|j| *j != i)
                    .map(|j| c.precision[(i, j)] * (phi[j] - c.mean[j]))
                    .sum::<f64>()
                    / q;
            let sd = (1. / q).sqrt();
            let initial = phi[i];
            let mut low = -1.;
            let mut high = 1.;
            let mut accepted = false;
            for _ in 0..1001 {
                let value = ar::truncated(mean, sd, low, high, &mut self.rng)?;
                phi[i] = value;
                let mut full = vec![0.; c.dimension];
                for (j, index) in c.indices.iter().enumerate() {
                    full[*index] = phi[j];
                }
                if stationary(&full) {
                    accepted = true;
                    break;
                }
                if value > initial {
                    high = value;
                } else {
                    low = value;
                }
            }
            if !accepted {
                return Err(Error::SamplingLimit);
            }
        }
        let mut full = vec![0.; c.dimension];
        for (j, index) in c.indices.iter().enumerate() {
            full[*index] = phi[j];
        }
        Ok(full)
    }
}

#[cfg(test)]
#[path = "../tests/sparse_sampler.rs"]
mod tests;
