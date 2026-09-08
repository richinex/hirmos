//! Lag-aligned X/Y/Z sample construction for Tigramite conditional-independence tests.
//!
//! Sample selection consumes an explicit [`TigramiteFrame`]; it never invents values for
//! missing observations. Lag cutoffs, role-aware masks, reference points, and bootstrap
//! projection remain visible in the returned audit record.

use crate::missing_data::{MaskType, PreprocessingError, TigramiteFrame};
use crate::parcorr::{dedup, CleanedXyz, CutOff, Node, TimeSeries};

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
    pub bootstrap: Option<&'a crate::bootstrap::Bootstrap>,
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
            bootstrap: None,
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
    pub bootstrap: Option<crate::bootstrap::BlockSummary>,
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

    let bootstrap = if let Some(plan) = options.bootstrap.filter(|_| !candidates.is_empty()) {
        let (indices, summary) = plan
            .draw(candidates.len())
            .map_err(PreprocessingError::Bootstrap)?;
        candidates = indices.into_iter().map(|i| candidates[i]).collect();
        Some(summary)
    } else {
        None
    };

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
            // DataFrame indexes every propagated sample; it does not clip at
            // the selected range's end. Preserve that failure as a typed error.
            if leading >= candidates.len() - position {
                return Err(PreprocessingError::MissingPropagationRange {
                    position,
                    propagation: leading,
                    samples: candidates.len(),
                });
            }
            let end = position + leading;
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
        return Err(match bootstrap {
            Some(summary) => PreprocessingError::EmptyBootstrap(summary),
            None => PreprocessingError::NoValidSamples,
        });
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
        bootstrap,
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
