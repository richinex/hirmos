use hirmos_causal_core::negbin_nuts::{irr_summary, quantile, PbcNegBinModel};
use hirmos_causal_core::nuts::NutsOptions;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/nuts_114i.json")).unwrap()
}

fn numbers(value: &Value) -> Vec<f64> {
    serde_json::from_value(value.clone()).unwrap()
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn std(values: &[f64]) -> f64 {
    let average = mean(values);
    (values
        .iter()
        .map(|value| (value - average).powi(2))
        .sum::<f64>()
        / (values.len() - 1) as f64)
        .sqrt()
}

fn model(root: &Value) -> PbcNegBinModel {
    PbcNegBinModel::from_raw(
        &numbers(&root["treatment"]),
        &numbers(&root["confounder"]),
        &numbers(&root["outcome"]),
    )
}

fn posterior_summary(model: &PbcNegBinModel, samples: &[Vec<f64>]) -> [f64; 13] {
    let beta_0: Vec<f64> = samples.iter().map(|draw| draw[0]).collect();
    let beta_treatment: Vec<f64> = samples.iter().map(|draw| draw[1]).collect();
    let beta_confounder: Vec<f64> = samples.iter().map(|draw| draw[2]).collect();
    let r: Vec<f64> = samples.iter().map(|draw| draw[3].exp()).collect();
    let irr: Vec<f64> = samples
        .iter()
        .map(|draw| (draw[1] / model.treatment_std).exp())
        .collect();
    let irr_interval = irr_summary(&irr);
    [
        mean(&beta_0),
        std(&beta_0),
        mean(&beta_treatment),
        std(&beta_treatment),
        mean(&beta_confounder),
        std(&beta_confounder),
        mean(&r),
        std(&r),
        mean(&irr),
        std(&irr),
        irr_interval.lower,
        irr_interval.median,
        irr_interval.upper,
    ]
}

fn pyro_average(root: &Value) -> [f64; 13] {
    let pyro = root["pyro_chains"].as_array().unwrap();
    let average = |path: &[&str]| {
        pyro.iter()
            .map(|chain| {
                path.iter()
                    .fold(chain, |value, key| &value[*key])
                    .as_f64()
                    .unwrap()
            })
            .sum::<f64>()
            / pyro.len() as f64
    };
    [
        average(&["beta_0", "mean"]),
        average(&["beta_0", "std"]),
        average(&["beta_treatment", "mean"]),
        average(&["beta_treatment", "std"]),
        average(&["beta_confounder", "mean"]),
        average(&["beta_confounder", "std"]),
        average(&["r", "mean"]),
        average(&["r", "std"]),
        average(&["irr", "mean"]),
        average(&["irr", "std"]),
        average(&["irr", "q025"]),
        average(&["irr", "median"]),
        average(&["irr", "q975"]),
    ]
}

fn retained_draw_maxdev(samples: &[Vec<f64>], oracle: &Value, seed: u64) -> f64 {
    let pyro_chain = oracle["pyro_chains"]
        .as_array()
        .unwrap()
        .iter()
        .find(|chain| chain["seed"].as_u64() == Some(seed))
        .expect("matching Pyro seed");
    let pyro_first: Vec<Vec<f64>> =
        serde_json::from_value(pyro_chain["first_draws"].clone()).unwrap();
    samples
        .iter()
        .take(3)
        .zip(&pyro_first)
        .flat_map(|(rust, pyro)| {
            [rust[0], rust[1], rust[2], rust[3].exp()]
                .into_iter()
                .zip(pyro)
                .map(|(actual, expected)| (actual - expected).abs())
                .collect::<Vec<_>>()
        })
        .fold(0.0_f64, f64::max)
}

#[test]
fn gamma_poisson_density_and_gradient_match_pyro_float64() {
    let root = fixture();
    let mut maximum_per_observation_log_prob_deviation: f64 = 0.0;
    let mut maximum_gradient_deviation: f64 = 0.0;

    for (name, case) in [("shaped", &root), ("Seatbelts", &root["seatbelts"])] {
        let model = model(case);
        let mut case_log_prob_deviation: f64 = 0.0;
        let mut case_gradient_deviation: f64 = 0.0;
        for point in case["fixed_float64"].as_array().unwrap() {
            let parameters = numbers(&point["parameters"]);
            let expected_gradient = numbers(&point["gradient"]);
            let (actual_log_prob, actual_gradient) = model.log_prob_grad(&parameters);
            case_log_prob_deviation = case_log_prob_deviation
                .max((actual_log_prob - point["log_prob"].as_f64().unwrap()).abs());
            for (actual, expected) in actual_gradient.iter().zip(&expected_gradient) {
                case_gradient_deviation = case_gradient_deviation.max((actual - expected).abs());
            }
        }
        let per_observation = case_log_prob_deviation / model.outcome.len() as f64;
        println!(
            "{name} fixed density: logp maxdev {case_log_prob_deviation:.3e} ({per_observation:.3e}/observation), gradient maxdev {case_gradient_deviation:.3e}"
        );
        maximum_per_observation_log_prob_deviation =
            maximum_per_observation_log_prob_deviation.max(per_observation);
        maximum_gradient_deviation = maximum_gradient_deviation.max(case_gradient_deviation);
    }

    assert!(maximum_per_observation_log_prob_deviation < 4e-14);
    assert!(maximum_gradient_deviation < 1e-12);
}

#[test]
fn seatbelts_real_time_series_posterior_matches_pyro() {
    let root = fixture();
    let case = &root["seatbelts"];
    assert_eq!(numbers(&case["outcome"]).len(), 192);
    let model = model(case);
    let mut rust_summaries = Vec::new();

    for seed in [41, 42, 43] {
        let result = model.sample(NutsOptions::default(), seed);
        let draw_deviation = retained_draw_maxdev(&result.samples, case, seed);
        let irr = irr_summary(&model.irr_draws(&result));
        println!(
            "Seatbelts Rust seed {seed}: IRR {:.6} [{:.6}, {:.6}], step {:.4}, accept {:.3}, divergences {}, first-draw maxdev vs Torch {draw_deviation:.3e}",
            irr.median,
            irr.lower,
            irr.upper,
            result.step_size,
            result.mean_accept_probability,
            result.divergences
        );
        assert_eq!(result.divergences, 0);
        assert!(result.mean_accept_probability > 0.65);
        assert!(draw_deviation > 1e-3);
        rust_summaries.push(posterior_summary(&model, &result.samples));
    }

    let expected = pyro_average(case);
    let rust_average: Vec<f64> = (0..expected.len())
        .map(|index| {
            mean(
                &rust_summaries
                    .iter()
                    .map(|row| row[index])
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let tolerances = [
        0.005, 0.005, 0.005, 0.005, 0.005, 0.005, 0.30, 0.30, 0.01, 0.01, 0.03, 0.01, 0.03,
    ];
    for index in 0..expected.len() {
        println!(
            "Seatbelts statistic {index}: Rust {:.6}, Pyro {:.6}, deviation {:.3e}",
            rust_average[index],
            expected[index],
            (rust_average[index] - expected[index]).abs()
        );
        assert!(
            (rust_average[index] - expected[index]).abs() < tolerances[index],
            "Seatbelts posterior statistic {index}: Rust {}, Pyro {}, tolerance {}",
            rust_average[index],
            expected[index],
            tolerances[index]
        );
    }
}

#[test]
fn multinomial_nuts_posterior_matches_pyro_across_seeds() {
    let root = fixture();
    let model = model(&root);
    let options = NutsOptions::default();
    let mut rust_summaries = Vec::new();

    for seed in [41, 42, 43] {
        let result = model.sample(options, seed);
        let first_draw_maxdev = retained_draw_maxdev(&result.samples, &root, seed);
        let irr = model.irr_draws(&result);
        let irr_interval = irr_summary(&irr);
        let summary = posterior_summary(&model, &result.samples);
        println!(
            "Rust seed {seed}: IRR {:.6} [{:.6}, {:.6}], step {:.4}, accept {:.3}, divergences {}, first-draw maxdev vs Torch {first_draw_maxdev:.3e}",
            irr_interval.median,
            irr_interval.lower,
            irr_interval.upper,
            result.step_size,
            result.mean_accept_probability,
            result.divergences
        );
        assert_eq!(result.divergences, 0);
        assert!(result.mean_accept_probability > 0.65);
        assert!(
            first_draw_maxdev > 1e-3,
            "Torch and Rust draws are not claimed to be identical"
        );
        rust_summaries.push(summary);
    }

    let expected = pyro_average(&root);
    let rust_average: Vec<f64> = (0..expected.len())
        .map(|index| {
            mean(
                &rust_summaries
                    .iter()
                    .map(|row| row[index])
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let tolerances = [
        0.015, 0.012, 0.015, 0.012, 0.015, 0.012, 0.40, 0.35, 0.05, 0.04, 0.10, 0.05, 0.12,
    ];
    let names = [
        "beta_0 mean",
        "beta_0 std",
        "beta_treatment mean",
        "beta_treatment std",
        "beta_confounder mean",
        "beta_confounder std",
        "r mean",
        "r std",
        "IRR mean",
        "IRR std",
        "IRR q025",
        "IRR median",
        "IRR q975",
    ];
    for index in 0..expected.len() {
        println!(
            "{}: Rust {:.6}, Pyro {:.6}, deviation {:.3e}",
            names[index],
            rust_average[index],
            expected[index],
            (rust_average[index] - expected[index]).abs()
        );
        assert!(
            (rust_average[index] - expected[index]).abs() < tolerances[index],
            "posterior statistic {index}: Rust {}, Pyro {}, tolerance {}",
            rust_average[index],
            expected[index],
            tolerances[index]
        );
    }

    // Exercise the public quantile independently of the IRR helper.
    assert_eq!(quantile(&[0.0, 10.0], 0.25), 2.5);
}
