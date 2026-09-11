//! Cox proportional-hazards regression for right-censored and start-stop data.

mod baseline;
mod concordance;
mod data;
mod fit;
mod frailty;
mod math;
mod prediction;
mod residuals;
mod time_varying;

pub use baseline::{BaselineCurves, BaselineEstimate, StratumBaseline};
pub use concordance::ConcordanceIndex;
pub(crate) use concordance::prediction_concordance_index;
pub use data::{CoxCovariates, CoxDataError, Event, RightCensoredData, TimeVaryingData};
pub use frailty::{
    fit_gamma_frailty, FrailtyTermTest, GammaFrailtyError, GammaFrailtyFit, GammaFrailtyOptions,
    PenaltyState, TieMethod,
};
pub use fit::{
    evaluate_right_censored, fit_right_censored, BatchMode, CoefficientEstimate, CoxFit,
    CoxFitError, CoxFitOptions, CoxPenalty, EfronEvaluation, RightCensoredFit, StandardErrorMethod,
};
pub use prediction::{
    predict_expectation, predict_percentile, predict_survival, CoxProfile, PredictionError,
    ProfilePrediction, SurvivalPrediction,
};
pub use residuals::{
    compute_right_censored_residuals, test_proportional_hazards, CoxResiduals,
    ProportionalHazardsDiagnostics, ProportionalHazardsTest, ResidualError, TimeTransform,
};
pub use time_varying::{evaluate_time_varying, fit_time_varying};
