//! Thin browser command façade over the parity-tested causal kernels.
//!
//! The façade owns browser-facing validation and serialization. It does not reinterpret test
//! evidence or copy the numerical implementations out of the Hirmos causal core.

use hirmos_causal_core::dynotears::dynotears_with_progress;
use hirmos_causal_core::lpcmci::run_lpcmci_with_progress;
use hirmos_causal_core::ocse::{discover_network_with_progress, CmiMethod};
use hirmos_causal_core::parcorr::{CiKind, TimeSeries};
use hirmos_causal_core::pcmciplus::run_pcmciplus;
use hirmos_causal_core::tsdiag::granger_ssr_ftest;
use hirmos_causal_core::{
    adfuller, kpss, zivot_andrews, AdfResult, KpssResult, Regression, ZaModel, ZaResult,
};
use nalgebra::DMatrix;
use serde::Serialize;
use wasm_bindgen::prelude::*;

const MIN_STATIONARITY_OBSERVATIONS: usize = 24;

#[derive(serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
enum AnalysisCommand {
    StationarityBattery,
    PcmciPlus {
        rows: usize,
        columns: usize,
        tau_max: usize,
        pc_alpha: f64,
    },
    Lpcmci {
        rows: usize,
        columns: usize,
        tau_max: usize,
        pc_alpha: f64,
    },
    Dynotears {
        rows: usize,
        columns: usize,
        max_lag: usize,
        lambda_w: f64,
        lambda_a: f64,
    },
    Ocse {
        rows: usize,
        columns: usize,
        max_lag: usize,
        alpha: f64,
        n_shuffles: usize,
        method: OcseMethod,
        k: usize,
    },
    GrangerSsrF {
        rows: usize,
        max_lag: usize,
    },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum OcseMethod {
    Gaussian,
    Knn,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AdfEvidence {
    statistic: f64,
    p_value: f64,
    used_lag: usize,
    observations: usize,
    critical_values: [f64; 3],
}

impl From<AdfResult> for AdfEvidence {
    fn from(value: AdfResult) -> Self {
        Self {
            statistic: value.stat,
            p_value: value.pvalue,
            used_lag: value.usedlag,
            observations: value.nobs,
            critical_values: value.crit,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KpssEvidence {
    statistic: f64,
    p_value: f64,
    used_lag: usize,
    critical_values: [f64; 4],
}

impl From<KpssResult> for KpssEvidence {
    fn from(value: KpssResult) -> Self {
        Self {
            statistic: value.stat,
            p_value: value.pvalue,
            used_lag: value.nlags,
            critical_values: value.crit,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ZivotAndrewsEvidence {
    statistic: f64,
    p_value: f64,
    critical_values: [f64; 3],
    base_lags: usize,
    break_index: usize,
}

impl From<ZaResult> for ZivotAndrewsEvidence {
    fn from(value: ZaResult) -> Self {
        Self {
            statistic: value.stat,
            p_value: value.pvalue,
            critical_values: value.crit,
            base_lags: value.baselags,
            break_index: value.bpidx,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeterministicEvidence<T> {
    constant: T,
    constant_and_trend: T,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BreakEvidence {
    level: ZivotAndrewsEvidence,
    trend: ZivotAndrewsEvidence,
    level_and_trend: ZivotAndrewsEvidence,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GrangerLagEvidence {
    lag: usize,
    statistic: f64,
    p_value: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OcseEdgeEvidence {
    source: usize,
    target: usize,
    lag: usize,
    cmi: f64,
    p_value: f64,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum AnalysisResult {
    StationarityBattery {
        observations: usize,
        adf: DeterministicEvidence<AdfEvidence>,
        kpss: DeterministicEvidence<KpssEvidence>,
        zivot_andrews: BreakEvidence,
    },
    PcmciPlus {
        observations: usize,
        variables: usize,
        tau_max: usize,
        pc_alpha: f64,
        graph: Vec<Vec<Vec<String>>>,
        p_matrix: Vec<Vec<Vec<f64>>>,
        val_matrix: Vec<Vec<Vec<f64>>>,
    },
    Lpcmci {
        observations: usize,
        variables: usize,
        tau_max: usize,
        pc_alpha: f64,
        graph: Vec<Vec<Vec<String>>>,
        p_matrix: Vec<Vec<Vec<f64>>>,
        val_matrix: Vec<Vec<Vec<f64>>>,
    },
    Dynotears {
        observations: usize,
        variables: usize,
        max_lag: usize,
        lambda_w: f64,
        lambda_a: f64,
        contemporaneous_weights: Vec<Vec<f64>>,
        lagged_weights: Vec<Vec<Vec<f64>>>,
    },
    Ocse {
        observations: usize,
        variables: usize,
        max_lag: usize,
        alpha: f64,
        n_shuffles: usize,
        method: OcseMethod,
        k: usize,
        seed: u64,
        edges: Vec<OcseEdgeEvidence>,
    },
    GrangerSsrF {
        observations: usize,
        max_lag: usize,
        tests: Vec<GrangerLagEvidence>,
    },
}

fn validate_dense_matrix(
    label: &str,
    values: &[f64],
    rows: usize,
    columns: usize,
) -> Result<(), String> {
    let expected = rows
        .checked_mul(columns)
        .ok_or_else(|| format!("{label} matrix dimensions overflowed"))?;
    if values.len() != expected {
        return Err(format!(
            "{label} received {} values for a {rows} by {columns} matrix",
            values.len()
        ));
    }
    if values.iter().any(|value| !value.is_finite()) {
        return Err(format!(
            "{label} requires finite dense values; resolve missingness first"
        ));
    }
    Ok(())
}

fn time_series_from_column_major(values: &[f64], rows: usize, columns: usize) -> TimeSeries {
    let mut row_major = vec![0.0; rows * columns];
    for column in 0..columns {
        for row in 0..rows {
            row_major[row * columns + column] = values[column * rows + row];
        }
    }
    TimeSeries {
        values: row_major,
        t: rows,
        n: columns,
    }
}

fn validate_stationarity_values(values: &[f64]) -> Result<(), String> {
    if values.len() < MIN_STATIONARITY_OBSERVATIONS {
        return Err(format!(
            "stationarity battery requires at least {MIN_STATIONARITY_OBSERVATIONS} observations"
        ));
    }
    if values.iter().any(|value| !value.is_finite()) {
        return Err("stationarity battery requires finite dense values".to_owned());
    }
    let first = values[0];
    if values.iter().all(|value| *value == first) {
        return Err("stationarity battery is undefined for a constant series".to_owned());
    }
    Ok(())
}

fn stationarity_battery(values: &[f64]) -> Result<AnalysisResult, String> {
    validate_stationarity_values(values)?;
    Ok(AnalysisResult::StationarityBattery {
        observations: values.len(),
        adf: DeterministicEvidence {
            constant: adfuller(values, Regression::C).into(),
            constant_and_trend: adfuller(values, Regression::Ct).into(),
        },
        kpss: DeterministicEvidence {
            constant: kpss(values, Regression::C).into(),
            constant_and_trend: kpss(values, Regression::Ct).into(),
        },
        zivot_andrews: BreakEvidence {
            level: zivot_andrews(values, None, ZaModel::C).into(),
            trend: zivot_andrews(values, None, ZaModel::T).into(),
            level_and_trend: zivot_andrews(values, None, ZaModel::Ct).into(),
        },
    })
}

fn pcmci_plus(
    values: &[f64],
    rows: usize,
    columns: usize,
    tau_max: usize,
    pc_alpha: f64,
) -> Result<AnalysisResult, String> {
    if !(2..=32).contains(&columns) {
        return Err("PCMCI+ requires between 2 and 32 selected variables".to_owned());
    }
    if !(1..=20).contains(&tau_max) {
        return Err("PCMCI+ tauMax must be between 1 and 20".to_owned());
    }
    if !pc_alpha.is_finite() || !(0.0..=1.0).contains(&pc_alpha) || pc_alpha == 0.0 {
        return Err("PCMCI+ pcAlpha must be finite and in (0, 1]".to_owned());
    }
    validate_dense_matrix("PCMCI+", values, rows, columns)?;
    if rows < (2 * tau_max + 16).max(24) {
        return Err("PCMCI+ has too few observations for the selected maximum lag".to_owned());
    }
    let data = time_series_from_column_major(values, rows, columns);
    let result = run_pcmciplus(&data, tau_max, pc_alpha, CiKind::ParCorr);
    Ok(AnalysisResult::PcmciPlus {
        observations: rows,
        variables: columns,
        tau_max,
        pc_alpha,
        graph: result.graph,
        p_matrix: result.p_matrix,
        val_matrix: result.val_matrix,
    })
}

fn lpcmci_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    tau_max: usize,
    pc_alpha: f64,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=32).contains(&columns) {
        return Err("LPCMCI requires between 2 and 32 selected variables".to_owned());
    }
    if !(1..=20).contains(&tau_max) {
        return Err("LPCMCI tauMax must be between 1 and 20".to_owned());
    }
    if !pc_alpha.is_finite() || !(0.0..=1.0).contains(&pc_alpha) || pc_alpha == 0.0 {
        return Err("LPCMCI pcAlpha must be finite and in (0, 1]".to_owned());
    }
    validate_dense_matrix("LPCMCI", values, rows, columns)?;
    if rows < (2 * tau_max + 16).max(24) {
        return Err("LPCMCI has too few observations for the selected maximum lag".to_owned());
    }
    let result = run_lpcmci_with_progress(
        &time_series_from_column_major(values, rows, columns),
        tau_max,
        pc_alpha,
        progress,
    );
    Ok(AnalysisResult::Lpcmci {
        observations: rows,
        variables: columns,
        tau_max,
        pc_alpha,
        graph: result.graph,
        p_matrix: result.p_matrix,
        val_matrix: result.val_matrix,
    })
}

fn dynotears_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    max_lag: usize,
    lambda_w: f64,
    lambda_a: f64,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err(
            "DYNOTEARS requires between 2 and 12 selected variables in the browser".to_owned(),
        );
    }
    if !(1..=6).contains(&max_lag) {
        return Err("DYNOTEARS maxLag must be between 1 and 6".to_owned());
    }
    if !lambda_w.is_finite() || lambda_w < 0.0 || !lambda_a.is_finite() || lambda_a < 0.0 {
        return Err("DYNOTEARS penalties must be finite and non-negative".to_owned());
    }
    validate_dense_matrix("DYNOTEARS", values, rows, columns)?;
    if rows < max_lag + 16 {
        return Err("DYNOTEARS has too few observations for the selected maximum lag".to_owned());
    }
    let samples = rows - max_lag;
    let x = DMatrix::from_fn(samples, columns, |row, column| {
        values[column * rows + max_lag + row]
    });
    let x_lags = DMatrix::from_fn(samples, columns * max_lag, |row, column| {
        let lag = column / columns + 1;
        let variable = column % columns;
        values[variable * rows + max_lag + row - lag]
    });
    let result = dynotears_with_progress(&x, &x_lags, lambda_w, lambda_a, progress);
    let contemporaneous_weights = (0..columns)
        .map(|source| {
            (0..columns)
                .map(|target| result.w[(source, target)])
                .collect()
        })
        .collect();
    let lagged_weights = (0..max_lag)
        .map(|lag| {
            (0..columns)
                .map(|source| {
                    (0..columns)
                        .map(|target| result.a[(lag * columns + source, target)])
                        .collect()
                })
                .collect()
        })
        .collect();
    Ok(AnalysisResult::Dynotears {
        observations: rows,
        variables: columns,
        max_lag,
        lambda_w,
        lambda_a,
        contemporaneous_weights,
        lagged_weights,
    })
}

fn ocse_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    max_lag: usize,
    alpha: f64,
    n_shuffles: usize,
    method: OcseMethod,
    k: usize,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err("oCSE requires between 2 and 12 selected variables in the browser".to_owned());
    }
    if !(1..=8).contains(&max_lag) {
        return Err("oCSE maxLag must be between 1 and 8".to_owned());
    }
    if !alpha.is_finite() || alpha <= 0.0 || alpha > 1.0 {
        return Err("oCSE alpha must be finite and in (0, 1]".to_owned());
    }
    if !(20..=2_000).contains(&n_shuffles) {
        return Err("oCSE nShuffles must be between 20 and 2000".to_owned());
    }
    if !(1..=20).contains(&k) {
        return Err("oCSE k must be between 1 and 20".to_owned());
    }
    validate_dense_matrix("oCSE", values, rows, columns)?;
    if rows < max_lag + 24 || matches!(method, OcseMethod::Knn) && k >= rows - max_lag {
        return Err("oCSE has too few observations for the selected lag and estimator".to_owned());
    }
    let data: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| values[column * rows + row])
                .collect()
        })
        .collect();
    let cmi_method = match method {
        OcseMethod::Gaussian => CmiMethod::Gaussian,
        OcseMethod::Knn => CmiMethod::Knn,
    };
    let edges = discover_network_with_progress(
        &data, max_lag, alpha, alpha, n_shuffles, cmi_method, k, progress,
    )
    .into_iter()
    .map(|edge| OcseEdgeEvidence {
        source: edge.src,
        target: edge.dst,
        lag: edge.lag,
        cmi: edge.cmi,
        p_value: edge.p_value,
    })
    .collect();
    Ok(AnalysisResult::Ocse {
        observations: rows,
        variables: columns,
        max_lag,
        alpha,
        n_shuffles,
        method,
        k,
        seed: 42,
        edges,
    })
}

fn granger_evidence(values: &[f64], rows: usize, max_lag: usize) -> Result<AnalysisResult, String> {
    if !(1..=20).contains(&max_lag) {
        return Err("Granger maxLag must be between 1 and 20".to_owned());
    }
    let expected = rows
        .checked_mul(2)
        .ok_or_else(|| "Granger matrix dimensions overflowed".to_owned())?;
    if values.len() != expected {
        return Err(format!(
            "Granger received {} values for a {rows} by 2 matrix",
            values.len()
        ));
    }
    if rows <= 3 * max_lag + 1 {
        return Err(
            "Granger has insufficient observations for the selected maximum lag".to_owned(),
        );
    }
    if values.iter().any(|value| !value.is_finite()) {
        return Err("Granger requires finite paired values; resolve missingness first".to_owned());
    }
    let target = &values[..rows];
    let cause = &values[rows..];
    let tests = granger_ssr_ftest(target, cause, max_lag)
        .into_iter()
        .enumerate()
        .map(|(index, (statistic, p_value))| GrangerLagEvidence {
            lag: index + 1,
            statistic,
            p_value,
        })
        .collect();
    Ok(AnalysisResult::GrangerSsrF {
        observations: rows,
        max_lag,
        tests,
    })
}

/// Execute one validated analysis command over a transferred dense numeric column.
#[wasm_bindgen(js_name = runAnalysis)]
pub fn run_analysis(
    command_json: &str,
    values: &[f64],
    progress_callback: &js_sys::Function,
) -> Result<String, JsError> {
    let command: AnalysisCommand = serde_json::from_str(command_json)
        .map_err(|error| JsError::new(&format!("invalid analysis command: {error}")))?;
    let progress = |stage: &'static str, completed: usize, total: usize| {
        let _ = progress_callback.call3(
            &JsValue::NULL,
            &JsValue::from_str(stage),
            &JsValue::from_f64(completed as f64),
            &JsValue::from_f64(total as f64),
        );
    };
    let result = match command {
        AnalysisCommand::StationarityBattery => stationarity_battery(values),
        AnalysisCommand::PcmciPlus {
            rows,
            columns,
            tau_max,
            pc_alpha,
        } => pcmci_plus(values, rows, columns, tau_max, pc_alpha),
        AnalysisCommand::Lpcmci {
            rows,
            columns,
            tau_max,
            pc_alpha,
        } => lpcmci_evidence(values, rows, columns, tau_max, pc_alpha, progress),
        AnalysisCommand::Dynotears {
            rows,
            columns,
            max_lag,
            lambda_w,
            lambda_a,
        } => dynotears_evidence(values, rows, columns, max_lag, lambda_w, lambda_a, progress),
        AnalysisCommand::Ocse {
            rows,
            columns,
            max_lag,
            alpha,
            n_shuffles,
            method,
            k,
        } => ocse_evidence(
            values, rows, columns, max_lag, alpha, n_shuffles, method, k, progress,
        ),
        AnalysisCommand::GrangerSsrF { rows, max_lag } => granger_evidence(values, rows, max_lag),
    }
    .map_err(|error| JsError::new(&error))?;
    serde_json::to_string(&result)
        .map_err(|error| JsError::new(&format!("could not serialize analysis result: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stationarity_battery_serializes_all_evidence_lanes() {
        let values: Vec<f64> = (0..120)
            .map(|index| {
                let time = index as f64;
                0.015 * time + (time / 5.0).sin() + 0.2 * (time / 2.7).cos()
            })
            .collect();
        let json = stationarity_battery(&values)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("fixture should produce stationarity evidence");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid result JSON");
        assert_eq!(value["kind"], "stationarityBattery");
        assert_eq!(value["observations"], 120);
        assert!(value["adf"]["constant"]["statistic"].is_number());
        assert!(value["kpss"]["constantAndTrend"]["pValue"].is_number());
        assert!(value["zivotAndrews"]["levelAndTrend"]["breakIndex"].is_number());
    }

    #[test]
    fn stationarity_battery_refuses_invalid_dense_inputs() {
        assert!(validate_stationarity_values(&[1.0; 24]).is_err());
        let mut non_finite = (0..24).map(|value| value as f64).collect::<Vec<_>>();
        non_finite[5] = f64::NAN;
        assert!(validate_stationarity_values(&non_finite).is_err());
        assert!(validate_stationarity_values(&[1.0, 2.0]).is_err());
    }

    #[test]
    fn pcmci_plus_accepts_column_major_input_and_serializes_raw_evidence() {
        let rows = 120;
        let mut values = vec![0.0; rows * 3];
        for row in 0..rows {
            let time = row as f64;
            let x = (time / 4.0).sin() + 0.1 * (time / 1.7).cos();
            let y = if row == 0 { 0.0 } else { 0.8 * values[row - 1] } + 0.05 * (time / 2.3).sin();
            let z = if row == 0 {
                0.0
            } else {
                -0.6 * values[rows + row - 1]
            } + 0.04 * (time / 3.1).cos();
            values[row] = x;
            values[rows + row] = y;
            values[2 * rows + row] = z;
        }
        let json = pcmci_plus(&values, rows, 3, 2, 0.05)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("fixture should produce PCMCI+ evidence");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid result JSON");
        assert_eq!(value["kind"], "pcmciPlus");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["variables"], 3);
        assert_eq!(value["graph"].as_array().unwrap().len(), 3);
        assert_eq!(value["pMatrix"][0][0].as_array().unwrap().len(), 3);
    }

    #[test]
    fn pcmci_plus_command_accepts_the_browser_camel_case_contract() {
        let command: AnalysisCommand = serde_json::from_str(
            r#"{"kind":"pcmciPlus","rows":120,"columns":3,"tauMax":2,"pcAlpha":0.05}"#,
        )
        .expect("browser command should parse");
        match command {
            AnalysisCommand::PcmciPlus {
                rows,
                columns,
                tau_max,
                pc_alpha,
            } => {
                assert_eq!(rows, 120);
                assert_eq!(columns, 3);
                assert_eq!(tau_max, 2);
                assert_eq!(pc_alpha, 0.05);
            }
            AnalysisCommand::StationarityBattery
            | AnalysisCommand::Lpcmci { .. }
            | AnalysisCommand::Dynotears { .. }
            | AnalysisCommand::Ocse { .. }
            | AnalysisCommand::GrangerSsrF { .. } => panic!("parsed the wrong command variant"),
        }
    }

    #[test]
    fn granger_command_serializes_every_lag_result() {
        let rows = 80;
        let mut values = vec![0.0; rows * 2];
        for row in 0..rows {
            let time = row as f64;
            values[rows + row] = (time / 3.0).sin() + 0.1 * (time / 1.7).cos();
            values[row] = if row == 0 {
                0.0
            } else {
                0.7 * values[rows + row - 1]
            } + 0.05 * (time / 2.1).sin();
        }
        let result = granger_evidence(&values, rows, 4).expect("fixture should produce evidence");
        let value = serde_json::to_value(result).expect("valid result JSON");
        assert_eq!(value["kind"], "grangerSsrF");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["maxLag"], 4);
        assert_eq!(value["tests"].as_array().unwrap().len(), 4);
        assert_eq!(value["tests"][0]["lag"], 1);
        assert!(value["tests"][0]["statistic"].is_number());
        assert!(value["tests"][0]["pValue"].is_number());
    }

    #[test]
    fn pcmci_plus_refuses_bad_shape_and_missing_values() {
        assert!(pcmci_plus(&[0.0; 100], 40, 3, 2, 0.05).is_err());
        let mut values = vec![0.0; 120];
        values[7] = f64::NAN;
        assert!(pcmci_plus(&values, 40, 3, 2, 0.05).is_err());
    }

    #[test]
    fn new_discovery_adapters_serialize_complete_outputs_and_report_real_progress() {
        let rows = 60;
        let columns = 3;
        let mut values = vec![0.0; rows * columns];
        for row in 0..rows {
            let time = row as f64;
            values[row] = (time / 3.7).sin() + 0.07 * (time / 1.9).cos();
            values[rows + row] = if row == 0 {
                0.0
            } else {
                0.72 * values[row - 1]
            } + 0.05 * (time / 2.1).sin();
            values[2 * rows + row] = if row == 0 {
                0.0
            } else {
                -0.55 * values[rows + row - 1]
            } + 0.04 * (time / 2.9).cos();
        }

        let mut lpcmci_progress = Vec::new();
        let lpcmci = lpcmci_evidence(&values, rows, columns, 1, 0.05, |stage, done, total| {
            lpcmci_progress.push((stage, done, total));
        })
        .expect("LPCMCI fixture should run");
        let lpcmci_json = serde_json::to_value(lpcmci).expect("LPCMCI result serializes");
        assert_eq!(lpcmci_json["kind"], "lpcmci");
        assert_eq!(lpcmci_json["graph"].as_array().unwrap().len(), columns);
        assert_eq!(lpcmci_progress.last(), Some(&("complete", 5, 5)));

        let mut dynotears_progress = Vec::new();
        let dynotears =
            dynotears_evidence(&values, rows, columns, 1, 0.1, 0.1, |stage, done, total| {
                dynotears_progress.push((stage, done, total))
            })
            .expect("DYNOTEARS fixture should run");
        let dynotears_json = serde_json::to_value(dynotears).expect("DYNOTEARS result serializes");
        assert_eq!(dynotears_json["kind"], "dynotears");
        assert_eq!(
            dynotears_json["contemporaneousWeights"]
                .as_array()
                .unwrap()
                .len(),
            columns
        );
        let (_, done, total) = dynotears_progress
            .last()
            .expect("DYNOTEARS reports completion");
        assert_eq!(done, total);

        let mut ocse_progress = Vec::new();
        let ocse = ocse_evidence(
            &values,
            rows,
            columns,
            1,
            0.05,
            20,
            OcseMethod::Gaussian,
            5,
            |stage, done, total| ocse_progress.push((stage, done, total)),
        )
        .expect("oCSE fixture should run");
        let ocse_json = serde_json::to_value(ocse).expect("oCSE result serializes");
        assert_eq!(ocse_json["kind"], "ocse");
        assert_eq!(ocse_json["seed"], 42);
        assert_eq!(ocse_progress.last(), Some(&("complete", columns, columns)));
    }
}
