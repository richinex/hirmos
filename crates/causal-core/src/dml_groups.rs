//! DoubleML's group average treatment effects (`DoubleMLIRM.gate`, `DoubleMLPLR.gate`).
//!
//! The reference regresses the model's orthogonal signal on a dummy basis of mutually exclusive
//! groups with statsmodels OLS and the HC0 sandwich, then reads each group's effect off its
//! coefficient with a pointwise normal interval. Because the dummies are orthogonal, that
//! regression separates into one weighted least-squares fit per group, which this port evaluates in
//! closed form. IRM offers it for the ATE score only; PLR scales the basis by the partialled-out
//! treatment. Joint intervals, which the reference draws by multiplier bootstrap, are not ported.

use crate::dml::DmlResult;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct GroupEffect {
    pub group: usize,
    pub observations: usize,
    pub effect: f64,
    pub standard_error: f64,
    pub confidence_interval: [f64; 2],
    /// The reference warns when a group effect rests on at most five observations.
    pub few_observations: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupEffects {
    pub confidence_level: f64,
    pub groups: Vec<GroupEffect>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroupEffectError {
    /// The IRM effect-on-the-treated score has no group-effect signal in the reference.
    ScoreNotSupported,
    LengthMismatch { expected: usize, actual: usize },
    GroupOutOfRange { row: usize, group: usize },
    EmptyGroup { group: usize },
    NoGroups,
    InvalidConfidenceLevel,
}

impl Display for GroupEffectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScoreNotSupported => write!(
                f,
                "group effects are defined for the ATE and partialling-out scores, not the effect on the treated"
            ),
            Self::LengthMismatch { expected, actual } => {
                write!(f, "group labels cover {actual} rows; the fit has {expected}")
            }
            Self::GroupOutOfRange { row, group } => {
                write!(f, "row {row} names group {group}, beyond the declared groups")
            }
            Self::EmptyGroup { group } => write!(f, "group {group} has no observations"),
            Self::NoGroups => write!(f, "group effects need at least one group"),
            Self::InvalidConfidenceLevel => {
                write!(f, "confidence level must be strictly between zero and one")
            }
        }
    }
}

impl Error for GroupEffectError {}

/// Effects per group for a fitted PLR or IRM model, with pointwise intervals at `level`.
///
/// `groups[i]` is the zero-based group of observation `i`; every group below `group_count` must
/// occur at least once, as the reference's dummy basis would otherwise be singular.
pub fn group_effects(
    fit: &DmlResult,
    groups: &[usize],
    group_count: usize,
    level: f64,
) -> Result<GroupEffects, GroupEffectError> {
    let signal = fit
        .group_signal()
        .ok_or(GroupEffectError::ScoreNotSupported)?;
    let n = signal.signal.len();
    if groups.len() != n {
        return Err(GroupEffectError::LengthMismatch {
            expected: n,
            actual: groups.len(),
        });
    }
    if group_count == 0 {
        return Err(GroupEffectError::NoGroups);
    }
    if let Some((row, &group)) = groups.iter().enumerate().find(|(_, &g)| g >= group_count) {
        return Err(GroupEffectError::GroupOutOfRange { row, group });
    }
    if !level.is_finite() || level <= 0.0 || level >= 1.0 {
        return Err(GroupEffectError::InvalidConfidenceLevel);
    }
    let z = spec_math::cephes64::ndtri(1.0 - (1.0 - level) / 2.0);

    let mut counts = vec![0usize; group_count];
    let mut cross = vec![0.0f64; group_count];
    let mut squares = vec![0.0f64; group_count];
    for (i, &g) in groups.iter().enumerate() {
        counts[g] += 1;
        cross[g] += signal.scale[i] * signal.signal[i];
        squares[g] += signal.scale[i] * signal.scale[i];
    }
    if let Some(group) = counts.iter().position(|&count| count == 0) {
        return Err(GroupEffectError::EmptyGroup { group });
    }
    // OLS on disjoint columns: each coefficient is the group's own projection. HC0 keeps the
    // squared residuals per observation, weighted by the column value, with no small-sample factor.
    let effects: Vec<f64> = (0..group_count).map(|g| cross[g] / squares[g]).collect();
    let mut meat = vec![0.0f64; group_count];
    for (i, &g) in groups.iter().enumerate() {
        let residual = signal.signal[i] - effects[g] * signal.scale[i];
        meat[g] += signal.scale[i] * signal.scale[i] * residual * residual;
    }
    let groups = (0..group_count)
        .map(|g| {
            let standard_error = (meat[g] / (squares[g] * squares[g])).sqrt();
            GroupEffect {
                group: g,
                observations: counts[g],
                effect: effects[g],
                standard_error,
                confidence_interval: [
                    effects[g] - z * standard_error,
                    effects[g] + z * standard_error,
                ],
                few_observations: counts[g] <= 5,
            }
        })
        .collect();
    Ok(GroupEffects {
        confidence_level: level,
        groups,
    })
}
