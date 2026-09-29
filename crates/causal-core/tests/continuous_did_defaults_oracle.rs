use hirmos_causal_core::continuous_did::defaults::{self, Error};
use serde_json::Value;
fn vector(v: &Value) -> Vec<f64> {
    if let Some(x) = v.as_f64() {
        vec![x]
    } else {
        v.as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_f64().unwrap())
            .collect()
    }
}
#[test]
fn default_grids_and_quantile_knots_match_r() {
    let cases: Value = serde_json::from_str(include_str!("fixtures/continuous_did/defaults.json")).unwrap();
    for (name, c) in cases.as_object().unwrap() {
        let d = defaults::from_first_period(
            &vector(&c["dose"]),
            c["num_knots"].as_u64().unwrap() as usize,
        )
        .unwrap();
        for (key, actual) in [("grid", d.grid), ("knots", d.knots)] {
            let expected = vector(&c[key]);
            assert_eq!(actual.len(), expected.len());
            for (a, e) in actual.iter().zip(expected) {
                assert!(
                    (a - e).abs() <= 1e-13 * (1.0 + e.abs()),
                    "{name} {key}: {a} != {e}"
                );
            }
        }
    }
}
#[test]
fn invalid_doses_are_not_silently_dropped() {
    assert!(matches!(
        defaults::from_first_period(&[], 0),
        Err(Error::NoPositiveDose)
    ));
    assert!(matches!(
        defaults::from_first_period(&[0.0], 0),
        Err(Error::NoPositiveDose)
    ));
    assert!(matches!(
        defaults::from_first_period(&[1.0, f64::NAN], 0),
        Err(Error::InvalidDose)
    ));
    assert!(matches!(
        defaults::from_first_period(&[-1.0, 1.0], 0),
        Err(Error::InvalidDose)
    ));
    assert!(matches!(
        defaults::from_first_period(&[1.0, 2.0], usize::MAX),
        Err(Error::Allocation)
    ));
}
