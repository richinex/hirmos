use crate::honest_did::{optimization, r_rng::RRng, Error, Periods};
use nalgebra::{DMatrix, DVector, SymmetricEigen};

/// HonestDiD::.wToLFn.
pub fn affine_coefficients(weights: &[f64]) -> Vec<f64> {
    weights
        .iter()
        .enumerate()
        .map(|(i, w)| w - if i == 0 { 0.0 } else { weights[i - 1] })
        .collect()
}

/// HonestDiD::.lToWFn.
pub fn weight_coordinates(coefficients: &[f64]) -> Vec<f64> {
    let mut sum = 0.0;
    coefficients
        .iter()
        .map(|c| {
            sum += c;
            sum
        })
        .collect()
}

pub fn bias_constant(contrast: &[f64]) -> f64 {
    let post = contrast.len();
    let constant: f64 = (1..=post)
        .map(|s| {
            contrast[post - s..]
                .iter()
                .enumerate()
                .map(|(i, l)| (i + 1) as f64 * l)
                .sum::<f64>()
                .abs()
        })
        .sum();
    constant
        - contrast
            .iter()
            .enumerate()
            .map(|(i, l)| (i + 1) as f64 * l)
            .sum::<f64>()
}

/// R's simulated folded-normal quantile, including its default inversion draws
/// and type-7 sample quantile. `draws` is explicit to preserve oracle semantics.
pub fn folded_normal_quantile(
    probability: f64,
    mean: f64,
    sd: f64,
    draws: usize,
    seed: u32,
) -> Result<f64, Error> {
    if !(0.0..=1.0).contains(&probability)
        || !mean.is_finite()
        || !sd.is_finite()
        || sd < 0.0
        || draws == 0
    {
        return Err(Error::InvalidInput("The folded-normal quantile requires a valid probability, finite parameters and at least one draw."));
    }
    let mut rng = RRng::new(seed);
    let mut sample: Vec<f64> = (0..draws)
        .map(|_| (sd * rng.normal() + mean).abs())
        .collect();
    Ok(type_seven_quantile(&mut sample, probability))
}

pub(crate) fn type_seven_quantile(sample: &mut [f64], probability: f64) -> f64 {
    let position = (sample.len() - 1) as f64 * probability;
    let lower = position.floor() as usize;
    let fraction = position - lower as f64;
    let (_, selected, above) = sample.select_nth_unstable_by(lower, f64::total_cmp);
    let low = *selected;
    if fraction == 0.0 {
        return low;
    }
    let high = above.iter().copied().min_by(f64::total_cmp).unwrap();
    (1.0 - fraction) * low + fraction * high
}

#[derive(Clone, Debug)]
pub struct BiasOptimum {
    pub bias: f64,
    pub weights: Vec<f64>,
    pub coefficients: Vec<f64>,
    pub variance: f64,
    pub accuracy: optimization::Accuracy,
}

#[derive(Clone, Debug)]
pub struct AffineProblem {
    periods: Periods,
    contrast: DVector<f64>,
    sigma: DMatrix<f64>,
    transform: DMatrix<f64>,
    quadratic_scale: f64,
    quadratic_basis: DMatrix<f64>,
}

impl AffineProblem {
    pub(crate) fn periods(&self) -> Periods { self.periods }
    pub(crate) fn covariance(&self) -> &DMatrix<f64> { &self.sigma }
    pub(crate) fn period_count(&self) -> usize {
        self.periods.total()
    }
    pub(crate) fn contrast(&self) -> &[f64] {
        self.contrast.as_slice()
    }
    pub fn new(periods: Periods, sigma: DMatrix<f64>, contrast: Vec<f64>) -> Result<Self, Error> {
        let n = periods.total();
        if sigma.shape() != (n, n) || contrast.len() != periods.post() {
            return Err(Error::InvalidInput("The covariance and contrast dimensions must match the pre-treatment and post-treatment coefficients."));
        }
        if sigma.iter().chain(contrast.iter()).any(|v| !v.is_finite()) {
            return Err(Error::InvalidInput(
                "The covariance and contrast must contain finite values.",
            ));
        }
        let scale = sigma.iter().fold(0.0_f64, |s, v| s.max(v.abs()));
        if scale == 0.0 || (&sigma - sigma.transpose()).amax() > scale * 1e-12 {
            return Err(Error::InvalidInput(
                "The covariance must be nonzero and symmetric.",
            ));
        }
        let symmetric = (&sigma + sigma.transpose()) * 0.5;
        let eigen = SymmetricEigen::new(symmetric.clone());
        if eigen.eigenvalues.iter().any(|v| *v < -scale * 1e-12) {
            return Err(Error::InvalidInput(
                "The covariance must be positive semidefinite.",
            ));
        }
        let p = periods.pre();
        let mut transform = DMatrix::identity(p, p);
        for i in 1..p {
            transform[(i, i - 1)] = -1.0;
        }
        let q = transform.transpose() * symmetric.view((0, 0), (p, p)) * &transform;
        let mut full = DMatrix::zeros(2 * p, 2 * p);
        full.view_mut((p, p), (p, p)).copy_from(&q);
        let mut values = vec![0.0; 2 * p];
        let info = crate::honest_did::lapack_dsyevd::dsyevd(
            crate::honest_did::lapack_dsyevd::EigenJob::Vectors,
            crate::honest_did::lapack_dsyevd::Triangle::Lower,
            2 * p,
            full.as_mut_slice(),
            2 * p,
            &mut values,
        )
        .map_err(|_| Error::InvalidInput("The quadratic constraint could not be decomposed."))?;
        if info != 0 {
            return Err(Error::InvalidInput(
                "The quadratic eigensolver did not converge.",
            ));
        }
        let quadratic_scale = values.iter().fold(0.0_f64, |scale, v| scale.max(v.abs()));
        let retained: Vec<usize> = (0..2 * p)
            .rev()
            .filter(|i| values[*i] / quadratic_scale > 1e6 * f64::EPSILON)
            .collect();
        let mut quadratic_basis = DMatrix::zeros(retained.len(), 2 * p);
        for (row, index) in retained.into_iter().enumerate() {
            for column in 0..2 * p {
                quadratic_basis[(row, column)] =
                    full[(column, index)] * (values[index] / quadratic_scale).sqrt();
            }
        }
        Ok(Self {
            periods,
            contrast: DVector::from_vec(contrast),
            sigma: symmetric,
            transform,
            quadratic_scale,
            quadratic_basis,
        })
    }

    pub fn variance(&self, weights: &[f64]) -> Result<f64, Error> {
        if weights.len() != self.periods.pre() || weights.iter().any(|v| !v.is_finite()) {
            return Err(Error::InvalidInput(
                "The weights must contain one finite value per pre-treatment coefficient.",
            ));
        }
        let mut l = affine_coefficients(weights);
        l.extend_from_slice(self.contrast.as_slice());
        let l = DVector::from_vec(l);
        Ok(l.dot(&(&self.sigma * &l)))
    }

    fn constraints(&self) -> (DMatrix<f64>, DVector<f64>) {
        let p = self.periods.pre();
        let mut a = DMatrix::zeros(2 * p + 1, 2 * p);
        for i in 0..p {
            a[(i, i)] = -1.0;
            a[(i + p, i)] = -1.0;
            for j in 0..=i {
                a[(i, p + j)] = 1.0;
                a[(i + p, p + j)] = -1.0;
            }
            a[(2 * p, p + i)] = 1.0;
        }
        let mut b = DVector::zeros(2 * p + 1);
        b[2 * p] = self
            .contrast
            .iter()
            .enumerate()
            .map(|(i, l)| (i + 1) as f64 * l)
            .sum();
        (a, b)
    }

    /// HonestDiD::.findLowestH: quadratic minimization with the same linear
    /// constraints, solved without inventing box bounds on the weights.
    pub fn minimum_sd(&self) -> Result<f64, Error> {
        let p = self.periods.pre();
        let pre = self.sigma.view((0, 0), (p, p));
        let cross = self.sigma.view((0, p), (p, self.periods.post()));
        let q = self.transform.transpose() * pre * &self.transform;
        let linear = 2.0 * self.transform.transpose() * cross * &self.contrast;
        let mut quadratic = DMatrix::zeros(2 * p, 2 * p);
        quadratic.view_mut((p, p), (p, p)).copy_from(&(2.0 * q));
        let mut cost = DVector::zeros(2 * p);
        cost.rows_mut(p, p).copy_from(&linear);
        let (a, b) = self.constraints();
        // CVXR's QP reduction places equalities before inequalities. Retain
        // that ordering and its OSQP objective value, not a reevaluated
        // variance: at the feasibility boundary rounding changes ECOS's path.
        let rows = |matrix: &DMatrix<f64>| {
            (0..matrix.nrows())
                .map(|i| {
                    (0..matrix.ncols())
                        .map(|j| matrix[(i, j)])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        };
        let mut constraints = rows(&a);
        constraints.rotate_right(1);
        let mut upper = b.as_slice().to_vec();
        upper.rotate_right(1);
        let mut lower = vec![f64::NEG_INFINITY; 2 * p + 1];
        lower[0] = upper[0];
        let optimum = osqp_reference_port::solve(
            &rows(&quadratic),
            cost.as_slice(),
            &constraints,
            &lower,
            &upper,
        )
        .map_err(|message| Error::Optimization(optimization::Failure::Setup(message.into())))?;
        match optimum.status {
            1 | 2 => {}
            3 | 4 => return Err(Error::Optimization(optimization::Failure::Infeasible)),
            5 | 6 => return Err(Error::Optimization(optimization::Failure::Unbounded)),
            status => {
                return Err(Error::Optimization(optimization::Failure::DidNotConverge(
                    format!("OSQP status {status}"),
                )))
            }
        }
        let post = self
            .sigma
            .view((p, p), (self.periods.post(), self.periods.post()));
        let constant = (self.contrast.transpose() * post * &self.contrast)[(0, 0)];
        let variance = optimum.objective + constant;
        if !variance.is_finite() || variance < 0.0 {
            return Err(Error::InvalidInput(
                "The minimum variance is not nonnegative and finite.",
            ));
        }
        Ok(variance.sqrt())
    }

    pub fn minimum_bias_sd(&self) -> Result<f64, Error> {
        let mut weights = vec![0.0; self.periods.pre()];
        *weights.last_mut().unwrap() = self
            .contrast
            .iter()
            .enumerate()
            .map(|(i, l)| (i + 1) as f64 * l)
            .sum();
        Ok(self.variance(&weights)?.max(0.0).sqrt())
    }

    /// HonestDiD::.findWorstCaseBiasGivenH, with CVXR's quadratic-over-linear
    /// canonicalization and the translated reference ECOS solver.
    pub fn minimum_bias_at_sd(&self, h: f64, magnitude: f64) -> Result<BiasOptimum, Error> {
        if !h.is_finite() || h <= 0.0 || !magnitude.is_finite() || magnitude < 0.0 {
            return Err(Error::InvalidInput("The standard-deviation bound must be positive and the smoothness bound must be nonnegative."));
        }
        let p = self.periods.pre();
        let (linear_a, linear_b) = self.constraints();
        let variables = 2 * p + 1;
        let cone_size = self.quadratic_basis.nrows() + 2;
        let mut g = vec![vec![0.0; variables]; 2 * p + 1 + cone_size];
        for row in 0..2 * p {
            for column in 0..2 * p {
                g[row][column] = linear_a[(row, column)];
            }
        }
        let cross = self.sigma.view((0, p), (p, self.periods.post()));
        let q_linear = 2.0 * self.transform.transpose() * cross * &self.contrast;
        for i in 0..p {
            g[2 * p][p + i] = q_linear[i];
        }
        g[2 * p][2 * p] = self.quadratic_scale;
        g[2 * p + 1][2 * p] = -1.0;
        g[2 * p + 2][2 * p] = 1.0;
        for row in 0..self.quadratic_basis.nrows() {
            for column in 0..2 * p {
                g[2 * p + 3 + row][column] = -2.0 * self.quadratic_basis[(row, column)];
            }
        }
        let mut rhs = vec![0.0; g.len()];
        let post_sigma = self
            .sigma
            .view((p, p), (self.periods.post(), self.periods.post()));
        let constant = self.contrast.dot(&(post_sigma * &self.contrast));
        rhs[2 * p] = h * h - constant;
        rhs[2 * p + 1] = 1.0;
        rhs[2 * p + 2] = 1.0;
        let mut equality = vec![vec![0.0; variables]];
        for i in 0..2 * p {
            equality[0][i] = linear_a[(2 * p, i)];
        }
        let g = ecos_reference_port::Matrix::from_rows(&g, variables)
            .map_err(|_| Error::InvalidInput("Invalid conic constraint."))?;
        let a = ecos_reference_port::Matrix::from_rows(&equality, variables)
            .map_err(|_| Error::InvalidInput("Invalid conic equality."))?;
        let mut cost = vec![0.0; variables];
        cost[..p].fill(1.0);
        let optimum = ecos_reference_port::solve(
            &cost,
            &g,
            &rhs,
            2 * p + 1,
            &[cone_size],
            &a,
            &[linear_b[2 * p]],
        )
        .map_err(|e| Error::Optimization(optimization::Failure::Setup(e.to_owned())))?;
        let accuracy = match optimum.status {
            0 => optimization::Accuracy::Solved,
            10 => optimization::Accuracy::ReferenceInaccurate,
            1 | 11 => return Err(Error::Optimization(optimization::Failure::Infeasible)),
            2 | 12 => return Err(Error::Optimization(optimization::Failure::Unbounded)),
            other => {
                return Err(Error::Optimization(optimization::Failure::DidNotConverge(
                    format!("ECOS status {other}"),
                )))
            }
        };
        let weights = optimum.x[p..2 * p].to_vec();
        Ok(BiasOptimum {
            bias: magnitude * (optimum.objective + bias_constant(self.contrast.as_slice())),
            coefficients: affine_coefficients(&weights),
            variance: self.variance(&weights)?,
            weights,
            accuracy,
        })
    }
}
