//! Validity, analysis masks, missing-value propagation errors, and explicit imputation.
//!
//! Missingness is represented by validity bits, never by a numeric sentinel in the
//! scientific matrix. [`TigramiteFrame::from_missing_flag`] exists only at the
//! reference-compatibility boundary.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::parcorr::{Node, TimeSeries};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreprocessingError {
    EmptyMatrix,
    RaggedMatrix,
    ShapeMismatch(&'static str),
    IncomingNan {
        time: usize,
        variable: usize,
    },
    ValidNan {
        time: usize,
        variable: usize,
    },
    InvalidNode(Node),
    PositiveLag(Node),
    TauMaxTooSmall {
        tau_max: usize,
        selected_lag: usize,
    },
    EmptyNodeSet,
    NoValidSamples,
    EmptyBootstrap(crate::bootstrap::BlockSummary),
    Bootstrap(crate::bootstrap::BlockError),
    MissingPropagationRange {
        position: usize,
        propagation: usize,
        samples: usize,
    },
    InvalidMaxGap,
    InvalidCoordinates,
    InvalidColumns,
}

impl Display for PreprocessingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMatrix => write!(f, "the time-series matrix must not be empty"),
            Self::RaggedMatrix => write!(f, "all time-series rows must have equal width"),
            Self::ShapeMismatch(name) => write!(f, "{name} must have shape (T, N)"),
            Self::IncomingNan { time, variable } => {
                write!(
                    f,
                    "NaN in input at ({time}, {variable}); Tigramite rejects literal NaN"
                )
            }
            Self::ValidNan { time, variable } => {
                write!(f, "valid cell ({time}, {variable}) contains NaN")
            }
            Self::InvalidNode(node) => write!(f, "node {node:?} has an out-of-range variable"),
            Self::PositiveLag(node) => write!(f, "node {node:?} has a positive lag"),
            Self::TauMaxTooSmall {
                tau_max,
                selected_lag,
            } => write!(
                f,
                "tau_max {tau_max} is smaller than selected lag {selected_lag}"
            ),
            Self::EmptyNodeSet => write!(f, "X, Y, Z, and extra-Z cannot all be empty"),
            Self::NoValidSamples => write!(f, "no valid constructed samples remain"),
            Self::EmptyBootstrap(_) => write!(f, "no valid bootstrap samples remain"),
            Self::Bootstrap(error) => write!(f, "invalid bootstrap block sampling: {error:?}"),
            Self::MissingPropagationRange { position, propagation, samples } => write!(
                f, "missing-value propagation from sample {position} by {propagation} exceeds the {samples} selected samples"
            ),
            Self::InvalidMaxGap => write!(f, "max_gap must be positive"),
            Self::InvalidCoordinates => {
                write!(
                    f,
                    "coordinates must be finite, strictly increasing, and length T"
                )
            }
            Self::InvalidColumns => write!(f, "at least one in-range column is required"),
        }
    }
}

impl Error for PreprocessingError {}

fn flatten_bool(
    rows: &[Vec<bool>],
    t: usize,
    n: usize,
    name: &'static str,
) -> Result<Vec<bool>, PreprocessingError> {
    if rows.len() != t || rows.iter().any(|row| row.len() != n) {
        return Err(PreprocessingError::ShapeMismatch(name));
    }
    Ok(rows.iter().flatten().copied().collect())
}

fn validate_rows(rows: &[Vec<f64>]) -> Result<(usize, usize), PreprocessingError> {
    if rows.is_empty() || rows[0].is_empty() {
        return Err(PreprocessingError::EmptyMatrix);
    }
    let n = rows[0].len();
    if rows.iter().any(|row| row.len() != n) {
        return Err(PreprocessingError::RaggedMatrix);
    }
    Ok((rows.len(), n))
}

/// A nullable time-series matrix plus a distinct role-sensitive analysis mask.
///
/// `validity == false` means the observation is absent. `analysis_mask == true`
/// means the present observation is intentionally excluded for selected X/Y/Z roles.
#[derive(Clone, Debug)]
pub struct TigramiteFrame {
    pub data: TimeSeries,
    pub validity: Vec<bool>,
    pub analysis_mask: Option<Vec<bool>>,
    pub data_type: Option<Vec<bool>>,
}

impl TigramiteFrame {
    /// Reference-compatible constructor: reject literal NaNs first, then recognize
    /// the numeric sentinel. Invalid storage is normalized to zero and never read.
    pub fn from_missing_flag(
        rows: Vec<Vec<f64>>,
        missing_flag: Option<f64>,
        analysis_mask: Option<Vec<Vec<bool>>>,
    ) -> Result<Self, PreprocessingError> {
        let (t, n) = validate_rows(&rows)?;
        for (time, row) in rows.iter().enumerate() {
            for (variable, value) in row.iter().enumerate() {
                if value.is_nan() {
                    return Err(PreprocessingError::IncomingNan { time, variable });
                }
            }
        }
        let mut validity = vec![true; t * n];
        let mut safe_rows = rows;
        if let Some(flag) = missing_flag {
            for (time, row) in safe_rows.iter_mut().enumerate() {
                for (variable, value) in row.iter_mut().enumerate() {
                    if *value == flag {
                        validity[time * n + variable] = false;
                        *value = 0.0;
                    }
                }
            }
        }
        let analysis_mask = analysis_mask
            .as_deref()
            .map(|mask| flatten_bool(mask, t, n, "analysis mask"))
            .transpose()?;
        Ok(Self {
            data: TimeSeries::new(safe_rows),
            validity,
            analysis_mask,
            data_type: None,
        })
    }

    /// Hirmos constructor: Arrow validity is authoritative and invalid storage is ignored.
    pub fn from_validity(
        rows: Vec<Vec<f64>>,
        validity: Vec<Vec<bool>>,
        analysis_mask: Option<Vec<Vec<bool>>>,
    ) -> Result<Self, PreprocessingError> {
        let (t, n) = validate_rows(&rows)?;
        let validity = flatten_bool(&validity, t, n, "validity")?;
        let mut safe_rows = rows;
        for (time, row) in safe_rows.iter_mut().enumerate() {
            for (variable, value) in row.iter_mut().enumerate() {
                let index = time * n + variable;
                if validity[index] && value.is_nan() {
                    return Err(PreprocessingError::ValidNan { time, variable });
                }
                if !validity[index] {
                    *value = 0.0;
                }
            }
        }
        let analysis_mask = analysis_mask
            .as_deref()
            .map(|mask| flatten_bool(mask, t, n, "analysis mask"))
            .transpose()?;
        Ok(Self {
            data: TimeSeries::new(safe_rows),
            validity,
            analysis_mask,
            data_type: None,
        })
    }

    pub fn with_data_type(mut self, data_type: Vec<Vec<bool>>) -> Result<Self, PreprocessingError> {
        self.data_type = Some(flatten_bool(
            &data_type,
            self.data.t,
            self.data.n,
            "data type",
        )?);
        Ok(self)
    }

    pub(crate) fn is_valid(&self, time: usize, variable: usize) -> bool {
        self.validity[time * self.data.n + variable]
    }

    pub(crate) fn is_masked(&self, time: usize, variable: usize) -> bool {
        self.analysis_mask
            .as_ref()
            .is_some_and(|mask| mask[time * self.data.n + variable])
    }

    pub(crate) fn is_discrete(&self, time: usize, variable: usize) -> Option<bool> {
        self.data_type
            .as_ref()
            .map(|types| types[time * self.data.n + variable])
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MaskType {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl MaskType {
    pub const NONE: Self = Self {
        x: false,
        y: false,
        z: false,
    };
    pub const X: Self = Self {
        x: true,
        y: false,
        z: false,
    };
    pub const Y: Self = Self {
        x: false,
        y: true,
        z: false,
    };
    pub const Z: Self = Self {
        x: false,
        y: false,
        z: true,
    };
    pub const XY: Self = Self {
        x: true,
        y: true,
        z: false,
    };
    pub const XZ: Self = Self {
        x: true,
        y: false,
        z: true,
    };
    pub const YZ: Self = Self {
        x: false,
        y: true,
        z: true,
    };
    pub const XYZ: Self = Self {
        x: true,
        y: true,
        z: true,
    };
}

#[derive(Clone, Debug)]
pub struct ImputationResult {
    pub frame: TigramiteFrame,
    /// True only for cells filled by this transform.
    pub imputed: Vec<bool>,
}

fn checked_coordinates(
    t: usize,
    coordinates: Option<&[f64]>,
) -> Result<Vec<f64>, PreprocessingError> {
    let coordinates =
        coordinates.map_or_else(|| (0..t).map(|i| i as f64).collect(), |x| x.to_vec());
    if coordinates.len() != t
        || coordinates.iter().any(|x| !x.is_finite())
        || coordinates.windows(2).any(|pair| pair[1] <= pair[0])
    {
        return Err(PreprocessingError::InvalidCoordinates);
    }
    Ok(coordinates)
}

/// Fill only bounded interior gaps of at most `max_gap`, using actual time coordinates.
pub fn linear_interpolate(
    frame: &TigramiteFrame,
    max_gap: usize,
    coordinates: Option<&[f64]>,
) -> Result<ImputationResult, PreprocessingError> {
    if max_gap == 0 {
        return Err(PreprocessingError::InvalidMaxGap);
    }
    let coordinates = checked_coordinates(frame.data.t, coordinates)?;
    let mut result = frame.clone();
    let mut imputed = vec![false; frame.data.t * frame.data.n];
    for variable in 0..frame.data.n {
        let mut cursor = 0;
        while cursor < frame.data.t {
            if result.validity[cursor * frame.data.n + variable] {
                cursor += 1;
                continue;
            }
            let start = cursor;
            while cursor < frame.data.t && !result.validity[cursor * frame.data.n + variable] {
                cursor += 1;
            }
            let end = cursor;
            if start == 0 || end == frame.data.t || end - start > max_gap {
                continue;
            }
            let left = start - 1;
            let right = end;
            let left_value = result.data.at(left, variable);
            let right_value = result.data.at(right, variable);
            let width = coordinates[right] - coordinates[left];
            for time in start..end {
                let weight = (coordinates[time] - coordinates[left]) / width;
                let index = time * frame.data.n + variable;
                result.data.values[index] = left_value + weight * (right_value - left_value);
                result.validity[index] = true;
                imputed[index] = true;
            }
        }
    }
    Ok(ImputationResult {
        frame: result,
        imputed,
    })
}

/// Carry a previous state through a whole missing run only when it fits the gap budget.
pub fn forward_fill(
    frame: &TigramiteFrame,
    max_gap: usize,
) -> Result<ImputationResult, PreprocessingError> {
    if max_gap == 0 {
        return Err(PreprocessingError::InvalidMaxGap);
    }
    let mut result = frame.clone();
    let mut imputed = vec![false; frame.data.t * frame.data.n];
    for variable in 0..frame.data.n {
        let mut cursor = 0;
        while cursor < frame.data.t {
            if result.validity[cursor * frame.data.n + variable] {
                cursor += 1;
                continue;
            }
            let start = cursor;
            while cursor < frame.data.t && !result.validity[cursor * frame.data.n + variable] {
                cursor += 1;
            }
            let end = cursor;
            if start == 0 || end - start > max_gap {
                continue;
            }
            let previous = result.data.at(start - 1, variable);
            for time in start..end {
                let index = time * frame.data.n + variable;
                result.data.values[index] = previous;
                result.validity[index] = true;
                imputed[index] = true;
            }
        }
    }
    Ok(ImputationResult {
        frame: result,
        imputed,
    })
}

/// Fill invalid cells with a structural zero. Calling this function is the explicit confirmation.
pub fn structural_zero(frame: &TigramiteFrame) -> ImputationResult {
    let mut result = frame.clone();
    let mut imputed = vec![false; frame.data.t * frame.data.n];
    for (index, was_imputed) in imputed.iter_mut().enumerate() {
        if !result.validity[index] {
            result.data.values[index] = 0.0;
            result.validity[index] = true;
            *was_imputed = true;
        }
    }
    ImputationResult {
        frame: result,
        imputed,
    }
}

/// Earliest longest half-open interval complete in every selected column.
pub fn longest_complete_interval(
    validity: &[bool],
    t: usize,
    n: usize,
    columns: &[usize],
) -> Result<Option<(usize, usize)>, PreprocessingError> {
    if validity.len() != t * n {
        return Err(PreprocessingError::ShapeMismatch("validity"));
    }
    if columns.is_empty() || columns.iter().any(|&column| column >= n) {
        return Err(PreprocessingError::InvalidColumns);
    }
    let complete = |time: usize| {
        columns
            .iter()
            .all(|&variable| validity[time * n + variable])
    };
    let mut best = None;
    let mut cursor = 0;
    while cursor < t {
        if !complete(cursor) {
            cursor += 1;
            continue;
        }
        let start = cursor;
        while cursor < t && complete(cursor) {
            cursor += 1;
        }
        let candidate = (start, cursor);
        if best.is_none_or(|(old_start, old_end)| candidate.1 - candidate.0 > old_end - old_start) {
            best = Some(candidate);
        }
    }
    Ok(best)
}
