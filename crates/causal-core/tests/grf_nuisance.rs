use hirmos_causal_core::grf::{
    causal,
    forest::{Options, SeedMode, Variance},
    sampling::Clusters,
    tree::{self, Honesty},
};
use serde_json::Value;
fn number(v: &Value) -> f64 {
    v.as_str()
        .map(|v| v.parse().unwrap())
        .or_else(|| v.as_f64())
        .unwrap()
}
fn floats(v: &Value) -> Vec<f64> {
    v.as_array().unwrap().iter().map(number).collect()
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn default_nuisance_and_causal_fits_match_r() {
    let f: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/nuisance.json")).unwrap();
    for c in f["cases"].as_array().unwrap() {
        let x: Vec<_> = c["X"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                v.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| if v.is_null() { f64::NAN } else { number(v) })
                    .collect()
            })
            .collect();
        let y = floats(&c["Y"]);
        let w = floats(&c["W"]);
        let weights = if c["weights"].is_null() {
            None
        } else {
            Some(floats(&c["weights"]))
        };
        let labels: Vec<_> = c["labels"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as usize)
            .collect();
        let clusters = Clusters::new(&labels, c["per_cluster"].as_u64().unwrap() as usize).unwrap();
        let options = Options {
            trees: 100,
            group_size: 2,
            sample_fraction: 0.5,
            seed: c["seed"].as_u64().unwrap() as u32,
            seed_mode: SeedMode::Indexed,
            batches: 1,
            tree: tree::Options {
                mtry: 4,
                min_node_size: 5,
                honesty: if c["honest"].as_bool().unwrap() {
                    Honesty::Enabled {
                        fraction: 0.5,
                        prune: c["prune"].as_bool().unwrap(),
                    }
                } else {
                    Honesty::Disabled
                },
                alpha: 0.05,
                imbalance_penalty: 0.0,
            },
        };
        let model = causal::fit_with_nuisance_pruning(
            &x,
            &y,
            &w,
            weights.as_deref(),
            &clusters,
            &options,
            None,
            None,
            c["stabilize"].as_bool().unwrap(),
            c["prune"].as_bool().unwrap(),
        )
        .unwrap();
        let tau: Vec<_> = model
            .forest
            .predict(&x, true, false)
            .unwrap()
            .iter()
            .map(|p| p.effect.unwrap())
            .collect();
        let debiasing = if w.iter().all(|v| *v == 0.0 || *v == 1.0) {
            None
        } else {
            Some(
                causal::continuous_debiasing(
                    &x,
                    &w,
                    &model.treatment_hat,
                    weights.as_deref(),
                    &clusters,
                    options.seed,
                    1,
                    SeedMode::Indexed,
                    500,
                )
                .unwrap(),
            )
        };
        let data = hirmos_causal_core::grf::inference::Observations {
            y: &y,
            w: &w,
            y_hat: &model.outcome_hat,
            w_hat: &model.treatment_hat,
            tau: &tau,
        };
        let inference_weights = hirmos_causal_core::grf::inference::observation_weights(
            &labels,
            weights.as_deref(),
            c["equalize"].as_bool().unwrap(),
        )
        .unwrap();
        let ate = hirmos_causal_core::grf::inference::aipw(
            &data,
            &inference_weights,
            &labels,
            hirmos_causal_core::grf::inference::Target::All,
            debiasing.as_deref(),
        )
        .unwrap();
        let expected = c["ate"].as_array().unwrap();
        assert!((ate.estimate - number(&expected[0])).abs() < 2e-8);
        assert!((ate.standard_error - number(&expected[1])).abs() < 2e-8);
        for (label, a, b) in [
            ("Yhat", &model.outcome_hat, floats(&c["Yhat"])),
            ("What", &model.treatment_hat, floats(&c["What"])),
        ] {
            for (i, (a, b)) in a.iter().zip(b).enumerate() {
                assert!(
                    (a - b).abs() < 2e-10 * b.abs().max(1.0),
                    "case {} {label} row {i}: {a} != {b}",
                    c["case"]
                );
            }
        }
        for (i, (a, b)) in model
            .forest
            .predict(&x, true, true)
            .unwrap()
            .iter()
            .zip(c["prediction"].as_array().unwrap())
            .enumerate()
        {
            let expected = number(&b["predictions"]);
            let actual = a.effect.unwrap();
            assert!(
                (actual - expected).abs() < 2e-9 * expected.abs().max(1.0),
                "case {} prediction {i}: {actual} != {expected}",
                c["case"]
            );
            let Variance::Estimate(actual) = a.variance else {
                panic!("missing variance")
            };
            let expected = number(&b["variance.estimates"]);
            assert!((actual - expected).abs() < 2e-9 * expected.abs().max(1.0));
        }
    }
}
