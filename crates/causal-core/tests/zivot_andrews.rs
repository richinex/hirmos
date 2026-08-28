// Parity against statsmodels zivot_andrews fixtures across the three break models.
use hirmos_causal_core::{zivot_andrews, ZaModel};
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

#[test]
fn zivot_andrews_matches_statsmodels() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/zivot_andrews.json")).unwrap();
    for fx in root.as_array().unwrap() {
        let name = fx["name"].as_str().unwrap();
        let series: Vec<f64> = serde_json::from_value(fx["series"].clone()).unwrap();
        let model = match fx["model"].as_str().unwrap() {
            "c" => ZaModel::C,
            "t" => ZaModel::T,
            _ => ZaModel::Ct,
        };
        let maxlag = fx["maxlag"].as_u64().map(|v| v as usize);
        let res = zivot_andrews(&series, maxlag, model);
        close(
            &format!("{name} stat"),
            res.stat,
            fx["stat"].as_f64().unwrap(),
        );
        close(
            &format!("{name} pval"),
            res.pvalue,
            fx["pval"].as_f64().unwrap(),
        );
        let crit: Vec<f64> = serde_json::from_value(fx["crit"].clone()).unwrap();
        for k in 0..3 {
            close(&format!("{name} crit[{k}]"), res.crit[k], crit[k]);
        }
        assert_eq!(
            res.baselags,
            fx["baselags"].as_u64().unwrap() as usize,
            "{name} baselags"
        );
        assert_eq!(
            res.bpidx,
            fx["bpidx"].as_u64().unwrap() as usize,
            "{name} bpidx"
        );
        println!("{name}: stat, p, crit, lags and break index match");
    }
}
