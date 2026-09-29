// Copyright (c) 2024 GRF Contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
// GRF 2.6.1 InstrumentalSplittingRule with Z = W. See COPYING.
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Split {
    pub variable: usize,
    pub value: f64,
    pub missing_left: bool,
}

#[derive(Clone, Copy, Default)]
struct Bucket {
    count: usize,
    small: usize,
    weight: f64,
    response: f64,
    treatment: f64,
    squared: f64,
}
impl Bucket {
    // Keep the pinned GCC oracle's fused accumulation explicit across targets.
    fn add_sample(&mut self, weight: f64, response: f64, treatment: f64, mean: f64) {
        self.count += 1;
        self.small += usize::from(treatment < mean);
        self.weight += weight;
        self.response = weight.mul_add(response, self.response);
        // GCC reuses the rounded weight*z product in both bucket sums. The
        // bucket's squared sum is not contracted, unlike the whole-node sum.
        let weighted = weight * treatment;
        self.treatment += weighted;
        self.squared += weighted * treatment;
    }
    fn add(&mut self, other: Self) {
        self.count += other.count;
        self.small += other.small;
        self.weight += other.weight;
        self.response += other.response;
        self.treatment += other.treatment;
        self.squared += other.squared;
    }
    fn size(&self) -> f64 {
        self.squared - self.treatment * self.treatment / self.weight
    }
}

/// Covariates are column-major; responses are the node's GRF relabeling,
/// indexed by original row. Candidate ordering is significant for tied scores.
pub fn best_split(
    columns: &[Vec<f64>],
    treatment: &[f64],
    weights: &[f64],
    responses: &[f64],
    samples: &[usize],
    candidates: &[usize],
    min_node_size: usize,
    alpha: f64,
    imbalance_penalty: f64,
) -> Result<Option<Split>, &'static str> {
    ValidatedInputs::new(columns, treatment, weights)?.best_split(
        responses,
        samples,
        candidates,
        min_node_size,
        alpha,
        imbalance_penalty,
    )
}

/// Static input validation is retained for the lifetime of the immutable
/// training slices. Each node still validates its changing responses and IDs.
/// Private fields prevent a caller from bypassing construction checks.
pub(crate) struct ValidatedInputs<'a> {
    columns: &'a [Vec<f64>],
    treatment: &'a [f64],
    weights: &'a [f64],
}
impl<'a> ValidatedInputs<'a> {
    pub(crate) fn new(
        columns: &'a [Vec<f64>],
        treatment: &'a [f64],
        weights: &'a [f64],
    ) -> Result<Self, &'static str> {
        let n = treatment.len();
        if n == 0
            || weights.len() != n
            || columns
                .iter()
                .any(|c| c.len() != n || c.iter().any(|v| v.is_infinite()))
            || treatment.iter().any(|v| !v.is_finite())
            || weights.iter().any(|v| !v.is_finite() || *v < 0.)
        {
            return Err("Invalid causal split inputs or options.");
        }
        Ok(Self {
            columns,
            treatment,
            weights,
        })
    }
    pub(crate) fn best_split(
        &self,
        responses: &[f64],
        samples: &[usize],
        candidates: &[usize],
        min_node_size: usize,
        alpha: f64,
        imbalance_penalty: f64,
    ) -> Result<Option<Split>, &'static str> {
        let Self {
            columns,
            treatment,
            weights,
        } = *self;
        let n = treatment.len();
        if responses.len() != n
            || responses.iter().any(|v| !v.is_finite())
            || samples.iter().any(|&i| i >= n)
            || candidates.iter().any(|&i| i >= columns.len())
            || min_node_size == 0
            || !alpha.is_finite()
            || !(0.0..0.5).contains(&alpha)
            || !imbalance_penalty.is_finite()
            || imbalance_penalty < 0.0
        {
            return Err("Invalid causal split inputs or options.");
        }
        if samples.len() < 2 {
            return Ok(None);
        }
        let mut node = Bucket::default();
        for &i in samples {
            node.weight += weights[i];
            node.response = weights[i].mul_add(responses[i], node.response);
            node.treatment += weights[i] * treatment[i];
            node.squared = (weights[i] * treatment[i]).mul_add(treatment[i], node.squared);
        }
        if node.weight <= 0.0 {
            return Ok(None);
        }
        let mean = node.treatment / node.weight;
        node.count = samples.len();
        node.small = samples.iter().filter(|&&i| treatment[i] < mean).count();
        let min_child_size = node.size() * alpha;
        let mut best = None;
        let mut best_decrease = 0.0;
        for &variable in candidates {
            let column = &columns[variable];
            let mut sorted = samples.to_vec();
            // Stable ordering is an upstream numerical requirement, including ties.
            sorted.sort_by(|&a, &b| match (column[a].is_nan(), column[b].is_nan()) {
                (true, true) => Ordering::Equal,
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => column[a].partial_cmp(&column[b]).unwrap(),
            });
            let mut values: Vec<_> = sorted.iter().map(|&i| column[i]).collect();
            values.dedup_by(|a, b| *a == *b || (a.is_nan() && b.is_nan()));
            if values.len() < 2 {
                continue;
            }
            let mut buckets = vec![Bucket::default(); values.len()];
            let mut missing = Bucket::default();
            let mut index = 0;
            // The final sorted row is deliberately excluded, as in GRF.
            for pair in sorted.windows(2) {
                let i = pair[0];
                let value = column[i];
                let target = if value.is_nan() {
                    &mut missing
                } else {
                    &mut buckets[index]
                };
                target.add_sample(weights[i], responses[i], treatment[i], mean);
                let next = column[pair[1]];
                if value != next && !next.is_nan() {
                    index += 1;
                }
            }
            for missing_left in [true, false] {
                if !missing_left && missing.count == 0 {
                    break;
                }
                let mut left = if missing_left {
                    missing
                } else {
                    Bucket::default()
                };
                for (i, bucket) in buckets.iter().take(values.len() - 1).enumerate() {
                    if i == 0 && !missing_left {
                        continue;
                    }
                    left.add(*bucket);
                    if left.small < min_node_size || left.count - left.small < min_node_size {
                        continue;
                    }
                    let right_small = node.small - left.small;
                    if right_small < min_node_size
                        || node.count - left.count - right_small < min_node_size
                    {
                        break;
                    }
                    let size_left = left.size();
                    if size_left < min_child_size || (imbalance_penalty > 0.0 && size_left == 0.0) {
                        continue;
                    }
                    let right = Bucket {
                        weight: node.weight - left.weight,
                        response: node.response - left.response,
                        treatment: node.treatment - left.treatment,
                        squared: node.squared - left.squared,
                        ..Bucket::default()
                    };
                    let size_right = right.size();
                    if size_right < min_child_size || (imbalance_penalty > 0.0 && size_right == 0.0)
                    {
                        continue;
                    }
                    let mut decrease = left.response * left.response / left.weight
                        + right.response * right.response / right.weight;
                    decrease =
                        (-imbalance_penalty).mul_add(1.0 / size_left + 1.0 / size_right, decrease);
                    if decrease > best_decrease {
                        best_decrease = decrease;
                        best = Some(Split {
                            variable,
                            value: values[i],
                            missing_left,
                        });
                    }
                }
            }
        }
        Ok(best)
    }
}
