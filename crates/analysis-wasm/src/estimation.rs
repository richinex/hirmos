//! Estimator façades: regression, count models, CausalEffects, causal impact, DML, ARDL, VECM, synthetic control, panel DID / SC / SDID, NUTS, discrete BN.

use super::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn backdoor_linear(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    hac_max_lags: Option<usize>,
    level: f64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("backdoor linear estimate", values, rows, columns)?;
    let mut used = vec![treatment, outcome];
    used.extend_from_slice(adjustment);
    if used.iter().any(|&column| column >= columns) {
        return Err("backdoor linear estimate columns must index the numeric matrix".to_owned());
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err(
            "backdoor linear estimate needs distinct treatment, outcome, and adjustment columns"
                .to_owned(),
        );
    }
    let parameters = 2 + adjustment.len();
    if rows <= parameters + 1 {
        return Err(format!(
            "backdoor linear estimate needs more than {} observations for {parameters} parameters",
            parameters + 1
        ));
    }
    if !(0.0..1.0).contains(&level) || level <= 0.5 {
        return Err("backdoor linear estimate level must lie in (0.5, 1)".to_owned());
    }
    if let Some(lags) = hac_max_lags {
        if lags >= rows {
            return Err(
                "backdoor linear estimate HAC lags must be fewer than the observations".to_owned(),
            );
        }
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    for &column in &used {
        let first = data[(0, column)];
        if (0..rows).all(|row| data[(row, column)] == first) {
            return Err("backdoor linear estimate is undefined for a constant column".to_owned());
        }
    }
    if (level - 0.95).abs() > 1e-12 {
        return Err(
            "backdoor linear estimate reports the 95% intervals of the ported estimation layer"
                .to_owned(),
        );
    }
    // Design [1, treatment, adjustment...]; the classical fit comes from the ported OLS, the HAC fit
    // from the ported 805 estimation layer.
    let design = DMatrix::<f64>::from_fn(rows, parameters, |row, column| match column {
        0 => 1.0,
        1 => data[(row, treatment)],
        _ => data[(row, adjustment[column - 2])],
    });
    let y: Vec<f64> = (0..rows).map(|row| data[(row, outcome)]).collect();
    let classical = Ols::fit(&design, &DVector::from_column_slice(&y));
    let degrees_of_freedom = rows - parameters;
    let standard_error = classical.params[1] / classical.tvalues()[1];
    let t = stdtri(degrees_of_freedom as isize, 0.975);
    // statsmodels' bandwidth when `maxlags` is not given: floor(4 (n/100)^(2/9)).
    let max_lags = hac_max_lags
        .unwrap_or_else(|| (4.0 * (rows as f64 / 100.0).powf(2.0 / 9.0)).floor() as usize);
    let hac = ols_hac(&y, &design, max_lags);
    Ok(AnalysisResult::BackdoorLinear {
        observations: rows,
        parameters,
        treatment,
        outcome,
        adjustment: adjustment.to_vec(),
        level,
        estimate: classical.params[1],
        standard_error,
        interval: [
            classical.params[1] - t * standard_error,
            classical.params[1] + t * standard_error,
        ],
        degrees_of_freedom,
        residual_sd: (classical.ssr / degrees_of_freedom as f64).sqrt(),
        r_squared: hac.rsquared,
        hac_max_lags: max_lags,
        hac_standard_error: hac.bse[1],
        hac_interval: [hac.conf_int[1].0, hac.conf_int[1].1],
        hac_p_value: hac.pvalues[1],
        durbin_watson: durbin_watson(&hac.resid),
    })
}

pub(crate) fn count_glm(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    family: CountFamily,
) -> Result<AnalysisResult, String> {
    let (_, design, y) = design_columns(
        "count GLM",
        values,
        rows,
        columns,
        treatment,
        outcome,
        adjustment,
    )?;
    if y.iter().any(|value| *value < 0.0 || value.fract() != 0.0) {
        return Err("count GLM needs a non-negative integer outcome".to_owned());
    }
    if y.iter().all(|value| *value == 0.0) {
        return Err("count GLM is undefined when every outcome is zero".to_owned());
    }
    let z = ndtri(0.975);
    let parameters = design.ncols();
    let (
        coefficient,
        standard_error,
        p_value,
        deviance,
        log_likelihood,
        alpha,
        degrees_of_freedom,
        converged,
        iterations,
    ) = match family {
        CountFamily::Poisson => {
            let fit = poisson_glm(&y, &design);
            (
                fit.params[1],
                fit.bse[1],
                fit.pvalues[1],
                fit.deviance,
                f64::NAN,
                f64::NAN,
                fit.df_resid as usize,
                fit.converged,
                fit.iterations,
            )
        }
        CountFamily::NegativeBinomial => {
            let fit = negative_binomial_p(&y, &design, 2.0);
            let alpha = fit.params[parameters];
            (
                fit.params[1],
                fit.bse[1],
                fit.pvalues[1],
                f64::NAN,
                fit.llf,
                alpha,
                rows - parameters - 1,
                fit.converged,
                fit.nit,
            )
        }
    };
    Ok(AnalysisResult::CountGlm {
        observations: rows,
        parameters,
        treatment,
        outcome,
        adjustment: adjustment.to_vec(),
        family,
        coefficient,
        standard_error,
        p_value,
        incidence_rate_ratio: coefficient.exp(),
        incidence_rate_ratio_interval: [
            (coefficient - z * standard_error).exp(),
            (coefficient + z * standard_error).exp(),
        ],
        level: 0.95,
        deviance,
        log_likelihood,
        alpha,
        degrees_of_freedom,
        converged,
        iterations,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn causal_effects_total(
    values: &[f64],
    rows: usize,
    columns: usize,
    stat_lag: usize,
    graph: &[Vec<Vec<String>>],
    x: &[Node],
    y: &[Node],
    hidden: &[Node],
    estimator: TotalEffectEstimator,
    interventions: [f64; 2],
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("causal effects", values, rows, columns)?;
    if graph.len() != columns
        || graph.iter().any(|targets| {
            targets.len() != columns || targets.iter().any(|lags| lags.len() != stat_lag + 1)
        })
    {
        return Err(
            "causal effects graph must be variables by variables by stat_lag + 1 marks".to_owned(),
        );
    }
    if x.is_empty() || y.len() != 1 {
        return Err("causal effects needs at least one X node and exactly one Y node".to_owned());
    }
    for &(variable, lag) in x.iter().chain(y).chain(hidden) {
        if variable >= columns || lag > 0 || lag.unsigned_abs() as usize > stat_lag + 1 {
            return Err(
                "causal effects nodes must name a variable at a non-positive lag within the window"
                    .to_owned(),
            );
        }
    }
    if !interventions.iter().all(|value| value.is_finite()) {
        return Err("causal effects intervention values must be finite".to_owned());
    }
    let mut stationary = StationaryGraph::new(columns, stat_lag);
    for (i, targets) in graph.iter().enumerate() {
        for (j, lags) in targets.iter().enumerate() {
            for (tau, text) in lags.iter().enumerate() {
                if text.is_empty() {
                    continue;
                }
                if text.len() != 3 {
                    return Err(format!(
                        "causal effects mark {text:?} is not a three character tigramite mark"
                    ));
                }
                stationary.set(i, j, tau, mark(text));
            }
        }
    }
    let effects = CausalEffects::new(stationary, x, y, &[], hidden);
    let mediators: Vec<Node> = effects.mediators.iter().copied().collect();
    let adjustment_set = effects.get_optimal_set();
    let no_causal_path = effects.no_causal_path;
    let (identifiable, adjustment, predictions, total_effect, fitted) = match adjustment_set {
        Some(set) if !no_causal_path => {
            let data = time_series_from_column_major(values, rows, columns);
            let chosen = match estimator {
                TotalEffectEstimator::Linear => Estimator::Linear,
                TotalEffectEstimator::Knn { k } => Estimator::KNeighbors { k: k.max(1) },
            };
            let model = effects
                .fit_total_effect(&data, chosen)
                .ok_or_else(|| "causal effects could not fit the total effect model".to_owned())?;
            let predicted =
                model.predict_total_effect(&[vec![interventions[0]], vec![interventions[1]]]);
            let effect = predicted[1] - predicted[0];
            (true, set, predicted, effect, model.n_obs)
        }
        _ => (false, Vec::new(), Vec::new(), f64::NAN, 0),
    };
    Ok(AnalysisResult::CausalEffectsTotal {
        observations: rows,
        tau_max: effects.tau_max,
        no_causal_path,
        identifiable,
        adjustment_set: adjustment,
        mediators,
        estimator,
        interventions,
        predictions,
        total_effect,
        fitted_observations: fitted,
    })
}

pub(crate) fn causal_impact_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    outcome: usize,
    controls: &[usize],
    n_pre: usize,
    max_iter: usize,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("causal impact", values, rows, columns)?;
    if outcome >= columns
        || controls
            .iter()
            .any(|&column| column >= columns || column == outcome)
    {
        return Err(
            "causal impact outcome and controls must be distinct numeric columns".to_owned(),
        );
    }
    if n_pre < 8 || n_pre >= rows {
        return Err("causal impact needs at least 8 pre-intervention rows and at least one post-intervention row".to_owned());
    }
    if !(1..=2000).contains(&max_iter) {
        return Err("causal impact maxIter must be between 1 and 2000".to_owned());
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let y: Vec<f64> = (0..rows).map(|row| data[(row, outcome)]).collect();
    let exog: Vec<Vec<f64>> = (0..rows)
        .map(|row| controls.iter().map(|&column| data[(row, column)]).collect())
        .collect();
    let impact = causal_impact(&y, &exog, n_pre, max_iter);
    Ok(AnalysisResult::CausalImpact {
        observations: rows,
        n_pre,
        n_post: rows - n_pre,
        outcome,
        controls: controls.to_vec(),
        counterfactual: impact.counterfactual,
        counterfactual_se: impact.counterfactual_se,
        pointwise: impact.pointwise,
        cumulative: impact.cumulative,
        average: impact.average,
        params: impact.params,
        log_likelihood: impact.llf,
    })
}

/// The DoubleML study frame: covariate rows, outcome, treatment, and the model flags.
pub(crate) fn dml_frame(
    label: &str,
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    model: DmlModel,
) -> Result<(Vec<Vec<f64>>, Vec<f64>, Vec<f64>, bool), String> {
    let (data, _, y) =
        design_columns(label, values, rows, columns, treatment, outcome, adjustment)?;
    if adjustment.is_empty() {
        return Err(format!(
            "{label} needs at least one adjustment column for the nuisance learners"
        ));
    }
    if rows < 20 {
        return Err(format!("{label} needs at least 20 rows for cross-fitting"));
    }
    let d: Vec<f64> = (0..rows).map(|row| data[(row, treatment)]).collect();
    let treat_binary = d.iter().all(|&value| value == 0.0 || value == 1.0);
    if model == DmlModel::Irm {
        if !treat_binary {
            return Err(format!(
                "{label} IRM needs a binary treatment coded 0 and 1"
            ));
        }
        let treated = d.iter().filter(|&&value| value == 1.0).count();
        if treated < 5 || rows - treated < 5 {
            return Err(format!(
                "{label} IRM needs at least five treated and five untreated rows"
            ));
        }
    }
    let x: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            adjustment
                .iter()
                .map(|&column| data[(row, column)])
                .collect()
        })
        .collect();
    Ok((x, y, d, treat_binary))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn double_ml(
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
        "double machine learning",
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
    let fit = worker_fit(&study, &mut fold_stream);
    Ok(AnalysisResult::DoubleMl {
        observations: rows,
        model,
        att,
        treat_binary,
        seed,
        estimate: fit.coef,
        standard_error: fit.se,
        interval: (fit.ci_low, fit.ci_high),
        level: 0.95,
    })
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

pub(crate) fn vecm(
    values: &[f64],
    rows: usize,
    columns: usize,
    endogenous: &[usize],
    max_lags: usize,
    deterministic: VecmDeterministic,
    significance: usize,
    break_index: Option<usize>,
) -> Result<AnalysisResult, String> {
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
            observations: rows,
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
    // beta is identity-normalised on top, so with the outcome first the long-run effect of the
    // second variable is the negated second row of the first relation.
    let long_run_effect = if rank == 1 && fit.beta.nrows() >= 2 {
        Some(-fit.beta[(1, 0)])
    } else {
        None
    };
    Ok(AnalysisResult::Vecm {
        observations: rows,
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

pub(crate) fn synthetic_control(
    values: &[f64],
    rows: usize,
    columns: usize,
    treated: usize,
    donors: &[usize],
    n_pre: usize,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("synthetic control", values, rows, columns)?;
    let mut used = vec![treated];
    used.extend_from_slice(donors);
    if donors.is_empty() || used.iter().any(|&column| column >= columns) {
        return Err("synthetic control needs a treated column and at least one donor column inside the matrix".to_owned());
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err("synthetic control treated and donor columns must be distinct".to_owned());
    }
    if n_pre < 2 || n_pre >= rows {
        return Err("synthetic control needs at least 2 pre-intervention rows and at least one post-intervention row".to_owned());
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let n_post = rows - n_pre;
    let y_pre_co = DMatrix::from_fn(n_pre, donors.len(), |row, j| data[(row, donors[j])]);
    let y_post_co = DMatrix::from_fn(n_post, donors.len(), |row, j| {
        data[(n_pre + row, donors[j])]
    });
    let y_pre_tr = DVector::from_fn(n_pre, |row, _| data[(row, treated)]);
    let y_post_tr = DVector::from_fn(n_post, |row, _| data[(n_pre + row, treated)]);
    let (control, effect) = synthetic_effect(&y_pre_co, &y_pre_tr, &y_post_co, &y_post_tr);
    let treated_series: Vec<f64> = (0..rows).map(|row| data[(row, treated)]).collect();
    let synthetic: Vec<f64> = (0..rows)
        .map(|row| {
            donors
                .iter()
                .zip(&control.weights)
                .map(|(&column, weight)| data[(row, column)] * weight)
                .sum()
        })
        .collect();
    Ok(AnalysisResult::SyntheticControl {
        observations: rows,
        n_pre,
        n_post,
        weights: control.weights,
        loss: control.loss,
        iterations: control.iterations,
        pre_gap: effect.pre_gap,
        post_gap: effect.post_gap,
        att: effect.att,
        treated: treated_series,
        synthetic,
    })
}

fn panel_method_evidence(
    estimate: hirmos_causal_core::panel::PanelEstimate,
) -> PanelMethodEvidence {
    PanelMethodEvidence {
        estimate: estimate.estimate,
        lambda: estimate.lambda,
        omega: estimate.omega,
        effect_curve: estimate.effect_curve,
        lambda_iterations: estimate.lambda_iterations,
        omega_iterations: estimate.omega_iterations,
        lambda_objective: estimate.lambda_objective,
        omega_objective: estimate.omega_objective,
        noise_level: estimate.noise_level,
    }
}

fn panel_problem(error: hirmos_causal_core::panel::PanelError) -> String {
    use hirmos_causal_core::panel::PanelError;
    match error {
        PanelError::Empty => "the panel has no observations".to_owned(),
        PanelError::EmptyUnit { row } => format!("the unit key is empty at row {}", row + 1),
        PanelError::NonFiniteOutcome { row } => format!("the outcome is not finite at row {}", row + 1),
        PanelError::NonFiniteTreatment { row } => format!("the treatment is not finite at row {}", row + 1),
        PanelError::TreatmentNotBinary { row } => format!("the treatment is not binary at row {}", row + 1),
        PanelError::DuplicateCell { unit, time } => format!("unit {unit} has more than one observation at time code {time}"),
        PanelError::MissingCell { unit, time } => format!("unit {unit} has no observation at time code {time}"),
        PanelError::NoTreatmentVariation => "the treatment has no zero-to-one variation".to_owned(),
        PanelError::NoPreTreatmentPeriod => "the panel has no pre-treatment period".to_owned(),
        PanelError::NoPostTreatmentPeriod => "the panel has no post-treatment period".to_owned(),
        PanelError::NoControlUnit => "the panel has no never-treated control unit".to_owned(),
        PanelError::NoTreatedUnit => "the panel has no treated unit".to_owned(),
        PanelError::NonSimultaneousAdoption => "treated units do not adopt simultaneously and remain treated".to_owned(),
        PanelError::InvalidMatrixBoundary => "the panel boundary does not contain controls, treated units, pre-periods, and post-periods".to_owned(),
        PanelError::DegenerateNoise => "the control pre-period has no usable first-difference variation for synthetic weighting".to_owned(),
    }
}

pub(crate) fn panel_intervention(
    values: &[f64],
    rows: usize,
    units: &[String],
    times: &[i64],
    progress: impl Fn(&'static str, usize, usize),
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("panel intervention", values, rows, 2)?;
    if units.len() != rows || times.len() != rows {
        return Err("panel intervention needs one unit and time key per numeric row".to_owned());
    }
    let observations = (0..rows)
        .map(|row| hirmos_causal_core::panel::PanelObservation {
            unit: units[row].clone(),
            time: times[row],
            outcome: values[row],
            treatment: values[rows + row],
        })
        .collect::<Vec<_>>();
    let panel = hirmos_causal_core::panel::panel_matrices(&observations).map_err(panel_problem)?;
    progress("validated-panel", 1, 4);
    let did = hirmos_causal_core::panel::did_estimate(&panel.y, panel.n0, panel.t0)
        .map_err(panel_problem)?;
    progress("difference-in-differences", 2, 4);
    let synthetic_control =
        hirmos_causal_core::panel::synthdid_sc_estimate(&panel.y, panel.n0, panel.t0)
            .map_err(panel_problem)?;
    progress("synthetic-control", 3, 4);
    let synthetic_did =
        hirmos_causal_core::panel::synthetic_did_estimate(&panel.y, panel.n0, panel.t0)
            .map_err(panel_problem)?;
    progress("synthetic-did", 4, 4);
    Ok(AnalysisResult::PanelIntervention {
        observations: rows,
        treated_units: panel.y.nrows() - panel.n0,
        control_units: panel.n0,
        n_pre: panel.t0,
        n_post: panel.y.ncols() - panel.t0,
        units: panel.units,
        times: panel.times,
        did: panel_method_evidence(did),
        synthetic_control: panel_method_evidence(synthetic_control),
        synthetic_did: panel_method_evidence(synthetic_did),
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn negbin_nuts(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    confounder: usize,
    warmup: usize,
    samples: usize,
    seed: u64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("negative binomial NUTS", values, rows, columns)?;
    let used = [treatment, outcome, confounder];
    if used.iter().any(|&column| column >= columns)
        || treatment == outcome
        || treatment == confounder
        || outcome == confounder
    {
        return Err("negative binomial NUTS needs distinct treatment, outcome, and confounder columns inside the matrix".to_owned());
    }
    if rows < 20 {
        return Err("negative binomial NUTS needs at least 20 rows".to_owned());
    }
    if !(10..=5000).contains(&warmup) || !(10..=5000).contains(&samples) {
        return Err(
            "negative binomial NUTS warmup and samples must be between 10 and 5000".to_owned(),
        );
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let column = |index: usize| -> Vec<f64> { (0..rows).map(|row| data[(row, index)]).collect() };
    let y = column(outcome);
    if y.iter().any(|value| *value < 0.0 || value.fract() != 0.0) {
        return Err("negative binomial NUTS outcome must hold non-negative integers".to_owned());
    }
    let d = column(treatment);
    let w = column(confounder);
    let spread = |series: &[f64]| -> bool {
        let mean = series.iter().sum::<f64>() / rows as f64;
        series.iter().any(|value| (value - mean).abs() > 0.0)
    };
    if !spread(&d) || !spread(&w) {
        return Err("negative binomial NUTS treatment and confounder must vary".to_owned());
    }
    let model = PbcNegBinModel::from_raw(&d, &w, &y);
    let options = NutsOptions {
        warmup,
        samples,
        ..NutsOptions::default()
    };
    let result = model.sample(options, seed);
    let draws = model.irr_draws(&result);
    let summary = irr_summary(&draws);
    let betas: Vec<f64> = result.samples.iter().map(|draw| draw[1]).collect();
    let beta_mean = betas.iter().sum::<f64>() / betas.len() as f64;
    let beta_sd = (betas
        .iter()
        .map(|value| (value - beta_mean).powi(2))
        .sum::<f64>()
        / (betas.len() as f64 - 1.0))
        .sqrt();
    let dispersion_mean =
        result.samples.iter().map(|draw| draw[3].exp()).sum::<f64>() / result.samples.len() as f64;
    Ok(AnalysisResult::NegbinNuts {
        observations: rows,
        warmup,
        samples,
        seed,
        irr_median: summary.median,
        irr_lower: summary.lower,
        irr_upper: summary.upper,
        beta_treatment_mean: beta_mean,
        beta_treatment_sd: beta_sd,
        dispersion_mean,
        divergences: result.divergences,
        acceptance_rate: result.acceptance_rate,
        mean_accept_probability: result.mean_accept_probability,
        step_size: result.step_size,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn discrete_bn_query(
    values: &[f64],
    rows: usize,
    columns: usize,
    nodes: &[usize],
    names: &[String],
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    bins: usize,
    equivalent_sample_size: f64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("discrete BN query", values, rows, columns)?;
    if nodes.len() < 2
        || nodes.len() != names.len()
        || nodes.iter().any(|&column| column >= columns)
    {
        return Err(
            "discrete BN query needs at least two nodes, each with a name and a matrix column"
                .to_owned(),
        );
    }
    if treatment >= nodes.len() || outcome >= nodes.len() || treatment == outcome {
        return Err(
            "discrete BN query treatment and outcome must be distinct node positions".to_owned(),
        );
    }
    if edges
        .iter()
        .any(|&(cause, effect)| cause >= nodes.len() || effect >= nodes.len() || cause == effect)
    {
        return Err("discrete BN query edges must join distinct node positions".to_owned());
    }
    if !(2..=10).contains(&bins) {
        return Err("discrete BN query bins must be between 2 and 10".to_owned());
    }
    if !(equivalent_sample_size > 0.0) || !equivalent_sample_size.is_finite() {
        return Err("discrete BN query equivalent sample size must be positive".to_owned());
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let mut frame: HashMap<String, Vec<String>> = HashMap::new();
    let mut means: Vec<std::collections::BTreeMap<String, f64>> = Vec::with_capacity(nodes.len());
    let mut state_counts = Vec::with_capacity(nodes.len());
    for (position, &column) in nodes.iter().enumerate() {
        let series: Vec<f64> = (0..rows).map(|row| data[(row, column)]).collect();
        let discretised = discretize_805(&series, bins).ok_or_else(|| {
            format!(
                "discrete BN query cannot discretise {}: it is constant",
                names[position]
            )
        })?;
        state_counts.push(discretised.means.len());
        frame.insert(names[position].clone(), discretised.labels);
        means.push(discretised.means);
    }
    let named_edges: Vec<(String, String)> = edges
        .iter()
        .map(|&(cause, effect)| (names[cause].clone(), names[effect].clone()))
        .collect();
    let dag = DiscreteDag::new(&named_edges, names);
    let bn = DiscreteBn::fit(dag, &frame, equivalent_sample_size);
    let treatment_name = &names[treatment];
    let outcome_name = &names[outcome];
    let treatment_states = &bn.state_names[treatment_name];
    let low = treatment_states
        .first()
        .cloned()
        .ok_or("discrete BN query treatment has no states")?;
    let high = treatment_states
        .last()
        .cloned()
        .ok_or("discrete BN query treatment has no states")?;
    if low == high {
        return Err(format!(
            "discrete BN query treatment {treatment_name} has a single state after discretisation"
        ));
    }
    let outcome_means = &means[outcome];
    let expectation = |distribution: &HashMap<String, f64>| -> f64 {
        distribution
            .iter()
            .map(|(state, probability)| {
                outcome_means.get(state).copied().unwrap_or(0.0) * probability
            })
            .sum()
    };
    let ordered = |distribution: HashMap<String, f64>| -> Vec<(String, f64)> {
        let mut entries: Vec<(String, f64)> = distribution.into_iter().collect();
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        entries
    };
    let distribution_low = bn.do_query(outcome_name, treatment_name, &low);
    let distribution_high = bn.do_query(outcome_name, treatment_name, &high);
    let expectations = (
        expectation(&distribution_low),
        expectation(&distribution_high),
    );
    let mut parents_adjusted = bn.dag.parents(treatment_name);
    parents_adjusted.sort();
    Ok(AnalysisResult::DiscreteBnQuery {
        observations: rows,
        bins,
        equivalent_sample_size,
        state_counts,
        treatment_states: (low, high),
        expectations,
        effect: expectations.1 - expectations.0,
        distribution_low: ordered(distribution_low),
        distribution_high: ordered(distribution_high),
        minimal_adjustment_set: bn
            .minimal_adjustment_set(treatment_name, outcome_name)
            .map(|set| set.into_iter().collect()),
        parents_adjusted,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backdoor_linear_serializes_both_intervals() {
        let rows = 120;
        let mut state = 0x9e37_79b9_7f4a_7c15u64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
        };
        let w: Vec<f64> = (0..rows).map(|_| uniform()).collect();
        let t: Vec<f64> = (0..rows).map(|i| 0.8 * w[i] + uniform()).collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| 0.5 * t[i] + 0.7 * w[i] + uniform())
            .collect();
        let mut values = Vec::with_capacity(rows * 3);
        values.extend(&t);
        values.extend(&y);
        values.extend(&w);
        let json = backdoor_linear(&values, rows, 3, 0, 1, &[2], None, 0.95)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("estimate should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "backdoorLinear");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["parameters"], 3);
        assert_eq!(value["hacMaxLags"], 4);
        let estimate = value["estimate"].as_f64().unwrap();
        assert!(value["interval"][0].as_f64().unwrap() < estimate);
        assert!(value["hacInterval"][1].as_f64().unwrap() > estimate);
        assert!(value["hacPValue"].as_f64().unwrap() < 0.05);
        assert!(value["durbinWatson"].as_f64().unwrap() > 1.0);

        assert!(backdoor_linear(&values, rows, 3, 0, 0, &[2], None, 0.95).is_err());
        assert!(backdoor_linear(&values, rows, 3, 0, 1, &[2], Some(rows), 0.95).is_err());
        assert!(backdoor_linear(&values, rows, 3, 0, 1, &[2], None, 0.4).is_err());
    }

    #[test]
    fn count_glm_reports_incidence_rate_ratios_for_both_families() {
        let rows = 150;
        let mut state = 0x1234_5678_9abc_def1u64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64
        };
        let w: Vec<f64> = (0..rows).map(|_| uniform() - 0.5).collect();
        let t: Vec<f64> = (0..rows)
            .map(|i| {
                if uniform() < 0.5 + 0.4 * w[i] {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| (3.0 + 2.0 * t[i] + w[i] + 4.0 * uniform()).floor())
            .collect();
        let mut values = Vec::new();
        values.extend(&t);
        values.extend(&y);
        values.extend(&w);
        for family in [CountFamily::Poisson, CountFamily::NegativeBinomial] {
            let json = count_glm(&values, rows, 3, 0, 1, &[2], family)
                .and_then(|result| {
                    serde_json::to_string(&result).map_err(|error| error.to_string())
                })
                .expect("count model should serialize");
            let value: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(value["kind"], "countGlm");
            assert!(value["incidenceRateRatio"].as_f64().unwrap() > 1.0);
            assert!(
                value["incidenceRateRatioInterval"][0].as_f64().unwrap()
                    < value["incidenceRateRatio"].as_f64().unwrap()
            );
            if matches!(family, CountFamily::Poisson) {
                assert_eq!(value["converged"], true);
            }
        }
        let mut fractional = values.clone();
        fractional[rows] = 2.5;
        assert!(count_glm(&fractional, rows, 3, 0, 1, &[2], CountFamily::Poisson).is_err());
    }

    #[test]
    fn causal_effects_total_identifies_and_predicts_a_linear_chain() {
        let rows = 400;
        let mut state = 0xdead_beef_cafe_f00du64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
        };
        let mut x = vec![0.0; rows];
        let mut y = vec![0.0; rows];
        let mut z = vec![0.0; rows];
        for i in 1..rows {
            z[i] = 0.5 * z[i - 1] + uniform();
            x[i] = 0.6 * z[i - 1] + uniform();
            y[i] = 0.8 * x[i - 1] + 0.4 * z[i - 1] + uniform();
        }
        let mut values = Vec::new();
        values.extend(&x);
        values.extend(&y);
        values.extend(&z);
        let empty = String::new();
        let mut graph = vec![vec![vec![empty.clone(); 2]; 3]; 3];
        graph[2][2][1] = "-->".to_owned();
        graph[2][0][1] = "-->".to_owned();
        graph[0][1][1] = "-->".to_owned();
        graph[2][1][1] = "-->".to_owned();
        let json = causal_effects_total(
            &values,
            rows,
            3,
            1,
            &graph,
            &[(0, -1)],
            &[(1, 0)],
            &[],
            TotalEffectEstimator::Linear,
            [0.0, 1.0],
        )
        .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
        .expect("total effect should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "causalEffectsTotal");
        assert_eq!(value["identifiable"], true);
        let effect = value["totalEffect"].as_f64().unwrap();
        assert!((effect - 0.8).abs() < 0.15, "total effect {effect}");
        assert!(causal_effects_total(
            &values,
            rows,
            3,
            1,
            &graph,
            &[],
            &[(1, 0)],
            &[],
            TotalEffectEstimator::Linear,
            [0.0, 1.0]
        )
        .is_err());
    }

    #[test]
    fn causal_impact_forecasts_the_post_window() {
        let rows = 120;
        let n_pre = 80;
        let control: Vec<f64> = (0..rows)
            .map(|i| (i as f64 / 9.0).sin() + i as f64 * 0.01)
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| 10.0 + 2.0 * control[i] + if i >= n_pre { 3.0 } else { 0.0 })
            .collect();
        let mut values = Vec::new();
        values.extend(&y);
        values.extend(&control);
        let json = causal_impact_evidence(&values, rows, 2, 0, &[1], n_pre, 100)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("impact should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "causalImpact");
        assert_eq!(value["nPost"], rows - n_pre);
        assert_eq!(value["pointwise"].as_array().unwrap().len(), rows - n_pre);
        let average = value["average"].as_f64().unwrap();
        assert!((average - 3.0).abs() < 0.5, "average effect {average}");
        assert!(causal_impact_evidence(&values, rows, 2, 0, &[1], 4, 100).is_err());
    }

    /// Two cointegrated random walks: x is a walk and y follows 2x plus stationary noise.
    fn cointegrated_pair(rows: usize) -> (Vec<f64>, Vec<f64>) {
        let mut stream = Mt19937::seeded(11);
        let mut x = Vec::with_capacity(rows);
        let mut level = 0.0;
        for _ in 0..rows {
            level += stream.next_f64() - 0.5;
            x.push(level);
        }
        let y: Vec<f64> = x
            .iter()
            .map(|value| 2.0 * value + 0.3 * (stream.next_f64() - 0.5))
            .collect();
        (x, y)
    }

    #[test]
    fn ardl_reports_the_long_run_effect_and_the_bounds_test() {
        let rows = 200;
        let (x, y) = cointegrated_pair(rows);
        let values: Vec<f64> = x.iter().chain(y.iter()).copied().collect();
        let result =
            serde_json::to_value(ardl_pss(&values, rows, 2, 0, 1, 4, ArdlTrend::Ct, 4).unwrap())
                .unwrap();
        assert_eq!(result["kind"], "ardlPss");
        let effect = result["longRunEffect"].as_f64().unwrap();
        assert!((effect - 2.0).abs() < 0.3, "long-run effect {effect}");
        assert!(
            result["boundsPUpper"].as_f64().unwrap() < 0.05,
            "bounds p upper {}",
            result["boundsPUpper"]
        );
        assert_eq!(result["boundsCritical"].as_array().unwrap().len(), 4);
        assert!(result["arLag"].as_u64().unwrap() >= 1);
        assert!(ardl_pss(&values, rows, 2, 0, 1, 4, ArdlTrend::C, 4).is_err());
        assert!(ardl_pss(&values, rows, 2, 0, 0, 4, ArdlTrend::Ct, 4).is_err());
    }

    #[test]
    fn vecm_selects_a_rank_and_reports_the_normalised_long_run_relation() {
        let rows = 200;
        let (x, y) = cointegrated_pair(rows);
        let values: Vec<f64> = x.iter().chain(y.iter()).copied().collect();
        let result = serde_json::to_value(
            vecm(
                &values,
                rows,
                2,
                &[1, 0],
                4,
                VecmDeterministic::Co,
                1,
                Some(100),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "vecm");
        assert_eq!(
            result["rank"].as_u64().unwrap(),
            1,
            "rank {}",
            result["rank"]
        );
        let effect = result["longRunEffect"].as_f64().unwrap();
        assert!((effect - 2.0).abs() < 0.3, "long-run effect {effect}");
        assert_eq!(result["beta"].as_array().unwrap().len(), 2);
        assert!(result["chow"].is_array());
        assert!(vecm(&values, rows, 2, &[0], 4, VecmDeterministic::Co, 1, None).is_err());
        assert!(vecm(&values, rows, 2, &[0, 1], 4, VecmDeterministic::Co, 3, None).is_err());
    }

    #[test]
    fn synthetic_control_weights_the_donors_and_reports_the_post_gap() {
        let rows = 40;
        let n_pre = 25;
        let donor_a: Vec<f64> = (0..rows).map(|i| 10.0 + i as f64 * 0.5).collect();
        let donor_b: Vec<f64> = (0..rows).map(|i| 30.0 - i as f64 * 0.2).collect();
        // The treated unit is 0.6 a + 0.4 b before the intervention and jumps by 5 afterwards.
        let treated: Vec<f64> = (0..rows)
            .map(|i| 0.6 * donor_a[i] + 0.4 * donor_b[i] + if i >= n_pre { 5.0 } else { 0.0 })
            .collect();
        let values: Vec<f64> = treated
            .iter()
            .chain(donor_a.iter())
            .chain(donor_b.iter())
            .copied()
            .collect();
        let result =
            serde_json::to_value(synthetic_control(&values, rows, 3, 0, &[1, 2], n_pre).unwrap())
                .unwrap();
        assert_eq!(result["kind"], "syntheticControl");
        let weights: Vec<f64> = result["weights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        assert!(
            (weights[0] - 0.6).abs() < 1e-6 && (weights[1] - 0.4).abs() < 1e-6,
            "weights {weights:?}"
        );
        assert!((result["att"].as_f64().unwrap() - 5.0).abs() < 1e-6);
        assert_eq!(result["synthetic"].as_array().unwrap().len(), rows);
        assert!(synthetic_control(&values, rows, 3, 0, &[0], n_pre).is_err());
        assert!(synthetic_control(&values, rows, 3, 0, &[1, 2], rows).is_err());
    }

    #[test]
    fn panel_facade_preserves_the_official_california_synthetic_did_result() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../causal-core/oracle/fixtures/panel_synthdid.json"
        ))
        .unwrap();
        let strings = |value: &serde_json::Value| {
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let numbers = |value: &serde_json::Value| {
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item.as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        let units = strings(&fixture["input"]["unit"]);
        let times = numbers(&fixture["input"]["time"])
            .into_iter()
            .map(|time| time as i64)
            .collect::<Vec<_>>();
        let outcome = numbers(&fixture["input"]["outcome"]);
        let treatment = numbers(&fixture["input"]["treatment"]);
        let rows = outcome.len();
        let values = outcome.into_iter().chain(treatment).collect::<Vec<_>>();
        let stages = std::cell::RefCell::new(Vec::new());
        let result =
            panel_intervention(&values, rows, &units, &times, |stage, completed, total| {
                stages.borrow_mut().push((stage, completed, total));
            })
            .unwrap();
        let encoded = serde_json::to_value(result).unwrap();
        assert_eq!(encoded["kind"], "panelIntervention");
        assert_eq!(encoded["controlUnits"], 38);
        assert_eq!(encoded["treatedUnits"], 1);
        assert_eq!(encoded["nPre"], 19);
        assert_eq!(encoded["nPost"], 12);
        let actual = encoded["syntheticDid"]["estimate"].as_f64().unwrap();
        let expected = fixture["sdid"]["estimate"].as_f64().unwrap();
        assert!(
            (actual - expected).abs() <= 2e-10,
            "{actual} versus {expected}"
        );
        assert_eq!(stages.borrow().last(), Some(&("synthetic-did", 4, 4)));
    }

    #[test]
    fn negative_binomial_nuts_recovers_a_rate_ratio_above_one() {
        let rows = 120;
        let mut stream = Mt19937::seeded(5);
        let treatment: Vec<f64> = (0..rows).map(|_| stream.next_f64() * 2.0).collect();
        let confounder: Vec<f64> = (0..rows).map(|_| stream.next_f64() - 0.5).collect();
        // Counts whose log mean rises by 0.7 per unit of treatment.
        let outcome: Vec<f64> = (0..rows)
            .map(|i| {
                ((1.0 + 0.7 * treatment[i] + 0.3 * confounder[i]).exp()
                    * (0.75 + 0.5 * stream.next_f64()))
                .round()
            })
            .collect();
        let values: Vec<f64> = treatment
            .iter()
            .chain(outcome.iter())
            .chain(confounder.iter())
            .copied()
            .collect();
        let result =
            serde_json::to_value(negbin_nuts(&values, rows, 3, 0, 1, 2, 150, 300, 9).unwrap())
                .unwrap();
        assert_eq!(result["kind"], "negbinNuts");
        let median = result["irrMedian"].as_f64().unwrap();
        assert!((median.ln() - 0.7).abs() < 0.3, "IRR median {median}");
        assert!(
            result["irrLower"].as_f64().unwrap() < median
                && median < result["irrUpper"].as_f64().unwrap()
        );
        assert!(result["acceptanceRate"].as_f64().unwrap() > 0.5);
        let mut fractional = values.clone();
        fractional[rows] = 0.5;
        assert!(negbin_nuts(&fractional, rows, 3, 0, 1, 2, 150, 300, 9).is_err());
    }

    #[test]
    fn discrete_bn_query_adjusts_for_the_confounder_and_reports_the_do_difference() {
        let rows = 400;
        let mut stream = Mt19937::seeded(21);
        let confounder: Vec<f64> = (0..rows).map(|_| stream.next_f64()).collect();
        let treatment: Vec<f64> = (0..rows)
            .map(|i| confounder[i] + 0.3 * stream.next_f64())
            .collect();
        let outcome: Vec<f64> = (0..rows)
            .map(|i| 2.0 * treatment[i] + 3.0 * confounder[i] + 0.2 * stream.next_f64())
            .collect();
        let values: Vec<f64> = treatment
            .iter()
            .chain(outcome.iter())
            .chain(confounder.iter())
            .copied()
            .collect();
        let names = vec![
            "treatment".to_owned(),
            "outcome".to_owned(),
            "confounder".to_owned(),
        ];
        let edges = vec![(0, 1), (2, 0), (2, 1)];
        let result = serde_json::to_value(
            discrete_bn_query(&values, rows, 3, &[0, 1, 2], &names, &edges, 0, 1, 3, 5.0).unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "discreteBnQuery");
        assert_eq!(result["parentsAdjusted"].as_array().unwrap().len(), 1);
        assert_eq!(result["minimalAdjustmentSet"].as_array().unwrap().len(), 1);
        let effect = result["effect"].as_f64().unwrap();
        let naive = result["expectations"][1].as_f64().unwrap()
            - result["expectations"][0].as_f64().unwrap();
        assert!(
            effect > 0.0 && (effect - naive).abs() < 1e-12,
            "effect {effect}"
        );
        assert_eq!(result["stateCounts"].as_array().unwrap().len(), 3);
        assert!(
            discrete_bn_query(&values, rows, 3, &[0, 1, 2], &names, &edges, 0, 0, 3, 5.0).is_err()
        );
        assert!(
            discrete_bn_query(&values, rows, 3, &[0, 1, 2], &names, &edges, 0, 1, 1, 5.0).is_err()
        );
    }
}
