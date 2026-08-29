//! Least squares matching statsmodels OLS: params via QR, gaussian log-likelihood, AIC, t-values.

use nalgebra::{DMatrix, DVector};

pub struct Ols {
    pub params: DVector<f64>,
    pub resid: DVector<f64>,
    pub ssr: f64,
    pub nobs: usize,
    pub k: usize,
    r: DMatrix<f64>,
}

impl Ols {
    /// `x` is nobs rows by k columns, column-major flat storage as nalgebra expects.
    pub fn fit(x: &DMatrix<f64>, y: &DVector<f64>) -> Self {
        let (nobs, k) = (x.nrows(), x.ncols());
        let qr = x.clone().qr();
        let q = qr.q();
        let r = qr.r();
        let qty = q.transpose() * y;
        let params = r
            .solve_upper_triangular(&qty)
            .expect("design matrix is rank deficient");
        let resid = y - x * &params;
        let ssr = resid.dot(&resid);
        Ols {
            params,
            resid,
            ssr,
            nobs,
            k,
            r,
        }
    }

    /// statsmodels: llf with the ML variance ssr/nobs.
    pub fn llf(&self) -> f64 {
        let n = self.nobs as f64;
        -n / 2.0 * ((2.0 * std::f64::consts::PI).ln() + (self.ssr / n).ln() + 1.0)
    }

    pub fn aic(&self) -> f64 {
        -2.0 * self.llf() + 2.0 * self.k as f64
    }

    /// (X'X)^-1 as R^-1 R^-T from the QR factor.
    pub fn xtx_inverse(&self) -> DMatrix<f64> {
        let identity = DMatrix::<f64>::identity(self.k, self.k);
        let rinv = self
            .r
            .solve_upper_triangular(&identity)
            .expect("triangular solve");
        &rinv * rinv.transpose()
    }

    /// t statistics: params over standard errors from sigma2 (X'X)^-1, sigma2 = ssr/(n-k).
    pub fn tvalues(&self) -> DVector<f64> {
        let sigma2 = self.ssr / (self.nobs - self.k) as f64;
        // (X'X)^-1 = R^-1 R^-T, so its diagonal is the squared row norms of R^-1.
        let identity = DMatrix::<f64>::identity(self.k, self.k);
        let rinv = self
            .r
            .solve_upper_triangular(&identity)
            .expect("triangular solve");
        DVector::from_iterator(
            self.k,
            (0..self.k).map(|i| {
                let diag: f64 = (0..self.k).map(|j| rinv[(i, j)] * rinv[(i, j)]).sum();
                self.params[i] / (sigma2 * diag).sqrt()
            }),
        )
    }
}
