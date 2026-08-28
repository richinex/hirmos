//! Auditable missing-data preprocessing for the Tigramite ports and Hirmos.
//!
//! Missingness is represented by validity bits, never by a numeric sentinel in the
//! scientific matrix.  [`TigramiteFrame::from_missing_flag`] exists only to reproduce
//! the Python constructor at an interoperability boundary.  Lag-aware exclusion and
//! imputation are separate operations because they have different scientific meanings.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::parcorr::{dedup, CleanedXyz, CutOff, Node, TimeSeries};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreprocessingError {
    EmptyMatrix,
    RaggedMatrix,
    ShapeMismatch(&'static str),
    IncomingNan { time: usize, variable: usize },
    ValidNan { time: usize, variable: usize },
    InvalidNode(Node),
    PositiveLag(Node),
    TauMaxTooSmall { tau_max: usize, selected_lag: usize },
    EmptyNodeSet,
    NoValidSamples,
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

    fn is_valid(&self, time: usize, variable: usize) -> bool {
        self.validity[time * self.data.n + variable]
    }

    fn is_masked(&self, time: usize, variable: usize) -> bool {
        self.analysis_mask
            .as_ref()
            .is_some_and(|mask| mask[time * self.data.n + variable])
    }

    fn is_discrete(&self, time: usize, variable: usize) -> Option<bool> {
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
pub struct ConstructOptions<'a> {
    pub cut_off: CutOff,
    pub remove_missing_upto_maxlag: bool,
    pub mask_type: MaskType,
    /// Half-open restriction over reference time; this never compresses the time grid.
    pub window: Option<(usize, usize)>,
    /// Exact reference points, corresponding to Tigramite's `reference_points`.
    pub reference_points: Option<&'a [usize]>,
    /// Compatibility keep-filter used by RPCMCI after reference-point selection.
    pub reference_filter: Option<&'a [bool]>,
}

impl Default for ConstructOptions<'_> {
    fn default() -> Self {
        Self {
            cut_off: CutOff::TwoTauMax,
            remove_missing_upto_maxlag: false,
            mask_type: MaskType::NONE,
            window: None,
            reference_points: None,
            reference_filter: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleExclusionReason {
    LeadingCutoff,
    TrailingCutoff,
    MissingSelectedValue,
    MissingnessPropagation,
    MaskedX,
    MaskedY,
    MaskedZ,
    OutsideWindow,
    OutsideDataset,
    ReferenceFilter,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SampleExclusion {
    pub reference_point: usize,
    pub reasons: Vec<SampleExclusionReason>,
}

#[derive(Clone, Debug)]
pub struct ConstructedArray {
    /// Rows are X, Y, Z, then extra-Z; columns are retained samples.
    pub values: Vec<Vec<f64>>,
    /// Tigramite row codes: 0=X, 1=Y, 2=Z, 3=extra-Z.
    pub xyz: Vec<u8>,
    pub data_type: Option<Vec<Vec<bool>>>,
    pub retained_reference_points: Vec<usize>,
    pub exclusions: Vec<SampleExclusion>,
    pub cleaned: CleanedXyz,
    pub extra_z: Vec<Node>,
}

fn cleaned_nodes(x: &[Node], y: &[Node], z: &[Node], extra_z: &[Node]) -> (CleanedXyz, Vec<Node>) {
    let x = dedup(x);
    let y = dedup(y);
    let z: Vec<Node> = dedup(z)
        .into_iter()
        .filter(|node| !x.contains(node) && !y.contains(node))
        .collect();
    let extra_z: Vec<Node> = dedup(extra_z)
        .into_iter()
        .filter(|node| !x.contains(node) && !y.contains(node) && !z.contains(node))
        .collect();
    (CleanedXyz { x, y, z }, extra_z)
}

fn selected_max_lag(nodes: &[Node]) -> usize {
    nodes
        .iter()
        .map(|node| (-node.1) as usize)
        .max()
        .unwrap_or(0)
}

fn cutoff_lag(cut_off: CutOff, tau_max: usize, selected_lag: usize) -> usize {
    match cut_off {
        CutOff::TwoTauMax => 2 * tau_max,
        CutOff::TauMax => tau_max,
        CutOff::MaxLag => selected_lag,
        CutOff::MaxLagOrTauMax => selected_lag.max(tau_max),
        CutOff::TwoTauMaxFuture => selected_lag,
    }
}

fn push_reason(reasons: &mut Vec<SampleExclusionReason>, reason: SampleExclusionReason) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

/// The shared lag constructor used by dense legacy callers and nullable Hirmos callers.
#[allow(clippy::too_many_arguments)]
pub fn construct_array_tracked(
    frame: &TigramiteFrame,
    x: &[Node],
    y: &[Node],
    z: &[Node],
    extra_z: &[Node],
    tau_max: usize,
    options: ConstructOptions<'_>,
) -> Result<ConstructedArray, PreprocessingError> {
    let (cleaned, extra_z) = cleaned_nodes(x, y, z, extra_z);
    let nodes: Vec<Node> = cleaned
        .x
        .iter()
        .chain(cleaned.y.iter())
        .chain(cleaned.z.iter())
        .chain(extra_z.iter())
        .copied()
        .collect();
    if nodes.is_empty() {
        return Err(PreprocessingError::EmptyNodeSet);
    }
    for &node in &nodes {
        if node.0 >= frame.data.n {
            return Err(PreprocessingError::InvalidNode(node));
        }
        if node.1 > 0 {
            return Err(PreprocessingError::PositiveLag(node));
        }
    }

    let selected_lag = selected_max_lag(&nodes);
    let leading = cutoff_lag(options.cut_off, tau_max, selected_lag);
    if leading < selected_lag {
        return Err(PreprocessingError::TauMaxTooSmall {
            tau_max,
            selected_lag,
        });
    }
    let source_refs: Vec<usize> = options
        .reference_points
        .map_or_else(|| (0..frame.data.t).collect(), |points| points.to_vec());
    let mut candidates = Vec::new();
    let mut exclusions = Vec::new();
    for reference_point in source_refs.iter().copied() {
        let mut reasons = Vec::new();
        if reference_point >= frame.data.t {
            push_reason(&mut reasons, SampleExclusionReason::OutsideDataset);
        } else if reference_point < leading {
            push_reason(&mut reasons, SampleExclusionReason::LeadingCutoff);
        }
        if let Some((start, end)) = options.window {
            if reference_point < start || reference_point >= end {
                push_reason(&mut reasons, SampleExclusionReason::OutsideWindow);
            }
        }
        if reasons.is_empty() {
            candidates.push(reference_point);
        } else {
            exclusions.push(SampleExclusion {
                reference_point,
                reasons,
            });
        }
    }
    candidates.sort_unstable();

    if options.cut_off == CutOff::TwoTauMaxFuture {
        let target_count = source_refs
            .iter()
            .filter(|&&point| {
                point >= 2 * tau_max
                    && point < frame.data.t
                    && options
                        .window
                        .is_none_or(|(start, end)| point >= start && point < end)
            })
            .count();
        while candidates.len() > target_count {
            let reference_point = candidates.pop().expect("non-empty candidates");
            exclusions.push(SampleExclusion {
                reference_point,
                reasons: vec![SampleExclusionReason::TrailingCutoff],
            });
        }
    }

    let mut reasons_by_candidate = vec![Vec::new(); candidates.len()];
    let mut missing_positions = Vec::new();
    for (position, &reference_point) in candidates.iter().enumerate() {
        if options
            .reference_filter
            .is_some_and(|filter| reference_point >= filter.len() || !filter[reference_point])
        {
            push_reason(
                &mut reasons_by_candidate[position],
                SampleExclusionReason::ReferenceFilter,
            );
        }
        if nodes.iter().any(|&(variable, lag)| {
            let time = (reference_point as i64 + lag as i64) as usize;
            !frame.is_valid(time, variable)
        }) {
            push_reason(
                &mut reasons_by_candidate[position],
                SampleExclusionReason::MissingSelectedValue,
            );
            missing_positions.push(position);
        }
    }
    if options.remove_missing_upto_maxlag {
        for position in missing_positions {
            let end = (position + leading).min(candidates.len().saturating_sub(1));
            for reasons in reasons_by_candidate
                .iter_mut()
                .take(end + 1)
                .skip(position + 1)
            {
                push_reason(reasons, SampleExclusionReason::MissingnessPropagation);
            }
        }
    }

    let x_end = cleaned.x.len();
    let y_end = x_end + cleaned.y.len();
    let z_end = y_end + cleaned.z.len();
    for (position, &reference_point) in candidates.iter().enumerate() {
        for (row, &(variable, lag)) in nodes.iter().enumerate() {
            let role_reason = if row < x_end && options.mask_type.x {
                Some(SampleExclusionReason::MaskedX)
            } else if row >= x_end && row < y_end && options.mask_type.y {
                Some(SampleExclusionReason::MaskedY)
            } else if row >= y_end && row < z_end && options.mask_type.z {
                Some(SampleExclusionReason::MaskedZ)
            } else {
                None // Public Tigramite mask_type deliberately excludes extra-Z / `e`.
            };
            if let Some(reason) = role_reason {
                let time = (reference_point as i64 + lag as i64) as usize;
                if frame.is_masked(time, variable) {
                    push_reason(&mut reasons_by_candidate[position], reason);
                }
            }
        }
    }

    let retained_reference_points: Vec<usize> = candidates
        .iter()
        .zip(&reasons_by_candidate)
        .filter_map(|(&point, reasons)| reasons.is_empty().then_some(point))
        .collect();
    for (&reference_point, reasons) in candidates.iter().zip(reasons_by_candidate) {
        if !reasons.is_empty() {
            exclusions.push(SampleExclusion {
                reference_point,
                reasons,
            });
        }
    }
    exclusions.sort_by_key(|entry| entry.reference_point);
    if retained_reference_points.is_empty() {
        return Err(PreprocessingError::NoValidSamples);
    }

    let values: Vec<Vec<f64>> = nodes
        .iter()
        .map(|&(variable, lag)| {
            retained_reference_points
                .iter()
                .map(|&point| {
                    frame
                        .data
                        .at((point as i64 + lag as i64) as usize, variable)
                })
                .collect()
        })
        .collect();
    let data_type = frame.data_type.as_ref().map(|_| {
        nodes
            .iter()
            .map(|&(variable, lag)| {
                retained_reference_points
                    .iter()
                    .map(|&point| {
                        frame
                            .is_discrete((point as i64 + lag as i64) as usize, variable)
                            .expect("data type exists")
                    })
                    .collect()
            })
            .collect()
    });
    let xyz = (0..nodes.len())
        .map(|row| {
            if row < x_end {
                0
            } else if row < y_end {
                1
            } else if row < z_end {
                2
            } else {
                3
            }
        })
        .collect();
    Ok(ConstructedArray {
        values,
        xyz,
        data_type,
        retained_reference_points,
        exclusions,
        cleaned,
        extra_z,
    })
}

/// Dense compatibility adapter used by the pre-existing PCMCI and CausalEffects callers.
#[allow(clippy::too_many_arguments)]
pub(crate) fn construct_dense_compat(
    data: &TimeSeries,
    x: &[Node],
    y: &[Node],
    z: &[Node],
    extra_z: &[Node],
    tau_max: usize,
    cut_off: CutOff,
    window: Option<(usize, usize)>,
    reference_filter: Option<&[bool]>,
) -> ((Vec<Vec<f64>>, CleanedXyz), Vec<Node>) {
    let frame = TigramiteFrame {
        data: data.clone(),
        validity: vec![true; data.t * data.n],
        analysis_mask: None,
        data_type: None,
    };
    let result = construct_array_tracked(
        &frame,
        x,
        y,
        z,
        extra_z,
        tau_max,
        ConstructOptions {
            cut_off,
            window,
            reference_filter,
            ..ConstructOptions::default()
        },
    )
    .expect("legacy dense construct_array input must be valid");
    ((result.values, result.cleaned), result.extra_z)
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
