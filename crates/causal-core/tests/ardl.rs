// Parity for the ARDL bounds test step of 805: order selection, the UECM fit, the
// cointegrating vector and the PSS bounds test.
use hirmos_causal_core::ardl::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/ardl.json")).unwrap()
}

fn floats(v: &Value) -> Vec<f64> {
    serde_json::from_value(v.clone()).unwrap()
}

#[test]
fn order_selection_matches() {
    let root = fixture();
    let (y, x) = (floats(&root["y"]), floats(&root["x"]));
    let maxlag = root["maxlag"].as_u64().unwrap() as usize;
    let sel = ardl_select_order(&y, maxlag, &x, maxlag, "aic", Trend::Ct);

    let want = root["aic_grid"].as_array().unwrap();
    assert_eq!(sel.grid.len(), want.len(), "candidate count");
    let mut worst = 0.0f64;
    for entry in want {
        let ar = entry["ar"].as_u64().unwrap() as usize;
        let dl = entry["dl"].as_u64().map(|v| v as usize);
        let got = sel
            .grid
            .iter()
            .find(|(p, q, ..)| *p == ar && *q == dl)
            .unwrap_or_else(|| panic!("candidate ar={ar} dl={dl:?} missing"));
        worst = worst.max((got.2 - entry["aic"].as_f64().unwrap()).abs());
    }
    println!(
        "{} candidates in the AIC grid, maxdev {worst:.3e}",
        want.len()
    );
    assert!(worst <= 1e-9, "AIC deviation {worst}");

    let p = sel.ar_lag.max(1);
    let q = sel.dl_lag.unwrap_or(1).max(1);
    assert_eq!(
        p,
        root["selected"]["p"].as_u64().unwrap() as usize,
        "endog order"
    );
    assert_eq!(
        q,
        root["selected"]["q"].as_u64().unwrap() as usize,
        "exog order"
    );
    println!("selected p={p}, q={q}, matching statsmodels");
}

#[test]
fn uecm_fit_matches() {
    let root = fixture();
    let (y, x) = (floats(&root["y"]), floats(&root["x"]));
    let m = uecm(
        &y,
        root["selected"]["p"].as_u64().unwrap() as usize,
        &x,
        root["selected"]["q"].as_u64().unwrap() as usize,
        Trend::Ct,
    );
    let want = &root["uecm"];
    let names = want["exog_names"].as_array().unwrap();
    assert_eq!(m.fit.params.len(), names.len(), "regressor count");
    println!(
        "regressors: {:?}",
        names
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<_>>()
    );

    let dev = |got: &[f64], want: Vec<f64>| {
        got.iter()
            .zip(&want)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f64, f64::max)
    };
    let params: Vec<f64> = m.fit.params.iter().copied().collect();
    println!(
        "params maxdev {:.3e}",
        dev(&params, floats(&want["params"]))
    );
    assert!(dev(&params, floats(&want["params"])) <= 1e-10);

    let bse: Vec<f64> = (0..m.fit.params.len())
        .map(|i| m.fit.cov_params[(i, i)].sqrt())
        .collect();
    println!("bse maxdev {:.3e}", dev(&bse, floats(&want["bse"])));
    assert!(dev(&bse, floats(&want["bse"])) <= 1e-12);

    // statsmodels reports nobs from the endog lag only, so it can exceed the design rows.
    assert_eq!(
        m.reported_nobs,
        want["nobs"].as_u64().unwrap() as usize,
        "reported nobs"
    );
    assert_eq!(
        m.reported_df_resid(),
        want["df_resid"].as_f64().unwrap(),
        "df_resid"
    );
    let resid_len: Vec<f64> = m.fit.resid.iter().copied().collect();
    assert_eq!(
        resid_len.len(),
        floats(&want["resid"]).len(),
        "residual count"
    );
    println!(
        "reported residuals maxdev {:.3e}",
        dev(&m.reported_resid, floats(&want["resid"]))
    );
    assert!(dev(&m.reported_resid, floats(&want["resid"])) <= 1e-10);
}

#[test]
fn cointegrating_vector_matches() {
    let root = fixture();
    let (y, x) = (floats(&root["y"]), floats(&root["x"]));
    let m = uecm(
        &y,
        root["selected"]["p"].as_u64().unwrap() as usize,
        &x,
        root["selected"]["q"].as_u64().unwrap() as usize,
        Trend::Ct,
    );
    let ci = m.cointegrating_vector(0.05);
    let want = &root["cointegrating_vector"];

    let mut worst: f64 = 0.0;
    for (name, (got, w)) in [
        ("params", (&ci.params, floats(&want["params"]))),
        ("bse", (&ci.bse, floats(&want["bse"]))),
    ] {
        let d = got
            .iter()
            .zip(&w)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f64, f64::max);
        println!("ci_{name} maxdev {d:.3e}");
        worst = worst.max(d);
    }
    // The entry normalised to one carries NaN in statsmodels; the rest must agree.
    for (i, w) in want["pvalues"].as_array().unwrap().iter().enumerate() {
        match w.as_f64() {
            Some(v) => worst = worst.max((ci.pvalues[i] - v).abs()),
            None => assert!(ci.tvalues[i].is_nan(), "index {i} should be NaN"),
        }
    }
    let lo = floats(&want["conf_int_lower"]);
    let hi = floats(&want["conf_int_upper"]);
    for (i, (l, h)) in ci.conf_int.iter().enumerate() {
        worst = worst.max((l - lo[i]).abs()).max((h - hi[i]).abs());
    }
    println!("ci p values and 95% intervals included, overall maxdev {worst:.3e}");
    assert!(worst <= 1e-10, "cointegrating vector deviation {worst}");

    let lr = ardl_long_run(&y, &x, root["maxlag"].as_u64().unwrap() as usize, 4);
    let w = &root["long_run"];
    println!(
        "long-run beta {:+.6} against {:+.6}, CI [{:+.6}, {:+.6}]",
        lr.beta,
        w["beta"].as_f64().unwrap(),
        lr.ci_lo,
        lr.ci_hi
    );
    assert!((lr.beta - w["beta"].as_f64().unwrap()).abs() <= 1e-10);
    assert!((lr.ci_lo - w["ci_lo"].as_f64().unwrap()).abs() <= 1e-10);
    assert!((lr.ci_hi - w["ci_hi"].as_f64().unwrap()).abs() <= 1e-10);
    assert!((lr.p_value - w["p"].as_f64().unwrap()).abs() <= 1e-12);
}

#[test]
fn bounds_test_matches() {
    let root = fixture();
    let (y, x) = (floats(&root["y"]), floats(&root["x"]));
    let m = uecm(
        &y,
        root["selected"]["p"].as_u64().unwrap() as usize,
        &x,
        root["selected"]["q"].as_u64().unwrap() as usize,
        Trend::Ct,
    );
    let bt = bounds_test(&m, 4);
    let want = &root["bounds_test"];
    println!(
        "case 4: stat {:.6} against {:.6}, p_lower {:.4e} p_upper {:.4e}",
        bt.stat,
        want["stat"].as_f64().unwrap(),
        bt.p_lower,
        bt.p_upper
    );
    assert!(
        (bt.stat - want["stat"].as_f64().unwrap()).abs() <= 1e-9,
        "statistic"
    );
    assert!(
        (bt.p_lower - want["p_lower"].as_f64().unwrap()).abs() <= 1e-14,
        "lower p value"
    );
    assert!(
        (bt.p_upper - want["p_upper"].as_f64().unwrap()).abs() <= 1e-14,
        "upper p value"
    );
    let cl = floats(&want["crit_lower"]);
    let cu = floats(&want["crit_upper"]);
    for (i, (l, u)) in bt.crit_vals.iter().enumerate() {
        assert_eq!(*l, cl[i], "lower critical value {i}");
        assert_eq!(*u, cu[i], "upper critical value {i}");
    }
    println!(
        "critical values at {:?} identical",
        hirmos_causal_core::pss_tables::CRIT_PERCENTILES
    );
}

#[test]
fn constant_trend_case_three_matches() {
    let root = fixture();
    let (y, x) = (floats(&root["y"]), floats(&root["x"]));
    let want = &root["trend_c"];
    let m = uecm(
        &y,
        want["p"].as_u64().unwrap() as usize,
        &x,
        want["q"].as_u64().unwrap() as usize,
        Trend::C,
    );
    let names = want["exog_names"].as_array().unwrap();
    assert_eq!(m.fit.params.len(), names.len(), "regressor count");
    let params: Vec<f64> = m.fit.params.iter().copied().collect();
    let d = params
        .iter()
        .zip(&floats(&want["params"]))
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max);
    let ci = m.cointegrating_vector(0.05);
    let dci = ci
        .params
        .iter()
        .zip(&floats(&want["ci_params"]))
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max);
    let bt = bounds_test(&m, 3);
    println!(
        "trend c, p=2 q=3: params maxdev {d:.3e}, ci maxdev {dci:.3e}, case 3 stat {:.6} against {:.6}",
        bt.stat,
        want["stat"].as_f64().unwrap()
    );
    assert!(d <= 1e-10 && dci <= 1e-10, "trend c deviation");
    assert!((bt.stat - want["stat"].as_f64().unwrap()).abs() <= 1e-9);
    assert!((bt.p_lower - want["p_lower"].as_f64().unwrap()).abs() <= 1e-14);
    assert!((bt.p_upper - want["p_upper"].as_f64().unwrap()).abs() <= 1e-14);
}
