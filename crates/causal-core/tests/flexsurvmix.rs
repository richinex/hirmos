use hirmos_causal_core::survival::flexsurv::distribution::FlexSurvDistribution;
use hirmos_causal_core::survival::flexsurv::mixture::{
    fit_mixture, EmControl, EventKnowledge, FlexSurvMixFit, FlexSurvMixPlan, MixtureComponent,
    MixtureObservation, MixturePredictionRequest, MixtureSimulationConfiguration,
    MixtureUncertainty,
};
use hirmos_causal_core::survival::flexsurv::model::CovariateMatrix;
use hirmos_causal_core::survival::flexsurv::model::FlexSurvFamily;
use hirmos_causal_core::survival::flexsurv::multistate::MixtureMultiStateModel;
use hirmos_causal_core::survival::flexsurv::observation::SurvivalObservation;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/flexsurvmix.json")).unwrap()
}

fn numbers(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_f64().unwrap())
        .collect()
}

fn number_or_numbers(value: &Value) -> Vec<f64> {
    match value.as_array() {
        Some(_) => numbers(value),
        None => vec![value.as_f64().unwrap()],
    }
}

fn close(actual: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs().max(1.0),
        "{label}: actual={actual:.17e}, expected={expected:.17e}"
    );
}

fn inputs() -> (Vec<MixtureObservation>, Vec<MixtureComponent>, Vec<f64>) {
    let fixture = fixture();
    let times = numbers(&fixture["data"]["time"]);
    let statuses = numbers(&fixture["data"]["status"]);
    let events = fixture["data"]["event"].as_array().unwrap();
    let observations = times
        .iter()
        .zip(statuses)
        .zip(events)
        .map(|((time, status), event)| MixtureObservation {
            survival: if status == 1.0 {
                SurvivalObservation::exact(*time).unwrap()
            } else {
                SurvivalObservation::right_censored(*time).unwrap()
            },
            event: match event.as_f64() {
                Some(value) => EventKnowledge::Known(value as usize - 1),
                None => EventKnowledge::Unknown,
            },
        })
        .collect();
    let components = fixture["initial"]["components"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(index, values)| {
            MixtureComponent::new(
                format!("event{}", index + 1),
                FlexSurvFamily::Gamma,
                numbers(values),
            )
        })
        .collect();
    (
        observations,
        components,
        numbers(&fixture["initial"]["probabilities"]),
    )
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn fixed_mixture_likelihood_matches_flexsurvmix() {
    let fixture = fixture();
    let (observations, components, probabilities) = inputs();
    let fit = fit_mixture(FlexSurvMixPlan::fixed(observations, components, probabilities).unwrap())
        .unwrap();
    close(
        fit.log_likelihood,
        fixture["fixed"]["loglik"].as_f64().unwrap(),
        3e-12,
        "fixed flexsurvmix log likelihood",
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn direct_mixture_search_matches_flexsurvmix() {
    let fixture = fixture();
    let (observations, components, probabilities) = inputs();
    let fit =
        fit_mixture(FlexSurvMixPlan::direct(observations, components, probabilities).unwrap())
            .unwrap();
    close(
        fit.log_likelihood,
        fixture["direct"]["loglik"].as_f64().unwrap(),
        3e-7,
        "direct flexsurvmix log likelihood",
    );
    let expected = numbers(&fixture["direct"]["estimates"]);
    for (actual, target) in fit.probabilities.iter().zip(&expected[..2]) {
        close(*actual, *target, 3e-6, "flexsurvmix event probability");
    }
    for (actual, target) in fit
        .natural_component_parameters
        .iter()
        .flatten()
        .zip(&expected[2..])
    {
        close(*actual, *target, 3e-6, "flexsurvmix component parameter");
    }
    assert_eq!(
        fit.convergence,
        fixture["direct"]["convergence"].as_u64().unwrap() as usize
    );
    let MixtureUncertainty::Hessian {
        covariance,
        covariance_order,
        ..
    } = &fit.uncertainty
    else {
        panic!("direct flexsurvmix fit should carry Hessian uncertainty")
    };
    assert_eq!(
        *covariance_order,
        fixture["direct"]["covariance_order"].as_u64().unwrap() as usize
    );
    for (actual, target) in covariance
        .iter()
        .zip(numbers(&fixture["direct"]["covariance"]))
    {
        close(*actual, target, 4e-9, "flexsurvmix covariance");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn em_mixture_search_matches_flexsurvmix() {
    let fixture = fixture();
    let (observations, components, probabilities) = inputs();
    let fit = fit_mixture(
        FlexSurvMixPlan::em(
            observations,
            components,
            probabilities,
            EmControl::flexsurv_defaults(),
        )
        .unwrap(),
    )
    .unwrap();
    close(
        fit.log_likelihood,
        fixture["em"]["loglik"].as_f64().unwrap(),
        3e-7,
        "EM flexsurvmix log likelihood",
    );
    let expected = numbers(&fixture["em"]["estimates"]);
    for (actual, target) in fit.probabilities.iter().zip(&expected[..2]) {
        close(*actual, *target, 3e-6, "EM flexsurvmix event probability");
    }
    for (actual, target) in fit
        .natural_component_parameters
        .iter()
        .flatten()
        .zip(&expected[2..])
    {
        close(*actual, *target, 3e-6, "EM flexsurvmix component parameter");
    }
    let MixtureUncertainty::Hessian {
        covariance,
        covariance_order,
        ..
    } = &fit.uncertainty
    else {
        panic!("EM flexsurvmix fit should carry Hessian uncertainty")
    };
    assert_eq!(
        *covariance_order,
        fixture["em"]["covariance_order"].as_u64().unwrap() as usize
    );
    for (actual, target) in covariance.iter().zip(numbers(&fixture["em"]["covariance"])) {
        close(*actual, target, 3e-7, "EM flexsurvmix covariance");
    }
}

fn covariate_inputs() -> (
    Vec<MixtureObservation>,
    Vec<MixtureComponent>,
    CovariateMatrix,
) {
    let fixture = fixture();
    let values = &fixture["covariates"];
    let times = numbers(&values["data"]["time"]);
    let statuses = numbers(&values["data"]["status"]);
    let events = values["data"]["event"].as_array().unwrap();
    let z = numbers(&values["data"]["z"]);
    let observations = times
        .iter()
        .zip(statuses)
        .zip(events)
        .map(|((time, status), event)| MixtureObservation {
            survival: if status == 1.0 {
                SurvivalObservation::exact(*time).unwrap()
            } else {
                SurvivalObservation::right_censored(*time).unwrap()
            },
            event: match event.as_f64() {
                Some(value) => EventKnowledge::Known(value as usize - 1),
                None => EventKnowledge::Unknown,
            },
        })
        .collect::<Vec<_>>();
    let design = CovariateMatrix::new(observations.len(), 1, z).unwrap();
    let components = values["initial"]["components"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(index, values)| {
            let initial = numbers(values);
            MixtureComponent::new(
                format!("event{}", index + 1),
                FlexSurvFamily::Exponential,
                vec![initial[0]],
            )
            .with_location_covariates(design.clone(), vec![initial[1]])
            .unwrap()
        })
        .collect();
    (observations, components, design)
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn covariate_models_match_flexsurvmix_direct_and_em() {
    let fixture = fixture();
    let oracle = &fixture["covariates"];
    for method in ["direct", "em"] {
        let (observations, components, design) = covariate_inputs();
        let probabilities = numbers(&oracle["initial"]["probabilities"]);
        let plan = if method == "direct" {
            FlexSurvMixPlan::direct(observations, components, probabilities).unwrap()
        } else {
            FlexSurvMixPlan::em(
                observations,
                components,
                probabilities,
                EmControl::flexsurv_defaults(),
            )
            .unwrap()
        }
        .with_probability_covariates(design, vec![0.0])
        .unwrap();
        let fit = fit_mixture(plan).unwrap();
        close(
            fit.log_likelihood,
            oracle[method]["loglik"].as_f64().unwrap(),
            3e-7,
            &format!("covariate flexsurvmix {method} likelihood"),
        );
        let expected = numbers(&oracle[method]["estimates"]);
        for (actual, target) in fit.probabilities.iter().zip(&expected[..2]) {
            close(*actual, *target, 3e-6, "covariate mixture probability");
        }
        close(
            fit.probability_coefficients[0],
            expected[2],
            3e-6,
            "covariate mixture probability coefficient",
        );
        let actual_components = fit
            .natural_component_parameters
            .iter()
            .zip(&fit.component_coefficients)
            .flat_map(|(baseline, coefficients)| {
                baseline.iter().chain(coefficients.iter()).copied()
            })
            .collect::<Vec<_>>();
        for (actual, target) in actual_components.iter().zip(&expected[3..]) {
            close(*actual, *target, 3e-6, "covariate component parameter");
        }

        if method == "direct" {
            let mut actual_probabilities = Vec::new();
            let mut actual_means = Vec::new();
            for z in numbers(&oracle["newdata"]["z"]) {
                let profile = fit
                    .prediction_profile(vec![z], vec![vec![z], vec![z]])
                    .unwrap();
                let predictions = fit
                    .predict_events(&profile, MixturePredictionRequest::Mean)
                    .unwrap();
                actual_probabilities.extend(
                    predictions
                        .iter()
                        .map(|prediction| prediction.event_probability),
                );
                actual_means.extend(predictions.iter().map(|prediction| prediction.value));
            }
            for (actual, target) in actual_probabilities
                .iter()
                .zip(numbers(&oracle["newdata"]["probabilities"]))
            {
                close(*actual, target, 3e-6, "newdata event probability");
            }
            for (actual, target) in actual_means
                .iter()
                .zip(numbers(&oracle["newdata"]["means"]))
            {
                // The conditional mean is the inverse fitted exponential
                // rate, so the accepted 3e-6 coefficient deviation is
                // amplified by that nonlinear transform on wasm32.
                close(*actual, target, 6e-6, "newdata conditional mean");
            }

            let configuration = MixtureSimulationConfiguration::new(
                oracle["newdata"]["interval_replicates"].as_u64().unwrap() as usize,
                oracle["newdata"]["interval_seed"].as_u64().unwrap() as u32,
            )
            .unwrap();
            let mut probability_lower = Vec::new();
            let mut probability_upper = Vec::new();
            let mut profiles = Vec::new();
            for z in numbers(&oracle["newdata"]["z"]) {
                let profile = fit
                    .prediction_profile(vec![z], vec![vec![z], vec![z]])
                    .unwrap();
                let probability = fit
                    .predict_event_probabilities_with_uncertainty(&profile, configuration)
                    .unwrap();
                probability_lower.extend(probability.iter().map(|value| value.lower));
                probability_upper.extend(probability.iter().map(|value| value.upper));
                profiles.push(profile);
            }
            let mean_intervals = fit
                .predict_event_profiles_with_uncertainty(
                    &profiles,
                    MixturePredictionRequest::Mean,
                    configuration,
                )
                .unwrap();
            let mean_lower = mean_intervals
                .iter()
                .flat_map(|profile| profile.iter().map(|value| value.lower))
                .collect::<Vec<_>>();
            let mean_upper = mean_intervals
                .iter()
                .flat_map(|profile| profile.iter().map(|value| value.upper))
                .collect::<Vec<_>>();
            for (actual, target) in probability_lower
                .iter()
                .zip(numbers(&oracle["newdata"]["probability_lower"]))
            {
                close(*actual, target, 1e-6, "newdata probability lower");
            }
            for (actual, target) in probability_upper
                .iter()
                .zip(numbers(&oracle["newdata"]["probability_upper"]))
            {
                close(*actual, target, 1e-6, "newdata probability upper");
            }
            for (actual, target) in mean_lower
                .iter()
                .zip(numbers(&oracle["newdata"]["mean_lower"]))
            {
                close(*actual, target, 7e-6, "newdata mean lower");
            }
            for (actual, target) in mean_upper
                .iter()
                .zip(numbers(&oracle["newdata"]["mean_upper"]))
            {
                close(*actual, target, 7e-6, "newdata mean upper");
            }
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn impossible_mixture_states_stop_at_construction() {
    let (observations, components, _) = inputs();
    assert!(
        FlexSurvMixPlan::fixed(observations.clone(), components.clone(), vec![1.0, 0.0]).is_err()
    );
    let mut invalid = observations;
    invalid[0].event = EventKnowledge::Known(2);
    assert!(FlexSurvMixPlan::direct(invalid, components, vec![0.5, 0.5]).is_err());

    let (mut partial, components, _) = inputs();
    partial[0].event = EventKnowledge::Possible(vec![0, 1]);
    assert_eq!(
        FlexSurvMixPlan::fixed(partial.clone(), components.clone(), vec![0.5, 0.5]),
        Err(
            hirmos_causal_core::survival::flexsurv::mixture::MixtureError::FlexSurv232RejectsPartialEvents
        )
    );
    let corrected =
        FlexSurvMixPlan::fixed_with_corrected_partial_events(partial, components, vec![0.5, 0.5])
            .unwrap();
    assert!(fit_mixture(corrected).unwrap().log_likelihood.is_finite());
}

fn pathway_model() -> MixtureMultiStateModel {
    let fixture = fixture();
    let oracle = &fixture["pathways"]["fits"];
    let models = ["stable", "response", "progression"]
        .into_iter()
        .map(|state| {
            let value = &oracle[state];
            let event_names = match value["event_names"].as_array() {
                Some(names) => names
                    .iter()
                    .map(|name| name.as_str().unwrap().to_owned())
                    .collect(),
                None => vec![value["event_names"].as_str().unwrap().to_owned()],
            };
            let probabilities = number_or_numbers(&value["probabilities"]);
            let component_distributions = value["distributions"]
                .as_array()
                .unwrap()
                .iter()
                .map(|distribution| {
                    FlexSurvDistribution::exponential(
                        number_or_numbers(&distribution["parameters"])[0],
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            let fit = FlexSurvMixFit::from_fitted_components(
                event_names,
                probabilities,
                component_distributions.iter().map(|_| vec![1.0]).collect(),
                component_distributions,
            )
            .unwrap();
            (state.to_owned(), fit)
        })
        .collect();
    MixtureMultiStateModel::new(models).unwrap()
}

fn fitted_pathway_model() -> MixtureMultiStateModel {
    let states = [
        ("stable", vec!["response", "progression", "death"]),
        ("response", vec!["progression", "death"]),
        ("progression", vec!["death"]),
    ];
    let models = states
        .into_iter()
        .map(|(state, events)| {
            let count = events.len();
            let observations = (0..count)
                .map(|event| MixtureObservation {
                    survival: SurvivalObservation::exact(1.0).unwrap(),
                    event: EventKnowledge::Known(event),
                })
                .collect::<Vec<_>>();
            let components = events
                .iter()
                .map(|event| MixtureComponent::new(*event, FlexSurvFamily::Exponential, vec![1.0]))
                .collect::<Vec<_>>();
            let probabilities = vec![1.0 / count as f64; count];
            let fit = fit_mixture(
                FlexSurvMixPlan::em(
                    observations,
                    components,
                    probabilities,
                    EmControl::flexsurv_defaults(),
                )
                .unwrap(),
            )
            .unwrap();
            (state.to_owned(), fit)
        })
        .collect::<Vec<_>>();
    MixtureMultiStateModel::new(models).unwrap()
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn mixture_multistate_path_summaries_match_fmixmsm() {
    let fixture = fixture();
    let oracle = &fixture["pathways"];
    let model = pathway_model();
    assert!(!model.has_cycle());
    let names = oracle["names"].as_array().unwrap();
    for (path, name) in model.pathways().iter().zip(names) {
        assert_eq!(path.states.join("-"), name.as_str().unwrap());
    }
    for (actual, target) in model
        .pathway_probabilities()
        .unwrap()
        .iter()
        .map(|summary| summary.value)
        .zip(numbers(&oracle["probability"]))
    {
        close(actual, target, 2e-15, "ppath_fmixmsm");
    }
    for (actual, target) in model
        .final_state_probabilities()
        .unwrap()
        .iter()
        .map(|summary| summary.value)
        .zip(number_or_numbers(&oracle["final_probability"]))
    {
        close(actual, target, 2e-15, "ppath_fmixmsm final");
    }
    for (actual, target) in model
        .mean_time_to_final()
        .unwrap()
        .iter()
        .map(|summary| summary.value)
        .zip(numbers(&oracle["mean"]))
    {
        close(actual, target, 2e-15, "meanfinal_fmixmsm");
    }
    for (actual, target) in model
        .mean_time_to_final_by_state()
        .unwrap()
        .iter()
        .map(|summary| summary.value)
        .zip(number_or_numbers(&oracle["final_mean"]))
    {
        close(actual, target, 2e-15, "meanfinal_fmixmsm final");
    }
    let actual = model
        .time_to_final_quantiles(
            oracle["quantile_n"].as_u64().unwrap() as usize,
            numbers(&oracle["quantile_probabilities"]),
            oracle["quantile_seed"].as_u64().unwrap() as u32,
        )
        .unwrap();
    for (actual, target) in actual.iter().zip(numbers(&oracle["quantile"])) {
        close(actual.value, target, 2e-15, "qfinal_fmixmsm");
    }
    let actual = model
        .time_to_final_quantiles_by_state(
            oracle["quantile_n"].as_u64().unwrap() as usize,
            numbers(&oracle["quantile_probabilities"]),
            oracle["quantile_seed"].as_u64().unwrap() as u32,
        )
        .unwrap();
    for (actual, target) in actual.iter().zip(numbers(&oracle["final_quantile"])) {
        close(actual.value, target, 2e-15, "qfinal_fmixmsm final");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn mixture_multistate_parameter_intervals_match_fmixmsm() {
    let fixture = fixture();
    let oracle = &fixture["pathways"];
    let model = fitted_pathway_model();
    let configuration = MixtureSimulationConfiguration::new(
        oracle["interval_replicates"].as_u64().unwrap() as usize,
        oracle["interval_seed"].as_u64().unwrap() as u32,
    )
    .unwrap();
    let probability = model.pathway_probability_intervals(configuration).unwrap();
    let means = model.mean_time_to_final_intervals(configuration).unwrap();
    let final_probability = model
        .final_state_probability_intervals(configuration)
        .unwrap();
    let final_means = model
        .mean_time_to_final_by_state_intervals(configuration)
        .unwrap();
    for (actual, target) in probability
        .iter()
        .map(|value| value.lower)
        .zip(numbers(&oracle["probability_lower"]))
    {
        close(actual, target, 4e-6, "fmixmsm probability lower");
    }
    for (actual, target) in probability
        .iter()
        .map(|value| value.upper)
        .zip(numbers(&oracle["probability_upper"]))
    {
        close(actual, target, 4e-6, "fmixmsm probability upper");
    }
    for (actual, target) in means
        .iter()
        .map(|value| value.lower)
        .zip(numbers(&oracle["mean_lower"]))
    {
        close(actual, target, 4e-6, "fmixmsm mean lower");
    }
    for (actual, target) in means
        .iter()
        .map(|value| value.upper)
        .zip(numbers(&oracle["mean_upper"]))
    {
        close(actual, target, 4e-6, "fmixmsm mean upper");
    }
    close(
        final_probability[0].lower,
        oracle["final_probability_lower"].as_f64().unwrap(),
        4e-6,
        "fmixmsm final probability lower",
    );
    close(
        final_probability[0].upper,
        oracle["final_probability_upper"].as_f64().unwrap(),
        4e-6,
        "fmixmsm final probability upper",
    );
    close(
        final_means[0].lower,
        oracle["final_mean_lower"].as_f64().unwrap(),
        4e-6,
        "fmixmsm final mean lower",
    );
    close(
        final_means[0].upper,
        oracle["final_mean_upper"].as_f64().unwrap(),
        4e-6,
        "fmixmsm final mean upper",
    );

    let quantile_configuration = MixtureSimulationConfiguration::new(
        oracle["quantile_interval_replicates"].as_u64().unwrap() as usize,
        oracle["quantile_interval_seed"].as_u64().unwrap() as u32,
    )
    .unwrap();
    let simulations = oracle["quantile_interval_n"].as_u64().unwrap() as usize;
    let quantiles = model
        .time_to_final_quantile_intervals(
            simulations,
            numbers(&oracle["quantile_probabilities"]),
            quantile_configuration,
        )
        .unwrap();
    let final_quantiles = model
        .time_to_final_quantile_intervals_by_state(
            simulations,
            numbers(&oracle["quantile_probabilities"]),
            quantile_configuration,
        )
        .unwrap();
    for (actual, target) in quantiles
        .iter()
        .map(|value| value.lower)
        .zip(numbers(&oracle["quantile_interval_lower"]))
    {
        close(actual, target, 4e-6, "fmixmsm quantile lower");
    }
    for (actual, target) in quantiles
        .iter()
        .map(|value| value.upper)
        .zip(numbers(&oracle["quantile_interval_upper"]))
    {
        close(actual, target, 4e-6, "fmixmsm quantile upper");
    }
    for (actual, target) in final_quantiles
        .iter()
        .map(|value| value.lower)
        .zip(numbers(&oracle["final_quantile_interval_lower"]))
    {
        close(actual, target, 4e-6, "fmixmsm final quantile lower");
    }
    for (actual, target) in final_quantiles
        .iter()
        .map(|value| value.upper)
        .zip(numbers(&oracle["final_quantile_interval_upper"]))
    {
        close(actual, target, 4e-6, "fmixmsm final quantile upper");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn mixture_multistate_newdata_matches_fmixmsm_path_rules() {
    let fixture = fixture();
    let oracle = &fixture["covariates"];
    let (observations, components, design) = covariate_inputs();
    let fit = fit_mixture(
        FlexSurvMixPlan::direct(
            observations,
            components,
            numbers(&oracle["initial"]["probabilities"]),
        )
        .unwrap()
        .with_probability_covariates(design, vec![0.0])
        .unwrap(),
    )
    .unwrap();
    let model = MixtureMultiStateModel::new(vec![("start".to_owned(), fit)]).unwrap();
    let mut probabilities = Vec::new();
    let mut means = Vec::new();
    for z in numbers(&oracle["newdata"]["z"]) {
        let profile = model
            .prediction_profile(vec![vec![z]], vec![vec![vec![z], vec![z]]])
            .unwrap();
        probabilities.extend(
            model
                .pathway_probabilities_for(&profile)
                .unwrap()
                .into_iter()
                .map(|value| value.value),
        );
        means.extend(
            model
                .mean_time_to_final_for(&profile)
                .unwrap()
                .into_iter()
                .map(|value| value.value),
        );
    }
    for (actual, target) in probabilities
        .iter()
        .zip(numbers(&oracle["newdata"]["probabilities"]))
    {
        close(*actual, target, 3e-6, "fmixmsm newdata probability");
    }
    for (actual, target) in means.iter().zip(numbers(&oracle["newdata"]["means"])) {
        close(*actual, target, 6e-6, "fmixmsm newdata mean");
    }
}
