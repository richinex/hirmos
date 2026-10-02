//! Cohort interactions and support-weighted contrasts from fixest 0.14.2.
//! Fitting is deliberately separate: this module does not replace the shared
//! fixed-effects regression, collinearity policy or uncertainty engine.
use nalgebra::{DMatrix, DVector};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cell {
    pub event: i64,
    pub cohort: i64,
}
#[derive(Debug, PartialEq)]
pub enum Error {
    Empty,
    Shape,
    NoTreatedCohort,
    NoReferencePeriod,
    Overflow,
    NonFinite,
    InvalidSupport,
    InvalidCovariance,
}
pub struct Design {
    pub cells: Vec<Cell>,
    pub matrix: DMatrix<f64>,
    pub retained_rows: Vec<usize>,
    pub removed_rows: Vec<usize>,
}

/// Calendar-period interface, with explicit reference periods and cohorts.
/// Matches fixest's calendar convention: cohorts absent from the observed
/// calendar are comparison cohorts, even if their numeric value is earlier.
/// The caller must record that convention, not infer it from row positions.
pub fn calendar_design(
    cohorts: &[i64],
    periods: &[i64],
    reference_periods: &[i64],
    reference_cohorts: &[i64],
) -> Result<Design, Error> {
    if cohorts.is_empty() {
        return Err(Error::Empty);
    }
    if cohorts.len() != periods.len() {
        return Err(Error::Shape);
    }
    if reference_periods.is_empty() {
        return Err(Error::NoReferencePeriod);
    }
    let calendar: BTreeSet<_> = periods.iter().copied().collect();
    if !cohorts.iter().any(|g| calendar.contains(g)) {
        return Err(Error::NoTreatedCohort);
    }
    let offsets: Vec<_> = cohorts
        .iter()
        .zip(periods)
        .map(|(g, t)| {
            if calendar.contains(g) {
                t.checked_sub(*g).ok_or(Error::Overflow)
            } else {
                Ok(-1)
            }
        })
        .collect::<Result<_, _>>()?;
    let mut ranges = BTreeMap::<i64, (i64, i64)>::new();
    for (&g, &e) in cohorts.iter().zip(&offsets) {
        let r = ranges.entry(g).or_insert((e, e));
        r.0 = r.0.min(e);
        r.1 = r.1.max(e);
    }
    let removed_rows: Vec<_> = (0..cohorts.len())
        .filter(|&i| ranges[&cohorts[i]].0 >= 0)
        .collect();
    let retained_rows: Vec<_> = (0..cohorts.len())
        .filter(|&i| ranges[&cohorts[i]].0 < 0)
        .collect();
    let active = |i: usize| {
        ranges[&cohorts[i]].1 >= 0
            && !reference_cohorts.contains(&cohorts[i])
            && !reference_periods.contains(&offsets[i])
    };
    let cells: Vec<_> = retained_rows
        .iter()
        .copied()
        .filter(|&i| active(i))
        .map(|i| Cell {
            event: offsets[i],
            cohort: cohorts[i],
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let matrix = DMatrix::from_fn(retained_rows.len(), cells.len(), |r, c| {
        let i = retained_rows[r];
        f64::from(
            active(i)
                && cells[c]
                    == Cell {
                        event: offsets[i],
                        cohort: cohorts[i],
                    },
        )
    });
    Ok(Design {
        cells,
        matrix,
        retained_rows,
        removed_rows,
    })
}

#[derive(Clone, Copy)]
pub enum Target {
    Event(i64),
    Cohort(i64),
    OverallAtt,
}
pub struct Contrast {
    pub weights: Vec<f64>,
    pub estimate: f64,
    pub variance: f64,
}
/// Operates on retained fitted cells, after the shared fitter's rank decisions.
/// Support is the sum of observation weights on each interaction, not the
/// number of periods or an equal-cohort average. Cross-cell covariance matters.
pub fn aggregate(
    cells: &[Cell],
    coefficients: &[f64],
    covariance: &DMatrix<f64>,
    support: &[f64],
    target: Target,
) -> Result<Contrast, Error> {
    let k = cells.len();
    if k == 0 {
        return Err(Error::Empty);
    }
    if coefficients.len() != k || support.len() != k || covariance.shape() != (k, k) {
        return Err(Error::Shape);
    }
    if coefficients
        .iter()
        .chain(support)
        .chain(covariance.iter())
        .any(|v| !v.is_finite())
    {
        return Err(Error::NonFinite);
    }
    if support.iter().any(|v| *v <= 0.) {
        return Err(Error::InvalidSupport);
    }
    for i in 0..k {
        for j in 0..k {
            if (covariance[(i, j)] - covariance[(j, i)]).abs()
                > 1e-10 * (1. + covariance[(i, j)].abs())
            {
                return Err(Error::InvalidCovariance);
            }
        }
    }
    let included = |c: &Cell| match target {
        Target::Event(e) => c.event == e,
        Target::Cohort(g) => c.cohort == g && c.event >= 0,
        Target::OverallAtt => c.event >= 0,
    };
    let total: f64 = cells
        .iter()
        .zip(support)
        .filter(|(c, _)| included(c))
        .map(|(_, s)| s)
        .sum();
    if total <= 0. || !total.is_finite() {
        return Err(Error::InvalidSupport);
    }
    let weights: Vec<_> = cells
        .iter()
        .zip(support)
        .map(|(c, s)| if included(c) { s / total } else { 0. })
        .collect();
    let w = DVector::from_column_slice(&weights);
    let estimate = w.dot(&DVector::from_column_slice(coefficients));
    let variance = w.dot(&(covariance * &w));
    if !estimate.is_finite() || !variance.is_finite() {
        return Err(Error::NonFinite);
    }
    if variance < 0. {
        return Err(Error::InvalidCovariance);
    }
    Ok(Contrast {
        weights,
        estimate,
        variance,
    })
}
