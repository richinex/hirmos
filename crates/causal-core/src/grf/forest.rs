// Copyright (c) 2024 GRF Contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
// Source port of ForestTrainer scheduling and OptimizedPredictionCollector.
use super::{
    prediction,
    sampling::{Clusters, Sampler},
    tree::{self, Data, Tree},
};
use crate::mt19937_64::Mt19937_64;

#[derive(Clone, Copy)]
pub enum SeedMode {
    Indexed,
    Legacy,
}
#[derive(Clone)]
pub struct Options {
    pub trees: u32,
    pub group_size: usize,
    pub sample_fraction: f64,
    pub seed: u32,
    pub seed_mode: SeedMode,
    /// Reproduce upstream batch boundaries; execution itself is serial.
    pub batches: usize,
    pub tree: tree::Options,
}
pub struct Forest {
    pub trees: Vec<Tree>,
    pub group_size: usize,
    training_rows: usize,
}
#[derive(Debug)]
pub enum Variance {
    NotRequested,
    Unavailable,
    Estimate(f64),
}
#[derive(Debug)]
pub struct Prediction {
    pub effect: Option<f64>,
    pub variance: Variance,
    pub contributing_trees: usize,
}

pub fn train(
    data: Data<'_>,
    clusters: &Clusters,
    options: &Options,
) -> Result<Forest, &'static str> {
    train_with_stabilization(data, clusters, options, true)
}
pub fn train_with_stabilization(
    data: Data<'_>,
    clusters: &Clusters,
    options: &Options,
    stabilize: bool,
) -> Result<Forest, &'static str> {
    let n = data
        .columns
        .get(data.outcome)
        .ok_or("Invalid outcome column.")?
        .len();
    let trees = train_shared(n, clusters, options, |sampler, selected| {
        tree::train_with_stabilization(data, sampler, clusters, selected, &options.tree, stabilize)
    })?;
    Ok(Forest {
        trees,
        group_size: options.group_size,
        training_rows: n,
    })
}
pub(crate) fn train_shared<T>(
    n: usize,
    clusters: &Clusters,
    options: &Options,
    mut build: impl FnMut(&mut Sampler, &[usize]) -> Result<T, &'static str>,
) -> Result<Vec<T>, &'static str> {
    let group = options.group_size;
    if options.trees == 0
        || group == 0
        || options.batches == 0
        || !options.sample_fraction.is_finite()
        || options.sample_fraction <= 0.0
        || options.sample_fraction > 1.0
        || (group > 1 && options.sample_fraction > 0.5)
    {
        return Err("Invalid forest size, sampling fraction or batch count.");
    }
    if (n as f64 * options.sample_fraction) < 1.0 {
        return Err("Sampling fraction leaves no rows.");
    }
    if let tree::Honesty::Enabled { fraction, .. } = options.tree.honesty {
        if n as f64 * options.sample_fraction * fraction < 1.0
            || n as f64 * options.sample_fraction * (1.0 - fraction) < 1.0
        {
            return Err("Honesty fraction leaves no growing or estimation rows.");
        }
    }
    // Preserve ForestOptions' actual expression, including nondefault group sizes.
    let rounded = (options.trees as usize)
        .checked_add(options.trees as usize % group)
        .ok_or("Forest size overflow.")?;
    let groups = rounded / group;
    if groups == 0 {
        return Err("No complete tree groups requested.");
    }
    let batches = options.batches.min(groups);
    let mut trees = Vec::new();
    let mut start = 0;
    for batch in 0..batches {
        let length = groups / batches + usize::from(batch < groups % batches);
        let mut legacy = Mt19937_64::new(options.seed as u64 + start as u64);
        for i in 0..length {
            let seed = match options.seed_mode {
                SeedMode::Indexed => options.seed.wrapping_add((start + i) as u32),
                SeedMode::Legacy => legacy.next() as u32,
            };
            let mut sampler = Sampler::new(seed);
            if group == 1 {
                let selected = sampler.sample_clusters(clusters, options.sample_fraction)?;
                trees.push(build(&mut sampler, &selected)?);
            } else {
                let half = sampler.sample_clusters(clusters, 0.5)?;
                for _ in 0..group {
                    let (selected, _) = sampler.subsample(&half, 2.0 * options.sample_fraction)?;
                    trees.push(build(&mut sampler, &selected)?);
                }
            }
        }
        start += length;
    }
    Ok(trees)
}
impl Forest {
    fn row_leaves(
        &self,
        columns: &[Vec<f64>],
        row: usize,
        oob: bool,
    ) -> Result<Vec<Option<prediction::Moments>>, &'static str> {
        self.trees
            .iter()
            .map(|tree| {
                if oob && tree.drawn_samples.contains(&row) {
                    Ok(None)
                } else {
                    Ok(tree.nodes[tree.leaf(columns, row)?].moments)
                }
            })
            .collect()
    }
    pub fn oob_errors(
        &self,
        columns: &[Vec<f64>],
        outcome: &[f64],
        treatment: &[f64],
    ) -> Result<Vec<Option<(f64, f64)>>, &'static str> {
        let n = self.training_rows;
        if outcome.len() != n
            || treatment.len() != n
            || columns.iter().any(|c| c.len() != n)
            || outcome.iter().chain(treatment).any(|v| !v.is_finite())
        {
            return Err("Invalid OOB error observations.");
        }
        (0..n)
            .map(|i| {
                let leaves = self.row_leaves(columns, i, true)?;
                let mut mean = [0.0; 7];
                let mut count = 0;
                for m in leaves.iter().flatten() {
                    count += 1;
                    for j in 0..7 {
                        mean[j] += m[j];
                    }
                }
                if count == 0 {
                    return Ok(None);
                }
                for v in &mut mean {
                    *v /= count as f64;
                }
                Ok(prediction::error(outcome[i], treatment[i], &mean, &leaves))
            })
            .collect()
    }
    /// Sparse forest kernel weights, without multiplying by observation weights.
    pub fn weights(
        &self,
        columns: &[Vec<f64>],
        oob: bool,
    ) -> Result<Vec<Vec<(usize, f64)>>, &'static str> {
        let n = columns.first().ok_or("No prediction columns.")?.len();
        if columns.iter().any(|c| c.len() != n) || (oob && n != self.training_rows) {
            return Err("Invalid forest weight dimensions.");
        }
        (0..n)
            .map(|i| {
                let mut pool = vec![0.0; self.training_rows];
                let mut seen = Vec::new();
                for tree in &self.trees {
                    if oob && tree.drawn_samples.contains(&i) {
                        continue;
                    }
                    let leaf = &tree.nodes[tree.leaf(columns, i)?].samples;
                    for &j in leaf {
                        if pool[j] <= 0.0 {
                            seen.push(j);
                        }
                        pool[j] += 1.0 / leaf.len() as f64;
                    }
                }
                let total: f64 = seen.iter().map(|&j| pool[j]).sum();
                Ok(seen.into_iter().map(|j| (j, pool[j] / total)).collect())
            })
            .collect()
    }
    pub fn split_frequencies(
        &self,
        columns: usize,
        max_depth: usize,
    ) -> Result<Vec<Vec<usize>>, &'static str> {
        if columns == 0 || max_depth == 0 {
            return Err("Positive feature count and depth required.");
        }
        let mut counts = vec![vec![0; columns]; max_depth];
        for tree in &self.trees {
            let mut level = vec![tree.root];
            for row in &mut counts {
                let mut next = Vec::new();
                for node in level {
                    if let Some(children) = tree.nodes[node].children {
                        let var = tree.nodes[node]
                            .split
                            .ok_or("Branch lacks split.")?
                            .variable;
                        *row.get_mut(var)
                            .ok_or("Feature index exceeds frequency columns.")? += 1;
                        next.extend(children);
                    }
                }
                level = next;
                if level.is_empty() {
                    break;
                }
            }
        }
        Ok(counts)
    }
    pub fn variable_importance(
        &self,
        columns: usize,
        max_depth: usize,
        exponent: f64,
    ) -> Result<Vec<f64>, &'static str> {
        if !exponent.is_finite() {
            return Err("Invalid depth decay exponent.");
        }
        let counts = self.split_frequencies(columns, max_depth)?;
        let mut result = vec![0.0; columns];
        let mut mass = 0.0;
        for (depth, row) in counts.iter().enumerate() {
            let w = ((depth + 1) as f64).powf(-exponent);
            mass += w;
            let sum = row.iter().sum::<usize>().max(1) as f64;
            for i in 0..columns {
                result[i] += row[i] as f64 / sum * w;
            }
        }
        for v in &mut result {
            *v /= mass;
        }
        Ok(result)
    }
    /// OOB rows must retain original training row order. In-bag membership uses
    /// every row of a sampled cluster, not merely rows selected within it.
    pub fn predict(
        &self,
        columns: &[Vec<f64>],
        oob: bool,
        uncertainty: bool,
    ) -> Result<Vec<Prediction>, &'static str> {
        let n = columns.first().ok_or("No prediction covariates.")?.len();
        if n == 0
            || columns
                .iter()
                .any(|c| c.len() != n || c.iter().any(|v| v.is_infinite()))
            || (oob && n != self.training_rows)
        {
            return Err("Invalid prediction dimensions or OOB row count.");
        }
        if uncertainty && self.group_size < 2 {
            return Err("Variance requires CI groups of at least two trees.");
        }
        let mut result = Vec::with_capacity(n);
        for row in 0..n {
            let leaves = self.row_leaves(columns, row, oob)?;
            let mut average = [0.0; 7];
            let mut count = 0;
            for moments in &leaves {
                if let Some(m) = moments {
                    count += 1;
                    for i in 0..7 {
                        average[i] += m[i];
                    }
                }
            }
            if count > 0 {
                for v in &mut average {
                    *v /= count as f64;
                }
            }
            let effect = if count == 0 {
                None
            } else {
                let v = prediction::predict(&average);
                v.is_finite().then_some(v)
            };
            let variance = if !uncertainty {
                Variance::NotRequested
            } else if count == 0
                || !leaves
                    .chunks_exact(self.group_size)
                    .any(|g| g.iter().all(Option::is_some))
            {
                Variance::Unavailable
            } else {
                let v = prediction::variance(&average, &leaves, self.group_size)?;
                if v.is_finite() {
                    Variance::Estimate(v)
                } else {
                    Variance::Unavailable
                }
            };
            result.push(Prediction {
                effect,
                variance,
                contributing_trees: count,
            });
        }
        Ok(result)
    }
}
