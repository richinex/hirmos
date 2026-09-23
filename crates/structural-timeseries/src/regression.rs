//! BOOM 0.9.16 BregVsSampler conjugate regression and SSVS flip sweep.
//! Copyright Google LLC and Steven L. Scott; LGPL-2.1-or-later, vendor sources.
//! Uses the reference's model weights, including its (DF/2 - 1) exponent.
use crate::{
    defaults::Scale,
    lapack_cholesky::{dpotrf, dpotrs, Triangle},
    prior::{Limit, Prior, Statistics},
    Error,
};
use crate::random::Random;
use nalgebra::{DMatrix, DVector};
mod oda;
mod swaps;
pub use oda::OdaOptions;

#[derive(Clone, Copy)]
pub enum Sampling {
    Ssvs(FlipSweep),
    Oda(OdaOptions),
}

#[derive(Clone, Copy)]
pub struct Inclusion(f64);
impl Inclusion {
    pub fn new(probability: f64) -> Result<Self, Error> {
        if !probability.is_finite() || !(0.0..=1.0).contains(&probability) {
            return Err(Error::InvalidProbability);
        }
        Ok(Self(probability))
    }
}

#[derive(Clone)]
pub struct Slab {
    mean: DVector<f64>,
    precision: DMatrix<f64>,
    inclusion: Vec<Inclusion>,
    residual: Prior,
}
pub(crate) fn factor(matrix: &DMatrix<f64>) -> Result<DMatrix<f64>, Error> {
    let n = matrix.nrows();
    let mut out = matrix.clone();
    if n > 0 && dpotrf(Triangle::Lower, n, out.as_mut_slice(), n).map_err(|_| Error::Shape)? != 0 {
        return Err(Error::Singular);
    }
    Ok(out)
}
fn logdet(factor: &DMatrix<f64>) -> f64 {
    2.0 * factor.diagonal().iter().map(|x| x.ln()).sum::<f64>()
}
pub(crate) fn solve(factor: &DMatrix<f64>, mut rhs: DMatrix<f64>) -> Result<DMatrix<f64>, Error> {
    let n = factor.nrows();
    if n > 0 {
        dpotrs(
            Triangle::Lower,
            n,
            rhs.ncols(),
            factor.as_slice(),
            n,
            rhs.as_mut_slice(),
            n,
        )
        .map_err(|_| Error::Shape)?;
    }
    Ok(rhs)
}
impl Slab {
    /// BOOM truncates the residual-scale draw without changing SSVS weights.
    pub fn with_residual_ceiling(mut self, ceiling: Scale) -> Self {
        self.residual = self.residual.with_ceiling(ceiling);
        self
    }
    pub fn mean(&self) -> &DVector<f64> {
        &self.mean
    }
    pub fn precision(&self) -> &DMatrix<f64> {
        &self.precision
    }
    pub fn inclusion_probabilities(&self) -> Vec<f64> {
        self.inclusion.iter().map(|p| p.0).collect()
    }
    pub fn residual_prior(&self) -> Prior {
        self.residual
    }
    pub fn new(
        mean: DVector<f64>,
        precision: DMatrix<f64>,
        inclusion: Vec<Inclusion>,
        sigma_guess: Scale,
        df: f64,
    ) -> Result<Self, Error> {
        Self::with_prior(
            mean,
            precision,
            inclusion,
            Prior::new(sigma_guess, df, Limit::Unbounded)?,
        )
    }
    pub(crate) fn with_prior(
        mean: DVector<f64>,
        precision: DMatrix<f64>,
        inclusion: Vec<Inclusion>,
        residual: Prior,
    ) -> Result<Self, Error> {
        let n = mean.len();
        if n == 0 || precision.shape() != (n, n) || inclusion.len() != n {
            return Err(Error::Shape);
        }
        if mean.iter().chain(precision.iter()).any(|x| !x.is_finite()) {
            return Err(Error::NonFinite);
        }
        if precision != precision.transpose() {
            return Err(Error::InvalidScale);
        }
        factor(&precision)?;
        Ok(Self {
            mean,
            precision,
            inclusion,
            residual,
        })
    }
}

#[derive(Clone)]
pub struct Regression {
    x: DMatrix<f64>,
    y: DVector<f64>,
    prior: Slab,
    included: Vec<bool>,
    rng: Random,
    order: Vec<usize>,
    coefficients: DVector<f64>,
    flips: FlipSweep,
    algorithm: Algorithm,
    variance: f64,
    swaps: swaps::Correlations,
}
#[derive(Clone)]
enum Algorithm {
    Ssvs,
    Oda(oda::Augmentation),
}
#[derive(Clone, Copy)]
pub enum FlipSweep {
    All,
    AtMost(std::num::NonZeroUsize),
}
pub struct Conditional {
    pub mean: DVector<f64>,
    pub precision: DMatrix<f64>,
    pub residual_squares: f64,
    pub log_weight: f64,
    indices: Vec<usize>,
}
#[derive(Clone)]
pub struct Draw {
    pub coefficients: Vec<f64>,
    pub included: Vec<bool>,
    pub variance: f64,
}
impl Regression {
    pub fn with_sampling(mut self, sampling: Sampling) -> Result<Self, Error> {
        match sampling {
            Sampling::Ssvs(sweep) => {
                self.algorithm = Algorithm::Ssvs;
                Ok(self.with_flip_sweep(sweep))
            }
            Sampling::Oda(options) => self.with_oda(options),
        }
    }
    pub fn new(x: DMatrix<f64>, y: DVector<f64>, prior: Slab, seed: u32) -> Result<Self, Error> {
        if x.nrows() == 0 || x.nrows() != y.len() || x.ncols() != prior.mean.len() {
            return Err(Error::Shape);
        }
        if x.iter().chain(y.iter()).any(|x| !x.is_finite()) {
            return Err(Error::NonFinite);
        }
        let included = prior.inclusion.iter().map(|p| p.0 == 1.0).collect();
        let coefficients = DVector::zeros(x.ncols());
        let swaps = swaps::Correlations::new(&x, 0.8)?;
        let order = (0..x.ncols()).collect();
        Ok(Self {
            x,
            y,
            prior,
            included,
            rng: Random::new(seed as u64),
            order,
            coefficients,
            flips: FlipSweep::All,
            algorithm: Algorithm::Ssvs,
            variance: 1.0,
            swaps,
        })
    }
    pub fn with_flip_sweep(mut self, flips: FlipSweep) -> Self {
        self.flips = flips;
        self
    }
    pub(crate) fn set_response(&mut self, y: DVector<f64>) -> Result<(), Error> {
        if y.len() != self.y.len() {
            return Err(Error::Shape);
        }
        if y.iter().any(|x| !x.is_finite()) {
            return Err(Error::NonFinite);
        }
        self.y = y;
        Ok(())
    }
    pub fn predict(&self, x: &DMatrix<f64>) -> Result<DVector<f64>, Error> {
        if x.ncols() != self.coefficients.len() {
            return Err(Error::Shape);
        }
        if x.iter().any(|x| !x.is_finite()) {
            return Err(Error::NonFinite);
        }
        let result = x * &self.coefficients;
        if result.iter().any(|x| !x.is_finite()) {
            return Err(Error::NonFinite);
        }
        Ok(result)
    }
    pub fn conditional(&self, included: &[bool]) -> Result<Conditional, Error> {
        if included.len() != self.x.ncols() {
            return Err(Error::Shape);
        }
        let mut log_prior = 0.0;
        for (p, yes) in self.prior.inclusion.iter().zip(included) {
            log_prior += if *yes { p.0.ln() } else { (-p.0).ln_1p() };
        }
        let indices: Vec<_> = included
            .iter()
            .enumerate()
            .filter_map(|(i, yes)| yes.then_some(i))
            .collect();
        let k = indices.len();
        let x = DMatrix::from_fn(self.x.nrows(), k, |i, j| self.x[(i, indices[j])]);
        let prior_mean = DVector::from_fn(k, |i, _| self.prior.mean[indices[i]]);
        let prior_precision =
            DMatrix::from_fn(k, k, |i, j| self.prior.precision[(indices[i], indices[j])]);
        let precision = &prior_precision + x.transpose() * &x;
        let rhs = &prior_precision * &prior_mean + x.transpose() * &self.y;
        let chol = factor(&precision)?;
        let mean = solve(&chol, DMatrix::from_column_slice(k, 1, rhs.as_slice()))?
            .column(0)
            .into_owned();
        let residual = &self.y - &x * &mean;
        let difference = &mean - prior_mean;
        let residual_squares =
            residual.dot(&residual) + difference.dot(&(&prior_precision * &difference));
        let stats = Statistics::from_summary(self.y.len(), residual_squares)?;
        let variance = self.prior.residual.conditional(stats)?;
        let log_weight = log_prior + 0.5 * (logdet(&factor(&prior_precision)?) - logdet(&chol))
            - (variance.shape() - 1.0) * (2.0 * variance.scale()).ln();
        Ok(Conditional {
            mean,
            precision,
            residual_squares,
            log_weight,
            indices,
        })
    }
    pub fn step(&mut self) -> Result<Draw, Error> {
        match self.algorithm.clone() {
            Algorithm::Ssvs => self.step_ssvs(),
            Algorithm::Oda(augmentation) => {
                let mut next = self.clone();
                let draw = next.step_oda(&augmentation)?;
                *self = next;
                Ok(draw)
            }
        }
    }
    fn step_ssvs(&mut self) -> Result<Draw, Error> {
        let mut included = self.included.clone();
        let mut rng = self.rng.clone();
        let mut order = self.order.clone();
        rng.shuffle(&mut order);
        let mut current = self.conditional(&included)?;
        if !current.log_weight.is_finite() {
            return Err(Error::NonFinite);
        }
        let attempts = match self.flips {
            FlipSweep::All => order.len(),
            FlipSweep::AtMost(n) => n.get().min(order.len()),
        };
        for i in order.iter().copied().take(attempts) {
            included[i] = !included[i];
            let proposal = self.conditional(&included)?;
            let log_uniform = rng.next_f64().ln();
            if proposal.log_weight.is_finite()
                && log_uniform <= proposal.log_weight - current.log_weight
            {
                current = proposal;
            } else {
                included[i] = !included[i];
            }
        }
        self.attempt_swap(&mut included, &mut current, &mut rng)?;
        let variance = self
            .prior
            .residual
            .conditional(Statistics::from_summary(
                self.y.len(),
                current.residual_squares,
            )?)?
            .draw(&mut rng)?
            .value();
        let k = current.indices.len();
        let mut coefficients = vec![0.0; included.len()];
        if k > 0 {
            let beta = draw_coefficients(&current.mean, &current.precision, variance,
                || rng.standard_normal())?;
            for (i, j) in current.indices.iter().enumerate() {
                coefficients[*j] = beta[i];
            }
        }
        self.included = included.clone();
        self.rng = rng;
        self.order = order;
        self.coefficients = DVector::from_vec(coefficients.clone());
        self.variance = variance;
        Ok(Draw {
            coefficients,
            included,
            variance,
        })
    }
}

fn draw_coefficients(
    mean: &DVector<f64>,
    precision: &DMatrix<f64>,
    variance: f64,
    mut normal: impl FnMut() -> f64,
) -> Result<DVector<f64>, Error> {
    let lower = factor(&(precision / variance))?;
    let z = DVector::from_fn(mean.len(), |_, _| normal());
    let noise = lower
        .lower_triangle()
        .transpose()
        .solve_upper_triangular(&z)
        .ok_or(Error::Singular)?;
    Ok(mean + noise)
}

#[cfg(test)]
#[path = "tests/regression_stream.rs"]
mod stream_tests;
