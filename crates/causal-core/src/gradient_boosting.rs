use crate::nprandom::Mt19937;
use crate::sktree::{build_tree_with_rand_r, DecisionTree, TreeParams, RAND_R_MAX};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GradientBoostingError {
    EmptySample,
    NoColumns,
    RowMismatch,
    NonFiniteValue,
    NotBinary { row: usize },
    Options,
    MissingArm { positive: bool },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    pub n_estimators: usize,
    pub learning_rate: f64,
    pub max_depth: usize,
    pub min_samples_leaf: usize,
    pub min_samples_split: usize,
    pub random_state: u32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            n_estimators: 100,
            learning_rate: 0.1,
            max_depth: 3,
            min_samples_leaf: 1,
            min_samples_split: 2,
            random_state: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GradientBoostingClassifier {
    trees: Vec<DecisionTree>,
    intercept: f64,
    learning_rate: f64,
    columns: usize,
}

fn expit(raw: f64) -> f64 {
    if raw >= 0.0 {
        1.0 / (1.0 + (-raw).exp())
    } else {
        let exponential = raw.exp();
        exponential / (1.0 + exponential)
    }
}

fn safe_divide(numerator: f64, denominator: f64) -> f64 {
    if denominator.abs() < 1e-150 {
        0.0
    } else {
        numerator / denominator
    }
}

fn baseline(observed: &[f64]) -> f64 {
    let epsilon = 10.0 * f64::EPSILON;
    let mean = crate::numpy_reduce::numpy_mean(observed).clamp(epsilon, 1.0 - epsilon);
    (mean / (1.0 - mean)).ln()
}

impl GradientBoostingClassifier {
    pub fn fit(
        design: &[Vec<f64>],
        outcome: &[f64],
        options: &Options,
    ) -> Result<Self, GradientBoostingError> {
        let n = design.len();
        if n == 0 {
            return Err(GradientBoostingError::EmptySample);
        }
        if outcome.len() != n {
            return Err(GradientBoostingError::RowMismatch);
        }
        let columns = design[0].len();
        if columns == 0 {
            return Err(GradientBoostingError::NoColumns);
        }
        if design.iter().any(|row| row.len() != columns) {
            return Err(GradientBoostingError::RowMismatch);
        }
        if design.iter().flatten().any(|value| !value.is_finite()) {
            return Err(GradientBoostingError::NonFiniteValue);
        }
        if let Some(row) = outcome
            .iter()
            .position(|value| *value != 0.0 && *value != 1.0)
        {
            return Err(GradientBoostingError::NotBinary { row });
        }
        if options.n_estimators == 0
            || options.max_depth == 0
            || options.min_samples_leaf == 0
            || options.min_samples_split < 2
            || !options.learning_rate.is_finite()
            || options.learning_rate <= 0.0
        {
            return Err(GradientBoostingError::Options);
        }
        let positives = outcome.iter().filter(|&&v| v == 1.0).count();
        if positives == 0 {
            return Err(GradientBoostingError::MissingArm { positive: true });
        }
        if positives == n {
            return Err(GradientBoostingError::MissingArm { positive: false });
        }

        let flat: Vec<f32> = design
            .iter()
            .flat_map(|row| row.iter().map(|&value| value as f32))
            .collect();
        let params = TreeParams {
            max_features: columns,
            min_samples_leaf: options.min_samples_leaf,
            min_samples_split: options.min_samples_split,
            max_depth: Some(options.max_depth),
        };

        let intercept = baseline(outcome);
        let mut raw = vec![intercept; n];
        let mut rng = Mt19937::seeded(options.random_state);
        let mut trees = Vec::with_capacity(options.n_estimators);

        for _ in 0..options.n_estimators {
            let probability: Vec<f64> = raw.iter().map(|&value| expit(value)).collect();
            let residual: Vec<f64> = outcome
                .iter()
                .zip(&probability)
                .map(|(&y, &p)| y - p)
                .collect();

            let rand_r_state = rng.randint(RAND_R_MAX as u64) as u32;
            let mut tree = build_tree_with_rand_r(
                &flat,
                columns,
                &residual,
                None,
                None,
                &params,
                rand_r_state,
            );

            let terminal: Vec<usize> = (0..n)
                .map(|row| tree.apply_row(&flat[row * columns..(row + 1) * columns]))
                .collect();
            let leaves: Vec<usize> = (0..tree.nodes.len())
                .filter(|&node| tree.nodes[node].left < 0)
                .collect();
            for leaf in leaves {
                let rows: Vec<usize> = (0..n).filter(|&row| terminal[row] == leaf).collect();
                if rows.is_empty() {
                    continue;
                }
                let numerator = crate::numpy_reduce::numpy_mean(
                    &rows.iter().map(|&row| residual[row]).collect::<Vec<_>>(),
                );
                let denominator = crate::numpy_reduce::numpy_mean(
                    &rows
                        .iter()
                        .map(|&row| probability[row] * (1.0 - probability[row]))
                        .collect::<Vec<_>>(),
                );
                tree.nodes[leaf].value[0] = safe_divide(numerator, denominator);
            }
            for row in 0..n {
                raw[row] += options.learning_rate * tree.nodes[terminal[row]].value[0];
            }
            trees.push(tree);
        }

        Ok(Self {
            trees,
            intercept,
            learning_rate: options.learning_rate,
            columns,
        })
    }

    pub fn decision_function(
        &self,
        design: &[Vec<f64>],
    ) -> Result<Vec<f64>, GradientBoostingError> {
        if design.iter().any(|row| row.len() != self.columns) {
            return Err(GradientBoostingError::RowMismatch);
        }
        if design.iter().flatten().any(|value| !value.is_finite()) {
            return Err(GradientBoostingError::NonFiniteValue);
        }
        Ok(design
            .iter()
            .map(|row| {
                let features: Vec<f32> = row.iter().map(|&value| value as f32).collect();
                let mut total = self.intercept;
                for tree in &self.trees {
                    total += self.learning_rate * tree.predict_row(&features)[0];
                }
                total
            })
            .collect())
    }

    pub fn predict_probability(
        &self,
        design: &[Vec<f64>],
    ) -> Result<Vec<f64>, GradientBoostingError> {
        Ok(self.decision_function(design)?.into_iter().map(expit).collect())
    }

    /// The decision function after each of the first `n_estimators` trees, which is how a grid
    /// over that axis is scored without refitting: a prefix of this fit is that shorter fit.
    pub fn staged_decision_function(
        &self,
        design: &[Vec<f64>],
        stages: &[usize],
    ) -> Result<Vec<Vec<f64>>, GradientBoostingError> {
        if design.iter().any(|row| row.len() != self.columns) {
            return Err(GradientBoostingError::RowMismatch);
        }
        if stages.iter().any(|&stage| stage == 0 || stage > self.trees.len()) {
            return Err(GradientBoostingError::Options);
        }
        let mut running = vec![self.intercept; design.len()];
        let mut wanted: Vec<(usize, usize)> =
            stages.iter().copied().enumerate().map(|(at, stage)| (stage, at)).collect();
        wanted.sort_unstable();
        let mut out = vec![Vec::new(); stages.len()];
        let mut next = 0;
        for (index, tree) in self.trees.iter().enumerate() {
            for (row, values) in design.iter().zip(&mut running) {
                let features: Vec<f32> = row.iter().map(|&value| value as f32).collect();
                *values += self.learning_rate * tree.predict_row(&features)[0];
            }
            while next < wanted.len() && wanted[next].0 == index + 1 {
                out[wanted[next].1] = running.clone();
                next += 1;
            }
        }
        Ok(out)
    }

    pub fn iterations(&self) -> usize {
        self.trees.len()
    }
}
