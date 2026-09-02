//! Refuters, the simulated confounder grid, the DML refutation batch, and series structure.

use super::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn linear_refutation(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    simulations: usize,
    subset_fraction: f64,
    seed: u32,
    ljung_box_lags: usize,
) -> Result<AnalysisResult, String> {
    let (data, design, y) = design_columns(
        "linear refutation",
        values,
        rows,
        columns,
        treatment,
        outcome,
        adjustment,
    )?;
    if !(1..=2000).contains(&simulations) {
        return Err("linear refutation simulations must be between 1 and 2000".to_owned());
    }
    if !(subset_fraction > 0.1 && subset_fraction < 1.0) {
        return Err("linear refutation subset fraction must lie in (0.1, 1)".to_owned());
    }
    if ljung_box_lags == 0 || ljung_box_lags >= rows / 2 {
        return Err(
            "linear refutation Ljung-Box lags must be positive and below half the rows".to_owned(),
        );
    }
    let estimate = backdoor_linear_ate(&data, treatment, outcome, adjustment);
    let mut placebo_stream = Mt19937::seeded(seed);
    let placebo_effect = refute_placebo(
        &data,
        treatment,
        outcome,
        adjustment,
        simulations,
        &mut placebo_stream,
    );
    let mut subset_stream = Mt19937::seeded(seed.wrapping_add(1));
    let subset_effect = refute_data_subset(
        &data,
        treatment,
        outcome,
        adjustment,
        subset_fraction,
        simulations,
        &mut subset_stream,
    );
    let mut common_stream = Mt19937::seeded(seed.wrapping_add(2));
    let random_common_cause_effect = refute_random_common_cause(
        &data,
        treatment,
        outcome,
        adjustment,
        simulations,
        &mut common_stream,
    );
    let fit = Ols::fit(&design, &DVector::from_column_slice(&y));
    let residuals: Vec<f64> = fit.resid.iter().copied().collect();
    let (statistics, p_values) = ljung_box(&residuals, ljung_box_lags);
    let (shapiro_w, shapiro_p) = if (3..=5000).contains(&rows) {
        let (w, p) = shapiro(&residuals);
        (Some(w), Some(p))
    } else {
        (None, None)
    };
    Ok(AnalysisResult::LinearRefutation {
        observations: rows,
        estimate,
        simulations,
        seed,
        placebo_effect,
        subset_fraction,
        subset_effect,
        random_common_cause_effect,
        ljung_box_lags: (1..=ljung_box_lags).collect(),
        ljung_box_statistics: statistics,
        ljung_box_p_values: p_values,
        shapiro_w,
        shapiro_p,
        durbin_watson: durbin_watson(&residuals),
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn unobserved_confounding(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    seed: u32,
    explicit_kappa_t: Option<Vec<f64>>,
    explicit_kappa_y: Option<Vec<f64>>,
) -> Result<AnalysisResult, String> {
    let (data, _, y) = design_columns(
        "unobserved confounding",
        values,
        rows,
        columns,
        treatment,
        outcome,
        adjustment,
    )?;
    if adjustment.is_empty() {
        return Err("unobserved confounding needs at least one observed common cause to size the simulated confounder".to_owned());
    }
    let t: Vec<f64> = (0..rows).map(|row| data[(row, treatment)]).collect();
    if t.iter().any(|value| *value != 0.0 && *value != 1.0) {
        return Err(
            "unobserved confounding simulates a binary flip, so the treatment must be 0 or 1"
                .to_owned(),
        );
    }
    if t.iter().all(|value| *value == 0.0) || t.iter().all(|value| *value == 1.0) {
        return Err("unobserved confounding needs both treated and untreated rows".to_owned());
    }
    let occ: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            adjustment
                .iter()
                .map(|&column| data[(row, column)])
                .collect()
        })
        .collect();
    for grid in [&explicit_kappa_t, &explicit_kappa_y].into_iter().flatten() {
        if grid.is_empty() || grid.len() > 200 || grid.iter().any(|value| !value.is_finite()) {
            return Err(
                "unobserved confounding kappa grids must hold 1 to 200 finite values".to_owned(),
            );
        }
    }
    if explicit_kappa_t
        .as_ref()
        .is_some_and(|grid| grid.iter().any(|value| !(0.0..=1.0).contains(value)))
    {
        return Err(
            "unobserved confounding kappa_t values are flip fractions in [0, 1]".to_owned(),
        );
    }
    let kappa_t = explicit_kappa_t.unwrap_or_else(|| infer_kappa_t(&occ, &t));
    let kappa_y = explicit_kappa_y.unwrap_or_else(|| infer_kappa_y(&occ, &y));
    let mut stream = Mt19937::seeded(seed);
    let effects = unobserved_common_cause_grid(&t, &y, &occ, &kappa_t, &kappa_y, &mut stream);
    Ok(AnalysisResult::UnobservedConfounding {
        observations: rows,
        seed,
        kappa_t,
        kappa_y,
        effects,
        original_effect: backdoor_linear_ate(&data, treatment, outcome, adjustment),
    })
}

pub(crate) fn series_structure(
    values: &[f64],
    rows: usize,
    columns: usize,
    period: Option<usize>,
    robust: bool,
    correlation_max_lag: usize,
    pelt_min_size: usize,
    pelt_jump: usize,
    pelt_penalty: f64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("series structure", values, rows, columns)?;
    if let Some(period) = period {
        if period < 2 {
            return Err("series structure period must be at least 2".to_owned());
        }
    }
    if correlation_max_lag == 0 || correlation_max_lag >= rows / 2 {
        return Err(
            "series structure correlation lag must be positive and below half the rows".to_owned(),
        );
    }
    if pelt_min_size == 0 || pelt_jump == 0 || !pelt_penalty.is_finite() || pelt_penalty < 0.0 {
        return Err(
            "series structure PELT settings must be positive with a finite non-negative penalty"
                .to_owned(),
        );
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let series = (0..columns)
        .map(|column| {
            let y: Vec<f64> = (0..rows).map(|row| data[(row, column)]).collect();
            let (trend_strength, seasonal_strength) = match period {
                Some(period) if rows >= 2 * period => {
                    let config = StlConfig::new(period, robust);
                    let fit = stl(&y, &config, None, None);
                    (
                        Some(strength(&fit.trend, &fit.resid)),
                        Some(strength(&fit.seasonal, &fit.resid)),
                    )
                }
                _ => (None, None),
            };
            let signal: Vec<Vec<f64>> = y.iter().map(|value| vec![*value]).collect();
            let mut change_points = pelt_l2(&signal, pelt_min_size, pelt_jump, pelt_penalty);
            change_points.retain(|&end| end < rows);
            let acf = hirmos_causal_core::tsdiag::acf(&y, correlation_max_lag);
            let pacf = hirmos_causal_core::tsdiag::pacf_yw_mle(&y, correlation_max_lag);
            let (acf_limits, pacf_limits) =
                hirmos_causal_core::tsdiag::correlation_plot_limits(&acf, rows);
            SeriesStructureEvidence {
                column,
                trend_strength,
                seasonal_strength,
                correlation_max_lag,
                acf,
                acf_limits,
                pacf,
                pacf_limits,
                change_points,
                pelt_penalty,
            }
        })
        .collect();
    Ok(AnalysisResult::SeriesStructure {
        observations: rows,
        period,
        series,
    })
}

/// One seeded stream, one fixed order: the main fit, then the placebo, random common cause, and
/// unobserved-confounding readings. Reordering would change the folds, so the order is data.
#[allow(clippy::too_many_arguments)]
pub(crate) fn dml_refutation_batch(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    model: DmlModel,
    att: bool,
    seed: u32,
) -> Result<AnalysisResult, String> {
    let (x, y, d, treat_binary) = dml_frame(
        "DML refutation batch",
        values,
        rows,
        columns,
        treatment,
        outcome,
        adjustment,
        model,
    )?;
    let study = WorkerStudy {
        x: &x,
        y: &y,
        d: &d,
        aipw: model == DmlModel::Irm,
        att,
        treat_binary,
    };
    let mut fold_stream = Mt19937::seeded(seed);
    let full = worker_fit(&study, &mut fold_stream);
    let placebo = placebo_refute(&study, full.coef, &mut fold_stream);
    let common = random_common_cause_refute(&study, &mut fold_stream);
    let sensitivity = unobserved_refute(&full);
    let outcome_of =
        |value: hirmos_causal_core::refute_dml::RefutationOutcome| DmlRefutationOutcome {
            original_effect: value.original_effect,
            refuted_effect: value.refuted_effect,
            p_value: value.p_value,
        };
    Ok(AnalysisResult::DmlRefutationBatch {
        observations: rows,
        model,
        att,
        seed,
        order: vec![
            "mainFit",
            "placebo",
            "randomCommonCause",
            "unobservedSensitivity",
        ],
        main_estimate: full.coef,
        placebo: outcome_of(placebo),
        random_common_cause: outcome_of(common),
        sensitivity: DmlSensitivity {
            scenarios: sensitivity
                .scenarios
                .iter()
                .map(|scenario| DmlSensitivityScenario {
                    confounding: scenario.confounding,
                    effect_lower: scenario.effect_lower,
                    effect_upper: scenario.effect_upper,
                    ci_lower: scenario.ci_lower,
                    ci_upper: scenario.ci_upper,
                })
                .collect(),
            robustness_value: sensitivity.robustness_value,
            robustness_value_ci: sensitivity.robustness_value_ci,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_refutation_and_unobserved_grid_report_the_dowhy_probes() {
        let rows = 200;
        let mut state = 0x0bad_cafe_1234_5678u64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64
        };
        let w: Vec<f64> = (0..rows).map(|_| uniform() - 0.5).collect();
        let t: Vec<f64> = (0..rows)
            .map(|i| {
                if uniform() < 0.5 + 0.6 * w[i] {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| 0.5 * t[i] + 0.8 * w[i] + (uniform() - 0.5))
            .collect();
        let mut values = Vec::new();
        values.extend(&t);
        values.extend(&y);
        values.extend(&w);
        let json = linear_refutation(&values, rows, 3, 0, 1, &[2], 20, 0.8, 555, 6)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("refutation should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "linearRefutation");
        let estimate = value["estimate"].as_f64().unwrap();
        assert!(value["placeboEffect"].as_f64().unwrap().abs() < estimate.abs());
        assert_eq!(value["ljungBoxLags"].as_array().unwrap().len(), 6);
        assert!(value["shapiroP"].as_f64().is_some());

        let grid = unobserved_confounding(
            &values,
            rows,
            3,
            0,
            1,
            &[2],
            100,
            Some(vec![0.1, 0.3, 0.5]),
            Some(vec![0.0, 0.5, 1.0, 1.5]),
        )
        .unwrap();
        let value: serde_json::Value = serde_json::to_value(grid).unwrap();
        assert_eq!(value["kind"], "unobservedConfounding");
        let kt = value["kappaT"].as_array().unwrap().len();
        let ky = value["kappaY"].as_array().unwrap().len();
        assert_eq!(value["effects"].as_array().unwrap().len(), kt);
        assert_eq!(value["effects"][0].as_array().unwrap().len(), ky);
        assert_eq!(kt, 3);
        assert_eq!(ky, 4);
        assert!(unobserved_confounding(&values, rows, 3, 0, 1, &[2], 100, None, None).is_ok());
        assert!(unobserved_confounding(&values, rows, 3, 0, 1, &[], 100, None, None).is_err());
        assert!(unobserved_confounding(&values, rows, 3, 1, 0, &[2], 100, None, None).is_err());
        assert!(
            unobserved_confounding(&values, rows, 3, 0, 1, &[2], 100, Some(vec![1.5]), None)
                .is_err()
        );
    }

    #[test]
    fn series_structure_finds_a_level_shift_and_seasonal_strength() {
        let rows = 144;
        let y: Vec<f64> = (0..rows)
            .map(|i| {
                (i as f64 * std::f64::consts::TAU / 12.0).sin() * 3.0
                    + if i >= 90 { 8.0 } else { 0.0 }
                    + ((i * 7919) % 13) as f64 * 0.05
            })
            .collect();
        let json = series_structure(&y, rows, 1, Some(12), false, 12, 4, 1, 10.0)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("structure should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "seriesStructure");
        let series = &value["series"][0];
        assert!(series["seasonalStrength"].as_f64().unwrap() > 0.9);
        assert_eq!(series["correlationMaxLag"], 12);
        for name in ["acf", "acfLimits", "pacf", "pacfLimits"] {
            assert_eq!(series[name].as_array().unwrap().len(), 13);
        }
        let points: Vec<u64> = series["changePoints"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect();
        assert!(
            points.iter().any(|&p| (88..=92).contains(&p)),
            "change points {points:?}"
        );
        assert!(series_structure(&y, rows, 1, Some(1), false, 12, 4, 1, 10.0).is_err());
        assert!(series_structure(&y, rows, 1, Some(12), false, rows / 2, 4, 1, 10.0).is_err());
    }

    #[test]
    fn dml_batch_repeats_the_estimation_fit_and_runs_the_refuters_in_order() {
        let rows = 120;
        let mut stream = Mt19937::seeded(3);
        let w1: Vec<f64> = (0..rows).map(|_| stream.next_f64() - 0.5).collect();
        let w2: Vec<f64> = (0..rows).map(|_| stream.next_f64() - 0.5).collect();
        let d: Vec<f64> = (0..rows)
            .map(|i| {
                if w1[i] + 0.3 * (stream.next_f64() - 0.5) > 0.0 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| 2.0 * d[i] + w1[i] + 0.5 * w2[i] + 0.2 * (stream.next_f64() - 0.5))
            .collect();
        let values: Vec<f64> = d
            .iter()
            .chain(y.iter())
            .chain(w1.iter())
            .chain(w2.iter())
            .copied()
            .collect();
        let fit = serde_json::to_value(
            double_ml(&values, rows, 4, 0, 1, &[2, 3], DmlModel::Plr, false, 7).unwrap(),
        )
        .unwrap();
        assert_eq!(fit["kind"], "doubleMl");
        let estimate = fit["estimate"].as_f64().unwrap();
        assert!((estimate - 2.0).abs() < 0.6, "estimate {estimate}");
        let batch = serde_json::to_value(
            dml_refutation_batch(&values, rows, 4, 0, 1, &[2, 3], DmlModel::Plr, false, 7).unwrap(),
        )
        .unwrap();
        assert_eq!(batch["kind"], "dmlRefutationBatch");
        assert_eq!(batch["mainEstimate"].as_f64().unwrap(), estimate);
        assert_eq!(batch["order"].as_array().unwrap().len(), 4);
        assert!(batch["placebo"]["refutedEffect"].as_f64().unwrap().abs() < estimate.abs());
        assert_eq!(
            batch["sensitivity"]["scenarios"].as_array().unwrap().len(),
            3
        );
        let irm = double_ml(&values, rows, 4, 0, 1, &[2, 3], DmlModel::Irm, true, 7).unwrap();
        assert!(matches!(
            irm,
            AnalysisResult::DoubleMl {
                att: true,
                treat_binary: true,
                ..
            }
        ));
        assert!(double_ml(&values, rows, 4, 2, 1, &[0, 3], DmlModel::Irm, false, 7).is_err());
        assert!(double_ml(&values, rows, 4, 0, 1, &[], DmlModel::Plr, false, 7).is_err());
    }
}
