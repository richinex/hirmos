//! Equal-weight post-adoption contrasts against never-treated units.
//! Periods must be consecutive integer indices, not compressed calendar gaps.
use crate::continuous_did::panel::{self, Adoption, Period, Row, Unit};
use std::collections::{BTreeMap, BTreeSet};

/// Optional sample eligibility dates are distinct from treatment adoption.
/// The replication archive assigns such dates even to zero-dose counties.
pub enum Eligibility<'a> {
    All,
    WindowEndWithinPanel(&'a BTreeMap<Unit, Option<Period>>),
}

#[derive(Debug)]
pub enum Error {
    Panel(crate::staggered_did::Error),
    Dose(panel::Error),
    InvalidWindow,
    IrregularPeriods,
    EligibilityKeys,
    UnknownAdoption(Period),
    MissingControls(Period),
    NoRetainedTreated,
    Numerical,
}

#[derive(Debug)]
pub struct Contrast {
    pub unit: Unit,
    pub group: Period,
    pub baseline: Period,
    pub dose: f64,
    pub change: f64,
    pub comparison_change: f64,
    pub contrast: f64,
    pub comparison_units: usize,
    pub periods: usize,
}

pub struct Prepared {
    pub rows: Vec<Contrast>,
    pub excluded_units: Vec<Unit>,
}

/// Baseline is the period before adoption. Every requested post period must
/// exist; no partial-window averages or implicit interpolation are permitted.
pub fn prepare(
    rows: &[Row],
    first: i64,
    last: i64,
    eligibility: Eligibility<'_>,
) -> Result<Prepared, Error> {
    if first < 0 || last < first {
        return Err(Error::InvalidWindow);
    }
    let observations: Vec<_> = rows.iter().map(|r| r.observation).collect();
    let panel = crate::staggered_did::Panel::new(&observations).map_err(Error::Panel)?;
    let doses = panel::dose_by_unit(rows).map_err(Error::Dose)?;
    let units: BTreeSet<_> = panel.units().collect();
    let periods: Vec<_> = observations
        .iter()
        .map(|r| r.period)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if periods
        .windows(2)
        .any(|p| p[0].0.checked_add(1) != Some(p[1].0))
    {
        return Err(Error::IrregularPeriods);
    }
    let end = periods[periods.len() - 1].0;
    if let Eligibility::WindowEndWithinPanel(ref dates) = eligibility {
        if dates.keys().copied().collect::<BTreeSet<_>>() != units {
            return Err(Error::EligibilityKeys);
        }
    }
    let adoption: BTreeMap<_, _> = observations.iter().map(|r| (r.unit, r.adoption)).collect();
    let outcomes: BTreeMap<_, _> = observations
        .iter()
        .map(|r| ((r.unit, r.period), r.outcome))
        .collect();
    let mut retained = BTreeSet::new();
    for unit in &units {
        let eligible = match &eligibility {
            Eligibility::All => true,
            Eligibility::WindowEndWithinPanel(dates) => match dates[unit] {
                None => true,
                Some(p) => p.0.checked_add(last).ok_or(Error::InvalidWindow)? <= end,
            },
        };
        if let Adoption::At(g) = adoption[unit] {
            if periods.binary_search(&g).is_err() {
                return Err(Error::UnknownAdoption(g));
            }
            if g.0.checked_add(last).ok_or(Error::InvalidWindow)? > end {
                continue;
            }
        }
        if eligible {
            retained.insert(*unit);
        }
    }
    let controls: Vec<_> = retained
        .iter()
        .filter(|u| adoption[u] == Adoption::Never)
        .copied()
        .collect();
    let count = usize::try_from(
        last.checked_sub(first)
            .and_then(|n| n.checked_add(1))
            .ok_or(Error::InvalidWindow)?,
    )
    .map_err(|_| Error::InvalidWindow)?;
    let mut output = Vec::new();
    for unit in &retained {
        let Adoption::At(g) = adoption[unit] else {
            continue;
        };
        if controls.is_empty() {
            return Err(Error::MissingControls(g));
        }
        let baseline = Period(g.0 - 1);
        let mut change = 0.0;
        let mut comparison = 0.0;
        for event in first..=last {
            let t = Period(g.0 + event);
            change += outcomes[&(*unit, t)] - outcomes[&(*unit, baseline)];
            comparison += controls
                .iter()
                .map(|u| outcomes[&(*u, t)] - outcomes[&(*u, baseline)])
                .sum::<f64>()
                / controls.len() as f64;
        }
        change /= count as f64;
        comparison /= count as f64;
        let contrast = change - comparison;
        if !change.is_finite() || !comparison.is_finite() || !contrast.is_finite() {
            return Err(Error::Numerical);
        }
        output.push(Contrast {
            unit: *unit,
            group: g,
            baseline,
            dose: doses[unit],
            change,
            comparison_change: comparison,
            contrast,
            comparison_units: controls.len(),
            periods: count,
        });
    }
    if output.is_empty() {
        return Err(Error::NoRetainedTreated);
    }
    Ok(Prepared {
        rows: output,
        excluded_units: units.difference(&retained).copied().collect(),
    })
}
