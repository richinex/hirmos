//! The ADF, KPSS and Zivot–Andrews battery.

use super::*;

fn checked_breaks(values: &[f64]) -> Result<[ZaResult; 3], String> {
    use hirmos_causal_core::zivot_andrews::{try_zivot_andrews_all, ZaError};
    try_zivot_andrews_all(values, None).map_err(|error| match error {
        ZaError::RankDeficient => "Zivot–Andrews auxiliary regression is not full rank.".to_owned(),
        ZaError::DecompositionFailed => "Zivot–Andrews auxiliary regression could not be solved.".to_owned(),
    })
}

pub(crate) fn stationarity_battery(values: &[f64]) -> Result<AnalysisResult, String> {
    validate_stationarity_values(values)?;
    let [level, trend, level_and_trend] = checked_breaks(values)?;
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
            level: level.into(),
            trend: trend.into(),
            level_and_trend: level_and_trend.into(),
        },
    })
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
}
