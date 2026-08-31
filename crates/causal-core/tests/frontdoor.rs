use hirmos_causal_core::nprandom::Mt19937;
use hirmos_causal_core::{
    frontdoor_two_stage, frontdoor_two_stage_with_progress, identify_frontdoor_set, Dag,
    FrontdoorBootstrap, FrontdoorError, FrontdoorInput, FrontdoorOptions,
};
use nalgebra::DMatrix;
use serde_json::Value;

fn close(label: &str, got: f64, want: f64, tolerance: f64) {
    assert!(
        (got - want).abs() <= tolerance,
        "{label}: got {got:.16e}, oracle {want:.16e}, deviation {:.3e}",
        (got - want).abs(),
    );
}

fn ness_columns() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let csv = include_str!("../oracle/fixtures/online_game_example_do_why.csv");
    let mut lines = csv.lines();
    let headers: Vec<&str> = lines.next().expect("CSV header").split(',').collect();
    let treatment_column = headers
        .iter()
        .position(|name| *name == "Side-quest Engagement")
        .unwrap();
    let mediator_column = headers
        .iter()
        .position(|name| *name == "Won Items")
        .unwrap();
    let outcome_column = headers
        .iter()
        .position(|name| *name == "In-game Purchases")
        .unwrap();

    let mut treatment = Vec::new();
    let mut mediator = Vec::new();
    let mut outcome = Vec::new();
    for line in lines.filter(|line| !line.is_empty()) {
        let values: Vec<&str> = line.split(',').collect();
        treatment.push(values[treatment_column].parse().unwrap());
        mediator.push(values[mediator_column].parse().unwrap());
        outcome.push(values[outcome_column].parse().unwrap());
    }
    (treatment, mediator, outcome)
}

#[test]
fn frontdoor_identifier_matches_the_chapter_and_upstream_graphs() {
    // DoWhy's upstream X <- U -> Y, X -> M -> Y example.
    let upstream = Dag::new(4, &[(3, 0), (3, 2), (0, 1), (1, 2)]);
    assert_eq!(
        identify_frontdoor_set(&upstream, 0, 2, &[3]).unwrap(),
        Some(vec![1])
    );

    // Ness chapter 11. Node 8 is Won Items, the front-door mediator selected by DoWhy.
    let chapter = Dag::new(
        10,
        &[
            (0, 1),
            (0, 2),
            (2, 1),
            (3, 4),
            (3, 5),
            (1, 4),
            (1, 5),
            (2, 4),
            (2, 5),
            (6, 4),
            (7, 4),
            (4, 8),
            (8, 5),
            (8, 9),
            (5, 9),
        ],
    );
    assert_eq!(
        identify_frontdoor_set(&chapter, 4, 5, &[]).unwrap(),
        Some(vec![8])
    );

    // A direct treatment-to-outcome arrow is not intercepted by the proposed mediator.
    let invalid = Dag::new(4, &[(3, 0), (3, 2), (0, 1), (1, 2), (0, 2)]);
    assert_eq!(identify_frontdoor_set(&invalid, 0, 2, &[3]).unwrap(), None);
}

#[test]
fn ness_chapter_11_matches_dowhy_point_and_bootstrap_path() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/frontdoor_dowhy.json")).unwrap();
    let expected = &fixture["estimate"];
    let (treatment, mediator, outcome) = ness_columns();
    let second_adjustment = DMatrix::from_column_slice(treatment.len(), 1, &treatment);
    let input = FrontdoorInput {
        treatment: &treatment,
        mediator: &mediator,
        outcome: &outcome,
        first_stage_adjustment: None,
        second_stage_adjustment: Some(&second_adjustment),
    };
    let options = FrontdoorOptions {
        bootstrap: Some(FrontdoorBootstrap::default()),
        ..FrontdoorOptions::default()
    };

    let mut progress = Vec::new();
    let got = frontdoor_two_stage_with_progress(input, options, |completed, total| {
        progress.push((completed, total));
    })
    .unwrap();

    let expected_first: Vec<f64> =
        serde_json::from_value(expected["first_stage_params"].clone()).unwrap();
    let expected_second: Vec<f64> =
        serde_json::from_value(expected["second_stage_params"].clone()).unwrap();
    let mut max_parameter_deviation = 0.0_f64;
    for (index, (&got, &want)) in got
        .first_stage_params
        .iter()
        .zip(&expected_first)
        .enumerate()
    {
        max_parameter_deviation = max_parameter_deviation.max((got - want).abs());
        close(&format!("first-stage parameter {index}"), got, want, 2e-10);
    }
    for (index, (&got, &want)) in got
        .second_stage_params
        .iter()
        .zip(&expected_second)
        .enumerate()
    {
        max_parameter_deviation = max_parameter_deviation.max((got - want).abs());
        close(&format!("second-stage parameter {index}"), got, want, 2e-9);
    }
    close("ATE", got.ate, expected["ate"].as_f64().unwrap(), 2e-9);

    let expected_bootstrap: Vec<f64> =
        serde_json::from_value(expected["bootstrap_estimates"].clone()).unwrap();
    assert_eq!(got.bootstrap_estimates.len(), expected_bootstrap.len());
    let mut max_bootstrap_deviation = 0.0_f64;
    for (index, (&got, &want)) in got
        .bootstrap_estimates
        .iter()
        .zip(&expected_bootstrap)
        .enumerate()
    {
        max_bootstrap_deviation = max_bootstrap_deviation.max((got - want).abs());
        close(&format!("bootstrap estimate {index}"), got, want, 3e-8);
    }
    let expected_interval: Vec<f64> =
        serde_json::from_value(expected["confidence_interval"].clone()).unwrap();
    let interval = got.confidence_interval.unwrap();
    close("interval lower", interval[0], expected_interval[0], 3e-8);
    close("interval upper", interval[1], expected_interval[1], 3e-8);

    assert_eq!(progress.first(), Some(&(0, 399)));
    assert_eq!(progress.last(), Some(&(399, 399)));
    assert_eq!(progress.len(), 400);
    println!(
        "Ness/DoWhy: params max dev={max_parameter_deviation:.3e}, ATE dev={:.3e}, bootstrap max dev={max_bootstrap_deviation:.3e}, CI max dev={:.3e}",
        (got.ate - expected["ate"].as_f64().unwrap()).abs(),
        (interval[0] - expected_interval[0])
            .abs()
            .max((interval[1] - expected_interval[1]).abs()),
    );
}

#[test]
fn upstream_dowhy_frontdoor_regression_case_matches_exactly() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/frontdoor_dowhy.json")).unwrap();
    let expected = &fixture["upstream_test"];
    let n = expected["rows"].as_u64().unwrap() as usize;
    let mut rng = Mt19937::seeded(expected["seed"].as_u64().unwrap() as u32);
    let mut gaussian_cache = None;
    let latent: Vec<f64> = (0..n)
        .map(|_| rng.standard_normal(&mut gaussian_cache))
        .collect();
    let treatment: Vec<f64> = latent
        .iter()
        .map(|latent| rng.standard_normal(&mut gaussian_cache) + 0.3 * latent)
        .collect();
    let mediator: Vec<f64> = treatment
        .iter()
        .map(|treatment| 0.7 * treatment + 0.3 * rng.standard_normal(&mut gaussian_cache))
        .collect();
    let outcome: Vec<f64> = mediator
        .iter()
        .zip(&latent)
        .map(|(mediator, latent)| 0.65 * mediator + 0.2 * latent)
        .collect();
    let second_adjustment = DMatrix::from_column_slice(n, 1, &treatment);
    let got = frontdoor_two_stage(
        FrontdoorInput {
            treatment: &treatment,
            mediator: &mediator,
            outcome: &outcome,
            first_stage_adjustment: None,
            second_stage_adjustment: Some(&second_adjustment),
        },
        FrontdoorOptions::default(),
    )
    .unwrap();

    let expected_first: Vec<f64> =
        serde_json::from_value(expected["first_stage_params"].clone()).unwrap();
    let expected_second: Vec<f64> =
        serde_json::from_value(expected["second_stage_params"].clone()).unwrap();
    for (index, (&got, &want)) in got
        .first_stage_params
        .iter()
        .zip(&expected_first)
        .enumerate()
    {
        close(&format!("upstream first-stage {index}"), got, want, 2e-12);
    }
    for (index, (&got, &want)) in got
        .second_stage_params
        .iter()
        .zip(&expected_second)
        .enumerate()
    {
        close(&format!("upstream second-stage {index}"), got, want, 2e-12);
    }
    close(
        "upstream ATE",
        got.ate,
        expected["ate"].as_f64().unwrap(),
        2e-12,
    );
}

#[test]
fn adjusted_two_stage_model_recovers_the_structural_product() {
    let treatment: Vec<f64> = (0..40).map(|row| (row % 5) as f64 - 2.0).collect();
    let z1: Vec<f64> = (0..40).map(|row| ((row * 7) % 11) as f64 - 5.0).collect();
    let z2: Vec<f64> = (0..40)
        .map(|row| ((row * 3 + 1) % 13) as f64 - 6.0)
        .collect();
    let mediator: Vec<f64> = treatment
        .iter()
        .zip(&z1)
        .map(|(treatment, z1)| 4.0 + 2.0 * treatment + 0.5 * z1)
        .collect();
    let outcome: Vec<f64> = mediator
        .iter()
        .zip(&treatment)
        .zip(&z2)
        .map(|((mediator, treatment), z2)| 8.0 + 3.0 * mediator + 5.0 * treatment - z2)
        .collect();
    let first_adjustment = DMatrix::from_column_slice(treatment.len(), 1, &z1);
    let second_adjustment = DMatrix::from_fn(treatment.len(), 2, |row, column| {
        if column == 0 {
            treatment[row]
        } else {
            z2[row]
        }
    });

    let got = frontdoor_two_stage(
        FrontdoorInput {
            treatment: &treatment,
            mediator: &mediator,
            outcome: &outcome,
            first_stage_adjustment: Some(&first_adjustment),
            second_stage_adjustment: Some(&second_adjustment),
        },
        FrontdoorOptions::default(),
    )
    .unwrap();
    close("first-stage effect", got.first_stage_effect, 2.0, 1e-12);
    close("second-stage effect", got.second_stage_effect, 3.0, 1e-12);
    close("product", got.ate, 6.0, 1e-12);
    assert!(got.confidence_interval.is_none());
    assert!(got.bootstrap_estimates.is_empty());
}

#[test]
fn invalid_inputs_are_reported_at_the_boundary() {
    let treatment = [0.0, 1.0, 0.0];
    let mediator = [0.0, 2.0];
    let outcome = [1.0, 2.0, 3.0];
    let error = frontdoor_two_stage(
        FrontdoorInput {
            treatment: &treatment,
            mediator: &mediator,
            outcome: &outcome,
            first_stage_adjustment: None,
            second_stage_adjustment: None,
        },
        FrontdoorOptions::default(),
    )
    .unwrap_err();
    assert_eq!(
        error,
        FrontdoorError::LengthMismatch {
            field: "mediator",
            expected: 3,
            actual: 2,
        }
    );

    let mediator = [0.0, 2.0, f64::NAN];
    let error = frontdoor_two_stage(
        FrontdoorInput {
            treatment: &treatment,
            mediator: &mediator,
            outcome: &outcome,
            first_stage_adjustment: None,
            second_stage_adjustment: None,
        },
        FrontdoorOptions::default(),
    )
    .unwrap_err();
    assert_eq!(
        error,
        FrontdoorError::NonFiniteValue {
            field: "mediator",
            row: 2,
        }
    );
}
