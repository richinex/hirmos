//! Numeric squared-error HistGradientBoostingRegressor settings used by DoWhy GOOD.
use super::{
    binning::Bins,
    histogram::{Derivatives, UnitDerivatives},
    tree::Tree,
};
use crate::{nprandom::Mt19937, numpy_reduce::numpy_mean};
use nalgebra::{DMatrix, DVector};
use std::collections::HashMap;
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
            UnitDerivatives::try_from(&derivatives).map_err(|_| BoostError::Tree)?;
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
        // Identical bins follow identical paths. Bound the cache independently of row count.
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
            // Cache misses still add trees in their original order for each row.
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
    pub fn iterations(&self) -> usize {
        self.trees.len()
    }
}

#[cfg(test)]
mod prediction_tests {
    use super::*;

    #[test]
    fn cached_bins_match_independent_rows() {
        let x = DMatrix::from_fn(128, 3, |r, c| ((r * (c + 3)) % 37) as f64);
        let y = DVector::from_fn(128, |r, _| x[(r, 0)] * x[(r, 1)] - x[(r, 2)]);
        let model = Regressor::fit(&x, &y, &Options::default(), &mut Mt19937::seeded(0)).unwrap();
        let mut rng = Mt19937::seeded(123);
        let mut input = DMatrix::from_fn(9001, 3, |_, _| rng.next_f64() * 36.0);
        input[(0, 0)] = f64::NAN;
        input[(127, 2)] = f64::NAN;
        input.row_mut(8999).fill(0.1);
        input.row_mut(9000).fill(0.2);
        let actual = model.predict(&input).unwrap();
        assert_eq!(actual[8999].to_bits(), actual[9000].to_bits());
        let small = model.predict(&input.rows(0, 777).into_owned()).unwrap();
        assert_eq!(actual.rows(0, 777), small);
        let keys: std::collections::HashSet<Vec<u8>> = (0..input.nrows())
            .map(|r| {
                model
                    .bins
                    .iter()
                    .enumerate()
                    .map(|(c, b)| b.map(input[(r, c)]))
                    .collect()
            })
            .collect();
        assert!(keys.len() > 4096, "Exercise full-cache misses");
        for r in 0..input.nrows() {
            let row: Vec<_> = model
                .bins
                .iter()
                .enumerate()
                .map(|(c, b)| b.map(input[(r, c)]))
                .collect();
            let mut expected = model.intercept;
            for tree in &model.trees {
                expected += tree.predict(&row, model.missing).unwrap();
            }
            assert_eq!(actual[r].to_bits(), expected.to_bits());
        }
        assert!(model.predict(&DMatrix::zeros(0, 3)).unwrap().is_empty());
        assert!(matches!(
            model.predict(&DMatrix::zeros(2, 4)),
            Err(BoostError::Shape)
        ));
        input[(0, 0)] = f64::INFINITY;
        assert!(matches!(model.predict(&input), Err(BoostError::NonFinite)));
    }
}
