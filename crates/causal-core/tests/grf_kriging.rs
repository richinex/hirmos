use hirmos_causal_core::grf::kriging::Surface;
use serde_json::Value;
fn num(v: &Value) -> f64 {
    v.as_str()
        .map(|s| s.parse().unwrap())
        .or_else(|| v.as_f64())
        .unwrap()
}
fn vector(v: &Value) -> Vec<f64> {
    if let Some(a) = v.as_array() {
        a.iter().map(num).collect()
    } else {
        vec![num(v)]
    }
}
fn matrix(v: &Value) -> Vec<Vec<f64>> {
    v.as_array().unwrap().iter().map(vector).collect()
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 2e-9 * b.abs().max(1.), "{a} != {b}");
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn fixed_parameter_kriging_matches_r() {
    let f: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/kriging.json")).unwrap();
    for c in f["cases"].as_array().unwrap() {
        let x = matrix(&c["design"]);
        let y = vector(&c["response"]);
        let p = vector(&c["parameters"]);
        let s = Surface::at(&x, &y, num(&c["noise"]), &p).unwrap();
        close(s.log_likelihood, num(&c["log_likelihood"]));
        close(s.trend, num(&c["trend"]));
        for (a, b) in s.gradient.iter().zip(vector(&c["gradient"])) {
            close(*a, b);
        }
        for (a, b) in s
            .predict(&matrix(&c["newdata"]))
            .unwrap()
            .iter()
            .zip(vector(&c["prediction"]))
        {
            close(*a, b);
        }
    }
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_kriging_inputs_are_rejected() {
    assert!(Surface::at(&[], &[], 1., &[1., 1.]).is_err());
    assert!(Surface::at(&[vec![0.], vec![0.]], &[1., 2.], 0., &[1., 1.]).is_err());
    assert!(Surface::at(&[vec![0.], vec![1.]], &[1., 2.], -1., &[1., 1.]).is_err());
    assert!(Surface::at(&[vec![0.], vec![1.]], &[1., 2.], 1., &[0., 1.]).is_err());
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn fitted_kriging_matches_r() {
    let f: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/kriging.json")).unwrap();
    for (index, c) in f["cases"].as_array().unwrap().iter().enumerate() {
        let fitted =
            hirmos_causal_core::grf::kriging::fit(&matrix(&c["design"]), &vector(&c["response"]), 7)
                .unwrap();
        // km stores only range starts, not the appended variance start.
        for (a, b) in fitted.initial.iter().zip(vector(&c["fitted"]["initial"])) {
            close(*a, b);
        }
        let expected = num(&c["fitted"]["log_likelihood"]);
        assert!(
            (fitted.surface.log_likelihood - expected).abs() < 2e-6 * expected.abs().max(1.),
            "case {index}: likelihood {} != {expected}",
            fitted.surface.log_likelihood
        );
        for (a, b) in fitted
            .surface
            .predict(&matrix(&c["newdata"]))
            .unwrap()
            .iter()
            .zip(vector(&c["fitted"]["prediction"]))
        {
            assert!(
                (a - b).abs() < 2e-5 * b.abs().max(1.),
                "case {index}: fitted prediction {a} != {b}"
            );
        }
    }
}
