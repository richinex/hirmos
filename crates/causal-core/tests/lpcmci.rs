// Parity against tigramite LPCMCI golden fixtures: exact graph strings and val/p matrices on all
// three seeded SCMs, including the one with a genuine latent confounder.
use hirmos_causal_core::lpcmci::run_lpcmci;
use hirmos_causal_core::parcorr::TimeSeries;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Fixture {
    data: Vec<Vec<f64>>,
    tau_max: usize,
    pc_alpha: f64,
    graph: Vec<Vec<Vec<String>>>,
    p_matrix: Vec<Vec<Vec<f64>>>,
    val_matrix: Vec<Vec<Vec<f64>>>,
}

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

#[test]
fn lpcmci_matches_tigramite() {
    let fixtures: BTreeMap<String, Fixture> =
        serde_json::from_str(include_str!("../oracle/fixtures/lpcmci.json"))
            .expect("fixtures parse");
    for (name, fx) in fixtures {
        let n = fx.data[0].len();
        let data = TimeSeries::new(fx.data);
        let result = run_lpcmci(&data, fx.tau_max, fx.pc_alpha);
        for i in 0..n {
            for j in 0..n {
                for tau in 0..=fx.tau_max {
                    assert_eq!(
                        result.graph[i][j][tau], fx.graph[i][j][tau],
                        "{name}: graph[{i}][{j}][{tau}]",
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
                }
            }
        }
        println!("{name}: graph, p and val all match");
    }
}
