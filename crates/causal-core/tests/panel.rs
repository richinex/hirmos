//! Parity for long-panel preparation and the DID / SC / SDID estimators.
//!
//! The fixture is emitted directly by `synth-inference/synthdid` at pinned
//! commit 70c1ce3eac58e28c30b67435ca377bb48baa9b8a, using its California
//! Proposition 99 panel and default estimator options.

use hirmos_causal_core::panel::*;
use nalgebra::DMatrix;
use proptest::prelude::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/panel_synthdid.json")).unwrap()
}

fn numbers(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item.as_f64().unwrap())
        .collect()
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item.as_str().unwrap().to_owned())
        .collect()
}

fn matrix(value: &Value) -> DMatrix<f64> {
    let rows = value.as_array().unwrap();
    DMatrix::from_fn(rows.len(), rows[0].as_array().unwrap().len(), |i, j| {
        rows[i][j].as_f64().unwrap()
    })
}

fn observations(root: &Value) -> Vec<PanelObservation> {
    let units = strings(&root["input"]["unit"]);
    let times = numbers(&root["input"]["time"]);
    let outcomes = numbers(&root["input"]["outcome"]);
    let treatments = numbers(&root["input"]["treatment"]);
    units
        .into_iter()
        .zip(times)
        .zip(outcomes)
        .zip(treatments)
        .map(|(((unit, time), outcome), treatment)| PanelObservation {
            unit,
            time: time as i64,
            outcome,
            treatment,
        })
        .collect()
}

fn max_deviation(actual: &[f64], expected: &[f64]) -> f64 {
    actual
        .iter()
        .zip(expected)
        .map(|(left, right)| (left - right).abs())
        .fold(0.0, f64::max)
}

fn check_estimate(name: &str, actual: &PanelEstimate, expected: &Value, tolerance: f64) {
    let expected_lambda = numbers(&expected["lambda"]);
    let expected_omega = numbers(&expected["omega"]);
    let expected_curve = numbers(&expected["effect_curve"]);
    let estimate_deviation = (actual.estimate - expected["estimate"].as_f64().unwrap()).abs();
    let lambda_deviation = max_deviation(&actual.lambda, &expected_lambda);
    let omega_deviation = max_deviation(&actual.omega, &expected_omega);
    let curve_deviation = max_deviation(&actual.effect_curve, &expected_curve);
    let lambda_objective = numbers(&expected["lambda_values"]);
    let omega_objective = numbers(&expected["omega_values"]);
    let lambda_objective_deviation = max_deviation(&actual.lambda_objective, &lambda_objective);
    let omega_objective_deviation = max_deviation(&actual.omega_objective, &omega_objective);
    println!(
        "{name}: estimate dev {estimate_deviation:.3e}, lambda {lambda_deviation:.3e}, omega {omega_deviation:.3e}, curve {curve_deviation:.3e}; iterations lambda {}, omega {}",
        actual.lambda_iterations, actual.omega_iterations
    );
    assert!(
        estimate_deviation <= tolerance,
        "{name} estimate deviation {estimate_deviation}"
    );
    assert!(
        lambda_deviation <= tolerance,
        "{name} lambda deviation {lambda_deviation}"
    );
    assert!(
        omega_deviation <= tolerance,
        "{name} omega deviation {omega_deviation}"
    );
    assert!(
        curve_deviation <= tolerance,
        "{name} curve deviation {curve_deviation}"
    );
    assert_eq!(actual.lambda_objective.len(), lambda_objective.len());
    assert_eq!(actual.omega_objective.len(), omega_objective.len());
    assert!(
        lambda_objective_deviation <= tolerance,
        "{name} lambda objective deviation {lambda_objective_deviation}"
    );
    assert!(
        omega_objective_deviation <= tolerance,
        "{name} omega objective deviation {omega_objective_deviation}"
    );
    let omega_sum = actual.omega.iter().sum::<f64>();
    assert!(
        (omega_sum - 1.0).abs() <= 1e-12,
        "{name} omega sum {omega_sum}"
    );
    assert!(actual.omega.iter().all(|weight| *weight >= -1e-15));
}

#[test]
fn california_long_panel_matches_panel_matrices_cell_for_cell() {
    let root = fixture();
    assert_eq!(
        root["reference_commit"].as_str().unwrap(),
        "70c1ce3eac58e28c30b67435ca377bb48baa9b8a"
    );
    let expected = &root["matrix"];
    let rows = observations(&root);
    let panel = panel_matrices(&rows).unwrap();
    assert_eq!(panel.units, strings(&expected["units"]));
    assert_eq!(
        panel.times,
        numbers(&expected["times"])
            .into_iter()
            .map(|value| value as i64)
            .collect::<Vec<_>>()
    );
    assert_eq!(panel.n0, expected["n0"].as_u64().unwrap() as usize);
    assert_eq!(panel.t0, expected["t0"].as_u64().unwrap() as usize);
    assert_eq!(panel.y, matrix(&expected["y"]));
    assert_eq!(panel.w, matrix(&expected["w"]));

    // `panel.matrices` is row-order invariant.
    let mut reversed = rows;
    reversed.reverse();
    let shuffled = panel_matrices(&reversed).unwrap();
    assert_eq!(shuffled.units, panel.units);
    assert_eq!(shuffled.times, panel.times);
    assert_eq!(shuffled.y, panel.y);
    assert_eq!(shuffled.w, panel.w);
}

#[test]
fn california_did_sc_and_sdid_match_the_official_r_package() {
    let root = fixture();
    let panel = panel_matrices(&observations(&root)).unwrap();
    let did = did_estimate(&panel.y, panel.n0, panel.t0).unwrap();
    let sc = synthdid_sc_estimate(&panel.y, panel.n0, panel.t0).unwrap();
    let sdid = synthetic_did_estimate(&panel.y, panel.n0, panel.t0).unwrap();
    let expected_noise = root["matrix"]["noise_level"].as_f64().unwrap();
    for estimate in [&did, &sc, &sdid] {
        assert!((estimate.noise_level - expected_noise).abs() <= 2e-14);
    }
    check_estimate("DID", &did, &root["did"], 2e-12);
    check_estimate("SC", &sc, &root["sc"], 2e-10);
    check_estimate("SDID", &sdid, &root["sdid"], 2e-10);
    assert_eq!(
        sdid.lambda_iterations,
        numbers(&root["sdid"]["lambda_values"]).len()
    );
    assert_eq!(
        sdid.omega_iterations,
        numbers(&root["sdid"]["omega_values"]).len()
    );
}

#[test]
fn california_placebo_refits_match_synthdid_algorithm_four() {
    let root = fixture();
    let panel = panel_matrices(&observations(&root)).unwrap();
    let inference = &root["inference"];
    for (name, kind, expected) in [
        (
            "SC",
            PanelEstimatorKind::SyntheticControl,
            &inference["sc_placebo"],
        ),
        (
            "SDID",
            PanelEstimatorKind::SyntheticDifferenceInDifferences,
            &inference["sdid_placebo"],
        ),
    ] {
        let permutations: Vec<Vec<usize>> = expected["indices"]
            .as_array()
            .unwrap()
            .iter()
            .map(|permutation| {
                permutation
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|index| index.as_u64().unwrap() as usize - 1)
                    .collect()
            })
            .collect();
        let actual =
            panel_placebo_standard_error(&panel.y, panel.n0, panel.t0, kind, &permutations)
                .unwrap();
        let expected_estimates = numbers(&expected["estimates"]);
        let draw_deviation = max_deviation(&actual.estimates, &expected_estimates);
        let se_deviation =
            (actual.standard_error - expected["standard_error"].as_f64().unwrap()).abs();
        println!(
            "{name} placebo: {} refits, draw maxdev {draw_deviation:.3e}, SE dev {se_deviation:.3e}",
            actual.estimates.len()
        );
        assert!(draw_deviation <= 3e-9, "{name}: {draw_deviation}");
        assert!(se_deviation <= 3e-9, "{name}: {se_deviation}");
    }
}

#[test]
fn multiple_treated_units_match_synthdid_placebo_refits() {
    let root = fixture();
    let mut rows = observations(&root);
    for row in &mut rows {
        if row.unit == "Kansas" && row.time >= 1989 {
            row.treatment = 1.0;
        }
    }
    let panel = panel_matrices(&rows).unwrap();
    let expected = &root["inference"]["multi_sdid_placebo"];
    let permutations: Vec<Vec<usize>> = expected["indices"]
        .as_array()
        .unwrap()
        .iter()
        .map(|permutation| {
            permutation
                .as_array()
                .unwrap()
                .iter()
                .map(|index| index.as_u64().unwrap() as usize - 1)
                .collect()
        })
        .collect();
    let actual = panel_placebo_standard_error(
        &panel.y,
        panel.n0,
        panel.t0,
        PanelEstimatorKind::SyntheticDifferenceInDifferences,
        &permutations,
    )
    .unwrap();
    let draw_deviation = max_deviation(&actual.estimates, &numbers(&expected["estimates"]));
    let se_deviation = (actual.standard_error - expected["standard_error"].as_f64().unwrap()).abs();
    println!(
        "multi-treated SDID placebo: draw maxdev {draw_deviation:.3e}, SE dev {se_deviation:.3e}"
    );
    assert!(draw_deviation <= 3e-9);
    assert!(se_deviation <= 3e-9);
}

#[test]
fn california_in_time_placebos_match_the_reference_refits() {
    let root = fixture();
    let panel = panel_matrices(&observations(&root)).unwrap();
    let inference = &root["inference"];
    let sc = panel_in_time_placebo(
        &panel.y,
        panel.n0,
        panel.t0,
        PanelEstimatorKind::SyntheticControl,
        None,
    )
    .unwrap();
    let sdid = panel_in_time_placebo(
        &panel.y,
        panel.n0,
        panel.t0,
        PanelEstimatorKind::SyntheticDifferenceInDifferences,
        None,
    )
    .unwrap();
    check_estimate("SC in-time placebo", &sc, &inference["sc_in_time"], 3e-9);
    check_estimate(
        "SDID in-time placebo",
        &sdid,
        &inference["sdid_in_time"],
        3e-9,
    );
    assert!(inference["sc_in_time_upstream_error"]
        .as_str()
        .unwrap()
        .contains("omega.intercept"));
}

#[test]
fn placebo_inference_rejects_invalid_sampling_boundaries() {
    let y = DMatrix::from_fn(3, 4, |row, column| row as f64 + column as f64);
    assert_eq!(
        panel_placebo_standard_error(
            &y,
            2,
            2,
            PanelEstimatorKind::DifferenceInDifferences,
            &[vec![0, 1]],
        )
        .unwrap_err(),
        PanelError::InsufficientPlaceboReplications
    );
    assert_eq!(
        panel_placebo_standard_error(
            &y,
            2,
            2,
            PanelEstimatorKind::DifferenceInDifferences,
            &[vec![0, 0], vec![0, 1]],
        )
        .unwrap_err(),
        PanelError::InvalidPlaceboPermutation { replication: 0 }
    );
    assert_eq!(
        panel_in_time_placebo(
            &y,
            2,
            2,
            PanelEstimatorKind::DifferenceInDifferences,
            Some(1.0),
        )
        .unwrap_err(),
        PanelError::InvalidTreatedFraction
    );
}

#[test]
fn multiple_treated_units_use_the_same_block_average_as_r() {
    let root = fixture();
    let mut rows = observations(&root);
    for row in &mut rows {
        if row.unit == "Kansas" && row.time >= 1989 {
            row.treatment = 1.0;
        }
    }
    let panel = panel_matrices(&rows).unwrap();
    let expected = &root["multi_treated"];
    assert_eq!(panel.units, strings(&expected["units"]));
    assert_eq!(
        &panel.units[panel.units.len() - 2..],
        ["California", "Kansas"]
    );
    assert_eq!(panel.n0, expected["n0"].as_u64().unwrap() as usize);
    assert_eq!(panel.t0, expected["t0"].as_u64().unwrap() as usize);
    check_estimate(
        "multi-treated DID",
        &did_estimate(&panel.y, panel.n0, panel.t0).unwrap(),
        &expected["did"],
        2e-12,
    );
    check_estimate(
        "multi-treated SC",
        &synthdid_sc_estimate(&panel.y, panel.n0, panel.t0).unwrap(),
        &expected["sc"],
        2e-10,
    );
    check_estimate(
        "multi-treated SDID",
        &synthetic_did_estimate(&panel.y, panel.n0, panel.t0).unwrap(),
        &expected["sdid"],
        2e-10,
    );
}

#[test]
fn california_panel_rejects_each_structural_failure() {
    let root = fixture();
    let rows = observations(&root);

    let mut duplicate = rows.clone();
    duplicate.push(rows[0].clone());
    assert!(matches!(
        panel_matrices(&duplicate),
        Err(PanelError::DuplicateCell { .. })
    ));

    assert!(matches!(
        panel_matrices(&rows[1..]),
        Err(PanelError::MissingCell { .. })
    ));

    let mut non_binary = rows.clone();
    non_binary[0].treatment = 0.5;
    assert!(matches!(
        panel_matrices(&non_binary),
        Err(PanelError::TreatmentNotBinary { .. })
    ));

    let mut no_variation = rows.clone();
    for row in &mut no_variation {
        row.treatment = 0.0;
    }
    assert_eq!(
        panel_matrices(&no_variation).unwrap_err(),
        PanelError::NoTreatmentVariation
    );

    let mut non_simultaneous = rows;
    for row in &mut non_simultaneous {
        if row.unit == "Alabama" && row.time >= 1988 {
            row.treatment = 1.0;
        }
    }
    assert_eq!(
        panel_matrices(&non_simultaneous).unwrap_err(),
        PanelError::NonSimultaneousAdoption
    );

    let mut no_pre_period = observations(&root);
    for row in &mut no_pre_period {
        if row.unit == "California" {
            row.treatment = 1.0;
        }
    }
    assert_eq!(
        panel_matrices(&no_pre_period).unwrap_err(),
        PanelError::NoPreTreatmentPeriod
    );

    let mut non_finite = observations(&root);
    non_finite[10].outcome = f64::NAN;
    assert!(matches!(
        panel_matrices(&non_finite),
        Err(PanelError::NonFiniteOutcome { row: 10 })
    ));
}

#[test]
fn did_and_sdid_obey_fixed_effect_and_scale_invariances() {
    let root = fixture();
    let panel = panel_matrices(&observations(&root)).unwrap();
    let baseline_did = did_estimate(&panel.y, panel.n0, panel.t0).unwrap();
    let baseline_sdid = synthetic_did_estimate(&panel.y, panel.n0, panel.t0).unwrap();

    let column_shifted = DMatrix::from_fn(panel.y.nrows(), panel.y.ncols(), |i, t| {
        panel.y[(i, t)] + 0.25 * t as f64
    });
    let row_shifted = DMatrix::from_fn(panel.y.nrows(), panel.y.ncols(), |i, t| {
        panel.y[(i, t)] + 0.5 * i as f64
    });
    for (label, shifted) in [("column", column_shifted), ("row", row_shifted)] {
        let did = did_estimate(&shifted, panel.n0, panel.t0).unwrap();
        let sdid = synthetic_did_estimate(&shifted, panel.n0, panel.t0).unwrap();
        assert!(
            (did.estimate - baseline_did.estimate).abs() <= 2e-12,
            "{label} DID"
        );
        assert!(
            (sdid.estimate - baseline_sdid.estimate).abs() <= 2e-9,
            "{label} SDID"
        );
        assert!(
            max_deviation(&sdid.lambda, &baseline_sdid.lambda) <= 2e-10,
            "{label} lambda"
        );
        assert!(
            max_deviation(&sdid.omega, &baseline_sdid.omega) <= 2e-10,
            "{label} omega"
        );
    }

    for scale in [1e-6, 1e6] {
        let scaled = &panel.y * scale;
        let sdid = synthetic_did_estimate(&scaled, panel.n0, panel.t0).unwrap();
        assert!((sdid.estimate - baseline_sdid.estimate * scale).abs() <= 2e-8 * scale.max(1.0));
        assert!(max_deviation(&sdid.lambda, &baseline_sdid.lambda) <= 2e-9);
        assert!(max_deviation(&sdid.omega, &baseline_sdid.omega) <= 2e-9);
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 256,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn arbitrary_balanced_parallel_trend_panels_recover_the_planted_did(
        n0 in 1usize..6,
        n1 in 1usize..4,
        t0 in 1usize..7,
        t1 in 1usize..5,
        effect in -20.0f64..20.0,
        reverse in any::<bool>(),
    ) {
        let mut rows = Vec::new();
        for unit_index in 0..(n0 + n1) {
            let treated = unit_index >= n0;
            let unit = if treated {
                format!("treated-{unit_index:02}")
            } else {
                format!("control-{unit_index:02}")
            };
            for time_index in 0..(t0 + t1) {
                let post = time_index >= t0;
                // Unit fixed effects plus a shared nonlinear time path satisfy
                // parallel trends exactly without making the panel constant.
                let outcome = 0.7 * unit_index as f64
                    + 0.2 * (time_index * time_index) as f64
                    - 0.1 * time_index as f64
                    + if treated && post { effect } else { 0.0 };
                rows.push(PanelObservation {
                    unit: unit.clone(),
                    time: 2000 + time_index as i64,
                    outcome,
                    treatment: f64::from(treated && post),
                });
            }
        }
        if reverse {
            rows.reverse();
        } else {
            let offset = rows.len() / 3;
            rows.rotate_left(offset);
        }
        let panel = panel_matrices(&rows).unwrap();
        prop_assert_eq!(panel.n0, n0);
        prop_assert_eq!(panel.t0, t0);
        let did = did_estimate(&panel.y, panel.n0, panel.t0).unwrap();
        prop_assert!((did.estimate - effect).abs() <= 2e-12);
        prop_assert!(did.effect_curve.iter().all(|value| (*value - effect).abs() <= 2e-12));

        rows.pop();
        let missing_cell = matches!(panel_matrices(&rows), Err(PanelError::MissingCell { .. }));
        prop_assert!(missing_cell);
    }
}
