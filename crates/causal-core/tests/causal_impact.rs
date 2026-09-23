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

    let impact = causal_impact(&y, &exog, n_pre, n_pre..y.len(), 100);
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

/// A narrowed evaluation window reports fewer rows of the same forecast. It must not move the
/// fit, the counterfactual or its standard errors, because the window says what is summarised
/// and the pre-intervention rows say what is fitted.
#[test]
fn a_narrowed_window_reports_the_same_forecast() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/causal_impact.json")).unwrap();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let exog: Vec<Vec<f64>> = serde_json::from_value(root["exog"].clone()).unwrap();
    let n_pre = root["n_pre"].as_u64().unwrap() as usize;

    let full = causal_impact(&y, &exog, n_pre, n_pre..y.len(), 100);
    for end in [n_pre + 1, n_pre + 7, y.len()] {
        let windowed = causal_impact(&y, &exog, n_pre, n_pre..end, 100);
        let rows = end - n_pre;
        assert_eq!(windowed.counterfactual.len(), rows, "window {end} rows");
        assert_eq!(windowed.params, full.params, "window {end} parameters");
        assert_eq!(
            maxdev(&windowed.counterfactual, &full.counterfactual[..rows]),
            0.0,
            "window {end} counterfactual"
        );
        assert_eq!(
            maxdev(&windowed.counterfactual_se, &full.counterfactual_se[..rows]),
            0.0,
            "window {end} standard errors"
        );
        assert_eq!(
            maxdev(&windowed.pointwise, &full.pointwise[..rows]),
            0.0,
            "window {end} pointwise effects"
        );
        let want: f64 = full.pointwise[..rows].iter().sum();
        assert!(
            (windowed.cumulative - want).abs() <= 1e-9,
            "window {end} cumulative"
        );
    }
}

/// The same invariant for the Bayesian route, which narrows the summary rather than the chain:
/// one seed, one posterior, and the evaluated rows read off it.
#[test]
fn a_narrowed_bayesian_window_reports_the_same_posterior() {
    use hirmos_causal_core::bayesian_impact::{fit, Plan};
    use nalgebra::DMatrix;
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/causal_impact.json")).unwrap();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let exog: Vec<Vec<f64>> = serde_json::from_value(root["exog"].clone()).unwrap();
    let n_pre = root["n_pre"].as_u64().unwrap() as usize;
    let x = DMatrix::from_fn(y.len(), exog[0].len(), |row, column| exog[row][column]);

    let plan = |end: usize| {
        Plan::new(&y, &x, n_pre, n_pre..end, 60, 20, 1234, 0.01).expect("plan")
    };
    let full = fit(&plan(y.len())).expect("full window");
    let windowed = fit(&plan(n_pre + 1)).expect("single row");
    assert_eq!(windowed.post_path.len(), 1);
    assert_eq!(windowed.means, full.means, "posterior means");
    let (narrow, wide) = (&windowed.post_path[0], &full.post_path[0]);
    assert_eq!(narrow.predicted.mean, wide.predicted.mean, "predicted mean");
    assert_eq!(narrow.effect.mean, wide.effect.mean, "pointwise effect");
    assert_eq!(
        windowed.summaries.average.absolute.mean, narrow.effect.mean,
        "the average over one row is that row"
    );
}
