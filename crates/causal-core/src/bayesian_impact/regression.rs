//! Fixed-inclusion conditional from TFP 0.25's DynamicSpikeSlabSampler.
//! This is a conditional kernel, not a complete CausalImpact sampler.
use crate::lapack_cholesky::{dpotrf, dpotrs, Triangle};
use nalgebra::{DMatrix, DVector};

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    InvalidWindow,
    Singular,
    InvalidScale,
}

pub struct Regression {
    observed_x: DMatrix<f64>,
    precision: DMatrix<f64>,
    factor: Vec<f64>,
}

pub struct Conditional {
    pub mean: Vec<f64>,
    pub precision: DMatrix<f64>,
    pub concentration: f64,
    pub scale: f64,
}

impl Regression {
    /// Already-standardized controls; the prior uses the entire prediction window.
    pub fn new(controls: &DMatrix<f64>, pre: usize) -> Result<Self, Error> {
        if controls.ncols() == 0 || controls.nrows() == 0 {
            return Err(Error::Shape);
        }
        if pre < 2 || pre > controls.nrows() {
            return Err(Error::InvalidWindow);
        }
        if controls.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let observed_x = controls.rows(0, pre).into_owned();
        let all_xtx = controls.transpose() * controls;
        let p = controls.ncols();
        let prior = DMatrix::from_fn(p, p, |i, j| {
            0.01 * all_xtx[(i, j)] * if i == j { 1.0 } else { 0.5 } / controls.nrows() as f64
        });
        let mut prior_factor = prior.as_slice().to_vec();
        if dpotrf(Triangle::Lower, p, &mut prior_factor, p).map_err(|_| Error::Shape)? != 0 {
            return Err(Error::Singular);
        }
        let precision = observed_x.transpose() * &observed_x + prior;
        if precision.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let mut factor = precision.as_slice().to_vec();
        if dpotrf(Triangle::Lower, p, &mut factor, p).map_err(|_| Error::Shape)? != 0 {
            return Err(Error::Singular);
        }
        Ok(Self {
            observed_x,
            precision,
            factor,
        })
    }

    pub fn conditional(&self, residuals: &[f64]) -> Result<Conditional, Error> {
        if residuals.len() != self.observed_x.nrows() {
            return Err(Error::Shape);
        }
        if residuals.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let xty = self.observed_x.transpose() * DVector::from_column_slice(residuals);
        let mut mean = xty.as_slice().to_vec();
        let p = mean.len();
        dpotrs(Triangle::Lower, p, 1, &self.factor, p, &mut mean, p).map_err(|_| Error::Shape)?;
        let scale = 5.0
            + (residuals.iter().map(|v| v * v).sum::<f64>()
                - mean.iter().zip(xty.iter()).map(|(a, b)| a * b).sum::<f64>())
                / 2.0;
        if !scale.is_finite() || scale <= 0.0 {
            return Err(Error::InvalidScale);
        }
        Ok(Conditional {
            mean,
            precision: self.precision.clone(),
            concentration: 25.0 + residuals.len() as f64 / 2.0,
            scale,
        })
    }
}
