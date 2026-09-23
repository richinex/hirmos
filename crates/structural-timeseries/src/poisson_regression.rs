//! BOOM 0.9.16 auxiliary-mixture Poisson regression and SpikeSlabSampler.
//! Source copyright Steven L. Scott / Google LLC, LGPL-2.1-or-later.
//! Each chain owns its source-ordered mixture table and random stream.
use crate::{
    poisson::{Imputer, Observation},
    sparse_ar::{Kernel, Selection, UnscaledSlab},
    weighted::Statistics,
    Error,
};
use hirmos_causal_core::nprandom::NpRng;
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Debug, PartialEq)]
pub struct Draw {
    pub coefficients: DVector<f64>,
    pub included: Vec<bool>,
}

#[derive(Clone)]
pub struct Regression {
    x: DMatrix<f64>,
    observations: Vec<Observation>,
    slab: UnscaledSlab,
    selection: Selection,
    parameters: Draw,
    rng: NpRng,
    imputer: Imputer,
}

impl Regression {
    /// The slab is unscaled. Its Gaussian residual prior is never used:
    /// Poisson has no residual-variance parameter or variance-sampling step.
    pub fn new(
        x: DMatrix<f64>,
        observations: Vec<Observation>,
        slab: UnscaledSlab,
        selection: Selection,
        seed: u64,
    ) -> Result<Self, Error> {
        if observations.is_empty() {
            return Err(Error::Empty);
        }
        if x.nrows() != observations.len() || x.ncols() != slab.parameters().mean().len() {
            return Err(Error::Shape);
        }
        if x.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let parameters = Draw {
            coefficients: DVector::zeros(x.ncols()),
            included: slab
                .parameters()
                .inclusion_probabilities()
                .iter()
                .map(|p| *p > 0.)
                .collect(),
        };
        Ok(Self {
            x,
            observations,
            slab,
            selection,
            parameters,
            rng: NpRng::seeded(seed),
            imputer: Imputer::default(),
        })
    }

    pub fn parameters(&self) -> &Draw {
        &self.parameters
    }

    /// Commit coefficients, inclusion and RNG together after a complete sweep.
    pub fn step(&mut self) -> Result<Draw, Error> {
        let mut next = self.clone();
        let stats = next.augment()?;
        let kernel = Kernel::from_weighted(&stats, next.slab.clone())?;
        kernel.select(
            &mut next.parameters.included,
            1.,
            next.selection,
            &mut next.rng,
        )?;
        next.parameters.coefficients = DVector::from_vec(
            kernel
                .conditional(&next.parameters.included, 1.)?
                .draw(&mut next.rng)?,
        );
        let draw = next.parameters.clone();
        *self = next;
        Ok(draw)
    }

    fn augment(&mut self) -> Result<Statistics, Error> {
        // Preserve the source accumulation order: internal, then external,
        // for each row. Combining them first changes rounding of X'WX/X'Wy.
        let mut rows = Vec::with_capacity(2 * self.x.nrows());
        let mut response = Vec::with_capacity(rows.capacity());
        let mut weights = Vec::with_capacity(rows.capacity());
        let eta = &self.x * &self.parameters.coefficients;
        for (i, value) in self.observations.iter().enumerate() {
            let draw = self.imputer.impute(*value, eta[i], &mut self.rng)?;
            for latent in draw
                .internal
                .into_iter()
                .chain(std::iter::once(draw.external))
            {
                rows.push(i);
                response.push(latent.negative_log_time - latent.mean);
                weights.push(latent.precision);
            }
        }
        let design = DMatrix::from_fn(rows.len(), self.x.ncols(), |i, j| self.x[(rows[i], j)]);
        Statistics::new(&design, &response, &weights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        defaults::Scale,
        regression::{FlipSweep, Inclusion, Slab},
    };
    fn model() -> Regression {
        let slab = Slab::new(
            DVector::zeros(1),
            DMatrix::identity(1, 1),
            vec![Inclusion::new(1.).unwrap()],
            Scale::new(1.).unwrap(),
            1.,
        )
        .unwrap();
        Regression::new(
            DMatrix::from_element(3, 1, 1.),
            vec![Observation::new(2, 1.).unwrap(); 3],
            UnscaledSlab::new(slab),
            Selection::Sweep(FlipSweep::All),
            73,
        )
        .unwrap()
    }
    #[test]
    fn failed_augmentation_preserves_the_entire_chain() {
        let mut a = model();
        for _ in 0..10 {
            a.step().unwrap();
        }
        let mut control = a.clone();
        a.observations[0] = Observation::new(499, 1.).unwrap();
        control.observations[0] = a.observations[0];
        a.x[(2, 0)] = f64::NAN;
        assert_eq!(a.step(), Err(Error::NonFinite));
        assert_eq!(a.parameters, control.parameters);
        assert_eq!(a.imputer.table().len(), 244);
        a.x[(2, 0)] = 1.;
        for _ in 0..20 {
            assert_eq!(a.step().unwrap(), control.step().unwrap());
        }
    }
}
