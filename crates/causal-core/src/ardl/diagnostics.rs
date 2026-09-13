//! Diagnostics evaluated on the fitted error-correction sample.
use super::{multivariate::Error, try_ols, Uecm};
use nalgebra::DMatrix;

pub struct SerialCorrelation {
    pub statistic: f64,
    pub p_value: f64,
    pub order: usize,
}

impl Uecm {
    /// lmtest::bgtest(type="Chisq", fill=0). Initial missing residual lags
    /// are zero-filled; the original estimation rows are not dropped.
    pub fn serial_correlation(&self, order: usize) -> Result<SerialCorrelation, Error> {
        let n = self.design.nrows();
        let k = self.design.ncols();
        if order == 0 || order >= n || order >= n.saturating_sub(k) {
            return Err(Error::InsufficientRows);
        }
        let x = DMatrix::from_fn(n, k + order, |row, column| {
            if column < k {
                return self.design[(row, column)];
            }
            let lag = column - k + 1;
            row.checked_sub(lag).map_or(0.0, |i| self.fit.resid[i])
        });
        let auxiliary = try_ols(&x, &self.fit.resid).map_err(Error::Regression)?;
        let fitted = &x * &auxiliary.params;
        let statistic = n as f64 * fitted.norm_squared() / self.fit.resid.norm_squared();
        if !statistic.is_finite() {
            return Err(Error::NonFiniteResult);
        }
        Ok(SerialCorrelation {
            statistic,
            p_value: crate::tsdiag::chi2_sf(statistic, order as f64),
            order,
        })
    }

    /// The authors maximize log-likelihood minus the coefficient-count penalty.
    /// This is not the usual minimized AIC/BIC display convention.
    pub fn pss_information_criteria(&self) -> Result<(f64, f64), Error> {
        let n = self.fit.nobs as f64;
        let k = self.fit.params.len() as f64;
        let sigma2 = self.fit.resid.norm_squared() / n;
        let log_likelihood = -0.5 * n * ((2.0 * std::f64::consts::PI * sigma2).ln() + 1.0);
        if !log_likelihood.is_finite() {
            return Err(Error::NonFiniteResult);
        }
        Ok((log_likelihood - k, log_likelihood - 0.5 * k * n.ln()))
    }
}
