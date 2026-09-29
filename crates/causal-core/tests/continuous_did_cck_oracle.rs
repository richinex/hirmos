use hirmos_causal_core::continuous_did::cck;
use serde_json::Value;
fn vector(v: &Value) -> Vec<f64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_f64().unwrap())
        .collect()
}
#[test]
fn npiv_fixed_and_selected_regressions_match_r() {
    let cases: Value = serde_json::from_str(include_str!("fixtures/continuous_did/cck.json")).unwrap();
    for (name, c) in cases.as_object().unwrap() {
        let s = c["segments"].as_u64().unwrap() as usize;
        let f = cck::fit(
            &vector(&c["x"]),
            &vector(&c["y"]),
            &vector(&c["grid"]),
            if s == 0 { None } else { Some(s) },
            c["iterations"].as_u64().unwrap() as usize,
            c["alpha"].as_f64().unwrap(),
            c["seed"].as_u64().unwrap() as u32,
        );
        let e = &c["result"];
        if let Some(error) = e["error"].as_str() {
            assert_eq!(name, "tied-0", "Unexpected new oracle failure: {error}");
            assert!(
                matches!(f, Err(cck::Error::Numerical)),
                "{name}: R refused this design ({error}), Rust returned a fit"
            );
            continue;
        }
        let f = f.unwrap_or_else(|error| panic!("{name}: {error:?}"));
        assert_eq!(
            f.segments,
            e["J.x.segments"].as_u64().unwrap() as usize,
            "{name}"
        );
        for (key, a) in [
            ("h", f.h),
            ("deriv", f.derivative),
            ("asy.se", f.se),
            ("deriv.asy.se", f.derivative_se),
            ("beta", f.coefficients),
            ("residuals", f.residuals),
        ] {
            let expected = e[key].as_array().unwrap();
            assert_eq!(a.len(), expected.len());
            for (i, (a, e)) in a.iter().zip(expected).enumerate() {
                let e = e.as_f64().or_else(|| e[0].as_f64()).unwrap();
                assert!(
                    (a - e).abs() < 1e-8 * (1.0 + e.abs()),
                    "{name} {key}[{i}]: {a} != {e}"
                );
            }
        }
        for (key, a) in [("cv", f.critical), ("cv.deriv", f.derivative_critical)] {
            let e = e[key].as_f64().unwrap();
            assert!(
                (a - e).abs() < 1e-8 * (1.0 + e.abs()),
                "{name} {key}: {a} != {e}"
            );
        }
    }
}

#[test]
fn invalid_inputs_are_refused_without_a_fallback() {
    let x: Vec<_> = (0..16).map(|i| i as f64 / 16.0).collect();
    let y: Vec<_> = x.iter().map(|v| v.sin()).collect();
    for alpha in [0.0, 1.0, -0.1, f64::NAN] {
        assert!(matches!(
            cck::fit(&x, &y, &x, Some(1), 9, alpha, 1),
            Err(cck::Error::Input)
        ));
    }
    assert!(matches!(
        cck::fit(&x, &y[..15], &x, Some(1), 9, 0.05, 1),
        Err(cck::Error::Input)
    ));
    assert!(matches!(
        cck::fit(&x, &y, &[], Some(1), 9, 0.05, 1),
        Err(cck::Error::Input)
    ));
    assert!(matches!(
        cck::fit(&x, &y, &x, Some(0), 9, 0.05, 1),
        Err(cck::Error::Input)
    ));
    assert!(matches!(
        cck::fit(&x, &y, &x, Some(1), 0, 0.05, 1),
        Err(cck::Error::Input)
    ));
    let mut invalid = x.clone();
    invalid[3] = f64::INFINITY;
    assert!(matches!(
        cck::fit(&invalid, &y, &x, Some(1), 9, 0.05, 1),
        Err(cck::Error::Input)
    ));
    assert!(cck::fit(&[1.0; 16], &y, &[1.0], Some(1), 9, 0.05, 1).is_err());
    assert!(matches!(
        cck::fit(&x, &y, &x, Some(usize::MAX), 9, 0.05, 1),
        Err(cck::Error::Input)
    ));
    assert!(matches!(
        cck::fit(&x, &y, &x, Some(1), usize::MAX, 0.05, 1),
        Err(cck::Error::Input)
    ));
    let mut extreme = x.clone();
    extreme[0] = -f64::MAX;
    extreme[15] = f64::MAX;
    assert!(matches!(
        cck::fit(&extreme, &y, &x, Some(1), 9, 0.05, 1),
        Err(cck::Error::Numerical)
    ));
}
