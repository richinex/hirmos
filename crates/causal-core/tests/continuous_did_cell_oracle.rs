use hirmos_causal_core::continuous_did::cell;
use serde_json::Value;
fn vector(x: &Value) -> Vec<f64> {
    x.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}
fn compare(name: &str, actual: &[f64], expected: &Value) {
    let expected = vector(expected);
    assert_eq!(actual.len(), expected.len(), "{name}");
    for (i, (a, e)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (a - e).abs() <= 1e-10 * (1.0 + e.abs()),
            "{name}[{i}]: {a} != {e}"
        );
    }
}
#[test]
fn group_time_algebra_matches_contdid() {
    let fixture: Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/continuous_did/cells.json"
        ))
        .unwrap(),
    )
    .unwrap();
    for (name, c) in fixture.as_object().unwrap() {
        let fit = cell::fit(
            &vector(&c["dose"]),
            &vector(&c["change"]),
            &vector(&c["grid"]),
            c["degree"].as_u64().unwrap() as usize,
            vector(&c["knots"]),
        )
        .unwrap();
        let result = &c["result"];
        let extra = &result["extra_gt_returns"];
        for (key, actual) in [
            ("bet", &fit.coefficients),
            ("att.d", &fit.level),
            ("acrt.d", &fit.slope),
        ] {
            compare(&format!("{name} {key}"), actual, &extra[key]);
        }
        compare(
            &format!("{name} influence"),
            &fit.influence,
            &result["inf_func"],
        );
        for (key, actual) in [
            ("att.overall", fit.average_level),
            ("acrt.overall", fit.average_slope),
        ] {
            let expected = extra[key].as_f64().unwrap();
            assert!(
                (actual - expected).abs() < 1e-10 * (1.0 + expected.abs()),
                "{name} {key}: {actual} != {expected}"
            );
        }
        for (key, actual) in [("bread", &fit.bread), ("Xe", &fit.scores)] {
            assert_eq!(actual.len(), extra[key].as_array().unwrap().len());
            for (i, row) in actual.iter().enumerate() {
                compare(&format!("{name} {key}[{i}]"), row, &extra[key][i]);
            }
        }
    }
}
#[test]
fn invalid_cells_do_not_silently_fit() {
    assert!(matches!(
        cell::fit(&[0.0, 0.0], &[1.0, 2.0], &[0.0, 1.0], 1, vec![]),
        Err(cell::Error::MissingGroup)
    ));
    assert!(matches!(
        cell::fit(&[0.0, 0.1], &[1.0, f64::NAN], &[0.0, 1.0], 1, vec![]),
        Err(cell::Error::NonFinite)
    ));
    assert!(matches!(
        cell::fit(&[0.0, 0.1, 0.9], &[1.0, 2.0, 3.0], &[0.1, 0.9], 3, vec![]),
        Err(cell::Error::InsufficientRows)
    ));
}

#[test]
fn degenerate_designs_fail_in_both_implementations() {
    let reference: Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/degenerate.json")).unwrap();
    let grid: Vec<_> = (0..11).map(|i| 0.1 + 0.08 * i as f64).collect();
    for (name, treated) in [
        ("constant_dose", vec![0.5; 24]),
        (
            "rank_deficient",
            (0..24)
                .map(|i| if i % 2 == 0 { 0.1 } else { 0.9 })
                .collect(),
        ),
    ] {
        assert!(reference[name]["result"]["error"].as_str().is_some());
        let mut dose = vec![0.0; 8];
        dose.extend(treated);
        let change: Vec<_> = dose
            .iter()
            .enumerate()
            .map(|(i, d)| 0.3 + 0.7 * d + 0.09 * ((i + 1) as f64 * 1.7).sin())
            .collect();
        let result = cell::fit(&dose, &change, &grid, 3, vec![]);
        match name {
            "constant_dose" => assert!(matches!(
                result,
                Err(cell::Error::Spline(
                    hirmos_causal_core::continuous_did::spline::Error::InvalidBoundary
                ))
            )),
            _ => assert!(matches!(result, Err(cell::Error::RankDeficient))),
        }
    }
}
