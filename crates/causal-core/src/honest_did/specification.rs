//! Validated numerical inputs, independent of worker and presentation state.
use crate::honest_did::{AffineProblem, ConditionalMethod, Error, Periods};
use nalgebra::DMatrix;

#[derive(Clone, Debug)]
pub struct EventStudy {
    coefficients: Vec<f64>,
    problem: AffineProblem,
}
impl EventStudy {
    pub fn new(
        periods: Periods,
        coefficients: Vec<f64>,
        covariance: DMatrix<f64>,
        contrast: Vec<f64>,
    ) -> Result<Self, Error> {
        if coefficients.len() != periods.total() || coefficients.iter().any(|v| !v.is_finite()) {
            return Err(Error::InvalidInput(
                "Each retained pre/post coefficient must have a finite estimate.",
            ));
        }
        if contrast.iter().all(|v| *v == 0.0) {
            return Err(Error::InvalidInput("The target contrast must not be zero."));
        }
        let problem = AffineProblem::new(periods, covariance, contrast)?;
        Ok(Self {
            coefficients,
            problem,
        })
    }
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }
    pub fn affine_problem(&self) -> &AffineProblem {
        &self.problem
    }
}
#[derive(Clone, Copy, Debug)]
pub enum MomentSelection {
    PostTreatment,
    AllPeriods,
}
#[derive(Clone, Copy, Debug)]
pub enum GridSpecification {
    ReferenceDefault {
        points: usize,
    },
    Explicit {
        lower: f64,
        upper: f64,
        points: usize,
    },
}
#[derive(Clone, Copy, Debug)]
pub struct RelativeMagnitudeSpecification {
    pub(crate) bound: f64,
    pub(crate) method: ConditionalMethod,
    pub(crate) moments: MomentSelection,
    pub(crate) grid: GridSpecification,
    pub(crate) seed: u32,
    pub(crate) alpha: f64,
}
impl RelativeMagnitudeSpecification {
    pub fn new(
        bound: f64,
        method: ConditionalMethod,
        moments: MomentSelection,
        grid: GridSpecification,
        seed: u32,
        alpha: f64,
    ) -> Result<Self, Error> {
        if !bound.is_finite() || bound < 0.0 || !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
            return Err(Error::InvalidInput(
                "The relative bound must be nonnegative and alpha must lie between zero and one.",
            ));
        }
        if let ConditionalMethod::LeastFavorableHybrid { kappa } = method {
            if !kappa.is_finite() || kappa <= 0.0 || kappa >= alpha {
                return Err(Error::InvalidInput(
                    "The hybrid size must be positive and smaller than alpha.",
                ));
            }
        }
        let points = match grid {
            GridSpecification::ReferenceDefault { points } => points,
            GridSpecification::Explicit {
                lower,
                upper,
                points,
            } => {
                if !lower.is_finite() || !upper.is_finite() || lower >= upper {
                    return Err(Error::InvalidInput(
                        "The confidence-set grid needs increasing finite endpoints.",
                    ));
                }
                points
            }
        };
        if points < 2 {
            return Err(Error::InvalidInput(
                "The confidence-set grid needs at least two points.",
            ));
        }
        Ok(Self {
            bound,
            method,
            moments,
            grid,
            seed,
            alpha,
        })
    }
}
