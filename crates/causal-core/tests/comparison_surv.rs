use hirmos_causal_core::survival::comparison_surv::{
    breslow_tarone_ware, crossing_times, fixed_point, lin_wang, lin_xu, long_term, short_term,
    weighted_kaplan_meier, ComparisonData, ComparisonTime, Event, Group, OverallConfiguration,
    RmstWindow, Side, SignificanceLevel, SurvivalSample, TwoStageConfiguration, Weight,
};
use serde_json::Value;
use std::num::NonZeroUsize;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/comparison_surv.json")).unwrap()
}

fn data(fixture: &Value) -> ComparisonData {
    let source = &fixture["data"];
    let samples = source["time"]
        .as_array()
        .unwrap()
        .iter()
        .zip(source["status"].as_array().unwrap())
        .zip(source["group"].as_array().unwrap())
        .map(|((time, status), group)| {
            SurvivalSample::new(
                time.as_f64().unwrap(),
                if status.as_i64().unwrap() == 1 {
                    Event::Observed
                } else {
                    Event::Censored
                },
                if group.as_i64().unwrap() == 0 {
                    Group::Zero
                } else {
                    Group::One
                },
            )
            .unwrap()
        })
        .collect();
    ComparisonData::new(samples).unwrap()
}

fn source_number(value: &Value) -> f64 {
    value
        .as_f64()
        .unwrap_or_else(|| value.as_str().unwrap().parse().unwrap())
}

fn close(actual: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs().max(1.0),
        "{label}: actual={actual:.17e}, expected={expected:.17e}"
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn weighted_overall_tests_match_comparison_surv() {
    let fixture = fixture();
    let data = data(&fixture);
    for (weight, name) in [
        (Weight::GehanWilcoxon, "gehan"),
        (Weight::TaroneWare, "tarone_ware"),
    ] {
        let actual = breslow_tarone_ware(&data, weight);
        let expected = &fixture[name][0];
        close(actual.statistic, source_number(&expected[2]), 8e-7, name);
        close(actual.p_value, source_number(&expected[3]), 8e-7, name);
    }

    let actual = weighted_kaplan_meier(&data).unwrap();
    let expected = &fixture["weighted_km"][0];
    close(actual.statistic, source_number(&expected[1]), 8e-7, "WKM");
    close(actual.p_value, source_number(&expected[2]), 8e-7, "WKM");

    let actual = lin_wang(&data);
    let expected = &fixture["squared_differences"][0];
    for (actual, index) in [
        (actual.expected_delta, 1),
        (actual.variance, 2),
        (actual.statistic, 3),
        (actual.p_value, 4),
    ] {
        close(actual, source_number(&expected[index]), 8e-7, "Lin-Wang");
    }

    let actual = lin_xu(&data, Side::TwoSided, 0.5).unwrap();
    let expected = &fixture["absolute_difference"][0];
    for (actual, index) in [
        (actual.delta, 1),
        (actual.expected_delta, 2),
        (actual.variance, 3),
        (actual.statistic, 6),
        (actual.p_value, 7),
    ] {
        close(actual, source_number(&expected[index]), 8e-7, "Lin-Xu");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn fixed_point_and_crossings_match_comparison_surv() {
    let fixture = fixture();
    let data = data(&fixture);
    let time = fixture["data"]["t0"].as_f64().unwrap();
    let actual = fixed_point(&data, time).unwrap();
    for (group, name) in [
        (&actual.group_zero, "est.g0"),
        (&actual.group_one, "est.g1"),
    ] {
        for (index, estimate) in group.iter().enumerate() {
            let expected = &fixture["fixed_point"][name][index];
            close(estimate.estimate, source_number(&expected[2]), 2e-7, name);
            close(estimate.lower, source_number(&expected[3]), 2e-7, name);
            close(estimate.upper, source_number(&expected[4]), 2e-7, name);
        }
    }
    for (index, test) in actual.tests.iter().enumerate() {
        let expected = &fixture["fixed_point"]["test"][index];
        close(
            test.statistic,
            source_number(&expected[1]),
            1e-12,
            "fixed statistic",
        );
        close(test.p_value, source_number(&expected[2]), 1e-12, "fixed p");
    }
    let actual_crossings = crossing_times(&data);
    let expected_crossings = fixture["crossings"].as_array().unwrap();
    assert_eq!(actual_crossings.len(), expected_crossings.len());
    for (actual, expected) in actual_crossings.iter().zip(expected_crossings) {
        close(*actual, expected.as_f64().unwrap(), 1e-14, "crossing");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn short_and_long_term_tests_match_comparison_surv() {
    let fixture = fixture();
    let data = data(&fixture);
    let time = fixture["data"]["t0"].as_f64().unwrap();

    let short = short_term(&data, time).unwrap();
    let expected = &fixture["short_term"];
    for (actual, row, column) in [
        (short.weighted_kaplan_meier_statistic, 0, 2),
        (short.weighted_kaplan_meier_p_value, 0, 3),
        (short.log_rank_statistic, 1, 2),
        (short.log_rank_p_value, 1, 3),
    ] {
        close(actual, source_number(&expected[row][column]), 6e-6, "Short");
    }

    let long = long_term(&data, time).unwrap();
    let expected = &fixture["long_term"];
    for (actual, row, column) in [
        (long.partial_log_rank_statistic, 0, 2),
        (long.partial_log_rank_p_value, 0, 3),
        (long.ordinary_least_squares_statistic, 1, 2),
        (long.ordinary_least_squares_p_value, 1, 3),
        (long.sample_path_statistic, 2, 2),
        (long.sample_path_p_value, 2, 3),
        (long.quadratic_statistic, 3, 2),
        (long.quadratic_p_value, 3, 3),
    ] {
        close(actual, source_number(&expected[row][column]), 6e-6, "Long");
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn descriptive_statistics_match_comparison_surv_and_survrm2() {
    let fixture = fixture();
    let result = hirmos_causal_core::survival::comparison_surv::describe(
        &data(&fixture),
        RmstWindow::Observed,
        SignificanceLevel::new(0.05).unwrap(),
    )
    .unwrap();
    let expected = &fixture["descriptive"];
    for (group, row) in [(&result.group_zero, 0), (&result.group_one, 1)] {
        let summary = &expected["result.summary"][row];
        assert_eq!(group.sample_size, summary[0].as_u64().unwrap() as usize);
        assert_eq!(group.events, summary[1].as_u64().unwrap() as usize);
        close(
            group.censoring_rate,
            summary[2].as_f64().unwrap(),
            1e-15,
            "censoring",
        );
        close(
            group.maximum_observed_time,
            summary[3].as_f64().unwrap(),
            1e-14,
            "max time",
        );
        close(
            group.maximum_event_time,
            summary[4].as_f64().unwrap(),
            1e-14,
            "max event",
        );

        let mean = &expected["result.mean"][row];
        for (actual, column) in [
            (group.mean.estimate, 0),
            (group.mean.standard_error, 1),
            (group.mean.lower, 2),
            (group.mean.upper, 3),
        ] {
            close(actual, mean[column].as_f64().unwrap(), 2e-12, "mean");
        }
        let rmst = &expected["result.RMST"][row];
        for (actual, column) in [
            (group.restricted_mean.estimate, 0),
            (group.restricted_mean.standard_error, 1),
            (group.restricted_mean.lower, 2),
            (group.restricted_mean.upper, 3),
        ] {
            close(actual, rmst[column].as_f64().unwrap(), 2e-12, "rmst");
        }
        let quantiles = &expected["result.quantile"][row];
        for (actual, column) in [
            (group.first_quartile.estimate, 0),
            (group.first_quartile.lower, 1),
            (group.first_quartile.upper, 2),
            (group.median.estimate, 3),
            (group.median.lower, 4),
            (group.median.upper, 5),
            (group.third_quartile.estimate, 6),
            (group.third_quartile.lower, 7),
            (group.third_quartile.upper, 8),
        ] {
            match (actual, quantiles[column].as_f64()) {
                (Some(actual), Some(expected)) => close(actual, expected, 1e-14, "quantile"),
                (None, None) => {}
                mismatch => panic!("quantile option mismatch: {mismatch:?}"),
            }
        }
    }
    close(
        result.truncation_time,
        expected["tau"].as_f64().unwrap(),
        1e-14,
        "tau",
    );

    let at_fixed_time = hirmos_causal_core::survival::comparison_surv::describe(
        &data(&fixture),
        RmstWindow::At(ComparisonTime::new(fixture["data"]["t0"].as_f64().unwrap()).unwrap()),
        SignificanceLevel::new(0.05).unwrap(),
    )
    .unwrap();
    let expected = &fixture["overall"];
    close(
        at_fixed_time
            .restricted_mean_comparison
            .group_one_minus_zero
            .estimate,
        source_number(&expected[8][1]),
        6e-6,
        "RMST difference",
    );
    close(
        at_fixed_time
            .restricted_mean_comparison
            .group_one_minus_zero
            .p_value,
        source_number(&expected[8][2]),
        6e-6,
        "RMST difference p",
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn two_stage_test_matches_tshrc() {
    let fixture = fixture();
    let result = hirmos_causal_core::survival::comparison_surv::two_stage(
        &data(&fixture),
        TwoStageConfiguration::new(NonZeroUsize::new(200).unwrap(), 0.05, 0.1, 12_345).unwrap(),
    )
    .unwrap();
    close(
        result.two_stage_p_value,
        source_number(&fixture["overall"][6][2]),
        6e-6,
        "two-stage",
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn aggregate_results_match_comparison_surv_overall_test() {
    let fixture = fixture();
    let result = hirmos_causal_core::survival::comparison_surv::overall_test(
        &data(&fixture),
        OverallConfiguration::new(
            fixture["data"]["t0"].as_f64().unwrap(),
            NonZeroUsize::new(200).unwrap(),
            12_345,
        )
        .unwrap(),
    )
    .unwrap();
    let expected = &fixture["overall"];
    for (actual, name) in [
        (result.proportional_hazards.coefficient, "coefficient"),
        (result.proportional_hazards.chi_square, "chi_square"),
        (result.proportional_hazards.p_value, "p_value"),
    ] {
        close(
            actual,
            fixture["proportional_hazards"][name].as_f64().unwrap(),
            3e-8,
            "proportional hazards",
        );
    }
    for (actual, row, column) in [
        (result.log_rank.statistic, 0, 1),
        (result.log_rank.p_value, 0, 2),
        (result.gehan_wilcoxon.statistic, 1, 1),
        (result.gehan_wilcoxon.p_value, 1, 2),
        (result.tarone_ware.statistic, 2, 1),
        (result.tarone_ware.p_value, 2, 2),
        (result.weighted_kaplan_meier.statistic, 3, 1),
        (result.weighted_kaplan_meier.p_value, 3, 2),
        (result.absolute_difference.statistic, 4, 1),
        (result.absolute_difference.p_value, 4, 2),
        (result.absolute_difference_permutation_p_value, 5, 2),
        (result.two_stage.two_stage_p_value, 6, 2),
        (result.squared_differences.statistic, 7, 1),
        (result.squared_differences.p_value, 7, 2),
        (result.restricted_mean_difference.estimate, 8, 1),
        (result.restricted_mean_difference.p_value, 8, 2),
    ] {
        close(
            actual,
            source_number(&expected[row][column]),
            6e-6,
            "overall",
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn smooth_hazard_plot_data_matches_muhaz() {
    let fixture = fixture();
    let result =
        hirmos_causal_core::survival::comparison_surv::smooth_hazard_curves(&data(&fixture));
    for (actual, name) in [
        (&result.group_zero, "group_zero"),
        (&result.group_one, "group_one"),
    ] {
        let expected = &fixture["smooth_hazard"][name];
        for (index, point) in actual.points.iter().enumerate() {
            close(
                point.time,
                expected["time"][index].as_f64().unwrap(),
                2e-14,
                "hazard grid",
            );
            close(
                point.hazard,
                expected["hazard"][index].as_f64().unwrap(),
                2e-14,
                "hazard estimate",
            );
        }
    }
}
