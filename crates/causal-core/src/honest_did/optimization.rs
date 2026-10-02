//! Shared conic solver boundary. Infeasible, unbounded and numerical outcomes
//! remain failures, never ordinary confidence intervals.
use clarabel::{algebra::CscMatrix, solver::*};
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Debug, PartialEq)]
pub enum Failure {
    InvalidDimensions,
    InvalidNumbers,
    Infeasible,
    Unbounded,
    DidNotConverge(String),
    Setup(String),
}

#[derive(Clone, Debug)]
pub struct Optimum {
    pub x: Vec<f64>,
    pub objective: f64,
    pub primal_residual: f64,
    pub dual_residual: f64,
    pub accuracy: Accuracy,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Accuracy {
    Solved,
    ResidualQualified,
    ReferenceInaccurate,
}

fn sparse(matrix: &DMatrix<f64>) -> CscMatrix<f64> {
    let mut offsets = vec![0];
    let mut indices = Vec::new();
    let mut values = Vec::new();
    for j in 0..matrix.ncols() {
        for i in 0..matrix.nrows() {
            if matrix[(i, j)] != 0.0 {
                indices.push(i);
                values.push(matrix[(i, j)]);
            }
        }
        offsets.push(values.len());
    }
    CscMatrix::new(matrix.nrows(), matrix.ncols(), offsets, indices, values)
}

pub fn solve(
    quadratic: &DMatrix<f64>,
    linear: &DVector<f64>,
    a: &DMatrix<f64>,
    b: &DVector<f64>,
    cones: &[SupportedConeT<f64>],
) -> Result<Optimum, Failure> {
    let n = linear.len();
    if quadratic.shape() != (n, n) || a.ncols() != n || b.len() != a.nrows() {
        return Err(Failure::InvalidDimensions);
    }
    if quadratic
        .iter()
        .chain(linear.iter())
        .chain(a.iter())
        .chain(b.iter())
        .any(|v| !v.is_finite())
    {
        return Err(Failure::InvalidNumbers);
    }
    let mut settings = DefaultSettings::default();
    settings.verbose = false;
    settings.tol_gap_abs = 1e-10;
    settings.tol_gap_rel = 1e-10;
    settings.tol_feas = 1e-10;
    settings.max_iter = 300;
    let mut solver = DefaultSolver::new(
        &sparse(quadratic),
        linear.as_slice(),
        &sparse(a),
        b.as_slice(),
        cones,
        settings,
    )
    .map_err(|e| Failure::Setup(e.to_string()))?;
    solver.solve();
    let approximate_is_qualified = solver.solution.r_prim <= 1e-7
        && solver.solution.r_dual <= 1e-7
        && (solver.solution.obj_val - solver.solution.obj_val_dual).abs()
            <= 1e-7 * (1.0 + solver.solution.obj_val.abs());
    match solver.solution.status {
        SolverStatus::Solved | SolverStatus::AlmostSolved
            if solver.solution.status == SolverStatus::Solved || approximate_is_qualified =>
        {
            Ok(Optimum {
                x: solver.solution.x,
                objective: solver.solution.obj_val,
                primal_residual: solver.solution.r_prim,
                dual_residual: solver.solution.r_dual,
                accuracy: if solver.solution.status == SolverStatus::Solved {
                    Accuracy::Solved
                } else {
                    Accuracy::ResidualQualified
                },
            })
        }
        SolverStatus::PrimalInfeasible | SolverStatus::AlmostPrimalInfeasible => {
            Err(Failure::Infeasible)
        }
        SolverStatus::DualInfeasible | SolverStatus::AlmostDualInfeasible => {
            Err(Failure::Unbounded)
        }
        status => Err(Failure::DidNotConverge(format!(
            "{status:?}; primal residual {}, dual residual {}, objective gap {}",
            solver.solution.r_prim,
            solver.solution.r_dual,
            (solver.solution.obj_val - solver.solution.obj_val_dual).abs()
        ))),
    }
}

pub use clarabel::solver::{NonnegativeConeT, SecondOrderConeT, ZeroConeT};
