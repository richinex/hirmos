//! Intervention and comparative designs: causal impact, synthetic control, panel DiD and synthetic DiD, and the sharp regression discontinuity.

use super::*;

/// The transport has exactly three columns: running variable, outcome, treatment.
/// Sharp assignment is checked against the recorded cutoff, not inferred from Y.
pub(crate) fn sharp_rd(values: &[f64], rows: usize, cutoff: f64) -> Result<AnalysisResult, String> {
    use hirmos_causal_core::rd;
    validate_dense_matrix("Sharp RD", values, rows, 3)?;
    if !cutoff.is_finite() { return Err("Enter a finite cutoff.".to_owned()); }
    let x = &values[..rows];
    let y = &values[rows..2 * rows];
    let treatment = &values[2 * rows..];
    if let Some(row) = (0..rows).find(|&i| treatment[i] != f64::from(x[i] >= cutoff)) {
        return Err(format!("Sharp RD requires treatment = 1 at or above the cutoff and 0 below it. Row {} does not follow that rule. Fuzzy assignment is not supported by this fit.", row + 1));
    }
    let explain = |error| match error {
        rd::Error::Shape => "The running variable and outcome must have the same non-zero number of rows.",
        rd::Error::NonFinite => "RD needs finite running-variable and outcome values.",
        rd::Error::MissingSide => "RD needs observations on both sides of the cutoff.",
        rd::Error::InsufficientSupport => "There are too few distinct running-variable values near the cutoff for this local-linear fit and its bias correction.",
        rd::Error::Singular => "The local RD design is singular. Inspect the running-variable support near the cutoff.",
        rd::Error::InvalidBandwidth => "The automatic bandwidth could not be estimated from these observations.",
    }.to_owned();
    let data = rd::SharpData::new(x, y, cutoff).map_err(explain)?;
    let fit = rd::fit(&data).map_err(explain)?;
    let estimate = |value: rd::Estimate| RdEstimateEvidence { value: value.value, standard_error: value.standard_error, interval: value.interval };
    Ok(AnalysisResult::SharpRd {
        cutoff, target: "local-at-cutoff".to_owned(), assignment: "at-or-above".to_owned(),
        kernel: "triangular".to_owned(), bandwidth_selection: "mserd".to_owned(),
        polynomial_order: 1, bias_order: 2, nearest_neighbors: 3,
        bandwidth: fit.bandwidth.estimation, bias_bandwidth: fit.bandwidth.bias,
        conventional: estimate(fit.conventional), bias_corrected: estimate(fit.bias_corrected), robust: estimate(fit.robust),
        observations: fit.observations, effective_observations: fit.effective_observations,
        left_coefficients: fit.left_coefficients, right_coefficients: fit.right_coefficients,
        points: x.iter().zip(y).map(|(&x, &y)| [x, y]).collect(),
    })
}

pub(crate) fn structural_causal_impact_evidence(
    values: &[f64], rows: usize, columns: usize, outcome: usize,
    controls: &[usize], n_pre: usize, post_end: usize, draws: usize, warmup: usize,
    seed: u32, model: StructuralModel,
) -> Result<AnalysisResult, String> {
    use hirmos_structural_timeseries::impact::{self, Seasonality, Trend};
    use hirmos_causal_core::bayesian_impact::summary;
    validate_dense_matrix("structural causal impact", values, rows, columns)?;
    let unique: std::collections::BTreeSet<_> = controls.iter().copied().collect();
    if outcome >= columns || unique.len() != controls.len() || controls.iter().any(|&c| c >= columns || c == outcome) {
        return Err("Choose distinct outcome and predictor columns.".to_owned());
    }
    if n_pre < 8 || n_pre >= post_end || post_end > rows {
        return Err("Choose at least eight training rows and an evaluation window after them.".to_owned());
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    for column in std::iter::once(&outcome).chain(controls) {
        let first = data[(0,*column)];
        if (0..n_pre).all(|t| data[(t,*column)] == first) {
            return Err(format!("Column {} has no variation before the intervention. Its association cannot be learned from that window.",column+1));
        }
    }
    let y: Vec<_> = (0..post_end).map(|t|data[(t,outcome)]).collect();
    let x = DMatrix::from_fn(post_end,controls.len(),|t,j|data[(t,controls[j])]);
    let trend = match model.trend { StructuralTrend::Level => Trend::Level, StructuralTrend::Linear => Trend::Linear, StructuralTrend::Semilocal => Trend::Semilocal };
    let seasonality = match model.seasonality {
        StructuralSeasonality::None => Ok(Seasonality::None),
        StructuralSeasonality::Seasonal { seasons, duration } => Seasonality::seasonal(seasons,duration),
        StructuralSeasonality::Harmonic { period, pairs } => Seasonality::harmonic(period,pairs),
    }.map_err(|_| "Review the seasonal period, duration and harmonic pairs.".to_owned())?;
    let plan = impact::Plan::new(&y[..n_pre],x,trend,seasonality,draws,warmup,seed)
        .map_err(|_| "The training data or sampling settings cannot define this structural model.".to_owned())?;
    let fit = impact::fit(&plan, |_| std::ops::ControlFlow::Continue(()))
        .map_err(|_| "The structural model could not complete sampling. No partial result was saved. Review the predictors, seasonal specification and training window.".to_owned())?;
    let predictions = summary::Predictions::new(&y,&fit.means,&fit.paths,n_pre..post_end,0.05)
        .map_err(|_| "The posterior paths could not be summarised.".to_owned())?;
    let aggregate = summary::summarize(&predictions)
        .map_err(|_| "The posterior effect summary is undefined, including its relative effect.".to_owned())?;
    let post = summary::post_path(&predictions).map_err(|_| "The posterior effect path is undefined.".to_owned())?;
    let before = summary::Predictions::new(&y,&fit.means,&fit.paths,0..n_pre,0.05)
        .and_then(|p| summary::post_path(&p)).map_err(|_| "The training path could not be summarised.".to_owned())?;
    let quantity = |q: &summary::Quantity| PosteriorQuantity { mean:q.mean, lower:q.interval.lower, upper:q.interval.upper, sd:q.sd };
    let report = |s: &summary::Summary| ImpactSummary { actual:s.actual,predicted:quantity(&s.predicted),absolute:quantity(&s.absolute),relative:quantity(&s.relative),tail_probability:s.tail_probability };
    let contributions = fit.contributions.iter().map(|c| StructuralContribution {
        component: match c.component { impact::Component::Trend => StructuralComponent::Trend, impact::Component::Seasonal => StructuralComponent::Seasonal, impact::Component::Predictor(j) => StructuralComponent::Predictor { column: controls[j] } },
        mean: c.paths.iter().map(|p|p.iter().sum::<f64>()/p.len() as f64).collect(),
        lower: c.paths.iter().map(|p|hirmos_causal_core::negbin_nuts::quantile(p,0.025)).collect(),
        upper: c.paths.iter().map(|p|hirmos_causal_core::negbin_nuts::quantile(p,0.975)).collect(),
    }).collect();
    Ok(AnalysisResult::StructuralCausalImpact { model, contributions, posterior: ImpactPosterior {
        control_inclusion: controls.iter().zip(&fit.inclusion_probabilities).map(|(&column,&probability)| ControlInclusion { column,probability }).collect(),
        observations: rows, n_pre, n_post:post_end-n_pre, post_end, outcome, controls:controls.to_vec(), draws, warmup, seed, level:0.95,
        pre_intervention_path: PreInterventionPath::Sampled { steps:(0..n_pre).collect(),observed:y[..n_pre].to_vec(),counterfactual:before.iter().map(|p|p.predicted.mean).collect(),lower:before.iter().map(|p|p.predicted.interval.lower).collect(),upper:before.iter().map(|p|p.predicted.interval.upper).collect() },
        counterfactual:post.iter().map(|p|p.predicted.mean).collect(),counterfactual_se:post.iter().map(|p|p.predicted.sd).collect(),
        counterfactual_lower:post.iter().map(|p|p.predicted.interval.lower).collect(),counterfactual_upper:post.iter().map(|p|p.predicted.interval.upper).collect(),
        pointwise:post.iter().map(|p|p.effect.mean).collect(),pointwise_lower:post.iter().map(|p|p.effect.interval.lower).collect(),pointwise_upper:post.iter().map(|p|p.effect.interval.upper).collect(),
        cumulative_lower:post.iter().map(|p|p.cumulative.interval.lower).collect(),cumulative_upper:post.iter().map(|p|p.cumulative.interval.upper).collect(),
        cumulative:aggregate.cumulative.absolute.mean,average:aggregate.average.absolute.mean,
        average_summary:report(&aggregate.average),cumulative_summary:report(&aggregate.cumulative),
    } })
}

pub(crate) fn bayesian_causal_impact_evidence(
    values: &[f64], rows: usize, columns: usize, outcome: usize,
    controls: &[usize], n_pre: usize, post_end: usize, draws: usize, warmup: usize,
    seed: u32, prior_level_sd: f64,
) -> Result<AnalysisResult, String> {
    use hirmos_causal_core::bayesian_impact::{self, summary};
    validate_dense_matrix("Bayesian causal impact", values, rows, columns)?;
    let unique: std::collections::BTreeSet<_> = controls.iter().copied().collect();
    if outcome >= columns || unique.len() != controls.len()
        || controls.iter().any(|&column| column >= columns || column == outcome) {
        return Err("Choose distinct outcome and control columns.".to_owned());
    }
    if n_pre < 8 || n_pre >= rows {
        return Err("Bayesian causal impact needs at least eight pre-intervention rows and one post-intervention row.".to_owned());
    }
    if post_end <= n_pre || post_end > rows {
        return Err("The evaluated window must end after the intervention row and within the observations.".to_owned());
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let y: Vec<_> = (0..rows).map(|t| data[(t,outcome)]).collect();
    let x = DMatrix::from_fn(rows, controls.len(), |t,j| data[(t,controls[j])]);
    let plan = bayesian_impact::Plan::new(&y, &x, n_pre, n_pre..post_end, draws, warmup, seed, prior_level_sd)
        .map_err(bayesian_impact_problem)?;
    let fit = bayesian_impact::fit(&plan)
        .map_err(bayesian_impact_problem)?;
    // The sampler already produced a predictive path for every row; read the fitted window off
    // it with the same summariser the evaluated window uses.
    let before = summary::Predictions::new(&y, &fit.means, &fit.paths, 0..n_pre, 0.05)
        .and_then(|predictions| summary::post_path(&predictions))
        .map_err(|_| "The pre-intervention path could not be summarised.".to_owned())?;
    let pre_intervention_path = PreInterventionPath::Sampled {
        steps: (0..n_pre).collect(),
        observed: y[..n_pre].to_vec(),
        counterfactual: before.iter().map(|point| point.predicted.mean).collect(),
        lower: before.iter().map(|point| point.predicted.interval.lower).collect(),
        upper: before.iter().map(|point| point.predicted.interval.upper).collect(),
    };
    let quantity = |q: &summary::Quantity| PosteriorQuantity { mean:q.mean, lower:q.interval.lower, upper:q.interval.upper, sd:q.sd };
    let aggregate = |s: &summary::Summary| ImpactSummary {
        actual:s.actual, predicted:quantity(&s.predicted), absolute:quantity(&s.absolute),
        relative:quantity(&s.relative), tail_probability:s.tail_probability,
    };
    Ok(AnalysisResult::BayesianCausalImpact {
        control_inclusion: controls.iter().zip(&fit.inclusion_probabilities).map(|(&column,&probability)| ControlInclusion { column,probability }).collect(),
        observations:rows, n_pre, n_post:post_end-n_pre, post_end, outcome, controls:controls.to_vec(),
        draws, warmup, seed, prior_level_sd, level:0.95, pre_intervention_path,
        counterfactual:fit.post_path.iter().map(|p|p.predicted.mean).collect(),
        counterfactual_se:fit.post_path.iter().map(|p|p.predicted.sd).collect(),
        counterfactual_lower:fit.post_path.iter().map(|p|p.predicted.interval.lower).collect(),
        counterfactual_upper:fit.post_path.iter().map(|p|p.predicted.interval.upper).collect(),
        pointwise:fit.post_path.iter().map(|p|p.effect.mean).collect(),
        pointwise_lower:fit.post_path.iter().map(|p|p.effect.interval.lower).collect(),
        pointwise_upper:fit.post_path.iter().map(|p|p.effect.interval.upper).collect(),
        cumulative_lower:fit.post_path.iter().map(|p|p.cumulative.interval.lower).collect(),
        cumulative_upper:fit.post_path.iter().map(|p|p.cumulative.interval.upper).collect(),
        cumulative:fit.summaries.cumulative.absolute.mean, average:fit.summaries.average.absolute.mean,
        average_summary:aggregate(&fit.summaries.average),
        cumulative_summary:aggregate(&fit.summaries.cumulative),
    })
}

fn bayesian_impact_problem(error: hirmos_causal_core::bayesian_impact::Error) -> String {
    use hirmos_causal_core::bayesian_impact::Error;
    match error {
        Error::Shape => "Outcome and control columns must have matching row counts.",
        Error::NonFinite => "The outcome and controls must contain finite values; resolve missing or infinite values before fitting.",
        Error::Window => "Choose a pre-intervention fitting window followed by a nonempty post-intervention window.",
        Error::ConstantOutcome => "The outcome has no usable variation before the intervention.",
        Error::ConstantControl => "A control has no usable variation before the intervention. Remove constant controls.",
        Error::Settings => "Choose at least two posterior draws, nonnegative warmup and a finite positive level-prior standard deviation.",
        Error::Singular => "The control regression is singular. Check for duplicate or linearly dependent control columns.",
        Error::Numerical => "Posterior sampling could not produce finite, valid quantities. Inspect the outcome scale and control dependence; no result was saved.",
    }.to_owned()
}

pub(crate) fn causal_impact_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    outcome: usize,
    controls: &[usize],
    n_pre: usize,
    post_end: usize,
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
    if post_end <= n_pre || post_end > rows {
        return Err("The evaluated window must end after the intervention row and within the observations.".to_owned());
    }
    if !(1..=2000).contains(&max_iter) {
        return Err("causal impact maxIter must be between 1 and 2000".to_owned());
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let y: Vec<f64> = (0..rows).map(|row| data[(row, outcome)]).collect();
    let exog: Vec<Vec<f64>> = (0..rows)
        .map(|row| controls.iter().map(|&column| data[(row, column)]).collect())
        .collect();
    let impact = causal_impact(&y, &exog, n_pre, n_pre..post_end, max_iter);
    Ok(AnalysisResult::CausalImpact {
        observations: rows,
        n_pre,
        n_post: post_end - n_pre,
        post_end,
        outcome,
        pre_intervention_path: PreInterventionPath::Fitted {
            observed: impact.pre_steps.iter().map(|&row| y[row]).collect(),
            steps: impact.pre_steps,
            counterfactual: impact.pre_counterfactual,
            se: impact.pre_counterfactual_se,
        },
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

pub(crate) fn synthetic_control(
    values: &[f64],
    rows: usize,
    columns: usize,
    treated: usize,
    donors: &[usize],
    n_pre: usize,
    cross_fit_folds: usize,
    alpha: f64,
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
    if cross_fit_folds < 2 {
        return Err("synthetic control cross-fitting needs at least 2 folds".to_owned());
    }
    if !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
        return Err("synthetic control inference alpha must be between zero and one".to_owned());
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
    let cross_fit = match debiased_synthetic_control(
        &y_pre_co,
        &y_pre_tr,
        &y_post_co,
        &y_post_tr,
        cross_fit_folds,
    ) {
        Ok(inference) => SyntheticCrossFitEvidence::Available {
            att: inference.att,
            standard_error: inference.standard_error,
            t_statistic: inference.t_statistic,
            degrees_of_freedom: inference.degrees_of_freedom,
            p_value: inference.p_value,
            confidence_interval: inference.confidence_interval,
            block_size: inference.block_size,
            folds: inference
                .folds
                .into_iter()
                .map(|fold| SyntheticCrossFitFoldEvidence {
                    held_out: fold.held_out,
                    weights: fold.weights,
                    bias: fold.bias,
                    att: fold.att,
                })
                .collect(),
        },
        Err(error) => SyntheticCrossFitEvidence::Unavailable {
            reason: synthetic_control_problem(error),
        },
    };
    let outcome_controls = DMatrix::from_fn(rows, donors.len(), |row, column| {
        data[(row, donors[column])]
    });
    let outcome_treated = DVector::from_fn(rows, |row, _| data[(row, treated)]);
    let donor_placebo = match donor_placebo_mspe_inference(
        &y_pre_co,
        &y_pre_tr,
        &outcome_controls,
        &outcome_treated,
        n_pre,
    ) {
        Ok(inference) => DonorPlaceboInferenceEvidence::Available {
            treated_pre_mspe: inference.treated.pre_mspe,
            treated_post_mspe: inference.treated.post_mspe,
            treated_mspe_ratio: inference
                .treated
                .mspe_ratio
                .is_finite()
                .then_some(inference.treated.mspe_ratio),
            placebos: inference
                .placebos
                .into_iter()
                .map(|placebo| DonorPlaceboEvidence {
                    donor: placebo.donor,
                    pre_mspe: placebo.summary.pre_mspe,
                    post_mspe: placebo.summary.post_mspe,
                    mspe_ratio: placebo
                        .summary
                        .mspe_ratio
                        .is_finite()
                        .then_some(placebo.summary.mspe_ratio),
                })
                .collect(),
            p_value: inference.p_value,
            n_valid_placebos: inference.n_valid_placebos,
        },
        Err(error) => DonorPlaceboInferenceEvidence::Unavailable {
            reason: synthetic_control_problem(error),
        },
    };
    let band = |method| match synthetic_control_prediction_band(
        &outcome_controls,
        &outcome_treated,
        &control.weights,
        n_pre,
        alpha,
        method,
    ) {
        Ok(inference) if inference.half_width.is_finite() => {
            SyntheticPredictionBandEvidence::Available {
                alpha: inference.alpha,
                intervals: inference.intervals,
                half_width: inference.half_width,
                pre_mspe: inference.pre_mspe,
                post_mspe: inference.post_mspe,
                mspe_ratio: inference
                    .mspe_ratio
                    .is_finite()
                    .then_some(inference.mspe_ratio),
            }
        }
        Ok(_) => SyntheticPredictionBandEvidence::Unavailable {
            reason: "the requested conformal level needs more pre-intervention residuals"
                .to_owned(),
        },
        Err(error) => SyntheticPredictionBandEvidence::Unavailable {
            reason: synthetic_control_problem(error),
        },
    };
    let conformal_band = band(SyntheticControlBandMethod::Conformal);
    let gaussian_band = band(SyntheticControlBandMethod::ParametricGaussian);
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
        cross_fit,
        donor_placebo,
        conformal_band,
        gaussian_band,
    })
}

fn synthetic_control_problem(error: SyntheticControlError) -> String {
    match error {
        SyntheticControlError::EmptyDesign => "the synthetic-control design is empty".to_owned(),
        SyntheticControlError::DimensionMismatch => "the treated and donor matrices do not have compatible dimensions".to_owned(),
        SyntheticControlError::NonFiniteInput => "the synthetic-control design contains a non-finite value".to_owned(),
        SyntheticControlError::InvalidFoldCount => "cross-fitting needs at least two folds".to_owned(),
        SyntheticControlError::InsufficientFoldData => "the pre- and post-intervention periods are too short for the requested cross-fitting folds".to_owned(),
        SyntheticControlError::InvalidPrePeriodBoundary => "prediction inference needs at least two pre-intervention rows and one post-intervention row".to_owned(),
        SyntheticControlError::InsufficientDonors => "donor-placebo inference needs at least two donor series".to_owned(),
        SyntheticControlError::InvalidAlpha => "prediction-band alpha must be between zero and one".to_owned(),
    }
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
        PanelError::TooFewControlsForPlacebo => "placebo variance needs more control units than treated units".to_owned(),
        PanelError::InsufficientPlaceboReplications => "placebo variance needs at least two replications".to_owned(),
        PanelError::InvalidPlaceboPermutation { replication } => format!("placebo replication {} is not a permutation of the control units", replication + 1),
        PanelError::InvalidTreatedFraction => "the pre-treatment period is too short for an in-time placebo".to_owned(),
    }
}

fn panel_placebo_evidence(
    result: Result<
        hirmos_causal_core::panel::PanelPlaceboInference,
        hirmos_causal_core::panel::PanelError,
    >,
    replications: usize,
    seed: u64,
) -> PanelPlaceboEvidence {
    match result {
        Ok(inference) => PanelPlaceboEvidence::Available {
            replications,
            seed,
            standard_error: inference.standard_error,
            estimates: inference.estimates,
        },
        Err(error) => PanelPlaceboEvidence::Unavailable {
            reason: panel_problem(error),
        },
    }
}

fn panel_in_time_evidence(
    result: Result<hirmos_causal_core::panel::PanelEstimate, hirmos_causal_core::panel::PanelError>,
) -> PanelInTimeEvidence {
    match result {
        Ok(estimate) => PanelInTimeEvidence::Available {
            estimate: estimate.estimate,
            effect_curve: estimate.effect_curve,
        },
        Err(error) => PanelInTimeEvidence::Unavailable {
            reason: panel_problem(error),
        },
    }
}

pub(crate) fn panel_intervention(
    values: &[f64],
    rows: usize,
    units: &[String],
    times: &[i64],
    placebo_replications: usize,
    seed: u64,
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
    progress("validated-panel", 1, 9);
    let did = hirmos_causal_core::panel::did_estimate(&panel.y, panel.n0, panel.t0)
        .map_err(panel_problem)?;
    progress("difference-in-differences", 2, 9);
    let synthetic_control =
        hirmos_causal_core::panel::synthdid_sc_estimate(&panel.y, panel.n0, panel.t0)
            .map_err(panel_problem)?;
    progress("synthetic-control", 3, 9);
    let synthetic_did =
        hirmos_causal_core::panel::synthetic_did_estimate(&panel.y, panel.n0, panel.t0)
            .map_err(panel_problem)?;
    progress("synthetic-did", 4, 9);
    let mut rng = NpRng::seeded(seed);
    let permutations = (0..placebo_replications)
        .map(|_| rng.permutation(panel.n0))
        .collect::<Vec<_>>();
    progress("placebo-permutations", 5, 9);
    let synthetic_control_placebo = panel_placebo_evidence(
        hirmos_causal_core::panel::panel_placebo_standard_error(
            &panel.y,
            panel.n0,
            panel.t0,
            hirmos_causal_core::panel::PanelEstimatorKind::SyntheticControl,
            &permutations,
        ),
        placebo_replications,
        seed,
    );
    progress("synthetic-control-placebos", 6, 9);
    let synthetic_did_placebo = panel_placebo_evidence(
        hirmos_causal_core::panel::panel_placebo_standard_error(
            &panel.y,
            panel.n0,
            panel.t0,
            hirmos_causal_core::panel::PanelEstimatorKind::SyntheticDifferenceInDifferences,
            &permutations,
        ),
        placebo_replications,
        seed,
    );
    progress("synthetic-did-placebos", 7, 9);
    let synthetic_control_in_time =
        panel_in_time_evidence(hirmos_causal_core::panel::panel_in_time_placebo(
            &panel.y,
            panel.n0,
            panel.t0,
            hirmos_causal_core::panel::PanelEstimatorKind::SyntheticControl,
            None,
        ));
    progress("synthetic-control-in-time", 8, 9);
    let synthetic_did_in_time =
        panel_in_time_evidence(hirmos_causal_core::panel::panel_in_time_placebo(
            &panel.y,
            panel.n0,
            panel.t0,
            hirmos_causal_core::panel::PanelEstimatorKind::SyntheticDifferenceInDifferences,
            None,
        ));
    progress("synthetic-did-in-time", 9, 9);
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
        synthetic_control_placebo,
        synthetic_did_placebo,
        synthetic_control_in_time,
        synthetic_did_in_time,
    })
}

pub(crate) fn panel_intervention_selected(
    primary: PanelPrimary,
    values: &[f64], rows: usize, units: &[String], times: &[i64],
    placebo_replications: usize, seed: u64,
    progress: impl Fn(&'static str, usize, usize),
) -> Result<AnalysisResult, String> {
    match primary {
        PanelPrimary::SyntheticDid => panel_intervention(values, rows, units, times, placebo_replications, seed, progress),
        PanelPrimary::Did => {
            validate_dense_matrix("panel DiD", values, rows, 2)?;
            if units.len() != rows || times.len() != rows { return Err("Panel DiD needs one unit and time key per row".to_owned()); }
            let observations = (0..rows).map(|row| hirmos_causal_core::panel::PanelObservation {
                unit: units[row].clone(), time: times[row], outcome: values[row], treatment: values[rows + row],
            }).collect::<Vec<_>>();
            let panel = hirmos_causal_core::panel::panel_matrices(&observations).map_err(panel_problem)?;
            progress("validated-panel", 1, 2);
            let did = hirmos_causal_core::panel::did_estimate(&panel.y, panel.n0, panel.t0).map_err(panel_problem)?;
            progress("difference-in-differences", 2, 2);
            Ok(AnalysisResult::PanelDid { observations: rows, treated_units: panel.y.nrows()-panel.n0, control_units: panel.n0,
                n_pre: panel.t0, n_post: panel.y.ncols()-panel.t0, units: panel.units, times: panel.times, did: panel_method_evidence(did) })
        }
    }
}

pub(crate) fn adjusted_panel(values: &[f64], rows: usize, columns: usize, units: &[String], times: &[i64]) -> Result<(hirmos_causal_core::panel::PanelMatrices, DMatrix<f64>), String> {
    use hirmos_causal_core::panel;
    validate_dense_matrix("adjusted DiD", values, rows, columns)?;
    if columns < 2 || units.len() != rows || times.len() != rows { return Err("DiD needs aligned outcomes, treatment and unit/time keys.".into()); }
    let observations = (0..rows).map(|i| panel::PanelObservation { unit: units[i].clone(), time: times[i], outcome: values[i], treatment: values[rows+i] }).collect::<Vec<_>>();
    let panel = panel::panel_matrices(&observations).map_err(panel_problem)?;
    if panel.times.len() != 2 || panel.t0 != 1 { return Err("This DiD specification requires exactly one period before and one after adoption.".into()); }
    // Lookup by keys: source query order is not the canonical control/treated order.
    let index: std::collections::BTreeMap<_,_> = (0..rows).map(|i| ((units[i].as_str(), times[i]), i)).collect();
    let mut ordered = Vec::with_capacity(rows);
    for unit in &panel.units { for time in &panel.times {
        ordered.push(*index.get(&(unit.as_str(), *time)).ok_or("A panel unit/time key is missing.")?);
    }}
    let covariates = DMatrix::from_fn(rows, columns-2, |i,j| values[(j+2)*rows+ordered[i]]);
    Ok((panel, covariates))
}

pub(crate) fn panel_adjusted(values: &[f64], rows: usize, columns: usize, units: &[String], times: &[i64], specification: AdjustedDidSpecification) -> Result<AnalysisResult, String> {
    use hirmos_causal_core::{did, did_regression, lbfgsb::LbfgsbTermination};
    let (panel, covariates) = adjusted_panel(values, rows, columns, units, times)?;
    let summary = did_regression::summarize(&panel).map_err(|_| "Observed panel group means could not be computed.")?;
    let group_means = [[summary.control.before,summary.control.after],[summary.treated.before,summary.treated.after]];
    let (estimate, standard_error, interval, inference) = match &specification {
        AdjustedDidSpecification::Regression => {
            let fit = did_regression::fit(&panel, &covariates).map_err(|error| match error {
                did_regression::Error::RankDeficient => "The DiD design is rank deficient. Remove constant or redundant covariates.",
                did_regression::Error::NoResidualDegreesOfFreedom => "There are too few observations for this DiD specification.",
                did_regression::Error::NonFinite => "DiD requires finite outcomes and covariates.",
                did_regression::Error::Decomposition => "The DiD regression decomposition failed.",
                did_regression::Error::InvalidLayout | did_regression::Error::CovariateShape => "DiD needs two matched periods and aligned covariates.",
            })?;
            (fit.coefficients[3],fit.standard_errors[3],fit.intervals[3],AdjustedDidInference::IndependentErrors { degrees_of_freedom: fit.residual_degrees_of_freedom, coefficients: fit.coefficients, standard_errors: fit.standard_errors, intervals: fit.intervals })
        }
        AdjustedDidSpecification::DoublyRobust { folds, seed, trimming, normalization } => {
            let baseline = DMatrix::from_fn(panel.units.len(),columns-2,|i,j| covariates[(2*i,j)]);
            let sample = did::PairedSample::from_panel(&panel,&baseline).map_err(|_| "DR DiD needs finite paired outcomes and at least one baseline covariate.")?;
            let normalization = match normalization { DidNormalization::InSample => did::Normalization::InSample, DidNormalization::Population => did::Normalization::Population };
            let plan = did::prepare(&sample,*folds,*seed,*trimming,normalization).map_err(|violations| violations.into_iter().map(|v| match v {
                did::Violation::TooFewFolds { .. } => "Use at least two folds.".to_owned(),
                did::Violation::TooManyFolds { smaller_group, .. } => format!("The fold count exceeds the smaller group ({smaller_group} units)."),
                did::Violation::InvalidTrimming { .. } => "Trimming must be greater than zero and less than one half.".to_owned(),
            }).collect::<Vec<_>>().join(" "))?;
            let fit = did::fit(&plan).map_err(|error| match error {
                did::Error::PropensityNotConverged { .. } => "The propensity fit did not converge after training-fold scaling and strict optimization. No DiD estimate was recorded. Review covariate redundancy and treatment overlap.",
                did::Error::Regression => "A cross-fitted outcome or propensity regression failed. Check covariate rank and fold sizes.",
                did::Error::PropensityBoundary => "Estimated propensity reached a probability boundary. Check treatment overlap.",
                did::Error::MissingGroup | did::Error::NonBinary => "DR DiD needs both treated and comparison units with binary assignment.",
                did::Error::NonFinite => "DR DiD produced non-finite values. Check covariate scale and overlap.",
                did::Error::Shape | did::Error::RequiresTwoPeriods => "DR DiD requires aligned baseline covariates and two paired periods.",
            })?;
            let optimizer_status = fit.propensity_termination.into_iter().map(|status| match status {
                LbfgsbTermination::ProjectedGradient => PropensityStatus::ProjectedGradient,
                LbfgsbTermination::FunctionTolerance => PropensityStatus::FunctionTolerance,
                LbfgsbTermination::IterationLimit => PropensityStatus::IterationLimit,
                LbfgsbTermination::LineSearchFailed => PropensityStatus::LineSearchFailed,
            }).collect();
            (fit.estimate.coef,fit.estimate.se,[fit.estimate.ci_low,fit.estimate.ci_high],AdjustedDidInference::CrossFitted { propensity: fit.propensity, optimizer_status, propensity_fit: crate::protocol::DidPropensityFit::StandardizedLogisticV1 })
        }
    };
    Ok(AnalysisResult::PanelAdjusted { observations: rows, control_units: panel.n0, treated_units: panel.units.len()-panel.n0, n_pre:1,n_post:1,covariates:columns-2,
        units:panel.units,times:panel.times,specification,estimate,standard_error,interval,group_means,inference })
}
