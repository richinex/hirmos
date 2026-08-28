//! Thin browser command façade over the parity-tested causal kernels.
//!
//! The façade owns browser-facing validation and serialization. It does not reinterpret test
//! evidence or copy the numerical implementations out of the Hirmos causal core.

use hirmos_causal_core::parcorr::{CiKind, TimeSeries};
use hirmos_causal_core::pcmciplus::run_pcmciplus;
use hirmos_causal_core::{
    adfuller, kpss, zivot_andrews, AdfResult, KpssResult, Regression, ZaModel, ZaResult,
};
use hirmos_causal_core::tsdiag::granger_ssr_ftest;
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
    GrangerSsrF {
        rows: usize,
        max_lag: usize,
    },
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
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
    GrangerSsrF {
        observations: usize,
        max_lag: usize,
        tests: Vec<GrangerLagEvidence>,
    },
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
    let expected = rows
        .checked_mul(columns)
        .ok_or_else(|| "PCMCI+ matrix dimensions overflowed".to_owned())?;
    if values.len() != expected {
        return Err(format!(
            "PCMCI+ received {} values for a {rows} by {columns} matrix",
            values.len()
        ));
    }
    if rows < (2 * tau_max + 16).max(24) {
        return Err("PCMCI+ has too few observations for the selected maximum lag".to_owned());
    }
    if values.iter().any(|value| !value.is_finite()) {
        return Err("PCMCI+ requires finite dense values; resolve missingness first".to_owned());
    }

    // The browser boundary is column-major to permit zero-copy single-column runs. Tigramite's
    // TimeSeries is row-major, so this is the one deliberate analysis-worker matrix construction.
    let mut row_major = vec![0.0; expected];
    for column in 0..columns {
        for row in 0..rows {
            row_major[row * columns + column] = values[column * rows + row];
        }
    }
    let data = TimeSeries {
        values: row_major,
        t: rows,
        n: columns,
    };
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

fn granger_evidence(
    values: &[f64],
    rows: usize,
    max_lag: usize,
) -> Result<AnalysisResult, String> {
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
        return Err("Granger has insufficient observations for the selected maximum lag".to_owned());
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
pub fn run_analysis(command_json: &str, values: &[f64]) -> Result<String, JsError> {
    let command: AnalysisCommand = serde_json::from_str(command_json)
        .map_err(|error| JsError::new(&format!("invalid analysis command: {error}")))?;
    let result = match command {
        AnalysisCommand::StationarityBattery => stationarity_battery(values),
        AnalysisCommand::PcmciPlus {
            rows,
            columns,
            tau_max,
            pc_alpha,
        } => pcmci_plus(values, rows, columns, tau_max, pc_alpha),
        AnalysisCommand::GrangerSsrF { rows, max_lag } => {
            granger_evidence(values, rows, max_lag)
        }
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
            let z = if row == 0 { 0.0 } else { -0.6 * values[rows + row - 1] } + 0.04 * (time / 3.1).cos();
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
            AnalysisCommand::StationarityBattery => panic!("parsed the wrong command variant"),
            AnalysisCommand::GrangerSsrF { .. } => panic!("parsed the wrong command variant"),
        }
    }

    #[test]
    fn granger_command_serializes_every_lag_result() {
        let rows = 80;
        let mut values = vec![0.0; rows * 2];
        for row in 0..rows {
            let time = row as f64;
            values[rows + row] = (time / 3.0).sin() + 0.1 * (time / 1.7).cos();
            values[row] = if row == 0 { 0.0 } else { 0.7 * values[rows + row - 1] }
                + 0.05 * (time / 2.1).sin();
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
}
