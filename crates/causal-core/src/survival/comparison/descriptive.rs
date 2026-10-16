//! Descriptive summaries and restricted means from ComparisonSurv/survRM2.

use spec_math::cephes64::ndtri;

use super::data::{ComparisonData, Event, Group, SurvivalSample};
use super::km::{curve, CurvePoint};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SignificanceLevel(f64);

impl SignificanceLevel {
    pub fn new(value: f64) -> Result<Self, DescriptiveError> {
        if !value.is_finite() || !(0.0..1.0).contains(&value) {
            return Err(DescriptiveError::InvalidSignificanceLevel);
        }
        Ok(Self(value))
    }

    fn normal_cutoff(self) -> f64 {
        ndtri(1.0 - self.0 / 2.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComparisonTime(f64);

impl ComparisonTime {
    pub fn new(value: f64) -> Result<Self, DescriptiveError> {
        if !value.is_finite() || value < 0.0 {
            return Err(DescriptiveError::InvalidTruncationTime);
        }
        Ok(Self(value))
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RmstWindow {
    Observed,
    Event,
    At(ComparisonTime),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DescriptiveError {
    InvalidSignificanceLevel,
    InvalidTruncationTime,
    UndefinedRestrictedMean,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntervalEstimate {
    pub estimate: f64,
    pub standard_error: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantileEstimate {
    pub estimate: Option<f64>,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroupDescription {
    pub sample_size: usize,
    pub events: usize,
    pub censoring_rate: f64,
    pub maximum_observed_time: f64,
    pub maximum_event_time: f64,
    pub mean: IntervalEstimate,
    pub first_quartile: QuantileEstimate,
    pub median: QuantileEstimate,
    pub third_quartile: QuantileEstimate,
    pub restricted_mean: IntervalEstimate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RmstContrast {
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
    pub p_value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RmstComparison {
    pub group_one_minus_zero: RmstContrast,
    pub group_one_over_zero: RmstContrast,
    pub loss_group_one_over_zero: RmstContrast,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DescriptiveResult {
    pub truncation_time: f64,
    pub group_zero: GroupDescription,
    pub group_one: GroupDescription,
    pub restricted_mean_comparison: RmstComparison,
}

fn normal_cdf(value: f64) -> f64 {
    0.5 * libm::erfc(-value / std::f64::consts::SQRT_2)
}

fn restricted_mean(samples: &[SurvivalSample], tau: f64) -> Result<(f64, f64), DescriptiveError> {
    if !tau.is_finite() || tau < 0.0 {
        return Err(DescriptiveError::InvalidTruncationTime);
    }
    let points = curve(samples);
    let retained = points
        .iter()
        .copied()
        .filter(|point| point.time <= tau)
        .collect::<Vec<_>>();
    let mut times = retained.iter().map(|point| point.time).collect::<Vec<_>>();
    times.push(tau);
    times.sort_by(f64::total_cmp);
    let mut previous_time = 0.0;
    let mut previous_survival = 1.0;
    let mut areas = Vec::with_capacity(times.len());
    for (index, time) in times.iter().copied().enumerate() {
        areas.push((time - previous_time) * previous_survival);
        previous_time = time;
        if let Some(point) = retained.get(index) {
            previous_survival = point.survival;
        }
    }
    let estimate = areas.iter().sum::<f64>();

    let greenwood = retained
        .iter()
        .map(|point| {
            if point.at_risk == point.events {
                0.0
            } else {
                point.events as f64 / (point.at_risk * (point.at_risk - point.events)) as f64
            }
        })
        .collect::<Vec<_>>();
    let mut remaining_area = 0.0;
    let mut variance = 0.0;
    for index in (0..retained.len()).rev() {
        remaining_area += areas[index + 1];
        variance += remaining_area * remaining_area * greenwood[index];
    }
    if !estimate.is_finite() || !variance.is_finite() {
        return Err(DescriptiveError::UndefinedRestrictedMean);
    }
    Ok((estimate, variance))
}

fn log_confidence(point: CurvePoint, cutoff: f64) -> (f64, f64) {
    if point.survival == 0.0 {
        return (f64::NAN, f64::NAN);
    }
    let width = cutoff * point.greenwood_sum.sqrt();
    (
        (point.survival.ln() - width).exp(),
        (point.survival.ln() + width).exp().min(1.0),
    )
}

fn quantile_from_curve(times: &[f64], distribution: &[f64], probability: f64) -> Option<f64> {
    let tolerance = f64::EPSILON.sqrt();
    let maximum_time = *times.last()?;
    let mut unique_times = Vec::new();
    let mut unique_distribution = Vec::new();
    for (&time, &value) in times.iter().zip(distribution) {
        if unique_distribution.last().copied() != Some(value) {
            unique_times.push(time);
            unique_distribution.push(value);
        }
    }
    let lower_index = unique_distribution
        .iter()
        .position(|value| value.is_finite() && *value + tolerance >= probability)?;
    let upper_index = unique_distribution
        .iter()
        .position(|value| value.is_finite() && *value - tolerance >= probability)?;
    if probability == 0.0 {
        return unique_times.first().copied();
    }
    let last = *unique_distribution.last()?;
    if last.is_finite() && (probability - last).abs() < tolerance {
        return Some((unique_times[lower_index] + maximum_time) / 2.0);
    }
    Some((unique_times[lower_index] + unique_times[upper_index]) / 2.0)
}

fn quantile(points: &[CurvePoint], probability: f64, cutoff: f64) -> QuantileEstimate {
    let mut times = Vec::with_capacity(points.len() + 1);
    let mut estimate = Vec::with_capacity(points.len() + 1);
    let mut lower = Vec::with_capacity(points.len() + 1);
    let mut upper = Vec::with_capacity(points.len() + 1);
    times.push(0.0);
    estimate.push(0.0);
    lower.push(0.0);
    upper.push(0.0);
    for point in points {
        let (survival_lower, survival_upper) = log_confidence(*point, cutoff);
        times.push(point.time);
        estimate.push(1.0 - point.survival);
        lower.push(1.0 - survival_lower);
        upper.push(1.0 - survival_upper);
    }
    QuantileEstimate {
        estimate: quantile_from_curve(&times, &estimate, probability),
        // survival::quantile reverses the survival-band labels when mapping to time.
        lower: quantile_from_curve(&times, &lower, probability),
        upper: quantile_from_curve(&times, &upper, probability),
    }
}

fn group_description(
    samples: &[SurvivalSample],
    common_mean_end: f64,
    rmst_end: f64,
    cutoff: f64,
) -> Result<GroupDescription, DescriptiveError> {
    let points = curve(samples);
    let events = samples
        .iter()
        .filter(|sample| sample.event() == Event::Observed)
        .count();
    let maximum_observed_time = samples
        .iter()
        .map(|sample| sample.time())
        .fold(f64::NEG_INFINITY, f64::max);
    let maximum_event_time = samples
        .iter()
        .filter(|sample| sample.event() == Event::Observed)
        .map(|sample| sample.time())
        .fold(f64::NEG_INFINITY, f64::max);
    let (mean, mean_variance) = restricted_mean(samples, common_mean_end)?;
    let mean_standard_error = mean_variance.sqrt();
    let (rmst, rmst_variance) = restricted_mean(samples, rmst_end)?;
    let rmst_standard_error = rmst_variance.sqrt();
    Ok(GroupDescription {
        sample_size: samples.len(),
        events,
        censoring_rate: 1.0 - events as f64 / samples.len() as f64,
        maximum_observed_time,
        maximum_event_time,
        // Descrip.method applies sqrt() to survfit's se(rmean); preserve it.
        mean: IntervalEstimate {
            estimate: mean,
            standard_error: mean_standard_error,
            lower: mean - cutoff * mean_standard_error.sqrt(),
            upper: mean + cutoff * mean_standard_error.sqrt(),
        },
        first_quartile: quantile(&points, 0.25, cutoff),
        median: quantile(&points, 0.5, cutoff),
        third_quartile: quantile(&points, 0.75, cutoff),
        restricted_mean: IntervalEstimate {
            estimate: rmst,
            standard_error: rmst_standard_error,
            lower: rmst - cutoff * rmst_standard_error,
            upper: rmst + cutoff * rmst_standard_error,
        },
    })
}

fn contrast(estimate: f64, standard_error: f64, cutoff: f64) -> RmstContrast {
    RmstContrast {
        estimate,
        lower: estimate - cutoff * standard_error,
        upper: estimate + cutoff * standard_error,
        p_value: 2.0 * normal_cdf(-(estimate / standard_error).abs()),
    }
}

pub fn describe(
    data: &ComparisonData,
    window: RmstWindow,
    significance: SignificanceLevel,
) -> Result<DescriptiveResult, DescriptiveError> {
    let zero = data.group_samples(Group::Zero);
    let one = data.group_samples(Group::One);
    let maxima = [
        zero.iter().map(|sample| sample.time()).fold(0.0, f64::max),
        one.iter().map(|sample| sample.time()).fold(0.0, f64::max),
    ];
    let event_maxima = [
        zero.iter()
            .filter(|sample| sample.event() == Event::Observed)
            .map(|sample| sample.time())
            .fold(0.0, f64::max),
        one.iter()
            .filter(|sample| sample.event() == Event::Observed)
            .map(|sample| sample.time())
            .fold(0.0, f64::max),
    ];
    let truncation_time = match window {
        RmstWindow::Observed => maxima[0].min(maxima[1]),
        RmstWindow::Event => event_maxima[0].min(event_maxima[1]),
        RmstWindow::At(value) => value.get(),
    };
    if truncation_time > maxima[0].min(maxima[1]) {
        return Err(DescriptiveError::InvalidTruncationTime);
    }
    let cutoff = significance.normal_cutoff();
    let common_mean_end = maxima[0].max(maxima[1]);
    let group_zero = group_description(&zero, common_mean_end, truncation_time, cutoff)?;
    let group_one = group_description(&one, common_mean_end, truncation_time, cutoff)?;
    let variance_zero = group_zero.restricted_mean.standard_error.powi(2);
    let variance_one = group_one.restricted_mean.standard_error.powi(2);
    let rmst_difference = group_one.restricted_mean.estimate - group_zero.restricted_mean.estimate;
    let rmst_difference_se = (variance_zero + variance_one).sqrt();
    let rmst_log_ratio =
        (group_one.restricted_mean.estimate / group_zero.restricted_mean.estimate).ln();
    let rmst_log_ratio_se = (variance_one / group_one.restricted_mean.estimate.powi(2)
        + variance_zero / group_zero.restricted_mean.estimate.powi(2))
    .sqrt();
    let loss_one = truncation_time - group_one.restricted_mean.estimate;
    let loss_zero = truncation_time - group_zero.restricted_mean.estimate;
    let loss_log_ratio = (loss_one / loss_zero).ln();
    let loss_log_ratio_se =
        (variance_one / loss_one.powi(2) + variance_zero / loss_zero.powi(2)).sqrt();
    let log_ratio = contrast(rmst_log_ratio, rmst_log_ratio_se, cutoff);
    let log_loss_ratio = contrast(loss_log_ratio, loss_log_ratio_se, cutoff);
    Ok(DescriptiveResult {
        truncation_time,
        group_zero,
        group_one,
        restricted_mean_comparison: RmstComparison {
            group_one_minus_zero: contrast(rmst_difference, rmst_difference_se, cutoff),
            group_one_over_zero: RmstContrast {
                estimate: log_ratio.estimate.exp(),
                lower: log_ratio.lower.exp(),
                upper: log_ratio.upper.exp(),
                p_value: log_ratio.p_value,
            },
            loss_group_one_over_zero: RmstContrast {
                estimate: log_loss_ratio.estimate.exp(),
                lower: log_loss_ratio.lower.exp(),
                upper: log_loss_ratio.upper.exp(),
                p_value: log_loss_ratio.p_value,
            },
        },
    })
}
