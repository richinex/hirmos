//! Matrix preparation and default nested fitting for R Synth's predictor-based specification.
//! X rows are predictors and columns are donors; Z rows are pre-period outcomes.
//! This is not the existing outcome-only fitting objective.
use super::ipop::{simplex_ipop, IpopError, IpopTermination};
use crate::lapack_lu::{dgetrf_fused, dgetrs_fused, reciprocal_condition_one_fused, Transpose};
use crate::r_nelder_mead::{minimize, NmError, NmTermination};
use crate::resampling::rounded_nonnegative_sum;
use crate::survival::flexsurv::r_optim::{r_optim_bfgs_numeric, ROptimControl, ROptimError};
use nalgebra::{DMatrix, DVector};
use std::num::NonZeroUsize;

pub(super) fn oracle_product(
    a: &DMatrix<f64>,
    b: &DMatrix<f64>,
) -> Result<DMatrix<f64>, SynthError> {
    use crate::lapack_dgelsd::blas::{dgemm_fused, Transpose as BlasTranspose};
    if a.ncols() != b.nrows() {
        return Err(SynthError::DimensionMismatch);
    }
    let mut result = DMatrix::zeros(a.nrows(), b.ncols());
    dgemm_fused(
        BlasTranspose::None,
        BlasTranspose::None,
        a.nrows(),
        b.ncols(),
        a.ncols(),
        1.,
        a.as_slice(),
        a.nrows(),
        b.as_slice(),
        b.nrows(),
        0.,
        result.as_mut_slice(),
        a.nrows(),
    )
    .map_err(|_| SynthError::DimensionMismatch)?;
    Ok(result)
}

#[derive(Debug, PartialEq)]
pub enum SynthError {
    EmptyPredictors,
    InsufficientDonors,
    EmptyPrePeriod,
    DimensionMismatch,
    NonFiniteInput,
    ConstantDonorPredictor { predictor: usize },
    InvalidPredictorWeights,
    DonorSolver(IpopError),
    DonorIterationLimit,
    NelderMead(NmError),
    Bfgs(ROptimError),
    NoUsableOptimizer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredictorOptimizer {
    NelderMead,
    Bfgs,
}
#[derive(Clone)]
pub struct OptimizerCandidate {
    pub method: PredictorOptimizer,
    pub parameters: Vec<f64>,
    pub outcome_mspe: f64,
    pub evaluations: usize,
    pub convergence_code: usize,
}

/// A failed optimx method is not a candidate with fabricated parameters.
pub enum OptimizerAttempt {
    Completed(OptimizerCandidate),
    Failed {
        method: PredictorOptimizer,
        error: SynthError,
        objective_evaluations: usize,
    },
}

pub struct StartedAttempt {
    pub start: PredictorStart,
    pub attempt: OptimizerAttempt,
}

pub struct FixedPredictorFit {
    pub predictor_weights: Vec<f64>,
    pub donor_weights: Vec<f64>,
    pub predictor_loss: f64,
    pub outcome_mspe: f64,
    pub donor_iterations: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredictorStart {
    Equal,
    Regression,
}

/// Keep the selected optimization record distinct from its final donor refit.
/// Synth includes optimizer candidates with nonzero convergence codes when
/// selecting the smallest loss; those codes remain available to the caller.
pub struct SelectedPredictorRun {
    pub start: PredictorStart,
    pub candidate: OptimizerCandidate,
}

pub enum PredictorSelection {
    SinglePredictor,
    SuppliedWeights,
    Optimized(SelectedPredictorRun),
}

pub struct SynthFit {
    pub selection: PredictorSelection,
    pub fit: FixedPredictorFit,
    pub attempts: Vec<StartedAttempt>,
}

pub struct SynthMatrices {
    pub scaled_donors: DMatrix<f64>,
    pub scaled_treated: DVector<f64>,
    pub predictor_scales: Vec<f64>,
    pub pre_donors: DMatrix<f64>,
    pub pre_treated: DVector<f64>,
}

impl SynthMatrices {
    /// Synth tries equal weights and, when its normal equations are invertible,
    /// a second start proportional to squared predictor regression coefficients.
    pub fn regression_start(&self) -> Result<Option<Vec<f64>>, SynthError> {
        let p = self.scaled_treated.len();
        let n = self.scaled_donors.ncols() + 1;
        let design = DMatrix::from_fn(n, p + 1, |r, c| {
            if c == 0 {
                1.
            } else if r == 0 {
                self.scaled_treated[c - 1]
            } else {
                self.scaled_donors[(c - 1, r - 1)]
            }
        });
        let normal = oracle_product(&design.transpose(), &design)?;
        let k = p + 1;
        let mut lu = normal.as_slice().to_vec();
        let mut pivots = vec![0; k];
        let info = dgetrf_fused(k, k, &mut lu, k, &mut pivots)
            .map_err(|_| SynthError::DimensionMismatch)?;
        if info != 0 {
            return Ok(None);
        }
        let mut inverse = DMatrix::<f64>::identity(k, k).as_slice().to_vec();
        dgetrs_fused(Transpose::None, k, k, &lu, k, &pivots, &mut inverse, k)
            .map_err(|_| SynthError::DimensionMismatch)?;
        let inverse = DMatrix::from_column_slice(k, k, &inverse);
        let norm = |m: &DMatrix<f64>| {
            (0..m.ncols())
                .map(|c| m.column(c).iter().map(|x| x.abs()).sum::<f64>())
                .fold(0., f64::max)
        };
        if reciprocal_condition_one_fused(k, &lu, k, norm(&normal))
            .map_err(|_| SynthError::NonFiniteInput)?
            < f64::EPSILON
        {
            return Ok(None);
        }
        let outcomes = DMatrix::from_fn(n, self.pre_treated.len(), |r, c| {
            if r == 0 {
                self.pre_treated[c]
            } else {
                self.pre_donors[(c, r - 1)]
            }
        });
        let beta = oracle_product(&oracle_product(&inverse, &design.transpose())?, &outcomes)?;
        let square = oracle_product(&beta, &beta.transpose())?;
        let weights: Vec<f64> = (1..k).map(|r| square[(r, r)]).collect();
        let total = rounded_nonnegative_sum(&weights).ok_or(SynthError::InvalidPredictorWeights)?;
        if !total.is_finite() || total <= 0. {
            return Err(SynthError::InvalidPredictorWeights);
        }
        Ok(Some(weights.iter().map(|v| v / total).collect()))
    }

    /// Default Synth 1.1-9 nested fit, excluding optional rgenoud/optimx methods.
    /// Within a start, equal losses keep the first method. Between starts,
    /// Synth's strict comparison selects the regression run on a tie.
    pub fn fit_default(&self) -> Result<SynthFit, SynthError> {
        if self.scaled_treated.len() == 1 {
            return Ok(SynthFit {
                selection: PredictorSelection::SinglePredictor,
                fit: self.fit_fixed_predictor_weights(&[1.])?,
                attempts: Vec::new(),
            });
        }
        let equal = vec![1. / self.scaled_treated.len() as f64; self.scaled_treated.len()];
        let mut attempts = Vec::new();
        let mut selected: Option<SelectedPredictorRun> = None;
        let mut consider = |start, runs: Vec<OptimizerAttempt>| {
            let mut best: Option<OptimizerCandidate> = None;
            for attempt in runs {
                if let OptimizerAttempt::Completed(candidate) = &attempt {
                    if best
                        .as_ref()
                        .is_none_or(|b| candidate.outcome_mspe < b.outcome_mspe)
                    {
                        best = Some(candidate.clone());
                    }
                }
                attempts.push(StartedAttempt { start, attempt });
            }
            if let Some(candidate) = best {
                if selected
                    .as_ref()
                    .is_none_or(|s| candidate.outcome_mspe <= s.candidate.outcome_mspe)
                {
                    selected = Some(SelectedPredictorRun { start, candidate });
                }
            }
        };
        consider(PredictorStart::Equal, self.optimize_start_attempts(&equal)?);
        if let Some(regression) = self.regression_start()? {
            consider(
                PredictorStart::Regression,
                self.optimize_start_attempts(&regression)?,
            );
        }
        let selected = selected.ok_or(SynthError::NoUsableOptimizer)?;
        let weights: Vec<f64> = selected
            .candidate
            .parameters
            .iter()
            .map(|v| v.abs())
            .collect();
        // synth normalizes V once before the final QP. Do not normalize it a
        // second time: its rounded sum need not be exactly one.
        let normalized = normalize_predictor_weights(&weights, self.scaled_treated.len())?;
        let fit = self.fit_normalized_predictor_weights(&normalized)?;
        Ok(SynthFit {
            selection: PredictorSelection::Optimized(selected),
            fit,
            attempts,
        })
    }

    /// Explicit nonnegative predictor weights bypass the outer optimization.
    /// As in Synth, a single predictor always has weight one.
    pub fn fit_custom(&self, weights: &[f64]) -> Result<SynthFit, SynthError> {
        if self.scaled_treated.len() == 1 {
            return self.fit_default();
        }
        Ok(SynthFit {
            selection: PredictorSelection::SuppliedWeights,
            fit: self.fit_fixed_predictor_weights(weights)?,
            attempts: Vec::new(),
        })
    }

    /// Run the two default optimx methods independently from one explicit start.
    /// Non-convergence remains visible rather than being labelled a successful fit.
    pub fn optimize_start(&self, start: &[f64]) -> Result<Vec<OptimizerCandidate>, SynthError> {
        self.optimize_start_attempts(start)?
            .into_iter()
            .map(|attempt| match attempt {
                OptimizerAttempt::Completed(candidate) => Ok(candidate),
                OptimizerAttempt::Failed { error, .. } => Err(error),
            })
            .collect()
    }

    /// Preserve every method outcome, as optimx does when one method fails.
    pub fn optimize_start_attempts(
        &self,
        start: &[f64],
    ) -> Result<Vec<OptimizerAttempt>, SynthError> {
        self.fit_fixed_predictor_weights(start)?;
        // R throws on a failed objective evaluation, including line-search
        // calls. Keep that error rather than treating it as a recoverable NaN.
        let failure = std::cell::RefCell::new(None);
        let evaluations = std::cell::Cell::new(0usize);
        let objective = |x: &[f64]| {
            if failure.borrow().is_some() {
                return f64::NAN;
            }
            evaluations.set(evaluations.get() + 1);
            match self.fit_fixed_predictor_weights(&x.iter().map(|v| v.abs()).collect::<Vec<_>>()) {
                Ok(fit) => fit.outcome_mspe,
                Err(error) => {
                    *failure.borrow_mut() = Some(error);
                    f64::NAN
                }
            }
        };
        let nm = minimize(
            start,
            500,
            f64::EPSILON.sqrt(),
            f64::NEG_INFINITY,
            objective,
        )
        .map_err(SynthError::NelderMead);
        let nm = match failure.borrow_mut().take() {
            Some(error) => Err(error),
            None => nm,
        };
        let nm_evaluations = evaluations.replace(0);
        let control = ROptimControl::bfgs_pinned_arm_defaults(
            NonZeroUsize::new(start.len()).ok_or(SynthError::EmptyPredictors)?,
        );
        let bfgs = r_optim_bfgs_numeric(start, &control, objective).map_err(SynthError::Bfgs);
        let bfgs = match failure.borrow_mut().take() {
            Some(error) => Err(error),
            None => bfgs,
        };
        Ok(vec![
            match nm {
                Ok(nm) => OptimizerAttempt::Completed(OptimizerCandidate {
                    method: PredictorOptimizer::NelderMead,
                    parameters: nm.parameters,
                    outcome_mspe: nm.value,
                    evaluations: nm.evaluations,
                    convergence_code: match nm.termination {
                        NmTermination::Converged => 0,
                        NmTermination::EvaluationLimit => 1,
                        NmTermination::ShrinkFailure => 10,
                    },
                }),
                Err(error) => OptimizerAttempt::Failed {
                    method: PredictorOptimizer::NelderMead,
                    error,
                    objective_evaluations: nm_evaluations,
                },
            },
            match bfgs {
                Ok(bfgs) => OptimizerAttempt::Completed(OptimizerCandidate {
                    method: PredictorOptimizer::Bfgs,
                    parameters: bfgs.parameters,
                    outcome_mspe: bfgs.value,
                    evaluations: bfgs.function_count,
                    convergence_code: bfgs.convergence,
                }),
                Err(error) => OptimizerAttempt::Failed {
                    method: PredictorOptimizer::Bfgs,
                    error,
                    objective_evaluations: evaluations.get(),
                },
            },
        ])
    }

    /// Fit Synth's default donor objective with explicitly supplied predictor
    /// weights. This does not optimize the predictor weights against Z.
    pub fn fit_fixed_predictor_weights(
        &self,
        weights: &[f64],
    ) -> Result<FixedPredictorFit, SynthError> {
        let normalized = normalize_predictor_weights(weights, self.scaled_treated.len())?;
        self.fit_normalized_predictor_weights(&normalized)
    }

    fn fit_normalized_predictor_weights(
        &self,
        predictor_weights: &[f64],
    ) -> Result<FixedPredictorFit, SynthError> {
        let v = DMatrix::from_diagonal(&DVector::from_column_slice(&predictor_weights));
        let weighted_donors = oracle_product(&self.scaled_donors.transpose(), &v)?;
        let h = oracle_product(&weighted_donors, &self.scaled_donors)?;
        // fn.V constructs c as -(X1' V) X0, not -X0' V X1.
        // Preserve its multiplication order, including intermediate rounding.
        let treated = DMatrix::from_column_slice(
            self.scaled_treated.len(),
            1,
            self.scaled_treated.as_slice(),
        );
        let weighted_treated = oracle_product(&treated.transpose(), &v)?;
        let c = -DVector::from_column_slice(
            oracle_product(&weighted_treated, &self.scaled_donors)?.as_slice(),
        );
        let fit = simplex_ipop(&h, &c, 5., 1000, 0.0005, 10.).map_err(SynthError::DonorSolver)?;
        if fit.termination != IpopTermination::SignificantFigures {
            return Err(SynthError::DonorIterationLimit);
        }
        let w = DMatrix::from_column_slice(fit.weights.len(), 1, &fit.weights);
        let predictor_residual = treated - oracle_product(&self.scaled_donors, &w)?;
        let weighted_residual = oracle_product(&predictor_residual.transpose(), &v)?;
        let predictor_loss = oracle_product(&weighted_residual, &predictor_residual)?[(0, 0)];
        let residual =
            DMatrix::from_column_slice(self.pre_treated.len(), 1, self.pre_treated.as_slice())
                - oracle_product(&self.pre_donors, &w)?;
        let outcome_mspe =
            oracle_product(&residual.transpose(), &residual)?[(0, 0)] / residual.nrows() as f64;
        Ok(FixedPredictorFit {
            predictor_weights: predictor_weights.to_vec(),
            donor_weights: fit.weights,
            predictor_loss,
            outcome_mspe,
            donor_iterations: fit.iterations,
        })
    }

    /// Synth::synth scales each predictor by multiplying by the reciprocal
    /// of its sample standard deviation. It does not mean-centre it.
    pub fn prepare(
        donors: &DMatrix<f64>,
        treated: &DVector<f64>,
        pre_donors: &DMatrix<f64>,
        pre_treated: &DVector<f64>,
    ) -> Result<Self, SynthError> {
        let predictors = donors.nrows();
        let controls = donors.ncols();
        if predictors == 0 {
            return Err(SynthError::EmptyPredictors);
        }
        if controls < 2 {
            return Err(SynthError::InsufficientDonors);
        }
        if pre_donors.nrows() == 0 {
            return Err(SynthError::EmptyPrePeriod);
        }
        if treated.len() != predictors
            || pre_donors.ncols() != controls
            || pre_donors.nrows() != pre_treated.len()
        {
            return Err(SynthError::DimensionMismatch);
        }
        if donors
            .iter()
            .chain(treated.iter())
            .chain(pre_donors.iter())
            .chain(pre_treated.iter())
            .any(|value| !value.is_finite())
        {
            return Err(SynthError::NonFiniteInput);
        }
        let mut scales = Vec::with_capacity(predictors);
        for p in 0..predictors {
            // The package rejects constant donor predictors even if the treated
            // value differs. Preserve that boundary rather than inventing a scale.
            if (1..controls).all(|c| donors[(p, c)] == donors[(p, 0)]) {
                return Err(SynthError::ConstantDonorPredictor { predictor: p });
            }
            // R stats::var uses an extended two-pass mean, rounds that mean
            // to binary64, then accumulates squared binary64 deviations in
            // extended precision. Reuse qd, already in the faer dependency
            // graph, instead of introducing a native-only long-double path.
            let values: Vec<f64> = (0..controls)
                .map(|c| donors[(p, c)])
                .chain(std::iter::once(treated[p]))
                .collect();
            let scale = super::statistics::variance(&values).sqrt();
            if !scale.is_finite() || scale == 0.0 {
                return Err(SynthError::NonFiniteInput);
            }
            scales.push(scale);
        }
        Ok(Self {
            scaled_donors: DMatrix::from_fn(predictors, controls, |p, c| {
                donors[(p, c)] * (1. / scales[p])
            }),
            scaled_treated: DVector::from_fn(predictors, |p, _| treated[p] * (1. / scales[p])),
            predictor_scales: scales,
            pre_donors: pre_donors.clone(),
            pre_treated: pre_treated.clone(),
        })
    }
}

fn normalize_predictor_weights(weights: &[f64], predictors: usize) -> Result<Vec<f64>, SynthError> {
    if weights.len() != predictors || weights.iter().any(|v| !v.is_finite() || *v < 0.) {
        return Err(SynthError::InvalidPredictorWeights);
    }
    let total = rounded_nonnegative_sum(weights).ok_or(SynthError::InvalidPredictorWeights)?;
    if total <= 0. {
        return Err(SynthError::InvalidPredictorWeights);
    }
    Ok(weights.iter().map(|v| v / total).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regression_start_stages_match_pinned_r_exactly() {
        let root: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/fixtures/synth-full-fit.json"))
                .unwrap();
        let case = &root["cases"][0];
        let e = &case["exact"];
        let exact = |v: &serde_json::Value| {
            let bytes: Vec<u8> = serde_json::from_value(v.clone()).unwrap();
            bytes
                .chunks_exact(8)
                .map(|b| f64::from_le_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>()
        };
        let check = |label: &str, actual: &[f64]| {
            let expected = exact(&e[label]);
            assert_eq!(actual.len(), expected.len(), "{label}");
            let mismatch = actual
                .iter()
                .zip(&expected)
                .filter(|(a, b)| a.to_bits() != b.to_bits())
                .count();
            let error = actual
                .iter()
                .zip(&expected)
                .map(|(a, b)| (a - b).abs())
                .fold(0., f64::max);
            println!("{label}: mismatches {mismatch}, max error {error}");
            assert_eq!(mismatch, 0, "{label} differs from pinned R");
        };
        let p = case["p"].as_u64().unwrap() as usize;
        let n = case["n"].as_u64().unwrap() as usize + 1;
        let t = case["t"].as_u64().unwrap() as usize;
        let design = DMatrix::from_column_slice(n, p + 1, &exact(&e["design"]));
        let normal = oracle_product(&design.transpose(), &design).unwrap();
        check("normal", normal.as_slice());
        let k = p + 1;
        let mut lu = normal.as_slice().to_vec();
        let mut pivots = vec![0; k];
        dgetrf_fused(k, k, &mut lu, k, &mut pivots).unwrap();
        check("lu", &lu);
        assert_eq!(
            pivots,
            serde_json::from_value::<Vec<usize>>(e["pivots"].clone()).unwrap()
        );
        let mut inverse = DMatrix::<f64>::identity(k, k);
        dgetrs_fused(
            Transpose::None,
            k,
            k,
            &lu,
            k,
            &pivots,
            inverse.as_mut_slice(),
            k,
        )
        .unwrap();
        check("inverse", inverse.as_slice());
        let left = oracle_product(&inverse, &design.transpose()).unwrap();
        check("left", left.as_slice());
        let z0 = DMatrix::from_column_slice(t, n - 1, &exact(&e["z0"]));
        let z1 = exact(&e["z1"]);
        let outcomes = DMatrix::from_fn(n, t, |r, c| if r == 0 { z1[c] } else { z0[(c, r - 1)] });
        let beta = oracle_product(&left, &outcomes).unwrap();
        let coefficients = beta.rows(1, p).into_owned();
        check("beta", coefficients.as_slice());
        let raw = SynthMatrices::prepare(
            &DMatrix::from_column_slice(p, n - 1, &exact(&e["x0"])),
            &DVector::from_vec(exact(&e["x1"])),
            &z0,
            &DVector::from_vec(z1),
        )
        .unwrap();
        check("scale", &raw.predictor_scales);
        check("sx0", raw.scaled_donors.as_slice());
        check("regression", &raw.regression_start().unwrap().unwrap());
    }

    #[test]
    fn sample_scaling_includes_treated_and_does_not_center() {
        let x = DMatrix::from_row_slice(2, 2, &[1., 3., 10., 30.]);
        let y = DVector::from_row_slice(&[5., 50.]);
        let z = DMatrix::from_row_slice(1, 2, &[2., 4.]);
        let matrices = SynthMatrices::prepare(&x, &y, &z, &DVector::from_element(1, 3.)).unwrap();
        assert_eq!(matrices.predictor_scales, vec![2., 20.]);
        assert_eq!(
            matrices.scaled_donors,
            DMatrix::from_row_slice(2, 2, &[0.5, 1.5, 0.5, 1.5])
        );
        assert_eq!(
            matrices.scaled_treated,
            DVector::from_row_slice(&[2.5, 2.5])
        );
    }

    #[test]
    fn constant_donor_predictor_is_rejected_even_when_treated_differs() {
        let result = SynthMatrices::prepare(
            &DMatrix::from_element(1, 2, 2.),
            &DVector::from_element(1, 5.),
            &DMatrix::from_element(1, 2, 1.),
            &DVector::from_element(1, 1.),
        );
        assert!(matches!(
            result,
            Err(SynthError::ConstantDonorPredictor { predictor: 0 })
        ));
    }

    #[test]
    fn mismatched_periods_and_nonfinite_values_are_rejected() {
        let x = DMatrix::from_row_slice(1, 2, &[1., 2.]);
        let y = DVector::from_element(1, 3.);
        assert!(matches!(
            SynthMatrices::prepare(&x, &y, &x, &DVector::zeros(2)),
            Err(SynthError::DimensionMismatch)
        ));
        assert!(matches!(
            SynthMatrices::prepare(&x, &y, &x, &DVector::from_element(1, f64::NAN)),
            Err(SynthError::NonFiniteInput)
        ));
    }
}
