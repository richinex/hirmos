//! EconML's T-learner (`econml.metalearners.TLearner`) with random-forest outcome models.
//!
//! The reference one-hot encodes the treatment with the first category as control, fits one copy of
//! the outcome model per arm on that arm's rows alone, and reads the constant marginal effect at a
//! query row as the treated model's prediction minus the control model's. The two models are clones
//! of one estimator, so they share the random state and differ only in the rows they see. EconML
//! offers an interval only through bootstrap inference, which is not ported: the effect is a point
//! per row.

use crate::sktree::{fit_forest, RandomForest};
use std::error::Error;
use std::fmt::{Display, Formatter};

/// The two arm models and the rows each was fitted on.
pub struct TLearnerFit {
    control: RandomForest,
    treated: RandomForest,
    pub control_rows: usize,
    pub treated_rows: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TLearnerError {
    /// A treatment value other than 0 or 1; the reference accepts binary treatments only.
    TreatmentNotBinary {
        row: usize,
    },
    /// One arm has no rows, so its outcome model has nothing to fit.
    ArmEmpty {
        treated: bool,
    },
    LengthMismatch,
    NoRows,
}

impl Display for TLearnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TLearnerError::TreatmentNotBinary { row } => {
                write!(
                    f,
                    "the treatment must be 0 or 1 in every row; row {row} is neither"
                )
            }
            TLearnerError::ArmEmpty { treated: true } => {
                write!(f, "no treated rows to fit the treated outcome model")
            }
            TLearnerError::ArmEmpty { treated: false } => {
                write!(f, "no control rows to fit the control outcome model")
            }
            TLearnerError::LengthMismatch => {
                write!(f, "covariates, treatment and outcome differ in length")
            }
            TLearnerError::NoRows => write!(f, "no rows"),
        }
    }
}

impl Error for TLearnerError {}

/// Fit `RandomForestRegressor(n_estimators, min_samples_leaf, random_state=seed)` per arm, as
/// `TLearner(models=RandomForestRegressor(...)).fit(y, t, X=x)` does.
pub fn fit_tlearner(
    x: &[Vec<f64>],
    y: &[f64],
    t: &[f64],
    n_estimators: usize,
    min_samples_leaf: usize,
    seed: u32,
) -> Result<TLearnerFit, TLearnerError> {
    if x.is_empty() {
        return Err(TLearnerError::NoRows);
    }
    if x.len() != y.len() || x.len() != t.len() {
        return Err(TLearnerError::LengthMismatch);
    }
    if let Some(row) = t.iter().position(|&value| value != 0.0 && value != 1.0) {
        return Err(TLearnerError::TreatmentNotBinary { row });
    }
    let arm = |treated: bool| -> (Vec<Vec<f64>>, Vec<f64>) {
        let wanted = if treated { 1.0 } else { 0.0 };
        let rows: Vec<Vec<f64>> = x
            .iter()
            .zip(t)
            .filter(|(_, &value)| value == wanted)
            .map(|(row, _)| row.clone())
            .collect();
        let outcomes: Vec<f64> = y
            .iter()
            .zip(t)
            .filter(|(_, &value)| value == wanted)
            .map(|(&value, _)| value)
            .collect();
        (rows, outcomes)
    };
    let (control_x, control_y) = arm(false);
    let (treated_x, treated_y) = arm(true);
    if control_x.is_empty() {
        return Err(TLearnerError::ArmEmpty { treated: false });
    }
    if treated_x.is_empty() {
        return Err(TLearnerError::ArmEmpty { treated: true });
    }
    Ok(TLearnerFit {
        control: fit_forest(
            &control_x,
            &control_y,
            n_estimators,
            min_samples_leaf,
            None,
            seed,
        ),
        treated: fit_forest(
            &treated_x,
            &treated_y,
            n_estimators,
            min_samples_leaf,
            None,
            seed,
        ),
        control_rows: control_x.len(),
        treated_rows: treated_x.len(),
    })
}

impl TLearnerFit {
    /// `const_marginal_effect(X)`: the treated prediction minus the control prediction, per row.
    pub fn effect(&self, x: &[Vec<f64>]) -> Vec<f64> {
        let treated = self.treated.predict(x);
        let control = self.control.predict(x);
        treated.iter().zip(&control).map(|(&a, &b)| a - b).collect()
    }
}
