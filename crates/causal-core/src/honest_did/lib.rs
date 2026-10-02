//! Numerical port of HonestDiD 0.2.8. This crate is independent of UI concerns.
//! Constraint primitives alone do not constitute robust confidence intervals.

#[path = "../../../../src/lapack_dgelsd/mod.rs"]
mod lapack_dgelsd;
#[path = "../../../../src/lapack_dsyevd/mod.rs"]
#[allow(dead_code, unused_parens)]
mod lapack_dsyevd;
#[path = "../../../../src/survival/r_rng.rs"]
#[allow(dead_code)]
mod r_rng;

mod constraints;
mod lpsolve;
mod conditional;
mod specification;
pub use specification::{EventStudy,RelativeMagnitudeSpecification,MomentSelection,GridSpecification};
pub use conditional::{relative_magnitude_confidence_set,ConditionalMethod,ConfidenceGrid,minimax,Minimax,least_favorable_critical_value,truncated_normal_quantile,conditional_moment_test};
mod flci;
mod interval;
pub mod optimization;

pub use constraints::{relative_magnitude_constraints, smoothness_constraints, Periods};
pub use flci::{
    affine_coefficients, bias_constant, folded_normal_quantile, weight_coordinates, AffineProblem,
    BiasOptimum,
};
pub use interval::{fixed_length_interval, FixedLengthInterval, SearchMethod};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InvalidInput(&'static str),
    Optimization(optimization::Failure),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(message) => f.write_str(message),
            Self::Optimization(failure) => {
                write!(f, "Optimization did not yield an optimum: {failure:?}")
            }
        }
    }
}
impl std::error::Error for Error {}
