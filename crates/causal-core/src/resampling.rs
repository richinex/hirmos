//! A narrow port of pandas 2.3.3 `DataFrame.resample` for Hirmos's daily downsampling lane.
//!
//! Weekly bins reproduce `W-MON, closed="left", label="left"`; monthly bins reproduce
//! `MS, closed="left", label="left"`. `sum` and `mean` use the compensated accumulation in
//! `pandas/_libs/groupby.pyx`. The supported input is one finite observation per UTC calendar day.

use chrono::{Datelike, Duration, NaiveDate, TimeZone, Utc, Weekday};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResampleFrequency {
    WeeklyMonday,
    MonthStart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aggregation {
    Mean,
    Sum,
    Median,
    Minimum,
    Maximum,
    First,
    Last,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncompleteBins {
    Keep,
    Drop,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResampleResult {
    pub values: Vec<f64>,
    pub timestamps_ms: Vec<i64>,
    pub imputed_cells: Vec<(usize, usize)>,
    pub source_rows: usize,
    pub output_rows: usize,
    pub incomplete_bins: usize,
    pub bins_dropped: usize,
    pub source_rows_dropped: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResampleError {
    EmptyInput,
    ShapeMismatch,
    AggregationCountMismatch,
    NonFiniteValue { index: usize },
    InvalidImputedCell { row: usize, column: usize },
    InvalidTimestamp { row: usize },
    TimeNotIncreasing { row: usize },
    MultipleObservationsInDay { row: usize },
    MissingDailyObservation { row: usize },
    NoBinsRemain,
}

#[derive(Clone, Copy, Debug)]
struct Bin {
    label: NaiveDate,
    first_row: usize,
    end_row: usize,
    complete: bool,
}

fn utc_day(timestamp_ms: i64, row: usize) -> Result<NaiveDate, ResampleError> {
    Utc.timestamp_millis_opt(timestamp_ms)
        .single()
        .map(|instant| instant.date_naive())
        .ok_or(ResampleError::InvalidTimestamp { row })
}

fn bin_label(day: NaiveDate, frequency: ResampleFrequency) -> NaiveDate {
    match frequency {
        ResampleFrequency::WeeklyMonday => {
            let offset = match day.weekday() {
                Weekday::Mon => 0,
                Weekday::Tue => 1,
                Weekday::Wed => 2,
                Weekday::Thu => 3,
                Weekday::Fri => 4,
                Weekday::Sat => 5,
                Weekday::Sun => 6,
            };
            day - Duration::days(offset)
        }
        ResampleFrequency::MonthStart => NaiveDate::from_ymd_opt(day.year(), day.month(), 1)
            .expect("an existing date has a valid month"),
    }
}

fn expected_days(label: NaiveDate, frequency: ResampleFrequency) -> usize {
    match frequency {
        ResampleFrequency::WeeklyMonday => 7,
        ResampleFrequency::MonthStart => {
            let next = if label.month() == 12 {
                NaiveDate::from_ymd_opt(label.year() + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(label.year(), label.month() + 1, 1)
            }
            .expect("the month after a valid date is valid");
            (next - label).num_days() as usize
        }
    }
}

fn calendar_bins(
    timestamps_ms: &[i64],
    frequency: ResampleFrequency,
) -> Result<Vec<Bin>, ResampleError> {
    if timestamps_ms.is_empty() {
        return Err(ResampleError::EmptyInput);
    }

    let mut days: Vec<NaiveDate> = Vec::with_capacity(timestamps_ms.len());
    for (row, &timestamp) in timestamps_ms.iter().enumerate() {
        if row > 0 && timestamp <= timestamps_ms[row - 1] {
            return Err(ResampleError::TimeNotIncreasing { row });
        }

        let day = utc_day(timestamp, row)?;
        if let Some(previous) = days.last() {
            let distance = (day - *previous).num_days();
            if distance == 0 {
                return Err(ResampleError::MultipleObservationsInDay { row });
            }
            if distance != 1 {
                return Err(ResampleError::MissingDailyObservation { row });
            }
        }
        days.push(day);
    }

    let mut bins = Vec::new();
    let mut first_row = 0;
    let mut current = bin_label(days[0], frequency);
    for row in 1..=days.len() {
        let next = (row < days.len()).then(|| bin_label(days[row], frequency));
        if next == Some(current) {
            continue;
        }

        let count = row - first_row;
        bins.push(Bin {
            label: current,
            first_row,
            end_row: row,
            complete: count == expected_days(current, frequency),
        });
        first_row = row;
        if let Some(label) = next {
            current = label;
        }
    }
    Ok(bins)
}

// pandas/_libs/groupby.pyx::group_sum and ::group_mean use this Kahan update.
fn compensated_sum(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0;
    for &value in values {
        let y = value - compensation;
        let next = sum + y;
        compensation = next - sum - y;
        if compensation.is_nan() {
            compensation = 0.0;
        }
        sum = next;
    }
    sum
}

fn aggregate(values: &[f64], method: Aggregation) -> f64 {
    match method {
        Aggregation::Mean => compensated_sum(values) / values.len() as f64,
        Aggregation::Sum => compensated_sum(values),
        Aggregation::Median => {
            let mut ordered = values.to_vec();
            ordered.sort_by(f64::total_cmp);
            let middle = ordered.len() / 2;
            if ordered.len() % 2 == 0 {
                (ordered[middle - 1] + ordered[middle]) / 2.0
            } else {
                ordered[middle]
            }
        }
        Aggregation::Minimum => values
            .iter()
            .copied()
            .reduce(f64::min)
            .expect("bins are non-empty"),
        Aggregation::Maximum => values
            .iter()
            .copied()
            .reduce(f64::max)
            .expect("bins are non-empty"),
        Aggregation::First => values[0],
        Aggregation::Last => values[values.len() - 1],
    }
}

/// Downsample a finite column-major daily matrix with the pinned pandas bin and aggregation rules.
pub fn pandas_resample_daily(
    values: &[f64],
    rows: usize,
    columns: usize,
    timestamps_ms: &[i64],
    aggregations: &[Aggregation],
    imputed_cells: &[(usize, usize)],
    frequency: ResampleFrequency,
    incomplete: IncompleteBins,
) -> Result<ResampleResult, ResampleError> {
    if rows == 0 {
        return Err(ResampleError::EmptyInput);
    }
    if values.len()
        != rows
            .checked_mul(columns)
            .ok_or(ResampleError::ShapeMismatch)?
        || timestamps_ms.len() != rows
    {
        return Err(ResampleError::ShapeMismatch);
    }
    if aggregations.len() != columns {
        return Err(ResampleError::AggregationCountMismatch);
    }
    if let Some(index) = values.iter().position(|value| !value.is_finite()) {
        return Err(ResampleError::NonFiniteValue { index });
    }
    if let Some(&(row, column)) = imputed_cells
        .iter()
        .find(|&&(row, column)| row >= rows || column >= columns)
    {
        return Err(ResampleError::InvalidImputedCell { row, column });
    }

    let bins = calendar_bins(timestamps_ms, frequency)?;
    let retained: Vec<Bin> = bins
        .iter()
        .copied()
        .filter(|bin| incomplete == IncompleteBins::Keep || bin.complete)
        .collect();
    if retained.is_empty() {
        return Err(ResampleError::NoBinsRemain);
    }

    let output_rows = retained.len();
    let mut output = vec![0.0; output_rows * columns];
    let mut output_imputed = Vec::new();
    for column in 0..columns {
        for (output_row, bin) in retained.iter().enumerate() {
            let start = column * rows + bin.first_row;
            let end = column * rows + bin.end_row;
            output[column * output_rows + output_row] =
                aggregate(&values[start..end], aggregations[column]);
            if imputed_cells.iter().any(|&(row, source_column)| {
                source_column == column && row >= bin.first_row && row < bin.end_row
            }) {
                output_imputed.push((output_row, column));
            }
        }
    }

    let kept_rows: usize = retained.iter().map(|bin| bin.end_row - bin.first_row).sum();
    let labels = retained
        .iter()
        .map(|bin| {
            bin.label
                .and_hms_opt(0, 0, 0)
                .expect("midnight is valid")
                .and_utc()
                .timestamp_millis()
        })
        .collect();
    let incomplete_bins = bins.iter().filter(|bin| !bin.complete).count();
    Ok(ResampleResult {
        values: output,
        timestamps_ms: labels,
        imputed_cells: output_imputed,
        source_rows: rows,
        output_rows,
        incomplete_bins,
        bins_dropped: bins.len() - retained.len(),
        source_rows_dropped: rows - kept_rows,
    })
}
