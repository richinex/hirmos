//! Calendar completeness, independent of values and imputation. Instants are interpreted in UTC.
use jiff::{
    civil::{Date, Weekday},
    tz::TimeZone,
    Timestamp,
};
use std::{collections::BTreeMap, num::NonZeroU64};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calendar {
    Daily,
    Weekly(Weekday),
    MonthStart,
    MonthEnd,
    QuarterStart,
    QuarterEnd,
    YearStart,
    YearEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    start: Date,
    end: Date,
}

impl Window {
    pub fn new(start: Date, end: Date) -> Result<Self, GridError> {
        if start > end {
            return Err(GridError::ReversedWindow);
        }
        Ok(Self { start, end })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coverage {
    Observed,
    Expected(Window),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GridError {
    Empty,
    ReversedWindow,
    InvalidTimestamp { row: usize },
    OutOfOrder { row: usize },
    DuplicatePeriod { row: usize },
    OffCalendar { row: usize },
    OffCalendarWindow,
    OutsideWindow { row: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Gap {
    first: Date,
    last: Date,
    count: NonZeroU64,
}
impl Gap {
    pub fn first(&self) -> Date {
        self.first
    }
    pub fn last(&self) -> Date {
        self.last
    }
    pub fn count(&self) -> u64 {
        self.count.get()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    calendar: Calendar,
    gaps: Vec<Gap>,
    observations: usize,
}
impl Grid {
    pub fn calendar(&self) -> Calendar {
        self.calendar
    }
    pub fn gaps(&self) -> &[Gap] {
        &self.gaps
    }
    pub fn observations(&self) -> usize {
        self.observations
    }
    pub fn missing(&self) -> u64 {
        self.gaps.iter().map(Gap::count).sum()
    }
}

fn date(ms: i64) -> Option<Date> {
    Some(
        Timestamp::from_millisecond(ms)
            .ok()?
            .to_zoned(TimeZone::UTC)
            .date(),
    )
}

fn position(d: Date, calendar: Calendar) -> Option<i64> {
    let days = i64::from(d.since(jiff::civil::date(1970, 1, 1)).ok()?.get_days());
    match calendar {
        Calendar::Daily => Some(days),
        Calendar::Weekly(day) => (d.weekday() == day).then(|| days.div_euclid(7)),
        Calendar::MonthStart => {
            (d.day() == 1).then(|| i64::from(d.year()) * 12 + i64::from(d.month()) - 1)
        }
        Calendar::MonthEnd => {
            (d == d.last_of_month()).then(|| i64::from(d.year()) * 12 + i64::from(d.month()) - 1)
        }
        Calendar::QuarterStart => (d.day() == 1 && (d.month() - 1) % 3 == 0)
            .then(|| i64::from(d.year()) * 4 + i64::from((d.month() - 1) / 3)),
        Calendar::QuarterEnd => (d == d.last_of_month() && d.month() % 3 == 0)
            .then(|| i64::from(d.year()) * 4 + i64::from((d.month() - 1) / 3)),
        Calendar::YearStart => (d.month() == 1 && d.day() == 1).then(|| i64::from(d.year())),
        Calendar::YearEnd => (d.month() == 12 && d.day() == 31).then(|| i64::from(d.year())),
    }
}

fn next(d: Date, calendar: Calendar) -> Date {
    use jiff::ToSpan;
    match calendar {
        Calendar::Daily => d.checked_add(1.days()).unwrap(),
        Calendar::Weekly(_) => d.checked_add(7.days()).unwrap(),
        Calendar::MonthStart => d.checked_add(1.months()).unwrap(),
        Calendar::MonthEnd => d
            .first_of_month()
            .checked_add(1.months())
            .unwrap()
            .last_of_month(),
        Calendar::QuarterStart => d.checked_add(3.months()).unwrap(),
        Calendar::QuarterEnd => d.first_of_month().checked_add(3.months()).unwrap().last_of_month(),
        Calendar::YearStart | Calendar::YearEnd => d.checked_add(1.years()).unwrap(),
    }
}

fn previous(d: Date, calendar: Calendar) -> Date {
    use jiff::ToSpan;
    match calendar {
        Calendar::Daily => d.checked_sub(1.days()).unwrap(),
        Calendar::Weekly(_) => d.checked_sub(7.days()).unwrap(),
        Calendar::MonthStart => d.checked_sub(1.months()).unwrap(),
        Calendar::MonthEnd => d
            .first_of_month()
            .checked_sub(1.months())
            .unwrap()
            .last_of_month(),
        Calendar::QuarterStart => d.checked_sub(3.months()).unwrap(),
        Calendar::QuarterEnd => d.first_of_month().checked_sub(3.months()).unwrap().last_of_month(),
        Calendar::YearStart | Calendar::YearEnd => d.checked_sub(1.years()).unwrap(),
    }
}

/// Input must be sorted, with one observation per declared calendar period.
/// Only gap boundaries are materialized, so cost does not grow with the number of absent dates.
pub fn inspect(times: &[i64], calendar: Calendar, coverage: Coverage) -> Result<Grid, GridError> {
    let bounds = match coverage {
        Coverage::Observed => None,
        Coverage::Expected(w) => Some((
            w,
            position(w.start, calendar).ok_or(GridError::OffCalendarWindow)?,
            position(w.end, calendar).ok_or(GridError::OffCalendarWindow)?,
        )),
    };
    let mut gaps = Vec::new();
    let mut prior: Option<(Date, i64)> = None;
    for (row, &ms) in times.iter().enumerate() {
        if row > 0 && ms < times[row - 1] {
            return Err(GridError::OutOfOrder { row });
        }
        let d = date(ms).ok_or(GridError::InvalidTimestamp { row })?;
        let p = position(d, calendar).ok_or(GridError::OffCalendar { row })?;
        if let Some((_, start, end)) = bounds {
            if p < start || p > end {
                return Err(GridError::OutsideWindow { row });
            }
        }
        match prior {
            Some((left, q)) => {
                if p == q {
                    return Err(GridError::DuplicatePeriod { row });
                }
                if let Some(count) = NonZeroU64::new((p - q - 1) as u64) {
                    gaps.push(Gap {
                        first: next(left, calendar),
                        last: previous(d, calendar),
                        count,
                    });
                }
            }
            None => {
                if let Some((w, start, _)) = bounds {
                    if let Some(count) = NonZeroU64::new((p - start) as u64) {
                        gaps.push(Gap {
                            first: w.start,
                            last: previous(d, calendar),
                            count,
                        });
                    }
                }
            }
        }
        prior = Some((d, p));
    }
    match (prior, bounds) {
        (None, None) => return Err(GridError::Empty),
        (None, Some((w, start, end))) => gaps.push(Gap {
            first: w.start,
            last: w.end,
            count: NonZeroU64::new((end - start + 1) as u64).unwrap(),
        }),
        (Some((last, p)), Some((w, _, end))) => {
            if let Some(count) = NonZeroU64::new((end - p) as u64) {
                gaps.push(Gap {
                    first: next(last, calendar),
                    last: w.end,
                    count,
                });
            }
        }
        (Some(_), None) => {}
    }
    Ok(Grid {
        calendar,
        gaps,
        observations: times.len(),
    })
}

/// Each caller-supplied unit has its own sorted time axis and coverage window.
/// Errors are retained per unit; one invalid unit cannot hide the other reports.
pub fn inspect_panel<'a, K: Ord + Clone>(
    units: &'a BTreeMap<K, (Vec<i64>, Coverage)>,
    calendar: Calendar,
) -> BTreeMap<K, Result<Grid, GridError>> {
    units
        .iter()
        .map(|(unit, (times, coverage))| (unit.clone(), inspect(times, calendar, *coverage)))
        .collect()
}
