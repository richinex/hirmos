//! Best-first numeric histogram trees, without missing training values or constraints.
//! Derived from scikit-learn 1.9.0 (BSD-3-Clause); see the preserved COPYING.

use super::{
    histogram::{subtract, Bin, Feature, UnitDerivatives},
    split::{Child, Curvature, Search, Split},
};
use std::num::NonZeroUsize;

#[derive(Debug)]
pub enum TreeError {
    Shape,
    NonFinite,
    Histogram,
    Split,
}

#[derive(Clone)]
pub enum Node {
    Leaf(f64),
    Branch {
        feature: usize,
        bin: usize,
        left: usize,
        right: usize,
        missing_left: bool,
    },
}
#[derive(Clone)]
pub struct Tree {
    nodes: Vec<Node>,
    columns: usize,
}
struct Candidate {
    node: usize,
    rows: Vec<usize>,
    histogram: Vec<Vec<Bin>>,
    feature: usize,
    split: Split,
}

// Match heapq's strict comparison and right-child choice on equal gains.
fn push(heap: &mut Vec<Candidate>, value: Candidate) {
    heap.push(value);
    let mut pos = heap.len() - 1;
    while pos > 0 {
        let parent = (pos - 1) / 2;
        if heap[pos].split.gain <= heap[parent].split.gain {
            break;
        }
        heap.swap(pos, parent);
        pos = parent;
    }
}
fn pop(heap: &mut Vec<Candidate>) -> Candidate {
    let last = heap.pop().unwrap();
    if heap.is_empty() {
        return last;
    }
    let result = std::mem::replace(&mut heap[0], last);
    let mut pos = 0;
    while 2 * pos + 1 < heap.len() {
        let mut child = 2 * pos + 1;
        if child + 1 < heap.len() && heap[child].split.gain <= heap[child + 1].split.gain {
            child += 1;
        }
        heap.swap(pos, child);
        pos = child;
    }
    while pos > 0 {
        let parent = (pos - 1) / 2;
        if heap[pos].split.gain <= heap[parent].split.gain {
            break;
        }
        heap.swap(pos, parent);
        pos = parent;
    }
    result
}

fn candidate(
    node: usize,
    rows: Vec<usize>,
    histogram: Vec<Vec<Bin>>,
    totals: Child,
    bins: &[usize],
    search: &Search,
    min_leaf: usize,
) -> Result<Option<Candidate>, TreeError> {
    if rows.len() < min_leaf.saturating_mul(2) {
        return Ok(None);
    }
    let mut best: Option<(usize, Split)> = None;
    for (feature, values) in histogram.iter().enumerate() {
        let split = search
            .find(&values[..bins[feature]], totals, Curvature::Unit)
            .map_err(|_| TreeError::Split)?;
        if let Some(split) = split {
            if best.as_ref().map_or(true, |(_, b)| split.gain > b.gain) {
                best = Some((feature, split));
            }
        }
    }
    Ok(best.map(|(feature, split)| Candidate {
        node,
        rows,
        histogram,
        feature,
        split,
    }))
}

impl Tree {
    /// Squared-error trees use constant unit Hessians and source-default split thresholds.
    pub fn fit(
        columns: &[Vec<u8>],
        bins: &[usize],
        width: usize,
        derivatives: &UnitDerivatives<'_>,
        min_leaf: NonZeroUsize,
        max_leaves: NonZeroUsize,
        l2: f64,
        shrinkage: f64,
    ) -> Result<Self, TreeError> {
        if columns.is_empty() || bins.len() != columns.len() || max_leaves.get() < 2 {
            return Err(TreeError::Shape);
        }
        if !shrinkage.is_finite() || shrinkage <= 0. {
            return Err(TreeError::NonFinite);
        }
        if bins.iter().any(|&b| b == 0 || b >= width) {
            return Err(TreeError::Shape);
        }
        if columns
            .iter()
            .zip(bins)
            .any(|(col, &b)| col.iter().any(|&v| v as usize >= b))
        {
            return Err(TreeError::Shape);
        }
        let features: Vec<_> = columns
            .iter()
            .map(|c| {
                Feature::new(c, derivatives.derivatives(), width).map_err(|_| TreeError::Shape)
            })
            .collect::<Result<_, _>>()?;
        let histogram: Vec<_> = features.iter().map(Feature::root).collect();
        let count = columns[0].len();
        let gradient = crate::numpy_reduce::numpy_sum(
            &histogram[0].iter().map(|b| b.gradient).collect::<Vec<_>>(),
        );
        let totals = Child {
            gradient,
            hessian: count as f64,
            count: count as u32,
            value: 0.,
        };
        let search = Search::new(min_leaf, 1e-3, 0., l2).map_err(|_| TreeError::Split)?;
        let mut nodes = vec![Node::Leaf(0.)];
        let mut queue = Vec::new();
        if let Some(root) = candidate(
            0,
            (0..count).collect(),
            histogram,
            totals,
            bins,
            &search,
            min_leaf.get(),
        )? {
            push(&mut queue, root);
        }
        let mut leaves = 1;
        while !queue.is_empty() && leaves < max_leaves.get() {
            let parent = pop(&mut queue);
            let (left_rows, right_rows): (Vec<_>, Vec<_>) = parent
                .rows
                .into_iter()
                .partition(|&r| columns[parent.feature][r] as usize <= parent.split.bin);
            let left = nodes.len();
            let right = left + 1;
            nodes.push(Node::Leaf(parent.split.left.value));
            nodes.push(Node::Leaf(parent.split.right.value));
            nodes[parent.node] = Node::Branch {
                feature: parent.feature,
                bin: parent.split.bin,
                left,
                right,
                missing_left: left_rows.len() > right_rows.len(),
            };
            leaves += 1;
            if leaves == max_leaves.get() {
                break;
            }
            let small_left = left_rows.len() < right_rows.len();
            let small = if small_left { &left_rows } else { &right_rows };
            let small_hist: Vec<_> = features
                .iter()
                .map(|f| f.node(small).map_err(|_| TreeError::Histogram))
                .collect::<Result<_, _>>()?;
            let mut large_hist = parent.histogram;
            for (large, small) in large_hist.iter_mut().zip(&small_hist) {
                subtract(large, small).map_err(|_| TreeError::Histogram)?;
            }
            let (left_hist, right_hist) = if small_left {
                (small_hist, large_hist)
            } else {
                (large_hist, small_hist)
            };
            if let Some(c) = candidate(
                left,
                left_rows,
                left_hist,
                parent.split.left,
                bins,
                &search,
                min_leaf.get(),
            )? {
                push(&mut queue, c);
            }
            if let Some(c) = candidate(
                right,
                right_rows,
                right_hist,
                parent.split.right,
                bins,
                &search,
                min_leaf.get(),
            )? {
                push(&mut queue, c);
            }
        }
        for node in &mut nodes {
            if let Node::Leaf(value) = node {
                *value *= shrinkage;
            }
        }
        Ok(Self {
            nodes,
            columns: columns.len(),
        })
    }
    pub fn predict(&self, row: &[u8], missing: u8) -> Result<f64, TreeError> {
        if row.len() != self.columns {
            return Err(TreeError::Shape);
        }
        let mut node = 0;
        loop {
            match self.nodes[node] {
                Node::Leaf(value) => return Ok(value),
                Node::Branch {
                    feature,
                    bin,
                    left,
                    right,
                    missing_left,
                } => {
                    let goes_left = if row[feature] == missing {
                        missing_left
                    } else {
                        row[feature] as usize <= bin
                    };
                    node = if goes_left { left } else { right };
                }
            }
        }
    }
}
