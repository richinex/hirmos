use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonEmptyVec<T> {
    values: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub fn try_from_vec(values: Vec<T>) -> Result<Self, DataPreparationError> {
        if values.is_empty() {
            return Err(DataPreparationError::EmptyCollection);
        }
        Ok(Self { values })
    }

    pub fn first(&self) -> &T {
        &self.values[0]
    }

    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    pub fn into_vec(self) -> Vec<T> {
        self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }
}

impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataPreparationError {
    EmptyCollection,
    EmptyMatrix,
    RaggedMatrix,
    ShapeMismatch { field: &'static str },
    NonFiniteObservedValue { time: usize, column: usize },
    InvalidCoordinates,
    InvalidColumn { column: usize, column_count: usize },
    InvalidWindow { start: usize, end_exclusive: usize },
    DuplicateReferencePoint { reference: usize },
    ExpectedPositive { value: usize },
    WarmupShorterThanFeatureHistory { warmup: usize, required: usize },
    EmptyConfirmation,
}

impl Display for DataPreparationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyCollection => write!(f, "a required collection is empty"),
            Self::EmptyMatrix => write!(f, "the dataset must contain at least one cell"),
            Self::RaggedMatrix => write!(f, "all matrix rows must have the same width"),
            Self::ShapeMismatch { field } => {
                write!(f, "{field} does not match the dataset shape")
            }
            Self::NonFiniteObservedValue { time, column } => write!(
                f,
                "observed cell ({time}, {column}) contains a non-finite value"
            ),
            Self::InvalidCoordinates => write!(
                f,
                "time coordinates must be finite, strictly increasing, and match the row count"
            ),
            Self::InvalidColumn {
                column,
                column_count,
            } => write!(
                f,
                "column {column} is outside the dataset's {column_count} columns"
            ),
            Self::InvalidWindow {
                start,
                end_exclusive,
            } => write!(f, "invalid half-open window [{start}, {end_exclusive})"),
            Self::DuplicateReferencePoint { reference } => {
                write!(f, "reference point {reference} is duplicated")
            }
            Self::ExpectedPositive { value } => {
                write!(f, "expected a positive integer, received {value}")
            }
            Self::WarmupShorterThanFeatureHistory { warmup, required } => write!(
                f,
                "warmup {warmup} is shorter than required feature history {required}"
            ),
            Self::EmptyConfirmation => write!(f, "an explicit confirmation id is required"),
        }
    }
}

impl Error for DataPreparationError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnId(usize);

impl ColumnId {
    pub fn new(column: usize, column_count: usize) -> Result<Self, DataPreparationError> {
        if column >= column_count {
            return Err(DataPreparationError::InvalidColumn {
                column,
                column_count,
            });
        }
        Ok(Self(column))
    }

    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lookback(usize);

impl Lookback {
    pub const fn new(steps: usize) -> Self {
        Self(steps)
    }

    pub const fn steps(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositiveUsize(usize);

impl PositiveUsize {
    pub fn new(value: usize) -> Result<Self, DataPreparationError> {
        if value == 0 {
            return Err(DataPreparationError::ExpectedPositive { value });
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfirmationId(String);

impl ConfirmationId {
    pub fn new(value: impl Into<String>) -> Result<Self, DataPreparationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DataPreparationError::EmptyConfirmation);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumericNullPolicy {
    NoSentinel,
    ConfirmedSentinel {
        value: f64,
        confirmation: ConfirmationId,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct NullableDataset {
    pub(super) values: Vec<f64>,
    pub(super) validity: Vec<bool>,
    pub(super) analysis_exclusions: Vec<bool>,
    pub(super) imputed: Vec<bool>,
    pub(super) data_type: Option<Vec<bool>>,
    pub(super) coordinates: Vec<f64>,
    pub(super) time_count: usize,
    pub(super) column_count: usize,
}

fn shape(rows: &[Vec<f64>]) -> Result<(usize, usize), DataPreparationError> {
    if rows.is_empty() || rows[0].is_empty() {
        return Err(DataPreparationError::EmptyMatrix);
    }
    let columns = rows[0].len();
    if rows.iter().any(|row| row.len() != columns) {
        return Err(DataPreparationError::RaggedMatrix);
    }
    Ok((rows.len(), columns))
}

fn flatten_bool(
    rows: &[Vec<bool>],
    times: usize,
    columns: usize,
    field: &'static str,
) -> Result<Vec<bool>, DataPreparationError> {
    if rows.len() != times || rows.iter().any(|row| row.len() != columns) {
        return Err(DataPreparationError::ShapeMismatch { field });
    }
    Ok(rows.iter().flatten().copied().collect())
}

fn validate_coordinates(
    coordinates: Vec<f64>,
    time_count: usize,
) -> Result<Vec<f64>, DataPreparationError> {
    if coordinates.len() != time_count
        || coordinates.iter().any(|value| !value.is_finite())
        || coordinates.windows(2).any(|pair| pair[1] <= pair[0])
    {
        return Err(DataPreparationError::InvalidCoordinates);
    }
    Ok(coordinates)
}

impl NullableDataset {
    pub fn from_rows(
        rows: Vec<Vec<f64>>,
        validity: Vec<Vec<bool>>,
        analysis_exclusions: Option<Vec<Vec<bool>>>,
        coordinates: Vec<f64>,
    ) -> Result<Self, DataPreparationError> {
        let (time_count, column_count) = shape(&rows)?;
        let validity = flatten_bool(&validity, time_count, column_count, "validity")?;
        let analysis_exclusions = analysis_exclusions
            .as_deref()
            .map(|rows| flatten_bool(rows, time_count, column_count, "analysis exclusions"))
            .transpose()?
            .unwrap_or_else(|| vec![false; time_count * column_count]);
        let coordinates = validate_coordinates(coordinates, time_count)?;
        let mut values: Vec<f64> = rows.into_iter().flatten().collect();
        for time in 0..time_count {
            for column in 0..column_count {
                let index = time * column_count + column;
                if validity[index] && !values[index].is_finite() {
                    return Err(DataPreparationError::NonFiniteObservedValue { time, column });
                }
                if !validity[index] {
                    values[index] = 0.0;
                }
            }
        }
        Ok(Self {
            values,
            validity,
            analysis_exclusions,
            imputed: vec![false; time_count * column_count],
            data_type: None,
            coordinates,
            time_count,
            column_count,
        })
    }

    pub fn import_numeric_rows(
        mut rows: Vec<Vec<f64>>,
        null_policy: NumericNullPolicy,
        analysis_exclusions: Option<Vec<Vec<bool>>>,
        coordinates: Vec<f64>,
    ) -> Result<Self, DataPreparationError> {
        let (time_count, column_count) = shape(&rows)?;
        let mut validity = vec![vec![true; column_count]; time_count];
        for time in 0..time_count {
            for column in 0..column_count {
                let value = rows[time][column];
                if !value.is_finite() {
                    return Err(DataPreparationError::NonFiniteObservedValue { time, column });
                }
                if let NumericNullPolicy::ConfirmedSentinel {
                    value: sentinel, ..
                } = &null_policy
                {
                    if value == *sentinel {
                        validity[time][column] = false;
                        rows[time][column] = 0.0;
                    }
                }
            }
        }
        Self::from_rows(rows, validity, analysis_exclusions, coordinates)
    }

    pub fn with_data_type(
        mut self,
        data_type: Vec<Vec<bool>>,
    ) -> Result<Self, DataPreparationError> {
        self.data_type = Some(flatten_bool(
            &data_type,
            self.time_count,
            self.column_count,
            "data type",
        )?);
        Ok(self)
    }

    pub fn time_count(&self) -> usize {
        self.time_count
    }

    pub fn column_count(&self) -> usize {
        self.column_count
    }

    pub fn coordinates(&self) -> &[f64] {
        &self.coordinates
    }

    pub fn value(&self, time: usize, column: ColumnId) -> f64 {
        self.values[time * self.column_count + column.index()]
    }

    pub fn is_valid(&self, time: usize, column: ColumnId) -> bool {
        self.validity[time * self.column_count + column.index()]
    }

    pub fn is_excluded(&self, time: usize, column: ColumnId) -> bool {
        self.analysis_exclusions[time * self.column_count + column.index()]
    }

    pub fn is_discrete(&self, time: usize, column: ColumnId) -> Option<bool> {
        self.data_type
            .as_ref()
            .map(|types| types[time * self.column_count + column.index()])
    }

    pub fn validity(&self) -> &[bool] {
        &self.validity
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn imputed(&self) -> &[bool] {
        &self.imputed
    }

    pub fn is_imputed(&self, time: usize, column: ColumnId) -> bool {
        self.imputed[time * self.column_count + column.index()]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeatureRole {
    CandidateCause,
    TestedOutcome,
    Conditioner,
    Auxiliary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LaggedFeature {
    pub column: ColumnId,
    pub lookback: Lookback,
    pub role: FeatureRole,
}

impl LaggedFeature {
    pub const fn new(column: ColumnId, lookback: Lookback, role: FeatureRole) -> Self {
        Self {
            column,
            lookback,
            role,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReferenceDomain {
    AllAvailable,
    Window { start: usize, end_exclusive: usize },
    Explicit(NonEmptyVec<usize>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryRequirement {
    MinimumForFeatures,
    FixedWarmup(Lookback),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GapInfluence {
    DirectOnly,
    FollowingGuard(PositiveUsize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnalysisExclusionPolicy {
    Ignore,
    ApplyTo(NonEmptyVec<FeatureRole>),
}

impl AnalysisExclusionPolicy {
    pub(super) fn applies_to(&self, role: FeatureRole) -> bool {
        match self {
            Self::Ignore => false,
            Self::ApplyTo(roles) => roles.as_slice().contains(&role),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatrixLayout {
    FeaturesBySamples,
    SamplesByFeatures,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LagProjectionRequest {
    pub features: NonEmptyVec<LaggedFeature>,
    pub reference_domain: ReferenceDomain,
    pub history: HistoryRequirement,
    pub gap_influence: GapInfluence,
    pub analysis_exclusions: AnalysisExclusionPolicy,
    pub layout: MatrixLayout,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExclusionReason {
    InsufficientHistory {
        required: usize,
    },
    OutsideWindow,
    OutsideDataset,
    MissingSourceCell {
        feature: usize,
        source_time: usize,
        column: ColumnId,
    },
    FollowingMissingnessGuard {
        origin_reference: usize,
        steps_after: usize,
    },
    AnalysisExcluded {
        feature: usize,
        source_time: usize,
        column: ColumnId,
        role: FeatureRole,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SampleDecision {
    Retained {
        reference: usize,
    },
    Excluded {
        reference: usize,
        reasons: NonEmptyVec<ExclusionReason>,
    },
}

impl SampleDecision {
    pub fn reference(&self) -> usize {
        match self {
            Self::Retained { reference } | Self::Excluded { reference, .. } => *reference,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LaggedAnalysisMatrix {
    pub values: Vec<Vec<f64>>,
    pub data_type: Option<Vec<Vec<bool>>>,
    pub imputed: Vec<Vec<bool>>,
    pub layout: MatrixLayout,
    pub features: NonEmptyVec<LaggedFeature>,
    pub retained_references: NonEmptyVec<usize>,
    pub decisions: NonEmptyVec<SampleDecision>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreparationRefusal {
    NoRetainedSamples,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectionOutcome {
    Built(LaggedAnalysisMatrix),
    Refused {
        decisions: NonEmptyVec<SampleDecision>,
        reasons: NonEmptyVec<PreparationRefusal>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellRef {
    pub time: usize,
    pub column: ColumnId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MissingRun {
    pub column: ColumnId,
    pub start: usize,
    pub end_exclusive: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImputationOutcome {
    NoChange {
        dataset: NullableDataset,
        unresolved_runs: Vec<MissingRun>,
    },
    Changed {
        dataset: NullableDataset,
        imputed_cells: NonEmptyVec<CellRef>,
        unresolved_runs: Vec<MissingRun>,
    },
}

impl ImputationOutcome {
    pub fn dataset(&self) -> &NullableDataset {
        match self {
            Self::NoChange { dataset, .. } | Self::Changed { dataset, .. } => dataset,
        }
    }

    pub fn imputed_cells(&self) -> &[CellRef] {
        match self {
            Self::NoChange { .. } => &[],
            Self::Changed { imputed_cells, .. } => imputed_cells.as_slice(),
        }
    }

    pub fn unresolved_runs(&self) -> &[MissingRun] {
        match self {
            Self::NoChange {
                unresolved_runs, ..
            }
            | Self::Changed {
                unresolved_runs, ..
            } => unresolved_runs,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompleteIntervalRefusal {
    NoCompletePoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompleteIntervalOutcome {
    Found { start: usize, end_exclusive: usize },
    Refused(CompleteIntervalRefusal),
}
