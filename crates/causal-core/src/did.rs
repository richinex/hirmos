//! DoubleML 0.10.1 observational two-period DiD on paired outcome differences.
//! Cross-fitting reuses the existing sklearn folds, centered LAPACK regression,
//! logistic optimizer and linear-score solver. Experimental scores are not implied.

use crate::{dml, logistic, nprandom::Mt19937, sklearn_linear::fit_sklearn_linear_regression};
use nalgebra::{DMatrix, DVector};

#[derive(Debug, Clone, Copy)]
pub enum Normalization {
    InSample,
    Population,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    NonBinary,
    MissingGroup,
    Regression,
    PropensityBoundary,
    RequiresTwoPeriods,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Requirement {
    CrossFitting,
    GroupFoldCapacity,
    TrimmingProbability,
}

pub const REQUIREMENTS: [Requirement; 3] = [
    Requirement::CrossFitting,
    Requirement::GroupFoldCapacity,
    Requirement::TrimmingProbability,
];

#[derive(Debug, PartialEq)]
pub enum Violation {
    TooFewFolds {
        supplied: usize,
        minimum: usize,
    },
    TooManyFolds {
        supplied: usize,
        smaller_group: usize,
    },
    InvalidTrimming {
        supplied: f64,
    },
}

/// An executable plan can only be constructed after all readiness rules pass.
pub struct Plan<'a> {
    sample: &'a PairedSample,
    folds: usize,
    seed: u32,
    trimming: f64,
    normalization: Normalization,
}

pub fn prepare(
    sample: &PairedSample,
    folds: usize,
    seed: u32,
    trimming: f64,
    normalization: Normalization,
) -> Result<Plan<'_>, Vec<Violation>> {
    let treated = sample.group.iter().filter(|d| **d == 1.0).count();
    let smaller_group = treated.min(sample.group.len() - treated);
    let violations: Vec<_> = REQUIREMENTS
        .iter()
        .filter_map(|rule| match rule {
            Requirement::CrossFitting if folds < 2 => Some(Violation::TooFewFolds {
                supplied: folds,
                minimum: 2,
            }),
            Requirement::GroupFoldCapacity if folds > smaller_group => {
                Some(Violation::TooManyFolds {
                    supplied: folds,
                    smaller_group,
                })
            }
            Requirement::TrimmingProbability
                if !trimming.is_finite() || trimming <= 0.0 || trimming >= 0.5 =>
            {
                Some(Violation::InvalidTrimming { supplied: trimming })
            }
            Requirement::CrossFitting
            | Requirement::GroupFoldCapacity
            | Requirement::TrimmingProbability => None,
        })
        .collect();
    if violations.is_empty() {
        Ok(Plan {
            sample,
            folds,
            seed,
            trimming,
            normalization,
        })
    } else {
        Err(violations)
    }
}

/// One before/after pair per unit, constructed from the panel rather than accepted
/// as arbitrary outcome levels. Baseline covariates follow the panel's unit order.
pub struct PairedSample {
    x: Vec<Vec<f64>>,
    differences: Vec<f64>,
    group: Vec<f64>,
}

impl PairedSample {
    pub fn from_panel(
        panel: &crate::panel::PanelMatrices,
        baseline: &DMatrix<f64>,
    ) -> Result<Self, Error> {
        let n = panel.y.nrows();
        if panel.y.ncols() != 2 || panel.t0 != 1 {
            return Err(Error::RequiresTwoPeriods);
        }
        if panel.n0 == 0 || panel.n0 >= n {
            return Err(Error::MissingGroup);
        }
        if baseline.nrows() != n || baseline.ncols() == 0 {
            return Err(Error::Shape);
        }
        if panel
            .y
            .iter()
            .chain(baseline.iter())
            .any(|v| !v.is_finite())
        {
            return Err(Error::NonFinite);
        }
        let differences: Vec<_> = (0..n).map(|i| panel.y[(i, 1)] - panel.y[(i, 0)]).collect();
        if differences.iter().any(|v| !v.is_finite()) { return Err(Error::NonFinite); }
        Ok(Self {
            x: (0..n)
                .map(|i| baseline.row(i).iter().copied().collect())
                .collect(),
            differences,
            group: (0..n).map(|i| f64::from(i >= panel.n0)).collect(),
        })
    }
}

pub struct Fit {
    pub estimate: dml::DmlResult,
    pub folds: Vec<(Vec<usize>, Vec<usize>)>,
    pub g0: Vec<f64>,
    pub propensity: Vec<f64>,
    pub psi_a: Vec<f64>,
    pub psi_b: Vec<f64>,
    /// Non-converged propensity fits remain visible, matching sklearn's warning behavior.
    pub propensity_termination: Vec<crate::lbfgsb::LbfgsbTermination>,
}

pub fn observational_score(
    differences: &[f64],
    group: &[f64],
    g0: &[f64],
    propensity: &[f64],
    normalization: Normalization,
) -> Result<(Vec<f64>, Vec<f64>), Error> {
    let n = differences.len();
    if n == 0
        || [group.len(), g0.len(), propensity.len()]
            .iter()
            .any(|len| *len != n)
    {
        return Err(Error::Shape);
    }
    if differences
        .iter()
        .chain(group)
        .chain(g0)
        .chain(propensity)
        .any(|v| !v.is_finite())
    {
        return Err(Error::NonFinite);
    }
    if group.iter().any(|d| *d != 0.0 && *d != 1.0) {
        return Err(Error::NonBinary);
    }
    let p = group.iter().sum::<f64>() / n as f64;
    if p == 0.0 || p == 1.0 {
        return Err(Error::MissingGroup);
    }
    if propensity.iter().any(|m| *m <= 0.0 || *m >= 1.0) {
        return Err(Error::PropensityBoundary);
    }
    let odds: Vec<_> = group
        .iter()
        .zip(propensity)
        .map(|(d, m)| (1.0 - d) * m / (1.0 - m))
        .collect();
    let mean_odds = odds.iter().sum::<f64>() / n as f64;
    let a = group.iter().map(|d| -d / p).collect();
    let b = (0..n)
        .map(|i| {
            let weight = match normalization {
                Normalization::InSample => group[i] / p - odds[i] / mean_odds,
                Normalization::Population => {
                    (group[i] - propensity[i]) / (p * (1.0 - propensity[i]))
                }
            };
            weight * (differences[i] - g0[i])
        })
        .collect();
    Ok((a, b))
}

/// One row per unit: pre-treatment covariates, Y_after - Y_before, and group membership.
pub fn fit(plan: &Plan<'_>) -> Result<Fit, Error> {
    let Plan {
        sample,
        folds,
        seed,
        trimming,
        normalization,
    } = *plan;
    let PairedSample {
        x,
        differences,
        group,
    } = sample;
    let n = differences.len();
    let mut rng = Mt19937::seeded(seed);
    // DoubleML base construction consumes a plain split before treatment stratification.
    let _ = dml::kfold_shuffled(n, folds, &mut rng);
    let splits = dml::kfold_stratified(group, folds, &mut rng);
    let mut g0 = vec![0.0; n];
    let mut propensity = vec![0.0; n];
    let mut termination = Vec::with_capacity(folds);
    for (train, test) in &splits {
        let controls: Vec<_> = train.iter().copied().filter(|i| group[*i] == 0.0).collect();
        let predictors = DMatrix::from_fn(controls.len(), x[0].len(), |i, j| x[controls[i]][j]);
        let target =
            DVector::from_iterator(controls.len(), controls.iter().map(|i| differences[*i]));
        // sklearn 1.6.1 dense LinearRegression uses max(X.shape) * eps as cond.
        let regression = fit_sklearn_linear_regression(
            &predictors,
            &target,
            controls.len().max(x[0].len()) as f64 * f64::EPSILON,
        )
        .map_err(|_| Error::Regression)?;
        let train_x: Vec<_> = train.iter().map(|i| x[*i].clone()).collect();
        let train_d: Vec<_> = train.iter().map(|i| group[*i]).collect();
        let classifier = logistic::fit_unpenalized(&train_x, &train_d);
        termination.push(classifier.termination);
        let test_x: Vec<_> = test.iter().map(|i| x[*i].clone()).collect();
        let probabilities = classifier.predict_probability(&test_x);
        for (&i, &probability) in test.iter().zip(&probabilities) {
            if !probability.is_finite() || probability < 1e-12 || probability > 1.0 - 1e-12 {
                return Err(Error::PropensityBoundary);
            }
            propensity[i] = probability.clamp(trimming, 1.0 - trimming);
            g0[i] = regression.intercept
                + x[i]
                    .iter()
                    .zip(regression.coefficients.iter())
                    .map(|(v, b)| v * b)
                    .sum::<f64>();
        }
    }
    let (psi_a, psi_b) = observational_score(differences, group, &g0, &propensity, normalization)?;
    let estimate = dml::solve_score(&psi_a, &psi_b);
    Ok(Fit {
        estimate,
        folds: splits,
        g0,
        propensity,
        psi_a,
        psi_b,
        propensity_termination: termination,
    })
}
