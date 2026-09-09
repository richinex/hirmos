//! Survival A/B-test summaries from base `survival` and the article baseline.

use super::data::{ComparisonData, Event, Group};
use super::km::{curve, value_at};
use super::methods::{fixed_point, two_sided_normal, ComparisonError, FixedPointResult};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GRho(f64);

impl GRho {
    pub fn new(value: f64) -> Result<Self, AbTestError> {
        if !value.is_finite() || value < 0.0 {
            return Err(AbTestError::InvalidRho);
        }
        Ok(Self(value))
    }

    pub const fn log_rank() -> Self {
        Self(0.0)
    }
    pub const fn peto_peto() -> Self {
        Self(1.0)
    }
    pub const fn get(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObservedConversionResult {
    pub control_rate: f64,
    pub treatment_rate: f64,
    pub difference: f64,
    pub standard_error: f64,
    pub statistic: f64,
    pub p_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FixedTimeConversionResult {
    pub time: f64,
    pub control_rate: f64,
    pub treatment_rate: f64,
    pub difference: f64,
    pub standard_error: f64,
    pub interval: [f64; 2],
    pub statistic: f64,
    pub p_value: f64,
    pub comparison_surv: FixedPointResult,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GRhoResult {
    pub rho: GRho,
    pub observed: [f64; 2],
    pub expected: [f64; 2],
    pub variance: [[f64; 2]; 2],
    pub statistic: f64,
    pub p_value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbTestError {
    InvalidRho,
    UndefinedStatistic,
    FixedTime(ComparisonError),
}

pub fn observed_conversion(data: &ComparisonData) -> Result<ObservedConversionResult, AbTestError> {
    let (control_events, control_n) = group_event_count(data, Group::Zero);
    let (treatment_events, treatment_n) = group_event_count(data, Group::One);
    let control_rate = control_events as f64 / control_n as f64;
    let treatment_rate = treatment_events as f64 / treatment_n as f64;
    let variance = control_rate * (1.0 - control_rate) / control_n as f64
        + treatment_rate * (1.0 - treatment_rate) / treatment_n as f64;
    if variance <= 0.0 || !variance.is_finite() {
        return Err(AbTestError::UndefinedStatistic);
    }
    let standard_error = variance.sqrt();
    let difference = treatment_rate - control_rate;
    let statistic = difference.abs() / standard_error;
    Ok(ObservedConversionResult {
        control_rate,
        treatment_rate,
        difference,
        standard_error,
        statistic,
        p_value: two_sided_normal(statistic),
    })
}

pub fn fixed_time_conversion(
    data: &ComparisonData,
    time: f64,
) -> Result<FixedTimeConversionResult, AbTestError> {
    let comparison_surv = fixed_point(data, time).map_err(AbTestError::FixedTime)?;
    let control = value_at(&curve(&data.group_samples(Group::Zero)), time);
    let treatment = value_at(&curve(&data.group_samples(Group::One)), time);
    let difference = control.survival - treatment.survival;
    let variance = control.survival.powi(2) * control.greenwood_sum
        + treatment.survival.powi(2) * treatment.greenwood_sum;
    if variance <= 0.0 || !variance.is_finite() {
        return Err(AbTestError::UndefinedStatistic);
    }
    let standard_error = variance.sqrt();
    let statistic = difference.abs() / standard_error;
    let cutoff = 1.959_963_984_540_054;
    Ok(FixedTimeConversionResult {
        time,
        control_rate: 1.0 - control.survival,
        treatment_rate: 1.0 - treatment.survival,
        difference,
        standard_error,
        interval: [
            difference - cutoff * standard_error,
            difference + cutoff * standard_error,
        ],
        statistic,
        p_value: two_sided_normal(statistic),
        comparison_surv,
    })
}

/// Port of `survival::survdiff.fit` and `survdiff2.c` for two groups and one stratum.
pub fn g_rho_test(data: &ComparisonData, rho: GRho) -> Result<GRhoResult, AbTestError> {
    let mut samples = approximately_equal_survival_times(data);
    samples.sort_by(|left, right| {
        left.0
            .total_cmp(&right.0)
            .then_with(|| right.1.cmp(&left.1))
    });
    let n = samples.len();
    let mut kaplan = vec![0.0; n];
    if rho.get() != 0.0 {
        let mut survival = 1.0;
        let mut start = 0;
        while start < n {
            let time = samples[start].0;
            let mut end = start;
            let mut deaths = 0;
            while end < n && samples[end].0 == time {
                kaplan[end] = survival;
                deaths += usize::from(samples[end].1 == Event::Observed);
                end += 1;
            }
            let at_risk = n - start;
            survival *= (at_risk - deaths) as f64 / at_risk as f64;
            start = end;
        }
    }

    let mut risk = [0.0; 2];
    let mut observed = [0.0; 2];
    let mut expected = [0.0; 2];
    let mut variance = [[0.0; 2]; 2];
    let mut end = n;
    while end > 0 {
        let time = samples[end - 1].0;
        let mut start = end - 1;
        while start > 0 && samples[start - 1].0 == time {
            start -= 1;
        }
        let weight = if rho.get() == 0.0 {
            1.0
        } else {
            kaplan[start].powf(rho.get())
        };
        let mut deaths = 0.0;
        for sample in &samples[start..end] {
            let group = group_index(sample.2);
            let event = f64::from(sample.1 == Event::Observed);
            deaths += event;
            risk[group] += 1.0;
            observed[group] += event * weight;
        }
        let at_risk = (n - start) as f64;
        if deaths > 0.0 {
            for group in 0..2 {
                expected[group] += weight * deaths * risk[group] / at_risk;
            }
            if at_risk > 1.0 {
                let squared_weight = weight * weight;
                for row in 0..2 {
                    let contribution = squared_weight * deaths * risk[row] * (at_risk - deaths)
                        / (at_risk * (at_risk - 1.0));
                    variance[row][row] += contribution;
                    for column in 0..2 {
                        variance[row][column] -= contribution * risk[column] / at_risk;
                    }
                }
            }
        }
        end = start;
    }

    if variance[0][0] <= 0.0 || !variance[0][0].is_finite() {
        return Err(AbTestError::UndefinedStatistic);
    }
    let statistic = (observed[0] - expected[0]).powi(2) / variance[0][0];
    Ok(GRhoResult {
        rho,
        observed,
        expected,
        variance,
        statistic,
        p_value: super::methods::chi_square_one_survival(statistic),
    })
}

/// Port of `survival::aeqSurv`, which `survdiff` applies by default.
fn approximately_equal_survival_times(data: &ComparisonData) -> Vec<(f64, Event, Group)> {
    let mut times: Vec<_> = data.samples().iter().map(|sample| sample.time()).collect();
    times.sort_by(f64::total_cmp);
    times.dedup();

    let mean_absolute = times.iter().map(|time| time.abs()).sum::<f64>() / times.len() as f64;
    let tolerance = f64::EPSILON.sqrt();
    let mut cuts = Vec::with_capacity(times.len());
    cuts.push(times[0]);
    for pair in times.windows(2) {
        let difference = pair[1] - pair[0];
        let tied = difference <= tolerance || difference / mean_absolute <= tolerance;
        if !tied {
            cuts.push(pair[1]);
        }
    }

    data.samples()
        .iter()
        .map(|sample| {
            let cut = cuts.partition_point(|candidate| *candidate <= sample.time()) - 1;
            (cuts[cut], sample.event(), sample.group())
        })
        .collect()
}

fn group_event_count(data: &ComparisonData, group: Group) -> (usize, usize) {
    let samples = data.group_samples(group);
    let events = samples
        .iter()
        .filter(|sample| sample.event() == Event::Observed)
        .count();
    (events, samples.len())
}

const fn group_index(group: Group) -> usize {
    match group {
        Group::Zero => 0,
        Group::One => 1,
    }
}
