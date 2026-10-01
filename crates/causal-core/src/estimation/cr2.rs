//! Cluster-robust CR2 inference under estimatr's identity working model.
//! Explicit full-rank design, including any fixed-effect dummy columns.
//! Oracle: estimatr 2.0.0, lm_variance. Alias selection is explicit opt-in;
//! fixed-effect absorption is not performed here.
use nalgebra::{DMatrix, DVector};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    AliasSelection(super::aliases::Error),
    Shape,
    NonFinite,
    InvalidWeights,
    InsufficientClusters,
    RankDeficient,
    Numerical,
    DegenerateInference,
}

#[derive(Debug)]
pub struct Fit {
    /// Only positive-weight observations and their clusters count toward inference.
    pub observations: usize,
    pub clusters: usize,
    pub params: Vec<f64>,
    pub covariance: DMatrix<f64>,
    pub degrees_of_freedom: Vec<f64>,
    pub residuals: Vec<f64>,
}

#[derive(Debug)]
pub struct RankSelectedFit {
    pub selection: super::aliases::Selection,
    /// Coefficients and covariance follow selection.kept(), never padded with zeros.
    pub fit: Fit,
}

/// Explicit opt-in to estimatr's normalized rank selection. The existing full-rank
/// entry point keeps refusing aliased inputs, so no old result changes silently.
pub fn fit_selecting_aliases(
    y: &[f64],
    x: &DMatrix<f64>,
    weights: &[f64],
    clusters: &[u64],
) -> Result<RankSelectedFit, Error> {
    if y.len() != x.nrows() {
        return Err(Error::Shape);
    }
    if y.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let selection =
        super::aliases::select_clustered(x, weights, clusters).map_err(Error::AliasSelection)?;
    if selection.kept().is_empty() {
        return Err(Error::RankDeficient);
    }
    let reduced = DMatrix::from_fn(x.nrows(), selection.kept().len(), |i, j| {
        x[(i, selection.kept()[j])]
    });
    let fit = fit(y, &reduced, weights, clusters)?;
    Ok(RankSelectedFit { selection, fit })
}

/// Nonnegative observation weights, one cluster id per row, no implied intercept.
/// Satterthwaite df are coefficient-specific, not residual or cluster df.
pub fn fit(y: &[f64], x: &DMatrix<f64>, weights: &[f64], clusters: &[u64]) -> Result<Fit, Error> {
    let (n, p) = x.shape();
    if p == 0 || y.len() != n || weights.len() != n || clusters.len() != n {
        return Err(Error::Shape);
    }
    if x.iter().chain(y).any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    if weights.iter().any(|v| !v.is_finite() || *v < 0.) {
        return Err(Error::InvalidWeights);
    }
    // estimatr excludes zero-weight observations from CR2 cluster blocks and df.
    let rows: Vec<usize> = (0..n).filter(|i| weights[*i] > 0.).collect();
    if rows.is_empty() {
        return Err(Error::InvalidWeights);
    }
    let xx = DMatrix::from_fn(rows.len(), p, |i, j| x[(rows[i], j)]);
    let yy: Vec<f64> = rows.iter().map(|i| y[*i]).collect();
    let ww: Vec<f64> = rows.iter().map(|i| weights[*i]).collect();
    let cc: Vec<u64> = rows.iter().map(|i| clusters[*i]).collect();
    let mut result = fit_positive(&yy, &xx, &ww, &cc)?;
    result.residuals = (0..n)
        .map(|i| y[i] - (0..p).map(|j| x[(i, j)] * result.params[j]).sum::<f64>())
        .collect();
    if result.residuals.iter().any(|v| !v.is_finite()) {
        return Err(Error::Numerical);
    }
    Ok(result)
}

fn fit_positive(
    y: &[f64],
    x: &DMatrix<f64>,
    weights: &[f64],
    clusters: &[u64],
) -> Result<Fit, Error> {
    let (n, p) = x.shape();
    if p == 0 || n <= p || y.len() != n || weights.len() != n || clusters.len() != n {
        return Err(Error::Shape);
    }
    if x.iter().chain(y).any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    if weights.iter().any(|v| !v.is_finite() || *v <= 0.) {
        return Err(Error::InvalidWeights);
    }
    let maxw = weights.iter().copied().fold(0., f64::max);
    let mean = weights.iter().map(|w| w / maxw).sum::<f64>() / n as f64;
    let w: Vec<f64> = weights.iter().map(|w| (w / maxw) / mean).collect();
    if w.iter().any(|v| !v.is_finite() || *v <= 0.) {
        return Err(Error::InvalidWeights);
    }
    let mut groups: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for (i, g) in clusters.iter().enumerate() {
        groups.entry(*g).or_default().push(i);
    }
    if groups.len() < 2 {
        return Err(Error::InsufficientClusters);
    }

    // Column rescaling is only numerical conditioning. Restore the original units below.
    let scale: Vec<f64> = (0..p)
        .map(|j| x.column(j).iter().fold(0_f64, |a, v| a.max(v.abs())))
        .collect();
    if scale.iter().any(|s| *s == 0.) {
        return Err(Error::RankDeficient);
    }
    let u = DMatrix::from_fn(n, p, |i, j| x[(i, j)] / scale[j]);
    let wx = DMatrix::from_fn(n, p, |i, j| u[(i, j)] * w[i]);
    let rootx = DMatrix::from_fn(n, p, |i, j| u[(i, j)] * w[i].sqrt());
    let rooty = DVector::from_iterator(n, y.iter().zip(&w).map(|(y, w)| y * w.sqrt()));
    let ols = crate::ols::Ols::try_fit(&rootx, &rooty).map_err(|_| Error::Numerical)?;
    if ols.rank != p {
        return Err(Error::RankDeficient);
    }
    let m = ols.xtx_inverse();
    let residuals = DVector::from_column_slice(y) - &u * &ols.params;
    let omega = &m * wx.transpose() * &wx * &m;
    let mut covariance = DMatrix::zeros(p, p);
    // Each row is the residual-adjusted influence of one cluster under the
    // identity working model. Its Gram matrix determines Satterthwaite df.
    let mut influences: Vec<DMatrix<f64>> =
        (0..p).map(|_| DMatrix::zeros(groups.len(), n)).collect();
    for (g, rows) in groups.values().enumerate() {
        let ng = rows.len();
        let ug = DMatrix::from_fn(ng, p, |i, j| u[(rows[i], j)]);
        let wg = DMatrix::from_fn(ng, p, |i, j| wx[(rows[i], j)]);
        let h = &ug * &m * wg.transpose();
        let b = DMatrix::identity(ng, ng) - &h - h.transpose() + &ug * &omega * ug.transpose();
        let e = crate::linalg::eigen_symmetric_lower(&b).map_err(|_| Error::Numerical)?;
        // Same generalized inverse square-root cutoff as estimatr::lm_variance.
        let diag = DVector::from_iterator(
            ng,
            e.values
                .iter()
                .map(|v| if *v > 1e-12 { 1. / v.sqrt() } else { 0. }),
        );
        let a = &e.vectors * DMatrix::from_diagonal(&diag) * e.vectors.transpose();
        let l = &m * wg.transpose() * a;
        let eg = DVector::from_iterator(ng, rows.iter().map(|i| residuals[*i]));
        let score = &l * eg;
        covariance += &score * score.transpose();
        let mut influence = -(&l * &ug * &m * wx.transpose());
        for (j, row) in rows.iter().enumerate() {
            for k in 0..p {
                influence[(k, *row)] += l[(k, j)];
            }
        }
        for k in 0..p {
            influences[k].row_mut(g).copy_from(&influence.row(k));
        }
    }
    let mut df = Vec::with_capacity(p);
    for influence in influences {
        let gram = &influence * influence.transpose();
        let trace = gram.trace();
        let denom = gram.norm_squared();
        if !denom.is_finite() || denom <= 0. {
            return Err(Error::DegenerateInference);
        }
        let value = trace * trace / denom;
        if !value.is_finite() || value <= 0. {
            return Err(Error::DegenerateInference);
        }
        df.push(value);
    }
    for i in 0..p {
        for j in 0..p {
            covariance[(i, j)] = covariance[(i, j)] / scale[i] / scale[j];
        }
    }
    let params: Vec<f64> = ols.params.iter().zip(&scale).map(|(b, s)| b / s).collect();
    if params
        .iter()
        .chain(covariance.iter())
        .chain(residuals.iter())
        .any(|v| !v.is_finite())
    {
        return Err(Error::Numerical);
    }
    Ok(Fit {
        observations: n,
        clusters: groups.len(),
        params,
        covariance,
        degrees_of_freedom: df,
        residuals: residuals.iter().copied().collect(),
    })
}
