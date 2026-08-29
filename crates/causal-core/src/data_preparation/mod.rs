//! Method-neutral, nullable time-series preparation.
//!
//! This module is independently structured around Hirmos's data contract. It keeps observed
//! validity, analysis exclusions, and imputation provenance distinct; it never compresses the
//! time axis; and it represents scientific insufficiency as a typed refusal. Frozen behavioral
//! fixtures verify compatibility without importing or calling the legacy compatibility layer.
//!
//! This is not represented as a strict two-person clean-room implementation: its author had
//! previously inspected the retired compatibility code. See `PREPROCESSING_REWRITE_DOP.md` for
//! the implementation boundary and provenance plan.

mod missingness;
mod model;
mod projection;

pub use missingness::{
    carry_forward_bounded, fill_confirmed_structural_zero, interpolate_bounded_linear,
    longest_complete_interval,
};
pub use model::{
    AnalysisExclusionPolicy, CellRef, ColumnId, CompleteIntervalOutcome, CompleteIntervalRefusal,
    ConfirmationId, DataPreparationError, ExclusionReason, FeatureRole, GapInfluence,
    HistoryRequirement, ImputationOutcome, LagProjectionRequest, LaggedAnalysisMatrix,
    LaggedFeature, Lookback, MatrixLayout, MissingRun, NonEmptyVec, NullableDataset,
    NumericNullPolicy, PositiveUsize, PreparationRefusal, ProjectionOutcome, ReferenceDomain,
    SampleDecision,
};
pub use projection::project_lagged;
