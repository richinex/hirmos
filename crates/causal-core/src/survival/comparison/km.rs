//! Kaplan–Meier and event-table primitives matching `survival::survfit`.

use super::data::{Event, SurvivalSample};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CurvePoint {
    pub time: f64,
    pub at_risk: usize,
    pub events: usize,
    pub censored: usize,
    pub survival: f64,
    pub greenwood_sum: f64,
}

pub(crate) fn curve(samples: &[SurvivalSample]) -> Vec<CurvePoint> {
    let mut times = samples
        .iter()
        .map(|sample| sample.time())
        .collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    times.dedup();
    let mut survival = 1.0;
    let mut greenwood = 0.0;
    let mut points = Vec::with_capacity(times.len());
    for time in times {
        let at_risk = samples
            .iter()
            .filter(|sample| sample.time() >= time)
            .count();
        let events = samples
            .iter()
            .filter(|sample| sample.time() == time && sample.event() == Event::Observed)
            .count();
        let censored = samples
            .iter()
            .filter(|sample| sample.time() == time && sample.event() == Event::Censored)
            .count();
        if events > 0 {
            survival *= 1.0 - events as f64 / at_risk as f64;
            if at_risk > events {
                greenwood += events as f64 / (at_risk * (at_risk - events)) as f64;
            }
        }
        points.push(CurvePoint {
            time,
            at_risk,
            events,
            censored,
            survival,
            greenwood_sum: greenwood,
        });
    }
    points
}

pub(crate) fn value_at(points: &[CurvePoint], time: f64) -> CurvePoint {
    points
        .iter()
        .copied()
        .take_while(|point| point.time <= time)
        .last()
        .unwrap_or(CurvePoint {
            time,
            at_risk: points.first().map_or(0, |point| point.at_risk),
            events: 0,
            censored: 0,
            survival: 1.0,
            greenwood_sum: 0.0,
        })
}
