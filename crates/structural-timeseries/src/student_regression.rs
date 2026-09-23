//! BOOM TRegressionSampler and its fixed-variance spike-and-slab subclass.
//! Copyright Steven L. Scott / Google LLC; LGPL-2.1-or-later.
use crate::{
    defaults::Scale,
    initial::Initial,
    prior::{Prior, Statistics as Squares},
    slice::{Positive, Shape},
    sparse_ar::{Kernel, Selection, UnscaledSlab},
    state::Variance,
    student::UniformTail,
    weighted::Statistics,
    Error,
};
use hirmos_causal_core::nprandom::{Mt19937, NpRng};
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Debug)]
pub struct Draw {
    pub coefficients: DVector<f64>,
    pub variance: Variance,
    pub nu: f64,
    pub included: Vec<bool>,
}

#[derive(Clone)]
enum CoefficientPrior {
    Fixed {
        mean: DVector<f64>,
        precision: DMatrix<f64>,
    },
    Selected {
        slab: UnscaledSlab,
        selection: Selection,
    },
}

#[derive(Clone)]
pub struct Regression {
    x: DMatrix<f64>,
    y: DVector<f64>,
    coefficient_prior: CoefficientPrior,
    prior: Prior,
    tail: UniformTail,
    parameters: Draw,
    slice: Positive,
    rng: NpRng,
    gamma: Mt19937,
}
impl Regression {
    pub fn new_selected(
        x: DMatrix<f64>,
        y: DVector<f64>,
        slab: UnscaledSlab,
        selection: Selection,
        tail: UniformTail,
        seed: u32,
    ) -> Result<Self, Error> {
        let p = slab.parameters();
        let mut model = Self::new(
            x,
            y,
            p.mean().clone(),
            p.precision().clone(),
            p.residual_prior(),
            tail,
            seed,
        )?;
        model.parameters.included = p
            .inclusion_probabilities()
            .iter()
            .map(|p| *p > 0.)
            .collect();
        model.coefficient_prior = CoefficientPrior::Selected { slab, selection };
        Ok(model)
    }
    pub fn new(
        x: DMatrix<f64>,
        y: DVector<f64>,
        mean: DVector<f64>,
        precision: DMatrix<f64>,
        prior: Prior,
        tail: UniformTail,
        seed: u32,
    ) -> Result<Self, Error> {
        if y.is_empty() {
            return Err(Error::Empty);
        }
        let variance = Scale::new(1.)?.variance();
        let stats = Statistics::new(&x, y.as_slice(), &vec![1.; y.len()])?;
        stats.coefficient_conditional(&mean, &precision, variance)?;
        // TRegressionModel starts at nu=30, sigma=1 and zero coefficients.
        if !tail.log_density(30.).is_finite() {
            return Err(Error::InvalidScale);
        }
        let coefficients = DVector::zeros(x.ncols());
        let included = vec![true; x.ncols()];
        Ok(Self {
            x,
            y,
            coefficient_prior: CoefficientPrior::Fixed { mean, precision },
            prior,
            tail,
            parameters: Draw {
                coefficients,
                variance,
                nu: 30.,
                included,
            },
            slice: Positive::new(Shape::General),
            rng: NpRng::seeded(seed as u64),
            gamma: Mt19937::seeded(seed),
        })
    }
    pub fn step(&mut self) -> Result<Draw, Error> {
        let mut next = self.clone();
        next.sweep()?;
        let draw = next.parameters.clone();
        *self = next;
        Ok(draw)
    }
    pub(crate) fn parameters(&self) -> &Draw {
        &self.parameters
    }
    /// State-space samplers own latent weights and update responses after
    /// subtracting the sampled state contribution. Do not impute weights here.
    pub(crate) fn conditioned_step(
        &mut self,
        response: DVector<f64>,
        weights: &[f64],
    ) -> Result<Draw, Error> {
        let stats = Statistics::new(&self.x, response.as_slice(), weights)?;
        let mut next = self.clone();
        next.y = response;
        next.draw_parameters(&stats)?;
        let result = next.parameters.clone();
        *self = next;
        Ok(result)
    }
    fn sweep(&mut self) -> Result<(), Error> {
        let residuals = &self.y - &self.x * &self.parameters.coefficients;
        let mut weights = Vec::with_capacity(self.y.len());
        for r in residuals.iter() {
            let shape = (self.parameters.nu + 1.) / 2.;
            let rate = (self.parameters.nu + r * r / self.parameters.variance.value()) / 2.;
            if !rate.is_finite() {
                return Err(Error::NonFinite);
            }
            let w = 1. / hirmos_causal_core::ucm::invgamma_rvs(shape, rate, &mut self.gamma);
            if !w.is_finite() || w <= 0. {
                return Err(Error::TailUnderflow);
            }
            weights.push(w);
        }
        let stats = Statistics::new(&self.x, self.y.as_slice(), &weights)?;
        self.draw_parameters(&stats)
    }
    fn draw_parameters(&mut self, stats: &Statistics) -> Result<(), Error> {
        self.parameters.coefficients = match &self.coefficient_prior {
            CoefficientPrior::Fixed { mean, precision } => {
                let (mean, covariance) =
                    stats.coefficient_conditional(mean, precision, self.parameters.variance)?;
                Initial::new(mean, covariance)?.draw(&mut self.rng)
            }
            CoefficientPrior::Selected { slab, selection } => {
                let kernel = Kernel::from_weighted(&stats, slab.clone())?;
                kernel.select(
                    &mut self.parameters.included,
                    self.parameters.variance.value(),
                    *selection,
                    &mut self.rng,
                )?;
                DVector::from_vec(
                    kernel
                        .conditional(&self.parameters.included, self.parameters.variance.value())?
                        .draw(&mut self.rng)?,
                )
            }
        };
        self.parameters.variance = self
            .prior
            .conditional(Squares::from_summary(
                stats.rows(),
                stats.squared_errors(&self.parameters.coefficients)?,
            )?)?
            .draw(&mut self.gamma)?;
        let residuals = &self.y - &self.x * &self.parameters.coefficients;
        let variance = self.parameters.variance.value();
        if variance <= 0. {
            return Err(Error::InvalidScale);
        }
        let tail = self.tail;
        self.parameters.nu = self.slice.draw(self.parameters.nu, &mut self.rng, |nu| {
            let prior = tail.log_density(nu);
            if !prior.is_finite() {
                return prior;
            }
            let normalizer = spec_math::cephes64::lgam((nu + 1.) / 2.)
                - spec_math::cephes64::lgam(nu / 2.)
                - 0.5 * (nu * std::f64::consts::PI * variance).ln();
            residuals.iter().fold(prior, |sum, r| {
                sum + normalizer - (nu + 1.) / 2. * (r * r / (nu * variance)).ln_1p()
            })
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prior::Limit;
    #[test]
    fn failed_sweep_preserves_coefficients_both_rngs_and_slice_width() {
        let prior = Prior::new(Scale::new(1.).unwrap(), 4., Limit::Unbounded).unwrap();
        let mut model = Regression::new(
            DMatrix::from_element(10, 1, 1.),
            DVector::from_iterator(10, (0..10).map(|i| (i as f64).sin())),
            DVector::zeros(1),
            DMatrix::identity(1, 1),
            prior,
            UniformTail::new(1., 50.).unwrap(),
            71,
        )
        .unwrap();
        for _ in 0..10 {
            model.step().unwrap();
        }
        let mut control = model.clone();
        model.prior = Prior::new(Scale::new(1.).unwrap(), 4., Limit::Zero).unwrap();
        assert!(model.step().is_err());
        model.prior = prior;
        for _ in 0..20 {
            let a = model.step().unwrap();
            let b = control.step().unwrap();
            assert_eq!(a.coefficients, b.coefficients);
            assert_eq!(a.variance.value(), b.variance.value());
            assert_eq!(a.nu, b.nu);
        }
    }
}
