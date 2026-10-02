use nalgebra::DMatrix;
use serde_json::Value;
use hirmos_causal_core::staggered_did::dr::{self, Error, FitStatus, IpwNormalization, Sample, Score};

fn values(v: &Value) -> Vec<f64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}
fn close(label: &str, actual: f64, expected: f64) {
    let tolerance = 1e-9 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "{label}: Rust {actual:.16}, R {expected:.16}, tolerance {tolerance}"
    );
}
fn check(label: &str, actual: &Score, expected: &Value) {
    close(&format!("{label}/ATT"), actual.att, expected["att"].as_f64().unwrap());
    close(&format!("{label}/SE"), actual.se, expected["se"].as_f64().unwrap());
    let influence = values(&expected["influence"]);
    assert_eq!(actual.influence.len(), influence.len());
    for (i, (a, b)) in actual.influence.iter().zip(influence).enumerate() {
        close(&format!("{label}/influence[{i}]"), *a, b);
    }
}

#[test]
fn panel_ipw_and_outcome_regression_match_pinned_drdid() {
    let data: Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/staggered-did/ipw-or.json"
        ))
        .expect("Run ipw-or-oracle.R in the pinned container"),
    )
    .unwrap();
    let mut different_normalizations = 0;
    for (name, c) in data["cases"].as_object().unwrap() {
        let n = c["x"].as_array().unwrap().len();
        let p = c["x"][0].as_array().unwrap().len();
        let x = DMatrix::from_fn(n, p, |i, j| c["x"][i][j].as_f64().unwrap());
        let sample = Sample::new(
            x,
            values(&c["change"]),
            values(&c["treated"]).iter().map(|v| *v == 1.0).collect(),
            values(&c["weights"]),
        )
        .unwrap();
        let trim = c["trim"].as_f64().unwrap();
        let prop = values(&c["propensity"]);
        for (key, normalization) in [
            ("abadie", IpwNormalization::Abadie),
            ("hajek", IpwNormalization::Hajek),
        ] {
            let fit = dr::fit_ipw(&sample, trim, normalization).unwrap();
            assert!(matches!(fit.status, FitStatus::Converged { .. }), "{name}");
            let conditional = dr::ipw_score(&sample, &prop, trim, normalization).unwrap();
            check(&format!("{name}/{key}/oracle-propensity"), &conditional, &c["results"][key]);
            for (i, (a, b)) in fit.propensity.iter().zip(&prop).enumerate() {
                close(&format!("{name}/propensity[{i}]"), *a, *b);
            }
            check(&format!("{name}/{key}/fitted"), &fit.score, &c["results"][key]);
        }
        let fit = dr::fit_outcome_regression(&sample).unwrap();
        check(&format!("{name}/regression"), &fit.score, &c["results"]["regression"]);
        for (a, b) in fit.outcome.iter().zip(values(&c["outcome"])) {
            close(name, *a, b);
        }
        // Factoring out the shared outcome fit must not change the established DR score.
        check(
            &format!("{name}/dr/oracle-nuisances"),
            &dr::score(&sample, &prop, &values(&c["outcome"]), trim).unwrap(),
            &c["results"]["dr"],
        );
        check(
            &format!("{name}/dr"),
            &dr::fit(&sample, trim).unwrap().score,
            &c["results"]["dr"],
        );
        if (c["results"]["abadie"]["att"].as_f64().unwrap()
            - c["results"]["hajek"]["att"].as_f64().unwrap())
        .abs()
            > 1e-3
        {
            different_normalizations += 1;
        }
    }
    assert!(data["cases"].as_object().unwrap().len() >= 40);
    assert!(
        different_normalizations > 0,
        "The tests must distinguish IPW denominators."
    );
}

#[test]
fn method_specific_boundaries_do_not_substitute_estimators() {
    let x = DMatrix::from_fn(20, 2, |i, j| if j == 0 { 1.0 } else { i as f64 - 9.5 });
    let d: Vec<_> = (0..20).map(|i| i >= 10).collect();
    let change: Vec<_> = (0..20)
        .map(|i| 1.0 + 0.4 * i as f64 + f64::from(d[i]))
        .collect();
    let sample = Sample::new(x, change, d, vec![1.0; 20]).unwrap();
    assert!(
        dr::fit_outcome_regression(&sample).is_ok(),
        "OR does not require fitting a propensity model."
    );
    assert!(matches!(
        dr::ipw_score(&sample, &[0.5; 19], 0.995, IpwNormalization::Hajek),
        Err(Error::Shape)
    ));
    assert!(matches!(
        dr::ipw_score(&sample, &[0.0; 20], 0.995, IpwNormalization::Hajek),
        Err(Error::Probability)
    ));
    assert!(matches!(
        dr::ipw_score(&sample, &[0.5; 20], 0.4, IpwNormalization::Hajek),
        Err(Error::MissingGroup)
    ));
    assert!(
        dr::ipw_score(&sample, &[0.5; 20], 0.4, IpwNormalization::Abadie).is_ok(),
        "Unnormalized zero comparison mass is not a normalized mean."
    );
    let singular = Sample::new(
        DMatrix::from_element(20, 2, 1.0),
        vec![1.0; 20],
        (0..20).map(|i| i >= 10).collect(),
        vec![1.0; 20],
    )
    .unwrap();
    assert!(matches!(
        dr::fit_outcome_regression(&singular),
        Err(Error::Singular)
    ));
    assert!(matches!(
        dr::fit_ipw(&singular, 0.995, IpwNormalization::Hajek),
        Err(Error::Singular)
    ));
}
