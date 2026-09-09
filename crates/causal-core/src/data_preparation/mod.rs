//! Method-neutral, nullable time-series preparation.
//!
//! This module is independently structured around Hirmos's data contract. It keeps observed
//! validity, analysis exclusions, and imputation records distinct; it never compresses the
//! time axis; and it represents scientific insufficiency as a typed refusal. Frozen behavioral
//! fixtures verify compatibility without importing or calling the legacy compatibility layer.
//!
//! This is not represented as a strict two-person clean-room implementation: its author had
//! previously inspected the retired compatibility code. See `PREPROCESSING_REWRITE_DOP.md` for
//! the implementation boundary and preparation record.

mod cohort_event;
mod missingness;
mod model;
mod projection;
mod survival;

pub use cohort_event::{
    prepare_cohort_events, CalendarFlowRow, CohortEntry, CohortEvent, CohortEventHistory,
    CohortEventPreparationError, CohortId, GroupedSurvivalRow, PositiveCount,
    PreparedCohortEventHistory, StratumId,
};
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
pub use survival::{
    prepare_longitudinal_states, prepare_wide_events, EventStatus, LongitudinalObservation,
    LongitudinalStateHistory, PreparedMultiStateData, PreparedTransitionRow, StateId,
    StateObservation, SubjectId, SurvivalPreparationError, SurvivalPreparationNotice, TransitionId,
    TransitionMatrix, WideEventHistory, WideSubject,
};
