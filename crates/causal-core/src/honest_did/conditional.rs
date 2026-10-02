//! ARP conditional moment-inequality calculations from HonestDiD 0.2.8.
//! The public confidence set retains its complete evaluated grid.
use crate::honest_did::{flci::type_seven_quantile, r_rng::RRng, relative_magnitude_constraints, Error};
use ecos_reference_port::Matrix;
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Copy, Debug)]
pub enum ConditionalMethod {
    Conditional,
    LeastFavorableHybrid { kappa: f64 },
}
#[derive(Clone, Debug)]
pub struct ConfidenceGrid {
    grid: Vec<f64>,
    accepted: Vec<bool>,
}
impl ConfidenceGrid {
    pub fn grid(&self) -> &[f64] {
        &self.grid
    }
    pub fn accepted(&self) -> &[bool] {
        &self.accepted
    }
    /// Envelope of the accepted grid points, not a claim that the set is connected.
    pub fn bounds(&self) -> Option<(f64, f64)> {
        let first = self.accepted.iter().position(|x| *x)?;
        let last = self.accepted.iter().rposition(|x| *x)?;
        Some((self.grid[first], self.grid[last]))
    }
    pub fn open_endpoints(&self) -> (bool, bool) {
        (
            self.accepted.first() == Some(&true),
            self.accepted.last() == Some(&true),
        )
    }
    pub fn length(&self) -> f64 {
        self.grid
            .windows(2)
            .zip(self.accepted.windows(2))
            .map(|(g, a)| (g[1] - g[0]) * (u8::from(a[0]) as f64 + u8::from(a[1]) as f64) * 0.5)
            .sum()
    }
}
fn failure(message: &str) -> Error {
    Error::Optimization(crate::honest_did::optimization::Failure::DidNotConverge(message.into()))
}
fn rows(a: &DMatrix<f64>) -> Vec<Vec<f64>> {
    (0..a.nrows())
        .map(|i| (0..a.ncols()).map(|j| a[(i, j)]).collect())
        .collect()
}
fn lp(
    cost: &DVector<f64>,
    workspace: &mut ecos_reference_port::LinearObjectiveWorkspace,
) -> Result<ecos_reference_port::Solution, Error> {
    let out = workspace.solve(cost.as_slice()).map_err(failure)?;
    match out.status {
        0 | 10 => Ok(out),
        1 | 11 => Err(Error::Optimization(
            crate::honest_did::optimization::Failure::Infeasible,
        )),
        2 | 12 => Err(Error::Optimization(crate::honest_did::optimization::Failure::Unbounded)),
        _ => Err(failure("The conditional linear program did not converge.")),
    }
}
fn sf(x: f64) -> f64 {
    0.5 * libm::erfc(x / std::f64::consts::SQRT_2)
}
fn qfun(x: f64) -> f64 {
    if x < 35.0 {
        return (0.5 * x * x).exp() * sf(x);
    }
    let mut sum = 1.0;
    let mut term = 1.0;
    for k in 1..=12 {
        term *= -(2 * k - 1) as f64 / (x * x);
        sum += term;
    }
    sum / (std::f64::consts::TAU.sqrt() * x)
}
/// TruncatedNormal 2.3's cases/Phinv/normq/newton scalar calculation.
pub fn truncated_normal_quantile(p: f64, lower: f64, upper: f64) -> Result<f64, Error> {
    if !p.is_finite()
        || !(0.0..=1.0).contains(&p)
        || lower.is_nan()
        || upper.is_nan()
        || lower > upper
    {
        return Err(Error::InvalidInput(
            "Invalid truncated normal probability or bounds.",
        ));
    }
    if p == 0.0 || lower == upper {
        return Ok(lower);
    }
    if p == 1.0 {
        return Ok(upper);
    }
    if upper < -35.0 {
        return truncated_normal_quantile(1.0 - p, -upper, -lower).map(|x| -x);
    }
    if lower > 35.0 {
        let initial = (lower * lower
            - 2.0 * (p * (0.5 * lower * lower - 0.5 * upper * upper).exp_m1()).ln_1p())
        .sqrt();
        if lower > 1e5 {
            return Ok(initial);
        }
        let ql = qfun(lower);
        let qu = if upper.is_infinite() {
            0.0
        } else {
            qfun(upper)
        };
        let mut x = initial;
        for _ in 0..1000 {
            let du = -qfun(x)
                + (1.0 - p) * (0.5 * (x * x - lower * lower)).exp() * ql
                + if qu == 0.0 {
                    0.0
                } else {
                    p * (0.5 * (x * x - upper * upper)).exp() * qu
                };
            x -= du;
            if du.abs() <= 1e-10 {
                return Ok(x);
            }
        }
        return Err(failure("The truncated normal quantile did not converge."));
    }
    let flip = upper < 0.0;
    let (l, u) = if flip {
        (-lower, -upper)
    } else {
        (lower, upper)
    };
    let pl = sf(l);
    let probability = pl + (sf(u) - pl) * p;
    let x = -crate::honest_did::r_rng::standard_normal_quantile(probability);
    Ok(if flip { -x } else { x })
}
fn covariance_root(sigma: &DMatrix<f64>) -> Result<DMatrix<f64>, Error> {
    if sigma.nrows() == 0 || sigma.nrows() != sigma.ncols() || sigma.iter().any(|v| !v.is_finite())
    {
        return Err(Error::InvalidInput(
            "The simulated covariance must be square and finite.",
        ));
    }
    let scale = sigma.amax();
    if scale == 0.0 || (sigma - sigma.transpose()).amax() > scale * 1e-12 {
        return Err(Error::InvalidInput(
            "The simulated covariance must be symmetric and nonzero.",
        ));
    }
    let n = sigma.nrows();
    let mut vectors = sigma.clone();
    let mut values = vec![0.0; n];
    let info = crate::honest_did::lapack_dsyevd::dsyevd(
        crate::honest_did::lapack_dsyevd::EigenJob::Vectors,
        crate::honest_did::lapack_dsyevd::Triangle::Lower,
        n,
        vectors.as_mut_slice(),
        n,
        &mut values,
    )
    .map_err(|_| failure("The simulated covariance could not be decomposed."))?;
    if info != 0 {
        return Err(failure("The covariance eigensolver did not converge."));
    }
    if values.iter().any(|v| *v < -scale * 1e-12) {
        return Err(Error::InvalidInput(
            "The simulated covariance must be positive semidefinite.",
        ));
    }
    let weights = DVector::from_iterator(n, values.into_iter().map(|v| v.max(0.0).sqrt()));
    Ok(&vectors * DMatrix::from_diagonal(&weights) * vectors.transpose())
}
#[derive(Debug)]
pub struct Minimax {
    pub eta: f64,
    pub delta: Vec<f64>,
    pub lambda: Vec<f64>,
}
pub fn minimax(y: &DVector<f64>, x: &DMatrix<f64>, sigma: &DMatrix<f64>) -> Result<Minimax, Error> {
    let _ = covariance_root(sigma)?;
    minimax_for_moments(y, x, sigma)
}
fn minimax_for_moments(
    y: &DVector<f64>,
    x: &DMatrix<f64>,
    sigma: &DMatrix<f64>,
) -> Result<Minimax, Error> {
    MomentProblem::new(x, sigma)?.solve(y)
}

/// Invariant LP coefficients shared by grid points and seeded simulated draws.
/// The solver still receives a fresh model with the same cold-start pivot rules.
struct MomentProblem {
    w: DMatrix<f64>,
    negative_w: DMatrix<f64>,
    cost: DVector<f64>,
}
impl MomentProblem {
    fn new(x: &DMatrix<f64>, sigma: &DMatrix<f64>) -> Result<Self, Error> {
        let m = x.nrows();
        let k = x.ncols();
        if m == 0 || sigma.shape() != (m, m)
            || x.iter().chain(sigma.iter()).any(|v| !v.is_finite()) {
            return Err(Error::InvalidInput("The moment values, nuisance design and covariance must have matching finite dimensions."));
        }
        let mut w = DMatrix::zeros(m, k + 1);
        for i in 0..m {
            if sigma[(i, i)] <= 0.0 {
                return Err(Error::InvalidInput("Every tested moment needs positive variance."));
            }
            w[(i, 0)] = sigma[(i, i)].sqrt();
            for j in 0..k { w[(i, j + 1)] = x[(i, j)]; }
        }
        let mut cost = DVector::zeros(k + 1);
        cost[0] = 1.0;
        let negative_w = -&w;
        Ok(Self { w, negative_w, cost })
    }
    fn solve(&self, y: &DVector<f64>) -> Result<Minimax, Error> {
        if y.len() != self.w.nrows() || y.iter().any(|v| !v.is_finite()) {
            return Err(Error::InvalidInput("The moment values, nuisance design and covariance must have matching finite dimensions."));
        }
        let (eta, parameters, lambda) = crate::honest_did::lpsolve::minimize(
            self.cost.as_slice(), &self.negative_w, (-y).as_slice())?;
        Ok(Minimax { eta, delta: parameters[1..].to_vec(), lambda })
    }
}
pub fn least_favorable_critical_value(
    x: Option<&DMatrix<f64>>,
    sigma: &DMatrix<f64>,
    kappa: f64,
    simulations: usize,
    seed: u32,
) -> Result<f64, Error> {
    if sigma.nrows() == 0
        || sigma.nrows() != sigma.ncols()
        || !(0.0..1.0).contains(&kappa)
        || kappa == 0.0
        || simulations < 2
    {
        return Err(Error::InvalidInput(
            "Invalid least-favorable simulation configuration.",
        ));
    }
    let root = covariance_root(sigma)?;
    let mut rng = RRng::new(seed);
    let n = sigma.nrows();
    let mut statistics = Vec::with_capacity(simulations);
    if (0..n).any(|i| sigma[(i, i)] <= 0.0)
        || x.is_some_and(|v| v.nrows() != n || v.iter().any(|v| !v.is_finite()))
    {
        return Err(Error::InvalidInput("The simulated moments require positive variances and a matching finite nuisance design."));
    }
    let problem = x.map(|x| MomentProblem::new(x, sigma)).transpose()?;
    for _ in 0..simulations {
        let normal = DVector::from_iterator(n, (0..n).map(|_| rng.normal()));
        let draw = &root * normal;
        statistics.push(if let Some(problem) = &problem {
            problem.solve(&draw)?.eta
        } else {
            (0..n)
                .map(|i| draw[i] / sigma[(i, i)].sqrt())
                .fold(f64::NEG_INFINITY, f64::max)
        });
    }
    Ok(type_seven_quantile(&mut statistics, 1.0 - kappa))
}

/// Conditional ARP test of the supplied moment inequalities, without a hybrid.
pub fn conditional_moment_test(
    y: &DVector<f64>,
    x: &DMatrix<f64>,
    sigma: &DMatrix<f64>,
    alpha: f64,
) -> Result<bool, Error> {
    if !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
        return Err(Error::InvalidInput(
            "The test size must lie strictly between zero and one.",
        ));
    }
    let _ = covariance_root(sigma)?;
    nuisance_reject(y, x, sigma, alpha, None)
}

fn no_nuisance_reject(
    y: &DVector<f64>,
    sigma: &DMatrix<f64>,
    normalized: &DMatrix<f64>,
    alpha: f64,
    hybrid: Option<(f64, f64)>,
) -> Result<bool, Error> {
    let moments = normalized * y;
    let index = (0..moments.len())
        .max_by(|i, j| moments[*i].total_cmp(&moments[*j]).then_with(|| j.cmp(i)))
        .unwrap();
    let stat = moments[index];
    if hybrid.is_some_and(|(_, cv)| stat > cv) {
        return Ok(true);
    }
    let gamma = normalized.row(index).transpose();
    let variance = gamma.dot(&(sigma * &gamma));
    let c = sigma * &gamma / variance;
    let z = y - &c * gamma.dot(y);
    let selected = normalized.row(index);
    let mut lower = f64::NEG_INFINITY;
    let mut upper = f64::INFINITY;
    for i in 0..normalized.nrows() {
        let row = normalized.row(i) - selected;
        let ac = (row.clone_owned() * &c)[0];
        let az = (row * &z)[0];
        if ac < 0.0 {
            lower = lower.max(-az / ac);
        }
        if ac > 0.0 {
            upper = upper.min(-az / ac);
        }
    }
    // The no-nuisance LF implementation in 0.2.8 adjusts size, but does not
    // additionally cap its truncation bound by lf_cv. Retain the reference.
    let size = hybrid.map_or(alpha, |(k, _)| (alpha - k) / (1.0 - k));
    let critical =
        truncated_normal_quantile(1.0 - size, lower / variance.sqrt(), upper / variance.sqrt())?
            .max(0.0)
            * variance.sqrt();
    Ok(stat > critical)
}

fn dual_bounds(
    eta: f64,
    y: &DVector<f64>,
    sigma: &DMatrix<f64>,
    gamma: &DVector<f64>,
    w: &DMatrix<f64>,
) -> Result<(f64, f64), Error> {
    let variance = gamma.dot(&(sigma * gamma));
    let c = sigma * gamma / variance;
    let s = y - &c * gamma.dot(y);
    let equality = w.transpose();
    let mut rhs = DVector::zeros(w.ncols());
    rhs[0] = 1.0;
    // Only the objective changes during dual-bound search. Preserve each ECOS
    // cold start and numerical factorization; prepare fixed constraints once.
    let g = Matrix::from_rows(&rows(&(-DMatrix::identity(y.len(), y.len()))), y.len())
        .map_err(failure)?;
    let equality = Matrix::from_rows(&rows(&equality), y.len()).map_err(failure)?;
    let zero = DVector::zeros(y.len());
    let mut workspace = ecos_reference_port::LinearObjectiveWorkspace::new(
        &g, zero.as_slice(), &equality, rhs.as_slice(),
    ).map_err(failure)?;
    let mut solve = |v: f64| -> Result<(bool, DVector<f64>), Error> {
        let f = &s + &c * v;
        let out = lp(&(-f), &mut workspace)?;
        Ok(((v + out.objective).abs() <= 1e-6, DVector::from_vec(out.x)))
    };
    if !solve(eta)?.0 {
        return Ok((eta, f64::INFINITY));
    }
    let se = variance.sqrt();
    let mut output = [f64::NEG_INFINITY, f64::INFINITY];
    for side in 0..2 {
        let edge = if side == 0 {
            (-100.0_f64).min(eta - 20.0 * se)
        } else {
            100.0_f64.max(eta + 20.0 * se)
        };
        let (okay, mut weights) = solve(edge)?;
        if okay {
            continue;
        }
        let update = |weights: &DVector<f64>| {
            let numerator = weights.dot(&s);
            let numerator = if numerator.abs() < f64::EPSILON.powf(0.75) {
                0.0
            } else {
                numerator
            };
            numerator / (1.0 - weights.dot(&c))
        };
        let mut midpoint = update(&weights);
        let mut iterations = 1;
        let mut bisect = false;
        loop {
            let (okay, new_weights) = solve(midpoint)?;
            weights = new_weights;
            if okay {
                break;
            }
            iterations += 1;
            if iterations >= 10 {
                bisect = true;
                break;
            }
            midpoint = update(&weights);
        }
        if bisect {
            let (mut low, mut high) = if side == 0 {
                (midpoint, eta)
            } else {
                (eta, midpoint)
            };
            while high - low > 1e-6 && iterations < 10000 {
                midpoint = (low + high) * 0.5;
                iterations += 1;
                let okay = solve(midpoint)?.0;
                if okay == (side == 0) {
                    high = midpoint;
                } else {
                    low = midpoint;
                }
            }
            if iterations >= 10000 {
                return Err(failure("Conditional dual bounds did not converge."));
            }
        }
        output[side] = midpoint;
    }
    Ok((output[0], output[1]))
}
fn nuisance_reject(
    y: &DVector<f64>,
    x: &DMatrix<f64>,
    sigma: &DMatrix<f64>,
    alpha: f64,
    hybrid: Option<(f64, f64)>,
) -> Result<bool, Error> {
    nuisance_reject_prepared(y, &MomentProblem::new(x, sigma)?, sigma, alpha, hybrid)
}
fn nuisance_reject_prepared(
    y: &DVector<f64>,
    problem: &MomentProblem,
    sigma: &DMatrix<f64>,
    alpha: f64,
    hybrid: Option<(f64, f64)>,
) -> Result<bool, Error> {
    let solution = problem.solve(y)?;
    if hybrid.is_some_and(|(_, cv)| solution.eta > cv) {
        return Ok(true);
    }
    let m = y.len();
    let w = &problem.w;
    let binding: Vec<_> = solution
        .lambda
        .iter()
        .enumerate()
        .filter_map(|(i, v)| (*v > 1e-6).then_some(i))
        .collect();
    let inverse = if binding.len() == w.ncols() {
        DMatrix::from_fn(binding.len(), w.ncols(), |i, j| w[(binding[i], j)]).try_inverse()
    } else {
        None
    };
    let (gamma, lower, mut upper) = if let Some(inverse) = inverse {
        let mut projection = DMatrix::zeros(w.ncols(), m);
        for (j, i) in binding.iter().enumerate() {
            projection.column_mut(*i).copy_from(&inverse.column(j));
        }
        let gamma = projection.row(0).transpose();
        let variance = gamma.dot(&(sigma * &gamma));
        if variance <= 0.0 {
            return Err(failure("The conditional binding variance is not positive."));
        }
        let mut lower = f64::NEG_INFINITY;
        let mut upper = f64::INFINITY;
        for i in 0..m {
            if binding.contains(&i) {
                continue;
            }
            let mut row = (w.row(i) * &projection).transpose();
            row[i] -= 1.0;
            let rho = row.dot(&(sigma * &gamma)) / variance;
            let bound = -row.dot(y) / rho + gamma.dot(y);
            if rho > 0.0 {
                lower = lower.max(bound);
            }
            if rho < 0.0 {
                upper = upper.min(bound);
            }
        }
        (gamma, lower, upper)
    } else {
        let gamma = DVector::from_vec(solution.lambda);
        let variance = gamma.dot(&(sigma * &gamma));
        if variance.abs() < f64::EPSILON {
            return Ok(solution.eta > 0.0);
        }
        if variance < 0.0 {
            return Err(failure("The conditional dual variance is negative."));
        }
        let (lower, upper) = dual_bounds(solution.eta, y, sigma, &gamma, w)?;
        (gamma, lower, upper)
    };
    if let Some((_, cv)) = hybrid {
        upper = upper.min(cv);
    }
    let sd = gamma.dot(&(sigma * &gamma)).sqrt();
    let statistic = solution.eta / sd;
    let (lower, upper) = (lower / sd, upper / sd);
    // HonestDiD explicitly returns non-rejection when its conditional statistic
    // is outside the computed truncation interval. Preserve this test rule;
    // optimization failures themselves remain errors, not accepted grid points.
    if !(lower <= statistic && statistic <= upper) {
        return Ok(false);
    }
    let size = hybrid.map_or(alpha, |(k, _)| (alpha - k) / (1.0 - k));
    Ok(statistic > truncated_normal_quantile(1.0 - size, lower, upper)?.max(0.0))
}

/// Union the conditional confidence sets across candidate maximum
/// pre-treatment differences and their signs, as computeConditionalCS_DeltaRM.
pub fn relative_magnitude_confidence_set(
    study: &crate::honest_did::EventStudy,
    specification: crate::honest_did::RelativeMagnitudeSpecification,
) -> Result<ConfidenceGrid, Error> {
    let problem = study.affine_problem();
    let periods = problem.periods();
    let beta = study.coefficients().to_vec();
    let sigma = problem.covariance().clone();
    let contrast = problem.contrast().to_vec();
    let crate::honest_did::RelativeMagnitudeSpecification {
        bound,
        method,
        moments,
        grid: requested_grid,
        seed,
        alpha,
    } = specification;
    let post_only = matches!(moments, crate::honest_did::MomentSelection::PostTreatment);
    let kappa = match method {
        ConditionalMethod::Conditional => None,
        ConditionalMethod::LeastFavorableHybrid { kappa } => {
            if !kappa.is_finite() || kappa <= 0.0 || kappa >= alpha {
                return Err(Error::InvalidInput(
                    "The hybrid size must be positive and smaller than alpha.",
                ));
            }
            Some(kappa)
        }
    };
    let pre = periods.pre();
    let post = periods.post();
    if post == 1 && contrast[0] != 1.0 {
        return Err(Error::InvalidInput(
            "The reference's single-post-period conditional route supports its unit contrast only.",
        ));
    }
    let l = DVector::from_vec(contrast);
    let sd = (l.transpose() * sigma.view((pre, pre), (post, post)) * &l)[0].sqrt();
    let (lower, upper, points) = match requested_grid {
        crate::honest_did::GridSpecification::ReferenceDefault { points } => (-20.0 * sd, 20.0 * sd, points),
        crate::honest_did::GridSpecification::Explicit {
            lower,
            upper,
            points,
        } => (lower, upper, points),
    };
    if !lower.is_finite() || !upper.is_finite() || lower >= upper {
        return Err(Error::InvalidInput(
            "The confidence-set grid needs increasing finite endpoints.",
        ));
    }
    let grid: Vec<_> = (0..points)
        .map(|i| {
            if i + 1 == points {
                upper
            } else {
                lower + (upper - lower) * i as f64 / (points - 1) as f64
            }
        })
        .collect();
    let mut accepted = vec![false; points];
    let beta = DVector::from_vec(beta);
    // pracma::rref([l I]) retains l then all unit vectors except the first
    // nonzero entry of l. This is the same pivot-column basis without a duplicate
    // generic RREF implementation.
    let pivot = l
        .iter()
        .position(|v| v.abs() > 1e-12)
        .ok_or(Error::InvalidInput("The target contrast must not be zero."))?;
    let mut gamma = DMatrix::zeros(post, post);
    gamma.row_mut(0).copy_from(&l.transpose());
    let mut row = 1;
    for j in 0..post {
        if j != pivot {
            gamma[(row, j)] = 1.0;
            row += 1;
        }
    }
    let inverse = gamma
        .try_inverse()
        .ok_or(Error::InvalidInput("The target basis is singular."))?;
    for difference in 0..pre {
        for positive in [true, false] {
            let all = relative_magnitude_constraints(periods, bound, difference, positive)?;
            let retained: Vec<_> = (0..all.nrows())
                .filter(|i| !post_only || (pre..periods.total()).any(|j| all[(*i, j)] != 0.0))
                .collect();
            let a = DMatrix::from_fn(retained.len(), all.ncols(), |i, j| all[(retained[i], j)]);
            if post == 1 {
                let moment_covariance = &a * &sigma * a.transpose();
                let mut normalized = a.clone();
                for i in 0..a.nrows() {
                    let sd = moment_covariance[(i, i)].sqrt();
                    if sd <= 0.0 { return Err(Error::InvalidInput("A tested inequality has zero variance.")); }
                    normalized.row_mut(i).scale_mut(1.0 / sd);
                }
                let hybrid = if let Some(k) = kappa {
                    Some((
                        k,
                        least_favorable_critical_value(
                            None,
                            &moment_covariance,
                            k,
                            1000,
                            seed,
                        )?,
                    ))
                } else {
                    None
                };
                for (i, theta) in grid.iter().enumerate() {
                    if accepted[i] {
                        continue;
                    }
                    let mut y = beta.clone();
                    y[pre] -= theta;
                    if !no_nuisance_reject(&y, &sigma, &normalized, alpha, hybrid)? {
                        accepted[i] = true;
                    }
                }
            } else {
                let target = a.view((0, pre), (a.nrows(), post)) * &inverse;
                let x = target.columns(1, post - 1).into_owned();
                let moment_covariance = &a * &sigma * a.transpose();
                let y = &a * &beta;
                let problem = MomentProblem::new(&x, &moment_covariance)?;
                // The multi-post-period wrapper omits seed at this call in version 0.2.8.
                let hybrid = if let Some(k) = kappa {
                    Some((
                        k,
                        least_favorable_critical_value(Some(&x), &moment_covariance, k, 1000, 0)?,
                    ))
                } else {
                    None
                };
                for (i, theta) in grid.iter().enumerate() {
                    if accepted[i] {
                        continue;
                    }
                    let values = &y - target.column(0) * (*theta);
                    if !nuisance_reject_prepared(&values, &problem, &moment_covariance, alpha, hybrid)? {
                        accepted[i] = true;
                    }
                }
            }
        }
    }
    Ok(ConfidenceGrid { grid, accepted })
}
