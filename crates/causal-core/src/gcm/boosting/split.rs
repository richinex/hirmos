//! Continuous, non-missing split search from scikit-learn 1.9.0 (BSD-3-Clause).
//! See reference/sklearn-1.9.0-gcm/licenses/COPYING.

use super::histogram::Bin;
use std::num::NonZeroUsize;

#[derive(Clone, Copy)]
pub enum Curvature {
    Unit,
    Variable,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SplitError {
    Shape,
    NonFinite,
    Constraint,
    Count,
}

#[derive(Clone, Copy, Debug)]
pub struct Child {
    pub gradient: f64,
    pub hessian: f64,
    pub count: u32,
    pub value: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Split {
    pub bin: usize,
    pub gain: f64,
    pub left: Child,
    pub right: Child,
}

pub struct Search {
    min_leaf: NonZeroUsize,
    min_hessian: f64,
    min_gain: f64,
    l2: f64,
}

impl Search {
    pub fn new(
        min_leaf: NonZeroUsize,
        min_hessian: f64,
        min_gain: f64,
        l2: f64,
    ) -> Result<Self, SplitError> {
        if [min_hessian, min_gain, l2]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(SplitError::Constraint);
        }
        Ok(Self {
            min_leaf,
            min_hessian,
            min_gain,
            l2,
        })
    }
    pub fn node_value(&self, gradient: f64, hessian: f64) -> f64 {
        -gradient / (hessian + self.l2 + 1e-15)
    }

    /// Histogram omits the reserved missing bin; totals retain the parent's reduction order.
    pub fn find(
        &self,
        histogram: &[Bin],
        total: Child,
        curvature: Curvature,
    ) -> Result<Option<Split>, SplitError> {
        if histogram.is_empty() {
            return Err(SplitError::Shape);
        }
        if [total.gradient, total.hessian, total.value]
            .iter()
            .any(|v| !v.is_finite())
            || histogram
                .iter()
                .any(|b| !b.gradient.is_finite() || !b.hessian.is_finite())
        {
            return Err(SplitError::NonFinite);
        }
        let count = histogram
            .iter()
            .try_fold(0u32, |n, b| n.checked_add(b.count))
            .ok_or(SplitError::Count)?;
        if count != total.count {
            return Err(SplitError::Count);
        }
        let mut left = Child {
            gradient: 0.,
            hessian: 0.,
            count: 0,
            value: 0.,
        };
        let mut best: Option<Split> = None;
        let loss = total.value * total.gradient;
        for (bin, hist) in histogram.iter().enumerate().take(histogram.len() - 1) {
            left.count += hist.count;
            left.gradient += hist.gradient;
            left.hessian += match curvature {
                Curvature::Unit => hist.count as f64,
                Curvature::Variable => hist.hessian,
            };
            let mut right = Child {
                gradient: total.gradient - left.gradient,
                hessian: total.hessian - left.hessian,
                count: total.count - left.count,
                value: 0.,
            };
            if (left.count as usize) < self.min_leaf.get() {
                continue;
            }
            if (right.count as usize) < self.min_leaf.get() {
                break;
            }
            if left.hessian < self.min_hessian {
                continue;
            }
            if right.hessian < self.min_hessian {
                break;
            }
            left.value = self.node_value(left.gradient, left.hessian);
            right.value = self.node_value(right.gradient, right.hessian);
            let gain = (loss - left.value * left.gradient) - right.value * right.gradient;
            if gain > self.min_gain && best.as_ref().map_or(true, |b| gain > b.gain) {
                best = Some(Split {
                    bin,
                    gain,
                    left,
                    right,
                });
            }
        }
        Ok(best)
    }
}
