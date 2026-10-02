//! Validated multivariable ARDL commands and source-derived post-estimation results.

use crate::{matrix::validate_dense_matrix, protocol::AnalysisResult};
use hirmos_causal_core::ardl::{
    bounds_test,
    multipliers::{multipliers, MultiplierTerm},
    multivariate::{
        fit_levels, fit_uecm,
        search::{Criterion, Search},
        Error, Input, Specification, Term, OmittedChange, RestrictedEcmSpecification, fit_restricted_levels,
    },
    Trend,
};
use serde::{Deserialize, Serialize};
mod r_analysis;

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Terms {
    RestrictedConstant,
    Constant,
    RestrictedTrend,
    Trend,
}
impl Terms {
    fn specification(self) -> (Trend, usize) {
        match self {
            Self::RestrictedConstant => (Trend::C, 2),
            Self::Constant => (Trend::C, 3),
            Self::RestrictedTrend => (Trend::Ct, 4),
            Self::Trend => (Trend::Ct, 5),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum Orders {
    Fixed {
        outcome_lag: usize,
        predictor_lags: Vec<Option<usize>>,
    },
    Search {
        maximum_lag: usize,
        maximum_orders: Vec<usize>,
    },
    RFixed {
        outcome_lag: usize,
        predictor_lags: Vec<usize>,
    },
    RRestricted {
        outcome_lag: usize,
        predictor_lags: Vec<usize>,
        omitted: Vec<Change>,
    },
    RHorizontal {
        maximum: Vec<usize>,
        fixed: Vec<Option<usize>>,
        starting: Vec<usize>,
    },
    RGrid {
        minimum_lag: usize,
        maximum_lag: usize,
        maximum_orders: Vec<usize>,
        fixed_orders: Vec<Option<usize>>,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum Future {
    None,
    Scenario {
        predictors: Vec<Vec<f64>>,
        fixed: Vec<Vec<f64>>,
        confidence: f64,
    },
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag="kind",rename_all="camelCase",deny_unknown_fields)]
pub(crate) enum Change { Outcome { lag:usize }, Predictor { column:usize, lag:usize } }
impl Change {
    fn numerical(&self)->OmittedChange { match *self { Self::Outcome{lag}=>OmittedChange::Outcome{lag},Self::Predictor{column,lag}=>OmittedChange::Predictor{column,lag} } }
}
fn ecm(input:&Input<'_>,spec:&Specification,omitted:&[OmittedChange])->Result<hirmos_causal_core::ardl::Uecm,Error>{
    if omitted.is_empty(){hirmos_causal_core::ardl::multivariate::fit_r_uecm(input,spec)}else{
        hirmos_causal_core::ardl::multivariate::fit_restricted_r_uecm(input,&RestrictedEcmSpecification::new(spec.clone(),omitted.to_vec())?)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    outcome: usize,
    predictors: Vec<usize>,
    fixed: Vec<usize>,
    terms: Terms,
    orders: Orders,
    hold_back: Option<usize>,
    multiplier_horizon: usize,
    future: Future,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum Coefficient {
    Constant,
    Trend,
    Outcome { lag: usize },
    Predictor { column: usize, lag: usize },
    Fixed { column: usize },
    OutcomeChange { lag: usize },
    PredictorChange { column: usize, lag: usize },
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum LongRun {
    Unavailable {
        reason: LongRunReason,
    },
    Recorded {
        departures: Vec<f64>,
        normalized: Vec<f64>,
        intervals: Vec<(f64, f64)>,
        bounds_statistic: f64,
        bounds_critical: Vec<(f64, f64)>,
        p_lower: f64,
        p_upper: f64,
    },
    Uncalibrated {
        departures: Vec<f64>,
        normalized: Vec<f64>,
        intervals: Vec<(f64, f64)>,
        bounds_statistic: f64,
    },
    RCalibrated {
        departures: Vec<f64>,
        normalized: Vec<f64>,
        intervals: Vec<(f64,f64)>,
        bounds_statistic: f64,
        f_bounds: Vec<CriticalBounds>,
        f_p_value: f64,
        t_bounds: TBounds,
    },
}
#[derive(Serialize)]
pub(crate) struct CriticalBounds {alpha:f64,i0:f64,i1:f64}
#[derive(Serialize)]
#[serde(tag="kind",rename_all="camelCase",rename_all_fields="camelCase")]
pub(crate) enum TBounds {
    NotApplicable,
    Recorded {statistic:f64,critical:Vec<CriticalBounds>,p_value:f64},
}
fn calibrated_rows(statistic:hirmos_causal_core::ardl::bounds_calibration::Statistic,k:usize,case:usize)->Result<(Vec<CriticalBounds>,f64),String>{
    use hirmos_causal_core::ardl::bounds_calibration::{calibrate,LEVELS};
    let rows=LEVELS.iter().map(|alpha|calibrate(statistic,k,case,*alpha)).collect::<Result<Vec<_>,_>>()
        .map_err(|e|format!("The selected bounds calibration is unavailable: {e:?}."))?;
    let p_value=rows[0].p_value;
    Ok((rows.into_iter().map(|r|CriticalBounds{alpha:r.alpha,i0:r.i0,i1:r.i1}).collect(),p_value))
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum LongRunReason {
    NoPredictors,
    ZeroOrder,
    TooManyPredictors,
    UndefinedNormalization,
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum Forecast {
    NotRequested,
    Recorded {
        confidence: f64,
        mean: Vec<f64>,
        variance: Vec<f64>,
        interval: Vec<(f64, f64)>,
    },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Multiplier {
    term: MultiplierVariable,
    short_run: f64,
    long_run: f64,
    long_run_se: f64,
    delay: Vec<f64>,
    standard_error: Vec<f64>,
    interval: Vec<(f64, f64)>,
    cumulative: Vec<f64>,
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum MultiplierVariable {
    Constant,
    Trend,
    Predictor { column: usize },
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum MultiplierResult {
    Recorded {
        confidence: f64,
        curves: Vec<Multiplier>,
    },
    Unavailable {
        reason: MultiplierReason,
    },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum MultiplierReason {
    NumericalFailure,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Evidence {
    observations: usize,
    start_row: usize,
    fitted_rows: usize,
    outcome_lag: usize,
    predictor_lags: Vec<Option<usize>>,
    coefficients: Vec<Coefficient>,
    params: Vec<f64>,
    covariance: Vec<Vec<f64>>,
    observed: Vec<f64>,
    fitted: Vec<f64>,
    long_run: LongRun,
    multipliers: MultiplierResult,
    forecast: Forecast,
    r_analysis: r_analysis::Evidence,
}

fn problem(error: Error) -> String {
    match error {
    Error::InsufficientRows=>"The selected lags leave too few observations to fit this model.",
    Error::HoldBackTooShort=>"The sample must start after every included lag.",
    Error::InvalidEcmRestriction=>"Choose distinct short-run change terms included in the specified model.",
    Error::SearchLimit=>"This lag search exceeds 10,000 candidate models. Reduce the maximum lags or specify the orders.",
    Error::RowMismatch|Error::OrderMismatch=>"The supplied columns or future scenario do not match the model.",
    Error::InvalidConfidence=>"Choose a confidence level strictly between zero and one.",
    Error::InvalidHorizon=>"Choose a horizon between 1 and 200 periods.",
    Error::EmptyOutcome=>"The prepared outcome has no observations.",
    Error::NonFinite=>"The model and future scenario need finite values.",
    Error::InvalidEcmOrder=>"A long-run test requires lagged values for each included predictor.",
    Error::Cancelled=>"The model search was cancelled.",
    Error::NonFiniteResult|Error::Regression(_)=>"The selected model could not produce finite estimates. Check the columns and lag specification.",
}.to_owned()
}

fn long_run(
    input: &Input<'_>,
    spec: &Specification,
    y: &[f64],
    x: &[Vec<f64>],
    case: usize,
    r: bool,
    omitted: &[OmittedChange],
) -> Result<LongRun, String> {
    let active: Vec<&[f64]> = x
        .iter()
        .zip(spec.predictor_lags())
        .filter(|(_, order)| order.is_some())
        .map(|(column, _)| column.as_slice())
        .collect();
    let reason = if active.is_empty() {
        Some(LongRunReason::NoPredictors)
    } else if !r && spec.predictor_lags().contains(&Some(0)) {
        Some(LongRunReason::ZeroOrder)
    } else if !r && active.len() > 9 {
        Some(LongRunReason::TooManyPredictors)
    } else {
        None
    };
    if let Some(reason) = reason {
        return Ok(LongRun::Unavailable { reason });
    }
    let fit = if r {
        ecm(input, spec, omitted)
    } else {
        fit_uecm(input, spec)
    }
    .map_err(problem)?;
    let vector = fit.cointegrating_vector(0.05);
    if !vector
        .params
        .iter()
        .chain(vector.conf_int.iter().flat_map(|(a, b)| [a, b]))
        .all(|v| v.is_finite())
    {
        return Ok(LongRun::Unavailable {
            reason: LongRunReason::UndefinedNormalization,
        });
    }
    let departures = fit.cointegrating_residuals_columns(y, &active, spec.trend());
    if r {
        let statistic = hirmos_causal_core::ardl::bounds_statistic(&fit, case);
        if !departures.iter().all(|v| v.is_finite()) || !statistic.is_finite() {
            return Ok(LongRun::Unavailable {
                reason: LongRunReason::UndefinedNormalization,
            });
        }
        if active.len()<=10 {
            use hirmos_causal_core::ardl::bounds_calibration::Statistic;
            let (f_bounds,f_p_value)=calibrated_rows(Statistic::F(statistic),active.len(),case)?;
            let t_bounds=if [1,3,5].contains(&case){
                let value=fit.fit.params[fit.n_det]/fit.fit.cov_params[(fit.n_det,fit.n_det)].sqrt();
                let (critical,p_value)=calibrated_rows(Statistic::T(value),active.len(),case)?;
                TBounds::Recorded{statistic:value,critical,p_value}
            }else{TBounds::NotApplicable};
            return Ok(LongRun::RCalibrated{departures,normalized:vector.params,intervals:vector.conf_int,
                bounds_statistic:statistic,f_bounds,f_p_value,t_bounds});
        }
        return Ok(LongRun::Uncalibrated {
            departures,
            normalized: vector.params,
            intervals: vector.conf_int,
            bounds_statistic: statistic,
        });
    }
    let bounds = bounds_test(&fit, case);
    if !departures
        .iter()
        .chain([&bounds.stat, &bounds.p_lower, &bounds.p_upper])
        .all(|v| v.is_finite())
    {
        return Ok(LongRun::Unavailable {
            reason: LongRunReason::UndefinedNormalization,
        });
    }
    Ok(LongRun::Recorded {
        departures,
        normalized: vector.params,
        intervals: vector.conf_int,
        bounds_statistic: bounds.stat,
        bounds_critical: bounds.crit_vals,
        p_lower: bounds.p_lower,
        p_upper: bounds.p_upper,
    })
}

pub(crate) fn fit(
    values: &[f64],
    rows: usize,
    columns: usize,
    request: Request,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("ARDL", values, rows, columns)?;
    let indices: Vec<usize> = std::iter::once(request.outcome)
        .chain(request.predictors.iter().copied())
        .chain(request.fixed.iter().copied())
        .collect();
    if request.predictors.is_empty()
        || indices.iter().any(|i| *i >= columns)
        || indices
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != indices.len()
    {
        return Err("Choose distinct outcome, predictor and fixed columns.".to_owned());
    }
    if request.multiplier_horizon > 200 {
        return Err("Choose at most 200 multiplier periods.".to_owned());
    }
    let column = |index: usize| values[index * rows..(index + 1) * rows].to_vec();
    let y = column(request.outcome);
    let x: Vec<_> = request.predictors.iter().map(|i| column(*i)).collect();
    let fixed: Vec<_> = request.fixed.iter().map(|i| column(*i)).collect();
    let input = Input::new(&y, &x, &fixed).map_err(problem)?;
    let (trend, case) = request.terms.specification();
    let valid_lags = match &request.orders {
        Orders::Fixed {
            outcome_lag,
            predictor_lags,
        } => *outcome_lag <= 24 && predictor_lags.iter().flatten().all(|q| *q <= 24),
        Orders::Search {
            maximum_lag,
            maximum_orders,
        } => *maximum_lag <= 24 && maximum_orders.iter().all(|q| *q <= 24),
        Orders::RFixed {
            outcome_lag,
            predictor_lags,
        } | Orders::RRestricted { outcome_lag, predictor_lags, .. } => *outcome_lag > 0 && *outcome_lag <= 24 && predictor_lags.iter().all(|q| *q <= 24),
        Orders::RHorizontal {
            maximum,
            fixed,
            starting,
        } => {
            maximum.len() == request.predictors.len() + 1
                && maximum
                    .iter()
                    .chain(starting)
                    .chain(fixed.iter().flatten())
                    .all(|q| *q <= 24)
        }
        Orders::RGrid {
            minimum_lag,
            maximum_lag,
            maximum_orders,
            fixed_orders,
        } => {
            *minimum_lag > 0
                && *maximum_lag <= 24
                && maximum_orders
                    .iter()
                    .chain(fixed_orders.iter().flatten())
                    .all(|q| *q <= 24)
        }
    };
    if !valid_lags {
        return Err("Choose lag orders between 0 and 24.".to_owned());
    }
    let is_r = matches!(
        request.orders,
        Orders::RFixed { .. } | Orders::RRestricted { .. } | Orders::RHorizontal { .. } | Orders::RGrid { .. }
    );
    let omitted=match &request.orders{Orders::RRestricted{omitted,..}=>omitted.iter().map(Change::numerical).collect::<Vec<_>>(),_=>vec![]};
    let (spec, ranking) = match request.orders {
        Orders::Fixed {
            outcome_lag,
            predictor_lags,
        } => (
            Specification::new(outcome_lag, predictor_lags, trend, request.hold_back)
                .map_err(problem)?,
            r_analysis::Ranking::NotRequested,
        ),
        Orders::Search {
            maximum_lag,
            maximum_orders,
        } => (
            Search::new(
                maximum_lag,
                maximum_orders,
                trend,
                request.hold_back,
                Criterion::Aic,
                10000,
            )
            .map_err(problem)?
            .run(&input, |_, _| true)
            .map_err(problem)?
            .specification,
            r_analysis::Ranking::NotRequested,
        ),
        orders => r_analysis::select(orders, &input, trend, request.hold_back)?,
    };
    let fitted = if omitted.is_empty(){fit_levels(&input, &spec)}else{fit_restricted_levels(&input,&RestrictedEcmSpecification::new(spec.clone(),omitted.clone()).map_err(problem)?)}.map_err(problem)?;
    let coefficient = |term: &Term| match *term {
        Term::Constant => Coefficient::Constant,
        Term::Trend => Coefficient::Trend,
        Term::Outcome { lag } => Coefficient::Outcome { lag },
        Term::Predictor { column, lag } => Coefficient::Predictor { column, lag },
        Term::Fixed { column } => Coefficient::Fixed { column },
        Term::OutcomeChange { lag } => Coefficient::OutcomeChange { lag },
        Term::PredictorChange { column, lag } => Coefficient::PredictorChange { column, lag },
    };
    let curves = match multipliers(&fitted, request.multiplier_horizon, 0.95) {
        Ok(curves) => MultiplierResult::Recorded {
            confidence: 0.95,
            curves: curves
                .into_iter()
                .map(|c| Multiplier {
                    term: match c.term {
                        MultiplierTerm::Constant => MultiplierVariable::Constant,
                        MultiplierTerm::Trend => MultiplierVariable::Trend,
                        MultiplierTerm::Predictor(column) => {
                            MultiplierVariable::Predictor { column }
                        }
                    },
                    short_run: c.short_run.estimate,
                    long_run: c.long_run.estimate,
                    long_run_se: c.long_run.standard_error,
                    delay: c.delay.iter().map(|d| d.estimate).collect(),
                    standard_error: c.delay.iter().map(|d| d.standard_error).collect(),
                    interval: c.delay.iter().map(|d| (d.lower, d.upper)).collect(),
                    cumulative: c.delay.iter().map(|d| d.cumulative).collect(),
                })
                .collect(),
        },
        Err(_) => MultiplierResult::Unavailable {
            reason: MultiplierReason::NumericalFailure,
        },
    };
    let forecast = match request.future {
        Future::None => Forecast::NotRequested,
        Future::Scenario {
            predictors,
            fixed,
            confidence,
        } => {
            let horizon = predictors.first().map_or(0, Vec::len);
            if horizon == 0 || horizon > 200 {
                return Err(
                    "Supply between 1 and 200 periods of future predictor values.".to_owned(),
                );
            }
            let predictions = fitted
                .forecast(horizon, &predictors, &fixed, confidence)
                .map_err(problem)?;
            Forecast::Recorded {
                confidence,
                mean: predictions.iter().map(|p| p.mean).collect(),
                variance: predictions.iter().map(|p| p.variance).collect(),
                interval: predictions.iter().map(|p| (p.lower, p.upper)).collect(),
            }
        }
    };
    let regression = fitted.regression();
    if !regression
        .params
        .iter()
        .chain(regression.cov_params.iter())
        .all(|v| v.is_finite())
    {
        return Err(
            "The selected model did not produce finite coefficients and covariance.".to_owned(),
        );
    }
    let evidence = Evidence {
        observations: rows,
        start_row: fitted.start_row,
        fitted_rows: regression.nobs,
        outcome_lag: spec.outcome_lag(),
        predictor_lags: spec.predictor_lags().to_vec(),
        coefficients: fitted.terms().iter().map(coefficient).collect(),
        params: regression.params.iter().copied().collect(),
        covariance: (0..regression.cov_params.nrows())
            .map(|r| regression.cov_params.row(r).iter().copied().collect())
            .collect(),
        long_run: long_run(&input, &spec, &y, &x, case, is_r, &omitted)?,
        observed: y.clone(),
        fitted: fitted.fitted,
        multipliers: curves,
        forecast,
        r_analysis: if is_r {
            r_analysis::fit(&input, &spec, case, ranking, coefficient, &omitted)?
        } else {
            r_analysis::Evidence::NotRequested
        },
    };
    Ok(AnalysisResult::ArdlModel { evidence })
}
