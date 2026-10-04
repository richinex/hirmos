//! Browser boundary for two-period, observational DR DiD sensitivity.
use crate::{protocol::{AnalysisResult, DidNormalization, PropensityStatus}, estimation::adjusted_panel};
use hirmos_causal_core::{did, did_sensitivity as core, lbfgsb::LbfgsbTermination};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    pub propensity_fit: crate::protocol::DidPropensityFit,
    pub folds: usize,
    pub seed: u32,
    pub trimming: f64,
    pub normalization: DidNormalization,
    pub scenarios: Vec<[f64; 2]>,
    pub rho: f64,
    /// One-sided endpoint confidence, following the reference implementation.
    pub level: f64,
    pub null: f64,
    pub outcome_shares: Vec<f64>,
    pub riesz_shares: Vec<f64>,
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct Bounds { pub effect: [f64;2], pub interval: [f64;2] }
impl From<core::Bounds> for Bounds {
    fn from(b:core::Bounds)->Self { Self { effect:b.effect, interval:b.interval } }
}
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub(crate) enum RieszEstimate { Orthogonal, NonOrthogonalReferenceRecovery }
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct Evidence {
    pub request: Request,
    pub units: Vec<String>,
    pub times: Vec<i64>,
    pub estimate: f64,
    pub standard_error: f64,
    pub sigma2: f64,
    pub nu2: f64,
    pub riesz_estimate: RieszEstimate,
    pub optimizer_status: Vec<PropensityStatus>,
    pub baseline: Bounds,
    pub scenarios: Vec<Bounds>,
    pub grid: Vec<Vec<Bounds>>,
    pub robustness_value: f64,
    pub robustness_value_ci: f64,
}

fn problem(error:core::Error)->String {
    match error {
        core::Error::Input(did::Error::PropensityNotConverged { .. }) => "The propensity fit did not converge. No sensitivity result was recorded. Review covariate redundancy and treatment overlap.",
        core::Error::Input(did::Error::Regression) => "A cross-fitted outcome or propensity regression failed. Check baseline covariates and fold sizes.",
        core::Error::Input(did::Error::PropensityBoundary) => "Estimated propensity reached a probability boundary. Check treatment overlap.",
        core::Error::Input(did::Error::MissingGroup | did::Error::NonBinary) => "DiD sensitivity requires treated and comparison units with binary assignment.",
        core::Error::Input(did::Error::NonFinite) => "DiD sensitivity produced non-finite values. Check covariate scale and overlap.",
        core::Error::Input(did::Error::Shape | did::Error::RequiresTwoPeriods) => "DiD sensitivity requires paired periods and aligned baseline covariates.",
        core::Error::InvalidScenario => "Use confounding shares from zero to less than one, a correlation from minus one to one, and a valid confidence level.",
        core::Error::InvalidEmbedding => "The sensitivity scores do not align with the panel units.",
        core::Error::DegenerateOutcome => "The outcome differences have no residual variation for this sensitivity analysis.",
        core::Error::InvalidGrid => "Choose nonempty confounding-share axes.",
    }.into()
}
fn status(value:LbfgsbTermination)->PropensityStatus {
    match value {
        LbfgsbTermination::ProjectedGradient => PropensityStatus::ProjectedGradient,
        LbfgsbTermination::FunctionTolerance => PropensityStatus::FunctionTolerance,
        LbfgsbTermination::IterationLimit => PropensityStatus::IterationLimit,
        LbfgsbTermination::LineSearchFailed => PropensityStatus::LineSearchFailed,
    }
}
pub(crate) fn fit(values:&[f64], rows:usize, columns:usize, units:&[String], times:&[i64], request:Request)->Result<AnalysisResult,String> {
    // Validate every requested calculation before fitting, including cost and axis ordering.
    if request.scenarios.is_empty() || request.scenarios.len()>100
        || request.outcome_shares.len()<2 || request.outcome_shares.len()>51
        || request.riesz_shares.len()<2 || request.riesz_shares.len()>51
        || request.level<=0.5 || request.level>=1.0 {
        return Err("Choose 1 to 100 scenarios, 2 to 51 values per axis, and an endpoint confidence level above 50% and below 100%.".into());
    }
    for axis in [&request.outcome_shares,&request.riesz_shares] {
        if axis.windows(2).any(|w|w[0]>=w[1]) {
            return Err("Confounding-share axes must be strictly increasing.".into());
        }
        for &v in axis { core::Scenario::new(v,v,request.rho,request.level,request.null).map_err(problem)?; }
    }
    let scenarios=request.scenarios.iter().map(|s|core::Scenario::new(s[0],s[1],request.rho,request.level,request.null)).collect::<Result<Vec<_>,_>>().map_err(problem)?;
    let baseline=core::Scenario::new(0.0,0.0,request.rho,request.level,request.null).map_err(problem)?;
    let (panel,covariates)=adjusted_panel(values,rows,columns,units,times)?;
    let baseline_covariates=DMatrix::from_fn(panel.units.len(),columns-2,|i,j|covariates[(2*i,j)]);
    let sample=did::PairedSample::from_panel(&panel,&baseline_covariates).map_err(|e|problem(core::Error::Input(e)))?;
    let normalization=match request.normalization { DidNormalization::InSample=>did::Normalization::InSample, DidNormalization::Population=>did::Normalization::Population };
    let plan=did::prepare(&sample,request.folds,request.seed,request.trimming,normalization)
        .map_err(|_|"Use at least two folds, no more than the smaller group, and trimming between zero and one half.".to_owned())?;
    let fitted=core::fit(&plan).map_err(problem)?;
    let f=&fitted.sensitivity;
    let robustness=f.robustness(baseline);
    let grid=f.grid(&request.outcome_shares,&request.riesz_shares,baseline).map_err(problem)?
        .into_iter().map(|row|row.into_iter().map(Bounds::from).collect()).collect();
    let scenarios=scenarios.into_iter().map(|s|f.bounds(s).into()).collect();
    let evidence=Evidence {
        request, units:panel.units, times:panel.times,
        estimate:f.estimate.coef, standard_error:f.estimate.se, sigma2:f.sigma2, nu2:f.nu2,
        riesz_estimate:match f.riesz_estimate { core::RieszEstimate::Orthogonal=>RieszEstimate::Orthogonal,core::RieszEstimate::NonOrthogonalReferenceRecovery=>RieszEstimate::NonOrthogonalReferenceRecovery },
        optimizer_status:fitted.propensity_termination.into_iter().map(status).collect(),
        baseline:f.bounds(baseline).into(), scenarios, grid,
        robustness_value:robustness.value, robustness_value_ci:robustness.interval_value,
    };
    Ok(AnalysisResult::DidSensitivity { evidence })
}
