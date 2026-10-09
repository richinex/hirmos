//! Solve convex quadratic programs with non-negative weights summing to one.
use crate::lapack_lu::{dgetrf_fused, dgetrs_fused, Transpose};
use nalgebra::{DMatrix, DVector};

fn multiply(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<DVector<f64>, Error> {
    use crate::lapack_dgelsd::blas::{dgemm_fused, Transpose};
    let mut result = DVector::zeros(a.nrows());
    dgemm_fused(
        Transpose::None,
        Transpose::None,
        a.nrows(),
        1,
        a.ncols(),
        1.0,
        a.as_slice(),
        a.nrows(),
        b.as_slice(),
        b.len(),
        0.0,
        result.as_mut_slice(),
        a.nrows(),
    )
    .map_err(|_| Error::InvalidInput)?;
    Ok(result)
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInput,
    SingularSystem,
    IllConditionedSystem { reciprocal_condition: f64 },
    NonFinite,
}

#[derive(Debug)]
pub enum Outcome {
    Converged(Fit),
    IterationLimit(Fit),
}

#[derive(Debug)]
pub struct Fit {
    pub weights: Vec<f64>,
    pub iterations: usize,
    pub primal_residual: f64,
    pub dual_residual: f64,
    pub relative_gap: f64,
}

impl Outcome {
    pub fn snapshot(&self) -> &Fit {
        match self {
            Self::Converged(fit) | Self::IterationLimit(fit) => fit,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum System {
    Full,
    Reduced,
}

#[derive(Clone, Copy, Debug)]
pub enum BarrierRule {
    PredictorCentered,
    Carried,
}

#[derive(Clone, Copy)]
pub enum LinearSolvePolicy {
    Pivoted,
    ConditionChecked,
}

enum BarrierState {
    PredictorCentered,
    Carried(f64),
}

impl BarrierState {
    fn target(&mut self, mean: f64, predictor_step: f64) -> f64 {
        match self {
            Self::PredictorCentered => {
                mean * ((1.0 - predictor_step) / (1.0 + 10.0 * predictor_step)).powi(2)
            }
            Self::Carried(current) => {
                let target = *current;
                *current *= ((1.0 - predictor_step) / (10.0 + predictor_step)).powi(2);
                target
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Settings {
    pub linear_solve: LinearSolvePolicy,
    pub system: System,
    pub barrier: BarrierRule,
    pub initial_floor: f64,
    pub step_fraction: f64,
    pub gap_tolerance: f64,
    pub feasibility_tolerance: f64,
    pub max_iterations: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            linear_solve: LinearSolvePolicy::Pivoted,
            system: System::Full,
            barrier: BarrierRule::PredictorCentered,
            initial_floor: 100.0,
            step_fraction: 0.95,
            gap_tolerance: 1e-8,
            feasibility_tolerance: 1e-6,
            max_iterations: 1000,
        }
    }
}

impl Settings {
    pub fn synthetic_control() -> Self {
        Self {
            linear_solve: LinearSolvePolicy::ConditionChecked,
            system: System::Reduced,
            barrier: BarrierRule::Carried,
            initial_floor: 10.0,
            step_fraction: 0.9995,
            gap_tolerance: 1e-5,
            ..Self::default()
        }
    }
}

enum NewtonFactor {
    Full(Factor),
    Reduced { factor: Factor, equality_scale: f64 },
}

impl NewtonFactor {
    fn new(
        system: System,
        h: &DMatrix<f64>,
        linear: &DMatrix<f64>,
        point: &DVector<f64>,
        pairs: &[(usize, usize)],
        policy: LinearSolvePolicy,
    ) -> Result<Self, Error> {
        let n = h.nrows();
        match system {
            System::Full => {
                let mut jacobian = linear.clone();
                for (k, &(a, b)) in pairs.iter().enumerate() {
                    jacobian[(3 * n + 3 + k, a)] = point[b];
                    jacobian[(3 * n + 3 + k, b)] = point[a];
                }
                Ok(Self::Full(Factor::with_policy(jacobian, policy)?))
            }
            System::Reduced => {
                let equality_scale = 1.0
                    / (point[5 * n + 3] / point[5 * n + 1] + point[5 * n + 4] / point[5 * n + 2]);
                let mut reduced = DMatrix::zeros(n + 1, n + 1);
                for i in 0..n {
                    for j in 0..n {
                        reduced[(i, j)] = -h[(i, j)];
                    }
                    reduced[(i, i)] -=
                        point[3 * n + i] / point[n + i] + point[4 * n + i] / point[2 * n + i];
                    reduced[(i, n)] = 1.0;
                    reduced[(n, i)] = 1.0;
                }
                reduced[(n, n)] = equality_scale;
                Ok(Self::Reduced {
                    factor: Factor::with_policy(reduced, policy)?,
                    equality_scale,
                })
            }
        }
    }
    fn solve_normalized(
        &self,
        point: &DVector<f64>,
        mut rhs: DVector<f64>,
        gamma: &[f64],
    ) -> Result<DVector<f64>, Error> {
        match self {
            Self::Full(factor) => {
                let n = (point.len() - 5) / 5;
                for (k, &(a, _)) in (Layout { n }).pairs().iter().enumerate() {
                    rhs[3 * n + 3 + k] = point[a] * gamma[k];
                }
                factor.solve(rhs)
            }
            Self::Reduced {
                factor,
                equality_scale,
            } => {
                let n = (point.len() - 5) / 5;
                let (w, p, v, q) = (
                    point[5 * n + 1],
                    point[5 * n + 2],
                    point[5 * n + 3],
                    point[5 * n + 4],
                );
                let alpha_hat = rhs[3 * n + 1] - gamma[2 * n + 1] * p / q;
                let beta_hat = -rhs[3 * n + 2] - gamma[2 * n] * v / w;
                let mut reduced_rhs = DVector::zeros(n + 1);
                let mut nu_hat = DVector::zeros(n);
                let mut tau_hat = DVector::zeros(n);
                for i in 0..n {
                    nu_hat[i] = rhs[n + 1 + i] + gamma[i] * point[n + i] / point[3 * n + i];
                    tau_hat[i] =
                        rhs[2 * n + 1 + i] - gamma[n + i] * point[2 * n + i] / point[4 * n + i];
                    reduced_rhs[i] = -rhs[i]
                        - nu_hat[i] * point[3 * n + i] / point[n + i]
                        - tau_hat[i] * point[4 * n + i] / point[2 * n + i];
                }
                reduced_rhs[n] = rhs[n] - equality_scale * (beta_hat - alpha_hat * q / p);
                let direction = factor.solve(reduced_rhs)?;
                let mut result = DVector::zeros(point.len());
                result.rows_mut(0, n).copy_from(&direction.rows(0, n));
                result[5 * n] = direction[n];
                result[5 * n + 1] = -equality_scale * (beta_hat - alpha_hat * q / p + direction[n]);
                result[5 * n + 4] = (result[5 * n + 1] - alpha_hat) * q / p;
                result[5 * n + 2] = (gamma[2 * n + 1] - result[5 * n + 4]) * p / q;
                result[5 * n + 3] = (gamma[2 * n] - result[5 * n + 1]) * v / w;
                for i in 0..n {
                    result[3 * n + i] =
                        (nu_hat[i] - direction[i]) * point[3 * n + i] / point[n + i];
                    result[4 * n + i] =
                        (direction[i] - tau_hat[i]) * point[4 * n + i] / point[2 * n + i];
                    result[n + i] =
                        (gamma[i] - result[3 * n + i]) * point[n + i] / point[3 * n + i];
                    result[2 * n + i] =
                        (gamma[n + i] - result[4 * n + i]) * point[2 * n + i] / point[4 * n + i];
                }
                if result.iter().any(|v| !v.is_finite()) {
                    return Err(Error::NonFinite);
                }
                Ok(result)
            }
        }
    }
}

struct Factor {
    lu: DMatrix<f64>,
    pivots: Vec<usize>,
}

impl Factor {
    fn with_policy(matrix: DMatrix<f64>, policy: LinearSolvePolicy) -> Result<Self, Error> {
        match policy {
            LinearSolvePolicy::Pivoted => Self::new(matrix),
            LinearSolvePolicy::ConditionChecked => Self::conditioned(matrix),
        }
    }
    fn conditioned(matrix: DMatrix<f64>) -> Result<Self, Error> {
        let n = matrix.nrows();
        let norm = (0..n)
            .map(|j| (0..n).map(|i| matrix[(i, j)].abs()).sum::<f64>())
            .fold(0.0, f64::max);
        let factor = Self::new(matrix)?;
        let reciprocal_condition =
            crate::lapack_lu::reciprocal_condition_one_fused(n, factor.lu.as_slice(), n, norm)
                .map_err(|_| Error::NonFinite)?;
        if reciprocal_condition < f64::EPSILON {
            return Err(Error::IllConditionedSystem {
                reciprocal_condition,
            });
        }
        Ok(factor)
    }
    fn new(mut matrix: DMatrix<f64>) -> Result<Self, Error> {
        let n = matrix.nrows();
        let mut pivots = vec![0; n];
        let info = dgetrf_fused(n, n, matrix.as_mut_slice(), n, &mut pivots)
            .map_err(|_| Error::InvalidInput)?;
        if info != 0 {
            return Err(Error::SingularSystem);
        }
        Ok(Self { lu: matrix, pivots })
    }
    fn solve(&self, mut rhs: DVector<f64>) -> Result<DVector<f64>, Error> {
        let n = rhs.len();
        dgetrs_fused(
            Transpose::None,
            n,
            1,
            self.lu.as_slice(),
            n,
            &self.pivots,
            rhs.as_mut_slice(),
            n,
        )
        .map_err(|_| Error::InvalidInput)?;
        if rhs.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        Ok(rhs)
    }
}

struct Layout {
    n: usize,
}

struct Iterate {
    values: DVector<f64>,
    hx: DVector<f64>,
}
impl Iterate {
    fn new(h: &DMatrix<f64>, values: DVector<f64>) -> Result<Self, Error> {
        let hx = multiply(h, &values.rows(0, h.ncols()).into_owned())?;
        Ok(Self { values, hx })
    }
    fn advance(
        &mut self,
        h: &DMatrix<f64>,
        direction: &DVector<f64>,
        fraction: f64,
    ) -> Result<(), Error> {
        self.values += fraction * direction;
        self.hx = multiply(h, &self.values.rows(0, h.ncols()).into_owned())?;
        Ok(())
    }
}

impl Layout {
    fn residual(
        &self,
        hx: &DVector<f64>,
        c: &DVector<f64>,
        point: &DVector<f64>,
    ) -> Result<DVector<f64>, Error> {
        let n = self.n;
        let x = point.rows(0, n).into_owned();
        let sum = multiply(&DMatrix::from_element(1, n, 1.0), &x)?[0];
        let mut result = DVector::zeros(self.size());
        for i in 0..n {
            result[i] = c[i] - point[5 * n] - point[3 * n + i] + point[4 * n + i] + hx[i];
            result[n + 1 + i] = -(-point[i] + point[n + i]);
            result[2 * n + 1 + i] = -(1.0 - point[i] - point[2 * n + i]);
        }
        result[n] = -(1.0 - sum + point[5 * n + 1]);
        result[3 * n + 1] = point[5 * n + 1] + point[5 * n + 2];
        result[3 * n + 2] = point[5 * n] + point[5 * n + 4] - point[5 * n + 3];
        Ok(result)
    }
    fn size(&self) -> usize {
        5 * self.n + 5
    }
    fn pairs(&self) -> Vec<(usize, usize)> {
        (0..self.n)
            .map(|i| (self.n + i, 3 * self.n + i))
            .chain((0..self.n).map(|i| (2 * self.n + i, 4 * self.n + i)))
            .chain([
                (5 * self.n + 3, 5 * self.n + 1),
                (5 * self.n + 2, 5 * self.n + 4),
            ])
            .collect()
    }
    fn linear_equations(&self, h: &DMatrix<f64>, c: &DVector<f64>) -> (DMatrix<f64>, DVector<f64>) {
        let n = self.n;
        let mut a = DMatrix::zeros(self.size(), self.size());
        let mut b = DVector::zeros(self.size());
        for i in 0..n {
            for j in 0..n {
                a[(i, j)] = h[(i, j)];
            }
            a[(i, 5 * n)] = -1.0;
            a[(i, 3 * n + i)] = -1.0;
            a[(i, 4 * n + i)] = 1.0;
            b[i] = -c[i];
            a[(n, i)] = 1.0;
            a[(n + 1 + i, i)] = 1.0;
            a[(n + 1 + i, n + i)] = -1.0;
            a[(2 * n + 1 + i, i)] = 1.0;
            a[(2 * n + 1 + i, 2 * n + i)] = 1.0;
            b[2 * n + 1 + i] = 1.0;
        }
        a[(n, 5 * n + 1)] = -1.0;
        b[n] = 1.0;
        a[(3 * n + 1, 5 * n + 1)] = 1.0;
        a[(3 * n + 1, 5 * n + 2)] = 1.0;
        a[(3 * n + 2, 5 * n)] = 1.0;
        a[(3 * n + 2, 5 * n + 4)] = 1.0;
        a[(3 * n + 2, 5 * n + 3)] = -1.0;
        (a, b)
    }
}

fn step(
    point: &DVector<f64>,
    direction: &DVector<f64>,
    pairs: &[(usize, usize)],
    fraction: f64,
) -> f64 {
    let relative_direction = pairs
        .iter()
        .flat_map(|&(a, b)| [a, b])
        .map(|i| direction[i] / point[i])
        .fold(-fraction, f64::min);
    (-fraction / relative_direction).min(1.0)
}

pub fn solve(h: &DMatrix<f64>, c: &DVector<f64>, settings: Settings) -> Result<Outcome, Error> {
    let n = c.len();
    if n == 0
        || h.nrows() != n
        || h.ncols() != n
        || h.iter().chain(c.iter()).any(|v| !v.is_finite())
        || !settings.initial_floor.is_finite()
        || settings.initial_floor <= 0.0
        || !(0.0 < settings.step_fraction && settings.step_fraction < 1.0)
        || !settings.gap_tolerance.is_finite()
        || settings.gap_tolerance <= 0.0
        || !settings.feasibility_tolerance.is_finite()
        || settings.feasibility_tolerance <= 0.0
    {
        return Err(Error::InvalidInput);
    }
    let scale = h.amax().max(1.0);
    if (h - h.transpose()).amax() > 1e-12 * scale {
        return Err(Error::InvalidInput);
    }
    if h.clone().symmetric_eigen().eigenvalues.min() < -1e-12 * scale {
        return Err(Error::InvalidInput);
    }
    let layout = Layout { n };
    let pairs = layout.pairs();
    let (linear, _) = layout.linear_equations(h, c);
    let mut opening = DMatrix::zeros(n + 1, n + 1);
    for i in 0..n {
        for j in 0..n {
            opening[(i, j)] = -h[(i, j)];
        }
        opening[(i, i)] -= 1.0;
        opening[(i, n)] = 1.0;
        opening[(n, i)] = 1.0;
    }
    opening[(n, n)] = 1.0;
    let mut rhs = DVector::from_element(n + 1, 1.0);
    rhs.rows_mut(0, n).copy_from(c);
    let initial = Factor::with_policy(opening, settings.linear_solve)?.solve(rhs)?;
    let mut point = DVector::zeros(layout.size());
    for i in 0..n {
        point[i] = initial[i];
        point[n + i] = initial[i].abs().max(settings.initial_floor);
        point[2 * n + i] = (1.0 - initial[i]).abs().max(settings.initial_floor);
        point[3 * n + i] = initial[i].abs().max(settings.initial_floor);
        point[4 * n + i] = initial[i].abs().max(settings.initial_floor);
    }
    point[5 * n] = initial[n];
    for i in 5 * n + 1..layout.size() {
        point[i] = initial[n].abs().max(settings.initial_floor);
    }
    let mean_product = |point: &DVector<f64>| {
        pairs.iter().map(|&(a, b)| point[a] * point[b]).sum::<f64>() / pairs.len() as f64
    };
    let mut barrier = match settings.barrier {
        BarrierRule::PredictorCentered => BarrierState::PredictorCentered,
        BarrierRule::Carried => BarrierState::Carried(mean_product(&point)),
    };
    let mut iterate = Iterate::new(h, point)?;
    for iteration in 0..=settings.max_iterations {
        let point = &iterate.values;
        let residual = layout.residual(&iterate.hx, c, point)?;
        let primal_residual = residual.rows(n, 2 * n + 2).norm() / 2.0;
        let dual_residual = (residual.rows(0, n).norm_squared() + residual[3 * n + 2].powi(2))
            .sqrt()
            / (1.0 + c.norm());
        let x = point.rows(0, n).into_owned();
        let quadratic = 0.5 * x.dot(&iterate.hx);
        let primal = quadratic + c.dot(&x);
        let dual = point[5 * n] - quadratic - point.rows(4 * n, n).sum();
        let gap = (primal - dual).abs() / (1.0 + primal.abs());
        if !gap.is_finite() || !primal_residual.is_finite() || !dual_residual.is_finite() {
            return Err(Error::NonFinite);
        }
        let converged = gap <= settings.gap_tolerance
            && primal_residual <= settings.feasibility_tolerance
            && dual_residual <= settings.feasibility_tolerance;
        if converged || iteration == settings.max_iterations {
            let fit = Fit {
                weights: x.as_slice().to_vec(),
                iterations: iteration,
                primal_residual,
                dual_residual,
                relative_gap: gap,
            };
            return Ok(if converged {
                Outcome::Converged(fit)
            } else {
                Outcome::IterationLimit(fit)
            });
        }
        let rhs = -&residual;
        let mut gamma = pairs.iter().map(|&(_, b)| -point[b]).collect::<Vec<_>>();
        let factor = NewtonFactor::new(
            settings.system,
            h,
            &linear,
            &point,
            &pairs,
            settings.linear_solve,
        )?;
        let predictor = factor.solve_normalized(&point, rhs.clone(), &gamma)?;
        let predicted_step = step(&point, &predictor, &pairs, settings.step_fraction);
        let mu = barrier.target(mean_product(&point), predicted_step);
        for (k, &(a, b)) in pairs.iter().enumerate() {
            gamma[k] = mu / point[a] - point[b] - predictor[a] * predictor[b] / point[a];
        }
        let corrector = factor.solve_normalized(&point, rhs, &gamma)?;
        let fraction = step(&point, &corrector, &pairs, settings.step_fraction);
        iterate.advance(h, &corrector, fraction)?;
        if iterate
            .values
            .iter()
            .chain(iterate.hx.iter())
            .any(|v| !v.is_finite())
            || pairs
                .iter()
                .any(|&(a, b)| iterate.values[a] <= 0.0 || iterate.values[b] <= 0.0)
        {
            return Err(Error::NonFinite);
        }
    }
    unreachable!()
}
