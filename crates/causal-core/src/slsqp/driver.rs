//! Checked boundary around the source reverse-communication driver.

use super::generated::__slsqp::{slsqp_body, SLSQP_vars};
use core::ffi::c_long;

#[derive(Clone, Copy)]
pub(crate) enum Bound {
    Unbounded,
    Lower(f64),
    Upper(f64),
    Interval { lower: f64, upper: f64 },
}

#[derive(Debug, PartialEq)]
pub(crate) enum InputError {
    Dimensions,
    Settings,
    Bounds,
    Nonfinite,
    Allocation,
    EvaluationShape,
}

#[derive(Debug)]
pub(crate) enum Error<E> {
    Invalid(InputError),
    Evaluation(E),
    UnexpectedMode(i64),
}

impl<E> From<InputError> for Error<E> {
    fn from(value: InputError) -> Self {
        Self::Invalid(value)
    }
}

pub(crate) struct Evaluation {
    pub value: f64,
    pub gradient: Vec<f64>,
    /// Equalities first (value = 0), followed by inequalities (value >= 0).
    pub constraints: Vec<f64>,
    /// Column-major, one row per constraint and one column per parameter.
    pub normals: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Termination {
    Converged,
    TooManyEqualities,
    LeastSquaresIterationLimit,
    IncompatibleConstraints,
    SingularObjectiveMatrix,
    SingularConstraintMatrix,
    RankDeficientEqualities,
    PositiveDirectionalDerivative,
    IterationLimit,
}

impl Termination {
    pub fn source_code(&self) -> i64 {
        match self {
            Self::Converged => 0,
            Self::TooManyEqualities => 2,
            Self::LeastSquaresIterationLimit => 3,
            Self::IncompatibleConstraints => 4,
            Self::SingularObjectiveMatrix => 5,
            Self::SingularConstraintMatrix => 6,
            Self::RankDeficientEqualities => 7,
            Self::PositiveDirectionalDerivative => 8,
            Self::IterationLimit => 9,
        }
    }
}

pub(crate) struct Fit {
    pub parameters: Vec<f64>,
    pub value: f64,
    pub gradient: Vec<f64>,
    pub multipliers: Vec<f64>,
    pub iterations: usize,
    pub termination: Termination,
}

pub(crate) struct Problem {
    initial: Vec<f64>,
    lower: Vec<f64>,
    upper: Vec<f64>,
    equalities: usize,
    constraints: usize,
    tolerance: f64,
    max_iterations: usize,
    workspace: usize,
    prefix: usize,
}

fn allocate<T: Clone>(length: usize, value: T) -> Result<Vec<T>, InputError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| InputError::Allocation)?;
    values.resize(length, value);
    Ok(values)
}

impl Problem {
    pub fn new(
        mut initial: Vec<f64>,
        bounds: &[Bound],
        equalities: usize,
        inequalities: usize,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<Self, InputError> {
        if !tolerance.is_finite() || max_iterations > i32::MAX as usize {
            return Err(InputError::Settings);
        }
        let n = initial.len();
        let m = equalities
            .checked_add(inequalities)
            .ok_or(InputError::Dimensions)?;
        if n == 0 || n > i32::MAX as usize || m > i32::MAX as usize || bounds.len() != n {
            return Err(InputError::Dimensions);
        }
        if initial.iter().any(|v| !v.is_finite()) {
            return Err(InputError::Nonfinite);
        }
        // Source formula, evaluated in a wider integer type before allocation.
        // Add explicit leading storage for f2c's one-based pointer adjustments.
        let (ni, mi, ei) = (n as i128, m as i128, equalities as i128);
        let workspace = ni * (ni + 1) / 2 + 3 * mi * ni - (mi + 5 * ni + 7) * ei
            + 9 * mi
            + 8 * ni * ni
            + 35 * ni
            + ei * ei
            + 28
            + if inequalities == 0 {
                2 * ni * (ni + 1)
            } else {
                0
            };
        let prefix = m
            .checked_add(n.checked_mul(3).ok_or(InputError::Dimensions)?)
            .and_then(|v| v.checked_add(16))
            .ok_or(InputError::Dimensions)?;
        if workspace < 1
            || workspace + prefix as i128 > i32::MAX as i128
            || mi * ni + prefix as i128 > i32::MAX as i128
        {
            return Err(InputError::Dimensions);
        }
        let mut lower = allocate(n, f64::NAN)?;
        let mut upper = allocate(n, f64::NAN)?;
        for (i, bound) in bounds.iter().enumerate() {
            let (lo, hi) = match *bound {
                Bound::Unbounded => (None, None),
                Bound::Lower(lo) => (Some(lo), None),
                Bound::Upper(hi) => (None, Some(hi)),
                Bound::Interval { lower, upper } => (Some(lower), Some(upper)),
            };
            if lo.iter().chain(hi.iter()).any(|v| !v.is_finite())
                || matches!((lo, hi), (Some(a), Some(b)) if a > b)
            {
                return Err(InputError::Bounds);
            }
            if let Some(lo) = lo {
                lower[i] = lo;
                initial[i] = initial[i].max(lo);
            }
            if let Some(hi) = hi {
                upper[i] = hi;
                initial[i] = initial[i].min(hi);
            }
        }
        Ok(Self {
            initial,
            lower,
            upper,
            equalities,
            constraints: m,
            tolerance,
            max_iterations,
            workspace: workspace as usize,
            prefix,
        })
    }

    fn validate(&self, evaluation: &Evaluation) -> Result<(), InputError> {
        if evaluation.gradient.len() != self.initial.len()
            || evaluation.constraints.len() != self.constraints
            || evaluation.normals.len() != self.constraints * self.initial.len()
        {
            return Err(InputError::EvaluationShape);
        }
        if !evaluation.value.is_finite()
            || evaluation
                .gradient
                .iter()
                .chain(&evaluation.constraints)
                .chain(&evaluation.normals)
                .any(|v| !v.is_finite())
        {
            return Err(InputError::Nonfinite);
        }
        Ok(())
    }

    pub fn minimize<E>(
        &self,
        mut evaluate: impl FnMut(&[f64]) -> Result<Evaluation, E>,
        mut on_iteration: impl FnMut(&[f64], f64),
    ) -> Result<Fit, Error<E>> {
        let n = self.initial.len();
        let m = self.constraints;
        let p = self.prefix;
        let mut x = allocate(p + n + 1, 0.0)?;
        x[p..p + n].copy_from_slice(&self.initial);
        let mut gradient = allocate(p + n + 1, 0.0)?;
        let mut normals = allocate(p + m.max(1) * n, 0.0)?;
        let mut values = allocate(p + m.max(1), 0.0)?;
        let mut multipliers = allocate(p + m + 2 * n + 2, 0.0)?;
        let mut lower = allocate(p + n + 1, f64::NAN)?;
        let mut upper = allocate(p + n + 1, f64::NAN)?;
        lower[p..p + n].copy_from_slice(&self.lower);
        upper[p..p + n].copy_from_slice(&self.upper);
        let mut buffer = allocate(p + self.workspace, 0.0)?;
        let mut indices = allocate(p + (m + 2 * n + 2).max(1), 0 as c_long)?;
        let mut state: SLSQP_vars = unsafe { core::mem::zeroed() };
        state.n = n as c_long;
        state.m = m as c_long;
        state.meq = self.equalities as c_long;
        state.acc = self.tolerance;
        state.tol = 10.0 * self.tolerance;
        state.itermax = self.max_iterations as c_long;
        let mut cached_point = self.initial.clone();
        let mut cached = evaluate(&cached_point).map_err(Error::Evaluation)?;
        self.validate(&cached)?;
        let mut objective = cached.value;
        gradient[p..p + n].copy_from_slice(&cached.gradient);
        values[p..p + m].copy_from_slice(&cached.constraints);
        normals[p..p + m * n].copy_from_slice(&cached.normals);
        let mut previous_iteration = 0;
        loop {
            // Dimensions/workspace were checked before reaching the raw source.
            // Prefix storage also covers nested BLAS/LAPACK pointer adjustments.
            unsafe {
                slsqp_body(
                    &mut state,
                    &mut objective,
                    gradient.as_mut_ptr().add(p),
                    normals.as_mut_ptr().add(p),
                    values.as_mut_ptr().add(p),
                    x.as_mut_ptr().add(p),
                    multipliers.as_mut_ptr().add(p),
                    lower.as_mut_ptr().add(p),
                    upper.as_mut_ptr().add(p),
                    buffer.as_mut_ptr().add(p),
                    indices.as_mut_ptr().add(p),
                );
            }
            if matches!(state.mode, -1 | 1) {
                if cached_point != x[p..p + n] {
                    cached_point.copy_from_slice(&x[p..p + n]);
                    cached = evaluate(&cached_point).map_err(Error::Evaluation)?;
                    self.validate(&cached)?;
                }
                match state.mode {
                    1 => {
                        objective = cached.value;
                        values[p..p + m].copy_from_slice(&cached.constraints);
                    }
                    -1 => {
                        gradient[p..p + n].copy_from_slice(&cached.gradient);
                        normals[p..p + m * n].copy_from_slice(&cached.normals);
                    }
                    _ => unreachable!(),
                }
            }
            if state.iter > previous_iteration {
                on_iteration(&x[p..p + n], objective);
            }
            previous_iteration = state.iter;
            let termination = match state.mode {
                -1 | 1 => continue,
                0 => Termination::Converged,
                2 => Termination::TooManyEqualities,
                3 => Termination::LeastSquaresIterationLimit,
                4 => Termination::IncompatibleConstraints,
                5 => Termination::SingularObjectiveMatrix,
                6 => Termination::SingularConstraintMatrix,
                7 => Termination::RankDeficientEqualities,
                8 => Termination::PositiveDirectionalDerivative,
                9 => Termination::IterationLimit,
                other => return Err(Error::UnexpectedMode(other as i64)),
            };
            return Ok(Fit {
                parameters: x[p..p + n].to_vec(),
                value: objective,
                gradient: gradient[p..p + n].to_vec(),
                multipliers: multipliers[p..p + m].to_vec(),
                iterations: state.iter as usize,
                termination,
            });
        }
    }
}
