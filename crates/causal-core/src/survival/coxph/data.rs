//! Validated inputs for right-censored and start-stop Cox models.

use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Event {
    Censored,
    Observed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoxCovariates {
    rows: NonZeroUsize,
    columns: NonZeroUsize,
    values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RightCensoredData {
    durations: Vec<f64>,
    events: Vec<Event>,
    weights: Vec<f64>,
    entries: Option<Vec<f64>>,
    strata: Option<Vec<usize>>,
    clusters: Option<Vec<usize>>,
    covariates: CoxCovariates,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeVaryingData {
    subjects: Vec<usize>,
    starts: Vec<f64>,
    stops: Vec<f64>,
    events: Vec<Event>,
    weights: Vec<f64>,
    strata: Option<Vec<usize>>,
    covariates: CoxCovariates,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoxDataError {
    EmptyRows,
    EmptyCovariates,
    MatrixLength { expected: usize, actual: usize },
    RowCount { expected: usize, actual: usize },
    NonFiniteCovariate { index: usize },
    InvalidDuration { row: usize },
    InvalidWeight { row: usize },
    InvalidEntry { row: usize },
    InvalidInterval { row: usize },
    ImmediateEvent { row: usize },
    NoEvents,
    ConstantCovariate { column: usize },
}

impl CoxCovariates {
    pub fn new(rows: usize, columns: usize, values: Vec<f64>) -> Result<Self, CoxDataError> {
        let rows = NonZeroUsize::new(rows).ok_or(CoxDataError::EmptyRows)?;
        let columns = NonZeroUsize::new(columns).ok_or(CoxDataError::EmptyCovariates)?;
        let expected = rows.get() * columns.get();
        if values.len() != expected {
            return Err(CoxDataError::MatrixLength {
                expected,
                actual: values.len(),
            });
        }
        if let Some(index) = values.iter().position(|value| !value.is_finite()) {
            return Err(CoxDataError::NonFiniteCovariate { index });
        }
        for column in 0..columns.get() {
            let first = values[column];
            if (1..rows.get()).all(|row| values[row * columns.get() + column] == first) {
                return Err(CoxDataError::ConstantCovariate { column });
            }
        }
        Ok(Self {
            rows,
            columns,
            values,
        })
    }

    pub fn rows(&self) -> usize {
        self.rows.get()
    }

    pub fn columns(&self) -> usize {
        self.columns.get()
    }

    pub fn row(&self, row: usize) -> &[f64] {
        let start = row * self.columns();
        &self.values[start..start + self.columns()]
    }

    pub(crate) fn values(&self) -> &[f64] {
        &self.values
    }
}

impl RightCensoredData {
    pub fn new(
        durations: Vec<f64>,
        events: Vec<Event>,
        covariates: CoxCovariates,
    ) -> Result<Self, CoxDataError> {
        let rows = durations.len();
        Self::with_options(durations, events, vec![1.0; rows], None, covariates)
    }

    pub fn with_options(
        durations: Vec<f64>,
        events: Vec<Event>,
        weights: Vec<f64>,
        entries: Option<Vec<f64>>,
        covariates: CoxCovariates,
    ) -> Result<Self, CoxDataError> {
        Self::with_grouping(durations, events, weights, entries, None, None, covariates)
    }

    pub fn with_strata(
        durations: Vec<f64>,
        events: Vec<Event>,
        weights: Vec<f64>,
        entries: Option<Vec<f64>>,
        strata: Vec<usize>,
        covariates: CoxCovariates,
    ) -> Result<Self, CoxDataError> {
        Self::with_grouping(
            durations,
            events,
            weights,
            entries,
            Some(strata),
            None,
            covariates,
        )
    }

    pub fn with_grouping(
        durations: Vec<f64>,
        events: Vec<Event>,
        weights: Vec<f64>,
        entries: Option<Vec<f64>>,
        strata: Option<Vec<usize>>,
        clusters: Option<Vec<usize>>,
        covariates: CoxCovariates,
    ) -> Result<Self, CoxDataError> {
        let rows = covariates.rows();
        for actual in [durations.len(), events.len(), weights.len()] {
            if actual != rows {
                return Err(CoxDataError::RowCount {
                    expected: rows,
                    actual,
                });
            }
        }
        if let Some(values) = &entries {
            if values.len() != rows {
                return Err(CoxDataError::RowCount {
                    expected: rows,
                    actual: values.len(),
                });
            }
        }
        if let Some(values) = &strata {
            if values.len() != rows {
                return Err(CoxDataError::RowCount {
                    expected: rows,
                    actual: values.len(),
                });
            }
        }
        if let Some(values) = &clusters {
            if values.len() != rows {
                return Err(CoxDataError::RowCount {
                    expected: rows,
                    actual: values.len(),
                });
            }
        }
        for row in 0..rows {
            if !durations[row].is_finite() || durations[row] < 0.0 {
                return Err(CoxDataError::InvalidDuration { row });
            }
            if !weights[row].is_finite() || weights[row] <= 0.0 {
                return Err(CoxDataError::InvalidWeight { row });
            }
            if let Some(values) = &entries {
                if !values[row].is_finite() || values[row] >= durations[row] {
                    return Err(CoxDataError::InvalidEntry { row });
                }
            }
        }
        if !events.contains(&Event::Observed) {
            return Err(CoxDataError::NoEvents);
        }
        Ok(Self {
            durations,
            events,
            weights,
            entries,
            strata,
            clusters,
            covariates,
        })
    }

    pub fn rows(&self) -> usize {
        self.covariates.rows()
    }
    pub fn columns(&self) -> usize {
        self.covariates.columns()
    }
    pub fn covariates(&self) -> &CoxCovariates {
        &self.covariates
    }
    pub(crate) fn durations(&self) -> &[f64] {
        &self.durations
    }
    pub(crate) fn events(&self) -> &[Event] {
        &self.events
    }
    pub(crate) fn weights(&self) -> &[f64] {
        &self.weights
    }
    pub(crate) fn entries(&self) -> Option<&[f64]> {
        self.entries.as_deref()
    }
    pub fn strata(&self) -> Option<&[usize]> {
        self.strata.as_deref()
    }
    pub fn clusters(&self) -> Option<&[usize]> {
        self.clusters.as_deref()
    }
}

impl TimeVaryingData {
    pub fn new(
        subjects: Vec<usize>,
        starts: Vec<f64>,
        stops: Vec<f64>,
        events: Vec<Event>,
        covariates: CoxCovariates,
    ) -> Result<Self, CoxDataError> {
        let rows = starts.len();
        Self::with_strata(
            subjects,
            starts,
            stops,
            events,
            vec![1.0; rows],
            None,
            covariates,
        )
    }

    pub fn with_weights(
        subjects: Vec<usize>,
        starts: Vec<f64>,
        stops: Vec<f64>,
        events: Vec<Event>,
        weights: Vec<f64>,
        covariates: CoxCovariates,
    ) -> Result<Self, CoxDataError> {
        Self::with_strata(subjects, starts, stops, events, weights, None, covariates)
    }

    pub fn with_strata(
        subjects: Vec<usize>,
        starts: Vec<f64>,
        stops: Vec<f64>,
        events: Vec<Event>,
        weights: Vec<f64>,
        strata: Option<Vec<usize>>,
        covariates: CoxCovariates,
    ) -> Result<Self, CoxDataError> {
        let rows = covariates.rows();
        for actual in [
            subjects.len(),
            starts.len(),
            stops.len(),
            events.len(),
            weights.len(),
        ] {
            if actual != rows {
                return Err(CoxDataError::RowCount {
                    expected: rows,
                    actual,
                });
            }
        }
        if let Some(values) = &strata {
            if values.len() != rows {
                return Err(CoxDataError::RowCount {
                    expected: rows,
                    actual: values.len(),
                });
            }
        }
        for row in 0..rows {
            if !starts[row].is_finite()
                || !stops[row].is_finite()
                || starts[row] < 0.0
                || starts[row] > stops[row]
            {
                return Err(CoxDataError::InvalidInterval { row });
            }
            if events[row] == Event::Observed && starts[row] == stops[row] {
                return Err(CoxDataError::ImmediateEvent { row });
            }
            if !weights[row].is_finite() || weights[row] <= 0.0 {
                return Err(CoxDataError::InvalidWeight { row });
            }
        }
        if !events.contains(&Event::Observed) {
            return Err(CoxDataError::NoEvents);
        }
        Ok(Self {
            subjects,
            starts,
            stops,
            events,
            weights,
            strata,
            covariates,
        })
    }

    pub fn rows(&self) -> usize {
        self.covariates.rows()
    }
    pub fn columns(&self) -> usize {
        self.covariates.columns()
    }
    pub fn subjects(&self) -> &[usize] {
        &self.subjects
    }
    pub(crate) fn starts(&self) -> &[f64] {
        &self.starts
    }
    pub(crate) fn stops(&self) -> &[f64] {
        &self.stops
    }
    pub(crate) fn events(&self) -> &[Event] {
        &self.events
    }
    pub(crate) fn weights(&self) -> &[f64] {
        &self.weights
    }
    pub fn strata(&self) -> Option<&[usize]> {
        self.strata.as_deref()
    }
    pub(crate) fn covariates(&self) -> &CoxCovariates {
        &self.covariates
    }
}
