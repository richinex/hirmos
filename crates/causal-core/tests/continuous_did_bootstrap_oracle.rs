use hirmos_causal_core::continuous_did::bootstrap::{Error, Stream};
use serde_json::Value;
fn vector(v: &Value) -> Vec<f64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_f64().unwrap())
        .collect()
}
fn compare(a: &[f64], e: &Value) {
    let e = vector(e);
    assert_eq!(a.len(), e.len());
    for (a, e) in a.iter().zip(e) {
        assert!((a - e).abs() <= 1e-11 * (1.0 + e.abs()), "{a} != {e}");
    }
}
#[test]
fn draws_scales_bands_and_stream_continuation_match_r() {
    let fixtures: Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/bootstrap.json")).unwrap();
    for (_, case) in fixtures.as_object().unwrap() {
        let input: Vec<_> = case["influence"]
            .as_array()
            .unwrap()
            .iter()
            .map(vector)
            .collect();
        let mut stream = Stream::new(case["seed"].as_u64().unwrap() as u32);
        for key in ["result", "next_result"] {
            let b = stream
                .run(
                    &input,
                    case["iterations"].as_u64().unwrap() as usize,
                    case["alpha"].as_f64().unwrap(),
                )
                .unwrap();
            compare(&b.standard_errors, &case[key]["boot_se"]);
            let e = case[key]["crit_val"].as_f64().unwrap();
            assert!((b.critical - e).abs() < 1e-11 * (1.0 + e.abs()));
            if key == "result" {
                assert_eq!(
                    b.pointwise_floor_applied,
                    !case["warnings"]
                        .as_array()
                        .map_or(case["warnings"].is_null(), Vec::is_empty)
                );
                for (i, row) in b.draws.iter().enumerate() {
                    compare(row, &case["draws"][i]);
                }
            }
        }
    }
}
#[test]
fn undefined_bands_do_not_become_valid_intervals() {
    let mut s = Stream::new(1);
    assert!(matches!(s.run(&[], 99, 0.05), Err(Error::Shape)));
    assert!(matches!(
        s.run(&vec![vec![0.0]; 3], 99, 0.05),
        Err(Error::DegenerateScale { column: 0 })
    ));
    assert!(matches!(
        s.run(&[vec![f64::NAN]], 99, 0.05),
        Err(Error::NonFinite)
    ));
    assert!(matches!(
        s.run(&[vec![1.0]], 0, 0.05),
        Err(Error::Iterations)
    ));
    assert!(matches!(s.run(&[vec![1.0]], 99, 0.0), Err(Error::Alpha)));
}
