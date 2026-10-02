use nalgebra::{DMatrix, DVector};

pub mod inference;
pub mod cr2;
pub mod aliases;

fn norm_sf(x: f64) -> f64 {
    0.5 * libm::erfc(x / std::f64::consts::SQRT_2)
}

pub struct HacOls {
    /// Full coefficient covariance, in params order.
    pub covariance: DMatrix<f64>,
    pub params: Vec<f64>,
    pub bse: Vec<f64>,
    pub pvalues: Vec<f64>,
    /// 95% normal-based intervals as (lower, upper).
    pub conf_int: Vec<(f64, f64)>,
    pub resid: Vec<f64>,
    pub rsquared: f64,
}

/// OLS with cov_type="HAC": Bartlett weights, no small-sample correction, z-based inference.
pub fn ols_hac(y: &[f64], x: &DMatrix<f64>, maxlags: usize) -> HacOls {
    let n = x.nrows();
    let k = x.ncols();
    let yv = DVector::from_column_slice(y);
    let fit = crate::ols::Ols::fit(x, &yv);
    let xtx_inv = fit.xtx_inverse();
    let beta = fit.params;
    let resid: Vec<f64> = fit.resid.iter().copied().collect();

    // xu rows: exog scaled by the residual.
    let mut xu = DMatrix::<f64>::zeros(n, k);
    for i in 0..n {
        for j in 0..k {
            xu[(i, j)] = x[(i, j)] * resid[i];
        }
    }
    let mut s = xu.transpose() * &xu;
    for lag in 1..=maxlags {
        let w = 1.0 - lag as f64 / (maxlags as f64 + 1.0);
        let top = xu.rows(lag, n - lag).transpose() * xu.rows(0, n - lag);
        s += (&top + top.transpose()) * w;
    }
    let cov = &xtx_inv * s * &xtx_inv;

    let bse: Vec<f64> = (0..k).map(|j| cov[(j, j)].sqrt()).collect();
    let pvalues: Vec<f64> = (0..k)
        .map(|j| 2.0 * norm_sf((beta[j] / bse[j]).abs()))
        .collect();
    let z975 = spec_math::cephes64::ndtri(0.975);
    let conf_int: Vec<(f64, f64)> = (0..k)
        .map(|j| (beta[j] - z975 * bse[j], beta[j] + z975 * bse[j]))
        .collect();

    let ymean = y.iter().sum::<f64>() / n as f64;
    let tss: f64 = y.iter().map(|v| (v - ymean) * (v - ymean)).sum();
    let ssr: f64 = resid.iter().map(|r| r * r).sum();
    HacOls {
        covariance: cov,
        params: beta.iter().copied().collect(),
        bse,
        pvalues,
        conf_int,
        resid,
        rsquared: 1.0 - ssr / tss,
    }
}

/// OLS whose covariance is a sandwich (X'X)^-1 S (X'X)^-1, with normal-based inference.
pub struct SandwichOls {
    /// Full coefficient covariance, in params order.
    pub covariance: DMatrix<f64>,
    pub params: Vec<f64>,
    pub bse: Vec<f64>,
    pub pvalues: Vec<f64>,
    /// 95% normal-based intervals as (lower, upper).
    pub conf_int: Vec<(f64, f64)>,
    pub resid: Vec<f64>,
}

/// The rows of `x` scaled by the residuals: the observation-wise scores of the least-squares fit.
fn scores(x: &DMatrix<f64>, resid: &[f64]) -> DMatrix<f64> {
    DMatrix::from_fn(x.nrows(), x.ncols(), |i, j| x[(i, j)] * resid[i])
}

/// Inference from a parameter vector and its covariance, at the 95% level.
fn sandwich_inference(beta: &DVector<f64>, cov: &DMatrix<f64>, resid: Vec<f64>) -> SandwichOls {
    let k = beta.len();
    let bse: Vec<f64> = (0..k).map(|j| cov[(j, j)].sqrt()).collect();
    let pvalues: Vec<f64> = (0..k)
        .map(|j| 2.0 * norm_sf((beta[j] / bse[j]).abs()))
        .collect();
    let z975 = spec_math::cephes64::ndtri(0.975);
    let conf_int: Vec<(f64, f64)> = (0..k)
        .map(|j| (beta[j] - z975 * bse[j], beta[j] + z975 * bse[j]))
        .collect();
    SandwichOls {
        covariance: cov.clone(),
        params: beta.iter().copied().collect(),
        bse,
        pvalues,
        conf_int,
        resid,
    }
}

/// HC1: each squared residual scaled by n / (n - rank) inside the sandwich.
pub fn ols_hc1(y: &[f64], x: &DMatrix<f64>) -> SandwichOls {
    let n = x.nrows();
    let yv = DVector::from_column_slice(y);
    let fit = crate::ols::Ols::fit(x, &yv);
    let xtx_inv = fit.xtx_inverse();
    let resid: Vec<f64> = fit.resid.iter().copied().collect();
    let correction = n as f64 / (n - fit.rank) as f64;
    let xu = scores(x, &resid);
    let s = xu.transpose() * &xu * correction;
    let cov = &xtx_inv * s * &xtx_inv;
    sandwich_inference(&fit.params, &cov, resid)
}

/// CRV1: the scores are summed within each cluster before the outer product, and the covariance
/// is scaled by G / (G - 1) * (n - 1) / (n - rank). `groups` holds one label per row.
pub fn ols_cluster(y: &[f64], x: &DMatrix<f64>, groups: &[u64]) -> SandwichOls {
    let n = x.nrows();
    let k = x.ncols();
    let yv = DVector::from_column_slice(y);
    let fit = crate::ols::Ols::fit(x, &yv);
    let xtx_inv = fit.xtx_inverse();
    let resid: Vec<f64> = fit.resid.iter().copied().collect();
    let xu = scores(x, &resid);

    let mut clusters: Vec<u64> = groups.to_vec();
    clusters.sort_unstable();
    clusters.dedup();
    let mut sums = DMatrix::<f64>::zeros(clusters.len(), k);
    for i in 0..n {
        let g = clusters.binary_search(&groups[i]).expect("every row's label is a cluster");
        for j in 0..k {
            sums[(g, j)] += xu[(i, j)];
        }
    }
    let n_groups = clusters.len() as f64;
    let correction = n_groups / (n_groups - 1.0) * ((n as f64 - 1.0) / (n - fit.rank) as f64);
    let s = sums.transpose() * &sums * correction;
    let cov = &xtx_inv * s * &xtx_inv;
    sandwich_inference(&fit.params, &cov, resid)
}

/// The covariance of a within regression: classical, HC1, or clustered by the unit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WithinErrors<'a> {
    Classical,
    Hc1,
    Cluster,
    ClusterBy(&'a [u64]),
}

/// Source-specific one-way cluster corrections. Existing callers retain PanelOLS.
#[derive(Clone, Copy, Debug)]
pub enum WithinConvention {
    PanelOls,
    Lfe,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WithinProblem {
    InvalidWeights,
    InvalidDimensions,
    NonFiniteValues,
    TooFewGroups,
    AbsorbedDesign,
    InsufficientDegreesOfFreedom,
}

impl std::fmt::Display for WithinProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidWeights => "Observation weights must be finite, strictly positive and match the row count.",
            Self::InvalidDimensions => "Fixed-effects labels and variables must have the same nonzero row count.",
            Self::NonFiniteValues => "Fixed-effects regression requires finite observed values.",
            Self::TooFewGroups => "Each selected effect or clustering dimension needs at least two groups.",
            Self::AbsorbedDesign => "The fixed effects absorb all regressors; no coefficient can be estimated.",
            Self::InsufficientDegreesOfFreedom => "The regressors and fixed effects leave no residual degrees of freedom.",
        })
    }
}

/// A regression with one fixed effect per unit, and one per period when asked, fitted on the
/// within deviations, with t-based inference on the degrees of freedom that count the effects.
pub struct WithinOls {
    /// Full coefficient covariance, in kept-column order (not original x order).
    pub covariance: DMatrix<f64>,
    /// One entry per kept column, in `kept` order.
    pub params: Vec<f64>,
    pub bse: Vec<f64>,
    pub pvalues: Vec<f64>,
    /// 95% t-based intervals as (lower, upper).
    pub conf_int: Vec<(f64, f64)>,
    pub resid: Vec<f64>,
    pub ssr: f64,
    /// Columns of `x` the effects do not absorb, in their original order.
    pub kept: Vec<usize>,
    /// Columns of `x` absorbed by the effects.
    pub dropped: Vec<usize>,
    pub units: usize,
    /// The number of periods when period effects were fitted.
    pub periods: Option<usize>,
    pub df_resid: usize,
    /// Reference degrees of freedom for intervals and p-values; may differ from residual df.
    pub inference_df: usize,
    /// The fit of the outcome purged of unit effects on the kept columns purged of unit effects.
    pub rsquared_within: f64,
    /// The demeaned outcome and kept columns the fit used.
    pub within_outcome: Vec<f64>,
    pub within_design: DMatrix<f64>,
    /// Every projected input column before source-specific rank decisions.
    pub projected_design: DMatrix<f64>,
}

/// The group index of every row, and the number of groups, from labels that need not be dense.
fn group_index(labels: &[u64]) -> (Vec<usize>, usize) {
    let mut distinct: Vec<u64> = labels.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    let index = labels
        .iter()
        .map(|label| distinct.binary_search(label).expect("every row's label is a group"))
        .collect();
    (index, distinct.len())
}

/// Each value less the mean of its group.
fn demean(values: &[f64], index: &[usize], groups: usize) -> Vec<f64> {
    let mut sums = vec![0.0; groups];
    let mut counts = vec![0usize; groups];
    for (value, &group) in values.iter().zip(index) {
        sums[group] += value;
        counts[group] += 1;
    }
    values
        .iter()
        .zip(index)
        .map(|(value, &group)| value - sums[group] / counts[group] as f64)
        .collect()
}

fn column(matrix: &DMatrix<f64>, j: usize) -> Vec<f64> {
    (0..matrix.nrows()).map(|i| matrix[(i, j)]).collect()
}

/// Every column of `x` through `transform`.
fn map_columns(x: &DMatrix<f64>, transform: impl Fn(&[f64]) -> Vec<f64>) -> DMatrix<f64> {
    let mut out = DMatrix::<f64>::zeros(x.nrows(), x.ncols());
    for j in 0..x.ncols() {
        let values = transform(&column(x, j));
        for i in 0..x.nrows() {
            out[(i, j)] = values[i];
        }
    }
    out
}

/// The columns the effects do not absorb: the rank deficiency of the demeaned design counts
/// the absorbed columns, and those with the smallest diagonal entries of its QR factor are dropped.
fn not_absorbed(xd: &DMatrix<f64>) -> Vec<usize> {
    let (n, p) = xd.shape();
    let all: Vec<usize> = (0..p).collect();
    let singular = xd.clone().svd(false, false).singular_values;
    let largest = singular.iter().copied().fold(0.0, f64::max);
    let rank_tol = largest * n.max(p) as f64 * f64::EPSILON;
    if singular.iter().filter(|value| **value > rank_tol).count() == p {
        return all;
    }
    let xpx = xd.transpose() * xd;
    let eigen = xpx.symmetric_eigen().eigenvalues;
    let eigen_max = eigen.iter().copied().fold(0.0, f64::max);
    if eigen_max == 0.0 {
        return Vec::new();
    }
    let eigen_tol = eigen_max * p as f64 * f64::EPSILON;
    let absorbed = eigen.iter().filter(|value| **value < eigen_tol).count();
    let r = xd.clone().qr().r();
    let diagonal: Vec<f64> = (0..p).map(|j| r[(j, j)].abs()).collect();
    let mut sorted = diagonal.clone();
    sorted.sort_by(f64::total_cmp);
    let threshold = sorted[absorbed];
    all.into_iter().filter(|&j| diagonal[j] >= threshold).collect()
}

/// linearmodels `PanelOLS(entity_effects=True, drop_absorbed=True)` fitted with `debiased=True`:
/// `cov_type="unadjusted"`, `"robust"`, or `"clustered"` by the entity with `group_debias=True`.
/// `x` has no intercept; `units` holds one label per row.
pub fn ols_within(y: &[f64], x: &DMatrix<f64>, units: &[u64], errors: WithinErrors<'_>) -> Result<WithinOls, WithinProblem> {
    within_fit(y, x, units, None, errors, None, WithinConvention::PanelOls)
}

/// The same fit with `time_effects=True` as well: the period effects enter as dummies for every
/// period but the first, demeaned within unit and projected out, which is how the reference removes
/// the second effect on an unbalanced panel. `times` holds one label per row.
pub fn ols_two_way(y: &[f64], x: &DMatrix<f64>, units: &[u64], times: &[u64], errors: WithinErrors<'_>) -> Result<WithinOls, WithinProblem> {
    within_fit(y, x, units, Some(times), errors, None, WithinConvention::PanelOls)
}

/// Weighted PanelOLS without an intercept. Weights are normalized to mean one;
/// effects are removed using weighted projection before fitting. `resid` is in
/// outcome units; within_outcome/design are square-root-weighted projections.
pub fn weighted_within(y: &[f64], x: &DMatrix<f64>, units: &[u64], times: Option<&[u64]>,
    weights: &[f64], errors: WithinErrors<'_>) -> Result<WithinOls, WithinProblem> {
    within_fit(y, x, units, times, errors, Some(weights), WithinConvention::PanelOls)
}

pub fn weighted_within_convention(y: &[f64], x: &DMatrix<f64>, units: &[u64], times: Option<&[u64]>,
    weights: &[f64], errors: WithinErrors<'_>, convention: WithinConvention) -> Result<WithinOls, WithinProblem> {
    within_fit(y, x, units, times, errors, Some(weights), convention)
}

fn within_fit(y: &[f64], x: &DMatrix<f64>, units: &[u64], times: Option<&[u64]>, errors: WithinErrors<'_>, weights: Option<&[f64]>, convention: WithinConvention) -> Result<WithinOls, WithinProblem> {
    let n = x.nrows();
    if n == 0 || x.ncols() == 0 || y.len() != n || units.len() != n
        || times.is_some_and(|labels| labels.len() != n)
        || matches!(errors, WithinErrors::ClusterBy(labels) if labels.len() != n) {
        return Err(WithinProblem::InvalidDimensions);
    }
    if y.iter().chain(x.iter()).any(|value| !value.is_finite()) {
        return Err(WithinProblem::NonFiniteValues);
    }
    let (index, unit_count) = group_index(units);
    if unit_count < 2 || times.is_some_and(|labels| group_index(labels).1 < 2) {
        return Err(WithinProblem::TooFewGroups);
    }
    let normalized = match weights {
        None => None,
        Some(w) => {
            if w.len() != n || w.iter().any(|v| !v.is_finite() || *v <= 0.0) { return Err(WithinProblem::InvalidWeights); }
            // Scale first to avoid overflow in the sum. Global weight scale does
            // not change a WLS fit, its covariance, or the normalized residual SS.
            let max = w.iter().copied().fold(0.0_f64, f64::max);
            let mean = w.iter().map(|v| v / max).sum::<f64>() / n as f64;
            let w: Vec<f64> = w.iter().map(|v| (v / max) / mean).collect();
            if w.iter().any(|v| !v.is_finite() || *v <= 0.0) { return Err(WithinProblem::InvalidWeights); }
            Some(w)
        }
    };
    let by_unit = |values: &[f64]| match &normalized {
        None => demean(values, &index, unit_count),
        Some(w) => {
            let mut sums = vec![0.0; unit_count];
            let mut totals = vec![0.0; unit_count];
            for i in 0..n { sums[index[i]] += w[i] * values[i]; totals[index[i]] += w[i]; }
            (0..n).map(|i| w[i].sqrt() * (values[i] - sums[index[i]] / totals[index[i]])).collect()
        }
    };
    let ye = by_unit(y);
    let xe = map_columns(x, by_unit);
    let (yd, xd, periods) = match times {
        None => (ye.clone(), xe.clone(), None),
        Some(times) => {
            let (time_index, period_count) = group_index(times);
            // On a complete, equally weighted grid the two projections
            // commute. Reuse the existing demean operation for exact
            // two-way deviations, avoiding an unnecessary dummy-matrix SVD.
            let balanced = normalized.is_none()
                && unit_count.checked_mul(period_count) == Some(n)
                && units.iter().zip(times).collect::<std::collections::BTreeSet<_>>().len() == n;
            if balanced {
                (demean(&ye, &time_index, period_count),
                 map_columns(&xe, |values| demean(values, &time_index, period_count)),
                 Some(period_count))
            } else {
            let mut dummies = DMatrix::<f64>::zeros(n, period_count - 1);
            for (i, &period) in time_index.iter().enumerate() {
                if period > 0 {
                    dummies[(i, period - 1)] = 1.0;
                }
            }
            let dummies = map_columns(&dummies, by_unit);
            let residualise = |values: &[f64]| -> Vec<f64> {
                crate::ols::Ols::fit(&dummies, &DVector::from_column_slice(values)).resid.iter().copied().collect()
            };
            (residualise(&ye), map_columns(&xe, residualise), Some(period_count))
            }
        }
    };
    let kept = not_absorbed(&xd);
    let dropped: Vec<usize> = (0..x.ncols()).filter(|j| !kept.contains(j)).collect();
    let design = DMatrix::from_fn(n, kept.len(), |i, j| xd[(i, kept[j])]);
    let k = kept.len();
    if k == 0 { return Err(WithinProblem::AbsorbedDesign); }
    let effects = unit_count + periods.map_or(0, |count| count - 1);
    let df_resid = n.checked_sub(k + effects)
        .filter(|df| *df > 0).ok_or(WithinProblem::InsufficientDegreesOfFreedom)?;
    let fit = crate::ols::Ols::fit(&design, &DVector::from_column_slice(&yd));
    let xtx_inv = fit.xtx_inverse();
    let resid: Vec<f64> = fit.resid.iter().copied().collect();
    let mut inference_df = df_resid;
    let cov = match errors {
        WithinErrors::Classical => &xtx_inv * (fit.ssr / df_resid as f64),
        WithinErrors::Hc1 => {
            let xu = scores(&design, &resid);
            let s = xu.transpose() * &xu * (n as f64 / df_resid as f64);
            &xtx_inv * s * &xtx_inv
        }
        WithinErrors::Cluster | WithinErrors::ClusterBy(_) => {
            let labels = match errors { WithinErrors::ClusterBy(labels) => labels, _ => units };
            let (cluster_index, cluster_count) = group_index(labels);
            if cluster_count < 2 { return Err(WithinProblem::TooFewGroups); }
            let xu = scores(&design, &resid);
            let mut sums = DMatrix::<f64>::zeros(cluster_count, k);
            for i in 0..n {
                for j in 0..k {
                    sums[(cluster_index[i], j)] += xu[(i, j)];
                }
            }
            // The unit effects are nested in the clusters and not counted; period effects are not
            // nested, and then the reference counts every effect.
            let mut group_cluster = vec![None; unit_count];
            let nested = index.iter().zip(&cluster_index).all(|(&unit, &cluster)| {
                match group_cluster[unit] {
                    Some(previous) => previous == cluster,
                    None => { group_cluster[unit] = Some(cluster); true }
                }
            });
            let counted = match convention {
                WithinConvention::PanelOls => if periods.is_some() || !nested { df_resid } else { n - k },
                WithinConvention::Lfe => {
                    let time_nested = times.is_some_and(|times| {
                        let (ti, count) = group_index(times);
                        let mut seen = vec![None; count];
                        ti.iter().zip(&cluster_index).all(|(&t, &c)| match seen[t] {
                            Some(previous) => previous == c,
                            None => { seen[t] = Some(c); true }
                        })
                    });
                    if nested || time_nested {
                        inference_df = df_resid.min(cluster_count - 1);
                        // lfe: (N-1)/(df + total FE parameters - 1).
                        n - k - 1
                    } else { df_resid }
                }
            };
            let g = cluster_count as f64;
            let correction = g / (g - 1.0) * ((n as f64 - 1.0) / counted as f64);
            let s = sums.transpose() * &sums * correction;
            &xtx_inv * s * &xtx_inv
        }
    };
    let bse: Vec<f64> = (0..k).map(|j| cov[(j, j)].sqrt()).collect();
    let pvalues: Vec<f64> = (0..k)
        .map(|j| crate::parcorr::analytic_pvalue_t((fit.params[j] / bse[j]).abs(), inference_df as f64))
        .collect();
    let t975 = spec_math::cephes64::stdtri(inference_df as isize, 0.975);
    let conf_int: Vec<(f64, f64)> = (0..k)
        .map(|j| (fit.params[j] - t975 * bse[j], fit.params[j] + t975 * bse[j]))
        .collect();
    let within_tss: f64 = ye.iter().map(|value| value * value).sum();
    let within_ssr: f64 = (0..n)
        .map(|i| {
            let fitted: f64 = kept.iter().enumerate().map(|(j, &col)| xe[(i, col)] * fit.params[j]).sum();
            (ye[i] - fitted).powi(2)
        })
        .sum();
    Ok(WithinOls {
        covariance: cov,
        params: fit.params.iter().copied().collect(),
        bse,
        pvalues,
        conf_int,
        resid: match &normalized {
            None => resid,
            Some(w) => resid.iter().zip(w).map(|(e,w)| e / w.sqrt()).collect(),
        },
        ssr: fit.ssr,
        kept,
        dropped,
        units: unit_count,
        periods,
        df_resid,
        inference_df,
        rsquared_within: 1.0 - within_ssr / within_tss,
        within_outcome: yd,
        within_design: design,
        projected_design: xd,
    })
}

pub struct WlsFit {
    /// Full classical coefficient covariance, in params order.
    pub covariance: DMatrix<f64>,
    pub params: Vec<f64>,
    pub bse: Vec<f64>,
    pub pvalues: Vec<f64>,
}

/// statsmodels WLS with t-based inference at the classic covariance.
pub fn wls(y: &[f64], x: &DMatrix<f64>, weights: &[f64]) -> WlsFit {
    let n = x.nrows();
    let k = x.ncols();
    let mut xw = x.clone();
    let mut yw = DVector::from_column_slice(y);
    for i in 0..n {
        let sw = weights[i].sqrt();
        yw[i] *= sw;
        for j in 0..k {
            xw[(i, j)] *= sw;
        }
    }
    let fit = crate::ols::Ols::fit(&xw, &yw);
    let xtx_inv = fit.xtx_inverse();
    let beta = fit.params;
    let ssr = fit.ssr;
    let df = (n - fit.rank) as f64;
    let sigma2 = ssr / df;
    let bse: Vec<f64> = (0..k).map(|j| (sigma2 * xtx_inv[(j, j)]).sqrt()).collect();
    let pvalues: Vec<f64> = (0..k)
        .map(|j| crate::parcorr::analytic_pvalue_t((beta[j] / bse[j]).abs(), df))
        .collect();
    WlsFit {
        covariance: xtx_inv * sigma2,
        params: beta.iter().copied().collect(),
        bse,
        pvalues,
    }
}

/// sum of squared residual differences over the residual sum of squares.
pub fn durbin_watson(resid: &[f64]) -> f64 {
    let num: f64 = resid
        .windows(2)
        .map(|w| (w[1] - w[0]) * (w[1] - w[0]))
        .sum();
    let den: f64 = resid.iter().map(|r| r * r).sum();
    num / den
}
