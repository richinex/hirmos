use hirmos_causal_core::survival::flexsurv::distribution::FlexSurvDistribution;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    distributions: Vec<DistributionCase>,
}

#[derive(Deserialize)]
struct DistributionCase {
    distribution: String,
    time: f64,
    parameters: Vec<f64>,
    density: f64,
    cdf: f64,
    survival: f64,
    hazard: f64,
    cumulative_hazard: f64,
}

fn distribution(case: &DistributionCase) -> FlexSurvDistribution {
    distribution_from(&case.distribution, &case.parameters)
}

fn distribution_from(name: &str, parameters: &[f64]) -> FlexSurvDistribution {
    match name {
        "exp" => FlexSurvDistribution::exponential(parameters[0]).unwrap(),
        "weibull" => FlexSurvDistribution::weibull(parameters[0], parameters[1]).unwrap(),
        "weibullPH" => FlexSurvDistribution::weibull_ph(parameters[0], parameters[1]).unwrap(),
        "lnorm" => FlexSurvDistribution::log_normal(parameters[0], parameters[1]).unwrap(),
        "gamma" => FlexSurvDistribution::gamma(parameters[0], parameters[1]).unwrap(),
        "gompertz" => FlexSurvDistribution::gompertz(parameters[0], parameters[1]).unwrap(),
        "llogis" => FlexSurvDistribution::log_logistic(parameters[0], parameters[1]).unwrap(),
        "gengamma" => {
            FlexSurvDistribution::generalized_gamma(parameters[0], parameters[1], parameters[2])
                .unwrap()
        }
        "gengamma.orig" => FlexSurvDistribution::generalized_gamma_original(
            parameters[0],
            parameters[1],
            parameters[2],
        )
        .unwrap(),
        "genf" => FlexSurvDistribution::generalized_f(
            parameters[0],
            parameters[1],
            parameters[2],
            parameters[3],
        )
        .unwrap(),
        "genf.orig" => FlexSurvDistribution::generalized_f_original(
            parameters[0],
            parameters[1],
            parameters[2],
            parameters[3],
        )
        .unwrap(),
        name => panic!("unhandled flexsurv distribution {name}"),
    }
}

fn close(actual: f64, expected: f64, tolerance: f64, label: &str) {
    let scale = expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance * scale,
        "{label}: actual {actual:.17e}, expected {expected:.17e}"
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn built_in_distributions_match_flexsurv_2_3_2() {
    let fixture: Fixture =
        serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap();

    for case in &fixture.distributions {
        let actual = distribution(case).evaluate(case.time).unwrap();
        let label = format!("{} at {}", case.distribution, case.time);
        close(
            actual.density,
            case.density,
            2e-10,
            &format!("{label} density"),
        );
        close(actual.cdf, case.cdf, 2e-10, &format!("{label} cdf"));
        close(
            actual.survival,
            case.survival,
            2e-10,
            &format!("{label} survival"),
        );
        close(actual.hazard, case.hazard, 2e-9, &format!("{label} hazard"));
        close(
            actual.cumulative_hazard,
            case.cumulative_hazard,
            2e-9,
            &format!("{label} cumulative hazard"),
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn quantiles_means_and_restricted_means_match_flexsurv_2_3_2() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap();
    for case in fixture["distribution_summaries"].as_array().unwrap() {
        let parameters = case["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        let name = case["distribution"].as_str().unwrap();
        let distribution = distribution_from(name, &parameters);
        for (probability, expected) in case["quantile_probabilities"]
            .as_array()
            .unwrap()
            .iter()
            .zip(case["quantiles"].as_array().unwrap())
        {
            close(
                distribution
                    .quantile(probability.as_f64().unwrap())
                    .unwrap(),
                expected.as_f64().unwrap(),
                3e-8,
                &format!("{name} quantile"),
            );
        }
        close(
            distribution.mean().unwrap(),
            case["mean"].as_f64().unwrap(),
            2e-7,
            &format!("{name} mean"),
        );
        close(
            distribution
                .restricted_mean(
                    case["rmst_start"].as_f64().unwrap(),
                    case["rmst_end"].as_f64().unwrap(),
                )
                .unwrap(),
            case["rmst"].as_f64().unwrap(),
            2e-7,
            &format!("{name} rmst"),
        );
    }
}
