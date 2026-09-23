//! Explicit source preprocessing. Every removed or recoded record is reported.
use super::{Adoption, Anticipation, Controls, Error, Panel, Period, Row, Unit};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq)]
pub enum Change {
    BeyondObservedWindow { unit: Unit, adoption: Period },
    LatestCohortAsComparison { unit: Unit, adoption: Period },
    NoUntreatedComparison { period: Period },
    NoPreTreatment { unit: Unit },
}

pub struct Prepared {
    pub panel: Panel,
    pub changes: Vec<Change>,
    pub small_cohorts: Vec<SmallCohort>,
}

#[derive(Debug, PartialEq)]
pub struct SmallCohort {
    pub adoption: Adoption,
    pub count: usize,
    pub required: usize,
}

/// Balanced, complete panel scope. Unbalanced panels and non-finite records are
/// rejected, never implicitly converted into repeated cross sections.
pub fn prepare(
    rows: &[Row],
    controls: Controls,
    anticipation: Anticipation,
) -> Result<Prepared, Error> {
    prepare_with_covariates(rows, controls, anticipation, 0)
}

/// Covariate count excludes the intercept. The initial scope accepts numerical
/// columns, so it equals the number of right-hand-side variables in the R formula.
pub fn prepare_with_covariates(
    rows: &[Row],
    controls: Controls,
    anticipation: Anticipation,
    covariates: usize,
) -> Result<Prepared, Error> {
    let mut keys = BTreeSet::new();
    let mut cohorts = BTreeMap::new();
    let mut periods = BTreeSet::new();
    for row in rows {
        if !row.outcome.is_finite() {
            return Err(Error::NonFinite {
                unit: row.unit,
                period: row.period,
            });
        }
        if !keys.insert((row.unit, row.period)) {
            return Err(Error::Duplicate {
                unit: row.unit,
                period: row.period,
            });
        }
        if cohorts
            .insert(row.unit, row.adoption)
            .is_some_and(|old| old != row.adoption)
        {
            return Err(Error::CohortChanged { unit: row.unit });
        }
        periods.insert(row.period);
    }
    if periods.len() < 2 {
        return Err(Error::TooFewPeriods);
    }
    for unit in cohorts.keys() {
        if periods.iter().any(|t| !keys.contains(&(*unit, *t))) {
            return Err(Error::Unbalanced { unit: *unit });
        }
    }
    let mut changes = Vec::new();
    let last = i128::from(periods.last().unwrap().0);
    let a = i128::from(anticipation.0);
    for (unit, g) in &mut cohorts {
        if let Adoption::At(adoption) = *g {
            if i128::from(adoption.0) > last + a {
                changes.push(Change::BeyondObservedWindow {
                    unit: *unit,
                    adoption,
                });
                *g = Adoption::Never;
            }
        }
    }
    if !cohorts.values().any(|g| *g == Adoption::Never) {
        let latest = cohorts
            .values()
            .filter_map(|g| match g {
                Adoption::At(g) => Some(*g),
                Adoption::Never => None,
            })
            .max()
            .ok_or(Error::MissingControls)?;
        periods.retain(|t| {
            let keep = i128::from(t.0) < i128::from(latest.0) - a;
            if !keep {
                changes.push(Change::NoUntreatedComparison { period: *t });
            }
            keep
        });
        if matches!(controls, Controls::NeverTreated) {
            for (unit, g) in &mut cohorts {
                if *g == Adoption::At(latest) {
                    changes.push(Change::LatestCohortAsComparison {
                        unit: *unit,
                        adoption: latest,
                    });
                    *g = Adoption::Never;
                }
            }
        }
    }
    let first = periods.first().ok_or(Error::TooFewPeriods)?;
    cohorts.retain(|unit, g| {
        let keep = !matches!(g,Adoption::At(g) if i128::from(g.0)<=i128::from(first.0)+a);
        if !keep {
            changes.push(Change::NoPreTreatment { unit: *unit });
        }
        keep
    });
    let retained: Vec<_> = rows
        .iter()
        .filter_map(|r| {
            let adoption = *cohorts.get(&r.unit)?;
            periods
                .contains(&r.period)
                .then_some(Row { adoption, ..*r })
        })
        .collect();
    let panel = Panel::new(&retained)?;
    let required = covariates.checked_add(5).ok_or(Error::NumericalOverflow)?;
    let mut sizes = BTreeMap::<Option<Period>, usize>::new();
    for g in cohorts.values() {
        *sizes
            .entry(match g {
                Adoption::Never => None,
                Adoption::At(g) => Some(*g),
            })
            .or_default() += 1;
    }
    let small_cohorts: Vec<_> = sizes
        .into_iter()
        .filter(|(_, n)| *n < required)
        .map(|(g, count)| SmallCohort {
            adoption: g.map(Adoption::At).unwrap_or(Adoption::Never),
            count,
            required,
        })
        .collect();
    if matches!(controls, Controls::NeverTreated) {
        if let Some(group) = small_cohorts.iter().find(|g| g.adoption == Adoption::Never) {
            return Err(Error::SmallControlGroup {
                count: group.count,
                required,
            });
        }
    }
    Ok(Prepared {
        panel,
        changes,
        small_cohorts,
    })
}
