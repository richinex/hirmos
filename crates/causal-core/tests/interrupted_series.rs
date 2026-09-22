// Parity for the interrupted time series of Lopez Bernal, Cummins and Gasparrini (IJE 2017) on
// the paper's Sicily data: the harmonic terms of tsModel::harmonic, the quasi-Poisson models 3
// and 4 with the population offset against both statsmodels and the paper's R code, and the
// same designs on the rate by OLS with Newey-West errors against statsmodels.
use hirmos_causal_core::interrupted_series::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/interrupted_series.json")).unwrap()
}

fn floats(v: &Value) -> Vec<f64> {
    serde_json::from_value(v.clone()).unwrap()
}

fn maxdev(got: &[f64], want: &[f64]) -> f64 {
    assert_eq!(got.len(), want.len(), "length mismatch");
    got.iter().zip(want).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max)
}

fn spec(root: &Value, impact: ImpactModel) -> InterruptedSeriesDesign {
    InterruptedSeriesDesign {
        intervention_row: root["intervention_row"].as_u64().unwrap() as usize,
        lag: 0,
        impact,
        harmonics: Some(Harmonics { pairs: 2, period: 12.0, phase: floats(&root["month"]) }),
    }
}

#[test]
fn harmonic_matches_tsmodel() {
    let root = fixture();
    let got = harmonic(&floats(&root["month"]), 2, 12.0);
    let want: Vec<Vec<f64>> = serde_json::from_value(root["harmonic"].clone()).unwrap();
    for (r, row) in want.iter().enumerate() {
        for (c, value) in row.iter().enumerate() {
            assert!((got[(r, c)] - value).abs() < 1e-14, "harmonic[{r},{c}]");
        }
    }
}

#[test]
fn design_is_the_papers() {
    let root = fixture();
    let d = design(59, &spec(&root, ImpactModel::LevelAndSlope));
    assert_eq!(d.names, ["const", "time", "step", "slope_change", "sin1", "sin2", "cos1", "cos2"]);
    let smokban = floats(&root["smokban"]);
    let time = floats(&root["time"]);
    for r in 0..59 {
        assert_eq!(d.x[(r, 1)], time[r]);
        assert_eq!(d.x[(r, 2)], smokban[r]);
        assert_eq!(d.x[(r, 3)], (time[r] - 36.0) * smokban[r], "I(time - 36):smokban at row {r}");
        assert_eq!(d.counterfactual[(r, 2)], 0.0);
        assert_eq!(d.counterfactual[(r, 3)], 0.0);
    }
}

fn check_count(root: &Value, key: &str, impact: ImpactModel) {
    let want = &root[key];
    let fit = fit_count(&floats(&root["aces"]), Some(&floats(&root["stdpop"])), &spec(root, impact), 12);
    let sm = &want["statsmodels"];
    assert_eq!(fit.fit.iterations, sm["iterations"].as_u64().unwrap() as usize, "{key} IRLS iterations");
    assert!(fit.fit.converged);
    for (name, got, w, tol) in [
        ("params", &fit.fit.params, floats(&sm["params"]), 1e-9),
        ("bse", &fit.fit.bse, floats(&sm["bse"]), 1e-9),
        ("pvalues", &fit.fit.pvalues, floats(&sm["pvalues"]), 1e-9),
        ("fittedvalues", &fit.fit.fitted, floats(&sm["fittedvalues"]), 1e-6),
        ("resid_deviance", &fit.fit.resid_deviance, floats(&sm["resid_deviance"]), 1e-8),
        ("resid_pearson", &fit.fit.resid_pearson, floats(&sm["resid_pearson"]), 1e-8),
    ] {
        let d = maxdev(got, &w);
        println!("{key} statsmodels {name} maxdev {d:.3e}");
        assert!(d < tol, "{key} {name} maxdev {d}");
    }
    assert!((fit.fit.scale - sm["scale"].as_f64().unwrap()).abs() < 1e-9, "{key} scale");
    for (name, got, w) in [
        ("standardised", &fit.standardised, floats(&sm["standardised"])),
        ("standardised_counterfactual", &fit.standardised_counterfactual, floats(&sm["standardised_counterfactual"])),
        ("deseasonalised", fit.deseasonalised.as_ref().unwrap(), floats(&sm["deseasonalised"])),
        ("deseasonalised_counterfactual", fit.deseasonalised_counterfactual.as_ref().unwrap(), floats(&sm["deseasonalised_counterfactual"])),
    ] {
        let d = maxdev(got, &w);
        println!("{key} curves {name} maxdev {d:.3e}");
        assert!(d < 1e-6, "{key} {name} maxdev {d}");
    }
    assert!((fit.fit.deviance - sm["deviance"].as_f64().unwrap()).abs() < 1e-7, "{key} deviance");

    // The paper's own R output: coefficients, quasi-Poisson dispersion, and ci.lin's rate ratios.
    let r = &want["r"];
    let rr: Vec<f64> = fit.rate_ratios.iter().map(|t| t.0).collect();
    let lo: Vec<f64> = fit.rate_ratios.iter().map(|t| t.1).collect();
    let hi: Vec<f64> = fit.rate_ratios.iter().map(|t| t.2).collect();
    for (name, got, w, tol) in [
        ("coefficients", &fit.fit.params, floats(&r["coefficients"]), 1e-9),
        ("standardErrors", &fit.fit.bse, floats(&r["standardErrors"]), 1e-7),
        ("rateRatios", &rr, floats(&r["rateRatios"]), 1e-9),
        ("rateRatioLower", &lo, floats(&r["rateRatioLower"]), 1e-7),
        ("rateRatioUpper", &hi, floats(&r["rateRatioUpper"]), 1e-7),
        ("fitted", &fit.fitted, floats(&r["fitted"]), 1e-6),
        ("devianceResiduals", &fit.fit.resid_deviance, floats(&r["devianceResiduals"]), 1e-7),
    ] {
        let d = maxdev(got, &w);
        println!("{key} R {name} maxdev {d:.3e}");
        assert!(d < tol, "{key} R {name} maxdev {d}");
    }
    // R's glm stops on a relative deviance change of 1e-8 where statsmodels stops on an absolute
    // one, so the two final IRLS states differ in the sixth figure of the dispersion; the standard
    // errors carry the same difference at 1e-8.
    assert!((fit.fit.scale - r["dispersion"].as_f64().unwrap()).abs() < 1e-5, "{key} R dispersion");
}

#[test]
fn model3_step_change_matches_statsmodels_and_r() {
    let root = fixture();
    check_count(&root, "model3", ImpactModel::Level);
    // exp(coef(model3)["time"] * 12), the paper's yearly trend.
    let fit = fit_count(&floats(&root["aces"]), Some(&floats(&root["stdpop"])), &spec(&root, ImpactModel::Level), 12);
    let trend = (fit.fit.params[1] * 12.0).exp();
    assert!((trend - root["trend_per_year_model3"].as_f64().unwrap()).abs() < 1e-9, "trend per year");
}

#[test]
fn model4_slope_change_matches_statsmodels_and_r() {
    let root = fixture();
    check_count(&root, "model4", ImpactModel::LevelAndSlope);
}

fn check_rate(root: &Value, key: &str, impact: ImpactModel) {
    let want = &root[key];
    let fit = fit_continuous(&floats(&root["rate"]), &spec(root, impact), 2, 12);
    let conf: Vec<Vec<f64>> = serde_json::from_value(want["conf_int"].clone()).unwrap();
    let lo: Vec<f64> = fit.fit.conf_int.iter().map(|c| c.0).collect();
    let hi: Vec<f64> = fit.fit.conf_int.iter().map(|c| c.1).collect();
    for (name, got, w, tol) in [
        ("params", &fit.fit.params, floats(&want["params"]), 1e-8),
        ("bse", &fit.fit.bse, floats(&want["bse"]), 1e-8),
        ("pvalues", &fit.fit.pvalues, floats(&want["pvalues"]), 1e-8),
        ("conf_int lower", &lo, conf.iter().map(|c| c[0]).collect(), 1e-7),
        ("conf_int upper", &hi, conf.iter().map(|c| c[1]).collect(), 1e-7),
        ("resid", &fit.fit.resid, floats(&want["resid"]), 1e-7),
    ] {
        let d = maxdev(got, &w);
        println!("{key} HAC {name} maxdev {d:.3e}");
        assert!(d < tol, "{key} {name} maxdev {d}");
    }
    assert!((fit.fit.rsquared - want["rsquared"].as_f64().unwrap()).abs() < 1e-10);
    // Fitted plus residual is the series; the counterfactual differs from the fit only after the event.
    let rate = floats(&root["rate"]);
    for r in 0..rate.len() {
        assert!((fit.fitted[r] + fit.fit.resid[r] - rate[r]).abs() < 1e-8);
        if r < 36 { assert!((fit.fitted[r] - fit.counterfactual[r]).abs() < 1e-10) }
    }
}

#[test]
fn rate_models_match_statsmodels_hac() {
    let root = fixture();
    check_rate(&root, "rate_model3_hac2", ImpactModel::Level);
    check_rate(&root, "rate_model4_hac2", ImpactModel::LevelAndSlope);
}

#[test]
fn lag_and_temporary_shapes() {
    let spec = InterruptedSeriesDesign { intervention_row: 10, lag: 2, impact: ImpactModel::TemporaryLevel { until: 15 }, harmonics: None };
    let d = design(20, &spec);
    assert_eq!(d.names, ["const", "time", "step"]);
    let step: Vec<f64> = (0..20).map(|r| d.x[(r, 2)]).collect();
    assert_eq!(step, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let slope = design(20, &InterruptedSeriesDesign { intervention_row: 10, lag: 2, impact: ImpactModel::Slope, harmonics: None });
    assert_eq!(slope.names, ["const", "time", "slope_change"]);
    let change: Vec<f64> = (0..20).map(|r| slope.x[(r, 2)]).collect();
    assert_eq!(&change[10..16], [0.0, 0.0, 1.0, 2.0, 3.0, 4.0]);
}

/// The step-change design with AR(1) errors reaches statsmodels' SARIMAX fit on the same
/// columns (`oracle/arma_regression.py`, `x3_ar1`): the regression mean is what the paper's
/// figures draw, and the serial-correlation tests read the standardised innovations.
#[test]
fn rate_model3_with_ar1_errors_matches_statsmodels_sarimax() {
    use hirmos_causal_core::arma_regression::{ArmaOrder, DEFAULT_MAX_ITER};
    let root = fixture();
    let arma: Value = serde_json::from_str(include_str!("../oracle/fixtures/arma_regression.json")).unwrap();
    let want = &arma["x3_ar1"];
    let fit = fit_continuous_arma(&floats(&root["rate"]), &spec(&root, ImpactModel::Level), ArmaOrder { p: 1, q: 0 }, DEFAULT_MAX_ITER, 12);
    assert_eq!(fit.names, ["const", "time", "step", "sin1", "sin2", "cos1", "cos2"]);
    // The design's phase is the row index rather than the month, the same angles to rounding;
    // the optimiser's path along the ridge shared by the constant, the AR term and the variance
    // then ends within its tolerance of the library's point (see tests/arma_regression.rs).
    let relative = |got: &[f64], want: &[f64]| got.iter().zip(want).map(|(a, b)| (a - b).abs() / b.abs().max(1.0)).fold(0.0f64, f64::max);
    let params = floats(&want["params"]);
    assert!(relative(&fit.fit.params, &params) < 2e-3, "params {:?} vs {:?}", fit.fit.params, params);
    assert!(relative(&fit.fit.bse, &floats(&want["bse"])) < 5e-3, "bse");
    assert!((fit.fit.llf - want["llf"].as_f64().unwrap()).abs() < 1e-6, "llf");
    assert!(fit.fit.converged);
    let standardized = floats(&want["standardized_forecasts_error"]);
    assert!(relative(&fit.fit.standardized_resid, &standardized) < 1e-3, "standardised residuals");
    // The mean function at the fitted coefficients, with the step at zero for the counterfactual.
    let x: Vec<Vec<f64>> = serde_json::from_value(arma["x3"].clone()).unwrap();
    for (r, row) in x.iter().enumerate() {
        let mean: f64 = row.iter().zip(&fit.fit.params).map(|(a, b)| a * b).sum();
        assert!((fit.fitted[r] - mean).abs() < 1e-9);
        let counterfactual = mean - row[2] * fit.fit.params[2];
        assert!((fit.counterfactual[r] - counterfactual).abs() < 1e-9);
    }
    assert_eq!(fit.ljung_box.0.len(), 12);
}
