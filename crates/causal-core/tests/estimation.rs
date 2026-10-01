// Parity for HAC OLS, WLS, and Durbin-Watson against statsmodels fixtures.
use hirmos_causal_core::{durbin_watson, ols_cluster, ols_hac, ols_hc1, ols_two_way, ols_within, wls, SandwichOls, WithinErrors};
use nalgebra::DMatrix;
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

#[test]
fn estimation_matches_statsmodels() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/estimation.json")).unwrap();
    let fx = &root["hac"];
    let y: Vec<f64> = serde_json::from_value(fx["y"].clone()).unwrap();
    let xr: Vec<Vec<f64>> = serde_json::from_value(fx["x"].clone()).unwrap();
    let x = DMatrix::from_fn(xr.len(), xr[0].len(), |i, j| xr[i][j]);

    for case in fx["cases"].as_array().unwrap() {
        let maxlags = case["maxlags"].as_u64().unwrap() as usize;
        let r = ols_hac(&y, &x, maxlags);
        check_covariance_oracle(&r.covariance, &case["covariance"]);
        let params: Vec<f64> = serde_json::from_value(case["params"].clone()).unwrap();
        let bse: Vec<f64> = serde_json::from_value(case["bse"].clone()).unwrap();
        let pv: Vec<f64> = serde_json::from_value(case["pvalues"].clone()).unwrap();
        let ci: Vec<Vec<f64>> = serde_json::from_value(case["conf_int"].clone()).unwrap();
        for j in 0..params.len() {
            close(&format!("hac{maxlags} params[{j}]"), r.params[j], params[j]);
            close(&format!("hac{maxlags} bse[{j}]"), r.bse[j], bse[j]);
            close(&format!("hac{maxlags} p[{j}]"), r.pvalues[j], pv[j]);
            close(
                &format!("hac{maxlags} ci_lo[{j}]"),
                r.conf_int[j].0,
                ci[j][0],
            );
            close(
                &format!("hac{maxlags} ci_hi[{j}]"),
                r.conf_int[j].1,
                ci[j][1],
            );
        }
        close(
            &format!("hac{maxlags} dw"),
            durbin_watson(&r.resid),
            case["dw"].as_f64().unwrap(),
        );
        close(
            &format!("hac{maxlags} r2"),
            r.rsquared,
            case["rsquared"].as_f64().unwrap(),
        );
    }
    println!("hac + dw: all cases match");

    let fw = &root["wls"];
    let weights: Vec<f64> = serde_json::from_value(fw["weights"].clone()).unwrap();
    let r = wls(&y, &x, &weights);
    check_covariance_oracle(&r.covariance, &fw["covariance"]);
    let params: Vec<f64> = serde_json::from_value(fw["params"].clone()).unwrap();
    let pv: Vec<f64> = serde_json::from_value(fw["pvalues"].clone()).unwrap();
    for j in 0..params.len() {
        close(&format!("wls params[{j}]"), r.params[j], params[j]);
        close(&format!("wls p[{j}]"), r.pvalues[j], pv[j]);
    }
    println!("wls: params and pvalues match");
}

fn matrix(value: &Value) -> DMatrix<f64> {
    let rows: Vec<Vec<f64>> = serde_json::from_value(value.clone()).unwrap();
    DMatrix::from_fn(rows.len(), rows[0].len(), |i, j| rows[i][j])
}

fn check_covariance(covariance: &DMatrix<f64>, standard_errors: &[f64]) {
    assert_eq!(covariance.shape(), (standard_errors.len(), standard_errors.len()));
    for i in 0..standard_errors.len() {
        close("covariance diagonal", covariance[(i, i)], standard_errors[i].powi(2));
        for j in 0..standard_errors.len() {
            assert!(covariance[(i, j)].is_finite());
            close("covariance symmetry", covariance[(i, j)], covariance[(j, i)]);
        }
    }
}

fn check_covariance_oracle(actual: &DMatrix<f64>, expected: &Value) {
    let expected = matrix(expected);
    assert_eq!(actual.shape(), expected.shape());
    for i in 0..actual.nrows() { for j in 0..actual.ncols() {
        close(&format!("oracle covariance[{i},{j}]"), actual[(i,j)], expected[(i,j)]);
    }}
}

#[test]
fn covariance_retains_off_diagonals_under_reparameterization() {
    let y = [1.0, 3.0, 2.0, 7.0, 4.0, 8.0, 6.0, 11.0];
    let x = DMatrix::from_fn(y.len(), 2, |i, j| if j == 0 { 1.0 } else { i as f64 });
    // X_new = X A, so Cov(beta) = A Cov(beta_new) A'. This checks
    // off-diagonals without deriving the expectation from reported standard errors.
    let a = DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 0.0, 1.0]);
    let shifted = &x * &a;
    let weights = [1.0, 2.0, 3.0, 1.0, 4.0, 2.0, 1.0, 3.0];
    let groups = [0, 0, 1, 1, 2, 2, 3, 3];
    let w = wls(&y, &x, &weights);
    let h = ols_hac(&y, &x, 1);
    let c = ols_cluster(&y, &x, &groups);
    let r = ols_hc1(&y, &x);
    for (cov, se, transformed) in [
        (&w.covariance, &w.bse, wls(&y, &shifted, &weights).covariance),
        (&h.covariance, &h.bse, ols_hac(&y, &shifted, 1).covariance),
        (&c.covariance, &c.bse, ols_cluster(&y, &shifted, &groups).covariance),
        (&r.covariance, &r.bse, ols_hc1(&y, &shifted).covariance),
    ] {
        check_covariance(cov, se);
        assert!(cov[(0, 1)].abs() > 1e-6);
        let recovered = &a * transformed * a.transpose();
        for i in 0..2 { for j in 0..2 {
            close("transformed covariance", recovered[(i, j)], cov[(i, j)]);
        }}
    }
}

fn check_sandwich(label: &str, fit: &SandwichOls, fx: &Value) {
    check_covariance_oracle(&fit.covariance, &fx["covariance"]);
    let params: Vec<f64> = serde_json::from_value(fx["params"].clone()).unwrap();
    let bse: Vec<f64> = serde_json::from_value(fx["bse"].clone()).unwrap();
    let pv: Vec<f64> = serde_json::from_value(fx["pvalues"].clone()).unwrap();
    let ci: Vec<Vec<f64>> = serde_json::from_value(fx["conf_int"].clone()).unwrap();
    for j in 0..params.len() {
        close(&format!("{label} params[{j}]"), fit.params[j], params[j]);
        close(&format!("{label} bse[{j}]"), fit.bse[j], bse[j]);
        close(&format!("{label} p[{j}]"), fit.pvalues[j], pv[j]);
        close(&format!("{label} ci_lo[{j}]"), fit.conf_int[j].0, ci[j][0]);
        close(&format!("{label} ci_hi[{j}]"), fit.conf_int[j].1, ci[j][1]);
    }
}

#[test]
fn hc1_matches_statsmodels() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/estimation.json")).unwrap();
    let fx = &root["hc1"];
    let y: Vec<f64> = serde_json::from_value(fx["y"].clone()).unwrap();
    check_sandwich("hc1", &ols_hc1(&y, &matrix(&fx["x"])), fx);
}

#[test]
fn cluster_matches_statsmodels() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/estimation.json")).unwrap();
    let fx = &root["cluster"];
    let y: Vec<f64> = serde_json::from_value(fx["y"].clone()).unwrap();
    let groups: Vec<u64> = serde_json::from_value(fx["groups"].clone()).unwrap();
    check_sandwich("cluster", &ols_cluster(&y, &matrix(&fx["x"]), &groups), fx);
}

fn check_within(label: &str, fx: &Value, two_way: bool) {
    let y: Vec<f64> = serde_json::from_value(fx["y"].clone()).unwrap();
    let groups: Vec<u64> = serde_json::from_value(fx["groups"].clone()).unwrap();
    let x = matrix(&fx["x"]);
    let kept: Vec<usize> = serde_json::from_value(fx["kept"].clone()).unwrap();
    let times: Vec<u64> = serde_json::from_value(fx["times"].clone()).unwrap();
    for (name, errors) in [("classical", WithinErrors::Classical), ("hc1", WithinErrors::Hc1), ("cluster", WithinErrors::Cluster)] {
        let fit = if two_way { ols_two_way(&y, &x, &groups, &times, errors) } else { ols_within(&y, &x, &groups, errors) }.unwrap();
        assert_eq!(fit.kept, kept, "{label} {name} kept columns");
        check_covariance(&fit.covariance, &fit.bse);
        assert_eq!(fit.units as u64, fx["units"].as_u64().unwrap(), "{label} {name} units");
        assert_eq!(fit.periods.map(|count| count as u64), two_way.then(|| fx["periods"].as_u64().unwrap()), "{label} {name} periods");
        assert_eq!(fit.df_resid as u64, fx["df_resid"].as_u64().unwrap(), "{label} {name} df_resid");
        close(&format!("{label} {name} ssr"), fit.ssr, fx["resid_ss"].as_f64().unwrap());
        close(&format!("{label} {name} r2 within"), fit.rsquared_within, fx["rsquared_within"].as_f64().unwrap());
        let case = &fx[name];
        check_covariance_oracle(&fit.covariance, &case["covariance"]);
        let params: Vec<f64> = serde_json::from_value(case["params"].clone()).unwrap();
        let bse: Vec<f64> = serde_json::from_value(case["bse"].clone()).unwrap();
        let pv: Vec<f64> = serde_json::from_value(case["pvalues"].clone()).unwrap();
        let ci: Vec<Vec<f64>> = serde_json::from_value(case["conf_int"].clone()).unwrap();
        for j in 0..params.len() {
            close(&format!("{label} {name} params[{j}]"), fit.params[j], params[j]);
            close(&format!("{label} {name} bse[{j}]"), fit.bse[j], bse[j]);
            close(&format!("{label} {name} p[{j}]"), fit.pvalues[j], pv[j]);
            close(&format!("{label} {name} ci_lo[{j}]"), fit.conf_int[j].0, ci[j][0]);
            close(&format!("{label} {name} ci_hi[{j}]"), fit.conf_int[j].1, ci[j][1]);
        }
    }
}

#[test]
fn within_matches_linearmodels() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/within.json")).unwrap();
    check_within("balanced_absorbed", &root["balanced_absorbed"], false);
    check_within("unbalanced", &root["unbalanced"], false);
}

#[test]
fn two_way_matches_linearmodels() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/within.json")).unwrap();
    check_within("two_way_balanced", &root["two_way_balanced"], true);
    check_within("two_way_unbalanced", &root["two_way_unbalanced"], true);
}

#[test]
fn independent_clusters_and_time_effects_match_linearmodels() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/within.json")).unwrap();
    for name in ["time_only_cluster_unit", "unit_cluster_school", "unit_cluster_time", "two_way_cluster_school"] {
        let fx = &root[name];
        let y: Vec<f64> = serde_json::from_value(fx["y"].clone()).unwrap();
        let units: Vec<u64> = serde_json::from_value(fx["groups"].clone()).unwrap();
        let times: Vec<u64> = serde_json::from_value(fx["times"].clone()).unwrap();
        let clusters: Vec<u64> = serde_json::from_value(fx["clusters"].clone()).unwrap();
        let x = matrix(&fx["x"]);
        for (covariance, errors) in [("classical", WithinErrors::Classical), ("hc1", WithinErrors::Hc1), ("cluster", WithinErrors::ClusterBy(&clusters))] {
            let fit = match name {
                "time_only_cluster_unit" => ols_within(&y, &x, &times, errors),
                "two_way_cluster_school" => ols_two_way(&y, &x, &units, &times, errors),
                _ => ols_within(&y, &x, &units, errors),
            }.unwrap();
            assert_eq!(fit.df_resid as u64, fx["df_resid"].as_u64().unwrap());
            for j in 0..fit.params.len() {
                close(name, fit.params[j], fx[covariance]["params"][j].as_f64().unwrap());
                close(name, fit.bse[j], fx[covariance]["bse"][j].as_f64().unwrap());
                close(name, fit.conf_int[j].0, fx[covariance]["conf_int"][j][0].as_f64().unwrap());
                close(name, fit.conf_int[j].1, fx[covariance]["conf_int"][j][1].as_f64().unwrap());
            }
        }
    }
}

#[test]
fn within_refuses_invalid_and_saturated_designs() {
    let x = DMatrix::from_column_slice(4, 1, &[0.0, 1.0, 0.0, 1.0]);
    let y = [1.0, 2.1, 3.2, 5.0];
    assert!(ols_within(&y[..3], &x, &[0, 0, 1, 1], WithinErrors::Classical).is_err());
    assert!(ols_within(&y, &x, &[0, 0, 0, 0], WithinErrors::Classical).is_err());
    assert!(ols_within(&y, &x, &[0, 1, 2, 3], WithinErrors::Classical).is_err());
    assert!(ols_two_way(&y, &x, &[0, 0, 1, 1], &[0, 1, 2, 3], WithinErrors::Classical).is_err());
    assert!(ols_within(&y, &x, &[0, 0, 1, 1], WithinErrors::ClusterBy(&[0, 0, 0, 0])).is_err());
}
