//! Numeric additive-noise mechanisms from DoWhy 0.14, using the existing DGELSD regression.

use super::selection::{Kind, Predictor};
use crate::nprandom::Mt19937;
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Copy, Debug)]
pub enum Features {
    Linear,
    Quadratic,
}

#[derive(Clone, Copy, Debug)]
pub enum Output {
    Continuous,
    Discrete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdditiveError {
    Shape,
    NonFinite,
    NonDiscrete,
    IntegerRange,
    Regression,
}

/// Only fitted, nonempty empirical distributions can be sampled.
#[derive(Clone)]
pub struct Empirical {
    values: DVector<f64>,
}

impl Empirical {
    pub fn fit(values: DVector<f64>) -> Result<Self, AdditiveError> {
        if values.is_empty() {
            return Err(AdditiveError::Shape);
        }
        if values.iter().any(|v| !v.is_finite()) {
            return Err(AdditiveError::NonFinite);
        }
        Ok(Self { values })
    }
    pub fn draw(&self, count: usize, rng: &mut Mt19937) -> DVector<f64> {
        DVector::from_iterator(
            count,
            (0..count).map(|_| self.values[rng.randint(self.values.len() as u64) as usize]),
        )
    }
    pub fn values(&self) -> &DVector<f64> {
        &self.values
    }
}

/// Predictor and noise fit share one validated column count and output domain.
#[derive(Clone)]
pub struct AdditiveModel {
    columns: usize,
    output: Output,
    predictor: Predictor,
    noise: Empirical,
}

/// PolynomialFeatures(degree=2, include_bias=False), in sklearn feature order.
pub fn features(x: &DMatrix<f64>, kind: Features) -> Result<DMatrix<f64>, AdditiveError> {
    if x.ncols() == 0 {
        return Err(AdditiveError::Shape);
    }
    if x.iter().any(|v| !v.is_finite()) {
        return Err(AdditiveError::NonFinite);
    }
    match kind {
        Features::Linear => Ok(x.clone()),
        Features::Quadratic => {
            let columns = x.ncols();
            let width = columns
                .checked_add(1)
                .and_then(|n| n.checked_mul(columns))
                .and_then(|n| columns.checked_add(n / 2))
                .ok_or(AdditiveError::Shape)?;
            let mut result = DMatrix::zeros(x.nrows(), width);
            result.columns_mut(0, columns).copy_from(x);
            let mut c = columns;
            for a in 0..columns {
                for b in a..columns {
                    for r in 0..x.nrows() {
                        result[(r, c)] = x[(r, a)] * x[(r, b)];
                    }
                    c += 1;
                }
            }
            if result.iter().any(|v| !v.is_finite()) {
                return Err(AdditiveError::NonFinite);
            }
            Ok(result)
        }
    }
}

fn discrete(values: &DVector<f64>) -> Result<(), AdditiveError> {
    if values.iter().any(|v| !v.is_finite()) {
        return Err(AdditiveError::NonFinite);
    }
    if values.iter().any(|v| v.fract() != 0.0) {
        return Err(AdditiveError::NonDiscrete);
    }
    Ok(())
}

// The oracle casts rounded predictions and training targets to int32. Out-of-range
// casts are explicitly rejected here rather than adopting platform-specific overflow.
fn int32(values: &DVector<f64>) -> Result<(), AdditiveError> {
    if values
        .iter()
        .any(|&v| v < i32::MIN as f64 || v > i32::MAX as f64)
    {
        return Err(AdditiveError::IntegerRange);
    }
    Ok(())
}

impl AdditiveModel {
    pub fn fit(
        x: &DMatrix<f64>,
        y: &DVector<f64>,
        kind: Features,
        output: Output,
    ) -> Result<Self, AdditiveError> {
        Self::validate_training(x, y, output)?;
        let predictor = Predictor::linear(x, y, kind).map_err(|_| AdditiveError::Regression)?;
        Self::from_predictor(x, y, predictor, output)
    }

    pub fn fit_selected(
        x: &DMatrix<f64>,
        y: &DVector<f64>,
        kind: Kind,
        output: Output,
        rng: &mut Mt19937,
    ) -> Result<Self, AdditiveError> {
        Self::validate_training(x, y, output)?;
        let predictor = kind.fit(x, y, rng).map_err(|_| AdditiveError::Regression)?;
        Self::from_predictor(x, y, predictor, output)
    }

    fn from_predictor(
        x: &DMatrix<f64>,
        y: &DVector<f64>,
        predictor: Predictor,
        output: Output,
    ) -> Result<Self, AdditiveError> {
        let prediction = Self::convert(
            predictor
                .predict(x)
                .map_err(|_| AdditiveError::Regression)?,
            output,
        )?;
        let residuals = y - prediction;
        if let Output::Discrete = output {
            int32(&residuals)?;
        }
        Ok(Self {
            columns: x.ncols(),
            output,
            predictor,
            noise: Empirical::fit(residuals)?,
        })
    }

    fn validate_training(
        x: &DMatrix<f64>,
        y: &DVector<f64>,
        output: Output,
    ) -> Result<(), AdditiveError> {
        if x.nrows() != y.len() || y.is_empty() {
            return Err(AdditiveError::Shape);
        }
        if x.iter().chain(y.iter()).any(|v| !v.is_finite()) {
            return Err(AdditiveError::NonFinite);
        }
        if let Output::Discrete = output {
            discrete(y)?;
            int32(y)?;
        }
        Ok(())
    }

    fn convert(
        mut prediction: DVector<f64>,
        output: Output,
    ) -> Result<DVector<f64>, AdditiveError> {
        if prediction.iter().any(|v| !v.is_finite()) {
            return Err(AdditiveError::NonFinite);
        }
        if let Output::Discrete = output {
            prediction.apply(|v| *v = v.round_ties_even());
            int32(&prediction)?;
        }
        Ok(prediction)
    }

    pub fn predict(&self, x: &DMatrix<f64>) -> Result<DVector<f64>, AdditiveError> {
        if x.ncols() != self.columns {
            return Err(AdditiveError::Shape);
        }
        Self::convert(
            self.predictor
                .predict(x)
                .map_err(|_| AdditiveError::Regression)?,
            self.output,
        )
    }

    pub fn estimate_noise(
        &self,
        x: &DMatrix<f64>,
        y: &DVector<f64>,
    ) -> Result<DVector<f64>, AdditiveError> {
        if x.nrows() != y.len() {
            return Err(AdditiveError::Shape);
        }
        if y.iter().any(|v| !v.is_finite()) {
            return Err(AdditiveError::NonFinite);
        }
        if let Output::Discrete = self.output {
            discrete(y)?;
        }
        Ok(y - self.predict(x)?)
    }

    pub fn evaluate(
        &self,
        x: &DMatrix<f64>,
        noise: &DVector<f64>,
    ) -> Result<DVector<f64>, AdditiveError> {
        if x.nrows() != noise.len() {
            return Err(AdditiveError::Shape);
        }
        if noise.iter().any(|v| !v.is_finite()) {
            return Err(AdditiveError::NonFinite);
        }
        if let Output::Discrete = self.output {
            discrete(noise)?;
        }
        let values = self.predict(x)? + noise;
        if values.iter().any(|v| !v.is_finite()) {
            return Err(AdditiveError::NonFinite);
        }
        Ok(values)
    }

    pub fn draw_noise(&self, count: usize, rng: &mut Mt19937) -> DVector<f64> {
        self.noise.draw(count, rng)
    }
    pub fn residuals(&self) -> &DVector<f64> {
        self.noise.values()
    }
    pub fn predictor(&self) -> &Predictor {
        &self.predictor
    }
}
