//! Balanced-panel cohort-time DiD with optional covariate adjustment.
//! Reference: did 2.5.1 and DRDID 1.3.0 (Callaway, Sant'Anna and Zhao).
//! Anticipation and bootstrap inference are explicit parts of the specification.

use std::collections::{BTreeMap, BTreeSet};

pub mod bootstrap;
pub mod dr;
pub mod inference;
pub mod preparation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Period(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adoption {
    Never,
    At(Period),
}

#[derive(Debug, Clone, Copy)]
pub struct Row {
    pub unit: Unit,
    pub period: Period,
    pub adoption: Adoption,
    pub outcome: f64,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    TooFewPeriods,
    NonFinite { unit: Unit, period: Period },
    Duplicate { unit: Unit, period: Period },
    CohortChanged { unit: Unit },
    Unbalanced { unit: Unit },
    NoPreTreatment { unit: Unit },
    UnknownPeriod(Period),
    MissingCohort(Period),
    NoBaseline,
    MissingControls,
    TrimmedControls,
    NoPostTreatment,
    CovariateShape,
    Regression(dr::Error),
    NumericalOverflow,
    InvalidWeights,
    InvalidWindow,
    SmallControlGroup { count: usize, required: usize },
}

#[derive(Debug)]
struct Series {
    unit: Unit,
    adoption: Adoption,
    values: Vec<f64>,
}

/// Only construction validates and orders rows. Estimation cannot bypass it.
#[derive(Debug)]
pub struct Panel {
    periods: Vec<Period>,
    series: Vec<Series>,
    weights: Vec<f64>,
}

impl Panel {
    pub fn new(rows: &[Row]) -> Result<Self, Error> {
        let periods: Vec<_> = rows
            .iter()
            .map(|r| r.period)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        if periods.len() < 2 {
            return Err(Error::TooFewPeriods);
        }
        let mut units: BTreeMap<Unit, (Adoption, BTreeMap<Period, f64>)> = BTreeMap::new();
        for row in rows {
            if !row.outcome.is_finite() {
                return Err(Error::NonFinite {
                    unit: row.unit,
                    period: row.period,
                });
            }
            let entry = units
                .entry(row.unit)
                .or_insert_with(|| (row.adoption, BTreeMap::new()));
            if entry.0 != row.adoption {
                return Err(Error::CohortChanged { unit: row.unit });
            }
            if entry.1.insert(row.period, row.outcome).is_some() {
                return Err(Error::Duplicate {
                    unit: row.unit,
                    period: row.period,
                });
            }
        }
        let mut series = Vec::with_capacity(units.len());
        for (unit, (adoption, values)) in units {
            if values.len() != periods.len() {
                return Err(Error::Unbalanced { unit });
            }
            if matches!(adoption, Adoption::At(g) if g <= periods[0]) {
                return Err(Error::NoPreTreatment { unit });
            }
            series.push(Series {
                unit,
                adoption,
                values: values.into_values().collect(),
            });
        }
        let weights = vec![1.0; series.len()];
        Ok(Self {
            periods,
            series,
            weights,
        })
    }

    pub fn units(&self) -> impl Iterator<Item = Unit> + '_ {
        self.series.iter().map(|s| s.unit)
    }

    /// Time-invariant sampling weights keyed to units, not input row positions.
    pub fn with_weights(mut self, entries: Vec<(Unit, f64)>) -> Result<Self, Error> {
        let mut weights = BTreeMap::new();
        for (unit, value) in entries {
            if !value.is_finite() || value < 0.0 || weights.insert(unit, value).is_some() {
                return Err(Error::InvalidWeights);
            }
        }
        if weights.len() != self.series.len() {
            return Err(Error::InvalidWeights);
        }
        let values = self
            .units()
            .map(|u| weights.get(&u).copied().ok_or(Error::InvalidWeights))
            .collect::<Result<Vec<_>, _>>()?;
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        if !mean.is_finite() || mean <= 0.0 {
            return Err(Error::InvalidWeights);
        }
        self.weights = values.iter().map(|w| w / mean).collect();
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Anticipation(pub u32);

/// Dynamic aggregation only. Calendar/group summaries retain their own meaning.
#[derive(Debug, Clone, Copy, Default)]
pub struct EventWindow {
    first: Option<i64>,
    last: Option<i64>,
    balance: Option<u32>,
}

impl EventWindow {
    pub fn bounds(&self) -> (Option<i64>, Option<i64>, Option<u32>) {
        (self.first, self.last, self.balance)
    }
    pub fn new(first: Option<i64>, last: Option<i64>, balance: Option<u32>) -> Result<Self, Error> {
        if matches!((first, last), (Some(a), Some(b)) if a > b) {
            return Err(Error::InvalidWindow);
        }
        Ok(Self {
            first,
            last,
            balance,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Controls {
    NeverTreated,
    NotYetTreated,
}

#[derive(Debug, Clone, Copy)]
pub enum Baseline {
    Varying,
    Universal,
}

#[derive(Debug, PartialEq)]
pub struct Estimate {
    pub att: f64,
    pub se: f64,
    /// Full-panel unit order, including zeros for units outside this comparison.
    pub influence: Vec<f64>,
    pub treated: usize,
    pub controls: usize,
    pub fit_status: Option<dr::FitStatus>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdjustmentMethod {
    DoublyRobust,
    OutcomeRegression,
    InverseProbability,
}

pub struct Adjusted<'a> {
    panel: &'a Panel,
    values: BTreeMap<(Unit, Period), Vec<f64>>,
    columns: usize,
    method: AdjustmentMethod,
}

impl<'a> Adjusted<'a> {
    /// Covariates exclude the intercept; every panel key must occur exactly once.
    pub fn new(panel: &'a Panel, rows: Vec<(Unit, Period, Vec<f64>)>) -> Result<Self, Error> {
        Self::new_with_method(panel, rows, AdjustmentMethod::DoublyRobust)
    }
    pub fn new_with_method(panel: &'a Panel, rows: Vec<(Unit, Period, Vec<f64>)>, method: AdjustmentMethod) -> Result<Self, Error> {
        let columns = rows
            .first()
            .map(|r| r.2.len())
            .ok_or(Error::CovariateShape)?;
        if columns == 0 || rows.len() != panel.series.len() * panel.periods.len() {
            return Err(Error::CovariateShape);
        }
        let mut values = BTreeMap::new();
        for (u, t, x) in rows {
            if x.len() != columns
                || x.iter().any(|v| !v.is_finite())
                || values.insert((u, t), x).is_some()
            {
                return Err(Error::CovariateShape);
            }
        }
        if panel.series.iter().any(|s| {
            panel
                .periods
                .iter()
                .any(|t| !values.contains_key(&(s.unit, *t)))
        }) {
            return Err(Error::CovariateShape);
        }
        Ok(Self {
            panel,
            values,
            columns,
            method,
        })
    }
    pub fn cell(&self, g: Period, t: Period, c: Controls, b: Baseline) -> Result<Cell, Error> {
        cell_impl(self.panel, g, t, c, b, Some(self), Anticipation(0))
    }
    pub fn dynamic(&self, c: Controls, b: Baseline) -> Result<Dynamic, Error> {
        dynamic_impl(
            self.panel,
            c,
            b,
            Some(self),
            Anticipation(0),
            EventWindow::default(),
        )
    }
    pub fn with_anticipation(
        &self,
        c: Controls,
        b: Baseline,
        a: Anticipation,
    ) -> Result<Dynamic, Error> {
        dynamic_impl(self.panel, c, b, Some(self), a, EventWindow::default())
    }
    pub fn event_study(
        &self,
        c: Controls,
        b: Baseline,
        a: Anticipation,
        window: EventWindow,
    ) -> Result<Dynamic, Error> {
        dynamic_impl(self.panel, c, b, Some(self), a, window)
    }
}

#[derive(Debug, PartialEq)]
pub enum Cell {
    /// A normalized reference period is not an estimated zero treatment effect.
    Reference,
    Estimated(Estimate),
}

/// Unadjusted specialization of DRDID's normalized panel score. With only an
/// intercept the nuisance-estimation corrections cancel, leaving centered
/// within-group changes. did rescales those scores to the full-panel size.
pub fn cell(
    panel: &Panel,
    cohort: Period,
    time: Period,
    controls: Controls,
    baseline: Baseline,
) -> Result<Cell, Error> {
    cell_impl(
        panel,
        cohort,
        time,
        controls,
        baseline,
        None,
        Anticipation(0),
    )
}

fn cell_impl(
    panel: &Panel,
    cohort: Period,
    time: Period,
    controls: Controls,
    baseline: Baseline,
    adjustment: Option<&Adjusted<'_>>,
    anticipation: Anticipation,
) -> Result<Cell, Error> {
    let target = panel
        .periods
        .binary_search(&time)
        .map_err(|_| Error::UnknownPeriod(time))?;
    if !panel
        .series
        .iter()
        .any(|s| s.adoption == Adoption::At(cohort))
    {
        return Err(Error::MissingCohort(cohort));
    }
    let reference = panel
        .periods
        .iter()
        .rposition(|t| i128::from(t.0) + i128::from(anticipation.0) < i128::from(cohort.0))
        .ok_or(Error::NoBaseline)?;
    let base = match baseline {
        Baseline::Universal => reference,
        Baseline::Varying if time < cohort => target.checked_sub(1).ok_or(Error::NoBaseline)?,
        Baseline::Varying => reference,
    };
    if target == base {
        return Ok(Cell::Reference);
    }
    let cutoff = time.max(panel.periods[base]);
    let mut treated = Vec::new();
    let mut comparison = Vec::new();
    for (i, s) in panel.series.iter().enumerate() {
        let is_treated = s.adoption == Adoption::At(cohort);
        let is_control = match (controls, s.adoption) {
            (_, Adoption::Never) => true,
            (Controls::NotYetTreated, Adoption::At(g)) => {
                i128::from(g.0) > i128::from(cutoff.0) + i128::from(anticipation.0) && !is_treated
            }
            (Controls::NeverTreated, Adoption::At(_)) => false,
        };
        if is_treated || is_control {
            let difference = s.values[target] - s.values[base];
            if !difference.is_finite() {
                return Err(Error::NumericalOverflow);
            }
            if is_treated {
                treated.push((i, difference));
            } else {
                comparison.push((i, difference));
            }
        }
    }
    if comparison.is_empty() {
        return Err(Error::MissingControls);
    }
    if let Some(adjusted) = adjustment {
        let selected: Vec<_> = treated.iter().chain(&comparison).copied().collect();
        // Universal placebo comparisons take covariates at the earlier period.
        let covariate_time = panel.periods[base.min(target)];
        let x = nalgebra::DMatrix::from_fn(selected.len(), adjusted.columns + 1, |i, j| {
            if j == 0 {
                1.0
            } else {
                adjusted.values[&(panel.series[selected[i].0].unit, covariate_time)][j - 1]
            }
        });
        let sample = dr::Sample::new(
            x,
            selected.iter().map(|(_, v)| *v).collect(),
            (0..selected.len()).map(|i| i < treated.len()).collect(),
            selected.iter().map(|(i, _)| panel.weights[*i]).collect(),
        )
        .map_err(Error::Regression)?;
        dr::panel_guard_for(&sample, adjusted.method).map_err(Error::Regression)?;
        let (score, fit_status) = match adjusted.method {
            AdjustmentMethod::DoublyRobust => {
                let fit = dr::fit(&sample, 0.995).map_err(Error::Regression)?;
                (fit.score, Some(fit.status))
            }
            AdjustmentMethod::InverseProbability => {
                let fit = dr::fit_ipw(&sample, 0.995, dr::IpwNormalization::Hajek)
                    .map_err(Error::Regression)?;
                (fit.score, Some(fit.status))
            }
            AdjustmentMethod::OutcomeRegression => {
                let fit = dr::fit_outcome_regression(&sample).map_err(Error::Regression)?;
                (fit.score, None)
            }
        };
        let mut influence = vec![0.0; panel.series.len()];
        for (i, (unit, _)) in selected.iter().enumerate() {
            influence[*unit] =
                panel.series.len() as f64 / selected.len() as f64 * score.influence[i];
        }
        let se = influence.iter().map(|v| v * v).sum::<f64>().sqrt() / panel.series.len() as f64;
        return Ok(Cell::Estimated(Estimate {
            att: score.att,
            se,
            influence,
            treated: treated.len(),
            controls: comparison.len(),
            fit_status,
        }));
    }
    // DRDID's default trim.level excludes controls at propensity >= .995.
    let weight_t: f64 = treated.iter().map(|(i, _)| panel.weights[*i]).sum();
    let weight_c: f64 = comparison.iter().map(|(i, _)| panel.weights[*i]).sum();
    if weight_t <= 0.0 || weight_c <= 0.0 {
        return Err(Error::InvalidWeights);
    }
    if weight_t / (weight_t + weight_c) >= 0.995 {
        return Err(Error::TrimmedControls);
    }
    let mean = |group: &[(usize, f64)], weight: f64| {
        group
            .iter()
            .map(|(i, d)| panel.weights[*i] * d)
            .sum::<f64>()
            / weight
    };
    let mean_t = mean(&treated, weight_t);
    let mean_c = mean(&comparison, weight_c);
    let n = panel.series.len() as f64;
    let mut influence = vec![0.0; panel.series.len()];
    for &(i, d) in &treated {
        influence[i] = n * panel.weights[i] / weight_t * (d - mean_t);
    }
    for &(i, d) in &comparison {
        influence[i] = -n * panel.weights[i] / weight_c * (d - mean_c);
    }
    let att = mean_t - mean_c;
    let se = influence.iter().map(|v| v * v).sum::<f64>().sqrt() / n;
    if !att.is_finite() || !se.is_finite() {
        return Err(Error::NumericalOverflow);
    }
    Ok(Cell::Estimated(Estimate {
        att,
        se,
        influence,
        treated: treated.len(),
        controls: comparison.len(),
        fit_status: None,
    }))
}

#[derive(Debug, PartialEq)]
pub struct Summary {
    pub att: f64,
    pub se: f64,
    pub influence: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub struct Dynamic {
    /// Calendar-time cohort facets retain normalized references as references.
    pub cells: BTreeMap<(Period, Period), Cell>,
    pub events: BTreeMap<i64, Summary>,
    pub event_support: BTreeMap<i64, EventSupport>,
    /// Equal-weight mean of supported event-time effects at and after adoption.
    pub overall: Summary,
    pub fits: Vec<(Period, Period, dr::FitStatus)>,
    pub by_group: BTreeMap<Period, Summary>,
    pub group_overall: Summary,
    pub by_calendar: BTreeMap<Period, Summary>,
    pub calendar_overall: Summary,
    pub simple: Summary,
}

#[derive(Debug, PartialEq)]
pub struct EventSupport {
    /// Cohorts contributing at this horizon, not all cohorts in the dataset.
    pub cohorts: Vec<Period>,
    pub treated_units: usize,
    /// Cohorts whose cell is a normalized baseline, not an estimated zero.
    pub reference_cohorts: Vec<Period>,
}

fn summary(att: f64, influence: Vec<f64>) -> Result<Summary, Error> {
    let se = influence.iter().map(|v| v * v).sum::<f64>().sqrt() / influence.len() as f64;
    if !att.is_finite() || !se.is_finite() {
        return Err(Error::NumericalOverflow);
    }
    Ok(Summary { att, se, influence })
}

fn weighted(panel: &Panel, cells: &[(Period, usize, &Summary)]) -> Result<Summary, Error> {
    if cells.is_empty() {
        return Err(Error::NoPostTreatment);
    }
    let mut masses = BTreeMap::<Period, f64>::new();
    for (s, w) in panel.series.iter().zip(&panel.weights) {
        if let Adoption::At(g) = s.adoption {
            *masses.entry(g).or_default() += w;
        }
    }
    let mass = |g: Period| masses.get(&g).copied().unwrap_or(0.0);
    let count = cells.iter().map(|(g, _, _)| mass(*g)).sum::<f64>();
    if count <= 0.0 {
        return Err(Error::InvalidWeights);
    }
    let att = cells.iter().map(|(g, _, e)| mass(*g) / count * e.att).sum();
    let mut influence = vec![0.0; panel.series.len()];
    for (g, _, e) in cells {
        for (i, s) in panel.series.iter().enumerate() {
            influence[i] += mass(*g) / count * e.influence[i];
            if s.adoption == Adoption::At(*g) {
                influence[i] +=
                    panel.series.len() as f64 * panel.weights[i] / count * (e.att - att);
            }
        }
    }
    summary(att, influence)
}

fn average(values: &[&Summary]) -> Result<Summary, Error> {
    let first = values.first().ok_or(Error::NoPostTreatment)?;
    let att = values.iter().map(|s| s.att).sum::<f64>() / values.len() as f64;
    let mut influence = vec![0.0; first.influence.len()];
    for s in values {
        for (i, v) in influence.iter_mut().enumerate() {
            *v += s.influence[i] / values.len() as f64;
        }
    }
    summary(att, influence)
}

/// Default dynamic aggregation, without event-window trimming or balance_e.
/// Missing cells are errors, not silently dropped estimates (R's na.rm=FALSE).
pub fn dynamic(panel: &Panel, controls: Controls, baseline: Baseline) -> Result<Dynamic, Error> {
    dynamic_impl(
        panel,
        controls,
        baseline,
        None,
        Anticipation(0),
        EventWindow::default(),
    )
}

pub fn with_anticipation(
    panel: &Panel,
    controls: Controls,
    baseline: Baseline,
    anticipation: Anticipation,
) -> Result<Dynamic, Error> {
    dynamic_impl(
        panel,
        controls,
        baseline,
        None,
        anticipation,
        EventWindow::default(),
    )
}

pub fn event_study(
    panel: &Panel,
    controls: Controls,
    baseline: Baseline,
    anticipation: Anticipation,
    window: EventWindow,
) -> Result<Dynamic, Error> {
    dynamic_impl(panel, controls, baseline, None, anticipation, window)
}

fn dynamic_impl(
    panel: &Panel,
    controls: Controls,
    baseline: Baseline,
    adjustment: Option<&Adjusted<'_>>,
    anticipation: Anticipation,
    window: EventWindow,
) -> Result<Dynamic, Error> {
    let mut fits = Vec::new();
    let mut fitted_cells = BTreeMap::new();
    let mut event_support = BTreeMap::<i64, EventSupport>::new();
    let mut cohorts = BTreeMap::<Period, usize>::new();
    for s in &panel.series {
        if let Adoption::At(g) = s.adoption {
            if g <= *panel.periods.last().unwrap() {
                *cohorts.entry(g).or_default() += 1;
            }
        }
    }
    let mut by_event = BTreeMap::<i64, Vec<(Period, usize, Summary)>>::new();
    for (&g, &size) in &cohorts {
        for (index, &t) in panel.periods.iter().enumerate() {
            if index == 0 && matches!(baseline, Baseline::Varying) {
                continue;
            }
            let cell = cell_impl(panel, g, t, controls, baseline, adjustment, anticipation)?;
            let estimate = match &cell {
                Cell::Reference => summary(0.0, vec![0.0; panel.series.len()])?,
                Cell::Estimated(e) => {
                    if let Some(status) = &e.fit_status {
                        fits.push((g, t, status.clone()));
                    }
                    summary(e.att, e.influence.clone())?
                }
            };
            let event = t.0.checked_sub(g.0).ok_or(Error::NumericalOverflow)?;
            let support = event_support.entry(event).or_insert_with(|| EventSupport {
                cohorts: Vec::new(),
                treated_units: 0,
                reference_cohorts: Vec::new(),
            });
            support.cohorts.push(g);
            support.treated_units += size;
            if matches!(cell, Cell::Reference) {
                support.reference_cohorts.push(g);
            }
            fitted_cells.insert((g, t), cell);
            by_event.entry(event).or_default().push((g, size, estimate));
        }
    }
    let post_cells: Vec<_> = by_event
        .range(0..)
        .flat_map(|(_, cells)| cells.iter().map(|(g, n, s)| (*g, *n, s)))
        .collect();
    let simple = weighted(panel, &post_cells)?;
    let mut group_cells: BTreeMap<Period, Vec<(Period, usize, &Summary)>> = BTreeMap::new();
    let mut calendar_cells: BTreeMap<Period, Vec<(Period, usize, &Summary)>> = BTreeMap::new();
    for (event, cells) in by_event.range(0..) {
        for (g, n, s) in cells {
            group_cells.entry(*g).or_default().push((*g, *n, s));
            let t = Period(g.0.checked_add(*event).ok_or(Error::NumericalOverflow)?);
            calendar_cells.entry(t).or_default().push((*g, *n, s));
        }
    }
    let by_group: BTreeMap<_, _> = group_cells
        .iter()
        .map(|(g, cells)| Ok((*g, weighted(panel, cells)?)))
        .collect::<Result<_, Error>>()?;
    let group_overall = weighted(
        panel,
        &by_group
            .iter()
            .map(|(g, s)| (*g, cohorts[g], s))
            .collect::<Vec<_>>(),
    )?;
    let by_calendar: BTreeMap<_, _> = calendar_cells
        .iter()
        .map(|(t, cells)| Ok((*t, weighted(panel, cells)?)))
        .collect::<Result<_, Error>>()?;
    let calendar_overall = average(&by_calendar.values().collect::<Vec<_>>())?;
    let mut events = BTreeMap::new();
    for (event, mut cells) in by_event {
        if window.first.is_some_and(|first| event < first)
            || window.last.is_some_and(|last| event > last)
        {
            continue;
        }
        if let Some(balance) = window.balance {
            let end = i128::from(panel.periods.last().unwrap().0);
            let first = i128::from(panel.periods[0].0);
            if i128::from(event) > i128::from(balance)
                || i128::from(event) < i128::from(balance) - end + first
            {
                continue;
            }
            cells.retain(|(g, _, _)| end - i128::from(g.0) >= i128::from(balance));
            let retained: BTreeSet<_> = cells.iter().map(|(g, _, _)| *g).collect();
            let support = event_support.get_mut(&event).unwrap();
            support.cohorts.retain(|g| retained.contains(g));
            support.reference_cohorts.retain(|g| retained.contains(g));
            support.treated_units = cells.iter().map(|(_, n, _)| n).sum();
        }
        if cells.is_empty() {
            continue;
        }
        events.insert(
            event,
            weighted(
                panel,
                &cells
                    .iter()
                    .map(|(g, n, s)| (*g, *n, s))
                    .collect::<Vec<_>>(),
            )?,
        );
    }
    event_support.retain(|event, _| events.contains_key(event));
    if events.is_empty() {
        return Err(Error::InvalidWindow);
    }
    let post: Vec<_> = events.range(0..).map(|(_, e)| e).collect();
    let overall = average(&post)?;
    Ok(Dynamic {
        cells: fitted_cells,
        event_support,
        events,
        overall,
        fits,
        by_group,
        group_overall,
        by_calendar,
        calendar_overall,
        simple,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows() -> Vec<Row> {
        let mut result = Vec::new();
        for (unit, adoption, slope) in [
            (1, Adoption::At(Period(2)), 4.0),
            (2, Adoption::At(Period(2)), 6.0),
            (3, Adoption::Never, 1.0),
            (4, Adoption::Never, 3.0),
            (5, Adoption::At(Period(3)), 9.0),
        ] {
            for period in 1..=3 {
                result.push(Row {
                    unit: Unit(unit),
                    period: Period(period),
                    adoption,
                    outcome: 10.0 + slope * (period - 1) as f64,
                });
            }
        }
        result
    }

    fn estimate(panel: &Panel, controls: Controls) -> Estimate {
        match cell(panel, Period(2), Period(2), controls, Baseline::Varying).unwrap() {
            Cell::Estimated(e) => e,
            Cell::Reference => panic!("expected estimate"),
        }
    }

    #[test]
    fn hand_checked_score_and_uncertainty() {
        let panel = Panel::new(&rows()).unwrap();
        let e = estimate(&panel, Controls::NeverTreated);
        assert_eq!(e.att, 3.0);
        assert_eq!(e.se, 1.0);
        assert_eq!(e.influence, vec![-2.5, 2.5, 2.5, -2.5, 0.0]);
        assert_eq!((e.treated, e.controls), (2, 2));
    }

    #[test]
    fn not_yet_treated_controls_change_by_time() {
        let panel = Panel::new(&rows()).unwrap();
        let e = estimate(&panel, Controls::NotYetTreated);
        assert_eq!(e.controls, 3);
        assert!((e.att - (5.0 - 13.0 / 3.0)).abs() < 1e-14);
        let Cell::Estimated(later) = cell(
            &panel,
            Period(2),
            Period(3),
            Controls::NotYetTreated,
            Baseline::Varying,
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(later.controls, 2);
    }

    #[test]
    fn row_permutations_preserve_unit_alignment() {
        let mut data = rows();
        let original = estimate(&Panel::new(&data).unwrap(), Controls::NotYetTreated);
        for _ in 0..data.len() {
            data.rotate_left(1);
            assert_eq!(
                original,
                estimate(&Panel::new(&data).unwrap(), Controls::NotYetTreated)
            );
        }
        data.reverse();
        assert_eq!(
            original,
            estimate(&Panel::new(&data).unwrap(), Controls::NotYetTreated)
        );
    }

    #[test]
    fn invalid_panel_states_are_rejected() {
        let mut data = rows();
        data.push(data[0]);
        assert!(matches!(Panel::new(&data), Err(Error::Duplicate { .. })));
        data.pop();
        data.pop();
        assert!(matches!(Panel::new(&data), Err(Error::Unbalanced { .. })));
        let mut data = rows();
        data[0].adoption = Adoption::Never;
        assert!(matches!(
            Panel::new(&data),
            Err(Error::CohortChanged { .. })
        ));
        let mut data = rows();
        data[0].outcome = f64::NAN;
        assert!(matches!(Panel::new(&data), Err(Error::NonFinite { .. })));
    }

    #[test]
    fn no_controls_is_not_a_zero_effect() {
        let data: Vec<_> = rows()
            .into_iter()
            .filter(|r| r.adoption != Adoption::Never)
            .collect();
        let panel = Panel::new(&data).unwrap();
        assert_eq!(
            cell(
                &panel,
                Period(2),
                Period(3),
                Controls::NotYetTreated,
                Baseline::Varying
            ),
            Err(Error::MissingControls)
        );
    }

    #[test]
    fn universal_reference_is_distinct_from_estimated_zero() {
        let panel = Panel::new(&rows()).unwrap();
        assert_eq!(
            cell(
                &panel,
                Period(3),
                Period(2),
                Controls::NeverTreated,
                Baseline::Universal
            ),
            Ok(Cell::Reference)
        );
        assert_eq!(
            cell(
                &panel,
                Period(3),
                Period(1),
                Controls::NeverTreated,
                Baseline::Varying
            ),
            Err(Error::NoBaseline)
        );
    }

    #[test]
    fn hand_checked_dynamic_aggregation() {
        let panel = Panel::new(&rows()).unwrap();
        let result = dynamic(&panel, Controls::NeverTreated, Baseline::Varying).unwrap();
        assert!((result.events[&0].att - 13.0 / 3.0).abs() < 1e-14);
        assert!((result.overall.att - 31.0 / 6.0).abs() < 1e-14);
        for summary in result
            .events
            .values()
            .chain(std::iter::once(&result.overall))
        {
            assert!(summary.influence.iter().sum::<f64>().abs() < 1e-12);
        }
        let universal = dynamic(&panel, Controls::NeverTreated, Baseline::Universal).unwrap();
        assert_eq!(universal.events[&-1].att, 0.0);
        assert_eq!(universal.events[&-2].att, -7.0);
        assert_eq!(result.overall, universal.overall);
    }

    #[test]
    fn plot_support_preserves_reference_and_shrinking_horizons() {
        let panel = Panel::new(&rows()).unwrap();
        let result = dynamic(&panel, Controls::NeverTreated, Baseline::Universal).unwrap();
        assert_eq!(result.event_support[&0].cohorts, vec![Period(2), Period(3)]);
        assert_eq!(result.event_support[&0].treated_units, 3);
        assert_eq!(result.event_support[&1].cohorts, vec![Period(2)]);
        assert_eq!(result.event_support[&1].treated_units, 2);
        assert_eq!(
            result.event_support[&-1].reference_cohorts,
            vec![Period(2), Period(3)]
        );
        assert!(result.event_support[&0].reference_cohorts.is_empty());
        assert_eq!(result.cells[&(Period(2), Period(1))], Cell::Reference);
        assert!(!result.events.contains_key(&2));
        for ((g, t), stored) in &result.cells {
            assert_eq!(
                *stored,
                cell(&panel, *g, *t, Controls::NeverTreated, Baseline::Universal).unwrap()
            );
        }
    }

    #[test]
    fn all_eventual_adopters_can_supply_earlier_controls() {
        let data: Vec<_> = rows()
            .into_iter()
            .filter(|r| r.adoption != Adoption::Never)
            .collect();
        let panel = Panel::new(&data).unwrap();
        let result = estimate(&panel, Controls::NotYetTreated);
        assert_eq!(result.att, -4.0);
        assert_eq!(result.controls, 1);
    }

    #[test]
    fn unit_relabelling_preserves_aggregate_results() {
        let data = rows();
        let original = dynamic(
            &Panel::new(&data).unwrap(),
            Controls::NeverTreated,
            Baseline::Varying,
        )
        .unwrap();
        let changed: Vec<_> = data
            .into_iter()
            .map(|r| Row {
                unit: Unit(100 - r.unit.0),
                ..r
            })
            .collect();
        let relabelled = dynamic(
            &Panel::new(&changed).unwrap(),
            Controls::NeverTreated,
            Baseline::Varying,
        )
        .unwrap();
        assert!((original.overall.att - relabelled.overall.att).abs() < 1e-14);
        assert!((original.overall.se - relabelled.overall.se).abs() < 1e-14);
        assert_eq!(
            original.overall.influence,
            relabelled
                .overall
                .influence
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
        );
    }
}
