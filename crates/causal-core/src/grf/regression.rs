// Copyright (c) 2024 GRF Contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
// GRF RegressionSplittingRule / RegressionPredictionStrategy, nuisance route.
use super::{
    forest,
    sampling::Clusters,
    splitting::Split,
    tree::{self, Tree},
};
use std::{cmp::Ordering, collections::BTreeSet};
#[derive(Clone, Copy, Default)]
struct Bin {
    count: usize,
    weight: f64,
    sum: f64,
}
impl Bin {
    fn sample(&mut self, w: f64, y: f64) {
        self.count += 1;
        self.weight += w;
        self.sum = w.mul_add(y, self.sum);
    }
    fn add(&mut self, b: Self) {
        self.count += b.count;
        self.weight += b.weight;
        self.sum += b.sum;
    }
}
pub(crate) fn split(
    columns: &[Vec<f64>],
    weights: &[f64],
    responses: &[f64],
    rows: &[usize],
    candidates: &[usize],
    alpha: f64,
    penalty: f64,
) -> Option<Split> {
    if rows.len() < 2 {
        return None;
    }
    let minimum = ((rows.len() as f64 * alpha).ceil() as usize).max(1);
    let mut total = Bin::default();
    for &i in rows {
        total.sample(weights[i], responses[i]);
    }
    let mut best = None;
    let mut best_score = 0.0;
    for &variable in candidates {
        let x = &columns[variable];
        let mut sorted = rows.to_vec();
        sorted.sort_by(|&a, &b| match (x[a].is_nan(), x[b].is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => x[a].partial_cmp(&x[b]).unwrap(),
        });
        let mut values: Vec<_> = sorted.iter().map(|&i| x[i]).collect();
        values.dedup_by(|a, b| *a == *b || (a.is_nan() && b.is_nan()));
        if values.len() < 2 {
            continue;
        }
        let mut bins = vec![Bin::default(); values.len()];
        let mut missing = Bin::default();
        let mut k = 0;
        for pair in sorted.windows(2) {
            let i = pair[0];
            if x[i].is_nan() {
                missing.sample(weights[i], responses[i]);
            } else {
                bins[k].sample(weights[i], responses[i]);
            }
            if x[i] != x[pair[1]] && !x[pair[1]].is_nan() {
                k += 1;
            }
        }
        for missing_left in [true, false] {
            if !missing_left && missing.count == 0 {
                break;
            }
            let mut left = if missing_left {
                missing
            } else {
                Bin::default()
            };
            for (i, b) in bins.iter().take(values.len() - 1).enumerate() {
                if i == 0 && !missing_left {
                    continue;
                }
                left.add(*b);
                if left.count < minimum {
                    continue;
                }
                let right_count = rows.len() - left.count;
                if right_count < minimum {
                    break;
                }
                let right_sum = total.sum - left.sum;
                let score = left.sum * left.sum / left.weight
                    + right_sum * right_sum / (total.weight - left.weight);
                let score =
                    (-penalty).mul_add(1.0 / left.count as f64 + 1.0 / right_count as f64, score);
                if score > best_score {
                    best_score = score;
                    best = Some(Split {
                        variable,
                        value: values[i],
                        missing_left,
                    });
                }
            }
        }
    }
    best
}

pub struct RegressionForest {
    pub trees: Vec<Tree<[f64; 2]>>,
    rows: usize,
}
pub fn train(
    columns: &[Vec<f64>],
    outcome: usize,
    weight: Option<usize>,
    clusters: &Clusters,
    options: &forest::Options,
) -> Result<RegressionForest, &'static str> {
    let y = columns.get(outcome).ok_or("Invalid regression outcome.")?;
    let n = y.len();
    if n == 0
        || clusters.row_count() != n
        || columns
            .iter()
            .any(|c| c.len() != n || c.iter().any(|v| v.is_infinite()))
        || y.iter().any(|v| !v.is_finite())
        || weight.is_some_and(|w| w >= columns.len() || w == outcome)
        || options.tree.min_node_size == 0
        || !options.tree.alpha.is_finite()
        || !(0.0..0.5).contains(&options.tree.alpha)
        || !options.tree.imbalance_penalty.is_finite()
        || options.tree.imbalance_penalty < 0.0
    {
        return Err("Invalid regression forest data or options.");
    }
    if let tree::Honesty::Enabled { fraction, .. } = options.tree.honesty {
        if !fraction.is_finite() || fraction <= 0.0 || fraction >= 1.0 {
            return Err("Invalid honesty fraction.");
        }
    }
    let unit = vec![1.0; n];
    let weights = weight.map(|w| columns[w].as_slice()).unwrap_or(&unit);
    if weights.iter().any(|w| !w.is_finite() || *w < 0.0) {
        return Err("Invalid regression weights.");
    }
    let mut excluded = BTreeSet::from([outcome]);
    if let Some(w) = weight {
        excluded.insert(w);
    }
    if excluded.len() == columns.len() {
        return Err("No regression covariates.");
    }
    let trees = forest::train_shared(n, clusters, options, |rng, selected| {
        tree::train_shared(
            columns,
            rng,
            clusters,
            selected,
            &options.tree,
            &excluded,
            |rows| Ok(Some(rows.iter().map(|&i| y[i]).collect())),
            |rows, candidates, responses| {
                Ok(split(
                    columns,
                    weights,
                    responses,
                    rows,
                    candidates,
                    options.tree.alpha,
                    options.tree.imbalance_penalty,
                ))
            },
            |rows| {
                if rows.is_empty() {
                    return Ok(None);
                }
                let mut b = Bin::default();
                for &i in rows {
                    b.sample(weights[i], y[i]);
                }
                Ok(if b.weight.abs() <= 1e-16 {
                    None
                } else {
                    Some([b.sum / rows.len() as f64, b.weight / rows.len() as f64])
                })
            },
        )
    })?;
    Ok(RegressionForest { trees, rows: n })
}
impl RegressionForest {
    /// RegressionPredictionStrategy::compute_error: finite-forest variance is
    /// subtracted from squared OOB prediction error before tuning averages it.
    pub fn oob_errors(
        &self,
        columns: &[Vec<f64>],
        outcome: &[f64],
    ) -> Result<Vec<Option<(f64, f64)>>, &'static str> {
        if outcome.len() != self.rows || outcome.iter().any(|v| !v.is_finite()) {
            return Err("Invalid regression error outcome.");
        }
        let predictions = self.predict(columns, true)?;
        (0..self.rows)
            .map(|i| {
                let Some(prediction) = predictions[i] else {
                    return Ok(None);
                };
                let mut leaves = vec![];
                for tree in &self.trees {
                    if !tree.drawn_samples.contains(&i) {
                        if let Some(m) = tree.nodes[tree.leaf(columns, i)?].moments {
                            leaves.push(m);
                        }
                    }
                }
                let count = leaves.len();
                if count <= 1 {
                    return Ok(None);
                }
                let weight = leaves.iter().map(|m| m[1]).sum::<f64>() / count as f64;
                let bias = leaves
                    .iter()
                    .map(|m| ((m[0] - prediction * m[1]) / weight).powi(2))
                    .sum::<f64>()
                    / (count * (count - 1)) as f64;
                let error = (prediction - outcome[i]).powi(2) - bias;
                Ok(if error.is_finite() && bias.is_finite() {
                    Some((error, bias))
                } else {
                    None
                })
            })
            .collect()
    }
    pub fn predict(
        &self,
        columns: &[Vec<f64>],
        oob: bool,
    ) -> Result<Vec<Option<f64>>, &'static str> {
        let n = columns.first().ok_or("No prediction columns.")?.len();
        if n == 0
            || (oob && n != self.rows)
            || columns
                .iter()
                .any(|c| c.len() != n || c.iter().any(|v| v.is_infinite()))
        {
            return Err("Invalid regression prediction data.");
        }
        (0..n)
            .map(|i| {
                let (mut sum, mut weight, mut count) = (0.0, 0.0, 0);
                for tree in &self.trees {
                    if oob && tree.drawn_samples.contains(&i) {
                        continue;
                    }
                    if let Some(m) = tree.nodes[tree.leaf(columns, i)?].moments {
                        sum += m[0];
                        weight += m[1];
                        count += 1;
                    }
                }
                if count == 0 {
                    return Ok(None);
                }
                // Preserve upstream normalization before the ratio.
                let value = (sum / count as f64) / (weight / count as f64);
                Ok(value.is_finite().then_some(value))
            })
            .collect()
    }
}
