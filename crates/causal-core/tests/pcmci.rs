// Parity against tigramite golden fixtures: ParCorr unit cases, PC1 parent sets, and the full
// PCMCI val/p matrices on the seeded SCM.
use hirmos_causal_core::{parcorr_test, run_pcmci, TimeSeries};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct UnitCase {
    #[serde(rename = "X")]
    x: Vec<(usize, i32)>,
    #[serde(rename = "Y")]
    y: Vec<(usize, i32)>,
    #[serde(rename = "Z")]
    z: Vec<(usize, i32)>,
    val: f64,
    pval: f64,
}

#[derive(Deserialize)]
struct Fixture {
    data: Vec<Vec<f64>>,
    tau_max: usize,
    pc_alpha: f64,
    unit_cases: Vec<UnitCase>,
    pc_parents: BTreeMap<String, Vec<(usize, i32)>>,
    val_matrix: Vec<Vec<Vec<f64>>>,
    p_matrix: Vec<Vec<Vec<f64>>>,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../oracle/fixtures/pcmci.json")).expect("fixture parses")
}

fn close(label: &str, got: f64, want: f64, tol: f64) {
    assert!(
        (got - want).abs() <= tol * want.abs().max(1.0),
        "{label}: got {got}, oracle {want} (diff {})",
        (got - want).abs(),
    );
}

#[test]
fn parcorr_matches_tigramite() {
    let fx = fixture();
    let data = TimeSeries::new(fx.data);
    for (index, case) in fx.unit_cases.iter().enumerate() {
        let (val, pval) = parcorr_test(&data, &case.x, &case.y, &case.z, fx.tau_max);
        close(&format!("case {index}/val"), val, case.val, 1e-8);
        close(&format!("case {index}/pval"), pval, case.pval, 1e-8);
    }
}

#[test]
fn pcmci_matches_tigramite() {
    let fx = fixture();
    let n = fx.data[0].len();
    let data = TimeSeries::new(fx.data);
    let result = run_pcmci(&data, fx.tau_max, fx.pc_alpha);
    for j in 0..n {
        assert_eq!(
            result.parents[j],
            fx.pc_parents[&j.to_string()],
            "parents of x{j}",
        );
    }
    for i in 0..n {
        for j in 0..n {
            for tau in 0..=fx.tau_max {
                close(
                    &format!("val[{i}][{j}][{tau}]"),
                    result.val_matrix[i][j][tau],
                    fx.val_matrix[i][j][tau],
                    1e-8,
                );
                close(
                    &format!("p[{i}][{j}][{tau}]"),
                    result.p_matrix[i][j][tau],
                    fx.p_matrix[i][j][tau],
                    1e-8,
                );
            }
        }
    }
}
