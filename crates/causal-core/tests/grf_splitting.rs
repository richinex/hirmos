use hirmos_causal_core::grf::splitting::best_split;
use serde_json::Value;
fn floats(v: &Value) -> Vec<f64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap_or(f64::NAN))
        .collect()
}
fn indices(v: &Value) -> Vec<usize> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as usize)
        .collect()
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn exact_upstream_split_decisions() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/splitting.json")).unwrap();
    assert_eq!(fixture["version"], "2.6.1");
    let mut missing_split = false;
    let mut stopped = false;
    for c in fixture["cases"].as_array().unwrap() {
        let columns: Vec<_> = c["columns"]
            .as_array()
            .unwrap()
            .iter()
            .map(floats)
            .collect();
        let actual = best_split(
            &columns,
            &floats(&c["treatment"]),
            &floats(&c["weights"]),
            &floats(&c["responses"]),
            &indices(&c["samples"]),
            &indices(&c["candidates"]),
            c["min_size"].as_u64().unwrap() as usize,
            c["alpha"].as_f64().unwrap(),
            c["penalty"].as_f64().unwrap(),
        )
        .unwrap();
        let e = &c["expected"];
        assert_eq!(
            actual.is_none(),
            e["stopped"].as_bool().unwrap(),
            "seed {}",
            c["seed"]
        );
        if let Some(s) = actual {
            assert_eq!(
                s.variable,
                e["variable"].as_u64().unwrap() as usize,
                "seed {}",
                c["seed"]
            );
            assert_eq!(
                s.missing_left,
                e["missing_left"].as_bool().unwrap(),
                "seed {}",
                c["seed"]
            );
            if e["value"].is_null() {
                assert!(s.value.is_nan());
                missing_split = true;
            } else {
                assert_eq!(s.value, e["value"].as_f64().unwrap(), "seed {}", c["seed"]);
            }
        } else {
            stopped = true;
        }
    }
    assert!(stopped);
    assert!(missing_split);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_splits_are_rejected() {
    let columns = vec![vec![0.0, 1.0, 2.0, 3.0]];
    let w = [0.0, 1.0, 0.0, 1.0];
    let weights = [1.0; 4];
    let response = [-1.0, -1.0, 1.0, 1.0];
    let rows = [0, 1, 2, 3];
    let run =
        |x: &[Vec<f64>], weights: &[f64], rows: &[usize], candidates: &[usize], alpha, penalty| {
            best_split(
                x, &w, weights, &response, rows, candidates, 1, alpha, penalty,
            )
        };
    assert!(run(&columns, &weights, &[4], &[0], 0.05, 0.0).is_err());
    assert!(run(&columns, &weights, &rows, &[1], 0.05, 0.0).is_err());
    assert!(run(&columns, &[-1.0; 4], &rows, &[0], 0.05, 0.0).is_err());
    assert!(run(&columns, &weights, &rows, &[0], f64::NAN, 0.0).is_err());
    assert!(run(&columns, &weights, &rows, &[0], 0.05, -1.0).is_err());
    assert!(run(&[vec![f64::INFINITY; 4]], &weights, &rows, &[0], 0.05, 0.0).is_err());
    assert!(run(&columns, &[0.0; 4], &rows, &[0], 0.05, 0.0)
        .unwrap()
        .is_none());
    assert!(run(&columns, &weights, &[], &[0], 0.05, 0.0)
        .unwrap()
        .is_none());
}
