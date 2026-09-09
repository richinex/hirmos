//! Numerical methods from `ComparisonSurv.R`.

use std::f64::consts::PI;

use super::data::{ComparisonData, Event, Group, SurvivalSample};
use super::km::{curve, value_at, CurvePoint};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Weight {
    GehanWilcoxon,
    TaroneWare,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Side {
    OneSided,
    TwoSided,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeightedRankResult {
    pub weight: Weight,
    pub statistic: f64,
    pub p_value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeightedKaplanMeierResult {
    pub statistic: f64,
    pub p_value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinWangResult {
    pub delta: f64,
    pub expected_delta: f64,
    pub variance: f64,
    pub statistic: f64,
    pub p_value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinXuResult {
    pub delta: f64,
    pub expected_delta: f64,
    pub variance: f64,
    pub side: Side,
    pub correlation: f64,
    pub statistic: f64,
    pub p_value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShortTermResult {
    pub time: f64,
    pub weighted_kaplan_meier_statistic: f64,
    pub weighted_kaplan_meier_p_value: f64,
    pub log_rank_statistic: f64,
    pub log_rank_p_value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LongTermResult {
    pub time: f64,
    pub partial_log_rank_statistic: f64,
    pub partial_log_rank_p_value: f64,
    pub ordinary_least_squares_statistic: f64,
    pub ordinary_least_squares_p_value: f64,
    pub sample_path_statistic: f64,
    pub sample_path_p_value: f64,
    pub quadratic_statistic: f64,
    pub quadratic_p_value: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixedPointScale {
    Naive,
    Log,
    ComplementaryLogLog,
    ArcsineSquareRoot,
    Logit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixedPointEstimate {
    pub scale: FixedPointScale,
    pub time: f64,
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixedPointTest {
    pub scale: FixedPointScale,
    pub statistic: f64,
    pub p_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FixedPointResult {
    pub group_zero: Vec<FixedPointEstimate>,
    pub group_one: Vec<FixedPointEstimate>,
    pub tests: Vec<FixedPointTest>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComparisonError {
    InvalidCorrelation,
    FixedTimeOutsideEventRange,
    ComparisonTimeTooEarly,
    ComparisonTimeTooLate,
    UndefinedStatistic,
}

#[derive(Clone, Copy)]
struct EventRow {
    time: f64,
    risk_zero: f64,
    event_zero: f64,
    risk_one: f64,
    event_one: f64,
}

fn normal_cdf(value: f64) -> f64 {
    0.5 * libm::erfc(-value / std::f64::consts::SQRT_2)
}

fn two_sided_normal(value: f64) -> f64 {
    2.0 * (1.0 - normal_cdf(value.abs()))
}

fn chi_square_one_survival(value: f64) -> f64 {
    two_sided_normal(value.max(0.0).sqrt())
}

fn round_five(value: f64) -> f64 {
    (value * 100_000.0).round() / 100_000.0
}

fn event_table(data: &ComparisonData) -> Vec<EventRow> {
    let zero = data.group_samples(Group::Zero);
    let one = data.group_samples(Group::One);
    let mut times = data
        .samples()
        .iter()
        .filter(|sample| sample.event() == Event::Observed)
        .map(|sample| sample.time())
        .collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    times.dedup();
    times
        .into_iter()
        .map(|time| EventRow {
            time,
            risk_zero: zero.iter().filter(|sample| sample.time() >= time).count() as f64,
            event_zero: zero
                .iter()
                .filter(|sample| sample.time() == time && sample.event() == Event::Observed)
                .count() as f64,
            risk_one: one.iter().filter(|sample| sample.time() >= time).count() as f64,
            event_one: one
                .iter()
                .filter(|sample| sample.time() == time && sample.event() == Event::Observed)
                .count() as f64,
        })
        .collect()
}

fn hypergeometric(row: EventRow) -> (f64, f64, f64) {
    let risk = row.risk_zero + row.risk_one;
    let events = row.event_zero + row.event_one;
    let expected = row.risk_zero * events / risk;
    let variance = if risk <= 1.0 {
        0.0
    } else {
        row.risk_zero / risk
            * (1.0 - row.risk_zero / risk)
            * ((risk - events) / (risk - 1.0))
            * events
    };
    (row.event_zero - expected, expected, variance)
}

pub(crate) fn log_rank(data: &ComparisonData) -> Result<(f64, f64), ComparisonError> {
    let mut difference = 0.0;
    let mut variance = 0.0;
    for row in event_table(data) {
        let (row_difference, _, row_variance) = hypergeometric(row);
        difference += row_difference;
        variance += row_variance;
    }
    if variance <= 0.0 {
        return Err(ComparisonError::UndefinedStatistic);
    }
    let statistic = difference * difference / variance;
    Ok((statistic, chi_square_one_survival(statistic)))
}

pub fn breslow_tarone_ware(data: &ComparisonData, weight: Weight) -> WeightedRankResult {
    let mut numerator = 0.0;
    let mut variance = 0.0;
    for row in event_table(data) {
        let risk = row.risk_zero + row.risk_one;
        let (difference, _, row_variance) = hypergeometric(row);
        let multiplier = match weight {
            Weight::GehanWilcoxon => risk,
            Weight::TaroneWare => risk.sqrt(),
        };
        numerator += multiplier * difference;
        variance += multiplier * multiplier * row_variance;
    }
    let statistic = numerator / variance.sqrt();
    WeightedRankResult {
        weight,
        statistic,
        p_value: two_sided_normal(statistic),
    }
}

fn flipped(samples: &[SurvivalSample]) -> Vec<SurvivalSample> {
    samples
        .iter()
        .copied()
        .map(|sample| {
            sample.with_event(match sample.event() {
                Event::Censored => Event::Observed,
                Event::Observed => Event::Censored,
            })
        })
        .collect()
}

struct WeightedCurveRow {
    time: f64,
    pooled: f64,
    previous_pooled: f64,
    zero: f64,
    one: f64,
    weight: f64,
}

fn weighted_curve_rows(data: &ComparisonData) -> Vec<WeightedCurveRow> {
    let zero = data.group_samples(Group::Zero);
    let one = data.group_samples(Group::One);
    let pooled = curve(data.samples());
    let zero_curve = curve(&zero);
    let one_curve = curve(&one);
    let zero_censor = curve(&flipped(&zero));
    let one_censor = curve(&flipped(&one));
    let n_zero = zero.len() as f64;
    let n_one = one.len() as f64;
    let mut previous_pooled = 1.0;
    let mut previous_zero_censor = 1.0;
    let mut previous_one_censor = 1.0;
    let mut rows = Vec::new();
    for point in pooled {
        let s_zero = value_at(&zero_curve, point.time).survival;
        let s_one = value_at(&one_curve, point.time).survival;
        let c_zero = value_at(&zero_censor, point.time).survival;
        let c_one = value_at(&one_censor, point.time).survival;
        if c_zero > 0.0 && c_one > 0.0 && s_zero > 0.0 && s_one > 0.0 {
            let weight = (n_zero + n_one) * previous_zero_censor * previous_one_censor
                / (n_zero * previous_zero_censor + n_one * previous_one_censor);
            rows.push(WeightedCurveRow {
                time: point.time,
                pooled: point.survival,
                previous_pooled,
                zero: s_zero,
                one: s_one,
                weight,
            });
            previous_pooled = point.survival;
            previous_zero_censor = c_zero;
            previous_one_censor = c_one;
        }
    }
    rows
}

pub fn weighted_kaplan_meier(
    data: &ComparisonData,
) -> Result<WeightedKaplanMeierResult, ComparisonError> {
    let rows = weighted_curve_rows(data);
    if rows.len() < 2 {
        return Err(ComparisonError::UndefinedStatistic);
    }
    let n_zero = data.group_samples(Group::Zero).len() as f64;
    let n_one = data.group_samples(Group::One).len() as f64;
    let active = &rows[..rows.len() - 1];
    let differences = active
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let width = rows[index + 1].time - row.time;
            row.weight * (row.one - row.zero) * width
        })
        .sum::<f64>();
    let statistic_numerator = (n_zero * n_one / (n_zero + n_one)).sqrt() * differences;

    let products = active
        .iter()
        .enumerate()
        .map(|(index, row)| row.weight * row.pooled * (rows[index + 1].time - row.time))
        .collect::<Vec<_>>();
    let mut reverse_sum = 0.0;
    let mut integrals = vec![0.0; products.len()];
    for index in (0..products.len()).rev() {
        reverse_sum += products[index];
        integrals[index] = reverse_sum;
    }
    let variance = -active
        .iter()
        .zip(integrals)
        .map(|(row, integral)| {
            let survival_change = row.pooled - row.previous_pooled;
            integral * integral * survival_change / (row.pooled * row.previous_pooled * row.weight)
        })
        .sum::<f64>();
    let statistic = statistic_numerator / variance.sqrt();
    Ok(WeightedKaplanMeierResult {
        statistic,
        p_value: two_sided_normal(statistic),
    })
}

pub fn lin_wang(data: &ComparisonData) -> LinWangResult {
    let mut delta = 0.0;
    let mut expected_delta = 0.0;
    let mut variance_delta = 0.0;
    for row in event_table(data) {
        let risk = row.risk_zero + row.risk_one;
        let events = row.event_zero + row.event_one;
        let (_, expected, variance) = hypergeometric(row);
        delta += (row.event_zero - expected).powi(2);
        expected_delta += variance;
        let second = variance + expected * expected;
        let third_factor = if risk <= 2.0 {
            0.0
        } else {
            events
                * (events - 1.0)
                * (events - 2.0)
                * row.risk_zero
                * (row.risk_zero - 1.0)
                * (row.risk_zero - 2.0)
                / (risk * (risk - 1.0) * (risk - 2.0) * 6.0)
        };
        let third = 3.0 * second - 2.0 * expected + 6.0 * third_factor;
        let fourth_factor = if risk <= 3.0 {
            0.0
        } else {
            events
                * (events - 1.0)
                * (events - 2.0)
                * (events - 3.0)
                * row.risk_zero
                * (row.risk_zero - 1.0)
                * (row.risk_zero - 2.0)
                * (row.risk_zero - 3.0)
                / (risk * (risk - 1.0) * (risk - 2.0) * (risk - 3.0) * 24.0)
        };
        let fourth = 6.0 * third - 11.0 * second + 6.0 * expected + 24.0 * fourth_factor;
        variance_delta += fourth - 4.0 * third * expected + 6.0 * second * expected.powi(2)
            - 3.0 * expected.powi(4)
            - variance.powi(2);
    }
    let statistic = (delta - expected_delta) / variance_delta.sqrt();
    LinWangResult {
        delta,
        expected_delta,
        variance: variance_delta,
        statistic,
        p_value: two_sided_normal(statistic),
    }
}

pub fn lin_xu(
    data: &ComparisonData,
    side: Side,
    correlation: f64,
) -> Result<LinXuResult, ComparisonError> {
    if !correlation.is_finite() || !(0.0..1.0).contains(&correlation) {
        return Err(ComparisonError::InvalidCorrelation);
    }
    let zero = data.group_samples(Group::Zero);
    let one = data.group_samples(Group::One);
    let zero_curve = curve(&zero);
    let one_curve = curve(&one);
    let mut times = zero_curve
        .iter()
        .chain(&one_curve)
        .map(|point| point.time)
        .collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    times.dedup();
    if times.len() < 2 {
        return Err(ComparisonError::UndefinedStatistic);
    }

    let last_zero = zero_curve
        .last()
        .ok_or(ComparisonError::UndefinedStatistic)?;
    let last_one = one_curve
        .last()
        .ok_or(ComparisonError::UndefinedStatistic)?;
    let zero_last_event = usize::from(last_zero.events > 0) as f64;
    let one_last_event = usize::from(last_one.events > 0) as f64;
    let tau = if zero_last_event == 0.0 && one_last_event == 0.0 {
        last_zero.time.min(last_one.time)
    } else if zero_last_event == 0.0 || one_last_event == 0.0 {
        (last_zero.time * (1.0 - zero_last_event)).max(last_one.time * (1.0 - one_last_event))
    } else {
        last_zero.time.max(last_one.time)
    };

    let mut selected = times
        .iter()
        .copied()
        .filter(|time| {
            data.samples()
                .iter()
                .any(|sample| sample.time() == *time && sample.event() == Event::Observed)
        })
        .collect::<Vec<_>>();
    if times.binary_search_by(|time| time.total_cmp(&tau)).is_ok() {
        selected.push(tau);
    }
    let widths = selected
        .iter()
        .enumerate()
        .map(|(index, time)| selected.get(index + 1).map_or(0.0, |next| next - time))
        .collect::<Vec<_>>();
    let delta = selected
        .iter()
        .zip(&widths)
        .map(|(time, width)| {
            (value_at(&zero_curve, *time).survival - value_at(&one_curve, *time).survival).abs()
                * width
        })
        .sum::<f64>();
    let variances = selected
        .iter()
        .map(|time| {
            let zero_point = value_at(&zero_curve, *time);
            let one_point = value_at(&one_curve, *time);
            zero_point.survival.powi(2) * zero_point.greenwood_sum
                + one_point.survival.powi(2) * one_point.greenwood_sum
        })
        .collect::<Vec<_>>();
    let expected_delta = variances
        .iter()
        .zip(&widths)
        .map(|(variance, width)| (2.0 / PI * variance).sqrt() * width)
        .sum::<f64>();
    let variance_diagonal = variances
        .iter()
        .zip(&widths)
        .map(|(variance, width)| (1.0 - 2.0 / PI) * variance * width * width)
        .sum::<f64>();
    let mut variance_cross = 0.0;
    for left in 0..variances.len().saturating_sub(1) {
        for right in left + 1..variances.len() {
            variance_cross += 2.0
                * correlation
                * widths[left]
                * widths[right]
                * (1.0 - 2.0 / PI)
                * (variances[left] * variances[right]).sqrt();
        }
    }
    let variance = variance_diagonal + variance_cross;
    let statistic = (delta - expected_delta) / variance.sqrt();
    let p_value = match side {
        Side::OneSided => 1.0 - normal_cdf(statistic),
        Side::TwoSided => two_sided_normal(statistic),
    };
    Ok(LinXuResult {
        delta,
        expected_delta,
        variance,
        side,
        correlation,
        statistic,
        p_value,
    })
}

pub fn short_term(data: &ComparisonData, time: f64) -> Result<ShortTermResult, ComparisonError> {
    let event_times = data
        .samples()
        .iter()
        .filter(|sample| sample.event() == Event::Observed)
        .map(|sample| sample.time())
        .collect::<Vec<_>>();
    if !time.is_finite() || time <= event_times.iter().copied().fold(f64::INFINITY, f64::min) {
        return Err(ComparisonError::ComparisonTimeTooEarly);
    }
    if time
        >= event_times
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    {
        return Err(ComparisonError::ComparisonTimeTooLate);
    }

    let truncated = data
        .samples()
        .iter()
        .map(|sample| {
            if sample.time() > time {
                SurvivalSample::censored_at(time, sample.group())
            } else {
                *sample
            }
        })
        .collect::<Vec<_>>();
    let truncated =
        ComparisonData::new(truncated).map_err(|_| ComparisonError::UndefinedStatistic)?;
    let weighted = weighted_kaplan_meier(&truncated)?;
    let (log_rank_statistic, log_rank_p_value) = log_rank(&truncated)?;
    Ok(ShortTermResult {
        time,
        weighted_kaplan_meier_statistic: weighted.statistic,
        weighted_kaplan_meier_p_value: weighted.p_value,
        log_rank_statistic,
        log_rank_p_value,
    })
}

fn nelson_aalen_at(points: &[CurvePoint], time: f64) -> (f64, f64) {
    points.iter().filter(|point| point.time <= time).fold(
        (0.0, 0.0),
        |(hazard, variance), point| {
            let events = point.events as f64;
            let risk = point.at_risk as f64;
            (hazard + events / risk, variance + events / (risk * risk))
        },
    )
}

pub fn long_term(data: &ComparisonData, time: f64) -> Result<LongTermResult, ComparisonError> {
    let event_times = data
        .samples()
        .iter()
        .filter(|sample| sample.event() == Event::Observed)
        .map(|sample| sample.time())
        .collect::<Vec<_>>();
    if !time.is_finite() || time <= event_times.iter().copied().fold(f64::INFINITY, f64::min) {
        return Err(ComparisonError::ComparisonTimeTooEarly);
    }
    if time
        >= event_times
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    {
        return Err(ComparisonError::ComparisonTimeTooLate);
    }

    let zero = curve(&data.group_samples(Group::Zero));
    let one = curve(&data.group_samples(Group::One));
    let pooled = curve(data.samples());
    let (zero_hazard, zero_hazard_variance) = nelson_aalen_at(&zero, time);
    let (one_hazard, one_hazard_variance) = nelson_aalen_at(&one, time);
    let hazard_difference = zero_hazard - one_hazard;
    let hazard_variance = zero_hazard_variance + one_hazard_variance;
    let hazard_statistic = hazard_difference / hazard_variance.sqrt();

    let after = event_table(data)
        .into_iter()
        .filter(|row| row.time > time)
        .collect::<Vec<_>>();
    if after.is_empty() {
        return Err(ComparisonError::ComparisonTimeTooLate);
    }
    let mut log_rank_numerator = 0.0;
    let mut log_rank_variance = 0.0;
    for row in after {
        let risk = row.risk_zero + row.risk_one;
        let events = row.event_zero + row.event_one;
        log_rank_numerator +=
            (row.risk_one * row.event_zero - row.risk_zero * row.event_one) / risk;
        log_rank_variance += if risk <= 1.0 {
            0.0
        } else {
            events * row.risk_zero * row.risk_one / (risk * risk) * (risk - events) / (risk - 1.0)
        };
    }
    if log_rank_variance <= 0.0 || hazard_variance <= 0.0 {
        return Err(ComparisonError::UndefinedStatistic);
    }
    let partial_log_rank = log_rank_numerator / log_rank_variance.sqrt();
    let ordinary_least_squares = (hazard_statistic + partial_log_rank) / 2.0_f64.sqrt();

    // ComparisonSurv uses the number of distinct per-group observation times here,
    // because that is what `survfit(...)$strata` contains in the source routine.
    let n_zero = zero.len() as f64;
    let n_one = one.len() as f64;
    let n = n_zero + n_one;
    let zero_before = zero.iter().filter(|point| point.time < time).count();
    let one_before = one.iter().filter(|point| point.time < time).count();
    let pooled_before = pooled.iter().filter(|point| point.time < time).count();
    if zero_before == 0 || one_before == 0 || pooled_before == 0 {
        return Err(ComparisonError::ComparisonTimeTooEarly);
    }
    let zero_survival = zero[zero_before - 1].survival;
    let one_survival = one[one_before - 1].survival;
    let pooled_point = pooled[pooled_before - 1];
    let pooled_variance = pooled_point.survival.powi(2) * pooled_point.greenwood_sum;
    let sample_path = (n_zero * n_one * (zero_survival - one_survival) / n + log_rank_numerator)
        / (n_zero * n_one * pooled_variance + log_rank_variance).sqrt();
    let quadratic = hazard_statistic * hazard_statistic + partial_log_rank * partial_log_rank;

    Ok(LongTermResult {
        time,
        partial_log_rank_statistic: partial_log_rank,
        partial_log_rank_p_value: two_sided_normal(partial_log_rank),
        ordinary_least_squares_statistic: ordinary_least_squares,
        ordinary_least_squares_p_value: two_sided_normal(ordinary_least_squares),
        sample_path_statistic: sample_path,
        sample_path_p_value: two_sided_normal(sample_path),
        quadratic_statistic: quadratic,
        quadratic_p_value: chi_square_one_survival(quadratic),
    })
}

pub fn fixed_point(data: &ComparisonData, time: f64) -> Result<FixedPointResult, ComparisonError> {
    let event_times = event_table(data)
        .into_iter()
        .map(|row| row.time)
        .collect::<Vec<_>>();
    if !time.is_finite() || time <= event_times[0] || time >= event_times[event_times.len() - 1] {
        return Err(ComparisonError::FixedTimeOutsideEventRange);
    }
    let zero = value_at(&curve(&data.group_samples(Group::Zero)), time);
    let one = value_at(&curve(&data.group_samples(Group::One)), time);
    let survival = [zero.survival, one.survival];
    let greenwood = [zero.greenwood_sum, one.greenwood_sum];
    let scales = [
        FixedPointScale::Naive,
        FixedPointScale::Log,
        FixedPointScale::ComplementaryLogLog,
        FixedPointScale::ArcsineSquareRoot,
        FixedPointScale::Logit,
    ];
    let mut estimates = [Vec::new(), Vec::new()];
    let mut tests = Vec::new();
    for scale in scales {
        let (statistic, intervals) = fixed_scale(scale, survival, greenwood);
        tests.push(FixedPointTest {
            scale,
            statistic: round_five(statistic),
            p_value: round_five(chi_square_one_survival(statistic)),
        });
        for group in 0..2 {
            estimates[group].push(FixedPointEstimate {
                scale,
                time,
                estimate: survival[group],
                lower: intervals[group].0,
                upper: intervals[group].1,
            });
        }
    }
    Ok(FixedPointResult {
        group_zero: estimates[0].clone(),
        group_one: estimates[1].clone(),
        tests,
    })
}

fn fixed_scale(
    scale: FixedPointScale,
    survival: [f64; 2],
    greenwood: [f64; 2],
) -> (f64, [(f64, f64); 2]) {
    let [s0, s1] = survival;
    let [g0, g1] = greenwood;
    match scale {
        FixedPointScale::Naive => {
            let statistic = (s0 - s1).powi(2) / (s0 * s0 * g0 + s1 * s1 * g1);
            let intervals = [
                (s0 - 1.96 * s0 * g0.sqrt(), s0 + 1.96 * s0 * g0.sqrt()),
                (s1 - 1.96 * s1 * g1.sqrt(), s1 + 1.96 * s1 * g1.sqrt()),
            ];
            (statistic, intervals)
        }
        FixedPointScale::Log => {
            let statistic = (s0.ln() - s1.ln()).powi(2) / (g0 + g1);
            let interval = |s: f64, g: f64| {
                (
                    s.powf(1.0 / (1.96 * g / s.ln()).exp()),
                    s.powf((1.96 * g / s.ln()).exp()),
                )
            };
            (statistic, [interval(s0, g0), interval(s1, g1)])
        }
        FixedPointScale::ComplementaryLogLog => {
            let statistic = (s0.ln().neg().ln() - s1.ln().neg().ln()).powi(2)
                / (g0 / s0.ln().powi(2) + g1 / s1.ln().powi(2));
            let interval = |s: f64, g: f64| {
                (
                    (-(((-s.ln()).ln() - 1.96 * g * s.ln()).exp())).exp(),
                    (-(((-s.ln()).ln() + 1.96 * g * s.ln()).exp())).exp(),
                )
            };
            (statistic, [interval(s0, g0), interval(s1, g1)])
        }
        FixedPointScale::ArcsineSquareRoot => {
            let variance = |s: f64, g: f64| s * g / (4.0 * (1.0 - s));
            let statistic = (s0.sqrt().asin() - s1.sqrt().asin()).powi(2)
                / (variance(s0, g0) + variance(s1, g1));
            let interval = |s: f64, g: f64| {
                let shift = 0.5 * 1.96 * g * (s / (1.0 - s)).sqrt();
                (
                    (s.sqrt().asin() - shift).max(0.0).sin().powi(2),
                    (s.sqrt().asin() + shift).min(PI / 2.0).sin().powi(2),
                )
            };
            (statistic, [interval(s0, g0), interval(s1, g1)])
        }
        FixedPointScale::Logit => {
            let statistic = ((s0 / (1.0 - s0)).ln() - (s1 / (1.0 - s1)).ln()).powi(2)
                / (g0 / (1.0 - s0).powi(2) + g1 / (1.0 - s1).powi(2));
            let interval = |s: f64, g: f64| {
                let ratio = (1.96 * g / (1.0 - s)).exp();
                (
                    s / ((ratio - 1.0) * s + ratio),
                    ratio * s / ((ratio - 1.0) * s + 1.0),
                )
            };
            (statistic, [interval(s0, g0), interval(s1, g1)])
        }
    }
}

pub fn crossing_times(data: &ComparisonData) -> Vec<f64> {
    let zero = curve(&data.group_samples(Group::Zero));
    let one = curve(&data.group_samples(Group::One));
    let mut times = data
        .samples()
        .iter()
        .map(|sample| sample.time())
        .collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    times.dedup();
    let differences = times
        .iter()
        .map(|time| value_at(&zero, *time).survival - value_at(&one, *time).survival)
        .collect::<Vec<_>>();
    let mut crossings = Vec::new();
    for index in 0..times.len() {
        let crossed = if index == 0 {
            differences[0] == 0.0
        } else {
            differences[index] * differences[index - 1] <= 0.0
        };
        if crossed {
            crossings.push(times[index]);
        }
    }
    crossings
}

use std::ops::Neg;
