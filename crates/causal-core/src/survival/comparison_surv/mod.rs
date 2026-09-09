//! Source-compatible port of CRAN ComparisonSurv 1.1.1.
//!
//! ComparisonSurv compares two already-observed survival curves.  Its tests
//! and summaries remain separate from flexsurv's parametric regression model.

mod cox;
mod data;
mod descriptive;
mod km;
mod methods;
mod overall;
mod plot;
mod tshrc;

pub use cox::{proportional_hazards_check, CoxError, ProportionalHazardsCheck};
pub use data::{ComparisonData, ComparisonDataError, Event, Group, SurvivalSample};
pub use descriptive::{
    describe, ComparisonTime, DescriptiveError, DescriptiveResult, GroupDescription,
    IntervalEstimate, QuantileEstimate, RmstComparison, RmstContrast, RmstWindow,
    SignificanceLevel,
};
pub use methods::{
    breslow_tarone_ware, crossing_times, fixed_point, lin_wang, lin_xu, long_term, short_term,
    weighted_kaplan_meier, ComparisonError, FixedPointEstimate, FixedPointResult, FixedPointScale,
    FixedPointTest, LinWangResult, LinXuResult, LongTermResult, ShortTermResult, Side, Weight,
    WeightedKaplanMeierResult, WeightedRankResult,
};
pub use overall::{overall_test, LogRankResult, OverallConfiguration, OverallError, OverallResult};
pub use plot::{
    comparison_curves, smooth_hazard_curves, ComparisonCurves, ComparisonHazardCurves, GroupCurve,
    GroupHazardCurve, HazardCurvePoint, SurvivalCurvePoint,
};
pub use tshrc::{two_stage, TwoStageConfiguration, TwoStageError, TwoStageResult};
