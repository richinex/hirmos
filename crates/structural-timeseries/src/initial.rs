//! Validated proper Gaussian initial states. Correlated covariance uses the
//! existing Hirmos LAPACK Cholesky, without jitter or eigenvalue clipping.
use crate::{
    lapack_cholesky::{dpotrf, Triangle},
    Error,
};
use hirmos_causal_core::nprandom::NpRng;
use nalgebra::{DMatrix, DVector};

#[derive(Clone)]
pub struct Initial {
    mean: DVector<f64>,
    covariance: DMatrix<f64>,
    factor: DMatrix<f64>,
}
impl Initial {
    pub fn new(mean: DVector<f64>, covariance: DMatrix<f64>) -> Result<Self, Error> {
        let n = mean.len();
        if n == 0 || covariance.shape() != (n, n) {
            return Err(Error::Shape);
        }
        if mean.iter().chain(covariance.iter()).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        if covariance != covariance.transpose() {
            return Err(Error::InvalidScale);
        }
        let diagonal = (0..n).all(|i| (0..n).all(|j| i == j || covariance[(i, j)] == 0.0));
        let mut factor = covariance.clone();
        if diagonal {
            for i in 0..n {
                if factor[(i, i)] < 0.0 {
                    return Err(Error::InvalidScale);
                }
                factor[(i, i)] = factor[(i, i)].sqrt();
            }
        } else {
            if dpotrf(Triangle::Lower, n, factor.as_mut_slice(), n).map_err(|_| Error::Shape)? != 0
            {
                return Err(Error::Singular);
            }
            for i in 0..n {
                for j in i + 1..n {
                    factor[(i, j)] = 0.0;
                }
            }
        }
        Ok(Self {
            mean,
            covariance,
            factor,
        })
    }
    pub fn dimension(&self) -> usize {
        self.mean.len()
    }
    /// Refresh a model parameter represented by a deterministic state coordinate.
    /// Never turns a random initial coordinate into a fixed one implicitly.
    pub(crate) fn with_known_value(mut self, index: usize, value: f64) -> Result<Self, Error> {
        if !value.is_finite() {
            return Err(Error::NonFinite);
        }
        self.known_value(index)?;
        self.mean[index] = value;
        Ok(self)
    }
    pub(crate) fn known_value(&self, index: usize) -> Result<f64, Error> {
        if index >= self.dimension() {
            return Err(Error::Shape);
        }
        if self.covariance.row(index).iter().any(|v| *v != 0.) {
            return Err(Error::InvalidScale);
        }
        Ok(self.mean[index])
    }
    pub fn moments(&self) -> (DVector<f64>, DMatrix<f64>) {
        (self.mean.clone(), self.covariance.clone())
    }
    pub fn draw(&self, rng: &mut NpRng) -> DVector<f64> {
        let z = DVector::from_fn(self.dimension(), |_, _| rng.standard_normal());
        &self.mean + &self.factor * z
    }
}
