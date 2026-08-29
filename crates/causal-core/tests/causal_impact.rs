// Parity for the causal impact model: a local level with static regression on controls,
// fitted on the pre-intervention window and forecast forward for the counterfactual. This
// is the model the CausalImpact packages build, checked against statsmodels directly.
use hirmos_causal_core::causal_impact::{causal_impact, kalman_filter, loglike, start_params};
use serde_json::Value;

fn maxdev(got: &[f64], want: &[f64]) -> f64 {
    got.iter()
        .zip(want)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max)
}

#[test]
fn causal_impact_matches_statsmodels() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/causal_impact.json")).unwrap();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let exog: Vec<Vec<f64>> = serde_json::from_value(root["exog"].clone()).unwrap();
    let n_pre = root["n_pre"].as_u64().unwrap() as usize;

    let want_start: Vec<f64> = serde_json::from_value(root["start_params"].clone()).unwrap();
    let got_start = start_params(&y[..n_pre], &exog[..n_pre]);
    let d = maxdev(&got_start, &want_start);
    println!("start_params maxdev {d:.3e}");
    assert!(d <= 1e-8, "start params deviation {d}");

    let f = kalman_filter(&want_start, &y[..n_pre], &exog[..n_pre]);
    let want_llf = root["llf_start"].as_f64().unwrap();
    println!("llf at start dev {:.3e}", (f.llf - want_llf).abs());
    assert!((f.llf - want_llf).abs() <= 1e-9, "start log likelihood");
    let want_ll: Vec<f64> = serde_json::from_value(root["loglikeobs_start"].clone()).unwrap();
    let d = maxdev(&f.loglikeobs, &want_ll);
    println!("per observation log likelihoods maxdev {d:.3e}");
    assert!(d <= 1e-9, "loglikeobs deviation {d}");
    let want_v: Vec<f64> = serde_json::from_value(root["forecasts_error_start"].clone()).unwrap();
    let d = maxdev(&f.forecasts_error, &want_v);
    println!("forecast errors maxdev {d:.3e}");
    assert!(d <= 1e-9, "forecast error deviation {d}");
    let _ = loglike(&want_start, &y[..n_pre], &exog[..n_pre]);

    let impact = causal_impact(&y, &exog, n_pre, 100);
    let want_p: Vec<f64> = serde_json::from_value(root["fit_params"].clone()).unwrap();
    let d = maxdev(&impact.params, &want_p);
    println!(
        "fit params maxdev {d:.3e}, llf dev {:.3e}",
        (impact.llf - root["fit_llf"].as_f64().unwrap()).abs()
    );
    assert!(d <= 1e-5, "fit parameter deviation {d}");

    let want_cf: Vec<f64> = serde_json::from_value(root["counterfactual"].clone()).unwrap();
    let d = maxdev(&impact.counterfactual, &want_cf);
    println!("counterfactual maxdev {d:.3e}");
    assert!(d <= 1e-4, "counterfactual deviation {d}");
    let want_se: Vec<f64> = serde_json::from_value(root["counterfactual_se"].clone()).unwrap();
    let d = maxdev(&impact.counterfactual_se, &want_se);
    println!("counterfactual standard errors maxdev {d:.3e}");
    assert!(d <= 1e-4, "counterfactual se deviation {d}");

    let want_avg = root["average_impact"].as_f64().unwrap();
    let want_cum = root["cumulative_impact"].as_f64().unwrap();
    println!(
        "average impact {:.6} (oracle {:.6}), cumulative {:.6} (oracle {:.6})",
        impact.average, want_avg, impact.cumulative, want_cum
    );
    assert!((impact.average - want_avg).abs() <= 1e-4, "average impact");
    assert!(
        (impact.cumulative - want_cum).abs() <= 1e-2,
        "cumulative impact"
    );
}
