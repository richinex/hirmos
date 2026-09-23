//! Preserved structural time-series kernels. Product integration uses impact::Plan.
//! Other numerical modules do not imply supported application configurations.
pub mod adapters;
pub mod ar;
pub mod brent;
pub mod defaults;
pub mod diagnostics;
mod eigen;
pub mod fit;
pub mod forecast;
pub mod gaussian;
pub mod initial;
pub mod impact;
pub mod numerics;
pub mod poisson;
pub mod poisson_level;
pub mod poisson_mixture;
pub mod poisson_regression;
mod poisson_table;
pub mod prior;
pub mod random;
pub mod regression;
pub mod run;
pub mod semilocal;
pub mod slice;
pub mod sparse_ar;
pub mod state;
pub mod student;
pub mod student_level;
pub mod student_regression;
pub mod weighted;
// Compile the existing source directly while the port remains isolated here.
#[path = "../../causal-core/src/lapack_cholesky.rs"]
mod lapack_cholesky;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NonFinite,
    InvalidScale,
    InvalidSeason,
    NonStationary,
    InvalidTruncation,
    Empty,
    Shape,
    Singular,
    MissingInitial,
    MissingPredictors,
    TailUnderflow,
    SamplingLimit,
    InvalidProbability,
    ImproperConditional,
    Cancelled,
    MissingMixture(u32),
}
