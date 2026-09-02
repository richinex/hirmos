//! Preparation steps applied at materialisation: STL seasonal adjustment.

use super::*;

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
