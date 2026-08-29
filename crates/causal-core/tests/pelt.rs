// Parity for the PELT changepoint detector against ruptures, both cost models.
use hirmos_causal_core::pelt::{pelt_l2, pelt_rbf};
use serde_json::Value;

#[test]
fn pelt_matches_ruptures() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/pelt.json")).unwrap();
    let mut checked = 0;
    for case in root["cases"].as_array().unwrap() {
        let signal: Vec<Vec<f64>> = serde_json::from_value(case["signal"].clone()).unwrap();
        let min_size = case["min_size"].as_u64().unwrap() as usize;
        let jump = case["jump"].as_u64().unwrap() as usize;
        let pen = case["pen"].as_f64().unwrap();
        let want: Vec<usize> = serde_json::from_value(case["breakpoints"].clone()).unwrap();
        let got = match case["model"].as_str().unwrap() {
            "rbf" => pelt_rbf(&signal, min_size, jump, pen),
            _ => pelt_l2(&signal, min_size, jump, pen),
        };
        println!(
            "{:28} got {:?} want {:?}",
            case["name"].as_str().unwrap(),
            got,
            want
        );
        assert_eq!(got, want, "breakpoints for {}", case["name"]);
        checked += 1;
    }
    println!("{checked} PELT cases match ruptures exactly");
}
