//! Forecasts from a complete parameter/state snapshot. This deliberately uses
//! the model equations, not BOOM 0.9.16's previous-mask coefficient replay.
use crate::{
    state::{Component, System, Variance},
    Error,
};
use hirmos_causal_core::nprandom::NpRng;
use nalgebra::DVector;
use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Replay {
    ModelEquationsV1,
}

pub struct Snapshot {
    system: System,
    observation: Variance,
    final_state: DVector<f64>,
    training_rows: NonZeroUsize,
}
pub struct Path {
    pub observations: Vec<f64>,
    pub states: Vec<Vec<f64>>,
}
impl Snapshot {
    pub fn new(
        components: Vec<Component>,
        observation: Variance,
        final_state: Vec<f64>,
        training_rows: NonZeroUsize,
    ) -> Result<Self, Error> {
        let dimension: usize = components
            .iter()
            .try_fold(0usize, |sum, c| sum.checked_add(c.dimension()))
            .ok_or(Error::Shape)?;
        if final_state.len() != dimension {
            return Err(Error::Shape);
        }
        if final_state.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let mut offset = 0;
        for component in &components {
            if let Component::Semilocal { slope, .. } = component {
                if final_state[offset + 2] != slope.mean() {
                    return Err(Error::InvalidScale);
                }
            }
            offset += component.dimension();
        }
        Ok(Self {
            system: System::new(components)?,
            observation,
            final_state: DVector::from_vec(final_state),
            training_rows,
        })
    }
    pub fn replay(&self) -> Replay {
        Replay::ModelEquationsV1
    }

    /// Offsets are predictions of any static regression for this same posterior
    /// draw. Gaussian-only models supply zeros. The seed never changes a chain.
    pub fn forecast(&self, offsets: &[f64], seed: u64) -> Result<Vec<f64>, Error> {
        Ok(self.path(offsets, seed)?.observations)
    }

    /// Retain latent states for component reporting without drawing a second path.
    pub fn path(&self, offsets: &[f64], seed: u64) -> Result<Path, Error> {
        if offsets.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let mut state = self.final_state.clone();
        let mut rng = NpRng::seeded(seed);
        let mut result = Vec::with_capacity(offsets.len());
        let mut states = Vec::with_capacity(offsets.len());
        for (h, offset) in offsets.iter().enumerate() {
            let t = (self.training_rows.get() - 1)
                .checked_add(h)
                .ok_or(Error::Shape)?;
            let (transition, innovation) = self.system.transition(t)?;
            let z = self
                .system
                .observation_at(t.checked_add(1).ok_or(Error::Shape)?)?;
            state = transition * state;
            for j in 0..state.len() {
                state[j] += innovation[(j, j)].sqrt() * rng.standard_normal();
            }
            let value =
                offset + z.dot(&state) + self.observation.value().sqrt() * rng.standard_normal();
            if !value.is_finite() {
                return Err(Error::NonFinite);
            }
            result.push(value);
            states.push(state.as_slice().to_vec());
        }
        Ok(Path { observations: result, states })
    }
}
