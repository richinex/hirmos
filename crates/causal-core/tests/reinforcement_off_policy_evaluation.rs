use hirmos_causal_core::numpy_reduce::numpy_mean;
use hirmos_causal_core::reinforcement::off_policy_evaluation::{
    bootstrap_interval, bootstrap_intervals, logged_rounds, paired_differences, policy_value, round_rewards, weight_diagnostics, Estimator,
};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_off_policy_evaluation.json")).unwrap()
}

fn numbers(value: &Value) -> Vec<f64> {
    value.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()
}

fn estimator(case: &Value) -> Estimator {
    match case["estimator"].as_str().unwrap() {
        "snipw" => Estimator::SelfNormalized,
        _ => Estimator::InverseProbability { max_weight: case["max_weight"].as_f64().unwrap_or(f64::INFINITY) },
    }
}

#[test]
fn policy_values_and_bootstrap_intervals_match_the_reference_bit_for_bit() {
    let fixture = fixture();
    let rounds = fixture["rounds"].as_u64().unwrap() as usize;
    let seed = fixture["seed"].as_u64().unwrap() as u32;
    for case in fixture["cases"].as_array().unwrap() {
        let log = logged_rounds(&numbers(&case["reward"]), &numbers(&case["propensity"]), &numbers(&case["target"])).unwrap();
        for expected in case["estimates"].as_array().unwrap() {
            let estimator = estimator(expected);
            let label = format!("{} {}", case["label"], expected["estimator"]);
            assert_eq!(policy_value(&log, estimator).unwrap(), expected["value"].as_f64().unwrap(), "{label}");
            let samples = round_rewards(&log, estimator).unwrap();
            let differences = paired_differences(&log, estimator).unwrap();
            let paired = &expected["paired"];
            assert_eq!(numpy_mean(&differences), paired["difference"].as_f64().unwrap(), "{label} paired difference");
            let found = bootstrap_interval(&differences, 0.05, rounds, seed).unwrap();
            assert_eq!(found.mean, paired["mean"].as_f64().unwrap(), "{label} paired mean");
            assert_eq!(found.lower, paired["lower"].as_f64().unwrap(), "{label} paired lower");
            assert_eq!(found.upper, paired["upper"].as_f64().unwrap(), "{label} paired upper");
            for (alpha, interval) in expected["intervals"].as_object().unwrap() {
                let found = bootstrap_interval(&samples, alpha.parse().unwrap(), rounds, seed).unwrap();
                assert_eq!(found.mean, interval["mean"].as_f64().unwrap(), "{label} alpha {alpha} mean");
                assert_eq!(found.lower, interval["lower"].as_f64().unwrap(), "{label} alpha {alpha} lower");
                assert_eq!(found.upper, interval["upper"].as_f64().unwrap(), "{label} alpha {alpha} upper");
            }
        }
    }
}

#[test]
fn a_propensity_outside_the_unit_interval_is_refused() {
    let log = logged_rounds(&[1.0], &[0.0], &[0.5]).unwrap();
    assert!(policy_value(&log, Estimator::SelfNormalized).is_err());
}

#[test]
fn weight_diagnostics_match_cobalts_effective_sample_size() {
    let cases: Value = serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_weight_diagnostics.json")).unwrap();
    for (index, case) in cases.as_array().unwrap().iter().enumerate() {
        let propensities = numbers(&case["propensity"]);
        let rewards = vec![0.0; propensities.len()];
        let log = logged_rounds(&rewards, &propensities, &numbers(&case["target"])).unwrap();
        let found = weight_diagnostics(&log).unwrap();
        let expected = case["effective_sample_size"].as_f64().unwrap();
        // R's sum accumulates in extended precision, so the last bits can differ from a 64-bit sum.
        assert!((found.effective_sample_size - expected).abs() <= 1e-12 * expected, "case {index}: {} against {expected}", found.effective_sample_size);
        assert_eq!(found.max_weight, case["max_weight"].as_f64().unwrap(), "case {index}");
        assert_eq!(found.support, case["support"].as_f64().unwrap(), "case {index}");
    }
}

#[test]
fn a_rule_the_log_never_supports_has_no_effective_rows() {
    let log = logged_rounds(&[1.0, 0.0], &[0.5, 0.5], &[0.0, 0.0]).unwrap();
    let found = weight_diagnostics(&log).unwrap();
    assert_eq!((found.support, found.max_weight, found.effective_sample_size), (0.0, 0.0, 0.0));
}

#[test]
fn shared_draws_give_each_vector_its_own_bootstrap_bit_for_bit() {
    let fixture = fixture();
    let rounds = fixture["rounds"].as_u64().unwrap() as usize;
    let seed = fixture["seed"].as_u64().unwrap() as u32;
    for case in fixture["cases"].as_array().unwrap() {
        let log = logged_rounds(&numbers(&case["reward"]), &numbers(&case["propensity"]), &numbers(&case["target"])).unwrap();
        let vectors: Vec<Vec<f64>> = case["estimates"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|expected| [round_rewards(&log, estimator(expected)).unwrap(), paired_differences(&log, estimator(expected)).unwrap()])
            .collect();
        let shared = bootstrap_intervals(&vectors.iter().map(Vec::as_slice).collect::<Vec<_>>(), 0.05, rounds, seed, |_| {}).unwrap();
        for (vector, found) in vectors.iter().zip(shared) {
            assert_eq!(found, bootstrap_interval(vector, 0.05, rounds, seed).unwrap());
        }
    }
}
