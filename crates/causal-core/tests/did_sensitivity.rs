use hirmos_causal_core::{did::Normalization, did_sensitivity::*};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/did_sensitivity.json")).unwrap()
}
fn vals(v: &Value) -> Vec<f64> {
    serde_json::from_value(v.clone()).unwrap()
}
fn fit(case: &Value) -> Fit {
    let positions: Vec<usize> = serde_json::from_value(case["positions"].clone()).unwrap();
    from_predictions(
        &vals(&case["y"]),
        &vals(&case["d"]),
        &vals(&case["predictions"]["ml_g0"]),
        &vals(&case["predictions"]["ml_g1"]),
        &vals(&case["predictions"]["ml_m"]),
        if case["normalized"].as_bool().unwrap() {
            Normalization::InSample
        } else {
            Normalization::Population
        },
        Units::Subset {
            total: case["total"].as_u64().unwrap() as usize,
            positions: &positions,
        },
    )
    .unwrap()
}
fn close(got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1.0),
        "{got} vs {want}"
    );
}

#[test]
fn did_scores_elements_scenarios_and_group_time_coordinates_match_oracle() {
    for case in fixture()["cases"].as_array().unwrap() {
        println!("{}", case["name"]);
        let f = fit(case);
        close(f.estimate.coef, case["coef"].as_f64().unwrap());
        close(f.estimate.se, case["se"].as_f64().unwrap());
        close(f.sigma2, case["elements"]["sigma2"][0].as_f64().unwrap());
        close(f.nu2, case["elements"]["nu2"][0].as_f64().unwrap());
        for (name, got) in [
            ("psi_sigma2", &f.psi_sigma2),
            ("psi_nu2", &f.psi_nu2),
            ("riesz_rep", &f.riesz),
        ] {
            let want = vals(&case["elements"][name]);
            assert_eq!(got.len(), want.len());
            for (&a, b) in got.iter().zip(want) {
                close(a, b);
            }
        }
        for row in case["results"].as_array().unwrap() {
            let p = &row["scenario"];
            let s = Scenario::new(
                p["cf_y"].as_f64().unwrap(),
                p["cf_d"].as_f64().unwrap(),
                p["rho"].as_f64().unwrap(),
                p["level"].as_f64().unwrap(),
                p["null_hypothesis"].as_f64().unwrap(),
            )
            .unwrap();
            let b = f.bounds(s);
            for i in 0..2 {
                close(b.effect[i], row["effect"][i].as_f64().unwrap());
                close(b.interval[i], row["interval"][i].as_f64().unwrap());
            }
            let r = f.robustness(s);
            close(r.value, row["rv"].as_f64().unwrap());
            close(r.interval_value, row["rva"].as_f64().unwrap());
            let grid = f
                .grid(
                    &[p["cf_y"].as_f64().unwrap()],
                    &[p["cf_d"].as_f64().unwrap()],
                    s,
                )
                .unwrap();
            assert_eq!(grid[0][0].effect, b.effect);
        }
        if case["name"].as_str().unwrap().ends_with("riesz_recovery") {
            assert_eq!(
                f.riesz_estimate,
                RieszEstimate::NonOrthogonalReferenceRecovery
            );
        }
    }
}

#[test]
fn covariate_gain_benchmark_matches_doubleml() {
    let root = fixture();
    let cases = root["cases"].as_array().unwrap();
    let long = fit(&cases[0]);
    let short = fit(&cases[2]);
    let b = benchmark(&long, &short).unwrap();
    for (key, value) in [
        ("cf_y", b.cf_y),
        ("cf_d", b.cf_d),
        ("rho", b.rho),
        ("delta_theta", b.delta_theta),
    ] {
        close(value, root["benchmark"][key].as_f64().unwrap());
    }
}

#[test]
fn invalid_scenarios_and_coordinates_are_rejected() {
    for s in [
        (1., 0., 1., 0.95, 0.),
        (0., -0.1, 1., 0.95, 0.),
        (0., 0., 2., 0.95, 0.),
        (0., 0., 1., 1., 0.),
        (f64::NAN, 0., 1., 0.95, 0.),
    ] {
        assert!(Scenario::new(s.0, s.1, s.2, s.3, s.4).is_err());
    }
    let y = [0., 1., 0., 2.];
    let d = [0., 1., 0., 1.];
    let g = [0.; 4];
    let m = [0.5; 4];
    assert!(matches!(
        from_predictions(
            &y,
            &d,
            &g,
            &g,
            &m,
            Normalization::InSample,
            Units::Subset {
                total: 5,
                positions: &[0, 0, 2, 3]
            }
        ),
        Err(Error::InvalidEmbedding)
    ));
    assert!(matches!(
        from_predictions(&y, &d, &g, &g[..3], &m, Normalization::InSample, Units::All),
        Err(Error::Input(_))
    ));
}

#[test]
fn existing_linear_logistic_crossfit_matches_oracle_without_changing_att() {
    use hirmos_causal_core::{did, panel::PanelMatrices};
    use nalgebra::DMatrix;
    let root = fixture();
    for case in root["cases"].as_array().unwrap().iter().take(2) {
        let y = vals(&case["y"]);
        let d = vals(&case["d"]);
        let x: Vec<Vec<f64>> = serde_json::from_value(case["x"].clone()).unwrap();
        let n = y.len();
        let panel = PanelMatrices {
            y: DMatrix::from_fn(n, 2, |i, j| if j == 0 { 0.0 } else { y[i] }),
            w: DMatrix::from_fn(n, 2, |i, j| if j == 0 { 0.0 } else { d[i] }),
            units: (0..n).map(|i| i.to_string()).collect(),
            times: vec![0, 1],
            n0: d.iter().filter(|&&v| v == 0.0).count(),
            t0: 1,
        };
        let baseline = DMatrix::from_fn(n, 2, |i, j| x[i][j]);
        let sample = did::PairedSample::from_panel(&panel, &baseline).unwrap();
        let normalized = if case["normalized"].as_bool().unwrap() {
            Normalization::InSample
        } else {
            Normalization::Population
        };
        let plan = did::prepare(&sample, 2, 7, 0.01, normalized).unwrap();
        let original = did::fit(&plan).unwrap();
        let fitted = hirmos_causal_core::did_sensitivity::fit(&plan).unwrap();
        if case["normalized"].as_bool().unwrap() {
            let omitted = benchmark_without(&plan, &[1]).unwrap();
            close(
                omitted.values.cf_y,
                root["benchmark"]["cf_y"].as_f64().unwrap(),
            );
            close(
                omitted.values.cf_d,
                root["benchmark"]["cf_d"].as_f64().unwrap(),
            );
            close(
                omitted.values.rho,
                root["benchmark"]["rho"].as_f64().unwrap(),
            );
            close(
                omitted.values.delta_theta,
                root["benchmark"]["delta_theta"].as_f64().unwrap(),
            );
            assert_eq!(omitted.long.propensity_termination.len(), 2);
            assert_eq!(omitted.short.propensity_termination.len(), 2);
            assert!(benchmark_without(&plan, &[0, 1]).is_err());
        }
        assert_eq!(original.estimate.coef, fitted.sensitivity.estimate.coef);
        close(
            fitted.sensitivity.estimate.coef,
            case["coef"].as_f64().unwrap(),
        );
        close(
            fitted.sensitivity.sigma2,
            case["elements"]["sigma2"][0].as_f64().unwrap(),
        );
        close(
            fitted.sensitivity.nu2,
            case["elements"]["nu2"][0].as_f64().unwrap(),
        );
    }
}
