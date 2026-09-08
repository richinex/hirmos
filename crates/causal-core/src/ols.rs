//! Statsmodels OLS through its default SVD pseudoinverse fit.

use nalgebra::{DMatrix, DVector};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OlsError {
    EmptySample,
    RowMismatch,
    NonFiniteValue,
    DecompositionFailed,
}

#[derive(Debug)]
pub struct Ols {
    pub params: DVector<f64>,
    pub resid: DVector<f64>,
    pub ssr: f64,
    pub nobs: usize,
    pub k: usize,
    pub rank: usize,
    pub singular_values: DVector<f64>,
    normalized_cov_params: DMatrix<f64>,
}

impl Ols {
    /// `x` is nobs rows by k columns, column-major flat storage as nalgebra expects.
    pub fn fit(x: &DMatrix<f64>, y: &DVector<f64>) -> Self {
        Self::try_fit(x, y).expect("statsmodels OLS input and decomposition")
    }

    /// Statsmodels `OLS.fit(method="pinv")`, including minimum-norm rank-deficient fits.
    pub fn try_fit(x: &DMatrix<f64>, y: &DVector<f64>) -> Result<Self, OlsError> {
        let (nobs, k) = (x.nrows(), x.ncols());
        if nobs == 0 {
            return Err(OlsError::EmptySample);
        }
        if y.len() != nobs {
            return Err(OlsError::RowMismatch);
        }
        if x.iter().chain(y.iter()).any(|value| !value.is_finite()) {
            return Err(OlsError::NonFiniteValue);
        }
        let decomposition =
            crate::linalg::pseudo_inverse(x, 1e-15).map_err(|_| OlsError::DecompositionFailed)?;
        let params = &decomposition.matrix * y;
        let resid = y - x * &params;
        let ssr = resid.dot(&resid);
        let maximum = decomposition.singular_values.get(0).copied().unwrap_or(0.0);
        let rank_cutoff = nobs.max(k) as f64 * f64::EPSILON * maximum;
        let rank = decomposition
            .singular_values
            .iter()
            .filter(|value| **value > rank_cutoff)
            .count();
        let normalized_cov_params = &decomposition.matrix * decomposition.matrix.transpose();
        Ok(Ols {
            params,
            resid,
            ssr,
            nobs,
            k,
            rank,
            singular_values: decomposition.singular_values,
            normalized_cov_params,
        })
    }

    /// statsmodels: llf with the ML variance ssr/nobs.
    pub fn llf(&self) -> f64 {
        let n = self.nobs as f64;
        -n / 2.0 * ((2.0 * std::f64::consts::PI).ln() + (self.ssr / n).ln() + 1.0)
    }

    pub fn aic(&self) -> f64 {
        -2.0 * self.llf() + 2.0 * self.rank as f64
    }

    /// Statsmodels `normalized_cov_params = pinv(X) pinv(X)'`.
    pub fn xtx_inverse(&self) -> DMatrix<f64> {
        self.normalized_cov_params.clone()
    }

    /// Classic t statistics with `df_resid = nobs - rank`.
    pub fn tvalues(&self) -> DVector<f64> {
        let sigma2 = self.ssr / (self.nobs - self.rank) as f64;
        DVector::from_iterator(
            self.k,
            (0..self.k)
                .map(|i| self.params[i] / (sigma2 * self.normalized_cov_params[(i, i)]).sqrt()),
        )
    }
}
