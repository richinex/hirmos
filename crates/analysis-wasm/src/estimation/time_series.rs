//! Time-series chapter fits: the interrupted series, the ARDL bounds test and the VECM.

use super::*;

/// The interrupted time series of Lopez Bernal, Cummins and Gasparrini on one prepared column:
/// the design from the declared impact model, fitted as the paper's quasi-Poisson model for a
/// count or by OLS with Newey-West errors for a continuous series.
#[allow(clippy::too_many_arguments)]
pub(crate) fn interrupted_series(
    values: &[f64],
    rows: usize,
    columns: usize,
    outcome: usize,
    model: InterruptedModel,
    intervention_row: usize,
    lag: usize,
    impact: InterruptedImpact,
    seasonal: InterruptedSeasonal,
    ljung_box_lags: usize,
) -> Result<AnalysisResult, String> {
    use hirmos_causal_core::interrupted_series::{fit_continuous, fit_continuous_arma, fit_count, Harmonics, ImpactModel, InterruptedSeriesDesign, ResidualCorrelation};
    validate_dense_matrix("interrupted series", values, rows, columns)?;
    if outcome >= columns {
        return Err("interrupted series received a column outside the matrix".to_owned());
    }
    if intervention_row == 0 || intervention_row + lag >= rows {
        return Err("interrupted series needs rows before and after the intervention row".to_owned());
    }
    let core_impact = match impact {
        InterruptedImpact::Level => ImpactModel::Level,
        InterruptedImpact::LevelAndSlope => ImpactModel::LevelAndSlope,
        InterruptedImpact::Slope => ImpactModel::Slope,
        InterruptedImpact::TemporaryLevel { until } => {
            if until <= intervention_row + lag || until > rows {
                return Err("a temporary level change must end after it starts and within the series".to_owned());
            }
            ImpactModel::TemporaryLevel { until }
        }
    };
    // The phase is the row's time, 1..=n: a sine and cosine pair at the period absorbs any shift of
    // origin, so this is the paper's harmonic(month, ...) on a regular grid.
    let harmonics = match seasonal {
        InterruptedSeasonal::None => None,
        InterruptedSeasonal::Harmonic { pairs, period } => {
            if pairs == 0 || !(period > 0.0) {
                return Err("harmonic terms need at least one pair and a positive period".to_owned());
            }
            Some(Harmonics { pairs, period, phase: (1..=rows).map(|t| t as f64).collect() })
        }
    };
    let spec = InterruptedSeriesDesign { intervention_row, lag, impact: core_impact, harmonics };
    let y: Vec<f64> = (0..rows).map(|row| values[outcome * rows + row]).collect();
    let terms_of = |names: &[String], params: &[f64], bse: &[f64], pvalues: &[f64], conf: &dyn Fn(usize) -> [f64; 2]| -> Vec<InterruptedTermEvidence> {
        names
            .iter()
            .enumerate()
            .map(|(i, name)| InterruptedTermEvidence { name: name.clone(), coefficient: params[i], standard_error: bse[i], p_value: pvalues[i], interval: conf(i) })
            .collect()
    };
    let seasonal_of = |deseasonalised: Option<Vec<f64>>, counterfactual: Option<Vec<f64>>| match (seasonal, deseasonalised, counterfactual) {
        (InterruptedSeasonal::Harmonic { pairs, period }, Some(fitted), Some(counterfactual)) => InterruptedSeasonalEvidence::Harmonic {
            pairs,
            period,
            deseasonalised: fitted.into_iter().zip(counterfactual).map(|(fitted, counterfactual)| InterruptedDeseasonalisedEvidence { fitted, counterfactual }).collect(),
        },
        _ => InterruptedSeasonalEvidence::None,
    };
    let correlations = |c: &ResidualCorrelation| -> (Vec<CorrelationEvidence>, Vec<CorrelationEvidence>) {
        let pair = |values: &[f64], limits: &[f64]| values.iter().zip(limits).map(|(&value, &limit)| CorrelationEvidence { value, limit }).collect();
        (pair(&c.acf, &c.acf_limits), pair(&c.pacf, &c.pacf_limits))
    };
    let boxes = |b: &(Vec<f64>, Vec<f64>)| -> Vec<LjungBoxEvidence> { b.0.iter().zip(&b.1).map(|(&statistic, &p_value)| LjungBoxEvidence { statistic, p_value }).collect() };
    let path_of = |observed: &[f64], fitted: &[f64], counterfactual: &[f64], residual: &[f64]| -> Vec<InterruptedRowEvidence> {
        (0..observed.len()).map(|r| InterruptedRowEvidence { observed: observed[r], fitted: fitted[r], counterfactual: counterfactual[r], residual: residual[r] }).collect()
    };
    match model {
        InterruptedModel::Continuous { errors } => {
            match errors {
                ContinuousErrors::NeweyWest { max_lags } => {
                    // statsmodels' bandwidth when `maxlags` is not given: floor(4 (n/100)^(2/9)).
                    let max_lags = max_lags
                        .unwrap_or_else(|| (4.0 * (rows as f64 / 100.0).powf(2.0 / 9.0)).floor() as usize);
                    let fit = fit_continuous(&y, &spec, max_lags, ljung_box_lags);
                    let terms = terms_of(&fit.names, &fit.fit.params, &fit.fit.bse, &fit.fit.pvalues, &|i| [fit.fit.conf_int[i].0, fit.fit.conf_int[i].1]);
                    let (residual_acf, residual_pacf) = correlations(&fit.correlation);
                    Ok(AnalysisResult::InterruptedSeries {
                        observations: rows,
                        outcome,
                        model: InterruptedModelEvidence::Continuous { errors: ContinuousErrorEvidence::NeweyWest { max_lags } },
                        intervention_row,
                        lag,
                        impact,
                        seasonal: seasonal_of(fit.deseasonalised, fit.deseasonalised_counterfactual),
                        terms,
                        path: path_of(&y, &fit.fitted, &fit.counterfactual, &fit.fit.resid),
                        ljung_box: boxes(&fit.ljung_box),
                        residual_acf,
                        residual_pacf,
                        converged: true,
                    })
                }
                ContinuousErrors::Arma { p, q, max_iter } => {
                    let order = arma_order("interrupted series", rows, p, q, max_iter)?;
                    let fit = fit_continuous_arma(&y, &spec, order, max_iter, ljung_box_lags);
                    // The design's terms only; the error process is reported beside them.
                    let terms = terms_of(&fit.names, &fit.fit.params[..fit.names.len()], &fit.fit.bse, &fit.fit.pvalues, &|i| [fit.fit.conf_int[i].0, fit.fit.conf_int[i].1]);
                    let (residual_acf, residual_pacf) = correlations(&fit.correlation);
                    let converged = fit.fit.converged;
                    Ok(AnalysisResult::InterruptedSeries {
                        observations: rows,
                        outcome,
                        model: InterruptedModelEvidence::Continuous { errors: ContinuousErrorEvidence::Arma(arma_error_evidence(&fit.fit)) },
                        intervention_row,
                        lag,
                        impact,
                        seasonal: seasonal_of(fit.deseasonalised, fit.deseasonalised_counterfactual),
                        terms,
                        // The residual is the standardised one-step-ahead forecast error.
                        path: path_of(&y, &fit.fitted, &fit.counterfactual, &fit.fit.standardized_resid),
                        ljung_box: boxes(&fit.ljung_box),
                        residual_acf,
                        residual_pacf,
                        converged,
                    })
                }
            }
        }
        InterruptedModel::Count { exposure } => {
            if exposure.is_some_and(|column| column >= columns) {
                return Err("interrupted series received an exposure column outside the matrix".to_owned());
            }
            if y.iter().any(|value| *value < 0.0 || value.fract() != 0.0) {
                return Err("a count model needs a non-negative integer outcome".to_owned());
            }
            if y.iter().all(|value| *value == 0.0) {
                return Err("a count model is undefined when every outcome is zero".to_owned());
            }
            let exposure_values: Option<Vec<f64>> = exposure.map(|column| (0..rows).map(|row| values[column * rows + row]).collect());
            if exposure_values.as_ref().is_some_and(|e| e.iter().any(|v| !(*v > 0.0))) {
                return Err("the exposure must be positive on every row".to_owned());
            }
            let fit = fit_count(&y, exposure_values.as_deref(), &spec, ljung_box_lags);
            let z = ndtri(0.975);
            let terms = terms_of(&fit.names, &fit.fit.params, &fit.fit.bse, &fit.fit.pvalues, &|i| [fit.fit.params[i] - z * fit.fit.bse[i], fit.fit.params[i] + z * fit.fit.bse[i]]);
            // The paper's rate: each count scaled to the mean exposure, so rows are comparable and
            // sit on the standardised curves.
            let observed: Vec<f64> = match &exposure_values {
                Some(e) => {
                    let mean = e.iter().sum::<f64>() / e.len() as f64;
                    y.iter().zip(e).map(|(count, exposure)| count * mean / exposure).collect()
                }
                None => y.clone(),
            };
            let (residual_acf, residual_pacf) = correlations(&fit.correlation);
            Ok(AnalysisResult::InterruptedSeries {
                observations: rows,
                outcome,
                model: InterruptedModelEvidence::Count { exposure, dispersion: fit.fit.scale },
                intervention_row,
                lag,
                impact,
                seasonal: seasonal_of(fit.deseasonalised, fit.deseasonalised_counterfactual),
                terms,
                path: path_of(&observed, &fit.standardised, &fit.standardised_counterfactual, &fit.fit.resid_deviance),
                ljung_box: boxes(&fit.ljung_box),
                residual_acf,
                residual_pacf,
                converged: fit.fit.converged,
            })
        }
    }
}

pub(crate) fn ardl_pss(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    max_lag: usize,
    trend: ArdlTrend,
    case: usize,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("ARDL", values, rows, columns)?;
    if treatment >= columns || outcome >= columns || treatment == outcome {
        return Err(
            "ARDL needs distinct treatment and outcome columns inside the matrix".to_owned(),
        );
    }
    if !(1..=24).contains(&max_lag) {
        return Err("ARDL maximum lag must be between 1 and 24".to_owned());
    }
    let valid_case = match trend {
        ArdlTrend::C => case == 2 || case == 3,
        ArdlTrend::Ct => case == 4 || case == 5,
    };
    if !valid_case || stat_star(1, case, true).is_none() {
        return Err(
            "ARDL PSS case must be 2 or 3 with a constant and 4 or 5 with a trend".to_owned(),
        );
    }
    if rows < 6 * (max_lag + 1) + 10 {
        return Err(format!(
            "ARDL needs at least {} rows for a maximum lag of {max_lag}",
            6 * (max_lag + 1) + 10
        ));
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let y: Vec<f64> = (0..rows).map(|row| data[(row, outcome)]).collect();
    let x: Vec<f64> = (0..rows).map(|row| data[(row, treatment)]).collect();
    let trend_kind = match trend {
        ArdlTrend::C => Trend::C,
        ArdlTrend::Ct => Trend::Ct,
    };
    let selection = ardl_select_order(&y, max_lag, &x, max_lag, "aic", trend_kind);
    let p = selection.ar_lag.max(1);
    let q = selection.dl_lag.unwrap_or(1).max(1);
    let model = uecm(&y, p, &x, q, trend_kind);
    let vector = model.cointegrating_vector(0.05);
    let index = model.n_det + 1;
    let bounds = bounds_test(&model, case);
    Ok(AnalysisResult::ArdlPss {
        observations: rows,
        long_run: ArdlLongRun::Recorded {
            departures: model.cointegrating_residuals(&y, &x, trend_kind),
            observed: y,
        },
        trend,
        case,
        ar_lag: p,
        dl_lag: q,
        grid: selection.grid,
        long_run_effect: -vector.params[index],
        p_value: vector.pvalues[index],
        interval: (-vector.conf_int[index].1, -vector.conf_int[index].0),
        level: 0.95,
        bounds_statistic: bounds.stat,
        bounds_critical: bounds.crit_vals,
        bounds_p_lower: bounds.p_lower,
        bounds_p_upper: bounds.p_upper,
    })
}

pub(crate) fn vecm_with_forecast(
    values: &[f64], rows: usize, columns: usize, endogenous: &[usize],
    max_lags: usize, deterministic: VecmDeterministic, significance: usize,
    break_index: Option<usize>, forecast_steps: Option<usize>,
) -> Result<AnalysisResult, String> {
    if forecast_steps.is_some_and(|steps| steps == 0 || steps > 200) {
        return Err("Choose between 1 and 200 forecast periods.".to_owned());
    }
    validate_dense_matrix("VECM", values, rows, columns)?;
    if endogenous.len() < 2 || endogenous.iter().any(|&column| column >= columns) {
        return Err("VECM needs at least two endogenous columns inside the matrix".to_owned());
    }
    let mut distinct = endogenous.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != endogenous.len() {
        return Err("VECM endogenous columns must be distinct".to_owned());
    }
    if !(1..=24).contains(&max_lags) {
        return Err("VECM maximum lags must be between 1 and 24".to_owned());
    }
    if significance > 2 {
        return Err("VECM significance column must be 0, 1 or 2".to_owned());
    }
    if rows < (max_lags + 2) * endogenous.len() * 3 + 10 {
        return Err(format!(
            "VECM needs more rows than {} for {} variables at {max_lags} lags",
            (max_lags + 2) * endogenous.len() * 3 + 10,
            endogenous.len()
        ));
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let endog: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            endogenous
                .iter()
                .map(|&column| data[(row, column)])
                .collect()
        })
        .collect();
    let code = deterministic.code();
    let k_ar_diff = vecm_select_order(&endog, max_lags, code).max(1);
    let rank = select_coint_rank(&endog, deterministic.det_order(), k_ar_diff, significance);
    let chow = match break_index {
        Some(index) => {
            if index < 4 || index + 4 > rows {
                return Err(
                    "VECM Chow break index must leave at least four rows on each side".to_owned(),
                );
            }
            let y: Vec<f64> = endog.iter().map(|row| row[0]).collect();
            let x: Vec<f64> = endog.iter().map(|row| row[1]).collect();
            Some(chow_break(&y, &x, index))
        }
        None => None,
    };
    let to_rows = |matrix: &DMatrix<f64>| -> Vec<Vec<f64>> {
        (0..matrix.nrows())
            .map(|i| (0..matrix.ncols()).map(|j| matrix[(i, j)]).collect())
            .collect()
    };
    if rank == 0 {
        return Ok(AnalysisResult::Vecm {
            forecast: if forecast_steps.is_some() {VecmForecast::NotFitted} else {VecmForecast::NotRequested},
            observations: rows,
            long_run: VecmLongRun::NotFitted,
            deterministic,
            k_ar_diff,
            rank,
            significance,
            long_run_effect: None,
            alpha: Vec::new(),
            beta: Vec::new(),
            gamma: Vec::new(),
            pvalues_alpha: Vec::new(),
            chow,
        });
    }
    let fit = vecm_fit(&endog, k_ar_diff, rank, code);
    let forecast = match forecast_steps {
        None => VecmForecast::NotRequested,
        Some(steps) => {
            let forecast = fit.forecast(steps, 0.95).map_err(|_| "The fitted VECM could not produce finite forecasts.".to_owned())?;
            VecmForecast::Recorded {confidence:0.95,mean:forecast.mean,lower:forecast.lower,upper:forecast.upper,covariance:forecast.covariance.iter().map(to_rows).collect()}
        }
    };
    // beta is identity-normalised on top, so with the outcome first the long-run effect of the
    // second variable is the negated second row of the first relation.
    let long_run_effect = if rank == 1 && fit.beta.nrows() >= 2 {
        Some(-fit.beta[(1, 0)])
    } else {
        None
    };
    Ok(AnalysisResult::Vecm {
        forecast,
        observations: rows,
        long_run: VecmLongRun::Recorded {
            start_row: k_ar_diff,
            departures: to_rows(&fit.cointegrating_residuals),
        },
        deterministic,
        k_ar_diff,
        rank,
        significance,
        long_run_effect,
        alpha: to_rows(&fit.alpha),
        beta: to_rows(&fit.beta),
        gamma: to_rows(&fit.gamma),
        pvalues_alpha: to_rows(&fit.pvalues_alpha),
        chow,
    })
}
