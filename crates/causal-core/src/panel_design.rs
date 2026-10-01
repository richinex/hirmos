//! Deterministic complete-panel design construction. No cross-team lagging.
use nalgebra::DMatrix;
pub mod bacon;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq)]
pub enum Error {
    Empty,
    Shape,
    NonFinite,
    DuplicateKey,
    Gap,
    Unbalanced,
    NoRows,
    InvalidLag,
    InvalidEvent,
    UnsupportedEvent,
}
pub struct Panel {
    keys: Vec<(u64, i64)>,
    original: Vec<usize>,
    values: DMatrix<f64>,
}
pub struct Design {
    pub matrix: DMatrix<f64>,
    pub keys: Vec<(u64, i64)>,
    pub original_rows: Vec<usize>,
    pub omitted_rows: Vec<usize>,
    pub terms: Vec<String>,
    pub reference_unit: u64,
    pub reference_period: i64,
}

/// Out-of-window treated rows either share the reference or enter endpoint bins.
#[derive(Clone, Copy)]
pub enum EventTails {
    Reference,
    Bin,
}
pub struct EventWindow {
    pub first: i64,
    pub last: i64,
    pub reference: i64,
    pub tails: EventTails,
}
/// Regressors only: the existing within fitter absorbs unit/time effects.
pub struct EventRegressors {
    pub matrix: DMatrix<f64>,
    pub original_rows: Vec<usize>,
    pub keys: Vec<(u64, i64)>,
    pub event_periods: Vec<i64>,
    pub support: Vec<usize>,
}
impl Panel {
    /// Pooled lead/lag indicators across adoption cohorts, retaining every row.
    /// Columns are selected covariates followed by event periods in sorted order.
    /// This is a TWFE design, not a cohort ATT aggregation.
    pub fn pooled_events(
        &self,
        adoption: &BTreeMap<u64, Option<i64>>,
        covariates: &[usize],
        window: EventWindow,
    ) -> Result<EventRegressors, Error> {
        let units: BTreeSet<_> = self.keys.iter().map(|k| k.0).collect();
        if adoption.keys().copied().collect::<BTreeSet<_>>() != units
            || !adoption.values().any(Option::is_some)
            || covariates.iter().any(|c| *c >= self.values.ncols())
            || covariates.iter().collect::<BTreeSet<_>>().len() != covariates.len()
            || window.first >= window.last
            || window.reference < window.first
            || window.reference > window.last
        {
            return Err(Error::InvalidEvent);
        }
        if matches!(window.tails, EventTails::Bin)
            && (window.reference == window.first || window.reference == window.last)
        {
            return Err(Error::InvalidEvent);
        }
        let offsets: Vec<Option<i64>> = self
            .keys
            .iter()
            .map(|(u, t)| {
                adoption[u]
                    .map(|g| t.checked_sub(g).ok_or(Error::InvalidEvent))
                    .transpose()
            })
            .collect::<Result<_, _>>()?;
        if !offsets.contains(&Some(window.reference)) {
            return Err(Error::UnsupportedEvent);
        }
        // Bound allocation by actual observations, not an arbitrary user range.
        let observed: BTreeSet<i64> = offsets
            .iter()
            .flatten()
            .copied()
            .map(|e| match window.tails {
                EventTails::Reference => e,
                EventTails::Bin => e.clamp(window.first, window.last),
            })
            .filter(|e| *e >= window.first && *e <= window.last && *e != window.reference)
            .collect();
        let expected = window
            .last
            .checked_sub(window.first)
            .ok_or(Error::InvalidEvent)?;
        if observed.len() as u128 != expected as u128 {
            return Err(Error::UnsupportedEvent);
        }
        let events: Vec<_> = observed.into_iter().collect();
        let mut support = vec![0; events.len()];
        let matrix = DMatrix::from_fn(self.keys.len(), covariates.len() + events.len(), |i, j| {
            if j < covariates.len() {
                return self.values[(i, covariates[j])];
            }
            let event = offsets[i].map(|e| match window.tails {
                EventTails::Reference => e,
                EventTails::Bin => e.clamp(window.first, window.last),
            });
            let yes = event == Some(events[j - covariates.len()]);
            if yes {
                support[j - covariates.len()] += 1;
            }
            f64::from(yes)
        });
        Ok(EventRegressors {
            matrix,
            original_rows: self.original.clone(),
            keys: self.keys.clone(),
            event_periods: events,
            support,
        })
    }
    pub fn new(keys: &[(u64, i64)], values: DMatrix<f64>) -> Result<Self, Error> {
        if keys.is_empty() {
            return Err(Error::Empty);
        }
        if keys.len() != values.nrows() {
            return Err(Error::Shape);
        }
        if values.iter().any(|v| !v.is_finite()) {
            return Err(Error::NonFinite);
        }
        let mut order: Vec<usize> = (0..keys.len()).collect();
        order.sort_by_key(|i| keys[*i]);
        let keys: Vec<_> = order.iter().map(|i| keys[*i]).collect();
        if keys.windows(2).any(|v| v[0] == v[1]) {
            return Err(Error::DuplicateKey);
        }
        let mut by_unit: BTreeMap<u64, Vec<i64>> = BTreeMap::new();
        for &(u, t) in &keys {
            by_unit.entry(u).or_default().push(t);
        }
        let first = by_unit.values().next().ok_or(Error::Empty)?;
        if first.windows(2).any(|v| v[1].checked_sub(v[0]) != Some(1)) {
            return Err(Error::Gap);
        }
        if by_unit.values().any(|t| t != first) {
            return Err(Error::Unbalanced);
        }
        Ok(Self {
            keys,
            values: DMatrix::from_fn(order.len(), values.ncols(), |i, j| values[(order[i], j)]),
            original: order,
        })
    }
    /// Numeric columns (source column, lag, name), followed by reference-coded unit/week effects.
    pub fn distributed_lags(&self, columns: &[(usize, usize, String)]) -> Result<Design, Error> {
        if columns.is_empty() {
            return Err(Error::Empty);
        }
        if columns
            .iter()
            .any(|(c, l, _)| *c >= self.values.ncols() || *l > i64::MAX as usize)
        {
            return Err(Error::InvalidLag);
        }
        let max_lag = columns.iter().map(|(_, l, _)| *l).max().unwrap();
        let lookup: BTreeMap<_, _> = self
            .keys
            .iter()
            .copied()
            .enumerate()
            .map(|(i, k)| (k, i))
            .collect();
        let first = self.keys[0].1;
        let keep: Vec<usize> = self
            .keys
            .iter()
            .enumerate()
            .filter_map(|(i, (_, t))| {
                ((*t as i128 - first as i128) >= max_lag as i128).then_some(i)
            })
            .collect();
        if keep.is_empty() {
            return Err(Error::NoRows);
        }
        let mut rows = Vec::new();
        for &i in &keep {
            let (u, t) = self.keys[i];
            let mut row = Vec::new();
            for (c, l, _) in columns {
                let key = (u, t.checked_sub(*l as i64).ok_or(Error::InvalidLag)?);
                row.push(self.values[(*lookup.get(&key).ok_or(Error::Gap)?, *c)]);
            }
            rows.push(row);
        }
        self.with_effects(
            &keep,
            &rows,
            columns.iter().map(|(_, _, n)| n.clone()).collect(),
        )
    }
    /// One adoption cohort and never-adopting controls; omit event -1.
    /// All observed event categories are retained, so tails are never silently pooled into the reference.
    pub fn cohort_events(
        &self,
        adoption: &BTreeMap<u64, Option<i64>>,
        cohort: i64,
        covariates: &[usize],
    ) -> Result<Design, Error> {
        let units: BTreeSet<_> = self.keys.iter().map(|(u, _)| *u).collect();
        if adoption.keys().copied().collect::<BTreeSet<_>>() != units
            || covariates.iter().any(|c| *c >= self.values.ncols())
        {
            return Err(Error::InvalidEvent);
        }
        if !adoption.values().any(|a| *a == Some(cohort)) || !adoption.values().any(Option::is_none)
        {
            return Err(Error::InvalidEvent);
        }
        let keep: Vec<usize> = self
            .keys
            .iter()
            .enumerate()
            .filter_map(|(i, (u, _))| {
                (adoption[u].is_none() || adoption[u] == Some(cohort)).then_some(i)
            })
            .collect();
        let events: BTreeSet<i64> = keep
            .iter()
            .filter(|i| adoption[&self.keys[**i].0] == Some(cohort))
            .map(|i| {
                self.keys[*i]
                    .1
                    .checked_sub(cohort)
                    .ok_or(Error::InvalidEvent)
            })
            .collect::<Result<_, _>>()?;
        if !events.contains(&-1) || !events.contains(&0) {
            return Err(Error::UnsupportedEvent);
        }
        let events: Vec<i64> = events.into_iter().filter(|e| *e != -1).collect();
        let rows: Vec<Vec<f64>> = keep
            .iter()
            .map(|i| {
                let (u, t) = self.keys[*i];
                let mut row: Vec<f64> = events
                    .iter()
                    .map(|e| {
                        f64::from(adoption[&u] == Some(cohort) && t.checked_sub(cohort) == Some(*e))
                    })
                    .collect();
                row.extend(covariates.iter().map(|c| self.values[(*i, *c)]));
                row
            })
            .collect();
        let mut names: Vec<String> = events.iter().map(|e| format!("event[{e}]")).collect();
        names.extend(covariates.iter().map(|c| format!("x[{c}]")));
        self.with_effects(&keep, &rows, names)
    }
    /// Literal finite-window specification: keep all rows, but only include the
    /// requested event indicators. Out-of-window treated rows therefore share
    /// the omitted category with event -1; this is not a -1-only baseline.
    pub fn cohort_events_in_window(
        &self,
        adoption: &BTreeMap<u64, Option<i64>>,
        cohort: i64,
        covariates: &[usize],
        first: i64,
        last: i64,
    ) -> Result<Design, Error> {
        if first > -1 || last < 0 {
            return Err(Error::InvalidEvent);
        }
        let mut design = self.cohort_events(adoption, cohort, covariates)?;
        let events: Vec<i64> = design
            .keys
            .iter()
            .filter(|(u, _)| adoption[u] == Some(cohort))
            .map(|(_, t)| t.checked_sub(cohort).ok_or(Error::InvalidEvent))
            .collect::<Result<BTreeSet<_>, _>>()?
            .into_iter()
            .filter(|e| *e != -1)
            .collect();
        let selected: Vec<usize> = (0..design.terms.len())
            .filter(|i| {
                *i == 0 || *i > events.len() || (events[*i - 1] >= first && events[*i - 1] <= last)
            })
            .collect();
        design.matrix = DMatrix::from_fn(design.matrix.nrows(), selected.len(), |i, j| {
            design.matrix[(i, selected[j])]
        });
        design.terms = selected.iter().map(|i| design.terms[*i].clone()).collect();
        Ok(design)
    }
    /// Cohort-specific treated-by-post summary, retaining the same controls and rows.
    pub fn cohort_summary(
        &self,
        adoption: &BTreeMap<u64, Option<i64>>,
        cohort: i64,
        covariates: &[usize],
    ) -> Result<Design, Error> {
        let events = self.cohort_events(adoption, cohort, covariates)?;
        let selected: BTreeSet<_> = events.keys.into_iter().collect();
        let keep: Vec<_> = self
            .keys
            .iter()
            .enumerate()
            .filter_map(|(i, k)| selected.contains(k).then_some(i))
            .collect();
        let rows: Vec<Vec<f64>> = keep
            .iter()
            .map(|i| {
                let (unit, time) = self.keys[*i];
                let mut row = vec![f64::from(adoption[&unit] == Some(cohort) && time >= cohort)];
                row.extend(covariates.iter().map(|c| self.values[(*i, *c)]));
                row
            })
            .collect();
        let mut names = vec!["treated_post".to_owned()];
        names.extend(covariates.iter().map(|c| format!("x[{c}]")));
        self.with_effects(&keep, &rows, names)
    }
    fn with_effects(
        &self,
        keep: &[usize],
        rows: &[Vec<f64>],
        names: Vec<String>,
    ) -> Result<Design, Error> {
        let units: Vec<_> = keep
            .iter()
            .map(|i| self.keys[*i].0)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let times: Vec<_> = keep
            .iter()
            .map(|i| self.keys[*i].1)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        if units.is_empty() || times.is_empty() {
            return Err(Error::NoRows);
        }
        let mut terms = vec!["Intercept".to_owned()];
        terms.extend(names);
        let numeric = terms.len() - 1;
        terms.extend(units.iter().skip(1).map(|u| format!("unit[{u}]")));
        terms.extend(times.iter().skip(1).map(|t| format!("period[{t}]")));
        let matrix = DMatrix::from_fn(keep.len(), terms.len(), |i, j| {
            if j == 0 {
                1.0
            } else if j <= numeric {
                rows[i][j - 1]
            } else if j < numeric + units.len() {
                f64::from(self.keys[keep[i]].0 == units[j - numeric])
            } else {
                f64::from(self.keys[keep[i]].1 == times[j - numeric - units.len() + 1])
            }
        });
        let selected: BTreeSet<_> = keep.iter().copied().collect();
        Ok(Design {
            matrix,
            keys: keep.iter().map(|i| self.keys[*i]).collect(),
            original_rows: keep.iter().map(|i| self.original[*i]).collect(),
            omitted_rows: (0..self.keys.len())
                .filter(|i| !selected.contains(i))
                .map(|i| self.original[i])
                .collect(),
            terms,
            reference_unit: units[0],
            reference_period: times[0],
        })
    }
}
