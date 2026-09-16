//! DoWhy GOOD model selection for complete, scalar numeric data.
use super::{
    additive::{features, Features},
    boosting::regressor::{Options, Regressor},
    shapley::Execution,
};
use crate::{
    dml::kfold_shuffled,
    nprandom::Mt19937,
    numpy_reduce::numpy_mean,
    sklearn_linear::{fit_sklearn_fortran, predict_fortran, SKLEARN_LINEAR_TOLERANCE},
};
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Linear,
    Boosted,
    Quadratic,
}

#[derive(Clone)]
pub enum Predictor {
    Regression(Regression),
    Boosted(Regressor),
}

#[derive(Clone)]
pub struct Regression {
    columns: usize,
    features: Features,
    coefficients: DVector<f64>,
    intercept: f64,
}

#[derive(Debug)]
pub enum SelectionError {
    Shape,
    NonFinite,
    Fit,
    Predict,
}

impl Kind {
    pub fn fit(
        self,
        x: &DMatrix<f64>,
        y: &DVector<f64>,
        rng: &mut Mt19937,
    ) -> Result<Predictor, SelectionError> {
        match self {
            Self::Linear => Predictor::linear(x,y,Features::Linear),
            Self::Quadratic => Predictor::linear(x,y,Features::Quadratic),
            Self::Boosted => Regressor::fit(x, y, &Options::default(), rng)
                .map(Predictor::Boosted)
                .map_err(|_| SelectionError::Fit),
        }
    }
}
impl Predictor {
    pub fn linear(
        x: &DMatrix<f64>,
        y: &DVector<f64>,
        kind: Features,
    ) -> Result<Self, SelectionError> {
        if x.nrows() != y.len() || y.is_empty() {
            return Err(SelectionError::Shape);
        }
        if y.iter().any(|v| !v.is_finite()) {
            return Err(SelectionError::NonFinite);
        }
        let expanded = features(x, kind).map_err(|_| SelectionError::Shape)?;
        let fit = fit_sklearn_fortran(&expanded, y, SKLEARN_LINEAR_TOLERANCE)
            .map_err(|_| SelectionError::Fit)?;
        Ok(Self::Regression(Regression {
            columns: x.ncols(),
            features: kind,
            coefficients: fit.coefficients,
            intercept: fit.intercept,
        }))
    }
    pub fn predict(&self, x: &DMatrix<f64>) -> Result<DVector<f64>, SelectionError> {
        match self {
            Self::Regression(model) => {
                if x.ncols() != model.columns {
                    return Err(SelectionError::Shape);
                }
                let input = features(x, model.features).map_err(|_| SelectionError::Predict)?;
                Ok(predict_fortran(
                    &input,
                    &model.coefficients,
                    model.intercept,
                ))
            }
            Self::Boosted(model) => model.predict(x).map_err(|_| SelectionError::Predict),
        }
    }
    pub fn linear_parameters(&self) -> Option<(&DVector<f64>, f64)> {
        match self {
            Self::Regression(model) => Some((&model.coefficients, model.intercept)),
            Self::Boosted(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct Candidate {
    pub kind: Kind,
    pub score: f64,
}
/// Ranked candidates are nonempty and sorted by held-out mean squared error.
#[derive(Clone)]
pub struct Selection {
    candidates: Vec<Candidate>,
}
impl Selection {
    pub fn best(&self) -> Kind {
        self.candidates[0].kind
    }
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }
}

fn rows(x: &DMatrix<f64>, indices: &[usize]) -> DMatrix<f64> {
    DMatrix::from_fn(indices.len(), x.ncols(), |r, c| x[(indices[r], c)])
}
fn evaluate(
    kind: Kind,
    x: &DMatrix<f64>,
    y: &DVector<f64>,
    folds: &[(Vec<usize>, Vec<usize>)],
    rng: &mut Mt19937,
) -> Result<f64, SelectionError> {
    let mut scores = Vec::with_capacity(folds.len());
    for (train, test) in folds {
        let train = &train[..train.len().min(20000)];
        let test = &test[..test.len().min(20000)];
        let model = kind.fit(
            &rows(x, train),
            &DVector::from_iterator(train.len(), train.iter().map(|&i| y[i])),
            rng,
        )?;
        let predicted = model.predict(&rows(x, test))?;
        scores.push(numpy_mean(
            &test
                .iter()
                .zip(predicted.iter())
                .map(|(&i, &p)| {
                    let d = y[i] - p;
                    d * d
                })
                .collect::<Vec<_>>(),
        ));
    }
    Ok(numpy_mean(&scores))
}

pub fn select(
    x: &DMatrix<f64>,
    y: &DVector<f64>,
    execution: Execution,
    rng: &mut Mt19937,
) -> Result<Selection, SelectionError> {
    if x.nrows() < 5 || x.ncols() == 0 || x.nrows() != y.len() {
        return Err(SelectionError::Shape);
    }
    if x.iter().chain(y.iter()).any(|v| !v.is_finite()) {
        return Err(SelectionError::NonFinite);
    }
    let kinds = if x.ncols() <= 5 {
        &[Kind::Linear, Kind::Boosted, Kind::Quadratic][..]
    } else {
        &[Kind::Linear, Kind::Boosted][..]
    };
    let folds = kfold_shuffled(x.nrows(), 5, rng);
    let seeds: Vec<_> = kinds
        .iter()
        .map(|_| rng.randint(i32::MAX as u64) as u32)
        .collect();
    let mut candidates = Vec::with_capacity(kinds.len());
    for (&kind, seed) in kinds.iter().zip(seeds) {
        let score = match execution {
            Execution::Serial => {
                *rng = Mt19937::seeded(seed);
                evaluate(kind, x, y, &folds, rng)?
            }
            Execution::Isolated => evaluate(kind, x, y, &folds, &mut Mt19937::seeded(seed))?,
        };
        if !score.is_finite() {
            return Err(SelectionError::NonFinite);
        }
        candidates.push(Candidate { kind, score });
    }
    // Python's sorted retains factory order on tied scores.
    candidates.sort_by(|a, b| a.score.total_cmp(&b.score));
    Ok(Selection { candidates })
}
