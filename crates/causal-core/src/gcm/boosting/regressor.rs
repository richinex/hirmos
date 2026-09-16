//! Numeric squared-error HistGradientBoostingRegressor settings used by DoWhy GOOD.
use super::{
    binning::Bins,
    histogram::{Derivatives, UnitDerivatives},
    tree::Tree,
};
use crate::{nprandom::Mt19937, numpy_reduce::numpy_mean};
use nalgebra::{DMatrix, DVector};
use std::num::NonZeroUsize;

#[derive(Clone, Copy)]
pub enum Stopping {
    Auto,
    Disabled,
}
pub struct Options {
    pub iterations: NonZeroUsize,
    pub leaves: NonZeroUsize,
    pub min_leaf: NonZeroUsize,
    pub bins: usize,
    pub learning_rate: f64,
    pub l2: f64,
    pub stopping: Stopping,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            iterations: 100.try_into().unwrap(),
            leaves: 31.try_into().unwrap(),
            min_leaf: 20.try_into().unwrap(),
            bins: 255,
            learning_rate: 0.1,
            l2: 0.,
            stopping: Stopping::Auto,
        }
    }
}
#[derive(Debug)]
pub enum BoostError {
    Shape,
    NonFinite,
    Options,
    Subsampling,
    Tree,
}
#[derive(Clone)]
pub struct Regressor {
    bins: Vec<Bins>,
    trees: Vec<Tree>,
    intercept: f64,
    missing: u8,
    pub validation_scores: Vec<f64>,
}

fn score(y: &[f64], prediction: &[f64]) -> f64 {
    -numpy_mean(
        &y.iter()
            .zip(prediction)
            .map(|(&y, &p)| {
                let d = p - y;
                0.5 * d * d
            })
            .collect::<Vec<_>>(),
    )
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
impl Regressor {
    pub fn fit(
        x: &DMatrix<f64>,
        y: &DVector<f64>,
        options: &Options,
        rng: &mut Mt19937,
    ) -> Result<Self, BoostError> {
        if x.nrows() == 0 || x.ncols() == 0 || x.nrows() != y.len() {
            return Err(BoostError::Shape);
        }
        if x.iter().chain(y.iter()).any(|v| !v.is_finite()) {
            return Err(BoostError::NonFinite);
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
        let seed = rng.randint(u32::MAX as u64) as u32;
        let _feature_seed = rng.randint(u32::MAX as u64);
        let stopping = matches!(options.stopping, Stopping::Auto) && x.nrows() > 10000;
        let (train, validation) = if stopping {
            let order = Mt19937::seeded(seed).permutation(x.nrows());
            let count = (x.nrows() as f64 * 0.1).ceil() as usize;
            (order[count..].to_vec(), order[..count].to_vec())
        } else {
            ((0..x.nrows()).collect(), Vec::new())
        };
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
        let validation_rows = map_rows(x, &validation, &bins);
        let columns: Vec<Vec<u8>> = (0..x.ncols())
            .map(|c| rows.iter().map(|r| r[c]).collect())
            .collect();
        let observed: Vec<_> = train.iter().map(|&r| y[r]).collect();
        let validation_y: Vec<_> = validation.iter().map(|&r| y[r]).collect();
        let intercept = numpy_mean(&observed);
        let mut predicted = vec![intercept; train.len()];
        let mut validation_prediction = vec![intercept; validation.len()];
        let mut scores = if stopping {
            vec![score(&validation_y, &validation_prediction)]
        } else {
            Vec::new()
        };
        let mut trees = Vec::new();
        for _ in 0..options.iterations.get() {
            let derivatives = Derivatives::squared_error(&observed, &predicted)
                .map_err(|_| BoostError::NonFinite)?;
            let unit = UnitDerivatives::try_from(&derivatives).map_err(|_| BoostError::Tree)?;
            let tree = Tree::fit(
                &columns,
                &counts,
                options.bins + 1,
                &unit,
                options.min_leaf,
                options.leaves,
                options.l2,
                options.learning_rate,
            )
            .map_err(|_| BoostError::Tree)?;
            for (r, p) in rows.iter().zip(&mut predicted) {
                *p += tree
                    .predict(r, options.bins as u8)
                    .map_err(|_| BoostError::Tree)?;
            }
            for (r, p) in validation_rows.iter().zip(&mut validation_prediction) {
                *p += tree
                    .predict(r, options.bins as u8)
                    .map_err(|_| BoostError::Tree)?;
            }
            trees.push(tree);
            if stopping {
                scores.push(score(&validation_y, &validation_prediction));
                if scores.len() >= 11 {
                    let reference = scores[scores.len() - 11] + 1e-7;
                    if !scores[scores.len() - 10..].iter().any(|&s| s > reference) {
                        break;
                    }
                }
            }
        }
        Ok(Self {
            bins,
            trees,
            intercept,
            missing: options.bins as u8,
            validation_scores: scores,
        })
    }
    pub fn predict(&self, x: &DMatrix<f64>) -> Result<DVector<f64>, BoostError> {
        if x.ncols() != self.bins.len() {
            return Err(BoostError::Shape);
        }
        if x.iter().any(|v| v.is_infinite()) {
            return Err(BoostError::NonFinite);
        }
        let mut values = DVector::from_element(x.nrows(), self.intercept);
        for r in 0..x.nrows() {
            let row: Vec<_> = self
                .bins
                .iter()
                .enumerate()
                .map(|(c, b)| b.map(x[(r, c)]))
                .collect();
            for tree in &self.trees {
                values[r] += tree
                    .predict(&row, self.missing)
                    .map_err(|_| BoostError::Tree)?;
            }
        }
        Ok(values)
    }
    pub fn iterations(&self) -> usize {
        self.trees.len()
    }
}
