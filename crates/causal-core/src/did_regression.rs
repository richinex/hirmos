//! Conventional two-period DiD with the chapter's classical regression inference.
//! Uses the existing SVD OLS implementation; does not assume independent errors are
//! appropriate merely because a panel contains two periods.

use crate::{ols::Ols, panel::PanelMatrices};
use nalgebra::{DMatrix, DVector};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidLayout,
    CovariateShape,
    NonFinite,
    RankDeficient,
    NoResidualDegreesOfFreedom,
    Decomposition,
}

#[derive(Debug)]
pub struct GroupMeans {
    pub before: f64,
    pub after: f64,
    pub change: f64,
}

#[derive(Debug)]
pub struct Summary {
    pub control: GroupMeans,
    pub treated: GroupMeans,
    pub difference: f64,
}

/// Average each observed group in the pre/post blocks of a validated balanced panel.
pub fn summarize(panel: &PanelMatrices) -> Result<Summary, Error> {
    let y = &panel.y;
    if panel.n0 == 0 || panel.n0 >= y.nrows() || panel.t0 == 0 || panel.t0 >= y.ncols() {
        return Err(Error::InvalidLayout);
    }
    if y.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let group = |start: usize, stop: usize| {
        let mean = |first: usize, last: usize| {
            let sum: f64 = (start..stop)
                .flat_map(|i| (first..last).map(move |t| y[(i, t)]))
                .sum();
            sum / ((stop - start) * (last - first)) as f64
        };
        let before = mean(0, panel.t0);
        let after = mean(panel.t0, y.ncols());
        GroupMeans {
            before,
            after,
            change: after - before,
        }
    };
    let control = group(0, panel.n0);
    let treated = group(panel.n0, y.nrows());
    let difference = treated.change - control.change;
    Ok(Summary {
        control,
        treated,
        difference,
    })
}

#[derive(Debug)]
pub struct Regression {
    /// Intercept, treated group, post period, interaction, then supplied covariates.
    pub coefficients: Vec<f64>,
    pub standard_errors: Vec<f64>,
    pub intervals: Vec<[f64; 2]>,
    pub residual_degrees_of_freedom: usize,
}

/// Fit Y ~ group * post + covariates. Covariates use unit-major, time-minor rows
/// in the validated panel's unit order. Restrict this chapter route to two periods;
/// multiple periods require an explicit time-effects specification.
pub fn fit(panel: &PanelMatrices, covariates: &DMatrix<f64>) -> Result<Regression, Error> {
    summarize(panel)?;
    let units = panel.y.nrows();
    let rows = 2 * units;
    if panel.y.ncols() != 2 || panel.t0 != 1 {
        return Err(Error::InvalidLayout);
    }
    if covariates.nrows() != rows {
        return Err(Error::CovariateShape);
    }
    if covariates.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let x = DMatrix::from_fn(rows, 4 + covariates.ncols(), |i, j| {
        let group = f64::from(i / 2 >= panel.n0);
        let post = (i % 2) as f64;
        match j {
            0 => 1.0,
            1 => group,
            2 => post,
            3 => group * post,
            _ => covariates[(i, j - 4)],
        }
    });
    let y = DVector::from_iterator(rows, (0..rows).map(|i| panel.y[(i / 2, i % 2)]));
    let fit = Ols::try_fit(&x, &y).map_err(|_| Error::Decomposition)?;
    if fit.rank != x.ncols() {
        return Err(Error::RankDeficient);
    }
    let df = rows
        .checked_sub(fit.rank)
        .filter(|df| *df > 0)
        .ok_or(Error::NoResidualDegreesOfFreedom)?;
    let covariance = fit.xtx_inverse() * (fit.ssr / df as f64);
    // Same inverse-incomplete-beta identity as the existing ARDL t quantile.
    let z = spec_math::cephes64::incbi(df as f64 / 2.0, 0.5, 0.05);
    let critical = (df as f64 * (1.0 - z) / z).sqrt();
    let standard_errors: Vec<_> = (0..fit.k).map(|j| covariance[(j, j)].sqrt()).collect();
    let intervals = fit
        .params
        .iter()
        .zip(&standard_errors)
        .map(|(b, se)| [b - critical * se, b + critical * se])
        .collect();
    Ok(Regression {
        coefficients: fit.params.as_slice().to_vec(),
        standard_errors,
        intervals,
        residual_degrees_of_freedom: df,
    })
}
