//! Local-level state with Student observation errors. BOOM state-space sweep
//! order, reusing the observation sampler and existing Gaussian state kernel.
//! Source copyright Steven L. Scott / Google LLC; LGPL-2.1-or-later.
use crate::{
    gaussian,
    prior::{Prior, Statistics, Update},
    regression::{Inclusion, Slab},
    sparse_ar::{Selection, UnscaledSlab},
    state::{Component, Normal, System, Variance},
    student::UniformTail,
    student_regression::{Draw as Observation, Regression},
    Error,
};
use hirmos_causal_core::nprandom::{Mt19937, NpRng};
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Debug)]
pub struct Draw {
    pub observation: Observation,
    pub level_variance: Variance,
    pub states: Vec<f64>,
}
#[derive(Clone)]
pub struct Chain {
    y: Vec<Option<f64>>,
    rows: Vec<usize>,
    initial: Normal,
    level: Update,
    variance: Variance,
    states: Vec<DVector<f64>>,
    weights: Vec<f64>,
    observation: Regression,
    rng: NpRng,
    gamma: Mt19937,
}
impl Chain {
    pub fn new(
        y: Vec<Option<f64>>,
        initial: Normal,
        level: Update,
        observation: Prior,
        tail: UniformTail,
        seed: u32,
    ) -> Result<Self, Error> {
        let rows: Vec<_> = y
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.map(|_| i))
            .collect();
        if rows.is_empty() {
            return Err(Error::Empty);
        }
        let system = System::new(vec![Component::Level {
            innovation: level.initial(),
            initial,
        }])?;
        let mut rng = NpRng::seeded(seed as u64);
        let states = gaussian::draw_states(&system, Variance::new(1.)?, &y, &mut rng)?;
        let slab = Slab::with_prior(
            DVector::zeros(1),
            DMatrix::identity(1, 1),
            vec![Inclusion::new(0.)?],
            observation,
        )?;
        let residuals = DVector::from_iterator(
            rows.len(),
            rows.iter().map(|i| y[*i].unwrap() - states[*i][0]),
        );
        let observation = Regression::new_selected(
            DMatrix::from_element(rows.len(), 1, 1.),
            residuals,
            UnscaledSlab::new(slab),
            Selection::Fixed,
            tail,
            seed.wrapping_add(2),
        )?;
        let weights = vec![1.; rows.len()];
        Ok(Self {
            y,
            rows,
            initial,
            level,
            variance: level.initial(),
            states,
            weights,
            observation,
            rng,
            gamma: Mt19937::seeded(seed.wrapping_add(1)),
        })
    }
    pub fn step(&mut self) -> Result<Draw, Error> {
        let mut next = self.clone();
        let response = DVector::from_iterator(
            next.rows.len(),
            next.rows
                .iter()
                .map(|i| next.y[*i].unwrap() - next.states[*i][0]),
        );
        let observation = next.observation.conditioned_step(response, &next.weights)?;
        let mut stats = Statistics::default();
        for t in 1..next.states.len() {
            stats.add(next.states[t][0] - next.states[t - 1][0])?;
        }
        next.variance = next.level.draw(stats, &mut next.gamma)?;
        let mut noise = vec![observation.variance; next.y.len()];
        for (j, t) in next.rows.iter().enumerate() {
            let residual = next.y[*t].unwrap() - next.states[*t][0];
            let rate = (observation.nu + residual * residual / observation.variance.value()) / 2.;
            if !rate.is_finite() {
                return Err(Error::NonFinite);
            }
            let weight = 1.
                / hirmos_causal_core::ucm::invgamma_rvs(
                    (observation.nu + 1.) / 2.,
                    rate,
                    &mut next.gamma,
                );
            if !weight.is_finite() || weight <= 0. {
                return Err(Error::TailUnderflow);
            }
            next.weights[j] = weight;
            noise[*t] = Variance::new(observation.variance.value() / weight)?;
        }
        let system = System::new(vec![Component::Level {
            innovation: next.variance,
            initial: next.initial,
        }])?;
        next.states = gaussian::draw_states_with_noise(&system, &noise, &next.y, &mut next.rng)?;
        let result = Draw {
            observation,
            level_variance: next.variance,
            states: next.states.iter().map(|x| x[0]).collect(),
        };
        *self = next;
        Ok(result)
    }
    pub fn forecast(&self, horizon: usize, seed: u32) -> Result<Vec<f64>, Error> {
        let mut rng = NpRng::seeded(seed as u64);
        let mut gamma = Mt19937::seeded(seed.wrapping_add(1));
        let mut state = self.states.last().ok_or(Error::Empty)?[0];
        let p = self.observation.parameters();
        let mut values = Vec::with_capacity(horizon);
        for _ in 0..horizon {
            state += self.variance.value().sqrt() * rng.standard_normal();
            let mixing = hirmos_causal_core::ucm::invgamma_rvs(p.nu / 2., p.nu / 2., &mut gamma);
            if !mixing.is_finite() || mixing <= 0. {
                return Err(Error::TailUnderflow);
            }
            let value = state + (p.variance.value() * mixing).sqrt() * rng.standard_normal();
            if !value.is_finite() {
                return Err(Error::NonFinite);
            }
            values.push(value);
        }
        Ok(values)
    }
}
