//! Tigramite ParCorrMult's max-correlation and Wilks-lambda dependence tests.
//! Source: tigramite/independence_tests/parcorr_mult.py (GPL-3.0).

use crate::parcorr::{analytic_pvalue, standardize};
use faer::linalg::solvers::DenseSolveCore;
use nalgebra::DMatrix;

pub mod block_length;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Correlation {
    #[default]
    MaxCorrelation,
    PccaWilksLambda,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    X,
    Y,
    Z,
}

/// Source fixed-threshold decisions: an all-constant X or Y has no p-value.
/// The other variants expose the source's 0/1 decision marker, not a statistical
/// p-value. Keeping the constant case distinct prevents fabricating a p-value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FixedDecision {
    Constant,
    Dependent { value: f64 },
    Independent { value: f64 },
}

/// Cached fixed-threshold evidence is independent of the requested threshold.
#[derive(Clone, Copy, Debug)]
pub enum FixedStatistic {
    Constant,
    Value(f64),
}

#[derive(Clone, Copy, Debug)]
pub enum AnalyticResult {
    Constant,
    Tested { value: f64, p_value: f64 },
}

impl AnalyticResult {
    pub fn values(self) -> (f64, f64) {
        match self {
            Self::Constant => (0.0, 1.0),
            Self::Tested { value, p_value } => (value, p_value),
        }
    }
}

impl FixedStatistic {
    pub fn decide(self, threshold: f64) -> FixedDecision {
        match self {
            Self::Constant => FixedDecision::Constant,
            Self::Value(value) if threshold_reached(value, threshold) => {
                FixedDecision::Dependent { value }
            }
            Self::Value(value) => FixedDecision::Independent { value },
        }
    }
}

/// Preserve `abs(value) >= abs(threshold)` across native and WASM arithmetic.
/// Tigramite accepts equality. A statistic reconstructed one or two ULPs below
/// a threshold copied from the reference is therefore the same boundary case,
/// not evidence for the opposite decision.
fn threshold_reached(value: f64, threshold: f64) -> bool {
    let value = value.abs();
    let threshold = threshold.abs();
    let equality_slack = 4.0 * f64::EPSILON * value.max(threshold).max(1.0);
    value >= threshold || (value - threshold).abs() <= equality_slack
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SampleError {
    Shape,
    EmptySamples,
    MissingRole(Role),
    NonFinite,
    LeastSquares,
    Covariance,
    ShuffleConfiguration,
    BlockLength(block_length::BlockError),
}

/// Feature rows with common samples and an explicit X/Y/Z role for each row.
#[derive(Clone, Debug)]
pub struct Samples {
    rows: Vec<Vec<f64>>,
    roles: Vec<Role>,
}

/// Deterministic product reduction used for fitted-value reconstruction.
///
/// NumPy delegates `dot` to a shape- and platform-specific BLAS kernel. This
/// explicit fold gives the native and WASM ports one shared numerical backend,
/// and is exposed so the Python control-flow oracle can use that same primitive.
pub fn fitted_product_sum(terms: impl IntoIterator<Item = (f64, f64)>) -> f64 {
    terms
        .into_iter()
        .fold(0.0, |sum, (left, right)| libm::fma(left, right, sum))
}

impl Samples {
    pub fn new(rows: Vec<Vec<f64>>, roles: Vec<Role>) -> Result<Self, SampleError> {
        if rows.len() != roles.len() {
            return Err(SampleError::Shape);
        }
        let n = rows.first().map_or(0, Vec::len);
        if n == 0 {
            return Err(SampleError::EmptySamples);
        }
        if rows.iter().any(|r| r.len() != n) {
            return Err(SampleError::Shape);
        }
        if rows.iter().flatten().any(|v| !v.is_finite()) {
            return Err(SampleError::NonFinite);
        }
        for role in [Role::X, Role::Y] {
            if !roles.contains(&role) {
                return Err(SampleError::MissingRole(role));
            }
        }
        Ok(Self { rows, roles })
    }

    fn residuals(&mut self, target: Role) -> Result<Vec<Vec<f64>>, SampleError> {
        // The source mutates and standardizes the shared array on EACH call.
        standardize(&mut self.rows);
        let ys: Vec<_> = self
            .rows
            .iter()
            .zip(&self.roles)
            .filter(|(_, role)| **role == target)
            .map(|(row, _)| row.clone())
            .collect();
        let zs: Vec<_> = self
            .rows
            .iter()
            .zip(&self.roles)
            .filter(|(_, role)| **role == Role::Z)
            .map(|(row, _)| row)
            .collect();
        if zs.is_empty() {
            return Ok(ys);
        }
        let n = self.rows[0].len();
        let z = DMatrix::from_fn(n, zs.len(), |i, j| zs[j][i]);
        let y = DMatrix::from_fn(n, ys.len(), |i, j| ys[j][i]);
        // np.linalg.lstsq(..., rcond=None): no intercept; epsilon * max(M,N).
        // Do not substitute the sklearn default tolerance or its extra centering.
        let fit = crate::least_squares::solve(&z, &y, f64::EPSILON * n.max(zs.len()) as f64)
            .map_err(|_| SampleError::LeastSquares)?;
        // NumPy's BLAS-backed np.dot uses fused multiply-add on the pinned
        // source platform.  Make that reconstruction explicit so native and
        // WASM do not choose different rounding paths for near-zero residuals.
        let residuals = DMatrix::from_fn(n, ys.len(), |i, j| {
            let fitted =
                fitted_product_sum((0..zs.len()).map(|k| (z[(i, k)], fit.coefficients[(k, j)])));
            y[(i, j)] - fitted
        });
        Ok((0..ys.len())
            .map(|j| residuals.column(j).iter().copied().collect())
            .collect())
    }

    /// Return the regression residuals and the shared array after the source's
    /// in-place standardization step.
    ///
    /// This is the coarse numerical boundary used by the Python control-flow
    /// oracle. Returning both values preserves ParCorrMult's second-call
    /// behavior: Y is residualized after X has already standardized the array.
    pub fn residuals_with_standardized_rows(
        mut self,
        target: Role,
    ) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>), SampleError> {
        let residuals = self.residuals(target)?;
        Ok((residuals, self.rows))
    }

    /// Direct get_dependence_measure, including source non-finite behavior for
    /// constant residuals. Use run_test for the base-class constant-input removal.
    pub fn dependence(&self) -> Result<f64, SampleError> {
        self.dependence_with(Correlation::MaxCorrelation)
    }

    pub fn dependence_with(&self, correlation: Correlation) -> Result<f64, SampleError> {
        self.clone().dependence_mut(correlation)
    }

    fn dependence_mut(&mut self, correlation: Correlation) -> Result<f64, SampleError> {
        let mut x = self.residuals(Role::X)?;
        let mut y = self.residuals(Role::Y)?;
        standardize(&mut x);
        standardize(&mut y);
        if correlation == Correlation::PccaWilksLambda {
            return wilks_lambda(&x, &y, None);
        }
        // ParCorrMult uses np.corrcoef, not scipy.stats.pearsonr. Preserve
        // covariance normalization and the two successive divisions by stddev.
        let nx = x.len();
        let ny = y.len();
        x.extend(y);
        let correlations = correlation_matrix(&mut x);
        let mut value: Option<f64> = None;
        for i in 0..nx {
            for j in nx..nx + ny {
                let r = correlations[(i, j)];
                // np.argmax chooses the first maximum, including the first NaN.
                if value.is_none_or(|v| !v.is_nan() && (r.is_nan() || r.abs() > v.abs())) {
                    value = Some(r);
                }
            }
        }
        Ok(value.expect("Samples guarantees X and Y"))
    }

    pub fn significance(&self, value: f64) -> f64 {
        self.significance_with(value, Correlation::MaxCorrelation)
    }

    pub fn significance_with(&self, value: f64, correlation: Correlation) -> f64 {
        let nx = self.roles.iter().filter(|r| **r == Role::X).count();
        let ny = self.roles.iter().filter(|r| **r == Role::Y).count();
        // Upstream does NOT clamp the Bonferroni-adjusted p-value to one.
        match correlation {
            Correlation::MaxCorrelation => {
                analytic_pvalue(value, self.rows[0].len(), self.rows.len()) * (nx * ny) as f64
            }
            Correlation::PccaWilksLambda => {
                let t = self.rows[0].len();
                if t <= self.rows.len() {
                    return f64::NAN;
                }
                let nz = self.rows.len() - nx - ny;
                let statistic = -((t - nz) as f64 - 0.5 * (nx + ny + 1) as f64) * value.ln();
                if statistic.is_nan() {
                    return f64::NAN;
                }
                if statistic <= 0.0 {
                    return 1.0;
                }
                // Preserve source's 1-cdf cancellation, not a substituted sf.
                1.0 - spec_math::cephes64::igam((nx * ny) as f64 / 2.0, statistic / 2.0)
            }
        }
    }

    /// CondIndTest.run_test's constant-component removal followed by max_corr.
    pub fn run_test(&self) -> Result<(f64, f64), SampleError> {
        self.run_test_with(Correlation::MaxCorrelation)
    }

    pub fn run_test_with(&self, correlation: Correlation) -> Result<(f64, f64), SampleError> {
        Ok(self.analytic_result(correlation)?.values())
    }

    pub fn analytic_result(&self, correlation: Correlation) -> Result<AnalyticResult, SampleError> {
        let Some(kept) = self.nonconstant()? else {
            return Ok(AnalyticResult::Constant);
        };
        let value = kept.dependence_with(correlation)?;
        Ok(AnalyticResult::Tested {
            value,
            p_value: kept.significance_with(value, correlation),
        })
    }

    pub fn run_shuffle_test(
        &self,
        correlation: Correlation,
        plan: &ShufflePlan,
        rng: &mut crate::nprandom::NpRng,
    ) -> Result<(f64, f64), SampleError> {
        let Some(mut kept) = self.nonconstant()? else {
            return Ok((0.0, 1.0));
        };
        // CondIndTest passes the array mutated by dependence evaluation to
        // significance evaluation; resetting it changes numerical trajectories.
        let value = kept.dependence_mut(correlation)?;
        let significance = kept.shuffle_significance(correlation, value, plan, rng)?;
        Ok((value, significance.p_value))
    }

    pub fn fixed_threshold(
        &self,
        correlation: Correlation,
        threshold: f64,
    ) -> Result<FixedDecision, SampleError> {
        Ok(self.fixed_statistic(correlation)?.decide(threshold))
    }

    pub fn fixed_statistic(&self, correlation: Correlation) -> Result<FixedStatistic, SampleError> {
        let Some(kept) = self.nonconstant()? else {
            return Ok(FixedStatistic::Constant);
        };
        let value = kept.dependence_with(correlation)?;
        // Source ParCorrMult declares two_sided=True for both correlation modes.
        Ok(FixedStatistic::Value(value))
    }

    pub(crate) fn nonconstant(&self) -> Result<Option<Self>, SampleError> {
        let mut rows = Vec::new();
        let mut roles = Vec::new();
        for (row, role) in self.rows.iter().zip(&self.roles) {
            let mean = crate::numpy_mean(row);
            let squares: Vec<_> = row.iter().map(|v| (v - mean).powi(2)).collect();
            let std = (crate::numpy_sum(&squares) / row.len() as f64).sqrt();
            if std != 0.0 {
                rows.push(row.clone());
                roles.push(*role);
            }
        }
        if !roles.contains(&Role::X) || !roles.contains(&Role::Y) {
            return Ok(None);
        }
        Self::new(rows, roles).map(Some)
    }

    /// get_model_selection_criterion after the DataFrame has constructed X/Y/Z.
    pub fn model_score(&self, corrected: bool) -> Result<f64, SampleError> {
        let mut work = self.clone();
        let residuals = work.residuals(Role::X)?;
        let t = self.rows[0].len() as f64;
        let p = (self.rows.len() - 1) as f64;
        let score: f64 = residuals
            .iter()
            .map(|row| {
                let rss = crate::numpy_sum(&row.iter().map(|v| v * v).collect::<Vec<_>>());
                t * rss.ln()
                    + 2.0 * p
                    + if corrected {
                        (2.0 * p * p + 2.0 * p) / (t - p - 1.0)
                    } else {
                        0.0
                    }
            })
            .sum();
        Ok(score / residuals.len() as f64)
    }

    /// Source residual-block shuffling with explicit or automatic block length. The RNG is
    /// borrowed so repeated tests consume one NumPy-compatible stream.
    pub fn shuffle_significance(
        &self,
        correlation: Correlation,
        value: f64,
        plan: &ShufflePlan,
        rng: &mut crate::nprandom::NpRng,
    ) -> Result<ShuffleResult, SampleError> {
        let t = self.rows[0].len();
        let mut work = self.clone();
        let mut original = work.residuals(Role::X)?;
        let nx = original.len();
        let y = work.residuals(Role::Y)?;
        let roles: Vec<_> = std::iter::repeat_n(Role::X, nx)
            .chain(std::iter::repeat_n(Role::Y, y.len()))
            .collect();
        original.extend(y);
        let mut shuffled = Samples::new(original.clone(), roles)?;
        let block = match plan.block {
            BlockLength::Fixed(length) => length.get(),
            BlockLength::Automatic => {
                shuffled
                    .automatic_block_length(block_length::BlockMode::Significance)
                    .map_err(SampleError::BlockLength)?
                    .length
            }
        };
        if block == 0 || block > t {
            return Err(SampleError::ShuffleConfiguration);
        }
        let starts: Vec<_> = (0..=t - block).step_by(block).collect();
        let full = (t / block) * block;
        let mut null_distribution = Vec::with_capacity(plan.samples);
        for _ in 0..plan.samples {
            let order = rng.permutation_of(&starts);
            let tail_at = if full < t {
                Some(starts[rng.bounded_uint64(starts.len() as u64) as usize])
            } else {
                None
            };
            for i in 0..nx {
                let mut row: Vec<_> = order
                    .iter()
                    .flat_map(|&start| original[i][start..start + block].iter().copied())
                    .collect();
                if let Some(at) = tail_at {
                    row.splice(at..at, original[i][full..].iter().copied());
                }
                shuffled.rows[i] = row;
            }
            // get_dependence_measure mutates array_shuffled. In particular, Y's
            // successive standardizations persist between shuffle iterations.
            null_distribution.push(shuffled.dependence_mut(correlation)?);
        }
        let extreme = null_distribution
            .iter()
            .filter(|v| v.abs() >= value.abs())
            .count();
        Ok(ShuffleResult {
            block_length: block,
            p_value: (extreme + 1) as f64 / (plan.samples + 1) as f64,
            null_distribution,
        })
    }
}

/// NumPy's unweighted covariance normalization and sequential stddev divisions.
/// Shared with CondIndTest's lag-by-lag ACF, which also calls np.corrcoef.
fn correlation_matrix(rows: &mut [Vec<f64>]) -> DMatrix<f64> {
    let t = rows[0].len();
    for row in rows.iter_mut() {
        let mean = crate::numpy_mean(row);
        for v in row {
            *v -= mean;
        }
    }
    let centered = DMatrix::from_fn(rows.len(), t, |i, j| rows[i][j]);
    let covariance = &centered * centered.transpose() * (1.0 / (t - 1) as f64);
    DMatrix::from_fn(rows.len(), rows.len(), |i, j| {
        (covariance[(i, j)] / covariance[(i, i)].sqrt() / covariance[(j, j)].sqrt())
            .clamp(-1.0, 1.0)
    })
}

pub struct ShufflePlan {
    samples: usize,
    block: BlockLength,
}

enum BlockLength {
    Automatic,
    Fixed(std::num::NonZeroUsize),
}

impl ShufflePlan {
    pub fn new(samples: usize, block: usize) -> Result<Self, SampleError> {
        if samples == usize::MAX {
            return Err(SampleError::ShuffleConfiguration);
        }
        Ok(Self {
            samples,
            block: BlockLength::Fixed(
                std::num::NonZeroUsize::new(block).ok_or(SampleError::ShuffleConfiguration)?,
            ),
        })
    }

    pub fn automatic(samples: usize) -> Result<Self, SampleError> {
        if samples == usize::MAX {
            return Err(SampleError::ShuffleConfiguration);
        }
        Ok(Self {
            samples,
            block: BlockLength::Automatic,
        })
    }
}

pub struct ShuffleResult {
    pub block_length: usize,
    pub p_value: f64,
    pub null_distribution: Vec<f64>,
}

/// `compute_wilks_lambda_cca`, including the source's lower-triangle eigvalsh
/// interpretation of M (M is not generally symmetric). Do not average triangles.
pub fn wilks_lambda(
    x: &[Vec<f64>],
    y: &[Vec<f64>],
    components: Option<usize>,
) -> Result<f64, SampleError> {
    let t = x.first().map_or(0, Vec::len);
    if t < 2
        || y.is_empty()
        || x.iter()
            .chain(y)
            .any(|r| r.len() != t || r.iter().any(|v| !v.is_finite()))
    {
        return Err(SampleError::Covariance);
    }
    let center = |rows: &[Vec<f64>]| {
        let mut out = DMatrix::from_fn(rows.len(), t, |i, j| rows[i][j]);
        // Explicit centering, followed by np.cov's own centering.
        for _ in 0..2 {
            for i in 0..out.nrows() {
                let mean = crate::numpy_mean(&out.row(i).iter().copied().collect::<Vec<_>>());
                for j in 0..t {
                    out[(i, j)] -= mean;
                }
            }
        }
        out
    };
    let x = center(x);
    let y = center(y);
    let factor = 1.0 / (t - 1) as f64;
    let mut cxx = &x * x.transpose() * factor;
    let mut cyy = &y * y.transpose() * factor;
    let cxy = &x * y.transpose() * factor;
    for i in 0..cxx.nrows() {
        cxx[(i, i)] += 1e-10;
    }
    for i in 0..cyy.nrows() {
        cyy[(i, i)] += 1e-10;
    }
    // scipy.linalg.inv uses a partial-pivot LU factorization.  In particular,
    // do not use nalgebra's fixed-size inverse path here: it loses substantial
    // accuracy for the regularized, nearly singular covariance matrices that
    // dummy/context variables can produce.
    let invert = |matrix: &DMatrix<f64>| {
        let faer_matrix = faer::Mat::from_fn(matrix.nrows(), matrix.ncols(), |i, j| matrix[(i, j)]);
        let inverse = faer_matrix.partial_piv_lu().inverse();
        let inverse = DMatrix::from_fn(matrix.nrows(), matrix.ncols(), |i, j| inverse[(i, j)]);
        inverse
            .iter()
            .all(|value| value.is_finite())
            .then_some(inverse)
    };
    let m = invert(&cxx).ok_or(SampleError::Covariance)?
        * &cxy
        * invert(&cyy).ok_or(SampleError::Covariance)?
        * cxy.transpose();
    if m.iter().any(|v| !v.is_finite()) {
        return Err(SampleError::Covariance);
    }
    let symmetric = DMatrix::from_fn(m.nrows(), m.ncols(), |i, j| {
        if i >= j {
            m[(i, j)]
        } else {
            m[(j, i)]
        }
    });
    let mut eigenvalues: Vec<_> = symmetric
        .symmetric_eigen()
        .eigenvalues
        .iter()
        .copied()
        .collect();
    eigenvalues.sort_by(|a, b| b.total_cmp(a));
    Ok(eigenvalues
        .iter()
        .take(components.unwrap_or(eigenvalues.len()))
        .map(|v| 1.0 - v.max(0.0).abs())
        .product())
}
