use super::{
    binning::Bins,
    histogram::Derivatives,
    regressor::{BoostError, Options, Stopping},
    tree::Tree,
};
use crate::{nprandom::Mt19937, numpy_reduce::numpy_mean};
use nalgebra::DMatrix;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Classifier {
    bins: Vec<Bins>,
    trees: Vec<Tree>,
    intercept: f64,
    missing: u8,
    pub validation_scores: Vec<f64>,
}

fn half_binomial_loss(y: f64, raw: f64) -> f64 {
    if raw <= -37.0 {
        raw.exp() - y * raw
    } else if raw <= -2.0 {
        libm::log1p(raw.exp()) - y * raw
    } else if raw <= 18.0 {
        libm::log1p((-raw).exp()) + (1.0 - y) * raw
    } else {
        (-raw).exp() + (1.0 - y) * raw
    }
}

fn score(y: &[f64], raw: &[f64]) -> f64 {
    -numpy_mean(
        &y.iter()
            .zip(raw)
            .map(|(&y, &raw)| half_binomial_loss(y, raw))
            .collect::<Vec<_>>(),
    )
}

fn expit(raw: f64) -> f64 {
    if raw >= 0.0 {
        1.0 / (1.0 + (-raw).exp())
    } else {
        let exponential = raw.exp();
        exponential / (1.0 + exponential)
    }
}

fn baseline(observed: &[f64]) -> f64 {
    let epsilon = 10.0 * f64::EPSILON;
    let mean = numpy_mean(observed).clamp(epsilon, 1.0 - epsilon);
    (mean / (1.0 - mean)).ln()
}

fn map_rows(x: &DMatrix<f64>, indices: &[usize], bins: &[Bins]) -> Vec<Vec<u8>> {
    indices
        .iter()
        .map(|&r| {
            bins.iter()
                .enumerate()
                .map(|(c, b)| b.map(x[(r, c)]))
                .collect()
        })
        .collect()
}

impl Classifier {
    pub fn fit(
        x: &DMatrix<f64>,
        y: &[f64],
        options: &Options,
        rng: &mut Mt19937,
    ) -> Result<Self, BoostError> {
        if x.nrows() == 0 || x.ncols() == 0 || x.nrows() != y.len() {
            return Err(BoostError::Shape);
        }
        if x.iter().any(|v| !v.is_finite()) || y.iter().any(|v| !v.is_finite()) {
            return Err(BoostError::NonFinite);
        }
        if y.iter().any(|&v| v != 0.0 && v != 1.0) {
            return Err(BoostError::Shape);
        }
        if !(2..=255).contains(&options.bins)
            || options.leaves.get() < 2
            || !options.learning_rate.is_finite()
            || options.learning_rate <= 0.
            || !options.l2.is_finite()
            || options.l2 < 0.
        {
            return Err(BoostError::Options);
        }
        if x.nrows() > 200000 {
            return Err(BoostError::Subsampling);
        }
        // sklearn stratifies the early-stopping validation split for classifiers, which this port
        // does not reproduce, so the case it would apply to is refused rather than approximated.
        if matches!(options.stopping, Stopping::Auto) && x.nrows() > 10000 {
            return Err(BoostError::Subsampling);
        }
        let _seed = rng.randint(u32::MAX as u64) as u32;
        let _feature_seed = rng.randint(u32::MAX as u64);

        let train: Vec<usize> = (0..x.nrows()).collect();
        let bins: Vec<_> = (0..x.ncols())
            .map(|c| {
                Bins::fit(
                    &train.iter().map(|&r| x[(r, c)]).collect::<Vec<_>>(),
                    options.bins,
                )
                .map_err(|_| BoostError::NonFinite)
            })
            .collect::<Result<_, _>>()?;
        let counts: Vec<_> = bins.iter().map(|b| b.thresholds().len() + 1).collect();
        let rows = map_rows(x, &train, &bins);
        let columns: Vec<Vec<u8>> = (0..x.ncols())
            .map(|c| rows.iter().map(|r| r[c]).collect())
            .collect();
        let observed: Vec<f64> = train.iter().map(|&r| y[r]).collect();
        let intercept = baseline(&observed);
        let mut raw = vec![intercept; train.len()];
        let mut trees = Vec::new();
        for _ in 0..options.iterations.get() {
            let derivatives =
                Derivatives::half_binomial(&observed, &raw).map_err(|_| BoostError::NonFinite)?;
            let tree = Tree::fit(
                &columns,
                &counts,
                options.bins + 1,
                &derivatives,
                options.min_leaf,
                options.leaves,
                options.l2,
                options.learning_rate,
            )
            .map_err(|_| BoostError::Tree)?;
            for (r, p) in rows.iter().zip(&mut raw) {
                *p += tree
                    .predict(r, options.bins as u8)
                    .map_err(|_| BoostError::Tree)?;
            }
            trees.push(tree);
        }
        Ok(Self {
            bins,
            trees,
            intercept,
            missing: options.bins as u8,
            validation_scores: vec![score(&observed, &raw)],
        })
    }

    pub fn decision_function(&self, x: &DMatrix<f64>) -> Result<Vec<f64>, BoostError> {
        if x.ncols() != self.bins.len() {
            return Err(BoostError::Shape);
        }
        if x.iter().any(|v| v.is_infinite()) {
            return Err(BoostError::NonFinite);
        }
        let mut values = vec![self.intercept; x.nrows()];
        let mut cache: HashMap<Vec<u8>, f64> = HashMap::new();
        let columns = self.bins.len();
        let mut rows = vec![0; x.nrows().min(256) * columns];
        let mut pending = Vec::with_capacity(x.nrows().min(256));
        for start in (0..x.nrows()).step_by(256) {
            pending.clear();
            let count = (x.nrows() - start).min(256);
            for r in 0..count {
                let row = &mut rows[r * columns..(r + 1) * columns];
                for (c, b) in self.bins.iter().enumerate() {
                    row[c] = b.map(x[(start + r, c)]);
                }
                match cache.get(&*row) {
                    Some(&value) => values[start + r] = value,
                    None => pending.push(r),
                }
            }
            for tree in &self.trees {
                for &r in &pending {
                    values[start + r] += tree
                        .predict(&rows[r * columns..(r + 1) * columns], self.missing)
                        .map_err(|_| BoostError::Tree)?;
                }
            }
            for &r in &pending {
                if cache.len() == 4096 {
                    break;
                }
                cache.insert(
                    rows[r * columns..(r + 1) * columns].to_vec(),
                    values[start + r],
                );
            }
        }
        Ok(values)
    }

    pub fn predict_probability(&self, x: &DMatrix<f64>) -> Result<Vec<f64>, BoostError> {
        Ok(self
            .decision_function(x)?
            .into_iter()
            .map(expit)
            .collect())
    }

    pub fn iterations(&self) -> usize {
        self.trees.len()
    }
}
