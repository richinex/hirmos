//! Balanced fixed-dose panels, zero anticipation and varying baselines.
//! Reuse binary DiD's keyed panel validation, not its different IF scaling.
use crate::continuous_did::cell;
pub use crate::staggered_did::{Adoption, Period, Row as Observation, Unit};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy)]
pub struct Row {
    pub observation: Observation,
    pub dose: f64,
}

#[derive(Debug)]
pub enum Error {
    Panel(crate::staggered_did::Error),
    InvalidDose(Unit),
    DoseChanged(Unit),
    UnknownAdoption(Period),
    MissingNeverTreated,
    MissingTreated,
    DegenerateInfluence {
        group: Period,
        time: Period,
    },
    Spline(crate::continuous_did::spline::Error),
    Cell {
        group: Period,
        time: Period,
        error: cell::Error,
    },
}

pub struct Sample {
    pub group: Period,
    pub time: Period,
    pub baseline: Period,
    pub units: Vec<Unit>,
    pub dose: Vec<f64>,
    pub change: Vec<f64>,
    pub fit: cell::Cell,
    pub weight: f64,
    pub binary_level: f64,
}

pub struct Fit {
    /// Every influence matrix uses this stable, keyed row order.
    pub units: Vec<Unit>,
    /// Time-major, then group-major, including pre-treatment placebo cells.
    pub cells: Vec<Sample>,
    pub raw_influence: Vec<Vec<f64>>,
    pub raw_level_influence: Vec<Vec<f64>>,
    pub average_level: f64,
    pub average_slope: f64,
    pub level: Vec<f64>,
    pub slope: Vec<f64>,
    pub average_slope_influence: Vec<f64>,
    pub level_influence: Vec<Vec<f64>>,
    pub slope_influence: Vec<Vec<f64>>,
}

/// Initial supported specification: unweighted, balanced, never and not-yet
/// treated comparisons, positive time-invariant doses assigned at adoption.
/// The dose is recorded on every row, including before adoption, as in contdid.
pub(crate) fn dose_by_unit(rows: &[Row]) -> Result<BTreeMap<Unit, f64>, Error> {
    let mut doses = BTreeMap::new();
    for row in rows {
        let r = row.observation;
        if !row.dose.is_finite()
            || match r.adoption {
                Adoption::Never => row.dose != 0.0,
                Adoption::At(_) => row.dose <= 0.0,
            }
        {
            return Err(Error::InvalidDose(r.unit));
        }
        if doses
            .insert(r.unit, row.dose)
            .is_some_and(|old| old != row.dose)
        {
            return Err(Error::DoseChanged(r.unit));
        }
    }
    Ok(doses)
}

pub fn fit(rows: &[Row], grid: &[f64], degree: usize, knots: Vec<f64>) -> Result<Fit, Error> {
    let observations: Vec<_> = rows.iter().map(|r| r.observation).collect();
    let validated = crate::staggered_did::Panel::new(&observations).map_err(Error::Panel)?;
    let units: Vec<_> = validated.units().collect();
    let periods: Vec<_> = observations
        .iter()
        .map(|r| r.period)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let doses = dose_by_unit(rows)?;
    let mut adoption = BTreeMap::new();
    let mut outcomes = BTreeMap::new();
    for row in rows {
        let r = row.observation;
        if let Adoption::At(g) = r.adoption {
            if !periods.contains(&g) {
                return Err(Error::UnknownAdoption(g));
            }
        }
        adoption.insert(r.unit, r.adoption);
        outcomes.insert((r.unit, r.period), r.outcome);
    }
    if !adoption.values().any(|a| *a == Adoption::Never) {
        return Err(Error::MissingNeverTreated);
    }
    let groups: Vec<_> = adoption
        .values()
        .filter_map(|a| match a {
            Adoption::At(g) => Some(*g),
            Adoption::Never => None,
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if groups.is_empty() {
        return Err(Error::MissingTreated);
    }
    let treated_count = adoption.values().filter(|a| **a != Adoption::Never).count();
    if grid.len() < 2 || grid.iter().any(|x| !x.is_finite()) {
        return Err(Error::Spline(crate::continuous_did::spline::Error::InvalidBoundary));
    }
    let bounds = [
        grid.iter().copied().fold(f64::INFINITY, f64::min),
        grid.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    ];
    let basis = crate::continuous_did::spline::Basis::new(degree, knots.clone(), bounds).map_err(Error::Spline)?;
    let mut cells = Vec::new();
    let mut raw_influence = vec![Vec::new(); units.len()];
    let mut raw_level_influence = vec![Vec::new(); units.len()];
    let mut level_influence = vec![vec![0.0; grid.len()]; units.len()];
    let mut slope_influence = level_influence.clone();
    for (ti, time) in periods.iter().enumerate().skip(1) {
        for group in &groups {
            let gi = periods.binary_search(group).expect("validated adoption");
            let baseline = periods[if time < group { ti - 1 } else { gi - 1 }];
            let mut selected = Vec::new();
            let mut dose = Vec::new();
            let mut change = Vec::new();
            for (i, unit) in units.iter().enumerate() {
                let a = adoption[unit];
                if a == Adoption::Never
                    || a == Adoption::At(*group)
                    || matches!(a, Adoption::At(g) if g > *time)
                {
                    selected.push(i);
                    dose.push(if a == Adoption::At(*group) {
                        doses[unit]
                    } else {
                        0.0
                    });
                    change.push(outcomes[&(*unit, *time)] - outcomes[&(*unit, baseline)]);
                }
            }
            let fitted =
                cell::fit(&dose, &change, grid, degree, knots.clone()).map_err(|error| {
                    Error::Cell {
                        group: *group,
                        time: *time,
                        error,
                    }
                })?;
            // compute.pte scales by ALL selected units, not just treated units.
            for row in &mut raw_influence {
                row.push(0.0);
            }
            for (j, i) in selected.iter().enumerate() {
                raw_influence[*i][cells.len()] =
                    units.len() as f64 / selected.len() as f64 * fitted.influence[j];
            }
            let cohort_count = adoption
                .values()
                .filter(|a| **a == Adoption::At(*group))
                .count();
            let weight = if time >= group {
                cohort_count as f64 / treated_count as f64 / (periods.len() - gi) as f64
            } else {
                0.0
            };
            let treated = dose.iter().filter(|d| **d > 0.0).count();
            // process_dose_gt infers the treated membership from nonzero IFs.
            // Refuse its degenerate case rather than silently changing that rule.
            if fitted.influence.iter().filter(|v| **v != 0.0).count() != treated {
                return Err(Error::DegenerateInfluence {
                    group: *group,
                    time: *time,
                });
            }
            let binary = crate::staggered_did::cell(
                &validated,
                *group,
                *time,
                crate::staggered_did::Controls::NotYetTreated,
                crate::staggered_did::Baseline::Varying,
            )
            .map_err(Error::Panel)?;
            let crate::staggered_did::Cell::Estimated(binary) = binary else {
                return Err(Error::DegenerateInfluence {
                    group: *group,
                    time: *time,
                });
            };
            for (row, value) in raw_level_influence.iter_mut().zip(&binary.influence) {
                row.push(*value);
            }
            let mut score = 0;
            for (j, i) in selected.iter().enumerate() {
                if dose[j] == 0.0 {
                    // Preserve process_dose_gt's explicit minus sign on the
                    // binary comparison IF, not a newly derived convention.
                    for value in &mut level_influence[*i] {
                        *value -= weight * binary.influence[*i];
                    }
                    continue;
                }
                let projected: Vec<f64> = (0..fitted.bread.len())
                    .map(|k| {
                        fitted.scores[score]
                            .iter()
                            .enumerate()
                            .map(|(l, x)| x * fitted.bread[l][k])
                            .sum()
                    })
                    .collect();
                score += 1;
                let scale = weight * units.len() as f64 / treated as f64;
                for (k, x) in grid.iter().enumerate() {
                    for (derivative, output) in
                        [(false, &mut level_influence), (true, &mut slope_influence)]
                    {
                        let row =
                            cell::design(&basis, *x, derivative).map_err(|error| Error::Cell {
                                group: *group,
                                time: *time,
                                error,
                            })?;
                        output[*i][k] +=
                            scale * projected.iter().zip(row).map(|(a, b)| a * b).sum::<f64>();
                    }
                }
            }
            cells.push(Sample {
                group: *group,
                time: *time,
                baseline,
                units: selected.iter().map(|i| units[*i]).collect(),
                dose,
                change,
                fit: fitted,
                weight,
                binary_level: binary.att,
            });
        }
    }
    let weight_sum: f64 = cells.iter().map(|c| c.weight).sum();
    for row in level_influence.iter_mut().chain(&mut slope_influence) {
        for value in row {
            *value /= weight_sum;
        }
    }
    // weighted.mean in R divides by the supplied weight sum, including rounding.
    let weighted = |values: Vec<f64>| -> f64 {
        values
            .iter()
            .zip(&cells)
            .map(|(x, c)| x * c.weight)
            .sum::<f64>()
            / weight_sum
    };
    Ok(Fit {
        average_level: weighted(cells.iter().map(|c| c.fit.average_level).collect()),
        average_slope: weighted(cells.iter().map(|c| c.fit.average_slope).collect()),
        level: (0..grid.len())
            .map(|j| weighted(cells.iter().map(|c| c.fit.level[j]).collect()))
            .collect(),
        slope: (0..grid.len())
            .map(|j| weighted(cells.iter().map(|c| c.fit.slope[j]).collect()))
            .collect(),
        average_slope_influence: raw_influence
            .iter()
            .map(|row| weighted(row.clone()))
            .collect(),
        units,
        cells,
        raw_influence,
        raw_level_influence,
        level_influence,
        slope_influence,
    })
}
