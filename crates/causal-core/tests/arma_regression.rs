// Parity for regression with ARMA errors against statsmodels SARIMAX(y, exog, order=(p, 0, q),
// trend='n') on the Sicily rate with the interrupted-series design: start parameters and their
// unconstrained form, the filter at the optimum, the fitted parameters, the OPG standard
// errors, and the information criteria.
use hirmos_causal_core::arma_regression::*;
use nalgebra::DMatrix;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/arma_regression.json")).unwrap()
}

fn floats(v: &Value) -> Vec<f64> {
    serde_json::from_value(v.clone()).unwrap()
}

fn matrix(v: &Value) -> DMatrix<f64> {
    let rows: Vec<Vec<f64>> = serde_json::from_value(v.clone()).unwrap();
    DMatrix::from_fn(rows.len(), rows[0].len(), |r, c| rows[r][c])
}

fn maxdev(got: &[f64], want: &[f64]) -> f64 {
    assert_eq!(got.len(), want.len(), "length mismatch");
    got.iter().zip(want).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max)
}

fn case(root: &Value, key: &str) -> (Value, DMatrix<f64>, ArmaOrder) {
    let c = root[key].clone();
    let x = matrix(&root[if key.starts_with("x3") { "x3" } else { "x1" }]);
    let order = ArmaOrder { p: c["p"].as_u64().unwrap() as usize, q: c["q"].as_u64().unwrap() as usize };
    (c, x, order)
}

#[test]
fn start_parameters_match_statsmodels() {
    let root = fixture();
    let y = floats(&root["y"]);
    for key in ["x3_ar1", "x3_ma1", "x3_arma11", "x3_ar2", "x1_arma21"] {
        let (c, x, order) = case(&root, key);
        let start = start_params(&y, &x, order);
        assert!(maxdev(&start, &floats(&c["start_params"])) < 1e-9, "{key} start_params");
        let unconstrained = untransform_params(&start, x.ncols(), order);
        assert!(maxdev(&unconstrained, &floats(&c["unconstrained_start"])) < 1e-9, "{key} untransform");
        let back = transform_params(&unconstrained, x.ncols(), order);
        assert!(maxdev(&back, &start) < 1e-12, "{key} transform round trip");
    }
}

#[test]
fn filter_at_the_optimum_matches_statsmodels() {
    let root = fixture();
    let y = floats(&root["y"]);
    for key in ["x3_ar1", "x3_ma1", "x3_arma11", "x3_ar2", "x1_arma21"] {
        let (c, x, order) = case(&root, key);
        let params = floats(&c["params"]);
        let out = kalman_filter(&params, &y, &x, order);
        assert!((out.llf - c["llf"].as_f64().unwrap()).abs() < 1e-9, "{key} llf");
        assert!(maxdev(&out.loglikeobs, &floats(&c["llf_obs"])) < 1e-10, "{key} llf_obs");
        assert!(maxdev(&out.forecasts_error, &floats(&c["resid"])) < 1e-9, "{key} resid");
        assert!(maxdev(&out.forecasts_error_cov, &floats(&c["forecasts_error_cov"])) < 1e-8, "{key} F");
    }
}

#[test]
fn stationary_covariance_matches_the_lyapunov_solution() {
    let root = fixture();
    let c = &root["x3_arma11"];
    let transition = matrix(&c["transition"]);
    let selection = floats(&c["selection"]);
    let params = floats(&c["params"]);
    let sigma2 = params[params.len() - 1];
    let r = DMatrix::from_column_slice(selection.len(), 1, &selection);
    let state_cov = &r * r.transpose() * sigma2;
    let p0 = stationary_covariance(&transition, &state_cov);
    let want = matrix(&c["initial_state_cov"]);
    for i in 0..2 {
        for j in 0..2 {
            assert!((p0[(i, j)] - want[(i, j)]).abs() < 1e-9, "P0[{i},{j}]");
        }
    }
}

// The optimiser is the crate's L-BFGS-B at scipy's settings on the same objective, gradient
// and start, and the likelihood at either run's optimum reproduces the other's to 1e-6. The
// parameters themselves agree to the tolerance the likelihood surface can resolve: on an
// error model with one coefficient the two runs take the same iterations and land within 5e-5;
// with two or three the constant, the AR terms and the variance share a flat ridge along which
// the projected gradient falls below `pgtol` at points 1e-3 apart in relative terms. That is
// the optimiser's tolerance, the same as the causal-impact port meets, not a difference in
// the model.
struct Tolerance {
    /// On each parameter, relative to `max(|value|, 1)`.
    params: f64,
    llf: f64,
    /// On each standard error, relative to `max(|value|, 1)`.
    bse: f64,
}

/// One error coefficient: the runs coincide to the optimiser's stopping tolerance.
const TIGHT: Tolerance = Tolerance { params: 5e-5, llf: 1e-7, bse: 1e-5 };
/// Two or three error coefficients: converged points along the ridge.
const RIDGE: Tolerance = Tolerance { params: 2e-3, llf: 2e-4, bse: 5e-3 };
/// A fit that stopped at statsmodels' default 50-iteration limit before converging: the two
/// paths have drifted apart along the ridge, so only the likelihood is close; the point each
/// stops at is not an optimum and the standard errors read from it differ accordingly.
const ITERATION_LIMIT: Tolerance = Tolerance { params: 1e-2, llf: 2e-4, bse: 5e-2 };
/// ARMA(2, 1) on the short design converges with its MA root at 0.999, on the invertibility
/// boundary the transform enforces: the OPG information is steep there, so the variance's
/// standard error moves by a factor of two between converged points 4e-4 apart in the MA term.
/// The scores themselves agree to 1e-10 (`per_observation_scores_match_complex_step`).
const BOUNDARY: Tolerance = Tolerance { params: 2e-3, llf: 2e-4, bse: 5e-2 };

fn relative_maxdev(got: &[f64], want: &[f64]) -> f64 {
    assert_eq!(got.len(), want.len(), "length mismatch");
    got.iter().zip(want).map(|(a, b)| (a - b).abs() / b.abs().max(1.0)).fold(0.0f64, f64::max)
}

fn check_fit(root: &Value, key: &str, tolerance: Tolerance) {
    let y = floats(&root["y"]);
    let (c, x, order) = case(root, key);
    let max_iter = c["maxiter"].as_u64().unwrap() as usize;
    let fit = fit(&y, &x, order, max_iter);
    let want = floats(&c["params"]);
    let dev = relative_maxdev(&fit.params, &want);
    assert!(dev < tolerance.params, "{key} params deviate by {dev}: got {:?} want {:?}", fit.params, want);
    assert!((fit.llf - c["llf"].as_f64().unwrap()).abs() < tolerance.llf, "{key} llf {} vs {}", fit.llf, c["llf"]);
    // The objective is the same function: the library's optimum scores the same here.
    assert!((loglike(&want, &y, &x, order) - c["llf"].as_f64().unwrap()).abs() < 1e-9, "{key} llf at the library's params");
    assert!(fit.iterations <= max_iter);
    // A run that converges on the very iteration the limit stops at reports either way.
    if c["converged"].as_bool().unwrap() {
        assert!(fit.converged, "{key} converged in statsmodels but not here");
    }
    let bse_dev = relative_maxdev(&fit.bse, &floats(&c["bse"]));
    assert!(bse_dev < tolerance.bse, "{key} bse deviate by {bse_dev}: got {:?} want {:?}", fit.bse, floats(&c["bse"]));
    let want_ci: Vec<Vec<f64>> = serde_json::from_value(c["conf_int"].clone()).unwrap();
    for (j, (lower, upper)) in fit.conf_int.iter().enumerate() {
        let slack = (tolerance.params + 2.0 * tolerance.bse) * want[j].abs().max(1.0) + 2.0 * tolerance.bse * fit.bse[j].max(1.0);
        assert!((lower - want_ci[j][0]).abs() < slack && (upper - want_ci[j][1]).abs() < slack, "{key} conf_int[{j}] ({lower}, {upper}) vs {:?}", want_ci[j]);
    }
    assert!((fit.aic - c["aic"].as_f64().unwrap()).abs() < 2.0 * tolerance.llf, "{key} aic");
    assert!((fit.bic - c["bic"].as_f64().unwrap()).abs() < 2.0 * tolerance.llf, "{key} bic");
    let want_std = floats(&c["standardized_forecasts_error"]);
    assert!(relative_maxdev(&fit.standardized_resid, &want_std) < tolerance.params * 10.0, "{key} standardized residuals");
    if c["converged"].as_bool().unwrap() {
        assert!(maxdev(&fit.pvalues, &floats(&c["pvalues"])) < tolerance.bse * 10.0, "{key} pvalues");
    }
}

#[test]
fn ar1_errors_fit_matches_statsmodels() {
    check_fit(&fixture(), "x3_ar1", TIGHT);
}

#[test]
fn ma1_errors_fit_matches_statsmodels() {
    check_fit(&fixture(), "x3_ma1", TIGHT);
}

#[test]
fn arma11_errors_fit_reaches_the_iteration_limit_as_statsmodels_does() {
    check_fit(&fixture(), "x3_arma11", ITERATION_LIMIT);
}

#[test]
fn ar2_errors_fit_reaches_the_iteration_limit_as_statsmodels_does() {
    check_fit(&fixture(), "x3_ar2", ITERATION_LIMIT);
}

#[test]
fn arma21_errors_on_the_short_design_reaches_the_iteration_limit_as_statsmodels_does() {
    check_fit(&fixture(), "x1_arma21", ITERATION_LIMIT);
}

#[test]
fn arma11_errors_fit_converges_with_more_iterations() {
    check_fit(&fixture(), "x3_arma11_converged", RIDGE);
}

#[test]
fn ar2_errors_fit_converges_with_more_iterations() {
    check_fit(&fixture(), "x3_ar2_converged", RIDGE);
}

#[test]
fn arma21_errors_fit_converges_with_more_iterations() {
    check_fit(&fixture(), "x1_arma21_converged", BOUNDARY);
}

/// The scores by central differences against statsmodels' complex-step scores at the
/// library's optimum: the OPG matrix the covariance inverts is built from these.
#[test]
fn per_observation_scores_match_complex_step() {
    let root = fixture();
    let y = floats(&root["y"]);
    for key in ["x3_ar1", "x3_ma1", "x3_arma11", "x3_ar2", "x1_arma21"] {
        let (c, x, order) = case(&root, key);
        let params = floats(&c["params"]);
        let want: Vec<Vec<f64>> = serde_json::from_value(c["score_obs"].clone()).unwrap();
        let got = score_obs(&params, &y, &x, order);
        let scale = want.iter().flatten().fold(0.0f64, |m, v| m.max(v.abs()));
        for (t, row) in want.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                assert!((got[(t, j)] - value).abs() < 1e-10 * scale, "{key} score_obs[{t},{j}] {} vs {value}", got[(t, j)]);
            }
        }
    }
}
