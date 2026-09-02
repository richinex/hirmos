// Parity against tigramite RPCMCI with a fixed seed: regimes and per-regime causal results.
use hirmos_causal_core::{run_rpcmci, run_rpcmci_frame, CiKind, RpcmciResult, TigramiteFrame};
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

fn run_case(data: &[Vec<f64>], case: &Value) -> RpcmciResult {
    run_rpcmci(
        data,
        case["num_regimes"].as_u64().unwrap() as usize,
        case["max_transitions"].as_u64().unwrap() as usize,
        case["switch_thres"].as_f64().unwrap(),
        case["num_iterations"].as_u64().unwrap() as usize,
        case["max_anneal"].as_u64().unwrap() as usize,
        case["tau_min"].as_u64().unwrap() as usize,
        case["tau_max"].as_u64().unwrap() as usize,
        case["pc_alpha"].as_f64().unwrap(),
        case["alpha_level"].as_f64().unwrap(),
        case["seed"].as_u64().unwrap(),
        CiKind::ParCorr,
    )
    .expect("rpcmci converged")
}

fn assert_convergence(label: &str, result: &RpcmciResult, case: &Value) {
    let all: Vec<Option<Vec<f64>>> = serde_json::from_value(case["diff_g_all"].clone()).unwrap();
    assert_eq!(result.diff_g_f.0.len(), all.len(), "{label} histories");
    for (annealing, (got, want)) in result.diff_g_f.0.iter().zip(&all).enumerate() {
        match (got, want) {
            (None, None) => {}
            (Some(got), Some(want)) => {
                assert_eq!(got.len(), want.len(), "{label} annealing {annealing}");
                for (iteration, (&got, &want)) in got.iter().zip(want).enumerate() {
                    close(
                        &format!("{label} diff_g[{annealing}][{iteration}]"),
                        got,
                        want,
                    );
                }
            }
            _ => panic!("{label} annealing {annealing} failure status differs"),
        }
    }
    let best: Vec<f64> = serde_json::from_value(case["diff_g_best"].clone()).unwrap();
    assert_eq!(result.diff_g_f.1.len(), best.len(), "{label} best history");
    for (iteration, (&got, &want)) in result.diff_g_f.1.iter().zip(&best).enumerate() {
        close(&format!("{label} best diff_g[{iteration}]"), got, want);
    }
}

fn assert_outputs(label: &str, result: &RpcmciResult, data: &[Vec<f64>], case: &Value) {
    assert_eq!(
        result.error_free_annealings,
        case["error_free_annealings"].as_u64().unwrap() as usize,
        "{label} error-free annealings"
    );
    let regimes: Vec<Vec<f64>> = serde_json::from_value(case["regimes"].clone()).unwrap();
    for k in 0..regimes.len() {
        for t in 0..regimes[0].len() {
            close(
                &format!("{label} gamma[{k}][{t}]"),
                result.regimes[k][t],
                regimes[k][t],
            );
        }
    }
    let tau_max = case["tau_max"].as_u64().unwrap() as usize;
    let n = data[0].len();
    for k in 0..regimes.len() {
        assert!(
            case["conf_matrices"][k.to_string()].is_null(),
            "{label} oracle unexpectedly requested confidence estimation"
        );
        assert!(
            result.conf_matrices[k].is_none(),
            "{label} regime {k} confidence matrix"
        );
        let graphs: Vec<Vec<Vec<String>>> =
            serde_json::from_value(case["graphs"][k.to_string()].clone()).unwrap();
        let vals: Vec<Vec<Vec<f64>>> =
            serde_json::from_value(case["val_matrices"][k.to_string()].clone()).unwrap();
        let ps: Vec<Vec<Vec<f64>>> =
            serde_json::from_value(case["p_matrices"][k.to_string()].clone()).unwrap();
        for i in 0..n {
            for j in 0..n {
                for tau in 0..=tau_max {
                    assert_eq!(
                        result.graphs[k][i][j][tau], graphs[i][j][tau],
                        "{label} regime {k} graph[{i}][{j}][{tau}]"
                    );
                    close(
                        &format!("{label} regime {k} val[{i}][{j}][{tau}]"),
                        result.val_matrices[k][i][j][tau],
                        vals[i][j][tau],
                    );
                    close(
                        &format!("{label} regime {k} p[{i}][{j}][{tau}]"),
                        result.p_matrices[k][i][j][tau],
                        ps[i][j][tau],
                    );
                }
            }
        }
    }
    assert_convergence(label, result, case);
}

#[test]
fn rpcmci_matches_tigramite() {
    let fx: Value = serde_json::from_str(include_str!("../oracle/fixtures/rpcmci.json")).unwrap();
    let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
    let result = run_case(&data, &fx);
    assert_outputs("fixed", &result, &data, &fx);
    println!("rpcmci: outputs and convergence histories all match");
}

#[test]
fn rpcmci_tau_min_matches_tigramite_pc1_and_mci() {
    let fx: Value = serde_json::from_str(include_str!("../oracle/fixtures/rpcmci.json")).unwrap();
    let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
    let case = &fx["tau_min_case"];
    let result = run_case(&data, case);
    assert_outputs("tau_min=2", &result, &data, case);
}

#[test]
fn rpcmci_tau_zero_matches_tigramite_contemporaneous_mci() {
    let fx: Value = serde_json::from_str(include_str!("../oracle/fixtures/rpcmci.json")).unwrap();
    let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
    let case = &fx["tau_zero_case"];
    let result = run_case(&data, case);
    assert_outputs("tau_min=0", &result, &data, case);
}

#[test]
fn rpcmci_exposed_ci_kinds_match_tigramite() {
    let fx: Value = serde_json::from_str(include_str!("../oracle/fixtures/rpcmci.json")).unwrap();
    let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
    for (name, kind) in [
        ("robust_parcorr", CiKind::RobustParCorr),
        ("parcorr_wls", CiKind::ParCorrWls),
    ] {
        let case = &fx["ci_cases"][name];
        let result = run_rpcmci(
            &data,
            case["num_regimes"].as_u64().unwrap() as usize,
            case["max_transitions"].as_u64().unwrap() as usize,
            case["switch_thres"].as_f64().unwrap(),
            case["num_iterations"].as_u64().unwrap() as usize,
            case["max_anneal"].as_u64().unwrap() as usize,
            case["tau_min"].as_u64().unwrap() as usize,
            case["tau_max"].as_u64().unwrap() as usize,
            case["pc_alpha"].as_f64().unwrap(),
            case["alpha_level"].as_f64().unwrap(),
            case["seed"].as_u64().unwrap(),
            kind,
        )
        .expect("rpcmci converged");
        assert_outputs(name, &result, &data, case);
    }
}

#[test]
fn rpcmci_missing_flag_matches_tigramite_failure() {
    let fx: Value = serde_json::from_str(include_str!("../oracle/fixtures/rpcmci.json")).unwrap();
    let mut data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
    let missing = &fx["missing_flag_case"];
    let time = missing["time"].as_u64().unwrap() as usize;
    let variable = missing["variable"].as_u64().unwrap() as usize;
    let flag = missing["missing_flag"].as_f64().unwrap();
    data[time][variable] = flag;
    let frame = TigramiteFrame::from_missing_flag(data, Some(flag), None).unwrap();
    let result = run_rpcmci_frame(
        &frame,
        2,
        3,
        0.05,
        1,
        1,
        1,
        2,
        0.2,
        0.05,
        7,
        CiKind::ParCorr,
    );
    assert!(missing["returns_none"].as_bool().unwrap());
    assert!(
        result.is_none(),
        "Tigramite's missing_flag path fails the run"
    );
}

#[test]
fn rpcmci_ignores_the_incoming_analysis_mask_like_tigramite() {
    let fx: Value = serde_json::from_str(include_str!("../oracle/fixtures/rpcmci.json")).unwrap();
    let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
    let mask = vec![vec![true; data[0].len()]; data.len()];
    let frame = TigramiteFrame::from_missing_flag(data.clone(), None, Some(mask)).unwrap();
    let dense = run_rpcmci(&data, 2, 3, 0.05, 2, 1, 1, 2, 0.2, 0.05, 7, CiKind::ParCorr);
    let masked = run_rpcmci_frame(
        &frame,
        2,
        3,
        0.05,
        2,
        1,
        1,
        2,
        0.2,
        0.05,
        7,
        CiKind::ParCorr,
    );
    assert_eq!(masked, dense);
}
