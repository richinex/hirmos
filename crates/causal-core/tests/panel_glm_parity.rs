use nalgebra::DMatrix;
use hirmos_causal_core::{
    panel_design::Panel,
    glm,
    panel_glm::{self as regression, Family, Uncertainty},
};
use serde_json::Value;
fn fixtures() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/panel_glm.json")).unwrap()
}
fn vec(v: &Value) -> Vec<f64> {
    serde_json::from_value(v.clone()).unwrap()
}
fn matrix(v: &Value) -> DMatrix<f64> {
    let rows: Vec<Vec<f64>> = serde_json::from_value(v.clone()).unwrap();
    DMatrix::from_fn(rows.len(), rows[0].len(), |i, j| rows[i][j])
}
fn close(got: &[f64], want: &[f64], tol: f64, label: &str) {
    assert_eq!(got.len(), want.len());
    assert!(got.iter().chain(want).all(|v| v.is_finite()), "{label}: non-finite comparison");
    let error = got
        .iter()
        .zip(want)
        .map(|(a, b)| (a - b).abs() / (1.0 + b.abs()))
        .fold(0.0, f64::max);
    assert!(
        error < tol,
        "{label}: scaled error {error:e}, tolerance {tol:e}"
    );
}

#[test]
fn fits_covariances_and_joint_contrasts() {
    for case in fixtures()["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let x = matrix(&case["x"]);
        let y = vec(&case["y"]);
        let exposure = vec(&case["exposure"]);
        let family = if case["family"] == "nb2" {
            Family::NegativeBinomial {
                counts: &y,
                exposure: &exposure,
            }
        } else {
            Family::Binomial {
                successes: &y,
                trials: &exposure,
            }
        };
        let fit = regression::fit(&x, family, 1000, case["tolerance"].as_f64().unwrap()).unwrap();
        println!(
            "{name}: converged {}, iterations {}",
            fit.converged, fit.iterations
        );
        assert_eq!(
            fit.converged,
            case["converged"].as_bool().unwrap(),
            "{name}"
        );
        close(
            &fit.coefficients,
            &vec(&case["params"]),
            2e-5,
            &format!("{name} parameters"),
        );
        close(
            &fit.fitted,
            &vec(&case["fitted"]),
            2e-5,
            &format!("{name} fitted"),
        );
        close(
            fit.scores.as_slice(),
            matrix(&case["scores"]).as_slice(),
            5e-5,
            &format!("{name} scores at fitted parameters"),
        );
        let ids: Vec<u64> = serde_json::from_value(case["groups"].clone()).unwrap();
        let times: Vec<i64> = (0..y.len() as i64).collect();
        for (key, uncertainty) in [
            ("model", Uncertainty::ModelBased),
            (
                "cluster",
                Uncertainty::Cluster {
                    ids: &ids,
                    correction: true,
                },
            ),
            (
                "cluster_uncorrected",
                Uncertainty::Cluster {
                    ids: &ids,
                    correction: false,
                },
            ),
            (
                "hac",
                Uncertainty::Hac {
                    times: &times,
                    lags: 3,
                    correction: true,
                },
            ),
            (
                "hac_uncorrected",
                Uncertainty::Hac {
                    times: &times,
                    lags: 3,
                    correction: false,
                },
            ),
        ] {
            let covariance = regression::covariance(&fit, uncertainty).unwrap();
            close(
                covariance.as_slice(),
                matrix(&case["covariances"][key]).as_slice(),
                2e-5,
                &format!("{name} {key} covariance"),
            );
            let c =
                regression::contrast(&fit.coefficients, &covariance, &vec(&case["weights"]), 0.95)
                    .unwrap();
            let expected = &case["contrasts"][key];
            close(
                &[c.estimate, c.standard_error, c.lower, c.upper],
                &[
                    expected["estimate"].as_f64().unwrap(),
                    expected["se"].as_f64().unwrap(),
                    expected["interval"][0].as_f64().unwrap(),
                    expected["interval"][1].as_f64().unwrap(),
                ],
                2e-5,
                &format!("{name} {key} contrast"),
            );
        }
        if case["family"] == "nb2" {
            let params = vec(&case["params"]);
            let offset: Vec<_> = exposure.iter().map(|e| e.ln()).collect();
            let (ll, scores, hessian) = glm::negative_binomial_evaluate(&y, &x, &offset, &params);
            close(
                &[ll],
                &[case["llf"].as_f64().unwrap()],
                1e-11,
                &format!("{name} fixed likelihood"),
            );
            close(
                scores.as_slice(),
                matrix(&case["scores"]).as_slice(),
                1e-10,
                &format!("{name} fixed scores"),
            );
            close(
                hessian.as_slice(),
                matrix(&case["hessian"]).as_slice(),
                1e-9,
                &format!("{name} fixed Hessian"),
            );
        }
    }
}

#[test]
fn original_count_fixtures_stay_unchanged() {
    let f: Value = serde_json::from_str(include_str!("../oracle/fixtures/glm.json")).unwrap();
    let y = vec(&f["y"]);
    let x = matrix(&f["x"]);
    let nb = glm::negative_binomial_p(&y, &x, 2.0);
    let expected = &f["negative_binomial_p"];
    close(
        &nb.params,
        &vec(&expected["params"]),
        1e-9,
        "legacy NB parameters",
    );
    close(&nb.bse, &vec(&expected["bse"]), 1e-9, "legacy NB errors");
    assert_eq!(nb.nfev, expected["fcalls"].as_u64().unwrap() as usize);
    let poisson = glm::poisson_glm(&y, &x);
    close(
        &poisson.params,
        &vec(&f["poisson_glm"]["params"]),
        1e-10,
        "legacy Poisson",
    );
}

#[test]
fn dummy_design_matches_independent_numpy_construction() {
    let fixture = fixtures();
    let f = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "nb2_team_week")
        .unwrap();
    let x = matrix(&f["x"]);
    let keys: Vec<_> = (0..12u64)
        .flat_map(|u| (0..18i64).map(move |t| (u, t)))
        .collect();
    let raw = DMatrix::from_fn(keys.len(), 2, |i, j| x[(i, j + 1)]);
    let p = Panel::new(&keys, raw).unwrap();
    let d = p
        .distributed_lags(&[(0, 0, "x1".into()), (1, 0, "x2".into())])
        .unwrap();
    assert_eq!(d.matrix, x);
}

#[test]
fn lag_and_cohort_designs_match_oracle_before_fitting() {
    for case in fixtures()["cases"].as_array().unwrap() {
        let Some(spec) = case.get("design") else {
            continue;
        };
        let keys: Vec<(u64, i64)> = serde_json::from_value(spec["keys"].clone()).unwrap();
        let panel = Panel::new(&keys, matrix(&spec["values"])).unwrap();
        let design = match spec["kind"].as_str().unwrap() {
            "lags" => panel
                .distributed_lags(
                    &serde_json::from_value::<Vec<(usize, usize, String)>>(spec["columns"].clone())
                        .unwrap(),
                )
                .unwrap(),
            "events" => panel
                .cohort_events(
                    &serde_json::from_value(spec["adoption"].clone()).unwrap(),
                    spec["cohort"].as_i64().unwrap(),
                    &[],
                )
                .unwrap(),
            "summary" => panel
                .cohort_summary(
                    &serde_json::from_value(spec["adoption"].clone()).unwrap(),
                    spec["cohort"].as_i64().unwrap(),
                    &[],
                )
                .unwrap(),
            "window" => panel
                .cohort_events_in_window(
                    &serde_json::from_value(spec["adoption"].clone()).unwrap(),
                    spec["cohort"].as_i64().unwrap(),
                    &[],
                    spec["first"].as_i64().unwrap(),
                    spec["last"].as_i64().unwrap(),
                )
                .unwrap(),
            other => panic!("unknown fixture design {other}"),
        };
        assert_eq!(design.matrix, matrix(&case["x"]), "{}", case["name"]);
        let groups: Vec<u64> = serde_json::from_value(case["groups"].clone()).unwrap();
        assert_eq!(
            design.keys.iter().map(|(u, _)| *u).collect::<Vec<_>>(),
            groups
        );
        for (key, row) in design.keys.iter().zip(&design.original_rows) {
            assert_eq!(*key, keys[*row]);
        }
        let mut partition = design.original_rows.clone();
        partition.extend(&design.omitted_rows);
        partition.sort_unstable();
        assert_eq!(partition, (0..keys.len()).collect::<Vec<_>>());
    }
}
