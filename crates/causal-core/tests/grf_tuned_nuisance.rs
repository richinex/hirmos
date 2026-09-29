use hirmos_causal_core::grf::{
    causal,
    forest::{Options, SeedMode},
    sampling::Clusters,
    tree::{self, Honesty},
    tuning::{Config, Parameter, Status},
};
use serde_json::Value;
fn vector(v: &Value) -> Vec<f64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| {
            v.as_str()
                .map(|s| s.parse().unwrap())
                .or_else(|| v.as_f64())
                .unwrap()
        })
        .collect()
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn full_tuned_nuisance_matches_r() {
    let inputs: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/tuning.json")).unwrap();
    let reference: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/tuned-nuisance.json")).unwrap();
    let c = &inputs["cases"][0];
    let x: Vec<_> = c["X"].as_array().unwrap().iter().map(vector).collect();
    let y = vector(&c["Y"]);
    let w = vector(&c["W"]);
    let clusters = Clusters::new(&(0..y.len()).collect::<Vec<_>>(), 1).unwrap();
    let options = Options {
        trees: 200,
        group_size: 2,
        sample_fraction: 0.5,
        seed: 7,
        seed_mode: SeedMode::Indexed,
        batches: 1,
        tree: tree::Options {
            mtry: 4,
            min_node_size: 5,
            honesty: Honesty::Enabled {
                fraction: 0.5,
                prune: true,
            },
            alpha: 0.05,
            imbalance_penalty: 0.,
        },
    };
    let config = Config {
        parameters: vec![Parameter::Mtry, Parameter::Alpha],
        mini_trees: 50,
        repetitions: 12,
        draws: 100,
    };
    for c in reference["cases"].as_array().unwrap() {
        let given = c["supplied"].as_bool().unwrap().then_some([0.]);
        let fit = causal::fit_tuned(
            &x,
            &y,
            &w,
            None,
            &clusters,
            &options,
            given.as_ref().map(|v| v.as_slice()),
            given.as_ref().map(|v| v.as_slice()),
            true,
            true,
            &config,
        )
        .unwrap();
        let tuning = fit.tuning.as_ref().unwrap();
        assert_eq!(tuning.outcome.is_none(), given.is_some());
        assert_eq!(tuning.treatment.is_none(), given.is_some());
        let status = match &tuning.causal.status {
            Status::Failed(_) => "failure",
            Status::Default { .. } => "default",
            Status::Tuned { .. } => "tuned",
        };
        assert_eq!(status, c["tuning"]["status"].as_str().unwrap());
        let predictions: Vec<_> = fit
            .forest
            .predict(&x, true, false)
            .unwrap()
            .iter()
            .map(|p| p.effect.unwrap())
            .collect();
        for (name, values) in [
            ("outcome_hat", &fit.outcome_hat),
            ("treatment_hat", &fit.treatment_hat),
            ("predictions", &predictions),
        ] {
            for (a, b) in values.iter().zip(vector(&c[name])) {
                assert!((a - b).abs() < 2e-9 * b.abs().max(1.), "{name}: {a} != {b}");
            }
        }
    }
}
