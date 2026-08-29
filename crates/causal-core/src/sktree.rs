//! sklearn's decision trees and random forests ported 1:1 from the 1.9 Cython sources: the
//! xorshift rand_r stream, three-way introsort, BestSplitter with constant-feature bookkeeping,
//! MSE and Gini criteria, the depth-first builder, and MT19937-seeded bootstrap forests.

use crate::nprandom::Mt19937;

const RAND_R_MAX: u32 = 2147483647;
const FEATURE_THRESHOLD: f32 = 1e-7;
const EPSILON: f64 = f64::EPSILON;

fn our_rand_r(seed: &mut u32) -> u32 {
    if *seed == 0 {
        *seed = 1;
    }
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed % (RAND_R_MAX + 1)
}

fn rand_int(low: usize, high: usize, seed: &mut u32) -> usize {
    low + (our_rand_r(seed) as usize) % (high - low)
}

// ---------------------------------------------------------------- introsort

fn swap_vi(values: &mut [f32], indices: &mut [usize], i: usize, j: usize) {
    values.swap(i, j);
    indices.swap(i, j);
}

fn median3(values: &[f32], n: usize) -> f32 {
    let a = values[0];
    let b = values[n / 2];
    let c = values[n - 1];
    if a < b {
        if b < c {
            b
        } else if a < c {
            c
        } else {
            a
        }
    } else if b < c {
        if a < c {
            a
        } else {
            c
        }
    } else {
        b
    }
}

fn insertion_sort(values: &mut [f32], indices: &mut [usize]) {
    let n = values.len();
    for i in 1..n {
        let temp_val = values[i];
        let temp_idx = indices[i];
        let mut j = i;
        while j > 0 && values[j - 1] > temp_val {
            values[j] = values[j - 1];
            indices[j] = indices[j - 1];
            j -= 1;
        }
        values[j] = temp_val;
        indices[j] = temp_idx;
    }
}

fn sift_down(values: &mut [f32], indices: &mut [usize], start: usize, end: usize) {
    let mut root = start;
    loop {
        let child = root * 2 + 1;
        let mut maxind = root;
        if child < end && values[maxind] < values[child] {
            maxind = child;
        }
        if child + 1 < end && values[maxind] < values[child + 1] {
            maxind = child + 1;
        }
        if maxind == root {
            break;
        }
        swap_vi(values, indices, root, maxind);
        root = maxind;
    }
}

fn heapsort(values: &mut [f32], indices: &mut [usize]) {
    let n = values.len();
    let mut start = (n - 2) / 2;
    let end = n;
    loop {
        sift_down(values, indices, start, end);
        if start == 0 {
            break;
        }
        start -= 1;
    }
    let mut end = n - 1;
    while end > 0 {
        swap_vi(values, indices, 0, end);
        sift_down(values, indices, 0, end);
        end -= 1;
    }
}

fn introsort_3way(values: &mut [f32], indices: &mut [usize], maxd_in: i64) {
    let mut maxd = maxd_in;
    let mut lo = 0usize;
    let mut n = values.len();
    while n > 15 {
        if maxd <= 0 {
            heapsort(&mut values[lo..lo + n], &mut indices[lo..lo + n]);
            return;
        }
        maxd -= 1;
        let pivot = median3(&values[lo..lo + n], n);
        let mut i = 0usize;
        let mut l = 0usize;
        let mut r = n;
        while i < r {
            if values[lo + i] < pivot {
                swap_vi(values, indices, lo + i, lo + l);
                i += 1;
                l += 1;
            } else if values[lo + i] > pivot {
                r -= 1;
                swap_vi(values, indices, lo + i, lo + r);
            } else {
                i += 1;
            }
        }
        introsort_3way(&mut values[lo..lo + l], &mut indices[lo..lo + l], maxd);
        lo += r;
        n -= r;
    }
    insertion_sort(&mut values[lo..lo + n], &mut indices[lo..lo + n]);
}

fn simultaneous_sort(values: &mut [f32], indices: &mut [usize]) {
    let n = values.len();
    if n == 0 {
        return;
    }
    let maxd = 2 * (n as f64).log2() as i64;
    introsort_3way(values, indices, maxd);
}

// ---------------------------------------------------------------- criterion

enum Kind {
    Mse,
    Gini { n_classes: usize },
}

/// Node statistics computed once at node_reset time, in the pre-sort sample order.
#[derive(Clone)]
struct NodeTotals {
    weighted_n_node: f64,
    sq_sum_total: f64,
    sum_total: Vec<f64>,
}

impl NodeTotals {
    fn compute(
        kind: &Kind,
        y: &[f64],
        w: Option<&[f64]>,
        samples: &[usize],
        start: usize,
        end: usize,
    ) -> NodeTotals {
        let nv = Criterion::n_vals(kind);
        let mut t = NodeTotals {
            weighted_n_node: 0.0,
            sq_sum_total: 0.0,
            sum_total: vec![0.0; nv],
        };
        for p in start..end {
            let i = samples[p];
            let wi = w.map_or(1.0, |w| w[i]);
            match kind {
                Kind::Mse => {
                    let y_ik = y[i];
                    let w_y_ik = wi * y_ik;
                    t.sum_total[0] += w_y_ik;
                    t.sq_sum_total += w_y_ik * y_ik;
                }
                Kind::Gini { .. } => {
                    t.sum_total[y[i] as usize] += wi;
                }
            }
            t.weighted_n_node += wi;
        }
        t
    }
}

struct Criterion<'a> {
    kind: &'a Kind,
    y: &'a [f64],
    w: Option<&'a [f64]>,
    samples: &'a [usize],
    start: usize,
    end: usize,
    pos: usize,
    weighted_n_samples: f64,
    weighted_n_node: f64,
    weighted_n_left: f64,
    weighted_n_right: f64,
    sq_sum_total: f64,
    sum_total: Vec<f64>,
    sum_left: Vec<f64>,
    sum_right: Vec<f64>,
}

impl<'a> Criterion<'a> {
    fn n_vals(kind: &Kind) -> usize {
        match kind {
            Kind::Mse => 1,
            Kind::Gini { n_classes } => *n_classes,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn with_totals(
        kind: &'a Kind,
        y: &'a [f64],
        w: Option<&'a [f64]>,
        weighted_n_samples: f64,
        samples: &'a [usize],
        start: usize,
        end: usize,
        totals: &NodeTotals,
    ) -> Criterion<'a> {
        let nv = Self::n_vals(kind);
        let mut c = Criterion {
            kind,
            y,
            w,
            samples,
            start,
            end,
            pos: start,
            weighted_n_samples,
            weighted_n_node: totals.weighted_n_node,
            weighted_n_left: 0.0,
            weighted_n_right: 0.0,
            sq_sum_total: totals.sq_sum_total,
            sum_total: totals.sum_total.clone(),
            sum_left: vec![0.0; nv],
            sum_right: vec![0.0; nv],
        };
        c.reset();
        c
    }

    fn reset(&mut self) {
        self.pos = self.start;
        self.sum_left.iter_mut().for_each(|v| *v = 0.0);
        self.sum_right.copy_from_slice(&self.sum_total);
        self.weighted_n_left = 0.0;
        self.weighted_n_right = self.weighted_n_node;
    }

    fn reverse_reset(&mut self) {
        self.pos = self.end;
        self.sum_right.iter_mut().for_each(|v| *v = 0.0);
        self.sum_left.copy_from_slice(&self.sum_total);
        self.weighted_n_right = 0.0;
        self.weighted_n_left = self.weighted_n_node;
    }

    fn update(&mut self, new_pos: usize) {
        let pos = self.pos;
        if (new_pos - pos) <= (self.end - new_pos) {
            for p in pos..new_pos {
                let i = self.samples[p];
                let wi = self.w.map_or(1.0, |w| w[i]);
                match self.kind {
                    Kind::Mse => self.sum_left[0] += wi * self.y[i],
                    Kind::Gini { .. } => self.sum_left[self.y[i] as usize] += wi,
                }
                self.weighted_n_left += wi;
            }
        } else {
            self.reverse_reset();
            for p in (new_pos..self.end).rev() {
                let i = self.samples[p];
                let wi = self.w.map_or(1.0, |w| w[i]);
                match self.kind {
                    Kind::Mse => self.sum_left[0] -= wi * self.y[i],
                    Kind::Gini { .. } => self.sum_left[self.y[i] as usize] -= wi,
                }
                self.weighted_n_left -= wi;
            }
        }
        self.weighted_n_right = self.weighted_n_node - self.weighted_n_left;
        for k in 0..self.sum_total.len() {
            self.sum_right[k] = self.sum_total[k] - self.sum_left[k];
        }
        self.pos = new_pos;
    }

    fn node_impurity(&self) -> f64 {
        match self.kind {
            Kind::Mse => {
                let mut imp = self.sq_sum_total / self.weighted_n_node;
                imp -= (self.sum_total[0] / self.weighted_n_node).powi(2);
                imp
            }
            Kind::Gini { n_classes } => {
                let mut sq = 0.0;
                for c in 0..*n_classes {
                    sq += self.sum_total[c] * self.sum_total[c];
                }
                1.0 - sq / (self.weighted_n_node * self.weighted_n_node)
            }
        }
    }

    fn proxy_impurity_improvement(&self) -> f64 {
        match self.kind {
            Kind::Mse => {
                self.sum_left[0] * self.sum_left[0] / self.weighted_n_left
                    + self.sum_right[0] * self.sum_right[0] / self.weighted_n_right
            }
            Kind::Gini { n_classes } => {
                let mut sq_l = 0.0;
                let mut sq_r = 0.0;
                for c in 0..*n_classes {
                    sq_l += self.sum_left[c] * self.sum_left[c];
                    sq_r += self.sum_right[c] * self.sum_right[c];
                }
                -self.weighted_n_right
                    * (1.0 - sq_r / (self.weighted_n_right * self.weighted_n_right))
                    - self.weighted_n_left
                        * (1.0 - sq_l / (self.weighted_n_left * self.weighted_n_left))
            }
        }
    }

    fn children_impurity(&self) -> (f64, f64) {
        match self.kind {
            Kind::Mse => {
                let mut sq_sum_left = 0.0;
                for p in self.start..self.pos {
                    let i = self.samples[p];
                    let wi = self.w.map_or(1.0, |w| w[i]);
                    let y_ik = self.y[i];
                    sq_sum_left += wi * y_ik * y_ik;
                }
                let sq_sum_right = self.sq_sum_total - sq_sum_left;
                let mut left = sq_sum_left / self.weighted_n_left;
                let mut right = sq_sum_right / self.weighted_n_right;
                left -= (self.sum_left[0] / self.weighted_n_left).powi(2);
                right -= (self.sum_right[0] / self.weighted_n_right).powi(2);
                (left, right)
            }
            Kind::Gini { n_classes } => {
                let mut sq_l = 0.0;
                let mut sq_r = 0.0;
                for c in 0..*n_classes {
                    sq_l += self.sum_left[c] * self.sum_left[c];
                    sq_r += self.sum_right[c] * self.sum_right[c];
                }
                (
                    1.0 - sq_l / (self.weighted_n_left * self.weighted_n_left),
                    1.0 - sq_r / (self.weighted_n_right * self.weighted_n_right),
                )
            }
        }
    }

    fn impurity_improvement(&self, parent: f64, left: f64, right: f64) -> f64 {
        self.weighted_n_node / self.weighted_n_samples
            * (parent
                - self.weighted_n_right / self.weighted_n_node * right
                - self.weighted_n_left / self.weighted_n_node * left)
    }

    fn node_value(&self) -> Vec<f64> {
        match self.kind {
            Kind::Mse => vec![self.sum_total[0] / self.weighted_n_node],
            Kind::Gini { n_classes } => (0..*n_classes)
                .map(|c| self.sum_total[c] / self.weighted_n_node)
                .collect(),
        }
    }
}

// ---------------------------------------------------------------- tree

#[derive(Clone)]
pub struct TreeNode {
    pub feature: usize,
    pub threshold: f64,
    pub left: i64,
    pub right: i64,
    pub value: Vec<f64>,
}

pub struct DecisionTree {
    pub nodes: Vec<TreeNode>,
}

pub struct TreeParams {
    pub max_features: usize,
    pub min_samples_leaf: usize,
    pub min_samples_split: usize,
}

struct SplitRecord {
    feature: usize,
    threshold: f64,
    pos: usize,
    improvement: f64,
    impurity_left: f64,
    impurity_right: f64,
}

struct Splitter<'a> {
    x: &'a [f32],
    n_features: usize,
    y: &'a [f64],
    w: Option<&'a [f64]>,
    kind: &'a Kind,
    samples: Vec<usize>,
    features: Vec<usize>,
    constant_features: Vec<usize>,
    feature_values: Vec<f32>,
    weighted_n_samples: f64,
    rand_r_state: u32,
    params: &'a TreeParams,
}

impl<'a> Splitter<'a> {
    #[inline]
    fn xv(&self, sample: usize, feature: usize) -> f32 {
        self.x[sample * self.n_features + feature]
    }

    /// node_split_best without the missing-value passes.
    fn node_split(
        &mut self,
        start: usize,
        end: usize,
        parent_impurity: f64,
        totals: &NodeTotals,
        n_constant_features: &mut usize,
    ) -> Option<SplitRecord> {
        let n_features = self.n_features;
        let max_features = self.params.max_features;
        let min_samples_leaf = self.params.min_samples_leaf;

        let mut best_proxy = f64::NEG_INFINITY;
        let mut best: Option<(usize, f64, usize)> = None; // feature, threshold, pos

        let mut f_i = n_features;
        let mut n_visited_features = 0usize;
        let mut n_found_constants = 0usize;
        let mut n_drawn_constants = 0usize;
        let n_known_constants = *n_constant_features;
        let mut n_total_constants = n_known_constants;

        while f_i > n_total_constants
            && (n_visited_features < max_features
                || n_visited_features <= n_found_constants + n_drawn_constants)
        {
            n_visited_features += 1;
            let mut f_j = rand_int(
                n_drawn_constants,
                f_i - n_found_constants,
                &mut self.rand_r_state,
            );
            if f_j < n_known_constants {
                self.features.swap(n_drawn_constants, f_j);
                n_drawn_constants += 1;
                continue;
            }
            f_j += n_found_constants;
            let current_feature = self.features[f_j];

            // sort_samples_and_feature_values
            for p in start..end {
                self.feature_values[p] = self.xv(self.samples[p], current_feature);
            }
            simultaneous_sort(
                &mut self.feature_values[start..end],
                &mut self.samples[start..end],
            );

            if self.feature_values[end - 1] <= self.feature_values[start] + FEATURE_THRESHOLD {
                self.features.swap(f_j, n_total_constants);
                n_found_constants += 1;
                n_total_constants += 1;
                continue;
            }
            f_i -= 1;
            self.features.swap(f_i, f_j);

            let mut criterion = Criterion::with_totals(
                self.kind,
                self.y,
                self.w,
                self.weighted_n_samples,
                &self.samples,
                start,
                end,
                totals,
            );
            let mut p = start;
            while p < end {
                // next_p
                p += 1;
                while p < end
                    && self.feature_values[p] <= self.feature_values[p - 1] + FEATURE_THRESHOLD
                {
                    p += 1;
                }
                let p_prev = p - 1;
                if p == end {
                    continue;
                }
                let n_left = p - start;
                let n_right = end - p;
                if n_left < min_samples_leaf || n_right < min_samples_leaf {
                    continue;
                }
                criterion.update(p);
                // min_weight_leaf is zero at our configurations.
                let proxy = criterion.proxy_impurity_improvement();
                if proxy > best_proxy {
                    best_proxy = proxy;
                    let threshold = self.feature_values[p_prev] as f64 / 2.0
                        + self.feature_values[p] as f64 / 2.0;
                    best = Some((current_feature, threshold, p));
                }
            }
        }

        let result = if let Some((feature, threshold, pos)) = best {
            // partition_samples_final
            let mut partition_start = start;
            let mut partition_end = end;
            while partition_start < partition_end {
                let value = self.xv(self.samples[partition_start], feature);
                if (value as f64) <= threshold {
                    partition_start += 1;
                } else {
                    partition_end -= 1;
                    self.samples.swap(partition_start, partition_end);
                }
            }
            let mut criterion = Criterion::with_totals(
                self.kind,
                self.y,
                self.w,
                self.weighted_n_samples,
                &self.samples,
                start,
                end,
                totals,
            );
            criterion.update(pos);
            let (impurity_left, impurity_right) = criterion.children_impurity();
            let improvement =
                criterion.impurity_improvement(parent_impurity, impurity_left, impurity_right);
            Some(SplitRecord {
                feature,
                threshold,
                pos,
                improvement,
                impurity_left,
                impurity_right,
            })
        } else {
            None
        };

        // Restore the constant-feature bookkeeping exactly as the memcpy pair does.
        self.features[..n_known_constants]
            .copy_from_slice(&self.constant_features[..n_known_constants]);
        let found: Vec<usize> =
            self.features[n_known_constants..n_known_constants + n_found_constants].to_vec();
        self.constant_features[n_known_constants..n_known_constants + n_found_constants]
            .copy_from_slice(&found);
        *n_constant_features = n_total_constants;
        result
    }
}

struct StackRecord {
    start: usize,
    end: usize,
    depth: usize,
    parent: i64,
    is_left: bool,
    impurity: f64,
    n_constant_features: usize,
}

/// DepthFirstTreeBuilder at sklearn defaults (max_depth None, min_impurity_decrease 0).
pub fn build_tree(
    x: &[f32],
    n_features: usize,
    y: &[f64],
    sample_weight: Option<&[f64]>,
    kind_is_gini: Option<usize>,
    params: &TreeParams,
    tree_seed: u32,
) -> DecisionTree {
    let n_samples_total = if n_features == 0 {
        0
    } else {
        x.len() / n_features
    };
    let kind = match kind_is_gini {
        Some(n_classes) => Kind::Gini { n_classes },
        None => Kind::Mse,
    };
    // Splitter init: rand_r seed from a fresh MT19937 on the tree seed.
    let mut mt = Mt19937::seeded(tree_seed);
    let rand_r_state = mt.randint(RAND_R_MAX as u64) as u32;

    let mut samples = Vec::with_capacity(n_samples_total);
    let mut weighted_n_samples = 0.0;
    for i in 0..n_samples_total {
        match sample_weight {
            Some(w) => {
                if w[i] != 0.0 {
                    samples.push(i);
                }
                weighted_n_samples += w[i];
            }
            None => {
                samples.push(i);
                weighted_n_samples += 1.0;
            }
        }
    }
    let n_node_samples_root = samples.len();

    let mut splitter = Splitter {
        x,
        n_features,
        y,
        w: sample_weight,
        kind: &kind,
        samples,
        features: (0..n_features).collect(),
        constant_features: vec![0; n_features],
        feature_values: vec![0.0; n_samples_total],
        weighted_n_samples,
        rand_r_state,
        params,
    };

    let mut nodes: Vec<TreeNode> = Vec::new();
    let mut stack = vec![StackRecord {
        start: 0,
        end: n_node_samples_root,
        depth: 0,
        parent: -1,
        is_left: false,
        impurity: f64::INFINITY,
        n_constant_features: 0,
    }];
    let mut first = true;

    while let Some(rec) = stack.pop() {
        let n_node_samples = rec.end - rec.start;
        let totals = NodeTotals::compute(
            &kind,
            splitter.y,
            splitter.w,
            &splitter.samples,
            rec.start,
            rec.end,
        );
        let criterion = Criterion::with_totals(
            &kind,
            splitter.y,
            splitter.w,
            splitter.weighted_n_samples,
            &splitter.samples,
            rec.start,
            rec.end,
            &totals,
        );
        let weighted_n_node = criterion.weighted_n_node;
        let mut impurity = rec.impurity;
        if first {
            impurity = criterion.node_impurity();
            first = false;
        }
        let mut is_leaf = n_node_samples < params.min_samples_split
            || n_node_samples < 2 * params.min_samples_leaf
            || impurity <= EPSILON;
        let value = criterion.node_value();
        drop(criterion);

        let mut split: Option<SplitRecord> = None;
        let mut n_constant = rec.n_constant_features;
        if !is_leaf {
            split = splitter.node_split(rec.start, rec.end, impurity, &totals, &mut n_constant);
            match &split {
                Some(s) => {
                    if s.improvement + EPSILON < 0.0 {
                        is_leaf = true;
                    }
                }
                None => is_leaf = true,
            }
        }

        let node_id = nodes.len() as i64;
        nodes.push(TreeNode {
            feature: split.as_ref().map_or(0, |s| s.feature),
            threshold: split.as_ref().map_or(0.0, |s| s.threshold),
            left: -1,
            right: -1,
            value,
        });
        if rec.parent >= 0 {
            let parent = rec.parent as usize;
            if rec.is_left {
                nodes[parent].left = node_id;
            } else {
                nodes[parent].right = node_id;
            }
        }
        let _ = weighted_n_node;

        if !is_leaf {
            let s = split.expect("split exists");
            // Right child first, then left: LIFO pops the left child next.
            stack.push(StackRecord {
                start: s.pos,
                end: rec.end,
                depth: rec.depth + 1,
                parent: node_id,
                is_left: false,
                impurity: s.impurity_right,
                n_constant_features: n_constant,
            });
            stack.push(StackRecord {
                start: rec.start,
                end: s.pos,
                depth: rec.depth + 1,
                parent: node_id,
                is_left: true,
                impurity: s.impurity_left,
                n_constant_features: n_constant,
            });
        }
    }

    DecisionTree { nodes }
}

impl DecisionTree {
    pub fn predict_row(&self, row: &[f32]) -> &[f64] {
        let mut node = 0usize;
        loop {
            let n = &self.nodes[node];
            if n.left < 0 {
                return &n.value;
            }
            if (row[n.feature] as f64) <= n.threshold {
                node = n.left as usize;
            } else {
                node = n.right as usize;
            }
        }
    }
}

// ---------------------------------------------------------------- forest

pub struct RandomForest {
    pub trees: Vec<DecisionTree>,
    pub n_classes: Option<usize>,
}

/// RandomForestRegressor / Classifier at the causal worker's settings: bootstrap on,
/// max_features 1.0 for regression and sqrt for classification.
pub fn fit_forest(
    x_rows: &[Vec<f64>],
    y: &[f64],
    n_estimators: usize,
    min_samples_leaf: usize,
    n_classes: Option<usize>,
    seed: u32,
) -> RandomForest {
    let n_samples = x_rows.len();
    let n_features = x_rows[0].len();
    let x32: Vec<f32> = x_rows
        .iter()
        .flat_map(|r| r.iter().map(|&v| v as f32))
        .collect();

    let max_features = match n_classes {
        None => n_features,
        Some(_) => ((n_features as f64).sqrt() as usize).max(1),
    };
    let params = TreeParams {
        max_features,
        min_samples_leaf,
        min_samples_split: 2,
    };

    let mut forest_rs = Mt19937::seeded(seed);
    let tree_seeds: Vec<u32> = (0..n_estimators)
        .map(|_| forest_rs.randint(2147483647) as u32)
        .collect();

    let mut trees = Vec::with_capacity(n_estimators);
    for &tree_seed in &tree_seeds {
        let mut boot_rs = Mt19937::seeded(tree_seed);
        let mut counts = vec![0.0f64; n_samples];
        for _ in 0..n_samples {
            counts[boot_rs.randint(n_samples as u64) as usize] += 1.0;
        }
        trees.push(build_tree(
            &x32,
            n_features,
            y,
            Some(&counts),
            n_classes,
            &params,
            tree_seed,
        ));
    }
    RandomForest { trees, n_classes }
}

impl RandomForest {
    /// Mean regression prediction, trees accumulated in order as the sequential oracle does.
    pub fn predict(&self, x_rows: &[Vec<f64>]) -> Vec<f64> {
        if x_rows.is_empty() {
            return Vec::new();
        }
        let n_features = x_rows[0].len();
        let x32: Vec<f32> = x_rows
            .iter()
            .flat_map(|row| row.iter().map(|&value| value as f32))
            .collect();
        let mut out = vec![0.0; x_rows.len()];
        for tree in &self.trees {
            for (i, row) in x32.chunks_exact(n_features).enumerate() {
                out[i] += tree.predict_row(row)[0];
            }
        }
        for v in &mut out {
            *v /= self.trees.len() as f64;
        }
        out
    }

    /// Mean class-probability prediction.
    pub fn predict_proba(&self, x_rows: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if x_rows.is_empty() {
            return Vec::new();
        }
        let k = self.n_classes.expect("classifier forest");
        let n_features = x_rows[0].len();
        let x32: Vec<f32> = x_rows
            .iter()
            .flat_map(|row| row.iter().map(|&value| value as f32))
            .collect();
        let mut out = vec![vec![0.0; k]; x_rows.len()];
        for tree in &self.trees {
            for (i, row) in x32.chunks_exact(n_features).enumerate() {
                let value = tree.predict_row(row);
                // Tree stores class fractions; normalize defensively as sklearn does.
                let total: f64 = value.iter().sum();
                for (c, v) in value.iter().enumerate() {
                    out[i][c] += v / total;
                }
            }
        }
        for row in &mut out {
            for v in row.iter_mut() {
                *v /= self.trees.len() as f64;
            }
        }
        out
    }
}
