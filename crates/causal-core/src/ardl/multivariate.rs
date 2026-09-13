//! Contiguous-lag ARDL and UECM designs with predictor-specific orders and fixed regressors.

use super::{try_ols, Ols, Trend, Uecm};
use nalgebra::{DMatrix, DVector};

pub mod search;
pub mod horizontal;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyOutcome,
    RowMismatch,
    NonFinite,
    OrderMismatch,
    InvalidEcmOrder,
    HoldBackTooShort,
    InsufficientRows,
    InvalidHorizon,
    InvalidConfidence,
    NonFiniteResult,
    Regression(crate::ols::OlsError),
    SearchLimit,
    Cancelled,
}

/// Finite, aligned input columns. Fixed regressors are included without lags.
#[derive(Clone, Copy)]
pub struct Input<'a> {
    outcome: &'a [f64],
    predictors: &'a [Vec<f64>],
    fixed: &'a [Vec<f64>],
}

impl<'a> Input<'a> {
    pub fn new(
        outcome: &'a [f64],
        predictors: &'a [Vec<f64>],
        fixed: &'a [Vec<f64>],
    ) -> Result<Self, Error> {
        if outcome.is_empty() {
            return Err(Error::EmptyOutcome);
        }
        if predictors
            .iter()
            .chain(fixed)
            .any(|c| c.len() != outcome.len())
        {
            return Err(Error::RowMismatch);
        }
        if outcome
            .iter()
            .chain(predictors.iter().chain(fixed).flatten())
            .any(|v| !v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        Ok(Self {
            outcome,
            predictors,
            fixed,
        })
    }
}

/// Each predictor has either an excluded state or a maximum included lag (starting at zero).
#[derive(Clone)]
pub struct Specification {
    outcome_lag: usize,
    predictor_lags: Vec<Option<usize>>,
    trend: Trend,
    hold_back: Option<usize>,
}

impl Specification {
    pub fn outcome_lag(&self) -> usize {
        self.outcome_lag
    }
    pub fn predictor_lags(&self) -> &[Option<usize>] {
        &self.predictor_lags
    }
    pub fn trend(&self) -> Trend {
        self.trend
    }
    pub fn hold_back(&self) -> Option<usize> {
        self.hold_back
    }
    pub fn new(
        outcome_lag: usize,
        predictor_lags: Vec<Option<usize>>,
        trend: Trend,
        hold_back: Option<usize>,
    ) -> Result<Self, Error> {
        let maximum = predictor_lags
            .iter()
            .flatten()
            .copied()
            .fold(outcome_lag, usize::max);
        if hold_back.is_some_and(|rows| rows < maximum) {
            return Err(Error::HoldBackTooShort);
        }
        Ok(Self {
            outcome_lag,
            predictor_lags,
            trend,
            hold_back,
        })
    }

    fn sample_start(&self, input: &Input<'_>, ecm: bool) -> Result<usize, Error> {
        if self.predictor_lags.len() != input.predictors.len() {
            return Err(Error::OrderMismatch);
        }
        let maximum = self
            .predictor_lags
            .iter()
            .flatten()
            .copied()
            .fold(self.outcome_lag, usize::max)
            .max(usize::from(ecm));
        let start = self.hold_back.unwrap_or(maximum);
        if start < maximum {
            return Err(Error::HoldBackTooShort);
        }
        if start >= input.outcome.len() {
            return Err(Error::InsufficientRows);
        }
        Ok(start)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Constant,
    Trend,
    Outcome { lag: usize },
    Predictor { column: usize, lag: usize },
    OutcomeChange { lag: usize },
    PredictorChange { column: usize, lag: usize },
    Fixed { column: usize },
}

fn deterministic(trend: Trend) -> Vec<Term> {
    match trend {
        Trend::None => vec![],
        Trend::C => vec![Term::Constant],
        Trend::Ct => vec![Term::Constant, Term::Trend],
    }
}

fn design(input: &Input<'_>, terms: &[Term], start: usize) -> Result<DMatrix<f64>, Error> {
    let rows = input.outcome.len() - start;
    if rows <= terms.len() {
        return Err(Error::InsufficientRows);
    }
    Ok(DMatrix::from_fn(rows, terms.len(), |row, col| {
        let t = start + row;
        match terms[col] {
            Term::Constant => 1.0,
            Term::Trend => (t + 1) as f64,
            Term::Outcome { lag } => input.outcome[t - lag],
            Term::Predictor { column, lag } => input.predictors[column][t - lag],
            Term::OutcomeChange { lag } => input.outcome[t - lag] - input.outcome[t - lag - 1],
            Term::PredictorChange { column, lag } => {
                input.predictors[column][t - lag] - input.predictors[column][t - lag - 1]
            }
            Term::Fixed { column } => input.fixed[column][t],
        }
    }))
}

pub struct LevelsFit<'a> {
    fit: Ols,
    terms: Vec<Term>,
    pub start_row: usize,
    pub fitted: Vec<f64>,
    input: Input<'a>,
    reported_nobs: usize,
}

/// Fit the ARDL levels equation using the shared SVD OLS and nonrobust covariance.
pub fn fit_levels<'a>(
    input: &Input<'a>,
    specification: &Specification,
) -> Result<LevelsFit<'a>, Error> {
    let start = specification.sample_start(input, false)?;
    let mut terms = deterministic(specification.trend);
    terms.extend((1..=specification.outcome_lag).map(|lag| Term::Outcome { lag }));
    for (column, order) in specification.predictor_lags.iter().enumerate() {
        if let Some(order) = order {
            terms.extend((0..=*order).map(|lag| Term::Predictor { column, lag }));
        }
    }
    terms.extend((0..input.fixed.len()).map(|column| Term::Fixed { column }));
    let x = design(input, &terms, start)?;
    let y = DVector::from_column_slice(&input.outcome[start..]);
    let fit = try_ols(&x, &y).map_err(Error::Regression)?;
    let fitted = (&x * &fit.params).iter().copied().collect();
    Ok(LevelsFit {
        fit,
        terms,
        start_row: start,
        fitted,
        input: *input,
        reported_nobs: input.outcome.len()
            - specification.hold_back.unwrap_or(specification.outcome_lag),
    })
}

pub struct ForecastPoint {
    pub mean: f64,
    pub variance: f64,
    pub lower: f64,
    pub upper: f64,
}

impl LevelsFit<'_> {
    pub fn regression(&self) -> &Ols {
        &self.fit
    }

    pub fn terms(&self) -> &[Term] {
        &self.terms
    }

    /// Out-of-sample ARDL predictions conditional on supplied future regressors.
    /// The innovation variance follows statsmodels ARDLResults.get_prediction;
    /// it does not include uncertainty about future regressors or coefficients.
    pub fn forecast(
        &self,
        horizon: usize,
        predictors: &[Vec<f64>],
        fixed: &[Vec<f64>],
        confidence: f64,
    ) -> Result<Vec<ForecastPoint>, Error> {
        if horizon == 0 {
            return Err(Error::InvalidHorizon);
        }
        if !(0.0 < confidence && confidence < 1.0) {
            return Err(Error::InvalidConfidence);
        }
        if predictors.len() != self.input.predictors.len() || fixed.len() != self.input.fixed.len()
        {
            return Err(Error::OrderMismatch);
        }
        if predictors
            .iter()
            .chain(fixed)
            .any(|column| column.len() != horizon)
        {
            return Err(Error::RowMismatch);
        }
        if predictors
            .iter()
            .chain(fixed)
            .flatten()
            .any(|v| !v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        let n = self.input.outcome.len();
        let mut outcome = self.input.outcome.to_vec();
        let mut impulse = vec![0.0; horizon];
        impulse[0] = 1.0;
        // AutoRegResults.sigma2 divides by the model's reported nobs, not design rows.
        let sigma2 = self.fit.resid.iter().map(|v| v * v).sum::<f64>() / self.reported_nobs as f64;
        let critical = spec_math::cephes64::ndtri(0.5 + confidence / 2.0);
        let mut variance = 0.0;
        let mut result = Vec::with_capacity(horizon);
        for step in 0..horizon {
            let t = n + step;
            let mut mean = 0.0;
            for (index, term) in self.terms.iter().enumerate() {
                let coefficient = self.fit.params[index];
                let value = match *term {
                    Term::Constant => 1.0,
                    Term::Trend => (t + 1) as f64,
                    Term::Outcome { lag } => {
                        if lag <= step {
                            impulse[step] += coefficient * impulse[step - lag];
                        }
                        outcome[t - lag]
                    }
                    Term::Predictor { column, lag } => {
                        let row = t - lag;
                        if row < n {
                            self.input.predictors[column][row]
                        } else {
                            predictors[column][row - n]
                        }
                    }
                    Term::Fixed { column } => fixed[column][step],
                    Term::OutcomeChange { .. } | Term::PredictorChange { .. } => {
                        unreachable!("levels fit contains no differences")
                    }
                };
                mean += coefficient * value;
            }
            variance += sigma2 * impulse[step].powi(2);
            let margin = critical * variance.sqrt();
            if ![mean, variance, mean - margin, mean + margin]
                .iter()
                .all(|v| v.is_finite())
            {
                return Err(Error::NonFiniteResult);
            }
            outcome.push(mean);
            result.push(ForecastPoint {
                mean,
                variance,
                lower: mean - margin,
                upper: mean + margin,
            });
        }
        Ok(result)
    }
}

/// Fit the source UECM without dropping fixed regressors or changing its sample.
pub fn fit_uecm(input: &Input<'_>, specification: &Specification) -> Result<Uecm, Error> {
    fit_ecm(input, specification, EcmConvention::Statsmodels)
}

/// R ARDL's conditional UECM: zero-order predictors enter at the current time,
/// without an additional differenced term. The reported sample is the fitted sample.
pub fn fit_r_uecm(input: &Input<'_>, specification: &Specification) -> Result<Uecm, Error> {
    if specification.outcome_lag == 0 {
        return Err(Error::InvalidEcmOrder);
    }
    fit_ecm(input, specification, EcmConvention::RArdl)
}

#[derive(Clone, Copy)]
enum EcmConvention {
    Statsmodels,
    RArdl,
}

fn fit_ecm(
    input: &Input<'_>,
    specification: &Specification,
    convention: EcmConvention,
) -> Result<Uecm, Error> {
    let orders: Vec<_> = specification
        .predictor_lags
        .iter()
        .enumerate()
        .filter_map(|(col, order)| order.map(|q| (col, q)))
        .collect();
    if orders.is_empty()
        || matches!(convention, EcmConvention::Statsmodels) && orders.iter().any(|(_, q)| *q == 0)
    {
        return Err(Error::InvalidEcmOrder);
    }
    let start = specification.sample_start(input, true)?;
    let mut terms = deterministic(specification.trend);
    terms.push(Term::Outcome { lag: 1 });
    terms.extend(orders.iter().map(|(column, order)| Term::Predictor {
        column: *column,
        lag: usize::from(*order > 0),
    }));
    terms.extend((1..specification.outcome_lag).map(|lag| Term::OutcomeChange { lag }));
    for (column, order) in &orders {
        terms.extend((0..*order).map(|lag| Term::PredictorChange {
            column: *column,
            lag,
        }));
    }
    terms.extend((0..input.fixed.len()).map(|column| Term::Fixed { column }));
    let x = design(input, &terms, start)?;
    let y = DVector::from_iterator(
        input.outcome.len() - start,
        (start..input.outcome.len()).map(|t| input.outcome[t] - input.outcome[t - 1]),
    );
    let fit = try_ols(&x, &y).map_err(Error::Regression)?;
    let fitted = &x * &fit.params;
    let reported_resid = (start..input.outcome.len())
        .enumerate()
        .map(|(row, t)| input.outcome[t] - fitted[row])
        .collect();
    let mut ardl_order = vec![specification.outcome_lag];
    ardl_order.extend(orders.iter().map(|(_, q)| *q));
    Ok(Uecm {
        design: x,
        terms,
        n_det: specification.trend.n_det(),
        n_levels: orders.len() + 1,
        ardl_order,
        fit,
        reported_resid,
        // Statsmodels initializes nobs before expanding the default hold-back for exog lags.
        reported_nobs: input.outcome.len()
            - match convention {
                EcmConvention::Statsmodels => {
                    specification.hold_back.unwrap_or(specification.outcome_lag)
                }
                EcmConvention::RArdl => start,
            },
    })
}
