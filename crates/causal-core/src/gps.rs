use nalgebra::{DMatrix, DVector};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GpsError {
    EmptySample,
    LengthMismatch,
    NoColumns,
    NonFiniteValue,
    DegenerateScale,
    NonPositiveDensity { row: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeightScale {
    InverseDensity,
    Stabilized,
}

#[derive(Clone, Debug)]
pub struct Gps {
    pub density: Vec<f64>,
    pub fitted_treatment: Vec<f64>,
    pub residual_scale: f64,
    pub treatment_params: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct GpsEffect {
    pub effect: f64,
    pub intercept: f64,
    pub standard_error: f64,
    pub weights: Vec<f64>,
    pub weight_sum: f64,
}

fn normal_pdf(value: f64, location: f64, scale: f64) -> f64 {
    let standardized = (value - location) / scale;
    ((-standardized * standardized / 2.0).exp() / (2.0 * core::f64::consts::PI).sqrt()) / scale
}

fn numpy_std(values: &[f64]) -> f64 {
    let mean = crate::numpy_reduce::numpy_mean(values);
    let squared: Vec<f64> = values
        .iter()
        .map(|value| {
            let deviation = value - mean;
            deviation * deviation
        })
        .collect();
    crate::numpy_reduce::numpy_mean(&squared).sqrt()
}

pub fn fit(design: &[Vec<f64>], treatment: &[f64]) -> Result<Gps, GpsError> {
    let n = design.len();
    if n == 0 {
        return Err(GpsError::EmptySample);
    }
    if treatment.len() != n {
        return Err(GpsError::LengthMismatch);
    }
    let columns = design[0].len();
    if columns == 0 {
        return Err(GpsError::NoColumns);
    }
    if design.iter().any(|row| row.len() != columns) {
        return Err(GpsError::LengthMismatch);
    }
    if design
        .iter()
        .flatten()
        .chain(treatment)
        .any(|value| !value.is_finite())
    {
        return Err(GpsError::NonFiniteValue);
    }

    let predictors = DMatrix::from_fn(n, columns + 1, |row, column| {
        if column == 0 {
            1.0
        } else {
            design[row][column - 1]
        }
    });
    let target = DVector::from_column_slice(treatment);
    let fitted = crate::ols::Ols::fit(&predictors, &target);
    let residual_scale = numpy_std(fitted.resid.as_slice());
    if !residual_scale.is_finite() || residual_scale <= 0.0 {
        return Err(GpsError::DegenerateScale);
    }
    let fitted_treatment: Vec<f64> = (&predictors * &fitted.params).iter().copied().collect();
    let density: Vec<f64> = (0..n)
        .map(|row| normal_pdf(treatment[row], fitted_treatment[row], residual_scale))
        .collect();
    Ok(Gps {
        density,
        fitted_treatment,
        residual_scale,
        treatment_params: fitted.params.iter().copied().collect(),
    })
}

pub fn weights(
    scores: &Gps,
    treatment: &[f64],
    scale: WeightScale,
) -> Result<Vec<f64>, GpsError> {
    if treatment.len() != scores.density.len() {
        return Err(GpsError::LengthMismatch);
    }
    if let Some(row) = scores
        .density
        .iter()
        .position(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err(GpsError::NonPositiveDensity { row });
    }
    match scale {
        WeightScale::InverseDensity => Ok(scores.density.iter().map(|value| 1.0 / value).collect()),
        WeightScale::Stabilized => {
            let mean = crate::numpy_reduce::numpy_mean(treatment);
            let centered: Vec<f64> = treatment.iter().map(|value| value - mean).collect();
            let marginal_scale = numpy_std(&centered);
            if !marginal_scale.is_finite() || marginal_scale <= 0.0 {
                return Err(GpsError::DegenerateScale);
            }
            Ok(treatment
                .iter()
                .zip(&scores.density)
                .map(|(value, density)| normal_pdf(*value, mean, marginal_scale) / density)
                .collect())
        }
    }
}

#[derive(Clone, Debug)]
pub struct GpsBootstrap {
    pub lower: f64,
    pub upper: f64,
    pub estimates: Vec<f64>,
}

pub fn bootstrap_interval(
    design: &[Vec<f64>],
    treatment: &[f64],
    outcome: &[f64],
    scale: WeightScale,
    rounds: usize,
    seed: u32,
    lower_percentile: f64,
    upper_percentile: f64,
) -> Result<GpsBootstrap, GpsError> {
    let n = design.len();
    if n == 0 {
        return Err(GpsError::EmptySample);
    }
    if treatment.len() != n || outcome.len() != n {
        return Err(GpsError::LengthMismatch);
    }
    let mut rng = crate::nprandom::Mt19937::seeded(seed);
    let mut estimates = Vec::with_capacity(rounds);
    for _ in 0..rounds {
        let rows = crate::dowhy_bootstrap::resample_rows(&mut rng, n, n);
        let resampled_design: Vec<Vec<f64>> = rows.iter().map(|&row| design[row].clone()).collect();
        let resampled_treatment: Vec<f64> = rows.iter().map(|&row| treatment[row]).collect();
        let resampled_outcome: Vec<f64> = rows.iter().map(|&row| outcome[row]).collect();
        let round = estimate(
            &resampled_design,
            &resampled_treatment,
            &resampled_outcome,
            scale,
        )?;
        estimates.push(round.effect);
    }
    if estimates.is_empty() {
        return Err(GpsError::EmptySample);
    }
    Ok(GpsBootstrap {
        lower: crate::causal_effects::numpy_percentile(&estimates, lower_percentile),
        upper: crate::causal_effects::numpy_percentile(&estimates, upper_percentile),
        estimates,
    })
}

pub fn estimate(
    design: &[Vec<f64>],
    treatment: &[f64],
    outcome: &[f64],
    scale: WeightScale,
) -> Result<GpsEffect, GpsError> {
    let n = design.len();
    if outcome.len() != n {
        return Err(GpsError::LengthMismatch);
    }
    let scores = fit(design, treatment)?;
    let weights = weights(&scores, treatment, scale)?;
    let predictors = DMatrix::from_fn(n, 2, |row, column| {
        if column == 0 {
            1.0
        } else {
            treatment[row]
        }
    });
    let fitted = crate::estimation::wls(outcome, &predictors, &weights);
    let weight_sum = crate::numpy_reduce::numpy_sum(&weights);
    Ok(GpsEffect {
        effect: fitted.params[1],
        intercept: fitted.params[0],
        standard_error: fitted.bse[1],
        weights,
        weight_sum,
    })
}
