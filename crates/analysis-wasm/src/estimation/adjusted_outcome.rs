//! Covariate-adjusted outcome models: the adjusted regression and its error models, the count GLMs, the Bayesian fits, the orthogonal-score learners and the T-learner.

use super::*;

pub(crate) fn backdoor_linear(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    hac_max_lags: Option<usize>,
    level: f64,
    error_model: LinearErrorModel,
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
    let error_model = match error_model {
        LinearErrorModel::NeweyWest => LinearErrorEvidence::NeweyWest,
        LinearErrorModel::Arma { p, q, max_iter } => {
            let order = arma_order("backdoor linear estimate", rows, p, q, max_iter)?;
            let fit = fit_arma_regression(&y, &design, order, max_iter);
            LinearErrorEvidence::Arma {
                estimate: fit.params[1],
                standard_error: fit.bse[1],
                interval: [fit.conf_int[1].0, fit.conf_int[1].1],
                p_value: fit.pvalues[1],
                errors: arma_error_evidence(&fit),
            }
        }
    };
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
        error_model,
    })
}

/// The ARMA order and iteration limit a request declares, checked against the sample: an
/// ARMA(0, 0) is the OLS fit already reported, and the state needs rows to start from.
pub(crate) fn arma_order(name: &str, rows: usize, p: usize, q: usize, max_iter: usize) -> Result<ArmaOrder, String> {
    if p + q == 0 {
        return Err(format!("{name} ARMA errors need at least one autoregressive or moving-average term"));
    }
    if p > 12 || q > 12 {
        return Err(format!("{name} ARMA errors are fitted up to order 12"));
    }
    if rows <= 3 * q.max(p) + p + q + 2 {
        return Err(format!("{name} has too few rows to start ARMA({p}, {q}) errors"));
    }
    if max_iter == 0 {
        return Err(format!("{name} needs at least one optimiser iteration"));
    }
    Ok(ArmaOrder { p, q })
}

/// The fitted error process as evidence: its coefficients with normal intervals, the variance,
/// the likelihood and the optimiser's outcome.
pub(crate) fn arma_error_evidence(fit: &ArmaRegressionFit) -> ArmaErrorEvidence {
    let term = |i: usize, name: String| InterruptedTermEvidence {
        name,
        coefficient: fit.params[i],
        standard_error: fit.bse[i],
        p_value: fit.pvalues[i],
        interval: [fit.conf_int[i].0, fit.conf_int[i].1],
    };
    let k = fit.k_exog;
    ArmaErrorEvidence {
        covariance: match &fit.covariance_status {
            hirmos_causal_core::arma_regression::CovarianceStatus::FullRank => ArmaCovarianceEvidence::FullRank,
            hirmos_causal_core::arma_regression::CovarianceStatus::RankDeficient { rank, parameters } => ArmaCovarianceEvidence::RankDeficient { rank: *rank, parameters: *parameters },
        },
        p: fit.order.p,
        q: fit.order.q,
        ar: (0..fit.order.p).map(|i| term(k + i, format!("ar.L{}", i + 1))).collect(),
        ma: (0..fit.order.q).map(|i| term(k + fit.order.p + i, format!("ma.L{}", i + 1))).collect(),
        sigma2: fit.params[fit.params.len() - 1],
        log_likelihood: fit.llf,
        aic: fit.aic,
        bic: fit.bic,
        iterations: fit.iterations,
        converged: fit.converged,
    }
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
pub(crate) fn bayesian_gaussian(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    warmup: usize,
    samples: usize,
    seed: u64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("Bayesian Gaussian regression", values, rows, columns)?;
    let mut used = vec![treatment, outcome];
    used.extend_from_slice(adjustment);
    if used.iter().any(|&column| column >= columns) {
        return Err(
            "Bayesian Gaussian regression columns must index the numeric matrix".to_owned(),
        );
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err("Bayesian Gaussian regression needs distinct treatment, outcome, and adjustment columns".to_owned());
    }
    if rows < 20 {
        return Err("Bayesian Gaussian regression needs at least 20 rows".to_owned());
    }
    if !(10..=5000).contains(&warmup) || !(10..=5000).contains(&samples) {
        return Err(
            "Bayesian Gaussian regression warmup and samples must be between 10 and 5000"
                .to_owned(),
        );
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let column = |index: usize| -> Vec<f64> { (0..rows).map(|row| data[(row, index)]).collect() };
    let d = column(treatment);
    if d.iter().any(|value| *value != 0.0 && *value != 1.0) {
        return Err("Bayesian Gaussian regression needs a 0/1 treatment".to_owned());
    }
    if d.iter().all(|value| *value == d[0]) {
        return Err("Bayesian Gaussian regression treatment must vary".to_owned());
    }
    let y = column(outcome);
    // Non-binary adjustment columns are divided by their population standard deviation so the
    // Normal(0, 1) slope priors are weakly informative regardless of covariate units. The
    // treatment coefficient, and so the effect, is unchanged by this scaling.
    let mut series_by_column: Vec<Vec<f64>> =
        adjustment.iter().map(|&index| column(index)).collect();
    for series in &mut series_by_column {
        if series.iter().all(|value| *value == 0.0 || *value == 1.0) {
            continue;
        }
        let mean = series.iter().sum::<f64>() / rows as f64;
        let sd = (series
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / rows as f64)
            .sqrt();
        if sd > 0.0 {
            for value in series.iter_mut() {
                *value /= sd;
            }
        }
    }
    let covariates: Vec<Vec<f64>> = (0..rows)
        .map(|row| series_by_column.iter().map(|series| series[row]).collect())
        .collect();
    let describe = |error: hirmos_causal_core::bayesian_gaussian::GaussianScmError| {
        format!("Bayesian Gaussian regression failed: {error:?}")
    };
    let model = BayesianGaussianScm::new(d, covariates, y).map_err(describe)?;
    const CHAINS: usize = 3;
    let mut effects = Vec::new();
    let mut sigmas = Vec::new();
    let mut pooled_samples: Vec<Vec<f64>> = Vec::new();
    let mut divergences = 0usize;
    let mut acceptance_sum = 0.0;
    let mut mean_accept_sum = 0.0;
    let mut step_sum = 0.0;
    for chain in 0..CHAINS {
        let options = NutsOptions {
            warmup,
            samples,
            ..NutsOptions::default()
        };
        let result = model.sample_adjustment(options, seed.wrapping_add(chain as u64));
        divergences += result.divergences;
        acceptance_sum += result.acceptance_rate;
        mean_accept_sum += result.mean_accept_probability;
        step_sum += result.step_size;
        effects.extend(
            model
                .adjustment_effect_draws(&result.samples, 0.0, 1.0)
                .map_err(describe)?,
        );
        sigmas.extend(
            result
                .samples
                .iter()
                .map(|draw| draw.last().copied().unwrap_or(0.0).exp()),
        );
        pooled_samples.extend(result.samples);
    }
    let summary = posterior_effect_summary(&effects, 0.94).map_err(describe)?;
    let sigma_mean = sigmas.iter().sum::<f64>() / sigmas.len() as f64;
    // Expected outcome across each covariate under do(0) and do(1), the other covariates at
    // their sample means, summarised per grid point by posterior quantiles across all draws.
    let covariate_count = adjustment.len();
    let column_means: Vec<f64> = (0..covariate_count)
        .map(|index| series_by_column[index].iter().sum::<f64>() / rows as f64)
        .collect();
    let curves: Vec<BayesianGaussianCurve> = (0..covariate_count)
        .map(|target| {
            let series = &series_by_column[target];
            let standardised = !series.iter().all(|value| *value == 0.0 || *value == 1.0);
            let series_low = series.iter().copied().fold(f64::INFINITY, f64::min);
            let series_high = series.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let grid: Vec<f64> = if standardised {
                (0..25)
                    .map(|step| series_low + (series_high - series_low) * step as f64 / 24.0)
                    .collect()
            } else {
                vec![0.0, 1.0]
            };
            let bases: Vec<(f64, f64, f64)> = pooled_samples
                .iter()
                .map(|draw| {
                    let mut base = draw[0];
                    for (index, mean) in column_means.iter().enumerate() {
                        if index != target {
                            base += draw[2 + index] * mean;
                        }
                    }
                    (base, draw[1], draw[2 + target])
                })
                .collect();
            let mut curve = BayesianGaussianCurve {
                standardised,
                grid: grid.clone(),
                control_lower: Vec::new(),
                control_median: Vec::new(),
                control_upper: Vec::new(),
                treated_lower: Vec::new(),
                treated_median: Vec::new(),
                treated_upper: Vec::new(),
            };
            for x in grid {
                let control: Vec<f64> = bases
                    .iter()
                    .map(|(base, _, slope)| base + slope * x)
                    .collect();
                let treated: Vec<f64> = bases
                    .iter()
                    .map(|(base, tau, slope)| base + tau + slope * x)
                    .collect();
                curve.control_lower.push(quantile(&control, 0.03));
                curve.control_median.push(quantile(&control, 0.5));
                curve.control_upper.push(quantile(&control, 0.97));
                curve.treated_lower.push(quantile(&treated, 0.03));
                curve.treated_median.push(quantile(&treated, 0.5));
                curve.treated_upper.push(quantile(&treated, 0.97));
            }
            curve
        })
        .collect();
    let low = effects.iter().copied().fold(f64::INFINITY, f64::min);
    let high = effects.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let bins = 60usize;
    let bin_width = ((high - low) / bins as f64).max(f64::EPSILON);
    let mut counts = vec![0u32; bins];
    for value in &effects {
        let index = (((value - low) / bin_width) as usize).min(bins - 1);
        counts[index] += 1;
    }
    Ok(AnalysisResult::BayesianGaussian {
        observations: rows,
        warmup,
        samples,
        chains: CHAINS,
        seed,
        effect_mean: summary.mean,
        effect_sd: summary.standard_deviation,
        effect_median: summary.median,
        hdi_lower: summary.hdi_lower,
        hdi_upper: summary.hdi_upper,
        probability_positive: summary.probability_positive,
        sigma_mean,
        divergences,
        acceptance_rate: acceptance_sum / CHAINS as f64,
        mean_accept_probability: mean_accept_sum / CHAINS as f64,
        step_size: step_sum / CHAINS as f64,
        histogram_start: low,
        histogram_bin_width: bin_width,
        histogram_counts: counts,
        curves,
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

pub(crate) type DmlFrame = (Vec<Vec<f64>>, Vec<f64>, Vec<f64>, bool);

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
) -> Result<DmlFrame, String> {
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
    groups: DmlGroups,
) -> Result<AnalysisResult, String> {
    let cut = match groups {
        DmlGroups::None => None,
        DmlGroups::Levels { column } => Some(cut_modifier(
            values, rows, columns, column, treatment, outcome, None,
        )?),
        DmlGroups::Quantiles { column, bins } => Some(cut_modifier(
            values,
            rows,
            columns,
            column,
            treatment,
            outcome,
            Some(bins),
        )?),
    };
    // The modifier joins the nuisance inputs, as DoubleML's heterogeneous-effects data passes the
    // covariate that shapes the effect among `x_cols` before `gate` groups on it.
    let nuisance: Vec<usize> = adjustment
        .iter()
        .copied()
        .chain(
            cut.as_ref()
                .map(|cut| cut.column)
                .filter(|column| !adjustment.contains(column)),
        )
        .collect();
    let (x, y, d, treat_binary) = dml_frame(
        "double machine learning",
        values,
        rows,
        columns,
        treatment,
        outcome,
        &nuisance,
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
    let level = 0.95;
    let groups = match cut {
        None => DmlGroupEvidence::None,
        Some(cut) => {
            let effects =
                group_effects(&fit, &cut.labels, cut.bounds.len(), level).map_err(|error| {
                    format!("double machine learning group effects failed: {error}")
                })?;
            DmlGroupEvidence::Grouped {
                modifier: cut.column,
                grouping: cut.grouping,
                level,
                groups: effects
                    .groups
                    .iter()
                    .zip(&cut.bounds)
                    .map(|(group, &(lower, upper))| DmlGroupEffectEvidence {
                        lower,
                        upper,
                        observations: group.observations,
                        effect: group.effect,
                        standard_error: group.standard_error,
                        interval: (group.confidence_interval[0], group.confidence_interval[1]),
                        few_observations: group.few_observations,
                    })
                    .collect(),
            }
        }
    };
    Ok(AnalysisResult::DoubleMl {
        observations: rows,
        model,
        att,
        treat_binary,
        seed,
        estimate: fit.coef,
        standard_error: fit.se,
        interval: (fit.ci_low, fit.ci_high),
        level,
        groups,
    })
}

/// The forest settings the worker uses for every random forest: 200 trees, minimum leaf 5.
const FOREST_TREES: usize = 200;

const FOREST_MIN_LEAF: usize = 5;

/// EconML's T-learner on the adjustment columns, reporting the effect at every prepared row.
pub(crate) fn t_learner(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    seed: u32,
) -> Result<AnalysisResult, String> {
    let label = "T-learner";
    let (data, _, y) =
        design_columns(label, values, rows, columns, treatment, outcome, adjustment)?;
    if adjustment.is_empty() {
        return Err(format!(
            "{label} needs at least one adjustment column as the forests' inputs"
        ));
    }
    let d: Vec<f64> = (0..rows).map(|row| data[(row, treatment)]).collect();
    let x: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            adjustment
                .iter()
                .map(|&column| data[(row, column)])
                .collect()
        })
        .collect();
    let fit = fit_tlearner(&x, &y, &d, FOREST_TREES, FOREST_MIN_LEAF, seed)
        .map_err(|error| format!("{label}: {error}"))?;
    let effects = fit.effect(&x);
    let average = effects.iter().sum::<f64>() / effects.len() as f64;
    Ok(AnalysisResult::TLearner {
        observations: rows,
        control_rows: fit.control_rows,
        treated_rows: fit.treated_rows,
        seed,
        trees: FOREST_TREES,
        min_leaf: FOREST_MIN_LEAF,
        effects,
        average,
        uncertainty: TLearnerUncertaintyEvidence::None,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn t_learner_with_uncertainty(
    values: &[f64], rows: usize, columns: usize, treatment: usize, outcome: usize,
    adjustment: &[usize], seed: u32, uncertainty: TLearnerUncertainty,
) -> Result<AnalysisResult, String> {
    use hirmos_causal_core::tlearner::bootstrap;
    use std::num::NonZeroUsize;
    let (samples, bootstrap_seed, level, method) = match uncertainty {
        TLearnerUncertainty::None => return t_learner(values, rows, columns, treatment, outcome, adjustment, seed),
        TLearnerUncertainty::Bootstrap { samples, seed, level, method } => (samples, seed, level, method),
    };
    if !(2..=1000).contains(&samples) {
        return Err("Choose between 2 and 1,000 bootstrap samples.".into());
    }
    let confidence = bootstrap::Confidence::new(level)
        .ok_or("The confidence level must be between zero and one.")?;
    let (data, _, y) = design_columns("T-learner", values, rows, columns, treatment, outcome, adjustment)?;
    if adjustment.is_empty() { return Err("Choose at least one adjustment variable or effect modifier.".into()); }
    let x: Vec<Vec<f64>> = (0..rows).map(|r| adjustment.iter().map(|&c| data[(r,c)]).collect()).collect();
    let d: Vec<f64> = (0..rows).map(|r| data[(r,treatment)]).collect();
    let fitted = bootstrap::fit(&x, &y, &d, &x,
        &bootstrap::Forest { trees: NonZeroUsize::new(FOREST_TREES).unwrap(), min_leaf: NonZeroUsize::new(FOREST_MIN_LEAF).unwrap(), seed },
        &bootstrap::Bootstrap { samples: NonZeroUsize::new(samples).unwrap(), seed: bootstrap_seed, confidence,
            method: match method { BootstrapMethod::Percentile => bootstrap::IntervalMethod::Percentile,
                BootstrapMethod::Pivot => bootstrap::IntervalMethod::Pivot, BootstrapMethod::Normal => bootstrap::IntervalMethod::Normal } })
        .map_err(|error| match error {
            bootstrap::Error::InvalidData => "The bootstrap inputs or calculated intervals are invalid.".to_string(),
            bootstrap::Error::Fit(error) => format!("T-learner: {error}"),
            bootstrap::Error::Resample { index, error } => format!("Bootstrap sample {} could not be fitted: {error}. No samples were silently replaced.", index + 1),
        })?;
    let treated_rows = d.iter().filter(|&&value| value == 1.).count();
    Ok(AnalysisResult::TLearner {
        observations: rows, control_rows: rows - treated_rows, treated_rows, seed,
        trees: FOREST_TREES, min_leaf: FOREST_MIN_LEAF,
        effects: fitted.effects, average: fitted.average.effect,
        uncertainty: TLearnerUncertaintyEvidence::Bootstrap { samples, seed: bootstrap_seed, level, method,
            intervals: fitted.intervals, standard_errors: fitted.standard_errors,
            average: BootstrapAverage { interval: fitted.average.interval, standard_error_bound: fitted.average.standard_error_bound } },
    })
}

struct ModifierCut {
    column: usize,
    grouping: DmlGroupingEvidence,
    /// Each row's zero-based group.
    labels: Vec<usize>,
    /// Each group's bounds on the modifier; `None` at an open outer edge.
    bounds: Vec<(Option<f64>, Option<f64>)>,
}

const MAX_MODIFIER_LEVELS: usize = 12;

/// Cut the modifier column into groups: one per distinct value, or `bins` quantile groups that are
/// right-inclusive with the lowest bin closed, as `pandas.qcut` cuts them.
fn cut_modifier(
    values: &[f64],
    rows: usize,
    columns: usize,
    column: usize,
    treatment: usize,
    outcome: usize,
    bins: Option<usize>,
) -> Result<ModifierCut, String> {
    if column >= columns {
        return Err("the effect modifier must index the numeric matrix".to_owned());
    }
    if column == treatment || column == outcome {
        return Err("the effect modifier must differ from the treatment and outcome".to_owned());
    }
    let modifier: Vec<f64> = (0..rows).map(|row| values[column * rows + row]).collect();
    if let Some(row) = modifier.iter().position(|value| !value.is_finite()) {
        return Err(format!(
            "the effect modifier has a non-finite value at row {row}"
        ));
    }
    let mut sorted = modifier.clone();
    sorted.sort_by(f64::total_cmp);
    match bins {
        None => {
            let mut levels = sorted.clone();
            levels.dedup();
            if levels.len() > MAX_MODIFIER_LEVELS {
                return Err(format!(
                    "the effect modifier has {} distinct values; group by quantiles or choose a variable with at most {MAX_MODIFIER_LEVELS} levels",
                    levels.len()
                ));
            }
            let labels = modifier
                .iter()
                .map(|value| {
                    levels
                        .iter()
                        .position(|level| level == value)
                        .expect("level present")
                })
                .collect();
            Ok(ModifierCut {
                column,
                grouping: DmlGroupingEvidence::Levels,
                labels,
                bounds: levels
                    .iter()
                    .map(|&level| (Some(level), Some(level)))
                    .collect(),
            })
        }
        Some(bins) => {
            if !(2..=10).contains(&bins) {
                return Err("quantile groups must number between 2 and 10".to_owned());
            }
            // pandas.qcut takes its probabilities from numpy.linspace(0, 1, bins + 1), and pandas.Series.quantile
            // hands them to numpy.percentile as percentages, which divides by 100 again. Repeating both
            // roundings puts a value that sits on an edge in the same group pandas gives it.
            let step = 1.0 / bins as f64;
            let edges: Vec<f64> = (1..bins)
                .map(|k| numpy_percentile(&modifier, (k as f64 * step) * 100.0 / 100.0))
                .collect();
            if edges.windows(2).any(|pair| pair[0] >= pair[1])
                || edges.first() == sorted.first()
                || edges.last() == sorted.last()
            {
                return Err(format!(
                    "the effect modifier has too few distinct values for {bins} quantile groups; group by its levels instead"
                ));
            }
            let labels = modifier
                .iter()
                .map(|value| edges.iter().filter(|edge| value > edge).count())
                .collect();
            let bounds = (0..bins)
                .map(|group| {
                    (
                        (group > 0).then(|| edges[group - 1]),
                        (group + 1 < bins).then(|| edges[group]),
                    )
                })
                .collect();
            Ok(ModifierCut {
                column,
                grouping: DmlGroupingEvidence::Quantiles { bins },
                labels,
                bounds,
            })
        }
    }
}
