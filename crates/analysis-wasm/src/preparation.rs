//! Preparation steps applied at materialisation: STL seasonal adjustment.

use super::*;

pub(crate) fn multicollinearity(
    values: &[f64],
    rows: usize,
    columns: usize,
    correlation_threshold: f64,
    vif_threshold: f64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("multicollinearity diagnostics", values, rows, columns)?;
    if rows < 3 {
        return Err("multicollinearity diagnostics need at least three observations".to_owned());
    }
    if !(2..=64).contains(&columns) {
        return Err("multicollinearity diagnostics need between two and 64 variables".to_owned());
    }
    if !correlation_threshold.is_finite() || !(0.0..=1.0).contains(&correlation_threshold) {
        return Err(
            "the correlation threshold must be greater than zero and at most one".to_owned(),
        );
    }
    if !vif_threshold.is_finite() || vif_threshold <= 1.0 {
        return Err("the VIF threshold must be finite and greater than one".to_owned());
    }

    let matrix = DMatrix::from_column_slice(rows, columns, values);
    for column in 0..columns {
        let first = matrix[(0, column)];
        if (1..rows).all(|row| matrix[(row, column)] == first) {
            return Err(format!(
                "multicollinearity diagnostics are undefined for constant variable {column}"
            ));
        }
    }

    let correlation = correlation_matrix(&matrix);
    let (correlation_keep, correlation_drop, correlation_clusters) =
        cluster_redundant(&correlation, correlation_threshold);
    let (vif_keep, vif_drop, vif_history) = vif_redundant(&matrix, vif_threshold);
    let correlation = (0..columns)
        .map(|row| {
            (0..columns)
                .map(|column| correlation[(row, column)])
                .collect()
        })
        .collect();
    let vif_history = vif_history
        .into_iter()
        .map(|(column, vif)| VifEliminationEvidence {
            column,
            vif: vif.is_finite().then_some(vif),
        })
        .collect();

    Ok(AnalysisResult::Multicollinearity {
        observations: rows,
        variables: columns,
        correlation_threshold,
        vif_threshold,
        correlation,
        correlation_keep,
        correlation_drop,
        correlation_clusters,
        vif_keep,
        vif_drop,
        vif_history,
    })
}

pub(crate) fn seasonal_adjust(
    values: &[f64],
    rows: usize,
    columns: usize,
    period: usize,
    robust: bool,
    adjust: &[usize],
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("seasonal adjustment", values, rows, columns)?;
    if period < 2 {
        return Err("seasonal adjustment period must be at least 2".to_owned());
    }
    if rows < 2 * period {
        return Err(format!(
            "seasonal adjustment needs at least two seasons ({} rows) but has {rows}",
            2 * period
        ));
    }
    if adjust.is_empty() {
        return Err("seasonal adjustment needs at least one column to adjust".to_owned());
    }
    let mut seen = vec![false; columns];
    for &column in adjust {
        if column >= columns {
            return Err(format!(
                "seasonal adjustment column {column} is outside the {columns} columns"
            ));
        }
        if seen[column] {
            return Err(format!(
                "seasonal adjustment column {column} is listed twice"
            ));
        }
        seen[column] = true;
    }
    let config = StlConfig::new(period, robust);
    let mut output = values.to_vec();
    let adjusted = adjust
        .iter()
        .map(|&column| {
            let slice = &values[column * rows..(column + 1) * rows];
            let fit = stl(slice, &config, None, None);
            let trend_strength = strength(&fit.trend, &fit.resid);
            let before = strength(&fit.seasonal, &fit.resid);
            let deseasonalised: Vec<f64> = slice
                .iter()
                .zip(&fit.seasonal)
                .map(|(y, s)| y - s)
                .collect();
            let refit = stl(&deseasonalised, &config, None, None);
            let after = strength(&refit.seasonal, &refit.resid);
            output[column * rows..(column + 1) * rows].copy_from_slice(&deseasonalised);
            SeasonalAdjustedColumn {
                column,
                observed: slice.to_vec(),
                trend: fit.trend,
                seasonal: fit.seasonal,
                remainder: fit.resid,
                robust_weights: fit.weights,
                trend_strength,
                seasonal_strength_before: before,
                seasonal_strength_after: after,
            }
        })
        .collect();
    Ok(AnalysisResult::SeasonalAdjusted {
        rows,
        columns,
        period,
        values: output,
        adjusted,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multicollinearity_serializes_correlation_groups_and_vif_path() {
        let rows = 8;
        let x = [1.0, 2.0, 4.0, 7.0, 11.0, 16.0, 22.0, 29.0];
        let near_x = [1.1, 2.1, 3.9, 7.2, 10.8, 16.1, 21.9, 29.2];
        let z = [2.0, -1.0, 3.0, 0.5, -2.0, 4.0, 1.0, -3.0];
        let values: Vec<f64> = x.into_iter().chain(near_x).chain(z).collect();
        let result =
            multicollinearity(&values, rows, 3, 0.99, 10.0).expect("diagnostics should run");
        let value = serde_json::to_value(result).unwrap();
        assert_eq!(value["kind"], "multicollinearity");
        assert_eq!(value["correlation"].as_array().unwrap().len(), 3);
        assert_eq!(value["correlationClusters"][0], serde_json::json!([0, 1]));
        assert_eq!(value["correlationDrop"], serde_json::json!([1]));
        assert!(!value["vifHistory"].as_array().unwrap().is_empty());
    }

    #[test]
    fn multicollinearity_refuses_a_constant_variable() {
        let values = [1.0, 2.0, 3.0, 1.0, 1.0, 1.0];
        let error = match multicollinearity(&values, 3, 2, 0.9, 10.0) {
            Ok(_) => panic!("constant input should be refused"),
            Err(error) => error,
        };
        assert!(error.contains("constant variable 1"));
    }

    #[test]
    fn seasonal_adjustment_removes_the_seasonal_component_and_leaves_other_columns() {
        let rows = 144;
        let seasonal: Vec<f64> = (0..rows)
            .map(|i| {
                (i as f64 * std::f64::consts::TAU / 12.0).sin() * 3.0
                    + i as f64 * 0.02
                    + ((i * 7919) % 13) as f64 * 0.05
            })
            .collect();
        let flat: Vec<f64> = (0..rows).map(|i| i as f64).collect();
        let values: Vec<f64> = seasonal.iter().chain(flat.iter()).copied().collect();
        let result =
            seasonal_adjust(&values, rows, 2, 12, false, &[0]).expect("adjustment should run");
        let value: serde_json::Value = serde_json::to_value(result).unwrap();
        assert_eq!(value["kind"], "seasonalAdjusted");
        let adjusted = &value["adjusted"][0];
        assert_eq!(adjusted["observed"].as_array().unwrap().len(), rows);
        assert_eq!(adjusted["trend"].as_array().unwrap().len(), rows);
        assert_eq!(adjusted["seasonal"].as_array().unwrap().len(), rows);
        assert_eq!(adjusted["remainder"].as_array().unwrap().len(), rows);
        assert_eq!(adjusted["robustWeights"].as_array().unwrap().len(), rows);
        assert!(adjusted["trendStrength"].as_f64().unwrap() > 0.0);
        assert!(adjusted["seasonalStrengthBefore"].as_f64().unwrap() > 0.9);
        assert!(
            adjusted["seasonalStrengthAfter"].as_f64().unwrap() < 0.5,
            "after {}",
            adjusted["seasonalStrengthAfter"]
        );
        let output: Vec<f64> = value["values"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        assert_eq!(&output[rows..], &flat[..]);
        let reconstructed = adjusted["observed"]
            .as_array()
            .unwrap()
            .iter()
            .zip(adjusted["trend"].as_array().unwrap())
            .zip(adjusted["seasonal"].as_array().unwrap())
            .zip(adjusted["remainder"].as_array().unwrap())
            .map(|(((observed, trend), seasonal), remainder)| {
                (observed.as_f64().unwrap()
                    - trend.as_f64().unwrap()
                    - seasonal.as_f64().unwrap()
                    - remainder.as_f64().unwrap())
                .abs()
            })
            .fold(0.0_f64, f64::max);
        assert!(reconstructed <= 1e-12, "reconstruction {reconstructed}");
        let amplitude = output[..rows].iter().cloned().fold(f64::MIN, f64::max)
            - output[..rows].iter().cloned().fold(f64::MAX, f64::min);
        assert!(amplitude < 4.5, "amplitude {amplitude}");
        assert!(seasonal_adjust(&values, rows, 2, 12, false, &[2]).is_err());
        assert!(seasonal_adjust(&values, rows, 2, 100, false, &[0]).is_err());
    }
}
