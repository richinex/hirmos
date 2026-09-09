//! Source-compatible automatic initial values for flexsurv 2.3.2.

use super::fit::SurvivalDataset;
use super::model::{FlexSurvFamily, RegressionModel};
use super::survreg::{self, Censoring, ErrorDistribution, Observation as SurvregObservation};

#[derive(Clone, Debug, PartialEq)]
pub enum InitialValueError {
    NonPositiveTransformedTime { row: usize },
    NoPositiveTimes,
    DegenerateTimes,
    SurvregFailed,
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn sample_variance(values: &[f64]) -> f64 {
    let center = mean(values);
    values
        .iter()
        .map(|value| {
            let residual = value - center;
            residual * residual
        })
        .sum::<f64>()
        / (values.len() - 1) as f64
}

fn type_seven_quantile(sorted: &[f64], probability: f64) -> f64 {
    let index = (sorted.len() - 1) as f64 * probability;
    let lower = index.floor() as usize;
    let fraction = index - lower as f64;
    if fraction == 0.0 {
        sorted[lower]
    } else {
        sorted[lower] + fraction * (sorted[lower + 1] - sorted[lower])
    }
}

fn weighted_mean_and_variance(values: &[f64], weights: &[f64]) -> (f64, f64) {
    let total_weight = weights.iter().sum::<f64>();
    let center = values
        .iter()
        .zip(weights)
        .map(|(value, weight)| value * weight)
        .sum::<f64>()
        / total_weight;
    let variance = values
        .iter()
        .zip(weights)
        .map(|(value, weight)| weight * (value - center).powi(2))
        .sum::<f64>()
        / total_weight;
    (center, variance)
}

fn source_distribution_initial(
    values: &[f64],
    weights: &[f64],
    distribution: ErrorDistribution,
) -> (f64, f64) {
    let (center, variance) = weighted_mean_and_variance(values, weights);
    match distribution {
        ErrorDistribution::ExtremeValue => (center + 0.572, variance / 1.64),
        ErrorDistribution::Logistic => (center, variance / 3.2),
        ErrorDistribution::Gaussian => (center, variance),
    }
}

fn prepare_survreg_observations(
    data: &SurvivalDataset,
) -> Result<Vec<SurvregObservation>, InitialValueError> {
    data.records()
        .iter()
        .copied()
        .enumerate()
        .map(|(row, record)| {
            let bounds = record.observation().bounds();
            let censoring = if bounds.exact {
                Censoring::Exact
            } else if bounds.upper.is_infinite() {
                Censoring::Right
            } else if bounds.lower == 0.0 {
                Censoring::Left
            } else {
                Censoring::Interval
            };
            let lower = if censoring == Censoring::Left {
                bounds.upper
            } else {
                bounds.lower
            };
            if lower <= 0.0 || (censoring == Censoring::Interval && bounds.upper <= 0.0) {
                return Err(InitialValueError::NonPositiveTransformedTime { row });
            }
            Ok(SurvregObservation {
                lower: lower.ln(),
                upper: if censoring == Censoring::Interval {
                    bounds.upper.ln()
                } else {
                    0.0
                },
                censoring,
                weight: record.weight(),
            })
        })
        .collect()
}

fn rescale_design(design: &[f64], rows: usize, columns: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut centers = vec![0.0; columns];
    let mut scales = vec![1.0; columns];
    for column in 0..columns {
        let values = (0..rows)
            .map(|row| design[row * columns + column])
            .collect::<Vec<_>>();
        let binary = values.iter().all(|value| *value == 0.0 || *value == 1.0);
        if !binary {
            centers[column] = mean(&values);
            scales[column] = sample_variance(&values).sqrt();
        }
    }
    let mut scaled = Vec::with_capacity(design.len());
    for row in 0..rows {
        for column in 0..columns {
            scaled.push((design[row * columns + column] - centers[column]) / scales[column]);
        }
    }
    (scaled, centers, scales)
}

fn glim_weights(
    observations: &[SurvregObservation],
    centers: &[f64],
    sigma: f64,
    distribution: ErrorDistribution,
) -> (Vec<f64>, Vec<f64>) {
    let mut working_weights = Vec::with_capacity(observations.len());
    let mut responses = Vec::with_capacity(observations.len());
    for (observation, center) in observations.iter().copied().zip(centers) {
        let derivatives = survreg::row_derivatives(observation, *center, sigma, distribution);
        working_weights.push(-derivatives.location_second * observation.weight);
        responses.push(
            -derivatives.location_second * observation.weight * center
                + observation.weight * derivatives.location,
        );
    }
    (working_weights, responses)
}

fn automatic_survreg(
    data: &SurvivalDataset,
    original_design: &[f64],
    columns: usize,
    distribution: ErrorDistribution,
    fixed_scale: Option<f64>,
) -> Result<(Vec<f64>, f64), InitialValueError> {
    let observations = prepare_survreg_observations(data)?;
    let rows = observations.len();
    let (design, centers, scales) = if columns > 1 {
        rescale_design(original_design, rows, columns)
    } else {
        (
            original_design.to_vec(),
            vec![0.0; columns],
            vec![1.0; columns],
        )
    };
    let midpoint = observations
        .iter()
        .map(|row| {
            if row.censoring == Censoring::Interval {
                (row.lower + row.upper) / 2.0
            } else {
                row.lower
            }
        })
        .collect::<Vec<_>>();
    let weights = observations
        .iter()
        .map(|row| row.weight)
        .collect::<Vec<_>>();
    let (_, initial_variance) = source_distribution_initial(&midpoint, &weights, distribution);
    let initial_log_scale = fixed_scale
        .map(f64::ln)
        .unwrap_or_else(|| (4.0 * initial_variance).ln() / 2.0);

    let scale_for_glim = initial_log_scale.exp();
    let (mean_weights, mean_response) =
        glim_weights(&observations, &midpoint, scale_for_glim, distribution);
    let mean_denominator = mean_weights.iter().sum::<f64>();
    let improved_center = mean_response.iter().sum::<f64>() / mean_denominator;
    let mean_design = vec![1.0; rows];
    let mut mean_initial = vec![improved_center];
    if fixed_scale.is_none() {
        mean_initial.push(initial_log_scale);
    }
    let mean_fit = survreg::fit(
        &observations,
        &mean_design,
        1,
        &mean_initial,
        fixed_scale,
        distribution,
        20,
        1e-9,
        1e-10,
    )
    .map_err(|_| InitialValueError::SurvregFailed)?;
    let fitted_log_scale = fixed_scale
        .map(f64::ln)
        .unwrap_or_else(|| mean_fit.coefficients[1]);

    let (working_weights, working_response) = glim_weights(
        &observations,
        &midpoint,
        fitted_log_scale.exp(),
        distribution,
    );
    let mut cross_product = vec![0.0; columns * columns];
    let mut right_hand_side = vec![0.0; columns];
    for row in 0..rows {
        for left in 0..columns {
            let left_value = design[row * columns + left];
            right_hand_side[left] += left_value * working_response[row];
            for right in 0..columns {
                cross_product[left * columns + right] +=
                    left_value * working_weights[row] * design[row * columns + right];
            }
        }
    }
    let coefficients = survreg::symmetric_solve(cross_product, right_hand_side, 1e-10);
    let mut initial = coefficients;
    if fixed_scale.is_none() {
        initial.push(fitted_log_scale);
    }
    let fit = survreg::fit(
        &observations,
        &design,
        columns,
        &initial,
        fixed_scale,
        distribution,
        30,
        1e-9,
        1e-10,
    )
    .map_err(|_| InitialValueError::SurvregFailed)?;

    let mut unscaled = fit.coefficients[..columns].to_vec();
    for column in 1..columns {
        unscaled[column] /= scales[column];
        unscaled[0] -= centers[column] / scales[column] * fit.coefficients[column];
    }
    let scale = fixed_scale.unwrap_or_else(|| fit.coefficients[columns].exp());
    Ok((unscaled, scale))
}

fn source_times(data: &SurvivalDataset) -> Vec<f64> {
    let weights = data
        .records()
        .iter()
        .map(|record| record.weight())
        .collect::<Vec<_>>();
    let total_weight = weights.iter().sum::<f64>();
    let multiplier = data.len() as f64 / total_weight;
    data.records()
        .iter()
        .copied()
        .zip(weights)
        .map(|(record, weight)| {
            let bounds = record.observation().bounds();
            let time = if !bounds.exact && bounds.upper.is_finite() && bounds.lower > 0.0 {
                (bounds.lower + bounds.upper) / 2.0
            } else {
                bounds.lower
            };
            time * weight * multiplier
        })
        .collect()
}

pub(crate) fn automatic_initial_values(
    model: &RegressionModel,
    data: &SurvivalDataset,
) -> Result<(Vec<f64>, Vec<f64>), InitialValueError> {
    let times = source_times(data);
    let counting = data.records()[0].observation().is_counting();
    let location_columns = model.location_coefficient_count();
    let mut coefficients = vec![0.0; model.coefficient_count()];

    let common = match model.family() {
        FlexSurvFamily::Exponential => Some((ErrorDistribution::ExtremeValue, Some(1.0))),
        FlexSurvFamily::Weibull | FlexSurvFamily::WeibullPh => {
            Some((ErrorDistribution::ExtremeValue, None))
        }
        FlexSurvFamily::LogNormal => Some((ErrorDistribution::Gaussian, None)),
        FlexSurvFamily::LogLogistic => Some((ErrorDistribution::Logistic, None)),
        _ => None,
    };
    if let Some((distribution, fixed_scale)) = common {
        if !counting {
            let columns = 1 + location_columns;
            let (survreg_coefficients, survreg_scale) = automatic_survreg(
                data,
                &model.location_design(),
                columns,
                distribution,
                fixed_scale,
            )?;
            let intercept = survreg_coefficients[0];
            let location = &survreg_coefficients[1..];
            return Ok(match model.family() {
                FlexSurvFamily::Exponential => {
                    coefficients[..location_columns]
                        .iter_mut()
                        .zip(location)
                        .for_each(|(target, value)| *target = -*value);
                    (vec![(-intercept).exp()], coefficients)
                }
                FlexSurvFamily::Weibull => {
                    coefficients[..location_columns].copy_from_slice(location);
                    (vec![1.0 / survreg_scale, intercept.exp()], coefficients)
                }
                FlexSurvFamily::WeibullPh => {
                    let shape = 1.0 / survreg_scale;
                    coefficients[..location_columns]
                        .iter_mut()
                        .zip(location)
                        .for_each(|(target, value)| *target = -*value * shape);
                    (vec![shape, intercept.exp().powf(-shape)], coefficients)
                }
                FlexSurvFamily::LogNormal => {
                    coefficients[..location_columns].copy_from_slice(location);
                    (vec![intercept, survreg_scale], coefficients)
                }
                FlexSurvFamily::LogLogistic => {
                    coefficients[..location_columns].copy_from_slice(location);
                    (vec![1.0 / survreg_scale, intercept.exp()], coefficients)
                }
                FlexSurvFamily::GeneralizedGamma
                | FlexSurvFamily::GeneralizedGammaOriginal
                | FlexSurvFamily::GeneralizedF
                | FlexSurvFamily::GeneralizedFOriginal
                | FlexSurvFamily::Gamma
                | FlexSurvFamily::Gompertz => return Err(InitialValueError::SurvregFailed),
            });
        }
    }

    let positive = times
        .iter()
        .copied()
        .filter(|time| *time > 0.0)
        .collect::<Vec<_>>();
    if positive.is_empty() {
        return Err(InitialValueError::NoPositiveTimes);
    }
    let logged = positive.iter().map(|time| time.ln()).collect::<Vec<_>>();
    if logged.len() < 2 || sample_variance(&logged) == 0.0 {
        return Err(InitialValueError::DegenerateTimes);
    }
    let natural = match model.family() {
        FlexSurvFamily::Exponential => vec![1.0 / mean(&times)],
        FlexSurvFamily::Weibull => vec![
            1.64 / sample_variance(&logged),
            (mean(&logged) + 0.572).exp(),
        ],
        FlexSurvFamily::WeibullPh => {
            let shape = 1.64 / sample_variance(&logged);
            let scale = (mean(&logged) + 0.572).exp();
            vec![shape, scale.powf(-shape)]
        }
        FlexSurvFamily::LogNormal => vec![mean(&logged), sample_variance(&logged).sqrt()],
        FlexSurvFamily::LogLogistic => {
            let mut sorted = times.clone();
            sorted.sort_by(f64::total_cmp);
            let scale = type_seven_quantile(&sorted, 0.5);
            let lower_quartile = type_seven_quantile(&sorted, 0.25);
            let mut shape = 1.0 / (lower_quartile / scale).log(3.0);
            if shape < 0.0 {
                shape = 1.0;
            }
            vec![shape, scale]
        }
        FlexSurvFamily::GeneralizedGamma => {
            vec![mean(&logged), sample_variance(&logged).sqrt(), 0.0]
        }
        FlexSurvFamily::GeneralizedGammaOriginal => vec![1.0, mean(&times), 1.0],
        FlexSurvFamily::GeneralizedF => {
            vec![mean(&logged), sample_variance(&logged).sqrt(), 0.0, 1.0]
        }
        FlexSurvFamily::GeneralizedFOriginal => {
            vec![mean(&logged), sample_variance(&logged).sqrt(), 1.0, 1.0]
        }
        FlexSurvFamily::Gamma => {
            let center = mean(&times);
            let variance = sample_variance(&times);
            vec![center * center / variance, center / variance]
        }
        FlexSurvFamily::Gompertz => vec![0.001, 1.0 / mean(&times)],
    };
    Ok((natural, coefficients))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::survival::flexsurv::fit::SurvivalRecord;
    use crate::survival::flexsurv::model::CovariateMatrix;
    use crate::survival::flexsurv::observation::SurvivalObservation;
    use serde_json::Value;

    fn fixture() -> Value {
        serde_json::from_str(include_str!("../../../oracle/fixtures/flexsurv.json")).unwrap()
    }

    fn number(value: &Value) -> f64 {
        value.as_f64().unwrap()
    }

    fn ovarian(
        family: FlexSurvFamily,
        covariate: Option<&str>,
    ) -> (RegressionModel, SurvivalDataset) {
        let fixture = fixture();
        let source = &fixture["ovarian"];
        let times = source["futime"].as_array().unwrap();
        let events = source["fustat"].as_array().unwrap();
        let records = times
            .iter()
            .zip(events)
            .map(|(time, event)| {
                let observation = if event.as_i64().unwrap() == 1 {
                    SurvivalObservation::exact(number(time)).unwrap()
                } else {
                    SurvivalObservation::right_censored(number(time)).unwrap()
                };
                SurvivalRecord::new(observation)
            })
            .collect();
        let model = RegressionModel::new(family, times.len()).unwrap();
        let model = match covariate {
            Some(name) => {
                let values = source[name]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(number)
                    .collect();
                model
                    .with_location_covariates(CovariateMatrix::new(times.len(), 1, values).unwrap())
                    .unwrap()
            }
            None => model,
        };
        (model, SurvivalDataset::new(records).unwrap())
    }

    fn interval() -> (RegressionModel, SurvivalDataset) {
        let fixture = fixture();
        let rows = fixture["censoring_data"].as_array().unwrap();
        let mut records = Vec::with_capacity(rows.len());
        let mut covariates = Vec::with_capacity(rows.len());
        for row in rows {
            let lower = row["lower"].as_f64();
            let upper = row["upper"].as_f64();
            let observation = match row["kind"].as_i64().unwrap() {
                0 => SurvivalObservation::right_censored(lower.unwrap()).unwrap(),
                1 => SurvivalObservation::exact(lower.unwrap()).unwrap(),
                2 => SurvivalObservation::left_censored(upper.unwrap()).unwrap(),
                3 => {
                    SurvivalObservation::interval_censored(lower.unwrap(), upper.unwrap()).unwrap()
                }
                kind => panic!("unexpected censoring kind {kind}"),
            };
            records.push(SurvivalRecord::weighted(observation, number(&row["weight"])).unwrap());
            covariates.push(number(&row["x"]));
        }
        let model = RegressionModel::new(FlexSurvFamily::Weibull, rows.len())
            .unwrap()
            .with_location_covariates(CovariateMatrix::new(rows.len(), 1, covariates).unwrap())
            .unwrap();
        (model, SurvivalDataset::new(records).unwrap())
    }

    fn counting() -> (RegressionModel, SurvivalDataset) {
        let fixture = fixture();
        let rows = fixture["counting_data"].as_array().unwrap();
        let mut records = Vec::with_capacity(rows.len());
        let mut covariates = Vec::with_capacity(rows.len());
        for row in rows {
            let start = number(&row["start"]);
            let stop = number(&row["stop"]);
            let observation = if row["event"].as_i64().unwrap() == 1 {
                SurvivalObservation::delayed_event(start, stop).unwrap()
            } else {
                SurvivalObservation::delayed_right_censored(start, stop).unwrap()
            };
            records.push(SurvivalRecord::new(observation));
            covariates.push(number(&row["x"]));
        }
        let model = RegressionModel::new(FlexSurvFamily::Weibull, rows.len())
            .unwrap()
            .with_location_covariates(CovariateMatrix::new(rows.len(), 1, covariates).unwrap())
            .unwrap();
        (model, SurvivalDataset::new(records).unwrap())
    }

    fn assert_initials(name: &str, model: RegressionModel, data: SurvivalDataset, tolerance: f64) {
        let fixture = fixture();
        let expected = fixture["automatic_initials"][name].as_array().unwrap();
        let (baseline, coefficients) = automatic_initial_values(&model, &data).unwrap();
        let actual = baseline.into_iter().chain(coefficients).collect::<Vec<_>>();
        assert_eq!(actual.len(), expected.len());
        for (index, value) in actual.iter().copied().enumerate() {
            let target = number(&expected[index]);
            assert!(
                (value - target).abs() <= tolerance * target.abs().max(1.0),
                "{name} initial {index}: actual={value:.17e}, expected={target:.17e}"
            );
        }
    }

    #[test]
    fn automatic_initial_values_match_flexsurv_2_3_2() {
        let cases = [
            ("exponential_age", FlexSurvFamily::Exponential, Some("age")),
            (
                "weibull_treatment",
                FlexSurvFamily::Weibull,
                Some("rx_indicator"),
            ),
            ("lognormal_age", FlexSurvFamily::LogNormal, Some("age")),
            ("loglogistic_age", FlexSurvFamily::LogLogistic, Some("age")),
            ("gamma_intercept", FlexSurvFamily::Gamma, None),
            (
                "generalized_gamma_intercept",
                FlexSurvFamily::GeneralizedGamma,
                None,
            ),
        ];
        for (name, family, covariate) in cases {
            let (model, data) = ovarian(family, covariate);
            assert_initials(name, model, data, 3e-11);
        }
        let (model, data) = interval();
        assert_initials("interval_weibull", model, data, 3e-10);
        let (model, data) = counting();
        assert_initials("counting_weibull", model, data, 3e-14);
    }
}
