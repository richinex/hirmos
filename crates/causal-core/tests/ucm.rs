// Parity for the Bayesian structural time series lane: statsmodels'
// UnobservedComponents('lltrend', use_exact_diffuse=True) and the Gibbs sampler built on it.
//
// One thing worth knowing about the original scripts: they call `sim.simulate()` with no
// `random_state`, and statsmodels then builds a fresh entropy seeded Generator on every
// call, so those chains cannot be reproduced from run to run even in Python. The oracle
// here threads one Generator through instead, which makes the chain deterministic and lets
// the port be checked draw for draw.
use hirmos_causal_core::nprandom::{Mt19937, NpRng};
use hirmos_causal_core::ucm::{
    fit_ucm, gibbs, kalman_filter, loglike, simulation_smoother, smoothed_state, start_params,
};
use serde_json::Value;

fn probe() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/ucm_probe.json")).unwrap()
}

fn maxdev(got: &[f64], want: &[f64]) -> f64 {
    got.iter()
        .zip(want)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max)
}

#[test]
fn start_params_and_diffuse_filter_match() {
    let root = probe();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let want_start: Vec<f64> = serde_json::from_value(root["start_params"].clone()).unwrap();

    // The Hodrick-Prescott start parameters, through two nested filters.
    let got = start_params(&y);
    let dev = maxdev(&got, &want_start);
    println!("start_params maxdev {dev:.3e}");
    assert!(dev <= 1e-8, "start params deviation {dev}");

    let f = kalman_filter(&want_start, &y, 1e-10);
    assert_eq!(
        f.nobs_diffuse,
        root["filter_start"]["nobs_diffuse"].as_u64().unwrap() as usize,
        "diffuse period count"
    );
    let want_llf = root["filter_start"]["llf"].as_f64().unwrap();
    println!("llf at start: dev {:.3e}", (f.llf - want_llf).abs());
    assert!((f.llf - want_llf).abs() <= 1e-10, "log likelihood");

    let want_ll: Vec<f64> =
        serde_json::from_value(root["filter_start"]["loglikeobs"].clone()).unwrap();
    let dev = maxdev(&f.loglikeobs, &want_ll);
    println!("per observation log likelihoods maxdev {dev:.3e}");
    assert!(dev <= 1e-12, "loglikeobs deviation {dev}");

    let want_v: Vec<f64> =
        serde_json::from_value(root["filter_start"]["forecasts_error"].clone()).unwrap();
    let dev = maxdev(&f.forecasts_error, &want_v);
    println!("forecast errors maxdev {dev:.3e}");
    assert!(dev <= 1e-12, "forecast error deviation {dev}");

    // The likelihood away from the start, so the surface is pinned rather than one point.
    let pts: Vec<Vec<f64>> = serde_json::from_value(root["probe_points"].clone()).unwrap();
    let want: Vec<f64> = serde_json::from_value(root["probe_loglikes"].clone()).unwrap();
    let mut worst = 0.0f64;
    for (p, w) in pts.iter().zip(&want) {
        worst = worst.max((loglike(p, &y) - w).abs());
    }
    println!("log likelihood at four probe points, maxdev {worst:.3e}");
    assert!(worst <= 1e-9, "probe likelihood deviation {worst}");
}

#[test]
fn mle_fit_matches() {
    let root = probe();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let fit = fit_ucm(&y, 100);
    let want_p: Vec<f64> = serde_json::from_value(root["fit"]["params"].clone()).unwrap();
    let want_llf = root["fit"]["llf"].as_f64().unwrap();
    let dev = maxdev(&fit.params, &want_p);
    println!(
        "fit params maxdev {dev:.3e}, llf dev {:.3e}",
        (fit.llf - want_llf).abs()
    );
    assert!(dev <= 1e-6, "fit parameter deviation {dev}");
    assert!((fit.llf - want_llf).abs() <= 1e-8, "fit log likelihood");
}

#[test]
fn smoother_matches_including_diffuse_periods() {
    let root = probe();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let start: Vec<f64> = serde_json::from_value(root["start_params"].clone()).unwrap();
    let got = smoothed_state(&start, &y);
    let want: Vec<Vec<f64>> =
        serde_json::from_value(root["smoothed_start"]["smoothed_state"].clone()).unwrap();
    let mut dev = 0.0f64;
    for (t, row) in got.iter().enumerate() {
        for i in 0..2 {
            dev = dev.max((row[i] - want[i][t]).abs());
        }
    }
    println!("smoothed state maxdev {dev:.3e} (diffuse periods included)");
    assert!(dev <= 1e-11, "smoothed state deviation {dev}");
}

#[test]
fn simulation_smoother_matches() {
    let root = probe();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let start: Vec<f64> = serde_json::from_value(root["start_params"].clone()).unwrap();
    for case in root["simulation_smoother"].as_array().unwrap() {
        let seed = case["seed"].as_u64().unwrap();

        // The draws themselves: measurement variates, then state variates, then the
        // initial state variates, all from numpy's PCG64.
        let mut rng = NpRng::seeded(seed);
        let want_md: Vec<f64> =
            serde_json::from_value(case["measurement_variates"].clone()).unwrap();
        let got_md: Vec<f64> = (0..want_md.len()).map(|_| rng.standard_normal()).collect();
        let want_sd: Vec<f64> = serde_json::from_value(case["state_variates"].clone()).unwrap();
        let got_sd: Vec<f64> = (0..want_sd.len()).map(|_| rng.standard_normal()).collect();
        assert_eq!(got_md, want_md, "measurement variates for seed {seed}");
        assert_eq!(got_sd, want_sd, "state variates for seed {seed}");

        let mut rng = NpRng::seeded(seed);
        let got = simulation_smoother(&start, &y, &mut rng);
        let want: Vec<Vec<f64>> = serde_json::from_value(case["simulated_state"].clone()).unwrap();
        let mut dev = 0.0f64;
        for (t, row) in got.iter().enumerate() {
            for i in 0..2 {
                dev = dev.max((row[i] - want[i][t]).abs());
            }
        }
        println!("seed {seed}: variates bitwise identical, simulated state maxdev {dev:.3e}");
        assert!(dev <= 1e-10, "simulated state deviation {dev}");
    }
}

#[test]
fn gibbs_chain_matches() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/bsts_gibbs.json")).unwrap();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let niter = root["niter"].as_u64().unwrap() as usize;
    let mut state_rng = NpRng::seeded(root["state_seed"].as_u64().unwrap());
    let mut ig_rng = Mt19937::seeded(root["invgamma_seed"].as_u64().unwrap() as u32);
    let res = gibbs(&y, niter, 0.001, 0.001, &mut state_rng, &mut ig_rng);

    let want_first: Vec<Vec<f64>> = serde_json::from_value(root["first_states"].clone()).unwrap();
    let mut dev = 0.0f64;
    for (t, row) in res.first_states.iter().enumerate() {
        for i in 0..2 {
            dev = dev.max((row[i] - want_first[t][i]).abs());
        }
    }
    println!("first iteration states maxdev {dev:.3e}");
    assert!(dev <= 1e-9, "first iteration state deviation {dev}");

    let want: Vec<Vec<f64>> = serde_json::from_value(root["params"].clone()).unwrap();
    let mut worst = 0.0f64;
    for (i, w) in want.iter().enumerate() {
        for j in 0..3 {
            worst = worst.max((res.params[i][j] - w[j]).abs() / w[j].abs().max(1e-12));
        }
    }
    println!("{niter} Gibbs iterations, worst relative deviation {worst:.3e}");
    assert!(worst <= 1e-6, "Gibbs chain deviation {worst}");
}
