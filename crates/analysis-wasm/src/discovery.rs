//! Cross-sectional and time-series discovery façades.

use super::*;

pub(crate) fn pcmci_plus(
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

pub(crate) fn lpcmci_evidence<F>(
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

pub(crate) fn dynotears_evidence<F>(
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

/// DirectLiNGAM over independent observations in a dense column-major matrix.
///
/// The core matrix is target-by-source, matching lingam's `adjacency_matrix_`; the browser
/// contract is source-by-target so every discovery plot and DAG adapter uses one orientation.
pub(crate) fn direct_lingam_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err(
            "DirectLiNGAM requires between 2 and 12 selected variables in the browser".to_owned(),
        );
    }
    validate_dense_matrix("DirectLiNGAM", values, rows, columns)?;
    if rows < columns + 16 {
        return Err(
            "DirectLiNGAM has too few independent observations for the selected variables"
                .to_owned(),
        );
    }
    for column in 0..columns {
        let series = &values[column * rows..(column + 1) * rows];
        let first = series[0];
        if series.iter().all(|value| *value == first) {
            return Err(format!(
                "DirectLiNGAM requires every variable to vary; column {column} is constant"
            ));
        }
    }
    let matrix = DMatrix::from_fn(rows, columns, |row, column| values[column * rows + row]);
    let (causal_order, adjacency) = direct_lingam_with_progress(&matrix, progress);
    let weights = (0..columns)
        .map(|source| {
            (0..columns)
                .map(|target| adjacency[(target, source)])
                .collect()
        })
        .collect();
    Ok(AnalysisResult::DirectLingam {
        observations: rows,
        variables: columns,
        causal_order,
        weights,
    })
}

/// VAR-LiNGAM over a dense column-major matrix; `lags` bounds the BIC lag sweep.
///
/// The kernel panics on a rank-deficient VAR design or a singular residual covariance, which a
/// wasm module cannot recover from, so constant columns and undersized samples are refused here.
pub(crate) fn var_lingam_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    lags: usize,
    prune: bool,
    mut progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err(
            "VAR-LiNGAM requires between 2 and 12 selected variables in the browser".to_owned(),
        );
    }
    if !(1..=6).contains(&lags) {
        return Err("VAR-LiNGAM lags must be between 1 and 6".to_owned());
    }
    validate_dense_matrix("VAR-LiNGAM", values, rows, columns)?;
    if rows < columns * (lags + 1) + 16 {
        return Err(
            "VAR-LiNGAM has too few observations for the selected variables and lag order"
                .to_owned(),
        );
    }
    for column in 0..columns {
        let series = &values[column * rows..(column + 1) * rows];
        let first = series[0];
        if series.iter().all(|value| *value == first) {
            return Err(format!(
                "VAR-LiNGAM requires every variable to vary; column {column} is constant"
            ));
        }
    }
    progress("var-fit", 0, 2);
    let data: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| values[column * rows + row])
                .collect()
        })
        .collect();
    let result = run_var_lingam(&data, lags, prune);
    progress("complete", 2, 2);
    let matrix = |tau: usize| -> Vec<Vec<f64>> {
        (0..columns)
            .map(|source| {
                (0..columns)
                    .map(|target| result.adjacency_matrices[tau][(target, source)])
                    .collect()
            })
            .collect()
    };
    Ok(AnalysisResult::VarLingam {
        observations: rows,
        variables: columns,
        lags,
        selected_lag: result.k_ar,
        prune,
        causal_order: result.causal_order.clone(),
        contemporaneous_weights: matrix(0),
        lagged_weights: (1..=result.k_ar).map(matrix).collect(),
    })
}

pub(crate) fn ocse_evidence<F>(
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

pub(crate) fn granger_evidence(
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

#[cfg(test)]
mod tests {
    use super::*;

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
            | AnalysisCommand::DirectLingam { .. }
            | AnalysisCommand::VarLingam { .. }
            | AnalysisCommand::Ocse { .. }
            | AnalysisCommand::GrangerSsrF { .. }
            | AnalysisCommand::BackdoorIdentify { .. }
            | AnalysisCommand::DagCheck { .. }
            | AnalysisCommand::BackdoorLinear { .. }
            | AnalysisCommand::FrontdoorTwoStage { .. }
            | AnalysisCommand::CountGlm { .. }
            | AnalysisCommand::CausalEffectsTotal { .. }
            | AnalysisCommand::CausalImpact { .. }
            | AnalysisCommand::LinearRefutation { .. }
            | AnalysisCommand::UnobservedConfounding { .. }
            | AnalysisCommand::SeriesStructure { .. }
            | AnalysisCommand::SeasonalAdjust { .. }
            | AnalysisCommand::DoubleMl { .. }
            | AnalysisCommand::DmlRefutationBatch { .. }
            | AnalysisCommand::ArdlPss { .. }
            | AnalysisCommand::Vecm { .. }
            | AnalysisCommand::SyntheticControl { .. }
            | AnalysisCommand::PanelIntervention { .. }
            | AnalysisCommand::LinearScmCounterfactual { .. }
            | AnalysisCommand::NegbinNuts { .. }
            | AnalysisCommand::BayesianGaussian { .. }
            | AnalysisCommand::DiscreteBnQuery { .. }
            | AnalysisCommand::BinaryEtt { .. }
            | AnalysisCommand::ResolveMissingness { .. } => {
                panic!("parsed the wrong command variant")
            }
        }
    }

    #[test]
    fn var_lingam_command_serializes_ordered_weights() {
        let rows = 160;
        let columns = 3;
        let mut state = 0x2545_f491_4f6c_dd1du64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
        };
        let mut series = vec![vec![0.0; columns]; rows];
        for row in 1..rows {
            let e: Vec<f64> = (0..columns).map(|_| uniform()).collect();
            series[row][0] = 0.6 * series[row - 1][0] + e[0];
            series[row][1] = 0.5 * series[row][0] + 0.3 * series[row - 1][1] + e[1];
            series[row][2] = 0.4 * series[row][1] + 0.2 * series[row - 1][0] + e[2];
        }
        let mut values = vec![0.0; rows * columns];
        for column in 0..columns {
            for row in 0..rows {
                values[column * rows + row] = series[row][column];
            }
        }
        let mut stages = Vec::new();
        let result = var_lingam_evidence(&values, rows, columns, 2, true, |stage, done, total| {
            stages.push((stage, done, total))
        })
        .expect("VAR-LiNGAM fixture should run");
        let json = serde_json::to_value(result).expect("VAR-LiNGAM result serializes");
        assert_eq!(json["kind"], "varLingam");
        assert_eq!(json["causalOrder"].as_array().unwrap().len(), columns);
        assert_eq!(
            json["contemporaneousWeights"].as_array().unwrap().len(),
            columns
        );
        assert_eq!(
            json["laggedWeights"].as_array().unwrap().len(),
            json["selectedLag"].as_u64().unwrap() as usize
        );
        assert_eq!(stages.last(), Some(&("complete", 2, 2)));

        let mut constant = values.clone();
        for row in 0..rows {
            constant[row] = 1.0;
        }
        match var_lingam_evidence(&constant, rows, columns, 1, true, |_, _, _| {}) {
            Err(message) => assert!(message.contains("constant")),
            Ok(_) => panic!("a constant column must be refused"),
        }
    }

    #[test]
    fn direct_lingam_serializes_cross_sectional_order_and_weights() {
        let rows = 96;
        let columns = 3;
        let mut values = vec![0.0; rows * columns];
        for row in 0..rows {
            let u = ((row * 37 % 101) as f64 - 50.0) / 25.0;
            let v = ((row * 61 % 103) as f64 - 51.0) / 30.0;
            let w = ((row * 73 % 107) as f64 - 53.0) / 35.0;
            values[row] = u;
            values[rows + row] = 0.8 * u + v;
            values[2 * rows + row] = -0.5 * values[rows + row] + w;
        }
        let mut stages = Vec::new();
        let result = direct_lingam_evidence(&values, rows, columns, |stage, done, total| {
            stages.push((stage, done, total));
        })
        .expect("DirectLiNGAM fixture should run");
        let json = serde_json::to_value(result).expect("DirectLiNGAM result serializes");
        assert_eq!(json["kind"], "directLingam");
        assert_eq!(json["causalOrder"].as_array().unwrap().len(), columns);
        assert_eq!(json["weights"].as_array().unwrap().len(), columns);
        assert!(stages.iter().any(|(stage, _, _)| *stage == "causal-order"));
        assert_eq!(stages.last(), Some(&("complete", columns + 1, columns + 1)));
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
