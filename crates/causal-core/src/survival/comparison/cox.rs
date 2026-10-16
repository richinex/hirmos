//! The one-covariate Cox and `cox.zph(transform = "km")` path used by Overall.test.

use super::data::{ComparisonData, Event, Group};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProportionalHazardsCheck {
    pub coefficient: f64,
    pub chi_square: f64,
    pub p_value: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoxError {
    SingularInformation,
    DidNotConverge,
}

#[derive(Clone, Copy)]
struct RiskMoments {
    weight: f64,
    first: f64,
    second: f64,
}

fn moments(data: &ComparisonData, time: f64, coefficient: f64, centered: bool) -> RiskMoments {
    let mean = if centered {
        data.samples()
            .iter()
            .map(|sample| usize::from(sample.group() == Group::One) as f64)
            .sum::<f64>()
            / data.len() as f64
    } else {
        0.0
    };
    data.samples()
        .iter()
        .filter(|sample| sample.time() >= time)
        .fold(
            RiskMoments {
                weight: 0.0,
                first: 0.0,
                second: 0.0,
            },
            |sum, sample| {
                let value = usize::from(sample.group() == Group::One) as f64 - mean;
                let risk = (coefficient * value).exp();
                RiskMoments {
                    weight: sum.weight + risk,
                    first: sum.first + risk * value,
                    second: sum.second + risk * value * value,
                }
            },
        )
}

fn death_moments(
    data: &ComparisonData,
    time: f64,
    coefficient: f64,
    centered: bool,
) -> (usize, f64, RiskMoments) {
    let mean = if centered {
        data.samples()
            .iter()
            .map(|sample| usize::from(sample.group() == Group::One) as f64)
            .sum::<f64>()
            / data.len() as f64
    } else {
        0.0
    };
    data.samples()
        .iter()
        .filter(|sample| sample.time() == time && sample.event() == Event::Observed)
        .fold(
            (
                0,
                0.0,
                RiskMoments {
                    weight: 0.0,
                    first: 0.0,
                    second: 0.0,
                },
            ),
            |(count, values, sum), sample| {
                let value = usize::from(sample.group() == Group::One) as f64 - mean;
                let risk = (coefficient * value).exp();
                (
                    count + 1,
                    values + value,
                    RiskMoments {
                        weight: sum.weight + risk,
                        first: sum.first + risk * value,
                        second: sum.second + risk * value * value,
                    },
                )
            },
        )
}

fn event_times(data: &ComparisonData) -> Vec<f64> {
    let mut times = data
        .samples()
        .iter()
        .filter(|sample| sample.event() == Event::Observed)
        .map(|sample| sample.time())
        .collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    times.dedup();
    times
}

fn cox_score_information(data: &ComparisonData, coefficient: f64) -> (f64, f64) {
    let mut score = 0.0;
    let mut information = 0.0;
    for time in event_times(data) {
        let risk = moments(data, time, coefficient, false);
        let (deaths, death_values, death_risk) = death_moments(data, time, coefficient, false);
        score += death_values;
        for step in 0..deaths {
            let fraction = step as f64 / deaths as f64;
            let denominator = risk.weight - fraction * death_risk.weight;
            let first = risk.first - fraction * death_risk.first;
            let second = risk.second - fraction * death_risk.second;
            let mean = first / denominator;
            score -= mean;
            information += second / denominator - mean * mean;
        }
    }
    (score, information)
}

fn fit_coefficient(data: &ComparisonData) -> Result<f64, CoxError> {
    let mut coefficient = 0.0;
    for _ in 0..20 {
        let (score, information) = cox_score_information(data, coefficient);
        if !information.is_finite() || information <= 0.0 {
            return Err(CoxError::SingularInformation);
        }
        let update = score / information;
        coefficient += update;
        if update.abs() <= 1e-9 {
            return Ok(coefficient);
        }
    }
    Err(CoxError::DidNotConverge)
}

fn km_time_weights(data: &ComparisonData) -> Vec<(f64, f64)> {
    let times = event_times(data);
    let mut survival = 1.0;
    let mut weights = Vec::with_capacity(times.len());
    for time in times {
        weights.push((time, 1.0 - survival));
        let risk = data
            .samples()
            .iter()
            .filter(|sample| sample.time() >= time)
            .count();
        let deaths = data
            .samples()
            .iter()
            .filter(|sample| sample.time() == time && sample.event() == Event::Observed)
            .count();
        survival *= 1.0 - deaths as f64 / risk as f64;
    }
    let mean = weights
        .iter()
        .map(|(time, weight)| {
            let deaths = data
                .samples()
                .iter()
                .filter(|sample| sample.time() == *time && sample.event() == Event::Observed)
                .count();
            weight * deaths as f64
        })
        .sum::<f64>()
        / data
            .samples()
            .iter()
            .filter(|sample| sample.event() == Event::Observed)
            .count() as f64;
    weights
        .into_iter()
        .map(|(time, weight)| (time, weight - mean))
        .collect()
}

pub fn proportional_hazards_check(
    data: &ComparisonData,
) -> Result<ProportionalHazardsCheck, CoxError> {
    let coefficient = fit_coefficient(data)?;
    let mut score_time = 0.0;
    let mut information = [[0.0; 2]; 2];
    for (time, time_weight) in km_time_weights(data) {
        let risk = moments(data, time, coefficient, true);
        let (deaths, death_values, death_risk) = death_moments(data, time, coefficient, true);
        let mut score = death_values;
        let mut event_information = 0.0;
        for step in 0..deaths {
            let fraction = step as f64 / deaths as f64;
            let denominator = risk.weight - fraction * death_risk.weight;
            let first = risk.first - fraction * death_risk.first;
            let second = risk.second - fraction * death_risk.second;
            let mean = first / denominator;
            score -= mean;
            event_information += second / denominator - mean * mean;
        }
        score_time += time_weight * score;
        information[0][0] += event_information;
        information[0][1] += time_weight * event_information;
        information[1][0] += time_weight * event_information;
        information[1][1] += time_weight * time_weight * event_information;
    }
    let determinant = information[0][0] * information[1][1] - information[0][1] * information[1][0];
    if !determinant.is_finite() || determinant <= 0.0 {
        return Err(CoxError::SingularInformation);
    }
    let chi_square = score_time * score_time * information[0][0] / determinant;
    let p_value = libm::erfc((chi_square / 2.0).sqrt());
    Ok(ProportionalHazardsCheck {
        coefficient,
        chi_square,
        p_value,
    })
}
