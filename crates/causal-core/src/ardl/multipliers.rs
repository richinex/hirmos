//! R ARDL multipliers and Gaussian delay intervals, with recursive analytic gradients.

use super::multivariate::{Error, LevelsFit, Term};
use nalgebra::DVector;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiplierTerm {
    Constant,
    Trend,
    Predictor(usize),
}

pub struct Summary {
    pub estimate: f64,
    pub standard_error: f64,
    pub t_value: f64,
    pub p_value: f64,
}

pub struct Delay {
    pub estimate: f64,
    pub standard_error: f64,
    pub lower: f64,
    pub upper: f64,
    pub cumulative: f64,
}

pub struct Multipliers {
    pub term: MultiplierTerm,
    pub short_run: Summary,
    pub long_run: Summary,
    pub delay: Vec<Delay>,
}

fn standard_error(model: &LevelsFit<'_>, gradient: &DVector<f64>) -> Result<f64, Error> {
    let variance = gradient.dot(&(&model.regression().cov_params * gradient));
    if !variance.is_finite() || variance < 0.0 {
        return Err(Error::NonFiniteResult);
    }
    Ok(variance.sqrt())
}

fn summary(
    model: &LevelsFit<'_>,
    estimate: f64,
    gradient: &DVector<f64>,
) -> Result<Summary, Error> {
    let standard_error = standard_error(model, gradient)?;
    let t_value = estimate / standard_error;
    let df = model.regression().df_resid();
    let p_value = spec_math::cephes64::incbet(df / 2.0, 0.5, df / (df + t_value * t_value));
    if ![estimate, standard_error, t_value, p_value]
        .iter()
        .all(|v| v.is_finite())
    {
        return Err(Error::NonFiniteResult);
    }
    Ok(Summary {
        estimate,
        standard_error,
        t_value,
        p_value,
    })
}

pub fn multipliers(
    model: &LevelsFit<'_>,
    horizon: usize,
    confidence: f64,
) -> Result<Vec<Multipliers>, Error> {
    if horizon > 200 {
        return Err(Error::InvalidHorizon);
    }
    if !(0.0 < confidence && confidence < 1.0) {
        return Err(Error::InvalidConfidence);
    }
    let params = &model.regression().params;
    let mut ar = Vec::new();
    let mut groups: BTreeMap<MultiplierTerm, Vec<(usize, usize)>> = BTreeMap::new();
    for (index, term) in model.terms().iter().enumerate() {
        match *term {
            Term::Outcome { lag } => ar.push((lag, index)),
            Term::Constant => groups
                .entry(MultiplierTerm::Constant)
                .or_default()
                .push((0, index)),
            Term::Trend => groups
                .entry(MultiplierTerm::Trend)
                .or_default()
                .push((0, index)),
            Term::Predictor { column, lag } => groups
                .entry(MultiplierTerm::Predictor(column))
                .or_default()
                .push((lag, index)),
            Term::Fixed { .. } => (),
            Term::OutcomeChange { .. } | Term::PredictorChange { .. } => {
                unreachable!("levels terms are constructed by fit_levels")
            }
        }
    }
    let denominator = 1.0 - ar.iter().map(|(_, i)| params[*i]).sum::<f64>();
    let critical = spec_math::cephes64::ndtri(0.5 + confidence / 2.0);
    let mut result = Vec::with_capacity(groups.len());
    for (term, direct) in groups {
        let mut values = Vec::with_capacity(horizon + 1);
        let mut gradients: Vec<DVector<f64>> = Vec::with_capacity(horizon + 1);
        let mut delay = Vec::with_capacity(horizon + 1);
        let mut cumulative = 0.0;
        for step in 0..=horizon {
            let mut value = 0.0;
            let mut gradient = DVector::zeros(params.len());
            for (lag, index) in &direct {
                if *lag == step {
                    value += params[*index];
                    gradient[*index] += 1.0;
                }
            }
            for (lag, index) in &ar {
                if *lag > step {
                    continue;
                }
                value += params[*index] * values[step - lag];
                gradient += &gradients[step - lag] * params[*index];
                gradient[*index] += values[step - lag];
            }
            let se = standard_error(model, &gradient)?;
            cumulative += value;
            let lower = value - critical * se;
            let upper = value + critical * se;
            if ![value, cumulative, lower, upper]
                .iter()
                .all(|v| v.is_finite())
            {
                return Err(Error::NonFiniteResult);
            }
            delay.push(Delay {
                estimate: value,
                standard_error: se,
                lower,
                upper,
                cumulative,
            });
            values.push(value);
            gradients.push(gradient);
        }
        let numerator = direct.iter().map(|(_, i)| params[*i]).sum::<f64>();
        let mut long_gradient = DVector::zeros(params.len());
        for (_, i) in &direct {
            long_gradient[*i] = 1.0 / denominator;
        }
        for (_, i) in &ar {
            long_gradient[*i] = numerator / (denominator * denominator);
        }
        result.push(Multipliers {
            term,
            short_run: summary(model, values[0], &gradients[0])?,
            long_run: summary(model, numerator / denominator, &long_gradient)?,
            delay,
        });
    }
    Ok(result)
}
