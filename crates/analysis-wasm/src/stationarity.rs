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
        // A trend with a random walk, drawn by a linear congruential generator so the series is
        // reproducible in the oracle. A sum of sinusoids satisfies an exact linear recurrence, and
        // statsmodels refuses that input with the same rank error this façade reports.
        let values: Vec<f64> = {
            let mut state: u64 = 12345;
            let mut level = 0.0;
            (0..120)
                .map(|index| {
                    state = (1103515245u64.wrapping_mul(state).wrapping_add(12345)) % (1u64 << 31);
                    level += state as f64 / (1u64 << 31) as f64 - 0.5;
                    0.015 * index as f64 + level
                })
                .collect()
        };
        let json = stationarity_battery(&values)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("fixture should produce stationarity evidence");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid result JSON");
        assert_eq!(value["kind"], "stationarityBattery");
        assert_eq!(value["observations"], 120);
        // statsmodels 0.14.6 on this series: adfuller(regression="c") and the three
        // zivot_andrews regressions, which agree to the shared double precision.
        let close = |got: f64, want: f64, name: &str| {
            assert!((got - want).abs() < 1e-6, "{name}: {got} against statsmodels {want}");
        };
        close(value["adf"]["constant"]["statistic"].as_f64().unwrap(), -2.2019397715, "adf constant");
        close(value["kpss"]["constantAndTrend"]["statistic"].as_f64().unwrap(), 0.2797843766, "kpss constant and trend");
        close(value["zivotAndrews"]["level"]["statistic"].as_f64().unwrap(), -3.4941997939, "za level");
        close(value["zivotAndrews"]["trend"]["statistic"].as_f64().unwrap(), -3.9675485250, "za trend");
        close(value["zivotAndrews"]["levelAndTrend"]["statistic"].as_f64().unwrap(), -4.4364387401, "za level and trend");
        assert_eq!(value["zivotAndrews"]["level"]["breakIndex"].as_u64().unwrap(), 20);
        assert_eq!(value["zivotAndrews"]["levelAndTrend"]["breakIndex"].as_u64().unwrap(), 54);
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
