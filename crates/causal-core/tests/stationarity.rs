// Parity against statsmodels golden fixtures: same stats, p-values, lag choices and critical
// values, to tight tolerance, for every series and both regressions.
use hirmos_causal_core::{adfuller, kpss, Regression};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct TestResult {
    stat: f64,
    pvalue: f64,
    #[serde(default)]
    usedlag: Option<usize>,
    #[serde(default)]
    nobs: Option<usize>,
    #[serde(default)]
    nlags: Option<usize>,
    crit: BTreeMap<String, f64>,
}

#[derive(Deserialize)]
struct SeriesFixture {
    values: Vec<f64>,
    tests: BTreeMap<String, TestResult>,
}

fn fixtures() -> BTreeMap<String, SeriesFixture> {
    serde_json::from_str(include_str!("../oracle/fixtures/adf_kpss.json")).expect("fixtures parse")
}

fn close(label: &str, got: f64, want: f64, tol: f64) {
    let scale = want.abs().max(1.0);
    assert!(
        (got - want).abs() <= tol * scale,
        "{label}: got {got}, oracle {want} (diff {})",
        (got - want).abs(),
    );
}

#[test]
fn adf_matches_statsmodels() {
    for (name, series) in fixtures() {
        for (regression, key) in [(Regression::C, "adf_c"), (Regression::Ct, "adf_ct")] {
            let oracle = &series.tests[key];
            let got = adfuller(&series.values, regression);
            close(&format!("{name}/{key}/stat"), got.stat, oracle.stat, 1e-8);
            close(
                &format!("{name}/{key}/pvalue"),
                got.pvalue,
                oracle.pvalue,
                1e-8,
            );
            assert_eq!(got.usedlag, oracle.usedlag.unwrap(), "{name}/{key}/usedlag");
            assert_eq!(got.nobs, oracle.nobs.unwrap(), "{name}/{key}/nobs");
            for (level, index) in [("1%", 0), ("5%", 1), ("10%", 2)] {
                close(
                    &format!("{name}/{key}/crit {level}"),
                    got.crit[index],
                    oracle.crit[level],
                    1e-10,
                );
            }
        }
    }
}

#[test]
fn kpss_matches_statsmodels() {
    for (name, series) in fixtures() {
        for (regression, key) in [(Regression::C, "kpss_c"), (Regression::Ct, "kpss_ct")] {
            let oracle = &series.tests[key];
            let got = kpss(&series.values, regression);
            close(&format!("{name}/{key}/stat"), got.stat, oracle.stat, 1e-8);
            close(
                &format!("{name}/{key}/pvalue"),
                got.pvalue,
                oracle.pvalue,
                1e-8,
            );
            assert_eq!(got.nlags, oracle.nlags.unwrap(), "{name}/{key}/nlags");
            for (level, index) in [("10%", 0), ("5%", 1), ("2.5%", 2), ("1%", 3)] {
                close(
                    &format!("{name}/{key}/crit {level}"),
                    got.crit[index],
                    oracle.crit[level],
                    1e-10,
                );
            }
        }
    }
}
