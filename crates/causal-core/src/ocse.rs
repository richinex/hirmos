//! Optimal Causation Entropy with corrected candidate-set semantics and an explicit
//! compatibility path for causationentropy's current `discover_network` implementation.

use crate::nprandom::NpRng;
use nalgebra::DMatrix;

#[cfg(target_os = "macos")]
#[link(name = "Accelerate", kind = "framework")]
extern "C" {
    #[link_name = "cblas_dsyrk$NEWLAPACK$ILP64"]
    fn cblas_dsyrk_ilp64(
        order: i32,
        uplo: i32,
        trans_a: i32,
        n: i64,
        k: i64,
        alpha: f64,
        a: *const f64,
        lda: i64,
        beta: f64,
        c: *mut f64,
        ldc: i64,
    );
    #[link_name = "dgetrf$NEWLAPACK$ILP64"]
    fn dgetrf_ilp64(
        m: *const i64,
        n: *const i64,
        a: *mut f64,
        lda: *const i64,
        ipiv: *mut i64,
        info: *mut i64,
    );
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CmiMethod {
    Gaussian,
    Knn,
}

/// Numerical and compatibility controls for standard oCSE discovery.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OcseOptions {
    /// Retain causationentropy's current behavior of testing target self-lags even though the
    /// same columns are already present in the initial conditioning set.
    pub include_initial_conditioning_candidates: bool,
    /// Optional correlation shrinkage `(1-a)R + aI` for Gaussian CMI. Must be in `[0, 1]`.
    pub correlation_adjustment: f64,
}

impl Default for OcseOptions {
    fn default() -> Self {
        Self {
            include_initial_conditioning_candidates: false,
            correlation_adjustment: 0.0,
        }
    }
}

impl OcseOptions {
    /// Reproduce the current Python package, including its duplicate self-lag candidates.
    pub const fn reference_compatible() -> Self {
        Self {
            include_initial_conditioning_candidates: true,
            correlation_adjustment: 0.0,
        }
    }
}

pub struct OcseEdge {
    pub src: usize,
    pub dst: usize,
    pub lag: usize,
    pub cmi: f64,
    pub p_value: f64,
}

/// numpy corrcoef of the columns of a: bias-corrected covariance normalised and clipped.
fn corrcoef(a: &DMatrix<f64>, adjustment: f64) -> DMatrix<f64> {
    let n = a.nrows();
    let p = a.ncols();
    let mut centered = a.clone();
    for mut col in centered.column_iter_mut() {
        // `np.mean(A, axis=0)` reduces a C-contiguous sample matrix one row at
        // a time, retaining one accumulator per variable.
        let mut total = 0.0;
        for value in col.iter() {
            total += *value;
        }
        let mean = total / n as f64;
        for v in col.iter_mut() {
            *v -= mean;
        }
    }
    #[cfg(target_os = "macos")]
    let mut c = {
        // NumPy on macOS dispatches `X @ X.T` to Accelerate's symmetric rank-k
        // kernel. Calling the same ILP64 kernel preserves its rounding, including
        // the one-ulp asymmetries between duplicated oCSE self-lag columns.
        let mut variables_by_sample = vec![0.0; p * n];
        for sample in 0..n {
            for variable in 0..p {
                variables_by_sample[sample * p + variable] = centered[(sample, variable)];
            }
        }
        let mut covariance = vec![0.0; p * p];
        unsafe {
            cblas_dsyrk_ilp64(
                102, // CblasColMajor, matching np.cov's Fortran-contiguous X
                122, // CblasLower
                111, // CblasNoTrans
                p as i64,
                n as i64,
                1.0,
                variables_by_sample.as_ptr(),
                p as i64,
                0.0,
                covariance.as_mut_ptr(),
                p as i64,
            );
        }
        for column in 0..p {
            for row in 0..column {
                covariance[row + column * p] = covariance[column + row * p];
            }
        }
        let normalisation = 1.0 / (n as f64 - 1.0);
        DMatrix::from_fn(p, p, |row, column| {
            covariance[row + column * p] * normalisation
        })
    };
    #[cfg(not(target_os = "macos"))]
    let mut c = centered.transpose() * &centered / (n as f64 - 1.0);
    let d: Vec<f64> = (0..p).map(|i| c[(i, i)].sqrt()).collect();
    for i in 0..p {
        for j in 0..p {
            c[(i, j)] /= d[i];
        }
    }
    for i in 0..p {
        for j in 0..p {
            c[(i, j)] /= d[j];
        }
    }
    for v in c.iter_mut() {
        *v = v.clamp(-1.0, 1.0);
    }
    if adjustment > 0.0 {
        c *= 1.0 - adjustment;
        for index in 0..p {
            c[(index, index)] += adjustment;
        }
    }
    c
}

/// LAPACK DGETRF2's recursive partial-pivot LU, which is the factorisation behind
/// `numpy.linalg.slogdet`.  The recursive update order matters for singular correlation
/// matrices: oCSE preserves NumPy's NaN/inf behaviour, and a scalar
/// left-looking elimination takes a different rounding path through duplicated columns.
#[cfg(target_os = "macos")]
fn slogdet(m: &DMatrix<f64>) -> (f64, f64) {
    let n = m.nrows() as i64;
    let mut a = m.as_slice().to_vec();
    let mut pivots = vec![0i64; n as usize];
    let mut info = 0i64;
    unsafe {
        dgetrf_ilp64(&n, &n, a.as_mut_ptr(), &n, pivots.as_mut_ptr(), &mut info);
    }
    if info > 0 {
        return (0.0, f64::NEG_INFINITY);
    }
    assert_eq!(info, 0, "LAPACK dgetrf received an invalid argument");
    let mut sign = if pivots
        .iter()
        .enumerate()
        .filter(|(i, pivot)| **pivot != *i as i64 + 1)
        .count()
        % 2
        == 0
    {
        1.0
    } else {
        -1.0
    };
    let mut logdet = 0.0;
    for index in 0..n as usize {
        let diagonal = a[index + index * n as usize];
        if diagonal == 0.0 {
            return (0.0, f64::NEG_INFINITY);
        }
        sign *= diagonal.signum();
        logdet += diagonal.abs().ln();
    }
    (sign, logdet)
}

#[cfg(not(target_os = "macos"))]
fn slogdet(m: &DMatrix<f64>) -> (f64, f64) {
    let mut a = m.clone();
    let mut pivots = Vec::with_capacity(m.nrows());

    fn factor(
        a: &mut DMatrix<f64>,
        row0: usize,
        col0: usize,
        rows: usize,
        cols: usize,
        pivots: &mut Vec<(usize, usize)>,
    ) {
        if rows == 0 || cols == 0 {
            return;
        }
        if rows == 1 {
            pivots.push((row0, row0));
            return;
        }
        if cols == 1 {
            let mut pivot = row0;
            for row in row0 + 1..row0 + rows {
                if a[(row, col0)].abs() > a[(pivot, col0)].abs() {
                    pivot = row;
                }
            }
            pivots.push((row0, pivot));
            if a[(pivot, col0)] != 0.0 {
                if pivot != row0 {
                    a.swap((row0, col0), (pivot, col0));
                }
                let diagonal = a[(row0, col0)];
                for row in row0 + 1..row0 + rows {
                    a[(row, col0)] /= diagonal;
                }
            }
            return;
        }

        let left_cols = rows.min(cols) / 2;
        let right_cols = cols - left_cols;

        let left_pivot_start = pivots.len();
        factor(a, row0, col0, rows, left_cols, pivots);
        let left_pivot_end = pivots.len();

        // DLASWP: apply the first panel's row pivots to the right panel.
        for pivot_index in left_pivot_start..left_pivot_end {
            let (first, pivot) = pivots[pivot_index];
            if first != pivot {
                for column in col0 + left_cols..col0 + cols {
                    a.swap((first, column), (pivot, column));
                }
            }
        }

        // DTRSM: solve L11 * U12 = A12 for unit-lower-triangular L11.
        for column in col0 + left_cols..col0 + cols {
            for i in 0..left_cols {
                let mut value = a[(row0 + i, column)];
                for k in 0..i {
                    value -= a[(row0 + i, col0 + k)] * a[(row0 + k, column)];
                }
                a[(row0 + i, column)] = value;
            }
        }

        // DGEMM: A22 -= L21 * U12.  Keep LAPACK's accumulation order; it is
        // observable when identical self-lag columns make A exactly singular.
        for column in col0 + left_cols..col0 + cols {
            for row in row0 + left_cols..row0 + rows {
                let mut value = a[(row, column)];
                for k in 0..left_cols {
                    value -= a[(row, col0 + k)] * a[(row0 + k, column)];
                }
                a[(row, column)] = value;
            }
        }

        let right_pivot_start = pivots.len();
        factor(
            a,
            row0 + left_cols,
            col0 + left_cols,
            rows - left_cols,
            right_cols,
            pivots,
        );
        let right_pivot_end = pivots.len();

        // DLASWP: propagate the second panel's pivots back through the left panel.
        for pivot_index in right_pivot_start..right_pivot_end {
            let (first, pivot) = pivots[pivot_index];
            if first != pivot {
                for column in col0..col0 + left_cols {
                    a.swap((first, column), (pivot, column));
                }
            }
        }
    }

    let n = m.nrows();
    factor(&mut a, 0, 0, n, n, &mut pivots);
    let mut sign = if pivots
        .iter()
        .filter(|(first, pivot)| first != pivot)
        .count()
        % 2
        == 0
    {
        1.0
    } else {
        -1.0
    };
    let mut logdet = 0.0;
    for diagonal in (0..n).map(|i| a[(i, i)]) {
        if diagonal == 0.0 {
            return (0.0, f64::NEG_INFINITY);
        }
        sign *= diagonal.signum();
        logdet += diagonal.abs().ln();
    }
    (sign, logdet)
}

/// The oracle's correlation_log_determinant with its -1000 singular fallback.
fn correlation_log_determinant(a: &DMatrix<f64>, adjustment: f64) -> f64 {
    if a.ncols() == 0 {
        return 0.0;
    }
    if a.ncols() == 1 {
        return 0.0;
    }
    let c = corrcoef(a, adjustment);
    let (sign, logdet) = slogdet(&c);
    if sign == 0.0 || !logdet.is_finite() {
        return -1000.0;
    }
    logdet
}

fn hstack(parts: &[&DMatrix<f64>]) -> DMatrix<f64> {
    let rows = parts[0].nrows();
    let cols: usize = parts.iter().map(|m| m.ncols()).sum();
    let mut out = DMatrix::<f64>::zeros(rows, cols);
    let mut at = 0;
    for m in parts {
        out.view_mut((0, at), (rows, m.ncols())).copy_from(*m);
        at += m.ncols();
    }
    out
}

fn gaussian_mi(x: &DMatrix<f64>, y: &DMatrix<f64>, adjustment: f64) -> f64 {
    let sx = correlation_log_determinant(x, adjustment);
    let sy = correlation_log_determinant(y, adjustment);
    let sxy = correlation_log_determinant(&hstack(&[x, y]), adjustment);
    0.5 * (sx + sy - sxy)
}

/// gaussian CMI via slogdet of correlation matrices; the sub-blocks skip the -1000 fallback,
/// exactly as _detcorr does in the oracle.
fn gaussian_cmi(
    x: &DMatrix<f64>,
    y: &DMatrix<f64>,
    z: Option<&DMatrix<f64>>,
    adjustment: f64,
) -> f64 {
    let Some(z) = z else {
        return gaussian_mi(x, y, adjustment);
    };
    let detcorr = |a: &DMatrix<f64>| -> f64 {
        if a.ncols() == 1 {
            return 0.0;
        }
        slogdet(&corrcoef(a, adjustment)).1
    };
    let sz = detcorr(z);
    let sxz = detcorr(&hstack(&[x, z]));
    let syz = detcorr(&hstack(&[y, z]));
    let sxyz = detcorr(&hstack(&[x, y, z]));
    0.5 * (sxz + syz - sz - sxyz)
}

fn euclidean_dists(a: &DMatrix<f64>) -> DMatrix<f64> {
    let n = a.nrows();
    let mut d = DMatrix::<f64>::zeros(n, n);
    for i in 0..n {
        for j in i + 1..n {
            let mut acc = 0.0;
            for c in 0..a.ncols() {
                let diff = a[(i, c)] - a[(j, c)];
                acc += diff * diff;
            }
            let dist = acc.sqrt();
            d[(i, j)] = dist;
            d[(j, i)] = dist;
        }
    }
    d
}

fn kth_smallest_per_row(d: &DMatrix<f64>, k: usize) -> Vec<f64> {
    (0..d.nrows())
        .map(|i| {
            let mut row: Vec<f64> = d.row(i).iter().copied().collect();
            row.sort_by(|a, b| a.partial_cmp(b).unwrap());
            row[k]
        })
        .collect()
}

fn count_within(d: &DMatrix<f64>, eps: &[f64]) -> Vec<i64> {
    (0..d.nrows())
        .map(|i| d.row(i).iter().filter(|&&v| v < eps[i]).count() as i64 - 1)
        .collect()
}

fn psi(x: f64) -> f64 {
    spec_math::cephes64::psi(x)
}

fn knn_mi(x: &DMatrix<f64>, y: &DMatrix<f64>, k: usize) -> f64 {
    let n = x.nrows();
    let js = hstack(&[x, y]);
    let eps = kth_smallest_per_row(&euclidean_dists(&js), k);
    let nx = count_within(&euclidean_dists(x), &eps);
    let ny = count_within(&euclidean_dists(y), &eps);
    let mean: f64 = nx
        .iter()
        .zip(&ny)
        .map(|(&a, &b)| psi((a + 1) as f64) + psi((b + 1) as f64))
        .sum::<f64>()
        / n as f64;
    psi(k as f64) + psi(n as f64) - mean
}

/// The VP-style knn CMI the oracle uses when Z is present.
fn knn_cmi(x: &DMatrix<f64>, y: &DMatrix<f64>, z: Option<&DMatrix<f64>>, k: usize) -> f64 {
    let Some(z) = z else {
        return knn_mi(x, y, k);
    };
    let js = hstack(&[x, y, z]);
    let eps = kth_smallest_per_row(&euclidean_dists(&js), k);
    let nxz = count_within(&euclidean_dists(&hstack(&[x, z])), &eps);
    let nyz = count_within(&euclidean_dists(&hstack(&[y, z])), &eps);
    let nz = count_within(&euclidean_dists(z), &eps);
    let n = x.nrows();
    let mean: f64 = (0..n)
        .map(|i| psi((nxz[i] + 1) as f64) + psi((nyz[i] + 1) as f64) - psi((nz[i] + 1) as f64))
        .sum::<f64>()
        / n as f64;
    psi(k as f64) - mean
}

/// conditional_mutual_information with the oracle's clamp of finite values to zero or above.
pub fn cmi(
    x: &DMatrix<f64>,
    y: &DMatrix<f64>,
    z: Option<&DMatrix<f64>>,
    method: CmiMethod,
    k: usize,
) -> f64 {
    cmi_with_adjustment(x, y, z, method, k, 0.0)
}

fn cmi_with_adjustment(
    x: &DMatrix<f64>,
    y: &DMatrix<f64>,
    z: Option<&DMatrix<f64>>,
    method: CmiMethod,
    k: usize,
    adjustment: f64,
) -> f64 {
    let value = match method {
        CmiMethod::Gaussian => gaussian_cmi(x, y, z, adjustment),
        CmiMethod::Knn => knn_cmi(x, y, z, k),
    };
    if value.is_finite() {
        value.max(0.0)
    } else {
        value
    }
}

/// numpy percentile with linear interpolation, NaN sorted last as numpy does.
fn percentile(values: &[f64], q: f64) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| match (a.is_nan(), b.is_nan()) {
        (true, true) => std::cmp::Ordering::Equal,
        (true, false) => std::cmp::Ordering::Greater,
        (false, true) => std::cmp::Ordering::Less,
        _ => a.partial_cmp(b).unwrap(),
    });
    let n = sorted.len();
    let pos = q / 100.0 * (n - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    let frac = pos - lo as f64;
    sorted[lo] + frac * (sorted[hi] - sorted[lo])
}

struct ShuffleOutcome {
    pass: bool,
    p_value: f64,
}

fn shuffle_test(
    x: &DMatrix<f64>,
    y: &DMatrix<f64>,
    z: Option<&DMatrix<f64>>,
    observed: f64,
    alpha: f64,
    rng: &mut NpRng,
    n_shuffles: usize,
    method: CmiMethod,
    k: usize,
    adjustment: f64,
) -> ShuffleOutcome {
    let n = x.nrows();
    let mut null = Vec::with_capacity(n_shuffles);
    for _ in 0..n_shuffles {
        let perm = rng.permutation(n);
        let x_perm = DMatrix::from_fn(n, x.ncols(), |i, j| x[(perm[i], j)]);
        null.push(cmi_with_adjustment(&x_perm, y, z, method, k, adjustment));
    }
    let threshold = percentile(&null, 100.0 * (1.0 - alpha));
    let p_value = null.iter().filter(|&&v| v >= observed).count() as f64 / n_shuffles as f64;
    ShuffleOutcome {
        pass: observed >= threshold,
        p_value,
    }
}

fn columns(m: &DMatrix<f64>, cols: &[usize]) -> DMatrix<f64> {
    DMatrix::from_fn(m.nrows(), cols.len(), |i, j| m[(i, cols[j])])
}

fn standard_forward(
    x_full: &DMatrix<f64>,
    y: &DMatrix<f64>,
    z_init: &DMatrix<f64>,
    rng: &mut NpRng,
    alpha: f64,
    n_shuffles: usize,
    method: CmiMethod,
    k: usize,
    initial_candidates: &[usize],
    adjustment: f64,
) -> Vec<usize> {
    let mut candidates = initial_candidates.to_vec();
    let mut selected: Vec<usize> = Vec::new();
    let mut z = z_init.clone();

    while !candidates.is_empty() {
        let mut ent = Vec::with_capacity(candidates.len());
        for &j in &candidates {
            let xj = columns(x_full, &[j]);
            ent.push(cmi_with_adjustment(&xj, y, Some(&z), method, k, adjustment));
        }
        // numpy argmax treats NaN as maximal and returns the first NaN index.
        let k_best = ent.iter().position(|v| v.is_nan()).unwrap_or_else(|| {
            let mut best = 0usize;
            for (i, v) in ent.iter().enumerate() {
                if *v > ent[best] {
                    best = i;
                }
            }
            best
        });
        let j_best = candidates[k_best];
        let x_best = columns(x_full, &[j_best]);
        let mi_best = ent[k_best];

        let passed = shuffle_test(
            &x_best,
            y,
            Some(&z),
            mi_best,
            alpha,
            rng,
            n_shuffles,
            method,
            k,
            adjustment,
        )
        .pass;
        if !passed {
            candidates.remove(k_best);
            continue;
        }
        selected.push(j_best);
        z = hstack(&[&z, &x_best]);
        candidates.remove(k_best);
    }
    selected
}

fn backward(
    x_full: &DMatrix<f64>,
    y: &DMatrix<f64>,
    s_init: &[usize],
    rng: &mut NpRng,
    alpha: f64,
    n_shuffles: usize,
    method: CmiMethod,
    k: usize,
    adjustment: f64,
) -> Vec<usize> {
    let mut s: Vec<usize> = s_init.to_vec();
    for j in rng.permutation_of(s_init) {
        let others: Vec<usize> = s.iter().copied().filter(|&v| v != j).collect();
        let z = if s.len() > 1 {
            Some(columns(x_full, &others))
        } else {
            None
        };
        let xj = columns(x_full, &[j]);
        let cmij = cmi_with_adjustment(&xj, y, z.as_ref(), method, k, adjustment);
        let passed = shuffle_test(
            &xj,
            y,
            z.as_ref(),
            cmij,
            alpha,
            rng,
            n_shuffles,
            method,
            k,
            adjustment,
        )
        .pass;
        if !passed {
            s.retain(|&v| v != j);
        }
    }
    s
}

/// Standard oCSE with corrected candidate-set semantics: self-lags in `Z_init` are excluded.
pub fn discover_network(
    data: &[Vec<f64>],
    max_lag: usize,
    alpha_forward: f64,
    alpha_backward: f64,
    n_shuffles: usize,
    method: CmiMethod,
    k: usize,
) -> Vec<OcseEdge> {
    discover_network_with_progress(
        data,
        max_lag,
        alpha_forward,
        alpha_backward,
        n_shuffles,
        method,
        k,
        |_, _, _| {},
    )
}

/// Standard oCSE while reporting completion of each target-variable search.
pub fn discover_network_with_progress<F>(
    data: &[Vec<f64>],
    max_lag: usize,
    alpha_forward: f64,
    alpha_backward: f64,
    n_shuffles: usize,
    method: CmiMethod,
    k: usize,
    progress: F,
) -> Vec<OcseEdge>
where
    F: FnMut(&'static str, usize, usize),
{
    discover_network_with_options_and_progress(
        data,
        max_lag,
        alpha_forward,
        alpha_backward,
        n_shuffles,
        method,
        k,
        OcseOptions::default(),
        progress,
    )
}

/// Reproduce causationentropy's current implementation, including duplicate self-lag candidates.
pub fn discover_network_reference_compatible(
    data: &[Vec<f64>],
    max_lag: usize,
    alpha_forward: f64,
    alpha_backward: f64,
    n_shuffles: usize,
    method: CmiMethod,
    k: usize,
) -> Vec<OcseEdge> {
    discover_network_with_options(
        data,
        max_lag,
        alpha_forward,
        alpha_backward,
        n_shuffles,
        method,
        k,
        OcseOptions::reference_compatible(),
    )
}

/// Standard oCSE with explicit candidate and Gaussian-correlation policies.
pub fn discover_network_with_options(
    data: &[Vec<f64>],
    max_lag: usize,
    alpha_forward: f64,
    alpha_backward: f64,
    n_shuffles: usize,
    method: CmiMethod,
    k: usize,
    options: OcseOptions,
) -> Vec<OcseEdge> {
    discover_network_with_options_and_progress(
        data,
        max_lag,
        alpha_forward,
        alpha_backward,
        n_shuffles,
        method,
        k,
        options,
        |_, _, _| {},
    )
}

fn discover_network_with_options_and_progress<F>(
    data: &[Vec<f64>],
    max_lag: usize,
    alpha_forward: f64,
    alpha_backward: f64,
    n_shuffles: usize,
    method: CmiMethod,
    k: usize,
    options: OcseOptions,
    mut progress: F,
) -> Vec<OcseEdge>
where
    F: FnMut(&'static str, usize, usize),
{
    assert!(
        (0.0..=1.0).contains(&options.correlation_adjustment),
        "correlation_adjustment must be in [0, 1]"
    );
    let t = data.len();
    let n = data[0].len();
    let rows = t - max_lag;
    let mut rng = NpRng::seeded(42);

    // Lagged predictors ordered variable-major, lag 1..=max_lag within each variable.
    let mut feature_names = Vec::new();
    let mut x_lagged = DMatrix::<f64>::zeros(rows, n * max_lag);
    let mut col = 0;
    for j in 0..n {
        for tau in 1..=max_lag {
            for r in 0..rows {
                x_lagged[(r, col)] = data[max_lag - tau + r][j];
            }
            feature_names.push((j, tau));
            col += 1;
        }
    }

    let mut edges = Vec::new();
    progress("target-variable searches", 0, n);
    for i in 0..n {
        let y = DMatrix::from_fn(rows, 1, |r, _| data[max_lag + r][i]);
        let z_init = DMatrix::from_fn(rows, max_lag, |r, tau| data[max_lag - (tau + 1) + r][i]);
        let candidates: Vec<usize> = (0..x_lagged.ncols())
            .filter(|candidate| {
                options.include_initial_conditioning_candidates || candidate / max_lag != i
            })
            .collect();
        let forward = standard_forward(
            &x_lagged,
            &y,
            &z_init,
            &mut rng,
            alpha_forward,
            n_shuffles,
            method,
            k,
            &candidates,
            options.correlation_adjustment,
        );
        let selected = backward(
            &x_lagged,
            &y,
            &forward,
            &mut rng,
            alpha_backward,
            n_shuffles,
            method,
            k,
            options.correlation_adjustment,
        );

        for &s in &selected {
            let (src_var, src_lag) = feature_names[s];
            let x_pred = columns(&x_lagged, &[s]);
            let others: Vec<usize> = selected.iter().copied().filter(|&v| v != s).collect();
            let z = if others.is_empty() {
                None
            } else {
                Some(columns(&x_lagged, &others))
            };
            let edge_cmi = cmi_with_adjustment(
                &x_pred,
                &y,
                z.as_ref(),
                method,
                k,
                options.correlation_adjustment,
            );
            let outcome = shuffle_test(
                &x_pred,
                &y,
                z.as_ref(),
                edge_cmi,
                alpha_backward,
                &mut rng,
                n_shuffles,
                method,
                k,
                options.correlation_adjustment,
            );
            edges.push(OcseEdge {
                src: src_var,
                dst: i,
                lag: src_lag,
                cmi: edge_cmi,
                p_value: outcome.p_value,
            });
        }
        progress("target-variable searches", i + 1, n);
    }
    progress("complete", n, n);
    edges
}
