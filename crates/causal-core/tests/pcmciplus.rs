// Parity against tigramite run_pcmciplus golden fixtures at 902 defaults, with fdr_bh q-values.
use hirmos_causal_core::{fdr_bh, run_pcmciplus, CiKind, TimeSeries};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Fixture {
    data: Vec<Vec<f64>>,
    tau_max: usize,
    ci: String,
    pc_alpha: f64,
    graph: Vec<Vec<Vec<String>>>,
    p_matrix: Vec<Vec<Vec<f64>>>,
    val_matrix: Vec<Vec<Vec<f64>>>,
    q_matrix: Vec<Vec<Vec<f64>>>,
}

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

#[test]
fn pcmciplus_matches_tigramite() {
    let fixtures: BTreeMap<String, Fixture> =
        serde_json::from_str(include_str!("../oracle/fixtures/pcmciplus.json")).unwrap();
    for (name, fx) in fixtures {
        let n = fx.data[0].len();
        let kind = if fx.ci == "parcorr" {
            CiKind::ParCorr
        } else {
            CiKind::RobustParCorr
        };
        let data = TimeSeries::new(fx.data);
        let result = run_pcmciplus(&data, fx.tau_max, fx.pc_alpha, kind);
        let q = fdr_bh(&result.p_matrix, false);
        for i in 0..n {
            for j in 0..n {
                for tau in 0..=fx.tau_max {
                    assert_eq!(
                        result.graph[i][j][tau], fx.graph[i][j][tau],
                        "{name}: graph[{i}][{j}][{tau}]"
                    );
                    close(
                        &format!("{name}: p[{i}][{j}][{tau}]"),
                        result.p_matrix[i][j][tau],
                        fx.p_matrix[i][j][tau],
                    );
                    close(
                        &format!("{name}: val[{i}][{j}][{tau}]"),
                        result.val_matrix[i][j][tau],
                        fx.val_matrix[i][j][tau],
                    );
                    close(
                        &format!("{name}: q[{i}][{j}][{tau}]"),
                        q[i][j][tau],
                        fx.q_matrix[i][j][tau],
                    );
                }
            }
        }
        println!("{name}: graph, p, val and q all match");
    }
}
