//! BOOM WeightedRegSuf and TRegressionSampler coefficient conditional.
//! Copyright Steven L. Scott / Google LLC; LGPL-2.1-or-later.
use crate::{
    regression::{factor, solve},
    state::Variance,
    Error,
};
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Debug)]
pub struct Statistics {
    xtx: DMatrix<f64>,
    xty: DVector<f64>,
    yty: f64,
    n: usize,
    sum_weights: f64,
    sum_log_weights: f64,
}
impl Statistics {
    pub fn new(x: &DMatrix<f64>, y: &[f64], weights: &[f64]) -> Result<Self, Error> {
        if x.ncols() == 0 || x.nrows() != y.len() || y.len() != weights.len() {
            return Err(Error::Shape);
        }
        if x.iter().chain(y).chain(weights).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        if weights.iter().any(|v| *v <= 0.) {
            return Err(Error::InvalidScale);
        }
        let p = x.ncols();
        let mut out = Self {
            xtx: DMatrix::zeros(p, p),
            xty: DVector::zeros(p),
            yty: 0.,
            n: y.len(),
            sum_weights: 0.,
            sum_log_weights: 0.,
        };
        for i in 0..y.len() {
            let w = weights[i];
            out.yty += w * y[i] * y[i];
            out.sum_weights += w;
            out.sum_log_weights += w.ln();
            for j in 0..p {
                out.xty[j] += w * y[i] * x[(i, j)];
                for k in 0..=j {
                    out.xtx[(k, j)] += w * x[(i, j)] * x[(i, k)];
                }
            }
        }
        for j in 0..p {
            for k in 0..j {
                out.xtx[(j, k)] = out.xtx[(k, j)];
            }
        }
        if out
            .xtx
            .iter()
            .chain(out.xty.iter())
            .chain([&out.yty, &out.sum_weights, &out.sum_log_weights])
            .any(|v| !v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(out)
    }
    pub fn xtx(&self) -> &DMatrix<f64> {
        &self.xtx
    }
    pub fn xty(&self) -> &DVector<f64> {
        &self.xty
    }
    pub fn yty(&self) -> f64 {
        self.yty
    }
    pub fn rows(&self) -> usize {
        self.n
    }
    pub fn sum_weights(&self) -> f64 {
        self.sum_weights
    }
    pub fn sum_log_weights(&self) -> f64 {
        self.sum_log_weights
    }
    pub fn squared_errors(&self, beta: &DVector<f64>) -> Result<f64, Error> {
        if beta.len() != self.xty.len() {
            return Err(Error::Shape);
        }
        if beta.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        // Preserve BOOM's sufficient-statistic expression, including cancellation.
        let value = beta.dot(&(&self.xtx * beta)) - 2. * beta.dot(&self.xty) + self.yty;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(Error::NonFinite)
        }
    }
    /// This is TRegressionSampler's fixed-covariance coefficient prior, not
    /// BregVsSampler's variance-integrated model-selection weight.
    pub fn coefficient_conditional(
        &self,
        mean: &DVector<f64>,
        precision: &DMatrix<f64>,
        variance: Variance,
    ) -> Result<(DVector<f64>, DMatrix<f64>), Error> {
        let p = self.xty.len();
        if mean.len() != p || precision.shape() != (p, p) {
            return Err(Error::Shape);
        }
        if mean.iter().chain(precision.iter()).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        if *precision != precision.transpose() {
            return Err(Error::InvalidScale);
        }
        factor(precision)?;
        let posterior = precision + &self.xtx / variance.value();
        let rhs = precision * mean + &self.xty / variance.value();
        if posterior.iter().chain(rhs.iter()).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let chol = factor(&posterior)?;
        let mut covariance = solve(&chol, DMatrix::identity(p, p))?;
        // A symmetric matrix owns one triangle. Mirror the LAPACK lower
        // triangle rather than treating independently rounded solves as data.
        for j in 0..p {
            for i in 0..j {
                covariance[(i, j)] = covariance[(j, i)];
            }
        }
        let mu = solve(&chol, DMatrix::from_column_slice(p, 1, rhs.as_slice()))?;
        if covariance.iter().chain(mu.iter()).any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        Ok((mu.column(0).into_owned(), covariance))
    }
}
