use hirmos_causal_core::survival::flexsurv::distribution::FlexSurvDistribution;
use hirmos_causal_core::survival::flexsurv::fit::{
    fit, FlexSurvFitPlan, SurvivalDataset, SurvivalRecord,
};
use hirmos_causal_core::survival::flexsurv::model::{
    CovariateMatrix, FlexSurvFamily, RegressionModel,
};
use hirmos_causal_core::survival::flexsurv::multistate::{
    compare_factor_strata, AalenJohansenData, AalenJohansenStratum, CompetingRisksModel,
    MarkovControl, MultiStateModel, OccupancyModel, StratumLabel, TransitionGraph,
    TransitionHistoryRow, TransitionModel,
};
use hirmos_causal_core::survival::flexsurv::observation::SurvivalObservation;
use hirmos_causal_core::survival::flexsurv::r_optim::ROptimControl;
use hirmos_causal_core::survival::flexsurv::uncertainty::PredictionSimulationConfiguration;
use serde_json::Value;
use std::num::NonZeroUsize;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/flexsurv_multistate.json")).unwrap()
}

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap()
}

fn numbers(value: &Value) -> Vec<f64> {
    if let Some(values) = value.as_array() {
        values.iter().map(number).collect()
    } else {
        vec![number(value)]
    }
}

fn matrix(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|row| row.as_array().unwrap().iter().map(number))
        .collect()
}

fn close(actual: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs().max(1.0),
        "{label}: actual={actual:.17e}, expected={expected:.17e}"
    );
}

fn graph() -> TransitionGraph {
    TransitionGraph::new(
        vec!["No BOS".into(), "BOS".into(), "Dead".into()],
        vec![(0, 1), (0, 2), (1, 2)],
    )
    .unwrap()
}

fn competing_graph() -> TransitionGraph {
    TransitionGraph::new(
        vec!["No BOS".into(), "BOS".into(), "Dead".into()],
        vec![(0, 1), (0, 2)],
    )
    .unwrap()
}

fn exponential_models() -> (
    MultiStateModel<hirmos_causal_core::survival::flexsurv::multistate::ClockForward>,
    MultiStateModel<hirmos_causal_core::survival::flexsurv::multistate::ClockReset>,
) {
    let fixture = fixture();
    let transitions = fixture["exponential"]["fits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|fit| {
            TransitionModel::fixed(
                FlexSurvDistribution::exponential(number(&fit["natural"])).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    (
        MultiStateModel::clock_forward(graph(), transitions.clone()).unwrap(),
        MultiStateModel::clock_reset(graph(), transitions).unwrap(),
    )
}

fn fitted_exponential_model(
) -> MultiStateModel<hirmos_causal_core::survival::flexsurv::multistate::ClockForward> {
    MultiStateModel::clock_forward(graph(), fitted_exponential_transitions()).unwrap()
}

fn fitted_exponential_reset_model(
) -> MultiStateModel<hirmos_causal_core::survival::flexsurv::multistate::ClockReset> {
    MultiStateModel::clock_reset(graph(), fitted_exponential_transitions()).unwrap()
}

fn fitted_exponential_transitions() -> Vec<TransitionModel> {
    let fixture = fixture();
    let data = &fixture["data"];
    let years = numbers(&data["years"]);
    let statuses = numbers(&data["status"]);
    let transition_ids = numbers(&data["trans"]);
    let mut transitions = Vec::new();
    for transition in 1..=3 {
        let records = years
            .iter()
            .zip(&statuses)
            .zip(&transition_ids)
            .filter(|(_, id)| **id == transition as f64)
            .map(|((time, status), _)| {
                let observation = if *status == 1.0 {
                    SurvivalObservation::exact(*time).unwrap()
                } else {
                    SurvivalObservation::right_censored(*time).unwrap()
                };
                SurvivalRecord::new(observation)
            })
            .collect::<Vec<_>>();
        let model = RegressionModel::new(FlexSurvFamily::Exponential, records.len()).unwrap();
        let plan = FlexSurvFitPlan::optimize(
            model,
            SurvivalDataset::new(records).unwrap(),
            &[0.2],
            &[],
            &[],
            ROptimControl::bfgs_defaults(NonZeroUsize::new(1).unwrap()),
        )
        .unwrap();
        transitions.push(TransitionModel::from_fit(fit(plan).unwrap(), vec![]).unwrap());
    }
    transitions
}

fn fitted_predictable_covariate_model(
) -> MultiStateModel<hirmos_causal_core::survival::flexsurv::multistate::ClockReset> {
    let fixture = fixture();
    let data = &fixture["data"];
    let years = numbers(&data["years"]);
    let statuses = numbers(&data["status"]);
    let transition_ids = numbers(&data["trans"]);
    let process_age = numbers(&fixture["predictable_covariate"]["covariate"]);
    let mut transitions = Vec::new();
    for transition in 1..=3 {
        let selected = transition_ids
            .iter()
            .enumerate()
            .filter(|(_, id)| **id == transition as f64)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let records = selected
            .iter()
            .map(|index| {
                let observation = if statuses[*index] == 1.0 {
                    SurvivalObservation::exact(years[*index]).unwrap()
                } else {
                    SurvivalObservation::right_censored(years[*index]).unwrap()
                };
                SurvivalRecord::new(observation)
            })
            .collect::<Vec<_>>();
        let covariates = selected
            .iter()
            .map(|index| process_age[*index])
            .collect::<Vec<_>>();
        let model = RegressionModel::new(FlexSurvFamily::Exponential, records.len())
            .unwrap()
            .with_location_covariates(CovariateMatrix::new(records.len(), 1, covariates).unwrap())
            .unwrap();
        let plan = FlexSurvFitPlan::optimize(
            model,
            SurvivalDataset::new(records).unwrap(),
            &[0.2],
            &[0.0],
            &[],
            ROptimControl::bfgs_defaults(NonZeroUsize::new(2).unwrap()),
        )
        .unwrap();
        transitions.push(
            TransitionModel::from_fit_with_predictable_time_covariates(
                fit(plan).unwrap(),
                vec![number(&fixture["predictable_covariate"]["profile"])],
                vec![0],
            )
            .unwrap(),
        );
    }
    MultiStateModel::clock_reset(graph(), transitions).unwrap()
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn transition_graph_rejects_states_fmsm_should_not_accept() {
    assert!(TransitionGraph::new(Vec::new(), Vec::new()).is_err());
    assert!(TransitionGraph::new(vec!["A".into()], vec![(0, 0)]).is_err());
    assert!(TransitionGraph::new(vec!["A".into(), "A".into()], Vec::new()).is_err());
    assert!(TransitionGraph::new(vec!["A".into(), "B".into()], vec![(0, 1), (0, 1)]).is_err());
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn exponential_markov_predictions_match_flexsurv_lsoda() {
    let fixture = fixture();
    let times = numbers(&fixture["times"]);
    let (model, _) = exponential_models();
    let probabilities = model
        .transition_probabilities(times.clone(), MarkovControl::matrix_exponential())
        .unwrap();
    let lengths = model
        .total_length_of_stay(times.clone(), MarkovControl::matrix_exponential())
        .unwrap();
    let cumulative = model.cumulative_transition_hazards(&times).unwrap();

    for (index, time) in times.iter().enumerate() {
        let key = time.to_string();
        let expected = matrix(&fixture["exponential"]["pmatrix_lsoda"][&key]);
        for (cell, target) in probabilities.probabilities[index].iter().zip(expected) {
            close(*cell, target, 2e-6, "pmatrix.fs LSODA");
        }
        let expected = matrix(&fixture["exponential"]["totlos_lsoda"][&key]);
        for (cell, target) in lengths.length_of_stay[index].iter().zip(expected) {
            close(*cell, target, 2e-6, "totlos.fs LSODA");
        }
        for row in 0..3 {
            close(
                probabilities.probabilities[index][row * 3..row * 3 + 3]
                    .iter()
                    .sum(),
                1.0,
                2e-12,
                "probability row sum",
            );
            close(
                lengths.length_of_stay[index][row * 3..row * 3 + 3]
                    .iter()
                    .sum(),
                *time,
                2e-12,
                "length-of-stay row sum",
            );
        }
    }
    for (transition, actual) in cumulative.iter().enumerate() {
        let expected = numbers(&fixture["exponential"]["cumulative_hazards"][transition]);
        for (value, target) in actual.iter().zip(expected) {
            close(*value, target, 2e-15, "msfit.flexsurvreg cumulative hazard");
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn de_solve_rk4_path_matches_flexsurv() {
    let fixture = fixture();
    let times = numbers(&fixture["times"]);
    let transitions = fixture["weibull_ph"]["fits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|fit| {
            let parameters = numbers(&fit["natural"]);
            TransitionModel::fixed(
                FlexSurvDistribution::weibull_ph(parameters[0], parameters[1]).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let model = MultiStateModel::clock_forward(graph(), transitions).unwrap();
    let control = MarkovControl::de_solve_rk4(1.0).unwrap();
    let probabilities = model
        .transition_probabilities(times.clone(), control)
        .unwrap();
    let lengths = model.total_length_of_stay(times.clone(), control).unwrap();
    for (index, time) in times.iter().enumerate() {
        let key = time.to_string();
        for (actual, expected) in probabilities.probabilities[index]
            .iter()
            .zip(matrix(&fixture["weibull_ph"]["pmatrix_rk4"][&key]))
        {
            close(*actual, expected, 2e-13, "pmatrix.fs RK4");
        }
        for (actual, expected) in lengths.length_of_stay[index]
            .iter()
            .zip(matrix(&fixture["weibull_ph"]["totlos_rk4"][&key]))
        {
            close(*actual, expected, 2e-13, "totlos.fs RK4");
        }
    }

    let control = MarkovControl::de_solve_rk45_dp7(1.0).unwrap();
    let probabilities = model
        .transition_probabilities(times.clone(), control)
        .unwrap();
    let lengths = model.total_length_of_stay(times.clone(), control).unwrap();
    for (index, time) in times.iter().enumerate() {
        let key = time.to_string();
        for (actual, expected) in probabilities.probabilities[index]
            .iter()
            .zip(matrix(&fixture["weibull_ph"]["pmatrix_rk45"][&key]))
        {
            close(*actual, expected, 2e-12, "pmatrix.fs RK45DP7");
        }
        for (actual, expected) in lengths.length_of_stay[index]
            .iter()
            .zip(matrix(&fixture["weibull_ph"]["totlos_rk45"][&key]))
        {
            close(*actual, expected, 2e-12, "totlos.fs RK45DP7");
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn conditional_probabilities_match_flexsurv() {
    let fixture = fixture();
    let times = numbers(&fixture["times"]);
    let (model, _) = exponential_models();
    let states = vec![
        model.graph().state(1).unwrap(),
        model.graph().state(2).unwrap(),
    ];
    let actual = model
        .conditional_transition_probabilities(
            times.clone(),
            states,
            MarkovControl::matrix_exponential(),
        )
        .unwrap();
    for (index, time) in times.iter().enumerate() {
        let expected =
            matrix(&fixture["exponential"]["pmatrix_conditional_2_3"][&time.to_string()]);
        for (value, target) in actual.probabilities[index].iter().zip(expected) {
            close(*value, target, 5e-6, "conditional pmatrix.fs");
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn markov_parameter_intervals_match_bootci_fmsm() {
    let fixture = fixture();
    let oracle = &fixture["exponential"]["pmatrix_ci"];
    let times = numbers(&oracle["times"]);
    let model = fitted_exponential_model();
    close(
        model.information_criterion(2.0).unwrap(),
        number(&fixture["exponential"]["aic"]),
        3e-7,
        "AIC.fmsm",
    );
    let configuration = PredictionSimulationConfiguration::new(
        oracle["B"].as_u64().unwrap() as usize,
        number(&oracle["cl"]),
        oracle["seed"].as_u64().unwrap() as u32,
    )
    .unwrap();
    let actual = model
        .transition_probability_intervals(
            times.clone(),
            MarkovControl::matrix_exponential(),
            configuration,
        )
        .unwrap();
    for (index, time) in times.iter().enumerate() {
        let key = time.to_string();
        for (value, target) in actual.lower[index]
            .iter()
            .zip(matrix(&oracle["lower"][&key]))
        {
            close(*value, target, 3e-6, "pmatrix.fs lower");
        }
        for (value, target) in actual.upper[index]
            .iter()
            .zip(matrix(&oracle["upper"][&key]))
        {
            close(*value, target, 3e-6, "pmatrix.fs upper");
        }
    }

    let oracle = &fixture["exponential"]["totlos_ci"];
    let actual = model
        .length_of_stay_intervals(
            times.clone(),
            MarkovControl::matrix_exponential(),
            configuration,
        )
        .unwrap();
    for (index, time) in times.iter().enumerate() {
        let key = time.to_string();
        for (value, target) in actual.length_lower[index]
            .iter()
            .zip(matrix(&oracle["lower"][&key]))
        {
            close(*value, target, 3e-6, "totlos.fs lower");
        }
        for (value, target) in actual.length_upper[index]
            .iter()
            .zip(matrix(&oracle["upper"][&key]))
        {
            close(*value, target, 3e-6, "totlos.fs upper");
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn cumulative_hazard_covariance_matches_msfit_flexsurvreg() {
    let fixture = fixture();
    let oracle = &fixture["exponential"]["cumulative_hazard_covariance"];
    let model = fitted_exponential_model();
    let configuration = PredictionSimulationConfiguration::new(
        oracle["B"].as_u64().unwrap() as usize,
        0.95,
        oracle["seed"].as_u64().unwrap() as u32,
    )
    .unwrap();
    let times = numbers(&oracle["times"]);
    let actual = model
        .cumulative_transition_hazard_covariance(times.clone(), configuration)
        .unwrap();
    let row_times = numbers(&oracle["time"]);
    let left = numbers(&oracle["trans1"]);
    let right = numbers(&oracle["trans2"]);
    let values = numbers(&oracle["value"]);
    for row in 0..values.len() {
        let time = times
            .iter()
            .position(|time| *time == row_times[row])
            .unwrap();
        let left = left[row] as usize - 1;
        let right = right[row] as usize - 1;
        close(
            actual.covariance[time][left * 3 + right],
            values[row],
            2e-7,
            "msfit.flexsurvreg covariance",
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn aalen_johansen_fit_check_matches_ajfit_fmsm() {
    let fixture = fixture();
    let data = &fixture["data"];
    let years = numbers(&data["years"]);
    let statuses = numbers(&data["status"]);
    let transition_ids = numbers(&data["trans"]);
    let (model, _) = exponential_models();
    let histories = (1..=3)
        .map(|transition| {
            years
                .iter()
                .zip(&statuses)
                .zip(&transition_ids)
                .filter(|(_, id)| **id == transition as f64)
                .map(|((time, status), _)| {
                    let observation = if *status == 1.0 {
                        SurvivalObservation::exact(*time).unwrap()
                    } else {
                        SurvivalObservation::right_censored(*time).unwrap()
                    };
                    TransitionHistoryRow::new(observation)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let data = AalenJohansenData::new(&model, histories).unwrap();
    let actual = model
        .compare_with_aalen_johansen(data, 10.0, MarkovControl::de_solve_rk45_dp7(1e10).unwrap())
        .unwrap();
    let oracle = &fixture["exponential"]["ajfit"];
    let times = numbers(&oracle["time"]);
    let values = numbers(&oracle["val"]);
    let models = oracle["model"].as_array().unwrap();
    let states = oracle["state"].as_array().unwrap();
    assert_eq!(actual.estimates.len(), values.len());
    for (index, estimate) in actual.estimates.iter().enumerate() {
        close(estimate.time, times[index], 2e-12, "ajfit_fmsm time");
        close(
            estimate.probability,
            values[index],
            if estimate.model == OccupancyModel::AalenJohansen {
                2e-12
            } else {
                2e-6
            },
            &format!("ajfit_fmsm probability row {index}"),
        );
        let expected_model = match models[index].as_str().unwrap() {
            "Aalen-Johansen" => OccupancyModel::AalenJohansen,
            "Parametric" => OccupancyModel::Parametric,
            value => panic!("unexpected ajfit model {value}"),
        };
        assert_eq!(estimate.model, expected_model);
        assert_eq!(
            model.graph().state_name(estimate.state),
            states[index].as_str().unwrap()
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn aalen_johansen_factor_strata_preserve_each_subgroup_result() {
    let fixture = fixture();
    let data = &fixture["data"];
    let years = numbers(&data["years"]);
    let statuses = numbers(&data["status"]);
    let transition_ids = numbers(&data["trans"]);
    let (model, _) = exponential_models();
    let histories = (1..=3)
        .map(|transition| {
            years
                .iter()
                .zip(&statuses)
                .zip(&transition_ids)
                .filter(|(_, id)| **id == transition as f64)
                .map(|((time, status), _)| {
                    let observation = if *status == 1.0 {
                        SurvivalObservation::exact(*time).unwrap()
                    } else {
                        SurvivalObservation::right_censored(*time).unwrap()
                    };
                    TransitionHistoryRow::new(observation)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let first = AalenJohansenStratum::new(
        StratumLabel::new("group-a").unwrap(),
        model.clone(),
        AalenJohansenData::new(&model, histories.clone()).unwrap(),
    );
    let second = AalenJohansenStratum::new(
        StratumLabel::new("group-b").unwrap(),
        model.clone(),
        AalenJohansenData::new(&model, histories).unwrap(),
    );
    let result = compare_factor_strata(
        vec![first, second],
        10.0,
        MarkovControl::de_solve_rk45_dp7(1e10).unwrap(),
    )
    .unwrap();
    let half = result.estimates.len() / 2;
    assert!(result.estimates[..half]
        .iter()
        .all(|(label, _)| label == "group-a"));
    assert!(result.estimates[half..]
        .iter()
        .all(|(label, _)| label == "group-b"));
    for (left, right) in result.estimates[..half]
        .iter()
        .zip(&result.estimates[half..])
    {
        assert_eq!(left.1, right.1);
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn exponential_clock_reset_paths_match_sim_fmsm() {
    let fixture = fixture();
    let (_, model) = exponential_models();
    let expected = &fixture["simulation"];
    let actual = model
        .simulate_paths(
            5.0,
            0,
            expected["paths_states"].as_array().unwrap().len(),
            expected["seed"].as_u64().unwrap() as u32,
        )
        .unwrap();
    for (path, state_values) in actual
        .paths
        .iter()
        .zip(expected["paths_states"].as_array().unwrap())
    {
        let states = numbers(state_values)
            .into_iter()
            .map(|state| state as usize - 1)
            .collect::<Vec<_>>();
        assert_eq!(
            path.states
                .iter()
                .map(|state| state.index())
                .collect::<Vec<_>>(),
            states
        );
    }
    for (path, time_values) in actual
        .paths
        .iter()
        .zip(expected["paths_times"].as_array().unwrap())
    {
        for (actual, expected) in path.times.iter().zip(numbers(time_values)) {
            close(*actual, expected, 2e-15, "sim.fmsm event time");
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn vector_horizons_and_starts_preserve_flexsurv_232_and_offer_a_corrected_mode() {
    let fixture = fixture();
    let expected = &fixture["vector_simulation"];
    let (_, model) = exponential_models();
    let horizons = numbers(&expected["horizons"]);
    let starts = numbers(&expected["starts"])
        .into_iter()
        .map(|state| state as usize - 1)
        .collect::<Vec<_>>();
    let seed = expected["seed"].as_u64().unwrap() as u32;

    let source = model
        .simulate_paths_for(horizons.clone(), starts.clone(), seed)
        .unwrap();
    for (path, state_values) in source
        .paths
        .iter()
        .zip(expected["paths_states"].as_array().unwrap())
    {
        let states = numbers(state_values)
            .into_iter()
            .map(|state| state as usize - 1)
            .collect::<Vec<_>>();
        assert_eq!(
            path.states
                .iter()
                .map(|state| state.index())
                .collect::<Vec<_>>(),
            states
        );
    }
    for (path, time_values) in source
        .paths
        .iter()
        .zip(expected["paths_times"].as_array().unwrap())
    {
        for (actual, target) in path.times.iter().zip(numbers(time_values)) {
            close(*actual, target, 2e-15, "sim.fmsm vector event time");
        }
    }

    let corrected = model
        .simulate_paths_for_corrected(horizons.clone(), starts, seed)
        .unwrap();
    for (path, horizon) in corrected.paths.iter().zip(horizons) {
        let final_time = *path.times.last().unwrap();
        if model.graph().is_absorbing(*path.states.last().unwrap()) {
            assert!(final_time <= horizon);
        } else {
            close(final_time, horizon, 2e-15, "corrected vector horizon");
        }
        assert!(path.times.windows(2).all(|times| times[0] <= times[1]));
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn simulation_summaries_follow_flexsurv_and_conservation_rules() {
    let fixture = fixture();
    let (_, model) = exponential_models();
    let expected = &fixture["simulation"];
    let probabilities = model
        .simulated_transition_probabilities(
            vec![1.0, 5.0],
            expected["pmatrix_M"].as_u64().unwrap() as usize,
            expected["seed"].as_u64().unwrap() as u32,
        )
        .unwrap();
    for (index, actual) in probabilities.probabilities.iter().enumerate() {
        let oracle = matrix(&expected["pmatrix"][index]);
        for (cell, target) in actual.iter().zip(oracle) {
            close(*cell, target, 1e-12, "pmatrix.simfs");
        }
    }
    let length = model
        .simulated_length_of_stay(
            5.0,
            0,
            expected["totlos_M"].as_u64().unwrap() as usize,
            expected["seed"].as_u64().unwrap() as u32,
        )
        .unwrap();
    for (actual, target) in length.iter().zip(numbers(&expected["totlos"])) {
        close(*actual, target, 2e-15, "totlos.simfs");
    }
    close(
        length.iter().sum(),
        5.0,
        2e-15,
        "simulated time conservation",
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn semi_markov_public_helpers_match_flexsurv() {
    let fixture = fixture();
    let (_, model) = exponential_models();
    let expected = &fixture["simulation"];

    let simulation = model.simulate_paths(5.0, 0, 12, 24680).unwrap();
    let tidy = MultiStateModel::transitions_by_path(&simulation);
    let ids = numbers(&expected["tidy"]["id"]);
    let starts = numbers(&expected["tidy"]["start"]);
    let ends = numbers(&expected["tidy"]["end"]);
    let times = numbers(&expected["tidy"]["time"]);
    let delays = numbers(&expected["tidy"]["delay"]);
    assert_eq!(tidy.len(), ids.len());
    for (index, transition) in tidy.iter().enumerate() {
        assert_eq!(transition.path + 1, ids[index] as usize);
        assert_eq!(transition.from.index() + 1, starts[index] as usize);
        assert_eq!(transition.to.index() + 1, ends[index] as usize);
        close(transition.time, times[index], 2e-15, "simfs_bytrans time");
        close(
            transition.delay,
            delays[index],
            2e-15,
            "simfs_bytrans delay",
        );
    }

    let grouped = model
        .grouped_length_of_stay(5.0, 0, 2000, 24680, vec![1, 1, 2])
        .unwrap();
    assert_eq!(grouped.groups, vec![1, 2]);
    for (actual, target) in grouped
        .expected_time
        .iter()
        .zip(numbers(&expected["grouped"]["value"]))
    {
        close(*actual, target, 2e-15, "totlos.simfs grouped");
    }

    let vector = model
        .simulate_paths_for(vec![1.0, 2.0, 3.0], vec![0, 1, 2], 123)
        .unwrap();
    assert_eq!(vector.horizons, vec![1.0, 2.0, 3.0]);
    assert_eq!(vector.paths.len(), 3);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn semi_markov_nested_parameter_intervals_match_flexsurv() {
    let fixture = fixture();
    let expected = &fixture["simulation"];
    let model = fitted_exponential_reset_model();

    let oracle = &expected["pmatrix_ci"];
    let configuration = PredictionSimulationConfiguration::new(
        oracle["B"].as_u64().unwrap() as usize,
        number(&oracle["cl"]),
        oracle["seed"].as_u64().unwrap() as u32,
    )
    .unwrap();
    let actual = model
        .simulated_transition_probability_intervals(
            numbers(&oracle["times"]),
            oracle["M"].as_u64().unwrap() as usize,
            configuration,
        )
        .unwrap();
    for time in 0..2 {
        for (value, target) in actual.estimate.probabilities[time]
            .iter()
            .zip(matrix(&oracle["estimate"][time]))
        {
            close(*value, target, 2e-15, "pmatrix.simfs estimate");
        }
        for (value, target) in actual.lower[time]
            .iter()
            .zip(matrix(&oracle["lower"][time]))
        {
            close(*value, target, 2e-15, "pmatrix.simfs lower");
        }
        for (value, target) in actual.upper[time]
            .iter()
            .zip(matrix(&oracle["upper"][time]))
        {
            close(*value, target, 2e-15, "pmatrix.simfs upper");
        }
    }

    let oracle = &expected["totlos_ci"];
    let configuration = PredictionSimulationConfiguration::new(
        oracle["B"].as_u64().unwrap() as usize,
        number(&oracle["cl"]),
        oracle["seed"].as_u64().unwrap() as u32,
    )
    .unwrap();
    let actual = model
        .simulated_length_of_stay_intervals(
            5.0,
            0,
            oracle["M"].as_u64().unwrap() as usize,
            configuration,
        )
        .unwrap();
    for (actual, target) in actual.estimate.iter().zip(numbers(&oracle["estimate"])) {
        close(*actual, target, 2e-8, "totlos.simfs estimate");
    }
    for (actual, target) in actual.lower.iter().zip(numbers(&oracle["lower"])) {
        close(*actual, target, 2e-8, "totlos.simfs lower");
    }
    for (actual, target) in actual.upper.iter().zip(numbers(&oracle["upper"])) {
        close(*actual, target, 2e-8, "totlos.simfs upper");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn final_outcome_summary_matches_simfinal_fmsm() {
    let fixture = fixture();
    let expected = &fixture["simulation"]["final"];
    let (_, model) = exponential_models();
    let actual = model
        .final_outcomes(
            number(&expected["horizon"]),
            expected["M"].as_u64().unwrap() as usize,
            expected["seed"].as_u64().unwrap() as u32,
            numbers(&expected["probabilities"]),
        )
        .unwrap();
    assert_eq!(actual.summaries.len(), 1);
    let summary = &actual.summaries[0];
    let values = numbers(&expected["value"]);
    for (actual, target) in summary.quantiles.iter().zip(&values[..3]) {
        close(*actual, *target, 2e-15, "simfinal_fmsm quantile");
    }
    close(summary.mean_time, values[3], 2e-15, "simfinal_fmsm mean");
    close(
        summary.conditional_probability,
        values[4],
        2e-15,
        "simfinal_fmsm probability",
    );

    let oracle = &expected["ci"];
    let configuration = PredictionSimulationConfiguration::new(
        oracle["B"].as_u64().unwrap() as usize,
        number(&oracle["cl"]),
        expected["seed"].as_u64().unwrap() as u32,
    )
    .unwrap();
    let fitted = fitted_exponential_reset_model();
    let actual = fitted
        .final_outcome_intervals(
            number(&expected["horizon"]),
            oracle["M"].as_u64().unwrap() as usize,
            numbers(&expected["probabilities"]),
            configuration,
        )
        .unwrap();
    let summary = &actual.estimate.summaries[0];
    let interval = &actual.intervals[0];
    let estimate = numbers(&oracle["value"]);
    let lower = numbers(&oracle["lower"]);
    let upper = numbers(&oracle["upper"]);
    for index in 0..3 {
        close(
            summary.quantiles[index],
            estimate[index],
            3e-6,
            "simfinal_fmsm CI estimate quantile",
        );
        close(
            interval.quantile_lower[index],
            lower[index],
            3e-6,
            "simfinal_fmsm lower quantile",
        );
        close(
            interval.quantile_upper[index],
            upper[index],
            3e-6,
            "simfinal_fmsm upper quantile",
        );
    }
    close(
        interval.mean_time_lower,
        lower[3],
        3e-6,
        "simfinal_fmsm lower mean",
    );
    close(
        interval.mean_time_upper,
        upper[3],
        3e-6,
        "simfinal_fmsm upper mean",
    );
    close(
        interval.conditional_probability_lower,
        lower[4],
        3e-6,
        "simfinal_fmsm lower probability",
    );
    close(
        interval.conditional_probability_upper,
        upper[4],
        3e-6,
        "simfinal_fmsm upper probability",
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn competing_risks_final_probabilities_match_pfinal_fmsm() {
    let fixture = fixture();
    let expected = &fixture["simulation"]["pfinal"];

    let fixed = fixture["exponential"]["fits"]
        .as_array()
        .unwrap()
        .iter()
        .take(2)
        .map(|fit| {
            TransitionModel::fixed(
                FlexSurvDistribution::exponential(number(&fit["natural"])).unwrap(),
            )
        })
        .collect();
    let fixed = MultiStateModel::clock_forward(competing_graph(), fixed).unwrap();
    let fixed = CompetingRisksModel::new(fixed, 0).unwrap();
    let actual = fixed
        .final_state_probabilities(
            number(&expected["horizon"]),
            MarkovControl::matrix_exponential(),
        )
        .unwrap();
    for (actual, target) in actual.probabilities.iter().zip(numbers(&expected["value"])) {
        close(*actual, target, 2e-12, "pfinal_fmsm estimate");
    }

    let fitted = fitted_exponential_transitions()
        .into_iter()
        .take(2)
        .collect();
    let fitted = MultiStateModel::clock_forward(competing_graph(), fitted).unwrap();
    let fitted = CompetingRisksModel::new(fitted, 0).unwrap();
    let configuration = PredictionSimulationConfiguration::new(
        expected["B"].as_u64().unwrap() as usize,
        0.95,
        expected["seed"].as_u64().unwrap() as u32,
    )
    .unwrap();
    let actual = fitted
        .final_state_probability_intervals(
            number(&expected["horizon"]),
            MarkovControl::matrix_exponential(),
            configuration,
        )
        .unwrap();
    for (actual, target) in actual.lower.iter().zip(numbers(&expected["lower"])) {
        close(*actual, target, 3e-6, "pfinal_fmsm lower");
    }
    for (actual, target) in actual.upper.iter().zip(numbers(&expected["upper"])) {
        close(*actual, target, 3e-6, "pfinal_fmsm upper");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn predictable_time_covariates_match_form_basepars_tcovs() {
    let fixture = fixture();
    let expected = &fixture["predictable_covariate"];
    let model = fitted_predictable_covariate_model();
    let actual = model
        .simulate_paths(
            number(&expected["horizon"]),
            0,
            expected["M"].as_u64().unwrap() as usize,
            expected["seed"].as_u64().unwrap() as u32,
        )
        .unwrap();
    for (path, state_values) in actual
        .paths
        .iter()
        .zip(expected["paths_states"].as_array().unwrap())
    {
        assert_eq!(
            path.states
                .iter()
                .map(|state| state.index() + 1)
                .collect::<Vec<_>>(),
            numbers(state_values)
                .into_iter()
                .map(|state| state as usize)
                .collect::<Vec<_>>()
        );
    }
    for (path, time_values) in actual
        .paths
        .iter()
        .zip(expected["paths_times"].as_array().unwrap())
    {
        for (actual, target) in path.times.iter().zip(numbers(time_values)) {
            close(*actual, target, 3e-6, "predictable covariate event time");
        }
    }
}
