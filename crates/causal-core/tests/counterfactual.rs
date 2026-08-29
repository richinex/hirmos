// Parity for the linear Gaussian counterfactual.
//
// The notebook abducts with stochastic variational inference, but the SCM there is linear
// with additive noise, so abduction is a linear solve and the counterfactual is closed form.
// The port is checked against the exact answer; the fixture also records where 5000 SVI
// steps land, which is the accuracy the notebook actually achieves.
use hirmos_causal_core::counterfactual::{Equation, LinearScm};
use serde_json::Value;

fn scm(c: &Value) -> LinearScm {
    let g = |k: &str| c[k].as_f64().unwrap();
    LinearScm {
        equations: vec![
            Equation {
                intercept: g("commits_mean"),
                parents: vec![],
                noise_scale: g("commits_std"),
            },
            Equation {
                intercept: g("bugs_intercept"),
                parents: vec![(0, g("bugs_beta_commits"))],
                noise_scale: g("bugs_residual_std"),
            },
            Equation {
                intercept: g("sec_intercept"),
                parents: vec![(0, g("sec_beta_commits")), (1, g("sec_beta_bugs"))],
                noise_scale: g("sec_residual_std"),
            },
            Equation {
                intercept: g("inc_intercept"),
                parents: vec![
                    (2, g("inc_beta_security")),
                    (0, g("inc_beta_commits")),
                    (1, g("inc_beta_bugs")),
                ],
                noise_scale: g("inc_residual_std"),
            },
        ],
    }
}

fn maxdev(got: &[f64], want: &[f64]) -> f64 {
    got.iter()
        .zip(want)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max)
}

#[test]
fn counterfactual_matches_closed_form() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/counterfactual_scm.json")).unwrap();
    let model = scm(&root["coefficients"]);
    let o = &root["observation"];
    let observed = [
        o["commits"].as_f64().unwrap(),
        o["bugs"].as_f64().unwrap(),
        o["security"].as_f64().unwrap(),
        o["incidents"].as_f64().unwrap(),
    ];
    let do_sec = root["do_security"].as_f64().unwrap();
    let scale = root["pseudo_scale"].as_f64().unwrap();

    // The affine map from noise to observations.
    let (a, b) = model.affine_map();
    let want_a: Vec<Vec<f64>> = serde_json::from_value(root["A"].clone()).unwrap();
    let want_b: Vec<f64> = serde_json::from_value(root["b"].clone()).unwrap();
    let mut dev = 0.0f64;
    for (i, row) in want_a.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            dev = dev.max((a[(i, j)] - v).abs());
        }
        dev = dev.max((b[i] - want_b[i]).abs());
    }
    println!("affine map maxdev {dev:.3e}");
    assert!(dev <= 1e-12, "affine map deviation {dev}");

    // Abduction, both variants.
    let exact = model.abduct_exact(&observed);
    let want_exact: Vec<f64> = serde_json::from_value(root["abduction_exact"].clone()).unwrap();
    let d = maxdev(&exact, &want_exact);
    println!("exact abduction maxdev {d:.3e}");
    assert!(d <= 1e-12, "exact abduction deviation {d}");

    let soft = model.abduct_with_observation_noise(&observed, scale);
    let want_soft: Vec<f64> =
        serde_json::from_value(root["abduction_pseudo_delta"].clone()).unwrap();
    let d = maxdev(&soft, &want_soft);
    println!("pseudo delta abduction maxdev {d:.3e}");
    assert!(d <= 1e-10, "pseudo delta abduction deviation {d}");

    // Action and prediction.
    let cf = model.counterfactual(&observed, &[(2, do_sec)], None);
    let want_cf: Vec<f64> = serde_json::from_value(root["counterfactual_exact"].clone()).unwrap();
    let d = maxdev(&cf, &want_cf);
    println!("counterfactual maxdev {d:.3e}");
    assert!(d <= 1e-12, "counterfactual deviation {d}");

    let cf_soft = model.counterfactual(&observed, &[(2, do_sec)], Some(scale));
    let want_soft_cf: Vec<f64> =
        serde_json::from_value(root["counterfactual_pseudo_delta"].clone()).unwrap();
    let d = maxdev(&cf_soft, &want_soft_cf);
    println!("counterfactual under pseudo delta maxdev {d:.3e}");
    assert!(d <= 1e-9, "pseudo delta counterfactual deviation {d}");

    // What the notebook's sampler reaches, for the record.
    let svi: Vec<f64> = serde_json::from_value(root["counterfactual_svi"].clone()).unwrap();
    println!(
        "incidents: observed {:.4}, counterfactual {:.4}, notebook SVI {:.4} (SVI error {:.1e})",
        observed[3],
        cf[3],
        svi[3],
        (svi[3] - cf[3]).abs()
    );
}
