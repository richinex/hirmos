//! flexsurv 2.3.2 multi-state prediction.
//!
//! The source package exposes clock-forward and clock-reset functions with no
//! runtime guard.  The clock marker on [`MultiStateModel`] makes that invalid
//! combination unavailable in this port.

pub mod ajfit;
pub mod graph;
pub mod markov;
pub mod mixture;
pub mod model;
pub mod semi_markov;

pub use ajfit::{
    compare_factor_strata, AalenJohansenData, AalenJohansenStratum, AjFitError, AjFitResult,
    OccupancyEstimate, OccupancyModel, StratifiedAjFitResult, StratumLabel, TransitionHistoryRow,
};
pub use graph::{StateId, Transition, TransitionGraph, TransitionGraphError, TransitionId};
pub use markov::{
    CompetingRisksModel, ConditionalProbabilityResult, CumulativeHazardCovarianceResult,
    FinalStateProbabilityIntervals, FinalStateProbabilityResult, LengthOfStayIntervals,
    LengthOfStayResult, MarkovControl, MarkovError, MarkovIntegrator,
    TransitionProbabilityIntervals, TransitionProbabilityResult,
};
pub use mixture::{
    MixtureFinalInterval, MixtureFinalQuantile, MixtureFinalQuantileInterval, MixtureFinalSummary,
    MixtureMultiStateError, MixtureMultiStateModel, MixtureMultiStateProfile, MixturePath,
    MixturePathInterval, MixturePathQuantile, MixturePathQuantileInterval, MixturePathSummary,
};
pub use model::{ClockForward, ClockReset, MultiStateModel, MultiStateModelError, TransitionModel};
pub use semi_markov::{
    FinalOutcomeIntervals, FinalOutcomeResult, FinalStateIntervals, FinalStateSummary,
    GroupedLengthOfStay, SemiMarkovError, SemiMarkovSemantics, SemiMarkovSimulation,
    SimulatedLengthOfStayIntervals, SimulatedPath, SimulatedProbabilityIntervals,
    SimulatedProbabilityResult, SimulatedTransition,
};
