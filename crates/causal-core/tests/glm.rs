// Parity for the two count estimators of 805 step 3f: the Poisson GLM by IRLS, and
// NegativeBinomialP by BFGS, including the scipy line search the latter runs through.
use hirmos_causal_core::bfgs::BfgsEvaluation;
use hirmos_causal_core::glm::*;
use nalgebra::DMatrix;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/glm.json")).unwrap()
}

fn floats(v: &Value) -> Vec<f64> {
    serde_json::from_value(v.clone()).unwrap()
}

fn design(v: &Value) -> DMatrix<f64> {
    let rows: Vec<Vec<f64>> = serde_json::from_value(v.clone()).unwrap();
    DMatrix::from_fn(rows.len(), rows[0].len(), |r, c| rows[r][c])
}

fn maxdev(got: &[f64], want: &[f64]) -> f64 {
    assert_eq!(got.len(), want.len(), "length mismatch");
    got.iter()
        .zip(want)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max)
}

fn compare_bfgs_trace(got: &[BfgsEvaluation], want: &Value, label: &str, tol: f64) {
    let trace = want.as_array().unwrap();
    assert_eq!(got.len(), trace.len(), "{label} BFGS trace length");
    let mut point_dev = 0.0f64;
    let mut value_dev = 0.0f64;
    for (index, (event, expected)) in got.iter().zip(trace).enumerate() {
        let expected_x = floats(&expected["x"]);
        match (event, expected["kind"].as_str().unwrap()) {
            (BfgsEvaluation::Function { x, value }, "f") => {
                point_dev = point_dev.max(maxdev(x, &expected_x));
                if let Some(expected_value) = expected["value"].as_f64() {
                    value_dev = value_dev.max((value - expected_value).abs());
                } else {
                    let expected_nonfinite = expected["value"].as_str().unwrap();
                    assert!(
                        (expected_nonfinite == "nan" && value.is_nan())
                            || (expected_nonfinite == "inf" && *value == f64::INFINITY)
                            || (expected_nonfinite == "-inf" && *value == f64::NEG_INFINITY),
                        "{label} BFGS trace event {index} non-finite value mismatch: \
                         Rust {value}, Python {expected_nonfinite}"
                    );
                }
            }
            (BfgsEvaluation::Gradient { x, value }, "g") => {
                point_dev = point_dev.max(maxdev(x, &expected_x));
                value_dev = value_dev.max(maxdev(value, &floats(&expected["value"])));
            }
            _ => panic!("{label} BFGS trace event {index} kind mismatch"),
        }
    }
    println!(
        "  {label}: all {} ordered BFGS evaluations match: points {point_dev:.3e}, values {value_dev:.3e}",
        trace.len()
    );
    assert!(
        point_dev <= tol,
        "{label} BFGS evaluation-point deviation {point_dev}"
    );
    assert!(
        value_dev <= tol,
        "{label} BFGS evaluation-value deviation {value_dev}"
    );
}

#[test]
fn poisson_glm_matches() {
    let root = fixture();
    let y = floats(&root["y"]);
    let x = design(&root["x"]);
    let fit = poisson_glm(&y, &x);
    let want = &root["poisson_glm"];

    assert_eq!(
        fit.iterations,
        want["iterations"].as_u64().unwrap() as usize,
        "IRLS iterations"
    );
    assert!(
        fit.converged && want["converged"].as_bool().unwrap(),
        "convergence"
    );
    println!("{} IRLS iterations, matching statsmodels", fit.iterations);

    for (name, got, w) in [
        ("params", &fit.params, floats(&want["params"])),
        ("bse", &fit.bse, floats(&want["bse"])),
        ("tvalues", &fit.tvalues, floats(&want["tvalues"])),
        ("pvalues", &fit.pvalues, floats(&want["pvalues"])),
        ("fittedvalues", &fit.fitted, floats(&want["fittedvalues"])),
    ] {
        let d = maxdev(got, &w);
        println!("  {name} maxdev {d:.3e}");
        assert!(d <= 1e-11, "{name} deviation {d}");
    }
    let dd = (fit.deviance - want["deviance"].as_f64().unwrap()).abs();
    println!(
        "  deviance dev {dd:.3e}, dev/df {:.6}",
        fit.deviance / fit.df_resid
    );
    assert!(dd <= 1e-9, "deviance deviation {dd}");
    assert_eq!(fit.df_resid, want["df_resid"].as_f64().unwrap(), "df_resid");
    assert_eq!(fit.scale, want["scale"].as_f64().unwrap(), "scale");

    // The 805 read: the incidence rate ratio and the additive effect at the mean rate.
    let irr = fit.params[1].exp();
    let mean_rate = fit.fitted.iter().sum::<f64>() / fit.fitted.len() as f64;
    println!(
        "  IRR x{irr:.6} against x{:.6}, mean rate {mean_rate:.6}",
        want["irr"].as_f64().unwrap()
    );
    assert!((irr - want["irr"].as_f64().unwrap()).abs() <= 1e-10);
    assert!((mean_rate - want["mean_rate"].as_f64().unwrap()).abs() <= 1e-10);
}

#[test]
fn negative_binomial_start_params_match() {
    let root = fixture();
    let y = floats(&root["y"]);
    let x = design(&root["x"]);
    let want = &root["poisson_mle"];
    let d = maxdev(&poisson_mle(&y, &x), &floats(&want["params"]));
    println!("preliminary Poisson MLE maxdev {d:.3e}");
    assert!(d <= 1e-10, "Poisson MLE deviation {d}");

    let fit = negative_binomial_p(&y, &x, 2.0);
    let w = &root["negative_binomial_p"];
    let d = maxdev(&fit.start_params, &floats(&w["start_params"]));
    println!(
        "BFGS start params maxdev {d:.3e} (dispersion estimate {:.8})",
        w["dispersion_estimate"].as_f64().unwrap()
    );
    assert!(d <= 1e-10, "start parameter deviation {d}");
}

#[test]
fn negative_binomial_p_matches() {
    let root = fixture();
    let y = floats(&root["y"]);
    let x = design(&root["x"]);
    let fit = negative_binomial_p(&y, &x, 2.0);
    let want = &root["negative_binomial_p"];

    // Call counts pin the stopping behavior; the ordered trace below proves that the BFGS
    // path and line search evaluated the same points. The wolfe2 fallback scipy keeps in
    // reserve was never needed.
    assert!(!fit.line_search_failed, "the wolfe1 line search failed");
    assert_eq!(
        fit.nfev,
        want["fcalls"].as_u64().unwrap() as usize,
        "function calls"
    );
    assert_eq!(
        fit.njev,
        want["gcalls"].as_u64().unwrap() as usize,
        "gradient calls"
    );
    assert!(
        fit.converged && want["warnflag"].as_u64() == Some(0),
        "convergence"
    );
    println!(
        "BFGS took {} function and {} gradient calls, the same as scipy",
        fit.nfev, fit.njev
    );

    compare_bfgs_trace(
        &fit.evaluations,
        &want["bfgs_trace"],
        "over-dispersed data",
        1e-12,
    );

    for (name, got, w) in [
        ("params", &fit.params, floats(&want["params"])),
        ("bse", &fit.bse, floats(&want["bse"])),
        ("tvalues", &fit.tvalues, floats(&want["tvalues"])),
        ("pvalues", &fit.pvalues, floats(&want["pvalues"])),
        ("fittedvalues", &fit.fitted, floats(&want["fittedvalues"])),
    ] {
        let d = maxdev(got, &w);
        println!("  {name} maxdev {d:.3e}");
        assert!(d <= 1e-9, "{name} deviation {d}");
    }
    let ld = (fit.llf - want["llf"].as_f64().unwrap()).abs();
    println!("  llf dev {ld:.3e}");
    assert!(ld <= 1e-9, "log likelihood deviation {ld}");

    let irr = fit.params[1].exp();
    let mean_rate = fit.fitted.iter().map(|v| v.exp()).sum::<f64>() / fit.fitted.len() as f64;
    println!(
        "  IRR x{irr:.6} against x{:.6}, mean rate {mean_rate:.4}",
        want["irr"].as_f64().unwrap()
    );
    assert!((irr - want["irr"].as_f64().unwrap()).abs() <= 1e-9);
    assert!((mean_rate - want["mean_rate"].as_f64().unwrap()).abs() <= 1e-8);
}

#[test]
fn second_dataset_matches() {
    let root = fixture();
    let want = &root["second"];
    let y = floats(&want["y"]);
    let x = design(&want["x"]);

    let pois = poisson_glm(&y, &x);
    let pd = maxdev(&pois.params, &floats(&want["poisson_params"]));
    let bd = maxdev(&pois.bse, &floats(&want["poisson_bse"]));
    println!("near-Poisson data: GLM params maxdev {pd:.3e}, bse maxdev {bd:.3e}");
    assert!(pd <= 1e-11 && bd <= 1e-11, "Poisson deviation");
    assert!(
        (pois.deviance - want["poisson_deviance"].as_f64().unwrap()).abs() <= 1e-9,
        "deviance"
    );

    let nb = negative_binomial_p(&y, &x, 2.0);
    assert_eq!(
        nb.nfev,
        want["nb_fcalls"].as_u64().unwrap() as usize,
        "function calls"
    );
    assert_eq!(
        nb.njev,
        want["nb_gcalls"].as_u64().unwrap() as usize,
        "gradient calls"
    );
    // Nine BFGS iterations rather than five, so the trial points accumulate further:
    // the trace still matches in length, order and kind, to eleven significant figures.
    compare_bfgs_trace(
        &nb.evaluations,
        &want["nb_bfgs_trace"],
        "near-Poisson data",
        1e-10,
    );
    let nd = maxdev(&nb.params, &floats(&want["nb_params"]));
    let nbd = maxdev(&nb.bse, &floats(&want["nb_bse"]));
    println!(
        "  NegBinP over {} BFGS calls: params maxdev {nd:.3e}, bse maxdev {nbd:.3e}",
        nb.nfev
    );
    assert!(nd <= 1e-9 && nbd <= 1e-9, "negative binomial deviation");
    assert!(
        (nb.llf - want["nb_llf"].as_f64().unwrap()).abs() <= 1e-9,
        "log likelihood"
    );
}
