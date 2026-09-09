//! Renderer-independent plot data for ComparisonSurv curves.

use super::data::{ComparisonData, Group};
use super::km::curve;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurvivalCurvePoint {
    pub time: f64,
    pub survival: f64,
    pub cumulative_hazard: f64,
    pub at_risk: usize,
    pub events: usize,
    pub censored: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupCurve {
    pub group: Group,
    pub points: Vec<SurvivalCurvePoint>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonCurves {
    pub group_zero: GroupCurve,
    pub group_one: GroupCurve,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HazardCurvePoint {
    pub time: f64,
    pub hazard: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupHazardCurve {
    pub group: Group,
    pub points: Vec<HazardCurvePoint>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonHazardCurves {
    pub group_zero: GroupHazardCurve,
    pub group_one: GroupHazardCurve,
}

fn group_curve(data: &ComparisonData, group: Group) -> GroupCurve {
    let points = curve(&data.group_samples(group))
        .into_iter()
        .map(|point| SurvivalCurvePoint {
            time: point.time,
            survival: point.survival,
            cumulative_hazard: -point.survival.ln(),
            at_risk: point.at_risk,
            events: point.events,
            censored: point.censored,
        })
        .collect();
    GroupCurve { group, points }
}

pub fn comparison_curves(data: &ComparisonData) -> ComparisonCurves {
    ComparisonCurves {
        group_zero: group_curve(data, Group::Zero),
        group_one: group_curve(data, Group::One),
    }
}

fn biquadratic_boundary_kernel(boundary: f64, value: f64) -> f64 {
    if boundary == 1.0 {
        15.0 * (1.0 - value * value).powi(2) / 16.0
    } else {
        let first = 60.0 * (value + 1.0).powi(2) * (boundary - value) / (1.0 + boundary).powi(6);
        let second =
            2.0 * boundary * boundary - 2.0 * boundary + 1.0 + value * (2.0 - 3.0 * boundary);
        first * second
    }
}

fn muhaz_at(
    samples: &[super::data::SurvivalSample],
    time: f64,
    maximum: f64,
    bandwidth: f64,
) -> f64 {
    let mut sorted = samples.to_vec();
    sorted.sort_by(|left, right| left.time().total_cmp(&right.time()));
    let mut estimate = 0.0;
    for (index, sample) in sorted.iter().enumerate() {
        if sample.event() != super::data::Event::Observed
            || sample.time() <= time - bandwidth
            || sample.time() >= time + bandwidth
        {
            continue;
        }
        let standardized = (time - sample.time()) / bandwidth;
        let kernel = if bandwidth <= time && time <= maximum - bandwidth {
            biquadratic_boundary_kernel(1.0, standardized)
        } else if time < bandwidth {
            biquadratic_boundary_kernel(time / bandwidth, standardized)
        } else {
            let boundary = (maximum - time) / bandwidth;
            if standardized < -boundary {
                continue;
            }
            biquadratic_boundary_kernel(boundary, -standardized)
        };
        estimate += kernel / (sorted.len() - index) as f64;
    }
    (estimate / bandwidth).max(0.0)
}

fn smooth_group_hazard(data: &ComparisonData, group: Group) -> GroupHazardCurve {
    let samples = data.group_samples(group);
    let maximum = samples
        .iter()
        .map(|sample| sample.time())
        .fold(0.0, f64::max);
    let points = (0..9)
        .map(|index| {
            let time = maximum * index as f64 / 8.0;
            HazardCurvePoint {
                time,
                hazard: muhaz_at(&samples, time, maximum, 13.0),
            }
        })
        .collect();
    GroupHazardCurve { group, points }
}

/// Plot data from ComparisonSurv's `Hazard.plot` defaults: muhaz global
/// bandwidth 13, biquadratic boundary kernel, and nine estimation points.
pub fn smooth_hazard_curves(data: &ComparisonData) -> ComparisonHazardCurves {
    ComparisonHazardCurves {
        group_zero: smooth_group_hazard(data, Group::Zero),
        group_one: smooth_group_hazard(data, Group::One),
    }
}
