//! Tigramite CondIndTest automatic block length, with SciPy Hilbert/curve_fit
//! prerequisites. FFT and MINPACK operations reuse pinned library ports.
use super::{Role, Samples};
use levenberg_marquardt::{LeastSquaresProblem, LevenbergMarquardt};
use nalgebra::{storage::Owned, DVector, Dyn, OMatrix, Vector2, U2};
use rustfft::{num_complex::Complex, FftPlanner};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockMode {
    Significance,
    Confidence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockError {
    Empty,
    NonFinite,
    TooFewPoints,
    FitFailed,
    InvalidDecay,
}

#[derive(Debug)]
pub struct BlockEstimate {
    pub length: usize,
    /// Source RuntimeErrors leave the current length unchanged, despite the
    /// printed fallback message. Retain the affected row indices for callers.
    pub failed_fits: Vec<usize>,
}

/// scipy.signal.hilbert, real one-dimensional input, default N, then abs.
pub fn hilbert_envelope(values: &[f64]) -> Result<Vec<f64>, BlockError> {
    let n = values.len();
    if n == 0 {
        return Err(BlockError::Empty);
    }
    if values.iter().any(|v| !v.is_finite()) {
        return Err(BlockError::NonFinite);
    }
    let mut planner = FftPlanner::<f64>::new();
    let mut spectrum: Vec<_> = values.iter().map(|&re| Complex::new(re, 0.0)).collect();
    planner.plan_fft_forward(n).process(&mut spectrum);
    for (i, v) in spectrum.iter_mut().enumerate() {
        let h = if i == 0 || (n % 2 == 0 && i == n / 2) {
            1.0
        } else if i < (n + 1) / 2 {
            2.0
        } else {
            0.0
        };
        *v *= h;
    }
    planner.plan_fft_inverse(n).process(&mut spectrum);
    Ok(spectrum
        .into_iter()
        .map(|v| (v / n as f64).norm())
        .collect())
}

struct DecayProblem {
    params: Vector2<f64>,
    target: DVector<f64>,
    #[cfg(test)]
    trace: std::cell::RefCell<Vec<[f64; 2]>>,
}

impl DecayProblem {
    fn at(&self, params: Vector2<f64>) -> DVector<f64> {
        #[cfg(test)]
        self.trace.borrow_mut().push([params[0], params[1]]);
        DVector::from_fn(self.target.len(), |i, _| {
            params[0] * params[1].powf(i as f64) - self.target[i]
        })
    }
}

impl LeastSquaresProblem<f64, Dyn, U2> for DecayProblem {
    type ResidualStorage = Owned<f64, Dyn>;
    type JacobianStorage = Owned<f64, Dyn, U2>;
    type ParameterStorage = Owned<f64, U2>;
    fn set_params(&mut self, params: &Vector2<f64>) {
        self.params = *params;
    }
    fn params(&self) -> Vector2<f64> {
        self.params
    }
    fn residuals(&self) -> Option<DVector<f64>> {
        Some(self.at(self.params))
    }
    fn jacobian(&self) -> Option<OMatrix<f64, Dyn, U2>> {
        // MINPACK fdjac2.f: forward differences, not a central/analytic Jacobian.
        let baseline = self.at(self.params);
        // SciPy 1.13.1's vendored dpmpar(1) is this rounded MINPACK constant,
        // slightly larger than f64::EPSILON; fdjac2 takes max(epsfcn, epsmch).
        let eps = 2.22044604926e-16_f64.sqrt();
        let mut jacobian = OMatrix::<f64, Dyn, U2>::zeros_generic(Dyn(self.target.len()), U2);
        for j in 0..2 {
            let mut shifted = self.params;
            let mut h = eps * shifted[j].abs();
            if h == 0.0 {
                h = eps;
            }
            shifted[j] += h;
            let changed = self.at(shifted);
            for i in 0..self.target.len() {
                jacobian[(i, j)] = (changed[i] - baseline[i]) / h;
            }
        }
        Some(jacobian)
    }
    fn jacobian_evaluations(&self) -> usize {
        2
    }
}

/// scipy.optimize.curve_fit(a * decay**x), default p0=[1,1], unweighted LM.
/// This prerequisite remains subject to direct parameter/termination parity.
pub fn fit_decay(envelope: &[f64]) -> Result<[f64; 2], BlockError> {
    if envelope.len() < 2 {
        return Err(BlockError::TooFewPoints);
    }
    if envelope.iter().any(|v| !v.is_finite()) {
        return Err(BlockError::NonFinite);
    }
    let problem = DecayProblem {
        params: Vector2::new(1.0, 1.0),
        target: DVector::from_column_slice(envelope),
        #[cfg(test)]
        trace: Default::default(),
    };
    let (fit, report) = LevenbergMarquardt::new()
        .with_tol(1.49012e-8)
        .with_stepbound(100.0)
        .with_patience(200)
        .minimize(problem);
    if !report.termination.was_successful() {
        return Err(BlockError::FitFailed);
    }
    Ok([fit.params[0], fit.params[1]])
}

fn acf(series: &[f64], max_lag: usize) -> Vec<f64> {
    let mut values = vec![1.0];
    for lag in 1..=max_lag {
        // CondIndTest._get_acf uses separately centered np.corrcoef at each lag,
        // not the fixed-denominator statsmodels ACF estimator.
        let mut rows = [
            series[lag..].to_vec(),
            series[..series.len() - lag].to_vec(),
        ];
        values.push(super::correlation_matrix(&mut rows)[(0, 1)]);
    }
    values
}

impl Samples {
    pub fn automatic_block_length(&self, mode: BlockMode) -> Result<BlockEstimate, BlockError> {
        let t = self.rows[0].len();
        let max_lag = (0.1 * t as f64) as usize;
        let mut length = 1;
        let mut failed_fits = Vec::new();
        for (i, row) in self.rows.iter().enumerate() {
            if mode == BlockMode::Significance && self.roles[i] != Role::X {
                continue;
            }
            let envelope = hilbert_envelope(&acf(row, max_lag))?;
            let phi = match fit_decay(&envelope) {
                Ok([_, phi]) => phi,
                Err(BlockError::FitFailed) => {
                    failed_fits.push(i);
                    continue;
                }
                Err(error) => return Err(error),
            };
            let optimal =
                (4.0 * t as f64 * (phi / (1.0 - phi) + phi.powi(2) / (1.0 - phi).powi(2)).powi(2)
                    / (1.0 + 2.0 * phi / (1.0 - phi)).powi(2))
                .powf(1.0 / 3.0);
            if !optimal.is_finite() || optimal < 0.0 || optimal >= usize::MAX as f64 {
                return Err(BlockError::InvalidDecay);
            }
            length = length.max(optimal as usize);
        }
        Ok(BlockEstimate {
            length: length.min(max_lag),
            failed_fits,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decay_evaluation_limits_match_scipy() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../oracle/fixtures/parcorr_mult_block_length.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        for case in fixture["limits"].as_array().unwrap() {
            let target: Vec<f64> = serde_json::from_value(case["target"].clone()).unwrap();
            let problem = DecayProblem {
                params: Vector2::new(1.0, 1.0),
                target: DVector::from_vec(target),
                trace: Default::default(),
            };
            let (_, report) = LevenbergMarquardt::new()
                .with_tol(1.49012e-8)
                .with_stepbound(100.0)
                .with_patience(case["patience"].as_u64().unwrap() as usize)
                .minimize(problem);
            let status = case["status"].as_u64().unwrap();
            use levenberg_marquardt::TerminationReason;
            let actual_status = match report.termination {
                TerminationReason::Converged {
                    ftol: true,
                    xtol: false,
                } => 1,
                TerminationReason::Converged {
                    ftol: false,
                    xtol: true,
                } => 2,
                TerminationReason::Converged {
                    ftol: true,
                    xtol: true,
                } => 3,
                TerminationReason::Orthogonal => 4,
                TerminationReason::LostPatience => 5,
                _ => 0,
            };
            if report.number_of_evaluations != case["evaluations"].as_u64().unwrap() as usize
                || actual_status != status
            {
                failures.push(format!(
                    "{}: actual {:?} after {}, source status {} after {}",
                    case["name"],
                    report.termination,
                    report.number_of_evaluations,
                    status,
                    case["evaluations"]
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "{} failures:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }

    #[test]
    fn decay_residuals_and_forward_differences_match_source() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../oracle/fixtures/parcorr_mult_block_length.json"
        ))
        .unwrap();
        let mut failures = Vec::new();
        let mut max_residual_error = 0.0_f64;
        let mut max_jacobian_error = 0.0_f64;
        for (index, case) in fixture["evaluations"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            let target: Vec<f64> = serde_json::from_value(case["target"].clone()).unwrap();
            let params: [f64; 2] = serde_json::from_value(case["params"].clone()).unwrap();
            let problem = DecayProblem {
                params: Vector2::from(params),
                target: DVector::from_vec(target),
                trace: Default::default(),
            };
            let residuals = problem.residuals().unwrap();
            let jacobian = problem.jacobian().unwrap();
            let expected: Vec<f64> = serde_json::from_value(case["residual"].clone()).unwrap();
            let residual_error = residuals
                .iter()
                .zip(expected)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0, f64::max);
            let jacobian_error = (0..jacobian.nrows())
                .flat_map(|i| (0..2).map(move |j| (i, j)))
                .map(|(i, j)| (jacobian[(i, j)] - case["jacobian"][i][j].as_f64().unwrap()).abs())
                .fold(0.0, f64::max);
            max_residual_error = max_residual_error.max(residual_error);
            max_jacobian_error = max_jacobian_error.max(jacobian_error);
            if residual_error > 2e-9 || jacobian_error > 2e-9 {
                failures.push(format!("evaluation {index}, params {params:?}: residual error {residual_error:e}, jacobian error {jacobian_error:e}"));
            }
        }
        eprintln!("maximum residual error {max_residual_error:e}; maximum Jacobian error {max_jacobian_error:e}");
        assert!(
            failures.is_empty(),
            "{} failing evaluations:\n{}",
            failures.len(),
            failures
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    #[test]
    fn decay_update_trace_matches_source() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../oracle/fixtures/parcorr_mult_block_length.json"
        ))
        .unwrap();
        let case = fixture["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["name"] == "43-0.0-significance")
            .unwrap();
        let expected = &case["fits"][1];
        let target: Vec<f64> = serde_json::from_value(expected["envelope"].clone()).unwrap();
        let problem = DecayProblem {
            params: Vector2::new(1.0, 1.0),
            target: DVector::from_vec(target),
            trace: Default::default(),
        };
        let (fit, _) = LevenbergMarquardt::new()
            .with_tol(1.49012e-8)
            .with_stepbound(100.0)
            .with_patience(200)
            .minimize(problem);
        let mut got = fit.trace.into_inner();
        let mut expected: Vec<[f64; 2]> =
            serde_json::from_value(expected["trace"].clone()).unwrap();
        got.dedup();
        expected.dedup();
        for (i, (a, b)) in got.iter().zip(&expected).enumerate() {
            assert_eq!(
                a,
                b,
                "first differing update {i}; preceding actual {:?}, source {:?}",
                got.get(i.saturating_sub(1)),
                expected.get(i.saturating_sub(1))
            );
        }
        assert_eq!(got.len(), expected.len());
    }
}
