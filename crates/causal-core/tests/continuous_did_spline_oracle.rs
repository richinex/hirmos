use hirmos_causal_core::continuous_did::spline::Basis;
use serde_json::Value;

#[test]
fn boundary_sorting_and_extrapolation_match_splines2() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/spline-boundary.json")).unwrap();
    for (name, case) in fixture.as_object().unwrap() {
        let b = Basis::new(
            case["degree"].as_u64().unwrap() as usize,
            case["knots"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect(),
            [
                case["boundary"][0].as_f64().unwrap(),
                case["boundary"][1].as_f64().unwrap(),
            ],
        )
        .unwrap();
        let intercept = case["intercept"].as_bool().unwrap();
        for (i, x) in case["x"].as_array().unwrap().iter().enumerate() {
            let x = x.as_f64().unwrap();
            for (field, actual) in [
                ("basis", b.evaluate(x, intercept).unwrap()),
                ("derivative", b.derivative(x, intercept).unwrap()),
            ] {
                let expected = case[field][i].as_array().unwrap();
                assert_eq!(actual.len(), expected.len());
                for (j, (a, e)) in actual.iter().zip(expected).enumerate() {
                    let e = e.as_f64().unwrap();
                    assert!(
                        (a - e).abs() < 1e-12 * (1.0 + e.abs()),
                        "{name} {field}[{i},{j}]: {a} != {e}"
                    );
                }
            }
        }
    }
}

#[test]
fn splines2_values_and_derivatives() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/continuous_did/spline-basis.json");
    let fixture: Value = serde_json::from_str(
        &std::fs::read_to_string(path)
            .expect("Generate spline-basis.json with the Docker oracle first"),
    )
    .unwrap();
    for (name, case) in fixture.as_object().unwrap() {
        let values = |field: &str| {
            case[field]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        let x = values("x");
        let b = Basis::new(
            case["degree"].as_u64().unwrap() as usize,
            values("knots"),
            [x[0], *x.last().unwrap()],
        )
        .unwrap();
        for (field, derivative) in [("basis", false), ("derivative", true)] {
            for (i, value) in x.iter().enumerate() {
                let actual = if derivative {
                    b.derivative(*value, false)
                } else {
                    b.evaluate(*value, false)
                }
                .unwrap();
                let expected = case[field][i].as_array().unwrap();
                assert_eq!(actual.len(), expected.len(), "{name} {field}");
                for (j, (a, e)) in actual.iter().zip(expected).enumerate() {
                    let e = e.as_f64().unwrap();
                    assert!(
                        (a - e).abs() <= 1e-12 * (1.0 + e.abs()),
                        "{name} {field}[{i},{j}]: {a} != {e}"
                    );
                }
            }
        }
    }
}
