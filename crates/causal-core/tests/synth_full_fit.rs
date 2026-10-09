use hirmos_causal_core::synthetic_control::synth::{
    PredictorOptimizer, PredictorSelection, SynthMatrices,
};
use nalgebra::{DMatrix, DVector};
use serde_json::Value;

fn exact(v: &Value) -> Vec<f64> {
    let bytes: Vec<u8> = serde_json::from_value(v.clone()).unwrap();
    bytes
        .chunks_exact(8)
        .map(|b| f64::from_le_bytes(b.try_into().unwrap()))
        .collect()
}
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/synth-full-fit.json")).unwrap()
}
fn matrices(case: &Value, raw: bool) -> SynthMatrices {
    let p = case["p"].as_u64().unwrap() as usize;
    let n = case["n"].as_u64().unwrap() as usize;
    let t = case["t"].as_u64().unwrap() as usize;
    let e = &case["exact"];
    let z0 = DMatrix::from_column_slice(t, n, &exact(&e["z0"]));
    let z1 = DVector::from_vec(exact(&e["z1"]));
    if raw {
        SynthMatrices::prepare(
            &DMatrix::from_column_slice(p, n, &exact(&e["x0"])),
            &DVector::from_vec(exact(&e["x1"])),
            &z0,
            &z1,
        )
        .unwrap()
    } else {
        SynthMatrices {
            scaled_donors: DMatrix::from_column_slice(p, n, &exact(&e["sx0"])),
            scaled_treated: DVector::from_vec(exact(&e["sx1"])),
            predictor_scales: vec![1.; p],
            pre_donors: z0,
            pre_treated: z1,
        }
    }
}

fn check_fits(raw: bool) {
    let oracle = fixture();
    let mut failures = Vec::new();
    for case in oracle["cases"].as_array().unwrap() {
        let data = matrices(case, raw);
        let custom = case["custom"]
            .as_array()
            .map(|a| a.iter().map(|v| v.as_f64().unwrap()).collect::<Vec<_>>());
        let result = match custom {
            Some(v) => data.fit_custom(&v),
            None => data.fit_default(),
        }
        .unwrap();
        let reference = case["outcome_mspe"].as_f64().unwrap();
        println!(
            "{} raw={raw}: loss {} against {reference}",
            case["name"], result.fit.outcome_mspe
        );
        if (result.fit.outcome_mspe - reference).abs() > 1e-10 * reference.abs().max(1.) {
            failures.push(format!(
                "{}: outcome loss {} != {reference}",
                case["name"], result.fit.outcome_mspe
            ));
        }
        for (label, values) in [
            ("v", &result.fit.predictor_weights),
            ("w", &result.fit.donor_weights),
        ] {
            let expected: Vec<f64> = serde_json::from_value(case[label].clone()).unwrap();
            assert_eq!(values.len(), expected.len());
            let error = values
                .iter()
                .zip(&expected)
                .map(|(a, b)| (a - b).abs())
                .fold(0., f64::max);
            println!("  {label} max error {error}");
            if error > 1e-10 {
                failures.push(format!("{}: {label} max error {error}", case["name"]));
            }
        }
        let expected_loss = case["predictor_loss"].as_f64().unwrap();
        if (result.fit.predictor_loss - expected_loss).abs() > 1e-10 * expected_loss.abs().max(1.) {
            failures.push(format!("{}: predictor loss", case["name"]));
        }
        match result.selection {
            PredictorSelection::Optimized(run) => {
                let expected = case["selected"]["method"].as_str().unwrap();
                assert_eq!(
                    run.candidate.method,
                    match expected {
                        "BFGS" => PredictorOptimizer::Bfgs,
                        "Nelder-Mead" => PredictorOptimizer::NelderMead,
                        _ => panic!("unexpected oracle method"),
                    }
                );
            }
            PredictorSelection::SinglePredictor => assert_eq!(case["p"], 1),
            PredictorSelection::SuppliedWeights => assert!(!case["custom"].is_null()),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
#[test]
fn default_selection_and_final_refit_match_r() {
    check_fits(false)
}
#[test]
fn raw_matrices_through_complete_fit_match_r() {
    check_fits(true)
}

#[test]
fn collinear_predictors_skip_regression_start() {
    let oracle = fixture();
    let case = oracle["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["name"] == "collinear_predictors")
        .unwrap();
    assert!(matrices(case, false).regression_start().unwrap().is_none());
}

#[test]
fn single_predictor_bypasses_custom_weight_validation_like_synth() {
    let oracle = fixture();
    let case = oracle["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["name"] == "single_predictor")
        .unwrap();
    let result = matrices(case, false).fit_custom(&[f64::NAN, 2.]).unwrap();
    assert!(matches!(
        result.selection,
        PredictorSelection::SinglePredictor
    ));
    assert_eq!(result.fit.predictor_weights, vec![1.]);
}
