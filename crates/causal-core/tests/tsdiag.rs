// Parity for the time series diagnostics batch against statsmodels.
use hirmos_causal_core::tsdiag::*;
use serde_json::Value;

fn maxdev(got: &[f64], want: &[f64]) -> f64 {
    got.iter()
        .zip(want)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max)
}

#[test]
fn diagnostics_match_statsmodels() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/tsdiag.json")).unwrap();
    let a: Vec<f64> = serde_json::from_value(root["a"].clone()).unwrap();
    let b: Vec<f64> = serde_json::from_value(root["b"].clone()).unwrap();

    let want: Vec<f64> = serde_json::from_value(root["acf"]["fft_false"].clone()).unwrap();
    let d = maxdev(&acf(&a, 8), &want);
    println!("acf maxdev {d:.3e}");
    assert!(d <= 1e-12, "acf deviation {d}");

    let want: Vec<f64> = serde_json::from_value(root["pacf_ywadjusted"].clone()).unwrap();
    let d = maxdev(&pacf_yw_adjusted(&a, 8), &want);
    println!("pacf maxdev {d:.3e}");
    assert!(d <= 1e-10, "pacf deviation {d}");

    let want: Vec<f64> = serde_json::from_value(root["pacf_ywmle"].clone()).unwrap();
    let pacf = pacf_yw_mle(&a, 8);
    let d = maxdev(&pacf, &want);
    println!("pacf ywmle maxdev {d:.3e}");
    assert!(d <= 1e-10, "pacf ywmle deviation {d}");

    let acf_values = acf(&a, 8);
    let (acf_limits, pacf_limits) = correlation_plot_limits(&acf_values, a.len());
    let want_acf: Vec<f64> = serde_json::from_value(root["plot_limits"]["acf"].clone()).unwrap();
    let want_pacf: Vec<f64> = serde_json::from_value(root["plot_limits"]["pacf"].clone()).unwrap();
    assert!(
        maxdev(&acf_limits, &want_acf) <= 1e-12,
        "ACF plotting limits"
    );
    assert!(
        maxdev(&pacf_limits, &want_pacf) <= 1e-15,
        "PACF plotting limits"
    );

    let (stat, pval) = ljung_box(&a, 8);
    let want_s: Vec<f64> = serde_json::from_value(root["ljungbox"]["stat"].clone()).unwrap();
    let want_p: Vec<f64> = serde_json::from_value(root["ljungbox"]["pvalue"].clone()).unwrap();
    println!(
        "ljung-box stat maxdev {:.3e}, p maxdev {:.3e}",
        maxdev(&stat, &want_s),
        maxdev(&pval, &want_p)
    );
    assert!(maxdev(&stat, &want_s) <= 1e-10, "ljung-box statistic");
    assert!(maxdev(&pval, &want_p) <= 1e-12, "ljung-box p value");

    let ar = autoreg1(&a);
    let want_p: Vec<f64> = serde_json::from_value(root["autoreg1"]["params"].clone()).unwrap();
    let want_pv: Vec<f64> = serde_json::from_value(root["autoreg1"]["pvalues"].clone()).unwrap();
    let want_r: Vec<f64> = serde_json::from_value(root["autoreg1"]["resid"].clone()).unwrap();
    println!(
        "AutoReg params maxdev {:.3e}, pvalues maxdev {:.3e}, resid maxdev {:.3e}",
        maxdev(&ar.params, &want_p),
        maxdev(&ar.pvalues, &want_pv),
        maxdev(&ar.resid, &want_r)
    );
    assert!(maxdev(&ar.params, &want_p) <= 1e-12, "AutoReg parameters");
    assert!(maxdev(&ar.resid, &want_r) <= 1e-12, "AutoReg residuals");
    assert!(maxdev(&ar.pvalues, &want_pv) <= 1e-9, "AutoReg p values");

    let gc = granger_ssr_ftest(&b, &a, 4);
    let mut worst = 0.0f64;
    for (i, (f, p)) in gc.iter().enumerate() {
        let want: Vec<f64> =
            serde_json::from_value(root["granger_ssr_ftest"][(i + 1).to_string()].clone()).unwrap();
        worst = worst.max((f - want[0]).abs()).max((p - want[1]).abs());
    }
    println!("granger ssr F test maxdev {worst:.3e}");
    assert!(worst <= 1e-9, "granger deviation {worst}");

    let endog: Vec<Vec<f64>> = serde_json::from_value(root["endog"].clone()).unwrap();
    for case in root["var_trend_c"].as_array().unwrap() {
        let p = case["lags"].as_u64().unwrap() as usize;
        let fit = var_fit_trend_c(&endog, p);
        let want_params: Vec<Vec<f64>> = serde_json::from_value(case["params"].clone()).unwrap();
        let mut pdev = 0.0f64;
        for (r, row) in want_params.iter().enumerate() {
            pdev = pdev.max(maxdev(&fit.params[r], row));
        }
        let fc = fit.forecast(&endog[endog.len() - p..]);
        let want_fc: Vec<Vec<f64>> = serde_json::from_value(case["forecast"].clone()).unwrap();
        println!(
            "VAR({p}): params maxdev {pdev:.3e}, aic dev {:.3e}, forecast maxdev {:.3e}",
            (fit.aic - case["aic"].as_f64().unwrap()).abs(),
            maxdev(&fc, &want_fc[0])
        );
        assert!(pdev <= 1e-10, "VAR({p}) parameters");
        assert!(
            (fit.aic - case["aic"].as_f64().unwrap()).abs() <= 1e-10,
            "VAR({p}) aic"
        );
        assert!(
            (fit.bic - case["bic"].as_f64().unwrap()).abs() <= 1e-10,
            "VAR({p}) bic"
        );
        assert!(maxdev(&fc, &want_fc[0]) <= 1e-10, "VAR({p}) forecast");
    }
}
