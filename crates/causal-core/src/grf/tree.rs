// Copyright (c) 2024 GRF Contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
// GRF 2.6.1 TreeTrainer and Tree, causal strategy. See COPYING.
use super::{
    prediction::{CausalData, Moments},
    sampling::{Clusters, Sampler},
    splitting::{Split, ValidatedInputs},
};
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
pub enum Honesty {
    Disabled,
    Enabled { fraction: f64, prune: bool },
}
#[derive(Clone)]
pub struct Options {
    pub mtry: u32,
    pub min_node_size: usize,
    pub honesty: Honesty,
    pub alpha: f64,
    pub imbalance_penalty: f64,
}
/// Preserve the upstream column layout: excluded columns affect feature draws.
#[derive(Clone, Copy)]
pub struct Data<'a> {
    pub columns: &'a [Vec<f64>],
    pub outcome: usize,
    pub treatment: usize,
    pub weight: Option<usize>,
}
#[derive(Debug)]
pub struct Node<M = Moments> {
    pub children: Option<[usize; 2]>,
    pub split: Option<Split>,
    pub samples: Vec<usize>,
    pub moments: Option<M>,
}
impl<M> Node<M> {
    fn empty() -> Self {
        Self {
            children: None,
            split: None,
            samples: vec![],
            moments: None,
        }
    }
}
#[derive(Debug)]
pub struct Tree<M = Moments> {
    pub root: usize,
    pub nodes: Vec<Node<M>>,
    pub drawn_samples: Vec<usize>,
}
fn goes_left(value: f64, split: Split) -> bool {
    value <= split.value || (value.is_nan() && (split.missing_left || split.value.is_nan()))
}
impl<M> Tree<M> {
    pub fn leaf(&self, columns: &[Vec<f64>], row: usize) -> Result<usize, &'static str> {
        let mut node = self.root;
        while let Some(children) = self.nodes[node].children {
            let split = self.nodes[node].split.ok_or("Branch has no split.")?;
            let value = *columns
                .get(split.variable)
                .and_then(|c| c.get(row))
                .ok_or("Prediction row or column is missing.")?;
            if value.is_infinite() {
                return Err("Infinite prediction covariate.");
            }
            node = children[usize::from(!goes_left(value, split))];
        }
        Ok(node)
    }
    fn empty_leaf(&self, n: usize) -> bool {
        self.nodes[n].children.is_none() && self.nodes[n].samples.is_empty()
    }
    fn prune_node(&mut self, n: usize) -> usize {
        let Some([left, right]) = self.nodes[n].children else {
            return n;
        };
        if self.empty_leaf(left) || self.empty_leaf(right) {
            self.nodes[n].children = None;
            if !self.empty_leaf(left) {
                return left;
            }
            if !self.empty_leaf(right) {
                return right;
            }
        }
        n
    }
    fn prune(&mut self) {
        for n in (self.root..self.nodes.len()).rev() {
            if let Some(mut children) = self.nodes[n].children {
                for child in &mut children {
                    if self.nodes[*child].children.is_some() {
                        *child = self.prune_node(*child);
                    }
                }
                self.nodes[n].children = Some(children);
            }
        }
        self.root = self.prune_node(self.root);
    }
}

/// Selected cluster IDs and the sampler are supplied by the forest's group
/// scheduler. No reseeding is done here, including between trees in a CI group.
pub fn train(
    data: Data<'_>,
    sampler: &mut Sampler,
    clusters: &Clusters,
    selected: &[usize],
    options: &Options,
) -> Result<Tree, &'static str> {
    train_with_stabilization(data, sampler, clusters, selected, options, true)
}
pub(crate) fn train_with_stabilization(
    data: Data<'_>,
    sampler: &mut Sampler,
    clusters: &Clusters,
    selected: &[usize],
    options: &Options,
    stabilize: bool,
) -> Result<Tree, &'static str> {
    let p = data.columns.len();
    if data.outcome >= p
        || data.treatment >= p
        || data.outcome == data.treatment
        || data
            .weight
            .is_some_and(|w| w >= p || w == data.outcome || w == data.treatment)
    {
        return Err("Invalid tree data roles.");
    }
    let n = data.columns[data.outcome].len();
    if clusters.row_count() != n {
        return Err("Cluster labels must cover every data row.");
    }
    if n == 0
        || data
            .columns
            .iter()
            .any(|c| c.len() != n || c.iter().any(|v| v.is_infinite()))
        || options.min_node_size == 0
        || !options.alpha.is_finite()
        || !(0.0..0.5).contains(&options.alpha)
        || !options.imbalance_penalty.is_finite()
        || options.imbalance_penalty < 0.0
    {
        return Err("Invalid tree data or options.");
    }
    if let Honesty::Enabled { fraction, .. } = options.honesty {
        if !fraction.is_finite() || fraction <= 0.0 || fraction >= 1.0 {
            return Err("Honesty fraction must be between zero and one.");
        }
    }
    let mut excluded = BTreeSet::from([data.outcome, data.treatment]);
    if let Some(w) = data.weight {
        excluded.insert(w);
    }
    let features = p - excluded.len();
    if features == 0 {
        return Err("At least one covariate is required.");
    }
    let unit_weights = vec![1.0; n];
    let weights = data
        .weight
        .map(|w| data.columns[w].as_slice())
        .unwrap_or(&unit_weights);
    let treatment = &data.columns[data.treatment];
    let causal = CausalData::new(&data.columns[data.outcome], treatment, weights)?;
    let split_inputs = ValidatedInputs::new(data.columns, treatment, weights)?;
    train_shared(
        data.columns,
        sampler,
        clusters,
        selected,
        options,
        &excluded,
        |rows| causal.relabel(rows),
        |rows, candidates, responses| {
            if !stabilize {
                return Ok(super::regression::split(
                    data.columns,
                    weights,
                    responses,
                    rows,
                    candidates,
                    options.alpha,
                    options.imbalance_penalty,
                ));
            }
            split_inputs.best_split(
                responses,
                rows,
                candidates,
                options.min_node_size,
                options.alpha,
                options.imbalance_penalty,
            )
        },
        |rows| causal.leaf_moments(rows),
    )
}

pub(crate) fn train_shared<M>(
    columns: &[Vec<f64>],
    sampler: &mut Sampler,
    clusters: &Clusters,
    selected: &[usize],
    options: &Options,
    excluded: &BTreeSet<usize>,
    mut relabel: impl FnMut(&[usize]) -> Result<Option<Vec<f64>>, &'static str>,
    mut split: impl FnMut(&[usize], &[usize], &[f64]) -> Result<Option<Split>, &'static str>,
    mut moments: impl FnMut(&[usize]) -> Result<Option<M>, &'static str>,
) -> Result<Tree<M>, &'static str> {
    let n = columns.first().ok_or("No tree columns.")?.len();
    let p = columns.len();
    let features = p - excluded.len();
    let drawn_samples = sampler.rows_in_clusters(clusters, selected, false)?;
    if drawn_samples.is_empty() || drawn_samples.iter().any(|&i| i >= n) {
        return Err("Invalid selected tree rows.");
    }
    let (growing, honest) = match options.honesty {
        Honesty::Disabled => (sampler.rows_in_clusters(clusters, selected, true)?, vec![]),
        Honesty::Enabled { fraction, .. } => {
            let (a, b) = sampler.subsample(selected, fraction)?;
            (
                sampler.rows_in_clusters(clusters, &a, true)?,
                sampler.rows_in_clusters(clusters, &b, true)?,
            )
        }
    };
    let mut tree = Tree {
        root: 0,
        nodes: vec![Node::empty()],
        drawn_samples,
    };
    tree.nodes[0].samples = growing;
    let mut responses = vec![0.0; n];
    let mut node = 0;
    while node < tree.nodes.len() {
        // Upstream draws candidates even for nodes that subsequently stop.
        let count = sampler.poisson(options.mtry).min(features).max(1);
        let candidates = sampler.draw(p, &excluded, count)?;
        let rows = &tree.nodes[node].samples;
        if rows.len() > options.min_node_size {
            if let Some(relabeled) = relabel(rows)? {
                for (&i, value) in rows.iter().zip(relabeled) {
                    responses[i] = value;
                }
                if let Some(split) = split(rows, &candidates, &responses)? {
                    let children = [tree.nodes.len(), tree.nodes.len() + 1];
                    tree.nodes.push(Node::empty());
                    tree.nodes.push(Node::empty());
                    let rows = std::mem::take(&mut tree.nodes[node].samples);
                    for i in rows {
                        let child =
                            children[usize::from(!goes_left(columns[split.variable][i], split))];
                        tree.nodes[child].samples.push(i);
                    }
                    tree.nodes[node].children = Some(children);
                    tree.nodes[node].split = Some(split);
                }
            }
        }
        node += 1;
    }
    // Upstream retains growing leaves when the honest sample is empty.
    if !honest.is_empty() {
        for node in &mut tree.nodes {
            node.samples.clear();
        }
        for i in honest {
            let leaf = tree.leaf(columns, i)?;
            tree.nodes[leaf].samples.push(i);
        }
        if let Honesty::Enabled { prune: true, .. } = options.honesty {
            tree.prune();
        }
    }
    for node in &mut tree.nodes {
        node.moments = moments(&node.samples)?;
    }
    Ok(tree)
}
