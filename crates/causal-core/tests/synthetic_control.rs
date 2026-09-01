// Parity for the synthetic control weights of 134. The reference is cvxpy with CLARABEL; the
// SCS answer 134 actually uses is carried alongside so its own gap is visible.
use hirmos_causal_core::synthetic_control::*;
use nalgebra::{DMatrix, DVector};
use serde_json::Value;

fn matrix(v: &Value) -> DMatrix<f64> {
    let rows: Vec<Vec<f64>> = serde_json::from_value(v.clone()).unwrap();
    DMatrix::from_fn(rows.len(), rows[0].len(), |r, c| rows[r][c])
}

fn vector(v: &Value) -> DVector<f64> {
    let raw: Vec<f64> = serde_json::from_value(v.clone()).unwrap();
    DVector::from_row_slice(&raw)
}

#[test]
fn synthetic_control_matches() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/synthetic_control.json")).unwrap();
    for case in root["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (sc, effect) = synthetic_effect(
            &matrix(&case["y_pre_co"]),
            &vector(&case["y_pre_tr"]),
            &matrix(&case["y_post_co"]),
            &vector(&case["y_post_tr"]),
        );
        let want_w: Vec<f64> = serde_json::from_value(case["weights"].clone()).unwrap();
        let wdev = sc
            .weights
            .iter()
            .zip(&want_w)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f64, f64::max);
        // The loss is compared relatively: outside the convex hull it is large, so a weight
        // agreeing to 1e-9 still moves it by 1e-7 in absolute terms.
        let ldev = (sc.loss / case["loss_recomputed"].as_f64().unwrap() - 1.0).abs();
        let solver_ldev = (sc.loss / case["loss"].as_f64().unwrap() - 1.0).abs();
        let adev = (effect.att - case["att"].as_f64().unwrap()).abs();

        // The constraints must hold exactly, not merely to the solver's tolerance.
        let total: f64 = sc.weights.iter().sum();
        assert!(
            (total - 1.0).abs() <= 1e-12,
            "{name}: weights sum to {total}"
        );
        assert!(
            sc.weights.iter().all(|&v| v >= 0.0),
            "{name}: negative weight"
        );

        println!(
            "{name}: {} iterations, weights maxdev {wdev:.3e}, relative loss dev {ldev:.3e} \
             (against the solver's own objective {solver_ldev:.3e}), ATT dev {adev:.3e}",
            sc.iterations
        );
        println!(
            "    SCS's own gap from the same reference was {:.3e}, at a constraint violation of {:.3e}",
            case["scs_weight_gap"].as_f64().unwrap(),
            case["scs_violation"].as_f64().unwrap()
        );
        assert!(wdev <= 1e-7, "{name}: weight deviation {wdev}");
        assert!(ldev <= 1e-8, "{name}: relative loss deviation {ldev}");
        assert!(
            solver_ldev <= 1e-8,
            "{name}: solver objective deviation {solver_ldev}"
        );
        assert!(adev <= 1e-7, "{name}: ATT deviation {adev}");

        let want_pre: Vec<f64> = serde_json::from_value(case["pre_gap"].clone()).unwrap();
        let want_post: Vec<f64> = serde_json::from_value(case["post_gap"].clone()).unwrap();
        let gdev = effect
            .pre_gap
            .iter()
            .chain(&effect.post_gap)
            .zip(want_pre.iter().chain(&want_post))
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f64, f64::max);
        println!("    pre and post gaps maxdev {gdev:.3e}");
        assert!(gdev <= 1e-6, "{name}: gap deviation {gdev}");
    }
}

#[test]
fn no_feasible_point_beats_the_active_set_optimum() {
    // The active set method returns the exact optimum, so no feasible point may do better.
    // SCS can report a lower loss, but only by stopping short of feasibility, so its weights
    // are projected back onto the simplex before being compared.
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/synthetic_control.json")).unwrap();
    for case in root["cases"].as_array().unwrap() {
        let x = matrix(&case["y_pre_co"]);
        let y = vector(&case["y_pre_tr"]);
        let sc = fit_synthetic_control(&x, &y);

        let loss_of = |w: &[f64]| -> f64 {
            let r = &x * DVector::from_row_slice(w) - &y;
            r.iter().map(|v| v * v).sum()
        };
        let mut projected: Vec<f64> = serde_json::from_value(case["scs_weights"].clone()).unwrap();
        let raw_loss = loss_of(&projected);
        for v in projected.iter_mut() {
            *v = v.max(0.0);
        }
        let total: f64 = projected.iter().sum();
        for v in projected.iter_mut() {
            *v /= total;
        }
        println!(
            "{}: ours {:.10}, SCS as returned {:.10} (violation {:.3e}), SCS projected {:.10}",
            case["name"].as_str().unwrap(),
            sc.loss,
            raw_loss,
            case["scs_violation"].as_f64().unwrap(),
            loss_of(&projected)
        );
        assert!(
            sc.loss <= loss_of(&projected) + 1e-12,
            "a feasible point did better"
        );
        let reference: Vec<f64> = serde_json::from_value(case["weights"].clone()).unwrap();
        assert!(
            sc.loss <= loss_of(&reference) + 1e-12,
            "the reference point did better"
        );
    }
}

#[test]
fn debiased_cross_fit_matches_134_fold_by_fold() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/synthetic_control.json")).unwrap();
    for expected in root["debiased_cases"].as_array().unwrap() {
        let name = expected["name"].as_str().unwrap();
        let source = root["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["name"] == expected["name"])
            .unwrap();
        let actual = debiased_synthetic_control(
            &matrix(&source["y_pre_co"]),
            &vector(&source["y_pre_tr"]),
            &matrix(&source["y_post_co"]),
            &vector(&source["y_post_tr"]),
            expected["fold_count"].as_u64().unwrap() as usize,
        )
        .unwrap();

        assert_eq!(
            actual.block_size,
            expected["block_size"].as_u64().unwrap() as usize
        );
        assert_eq!(
            actual.degrees_of_freedom,
            expected["degrees_of_freedom"].as_u64().unwrap() as usize
        );
        assert_eq!(
            actual.folds.len(),
            expected["folds"].as_array().unwrap().len()
        );
        let mut maximum_fold_deviation = 0.0f64;
        for (fold, wanted) in actual
            .folds
            .iter()
            .zip(expected["folds"].as_array().unwrap())
        {
            let held_out: Vec<usize> = serde_json::from_value(wanted["held_out"].clone()).unwrap();
            let weights: Vec<f64> = serde_json::from_value(wanted["weights"].clone()).unwrap();
            assert_eq!(fold.held_out, held_out);
            maximum_fold_deviation = maximum_fold_deviation
                .max((fold.bias - wanted["bias"].as_f64().unwrap()).abs())
                .max((fold.att - wanted["att"].as_f64().unwrap()).abs())
                .max(
                    fold.weights
                        .iter()
                        .zip(weights)
                        .map(|(left, right)| (left - right).abs())
                        .fold(0.0, f64::max),
                );
        }
        let summary_deviation = [
            (actual.att - expected["att"].as_f64().unwrap()).abs(),
            (actual.standard_error - expected["standard_error"].as_f64().unwrap()).abs(),
            (actual.t_statistic - expected["t_statistic"].as_f64().unwrap()).abs(),
            (actual.p_value - expected["p_value"].as_f64().unwrap()).abs(),
            (actual.confidence_interval.0 - expected["confidence_interval"][0].as_f64().unwrap())
                .abs(),
            (actual.confidence_interval.1 - expected["confidence_interval"][1].as_f64().unwrap())
                .abs(),
        ]
        .into_iter()
        .fold(0.0, f64::max);
        println!(
            "debiased {name}: fold maxdev {maximum_fold_deviation:.3e}, summary maxdev {summary_deviation:.3e}"
        );
        println!(
            "    ATT {:.12}/{:.12}, SE {:.12}/{:.12}, t {:.12}/{:.12}, p {:.12}/{:.12}, CI {:?}/{:?}",
            actual.att,
            expected["att"].as_f64().unwrap(),
            actual.standard_error,
            expected["standard_error"].as_f64().unwrap(),
            actual.t_statistic,
            expected["t_statistic"].as_f64().unwrap(),
            actual.p_value,
            expected["p_value"].as_f64().unwrap(),
            actual.confidence_interval,
            (
                expected["confidence_interval"][0].as_f64().unwrap(),
                expected["confidence_interval"][1].as_f64().unwrap()
            )
        );
        assert!(
            maximum_fold_deviation <= 2e-7,
            "{name}: {maximum_fold_deviation}"
        );
        assert!(summary_deviation <= 1e-6, "{name}: {summary_deviation}");
    }
}

#[test]
fn debiased_cross_fit_rejects_invalid_boundaries() {
    let x = DMatrix::from_element(6, 2, 1.0);
    let y = DVector::from_element(6, 1.0);
    let post_x = DMatrix::from_element(2, 2, 1.0);
    let post_y = DVector::from_element(2, 1.0);
    assert_eq!(
        debiased_synthetic_control(&x, &y, &post_x, &post_y, 1).unwrap_err(),
        SyntheticControlError::InvalidFoldCount
    );
    let short_x = x.rows(0, 5).into_owned();
    let short_y = y.rows(0, 5).into_owned();
    assert_eq!(
        debiased_synthetic_control(&short_x, &short_y, &post_x, &post_y, 3).unwrap_err(),
        SyntheticControlError::InsufficientFoldData
    );
}

#[test]
fn california_donor_placebos_match_synth_semantics_and_clarabel() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/synthetic_control.json")).unwrap();
    let expected = &root["donor_placebo"];
    let actual = donor_placebo_mspe_inference(
        &matrix(&expected["matching_controls"]),
        &vector(&expected["matching_treated"]),
        &matrix(&expected["outcome_controls"]),
        &vector(&expected["outcome_treated"]),
        expected["pre_periods"].as_u64().unwrap() as usize,
    )
    .unwrap();

    let compare = |name: &str, got: &SyntheticControlMspeSummary, want: &Value| {
        let weights: Vec<f64> = serde_json::from_value(want["weights"].clone()).unwrap();
        let gap: Vec<f64> = serde_json::from_value(want["gap"].clone()).unwrap();
        let weight_deviation = got
            .weights
            .iter()
            .zip(weights)
            .map(|(left, right)| (left - right).abs())
            .fold(0.0, f64::max);
        let gap_deviation = got
            .gap
            .iter()
            .zip(gap)
            .map(|(left, right)| (left - right).abs())
            .fold(0.0, f64::max);
        let summary_deviation = [
            (got.pre_mspe - want["pre_mspe"].as_f64().unwrap()).abs(),
            (got.post_mspe - want["post_mspe"].as_f64().unwrap()).abs(),
            (got.mspe_ratio - want["mspe_ratio"].as_f64().unwrap()).abs(),
        ]
        .into_iter()
        .fold(0.0, f64::max);
        let weight_sum = got.weights.iter().sum::<f64>();
        assert!(
            (weight_sum - 1.0).abs() <= 2e-12,
            "{name}: sum {weight_sum}"
        );
        assert!(
            got.weights.iter().all(|weight| *weight >= 0.0),
            "{name}: negative weight"
        );
        assert!(
            weight_deviation <= 5e-7,
            "{name}: weights {weight_deviation}"
        );
        assert!(gap_deviation <= 1e-5, "{name}: gaps {gap_deviation}");
        assert!(
            summary_deviation <= 1e-4,
            "{name}: summary {summary_deviation}"
        );
        (weight_deviation, gap_deviation, summary_deviation)
    };

    let mut maximum = compare("California", &actual.treated, &expected["treated"]);
    for (placebo, wanted) in actual
        .placebos
        .iter()
        .zip(expected["placebos"].as_array().unwrap())
    {
        assert_eq!(placebo.donor, wanted["donor"].as_u64().unwrap() as usize);
        let deviations = compare(wanted["name"].as_str().unwrap(), &placebo.summary, wanted);
        maximum.0 = maximum.0.max(deviations.0);
        maximum.1 = maximum.1.max(deviations.1);
        maximum.2 = maximum.2.max(deviations.2);
    }
    assert_eq!(
        actual.n_valid_placebos,
        expected["n_valid_placebos"].as_u64().unwrap() as usize
    );
    assert!((actual.p_value - expected["p_value"].as_f64().unwrap()).abs() <= f64::EPSILON);
    println!(
        "California donor placebos: weights {:.3e}, gaps {:.3e}, MSPE summaries {:.3e}; p={:.8}",
        maximum.0, maximum.1, maximum.2, actual.p_value
    );
}

#[test]
fn donor_placebos_reject_invalid_designs() {
    let matching = DMatrix::from_element(3, 1, 1.0);
    let target = DVector::from_element(3, 1.0);
    let outcomes = DMatrix::from_element(5, 1, 1.0);
    let treated = DVector::from_element(5, 1.0);
    assert_eq!(
        donor_placebo_mspe_inference(&matching, &target, &outcomes, &treated, 3).unwrap_err(),
        SyntheticControlError::InsufficientDonors
    );
}

#[test]
fn california_prediction_bands_match_synth_formulas() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/synthetic_control.json")).unwrap();
    let fixture = &root["donor_placebo"];
    let expected = &fixture["prediction_bands"];
    let controls = matrix(&fixture["outcome_controls"]);
    let treated = vector(&fixture["outcome_treated"]);
    let weights: Vec<f64> = serde_json::from_value(fixture["treated"]["weights"].clone()).unwrap();
    let pre_periods = fixture["pre_periods"].as_u64().unwrap() as usize;
    let alpha = expected["alpha"].as_f64().unwrap();

    for (name, method, interval_key, half_width) in [
        (
            "conformal",
            SyntheticControlBandMethod::Conformal,
            "conformal_intervals",
            expected["conformal_q"].as_f64().unwrap(),
        ),
        (
            "parametric",
            SyntheticControlBandMethod::ParametricGaussian,
            "parametric_intervals",
            expected["parametric_half_width"].as_f64().unwrap(),
        ),
    ] {
        let actual = synthetic_control_prediction_band(
            &controls,
            &treated,
            &weights,
            pre_periods,
            alpha,
            method,
        )
        .unwrap();
        let wanted: Vec<Vec<f64>> = serde_json::from_value(expected[interval_key].clone()).unwrap();
        let interval_deviation = actual
            .intervals
            .iter()
            .zip(wanted)
            .flat_map(|(&(lower, upper), expected)| {
                [(lower - expected[0]).abs(), (upper - expected[1]).abs()]
            })
            .fold(0.0, f64::max);
        let half_width_deviation = (actual.half_width - half_width).abs();
        println!(
            "California {name} band: half-width dev {half_width_deviation:.3e}, interval maxdev {interval_deviation:.3e}"
        );
        assert!(half_width_deviation <= 2e-12);
        assert!(interval_deviation <= 2e-5);
    }
}
