//! ParCorr conditional independence test, ported 1:1 from tigramite: construct_array with the
//! 2xtau_max cut-off, in-place standardisation, OLS residuals, Pearson correlation, Student-t p.

use nalgebra::{DMatrix, DVector};

use crate::ci_samples::{construct_array_tracked, ConstructOptions, SampleExclusion};
use crate::missing_data::{MaskType, PreprocessingError, TigramiteFrame};

/// (variable index, lag <= 0).
pub type Node = (usize, i32);

/// Row-major `(T, N)` time series.
#[derive(Clone, Debug)]
pub struct TimeSeries {
    pub values: Vec<f64>,
    pub t: usize,
    pub n: usize,
}

impl TimeSeries {
    pub fn new(rows: Vec<Vec<f64>>) -> Self {
        let t = rows.len();
        let n = rows[0].len();
        let mut values = Vec::with_capacity(t * n);
        for row in &rows {
            values.extend_from_slice(row);
        }
        TimeSeries { values, t, n }
    }

    pub(crate) fn at(&self, time: usize, var: usize) -> f64 {
        self.values[time * self.n + var]
    }
}

pub(crate) fn dedup(nodes: &[Node]) -> Vec<Node> {
    let mut out: Vec<Node> = Vec::new();
    for node in nodes {
        if !out.contains(node) {
            out.push(*node);
        }
    }
    out
}

/// Cleaned node lists after dedup and Z-overlap removal: the cache key ingredients.
#[derive(Clone, Debug)]
pub struct CleanedXyz {
    pub x: Vec<Node>,
    pub y: Vec<Node>,
    pub z: Vec<Node>,
}

/// tigramite's `cut_off`, which decides how many samples are dropped at the start.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CutOff {
    TwoTauMax,
    TauMax,
    MaxLag,
    MaxLagOrTauMax,
    TwoTauMaxFuture,
}

/// Rows are X, then Y, then Z (overlaps with X or Y removed); samples run over the 2xtau_max cut,
/// optionally restricted to a reference window [start, end) as sliding windows do.
pub fn construct_array(
    data: &TimeSeries,
    x: &[Node],
    y: &[Node],
    z: &[Node],
    tau_max: usize,
) -> (Vec<Vec<f64>>, CleanedXyz) {
    construct_array_windowed(data, x, y, z, tau_max, None)
}

pub fn construct_array_windowed(
    data: &TimeSeries,
    x: &[Node],
    y: &[Node],
    z: &[Node],
    tau_max: usize,
    window: Option<(usize, usize)>,
) -> (Vec<Vec<f64>>, CleanedXyz) {
    construct_array_filtered(data, x, y, z, tau_max, window, None)
}

pub fn construct_array_filtered(
    data: &TimeSeries,
    x: &[Node],
    y: &[Node],
    z: &[Node],
    tau_max: usize,
    window: Option<(usize, usize)>,
    filter: Option<&[bool]>,
) -> (Vec<Vec<f64>>, CleanedXyz) {
    construct_array_general(
        data,
        x,
        y,
        z,
        &[],
        tau_max,
        CutOff::TwoTauMax,
        window,
        filter,
    )
    .0
}

/// `DataFrame.construct_array` with the `extraZ` block and a selectable cut off, as the
/// causal effect estimator uses. Rows run X, Y, Z, extraZ, matching the xyz codes 0, 1, 2, 3.
#[allow(clippy::too_many_arguments)]
pub fn construct_array_general(
    data: &TimeSeries,
    x: &[Node],
    y: &[Node],
    z: &[Node],
    extra_z: &[Node],
    tau_max: usize,
    cut_off: CutOff,
    window: Option<(usize, usize)>,
    filter: Option<&[bool]>,
) -> ((Vec<Vec<f64>>, CleanedXyz), Vec<Node>) {
    crate::ci_samples::construct_dense_compat(
        data, x, y, z, extra_z, tau_max, cut_off, window, filter,
    )
}

/// In-place standardisation with ddof 0, skipping constant rows, exactly as the oracle mutates.
pub(crate) fn standardize(array: &mut [Vec<f64>]) {
    for row in array.iter_mut() {
        let t = row.len() as f64;
        let mean = crate::numpy_mean(row);
        for v in row.iter_mut() {
            *v -= mean;
        }
        // np.std centers again when computing the variance of the shifted row.
        let residual_mean = crate::numpy_mean(row);
        let squares: Vec<_> = row.iter().map(|v| (v - residual_mean).powi(2)).collect();
        let std = (crate::numpy_sum(&squares) / t).sqrt();
        if std != 0.0 {
            for v in row.iter_mut() {
                *v /= std;
            }
        }
    }
}

/// Residual of the target row regressed on the Z rows (no intercept), after standardising all rows.
fn single_residuals(array: &mut [Vec<f64>], target: usize) -> Vec<f64> {
    standardize(array);
    let dim_z = array.len() - 2;
    let y: Vec<f64> = array[target].clone();
    if dim_z == 0 {
        return y;
    }
    let samples = y.len();
    let mut design = DMatrix::<f64>::zeros(samples, dim_z);
    for (k, row) in array[2..].iter().enumerate() {
        for (s, &v) in row.iter().enumerate() {
            design[(s, k)] = v;
        }
    }
    let beta = lstsq(&design, &y);
    let fitted = design * beta;
    y.iter().zip(fitted.iter()).map(|(a, b)| a - b).collect()
}

/// NumPy's `lstsq(..., rcond=None)` dense path uses DGELSD with a relative
/// cutoff of machine precision times the larger matrix dimension.
fn lstsq(design: &DMatrix<f64>, target: &[f64]) -> DVector<f64> {
    let targets = DMatrix::from_column_slice(target.len(), 1, target);
    let tolerance = f64::EPSILON * design.nrows().max(design.ncols()) as f64;
    crate::least_squares::solve(design, &targets, tolerance)
        .expect("finite conditional-independence least-squares inputs")
        .coefficients
        .column(0)
        .into_owned()
}

pub(crate) fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let ma = a.iter().sum::<f64>() / n;
    let mb = b.iter().sum::<f64>() / n;
    let mut num = 0.0;
    let mut da = 0.0;
    let mut db = 0.0;
    for (x, y) in a.iter().zip(b) {
        num += (x - ma) * (y - mb);
        da += (x - ma) * (x - ma);
        db += (y - mb) * (y - mb);
    }
    (num / (da * db).sqrt()).clamp(-1.0, 1.0)
}

const MACHEP: f64 = 1.1102230246251565e-16;

/// Student-t CDF, a 1:1 port of cephes stdtr as scipy uses it: incbet tail for t < -2, exact
/// finite series otherwise.
fn stdtr(rk: f64, t: f64) -> f64 {
    if t == 0.0 {
        return 0.5;
    }
    if t < -2.0 {
        let z = rk / (rk + t * t);
        return 0.5 * spec_math::cephes64::incbet(0.5 * rk, 0.5, z);
    }
    let x = t.abs();
    let z = 1.0 + (x * x) / rk;
    let mut p;
    if rk % 2.0 != 0.0 {
        let xsqk = x / rk.sqrt();
        p = xsqk.atan();
        if rk > 1.0 {
            let mut f = 1.0;
            let mut tz = 1.0;
            let mut j = 3.0;
            while j <= rk - 2.0 && tz / f > MACHEP {
                tz *= (j - 1.0) / (z * j);
                f += tz;
                j += 2.0;
            }
            p += f * xsqk / z;
        }
        p *= 2.0 / std::f64::consts::PI;
    } else {
        let mut f = 1.0;
        let mut tz = 1.0;
        let mut j = 2.0;
        while j <= rk - 2.0 && tz / f > MACHEP {
            tz *= (j - 1.0) / (z * j);
            f += tz;
            j += 2.0;
        }
        p = f * x / (z * rk).sqrt();
    }
    if t < 0.0 {
        p = -p;
    }
    0.5 + 0.5 * p
}

/// Two-sided Student-t p-value for a t statistic at the given degrees of freedom.
pub fn analytic_pvalue_t(t_abs: f64, deg_f: f64) -> f64 {
    2.0 * stdtr(deg_f, -t_abs)
}

pub fn analytic_pvalue(value: f64, samples: usize, dim: usize) -> f64 {
    let deg_f = samples as f64 - dim as f64;
    if deg_f < 1.0 {
        return f64::NAN;
    }
    if (value.abs() - 1.0).abs() <= f64::MIN_POSITIVE {
        return 0.0;
    }
    let trafo = value * (deg_f / (1.0 - value * value)).sqrt();
    2.0 * stdtr(deg_f, -trafo.abs())
}

/// Plain ParCorr, RobustParCorr with normal-scores marginals, or weighted least squares.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CiKind {
    #[default]
    ParCorr,
    RobustParCorr,
    ParCorrWls,
}

/// tigramite's trafo2normal on one row: empirical CDF onto numpy's linspace(1/T, 1, T) grid with
/// np.interp's last-duplicate tie rule, clamped away from 1, then cephes ndtri.
fn trafo_row(row: &mut Vec<f64>) {
    const THRES: f64 = 0.00001;
    let t = row.len();
    let mut sorted = row.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let start = 1.0 / t as f64;
    let step = (1.0 - start) / (t - 1) as f64;
    for v in row.iter_mut() {
        let j = sorted.partition_point(|s| s <= v) - 1;
        let mut u = if j == t - 1 {
            1.0
        } else {
            j as f64 * step + start
        };
        if u == 0.0 {
            u = THRES;
        }
        if u == 1.0 {
            u = 1.0 - THRES;
        }
        *v = spec_math::cephes64::ndtri(u);
    }
}

fn trafo2normal(array: &mut [Vec<f64>]) {
    for row in array.iter_mut() {
        trafo_row(row);
    }
}

/// OLS residuals of the target row on the Z rows, without standardisation.
fn plain_residuals(array: &[Vec<f64>], target: usize) -> Vec<f64> {
    let y = array[target].clone();
    let dim_z = array.len() - 2;
    if dim_z == 0 {
        return y;
    }
    let samples = y.len();
    let mut design = DMatrix::<f64>::zeros(samples, dim_z);
    for (k, row) in array[2..].iter().enumerate() {
        for (s, &v) in row.iter().enumerate() {
            design[(s, k)] = v;
        }
    }
    let beta = lstsq(&design, &y);
    let fitted = design * beta;
    y.iter().zip(fitted.iter()).map(|(a, b)| a - b).collect()
}

/// ParCorrWLS time-dependent noise scale: moving average of absolute OLS residuals, window 10.
fn estimate_std_time(array: &[Vec<f64>], target: usize) -> Vec<f64> {
    const WINDOW: usize = 10;
    let resid: Vec<f64> = plain_residuals(array, target)
        .iter()
        .map(|v| v.abs())
        .collect();
    let t = resid.len();
    let mut stds = vec![1.0; t];
    for k in WINDOW - 1..t {
        stds[k] = resid[k + 1 - WINDOW..=k].iter().sum::<f64>() / WINDOW as f64;
    }
    stds
}

/// WLS residuals with weights 1/std, unstandardised, exactly as the oracle's override defaults.
fn wls_residuals(array: &[Vec<f64>], target: usize, stds: &[f64]) -> Vec<f64> {
    let y = &array[target];
    let w: Vec<f64> = stds.iter().map(|s| 1.0 / s).collect();
    let dim_z = array.len() - 2;
    if dim_z == 0 {
        return y.iter().zip(&w).map(|(v, wi)| v * wi).collect();
    }
    let samples = y.len();
    let mut design_w = DMatrix::<f64>::zeros(samples, dim_z);
    for (k, row) in array[2..].iter().enumerate() {
        for (s, &v) in row.iter().enumerate() {
            design_w[(s, k)] = v * w[s];
        }
    }
    let yw: Vec<f64> = y.iter().zip(&w).map(|(v, wi)| v * wi).collect();
    let beta = lstsq(&design_w, &yw);
    (0..samples)
        .map(|s| {
            let mut mean = 0.0;
            for (k, row) in array[2..].iter().enumerate() {
                mean += row[s] * beta[k];
            }
            (y[s] - mean) * w[s]
        })
        .collect()
}

fn evaluate(mut array: Vec<Vec<f64>>, dim_z: usize, kind: CiKind) -> (f64, f64) {
    let samples = array[0].len();
    if kind == CiKind::ParCorrWls {
        let stds_x = estimate_std_time(&array, 0);
        let stds_y = estimate_std_time(&array, 1);
        let x_resid = wls_residuals(&array, 0, &stds_x);
        let y_resid = wls_residuals(&array, 1, &stds_y);
        let val = pearson(&x_resid, &y_resid);
        let pval = analytic_pvalue(val, samples, 2 + dim_z);
        return (val, pval);
    }
    if kind == CiKind::RobustParCorr {
        trafo2normal(&mut array);
    }
    let x_resid = single_residuals(&mut array, 0);
    let y_resid = single_residuals(&mut array, 1);
    let val = pearson(&x_resid, &y_resid);
    let pval = analytic_pvalue(val, samples, 2 + dim_z);
    (val, pval)
}

/// ParCorr.get_model_selection_criterion with the AIC default: T ln(rss) + 2p on the
/// standardized residuals of j regressed on its parents.
pub fn model_selection_aic(data: &TimeSeries, j: usize, parents: &[Node], tau_max: usize) -> f64 {
    let (mut array, _cleaned) = construct_array(data, &[(j, 0)], &[(j, 0)], parents, tau_max);
    let t = array[0].len() as f64;
    let resid = single_residuals(&mut array, 1);
    let rss: f64 = resid.iter().map(|v| v * v).sum();
    let p = (array.len() - 1) as f64;
    t * rss.ln() + 2.0 * p
}

/// tigramite's ParCorr.run_test with analytic significance: returns (val, pval).
pub fn run_test(
    data: &TimeSeries,
    x: &[Node],
    y: &[Node],
    z: &[Node],
    tau_max: usize,
) -> (f64, f64) {
    let (array, cleaned) = construct_array(data, x, y, z, tau_max);
    evaluate(array, cleaned.z.len(), CiKind::ParCorr)
}

/// The oracle's cached_ci_results: keys are order-independent within X, Y and Z, and symmetric in
/// X and Y, on the cleaned node lists.
#[derive(Clone, Copy, Debug)]
pub struct RoleAwareSamplePolicy {
    pub cut_off: CutOff,
    pub remove_missing_upto_maxlag: bool,
    pub mask_type: MaskType,
}

#[derive(Clone, Debug)]
pub struct CiSampleAudit {
    pub x: Vec<Node>,
    pub y: Vec<Node>,
    pub z: Vec<Node>,
    pub retained_reference_points: Vec<usize>,
    pub exclusions: Vec<SampleExclusion>,
}

#[derive(Default)]
enum SampleSource {
    #[default]
    Dense,
    RoleAware {
        frame: TigramiteFrame,
        policy: RoleAwareSamplePolicy,
    },
}

#[derive(Default)]
pub struct ParCorrCi {
    kind: CiKind,
    /// Reference window [start, end) for sliding-window analysis; None uses the full sample.
    pub window: Option<(usize, usize)>,
    /// Boolean keep-filter over reference time points, the mask_type "y" masking of RPCMCI.
    pub sample_filter: Option<Vec<bool>>,
    /// Normal-scores transform per node: the transform is row-local, so it is reusable across
    /// tests on the same dataset and window.
    trafo_cache: std::collections::HashMap<(Node, Vec<usize>), Vec<f64>>,
    cache: std::collections::HashMap<(Vec<Node>, Vec<Node>, Vec<Node>, Vec<usize>), (f64, f64)>,
    sample_source: SampleSource,
    preprocessing_error: Option<PreprocessingError>,
    pub sample_audits: Vec<CiSampleAudit>,
    /// Every run_test call in order: (X, Y, sorted Z, val, pval). For parity debugging.
    pub trace: Vec<(Node, Node, Vec<Node>, f64, f64)>,
}

impl ParCorrCi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_kind(kind: CiKind) -> Self {
        ParCorrCi {
            kind,
            ..Self::default()
        }
    }

    pub fn with_frame(
        kind: CiKind,
        frame: TigramiteFrame,
        sample_policy: RoleAwareSamplePolicy,
    ) -> Self {
        Self {
            kind,
            sample_source: SampleSource::RoleAware {
                frame,
                policy: sample_policy,
            },
            ..Self::default()
        }
    }

    pub fn preprocessing_error(&self) -> Option<&PreprocessingError> {
        self.preprocessing_error.as_ref()
    }

    pub fn run_test(
        &mut self,
        data: &TimeSeries,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau_max: usize,
    ) -> (f64, f64) {
        if self.preprocessing_error.is_some() {
            return (0.0, 1.0);
        }

        let (array, cleaned, retained_reference_points) =
            if let SampleSource::RoleAware { frame, policy } = &self.sample_source {
                match construct_array_tracked(
                    frame,
                    x,
                    y,
                    z,
                    &[],
                    tau_max,
                    ConstructOptions {
                        cut_off: policy.cut_off,
                        remove_missing_upto_maxlag: policy.remove_missing_upto_maxlag,
                        mask_type: policy.mask_type,
                        window: self.window,
                        reference_points: None,
                        reference_filter: self.sample_filter.as_deref(),
                        bootstrap: None,
                    },
                ) {
                    Ok(constructed) => {
                        let retained = constructed.retained_reference_points.clone();
                        self.sample_audits.push(CiSampleAudit {
                            x: constructed.cleaned.x.clone(),
                            y: constructed.cleaned.y.clone(),
                            z: constructed.cleaned.z.clone(),
                            retained_reference_points: retained.clone(),
                            exclusions: constructed.exclusions.clone(),
                        });
                        (constructed.values, constructed.cleaned, retained)
                    }
                    Err(error) => {
                        self.preprocessing_error = Some(error);
                        return (0.0, 1.0);
                    }
                }
            } else {
                let (array, cleaned) = construct_array_filtered(
                    data,
                    x,
                    y,
                    z,
                    tau_max,
                    self.window,
                    self.sample_filter.as_deref(),
                );
                (array, cleaned, Vec::new())
            };
        let mut xk = cleaned.x.clone();
        let mut yk = cleaned.y.clone();
        let mut zk = cleaned.z.clone();
        xk.sort_unstable();
        yk.sort_unstable();
        zk.sort_unstable();
        let key = if xk <= yk {
            (
                xk.clone(),
                yk,
                zk.clone(),
                retained_reference_points.clone(),
            )
        } else {
            (
                yk,
                xk.clone(),
                zk.clone(),
                retained_reference_points.clone(),
            )
        };
        if let Some(&hit) = self.cache.get(&key) {
            self.trace.push((x[0], y[0], zk, hit.0, hit.1));
            return hit;
        }
        // The oracle's remove_constant_data: drop exactly-constant rows; a constant X or Y
        // short-circuits to no dependence.
        let n_x = cleaned.x.len();
        let n_y = cleaned.y.len();
        let constant: Vec<bool> = array
            .iter()
            .map(|row| {
                let t = row.len() as f64;
                let mean = row.iter().sum::<f64>() / t;
                (row.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / t).sqrt() == 0.0
            })
            .collect();
        let x_alive = constant[..n_x].iter().filter(|c| !**c).count();
        let y_alive = constant[n_x..n_x + n_y].iter().filter(|c| !**c).count();
        let mut array = array;
        if self.kind == CiKind::RobustParCorr {
            let nodes: Vec<Node> = cleaned
                .x
                .iter()
                .chain(cleaned.y.iter())
                .chain(cleaned.z.iter())
                .copied()
                .collect();
            let mut node_iter = nodes.iter();
            for row in array.iter_mut() {
                let node = *node_iter.next().expect("row/node mismatch");
                let transform_key = (node, retained_reference_points.clone());
                match self.trafo_cache.get(&transform_key) {
                    Some(cached) => row.clone_from(cached),
                    None => {
                        trafo_row(row);
                        self.trafo_cache.insert(transform_key, row.clone());
                    }
                }
            }
        }
        let eval_kind = if self.kind == CiKind::RobustParCorr {
            CiKind::ParCorr
        } else {
            self.kind
        };
        let result = if x_alive == 0 || y_alive == 0 {
            (0.0, 1.0)
        } else if constant.iter().any(|&c| c) {
            let kept: Vec<Vec<f64>> = array
                .into_iter()
                .zip(&constant)
                .filter(|(_, &c)| !c)
                .map(|(row, _)| row)
                .collect();
            let dim_z = kept.len() - x_alive - y_alive;
            evaluate(kept, dim_z, eval_kind)
        } else {
            evaluate(array, key.2.len(), eval_kind)
        };
        self.cache.insert(key, result);
        self.trace.push((x[0], y[0], zk, result.0, result.1));
        result
    }
}
