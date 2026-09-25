//! statsmodels' own `TestLogitNewton`, which fits the Spector and Mazzeo data and checks the
//! result against Stata. Its assertions come first; the parity against the Python fit follows.

use hirmos_causal_core::logit::{self, Settings, Termination};

/// `assert_almost_equal(..., DECIMAL_4)` passes when the gap is under 1.5e-4.
const STATA_TOLERANCE: f64 = 1.5e-4;
/// statsmodels and the port run the same loop over the same LAPACK solve.
const PYTHON_TOLERANCE: f64 = 1e-12;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../oracle/fixtures/statsmodels_logit_spector.json")).unwrap()
}

fn numbers(value: &serde_json::Value) -> Vec<f64> {
    value.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()
}

/// `add_constant(prepend=False)` puts the constant last, after GPA, TUCE and PSI.
fn spector() -> (Vec<Vec<f64>>, Vec<f64>, logit::Logit) {
    let fixture = fixture();
    let design: Vec<Vec<f64>> = fixture["design"].as_array().unwrap().iter().map(numbers).collect();
    let endog = numbers(&fixture["endog"]);
    let fitted = logit::fit(&design, &endog, Settings::default()).expect("the Spector design fits");
    (design, endog, fitted)
}

#[test]
fn test_params() {
    let (_, _, fitted) = spector();
    let stata = numbers(&fixture()["stata"]["params"]);
    for (ours, theirs) in fitted.params.iter().zip(&stata) {
        assert!((ours - theirs).abs() < STATA_TOLERANCE, "ours={ours:.12} stata={theirs:.12}");
    }
}

#[test]
fn test_predict() {
    let (design, _, fitted) = spector();
    let stata = numbers(&fixture()["stata"]["phat"]);
    for (ours, theirs) in fitted.predict_probability(&design).iter().zip(&stata) {
        assert!((ours - theirs).abs() < STATA_TOLERANCE, "ours={ours:.12} stata={theirs:.12}");
    }
}

#[test]
fn test_predict_xb() {
    let (design, _, fitted) = spector();
    let stata = numbers(&fixture()["stata"]["yhat"]);
    for (ours, theirs) in fitted.predict_linear(&design).iter().zip(&stata) {
        assert!((ours - theirs).abs() < STATA_TOLERANCE, "ours={ours:.12} stata={theirs:.12}");
    }
}

#[test]
fn test_llf() {
    let (design, endog, fitted) = spector();
    let stata = fixture()["stata"]["llf"].as_f64().unwrap();
    let ours = fitted.log_likelihood(&design, &endog);
    assert!((ours - stata).abs() < STATA_TOLERANCE, "ours={ours:.12} stata={stata:.12}");
}

#[test]
fn the_port_reaches_the_python_fit() {
    let (design, endog, fitted) = spector();
    let fixture = fixture();
    let python = &fixture["statsmodels"];
    assert_eq!(fitted.termination, Termination::Converged);
    assert!(python["converged"].as_bool().unwrap());
    assert_eq!(fitted.iterations, python["iterations"].as_u64().unwrap() as usize);

    let oracle = numbers(&python["params"]);
    let worst = fitted.params.iter().zip(&oracle)
        .map(|(a, b)| (a - b).abs()).fold(0f64, f64::max);
    let likelihood = (fitted.log_likelihood(&design, &endog) - python["llf"].as_f64().unwrap()).abs();
    println!("worst parameter gap {worst:.3e}, log-likelihood gap {likelihood:.3e}");
    assert!(worst < PYTHON_TOLERANCE, "worst parameter gap {worst:.3e}");
    assert!(likelihood < PYTHON_TOLERANCE, "log-likelihood gap {likelihood:.3e}");
}
