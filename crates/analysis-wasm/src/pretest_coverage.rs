use crate::protocol::AnalysisResult;
use hirmos_causal_core::pretest_coverage::{coverage, Error, Scenario};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    target_bias: f64,
    pretest_bias: f64,
    correlation: f64,
    interval_alpha: f64,
    pretest_alpha: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Evidence {
    kind: &'static str,
    request: Request,
    unconditional: f64,
    conditional: f64,
    pass_probability: f64,
    joint_probability: f64,
    integration_error: f64,
    evaluations: usize,
}

pub(crate) fn fit(request: Request) -> Result<AnalysisResult, String> {
    let result = coverage(Scenario {
        target_bias: request.target_bias,
        pretest_bias: request.pretest_bias,
        correlation: request.correlation,
        interval_alpha: request.interval_alpha,
        pretest_alpha: request.pretest_alpha,
    }).map_err(|error| match error {
        Error::InvalidBias => "Choose finite standardised biases.",
        Error::InvalidCorrelation => "The correlation must be between -1 and 1.",
        Error::InvalidLevel => "Test levels must be strictly between zero and one.",
        Error::NumericalRange => "The scenario exceeds the numerical range of this calculation.",
        Error::PassProbabilityTooSmall => "Passing the pre-test is too unlikely to calculate conditional coverage reliably for this scenario.",
        Error::IntegrationDidNotConverge => "The coverage calculation did not reach its numerical tolerance. No result was substituted.",
    }.to_string())?;
    Ok(AnalysisResult::PretestCoverage {
        evidence: Evidence {
            kind: "pretestCoverage",
            request,
            unconditional: result.unconditional,
            conditional: result.conditional,
            pass_probability: result.pass_probability,
            joint_probability: result.joint_probability,
            integration_error: result.integration_error,
            evaluations: result.evaluations,
        },
    })
}
