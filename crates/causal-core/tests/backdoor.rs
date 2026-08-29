// Parity for DoWhy's default backdoor identification and the linear ATE.
use hirmos_causal_core::{backdoor_linear_ate, backdoor_variables, Dag};
use nalgebra::DMatrix;
use serde_json::Value;

#[test]
fn backdoor_matches_dowhy() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/backdoor.json")).unwrap();
    for fx in root.as_array().unwrap() {
        let name = fx["name"].as_str().unwrap();
        let nodes: Vec<String> = serde_json::from_value(fx["nodes"].clone()).unwrap();
        let idx = |s: &str| nodes.iter().position(|n| n == s).unwrap();
        let edges: Vec<(String, String)> = serde_json::from_value(fx["edges"].clone()).unwrap();
        let edge_idx: Vec<(usize, usize)> = edges.iter().map(|(a, b)| (idx(a), idx(b))).collect();
        let dag = Dag::new(nodes.len(), &edge_idx);
        let (t, y) = (idx("T"), idx("Y"));
        let mut got: Vec<String> = backdoor_variables(&dag, t, y)
            .iter()
            .map(|&v| nodes[v].clone())
            .collect();
        got.sort();
        let want: Vec<String> = serde_json::from_value(fx["backdoor"].clone()).unwrap();
        assert_eq!(got, want, "{name} backdoor set");

        let rows: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
        let data = DMatrix::from_fn(rows.len(), rows[0].len(), |i, j| rows[i][j]);
        let adjust: Vec<usize> = want.iter().map(|s| idx(s)).collect();
        let ate = backdoor_linear_ate(&data, t, y, &adjust);
        let want_ate = fx["ate"].as_f64().unwrap();
        assert!(
            (ate - want_ate).abs() <= 1e-8 * want_ate.abs().max(1.0),
            "{name} ate: got {ate}, oracle {want_ate}",
        );
        println!("{name}: backdoor set and ate match");
    }
}

#[test]
fn refuters_match_dowhy() {
    use hirmos_causal_core::nprandom::Mt19937;
    use hirmos_causal_core::{refute_data_subset, refute_placebo, refute_random_common_cause};
    let fx: Value = serde_json::from_str(include_str!("../oracle/fixtures/refuters.json")).unwrap();
    let rows: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
    let data = DMatrix::from_fn(rows.len(), rows[0].len(), |i, j| rows[i][j]);
    let sims = fx["num_simulations"].as_u64().unwrap() as usize;
    let adjust = [2usize]; // W

    let close = |label: &str, got: f64, want: f64| {
        assert!(
            (got - want).abs() <= 1e-8 * want.abs().max(1.0),
            "{label}: got {got}, oracle {want}",
        );
    };

    let mut mt = Mt19937::seeded(555);
    let got = refute_placebo(&data, 0, 1, &adjust, sims, &mut mt);
    close("placebo", got, fx["results"]["placebo"].as_f64().unwrap());

    let mut mt = Mt19937::seeded(556);
    let got = refute_data_subset(&data, 0, 1, &adjust, 0.8, sims, &mut mt);
    close("subset", got, fx["results"]["subset"].as_f64().unwrap());

    let mut mt = Mt19937::seeded(557);
    let got = refute_random_common_cause(&data, 0, 1, &adjust, sims, &mut mt);
    close(
        "random_cc",
        got,
        fx["results"]["random_cc"].as_f64().unwrap(),
    );
    println!("refuters: placebo, subset and random common cause all match");
}
