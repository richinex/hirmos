//! Survival forest prediction from ranger-compatible fitted trees.
mod maxstat;
mod random;
mod sampling;
mod training;
pub use sampling::{Replacement, Sampling};
pub use training::{train, train_with_sampling, Data, Settings, SplitRule, Training};

#[derive(Clone, Debug)]
pub enum Split {
    Ordered { column: usize, threshold: f64 },
    Categorical { column: usize, right_levels: u64 },
}

#[derive(Clone, Debug)]
pub enum Node {
    Branch {
        split: Split,
        left: usize,
        right: usize,
    },
    Terminal {
        hazard: Vec<f64>,
    },
}

fn goes_right(split: &Split, row: &[f64]) -> Result<bool, Error> {
    match split {
        Split::Ordered { column, threshold } => Ok(row[*column] > *threshold),
        Split::Categorical {
            column,
            right_levels,
        } => {
            let level = row[*column];
            if level < 1.0 || level > 53.0 || level.fract() != 0.0 {
                return Err(Error::InvalidCategory);
            }
            Ok(right_levels & (1_u64 << (level as u32 - 1)) != 0)
        }
    }
}

fn terminal_node(tree: &[Node], row: &[f64]) -> Result<usize, Error> {
    let mut id = 0;
    loop {
        match &tree[id] {
            Node::Terminal { .. } => return Ok(id),
            Node::Branch { split, left, right } => {
                id = if goes_right(split, row)? {
                    *right
                } else {
                    *left
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    EmptyForest,
    InvalidTimes,
    InvalidTree,
    InvalidHazard,
    InvalidRow,
    InvalidCategory,
    InvalidInbag,
    InvalidSettings,
}

/// Forest validated before traversing child indices or reading hazard arrays.
pub struct Forest {
    times: Vec<f64>,
    columns: usize,
    trees: Vec<Vec<Node>>,
}

pub struct Prediction {
    pub terminal_nodes: Vec<usize>,
    pub hazard: Vec<f64>,
    pub survival: Vec<f64>,
}

impl Forest {
    pub fn new(times: Vec<f64>, columns: usize, trees: Vec<Vec<Node>>) -> Result<Self, Error> {
        if trees.is_empty() || columns == 0 {
            return Err(Error::EmptyForest);
        }
        if times.is_empty()
            || times.iter().any(|x| !x.is_finite())
            || times.windows(2).any(|w| w[0] >= w[1])
        {
            return Err(Error::InvalidTimes);
        }
        for tree in &trees {
            if tree.is_empty() {
                return Err(Error::InvalidTree);
            }
            let mut parents = vec![0; tree.len()];
            for (id, node) in tree.iter().enumerate() {
                match node {
                    Node::Terminal { hazard } => {
                        if hazard.len() != times.len()
                            || hazard.iter().any(|v| !v.is_finite() || *v < 0.0)
                            || hazard.windows(2).any(|w| w[0] > w[1])
                        {
                            return Err(Error::InvalidHazard);
                        }
                    }
                    Node::Branch { split, left, right } => {
                        // ranger assigns new child IDs after the parent. This also rules out cycles.
                        if *left <= id
                            || *right <= id
                            || *left >= tree.len()
                            || *right >= tree.len()
                            || left == right
                        {
                            return Err(Error::InvalidTree);
                        }
                        parents[*left] += 1;
                        parents[*right] += 1;
                        let column = match split {
                            Split::Ordered { column, threshold } => {
                                if !threshold.is_finite() {
                                    return Err(Error::InvalidTree);
                                }
                                *column
                            }
                            Split::Categorical { column, .. } => *column,
                        };
                        if column >= columns {
                            return Err(Error::InvalidTree);
                        }
                    }
                }
            }
            if parents[1..].iter().any(|count| *count != 1) {
                return Err(Error::InvalidTree);
            }
        }
        Ok(Self {
            times,
            columns,
            trees,
        })
    }

    pub fn times(&self) -> &[f64] {
        &self.times
    }

    pub fn trees(&self) -> &[Vec<Node>] {
        &self.trees
    }

    /// Predict by averaging cumulative hazards, then exponentiating their negative.
    pub fn predict(&self, row: &[f64]) -> Result<Prediction, Error> {
        let nodes = self.terminal_nodes(row)?;
        let hazard = self.average_hazard(&nodes, |_| true).unwrap();
        let survival = hazard.iter().map(|h| (-h).exp()).collect();
        Ok(Prediction {
            terminal_nodes: nodes,
            hazard,
            survival,
        })
    }

    /// Return no estimate when every tree included this row in its training sample.
    pub fn predict_out_of_bag(
        &self,
        row: &[f64],
        inbag: &[usize],
    ) -> Result<Option<Prediction>, Error> {
        if inbag.len() != self.trees.len() {
            return Err(Error::InvalidInbag);
        }
        let nodes = self.terminal_nodes(row)?;
        let Some(hazard) = self.average_hazard(&nodes, |tree| inbag[tree] == 0) else {
            return Ok(None);
        };
        let survival = hazard.iter().map(|h| (-h).exp()).collect();
        Ok(Some(Prediction {
            terminal_nodes: nodes,
            hazard,
            survival,
        }))
    }

    fn terminal_nodes(&self, row: &[f64]) -> Result<Vec<usize>, Error> {
        if row.len() != self.columns || row.iter().any(|v| !v.is_finite()) {
            return Err(Error::InvalidRow);
        }
        self.trees
            .iter()
            .map(|tree| terminal_node(tree, row))
            .collect()
    }

    fn average_hazard(&self, nodes: &[usize], include: impl Fn(usize) -> bool) -> Option<Vec<f64>> {
        let mut count = 0;
        let mut hazard = vec![0.0; self.times.len()];
        for (tree, &node) in nodes.iter().enumerate() {
            if !include(tree) {
                continue;
            }
            let Node::Terminal { hazard: values } = &self.trees[tree][node] else {
                unreachable!("validated traversal ends at a terminal node")
            };
            for (sum, value) in hazard.iter_mut().zip(values) {
                *sum += value;
            }
            count += 1;
        }
        if count == 0 {
            return None;
        }
        for value in &mut hazard {
            *value /= count as f64;
        }
        Some(hazard)
    }
}
