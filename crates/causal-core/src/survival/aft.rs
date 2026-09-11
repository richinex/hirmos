//! Penalized right-censored AFT objectives on lifelines-normalized designs.
//! Includes lifelines' univariate initialization and explicit normalized starts.

use crate::lapack_dgesdd::{dgesdd, SvdJob};
use crate::lapack_lu::{dgetrf, dgetrs, Transpose};
use crate::slsqp::driver::{Bound, Evaluation, Problem, Termination};
use spec_math::cephes64::{chdtrc, ndtri};

#[derive(Clone, Copy)]
pub enum AftFamily {
    Weibull,
    LogLogistic,
}

#[derive(Debug, PartialEq)]
pub enum AftError {
    InvalidDesign,
    InvalidParameters,
    NonfiniteEvaluation,
}

pub struct NormalizedAftData {
    primary: usize,
    columns: usize,
    design: Vec<f64>,
    log_times: Vec<f64>,
    events: Vec<bool>,
    penalized: Vec<bool>,
}

pub struct AftEvaluation {
    pub objective: f64,
    pub gradient: Vec<f64>,
    pub hessian: Vec<f64>,
}

#[derive(Debug)]
pub enum AftFitError {
    InvalidSettings,
    Objective(AftError),
    OptimizerInput,
    OptimizerStopped(i64),
    Covariance,
    Initialization,
}

#[derive(Debug, PartialEq)]
pub enum AftCovarianceMethod {
    Inverse,
    PseudoInverse,
}

pub struct AftCoefficient {
    pub estimate: f64,
    pub standard_error: f64,
    pub z: f64,
    pub p_value: f64,
    pub lower: f64,
    pub upper: f64,
}

pub struct AftFit {
    family: AftFamily,
    primary: usize,
    pub coefficients: Vec<AftCoefficient>,
    /// Row-major, on the original covariate scales.
    pub covariance: Vec<f64>,
    pub covariance_method: AftCovarianceMethod,
    /// lifelines stores the penalized log likelihood here.
    pub log_likelihood: f64,
    pub aic: f64,
    pub bic: f64,
    pub iterations: usize,
}

impl AftFit {
    /// Unconditional median duration on the original, unnormalized design.
    /// Include both parameter groups' intercept columns, in fitted column order.
    pub fn predict_medians(&self, design: &[f64]) -> Result<Vec<f64>, AftError> {
        let d = self.coefficients.len();
        if design.len() % d != 0 || design.iter().any(|x| !x.is_finite()) {
            return Err(AftError::InvalidDesign);
        }
        Ok(design
            .chunks_exact(d)
            .map(|row| {
                let score = |begin: usize, end: usize| {
                    (begin..end)
                        .map(|j| row[j] * self.coefficients[j].estimate)
                        .sum::<f64>()
                        .exp()
                };
                let scale = score(0, self.primary);
                let shape = score(self.primary, d);
                match self.family {
                    AftFamily::Weibull => scale * std::f64::consts::LN_2.powf(1.0 / shape),
                    AftFamily::LogLogistic => scale,
                }
            })
            .collect())
    }

    pub fn concordance(
        &self,
        design: &[f64],
        times: &[f64],
        events: &[bool],
    ) -> Result<Option<super::coxph::ConcordanceIndex>, AftError> {
        if times.len().checked_mul(self.coefficients.len()) != Some(design.len())
            || times.len() != events.len()
            || times.iter().any(|t| !t.is_finite() || *t <= 0.0)
        {
            return Err(AftError::InvalidDesign);
        }
        let medians = self.predict_medians(design)?;
        if medians.iter().any(|m| !m.is_finite()) {
            return Err(AftError::NonfiniteEvaluation);
        }
        let events: Vec<_> = events
            .iter()
            .map(|e| {
                if *e {
                    super::coxph::Event::Observed
                } else {
                    super::coxph::Event::Censored
                }
            })
            .collect();
        Ok(super::coxph::prediction_concordance_index(
            times, &events, &medians, None,
        ))
    }
}

fn covariance(hessian: &[f64], d: usize) -> Result<(Vec<f64>, AftCovarianceMethod), AftFitError> {
    let mut matrix = vec![0.0; d * d];
    for i in 0..d {
        for j in 0..d {
            matrix[i + j * d] = hessian[i * d + j];
        }
    }
    let mut lu = matrix.clone();
    let mut pivots = vec![0; d];
    let info = dgetrf(d, d, &mut lu, d, &mut pivots).map_err(|_| AftFitError::Covariance)?;
    let mut inverse = vec![0.0; d * d];
    if info == 0 {
        for i in 0..d {
            inverse[i + i * d] = 1.0;
        }
        dgetrs(Transpose::None, d, d, &lu, d, &pivots, &mut inverse, d)
            .map_err(|_| AftFitError::Covariance)?;
        return Ok((inverse, AftCovarianceMethod::Inverse));
    }
    // NumPy's default pinv uses SVD and cutoff 1e-15 * largest singular value.
    let (info, svd) =
        dgesdd(SvdJob::Some, d, d, &mut matrix, d).map_err(|_| AftFitError::Covariance)?;
    if info != 0 {
        return Err(AftFitError::Covariance);
    }
    let u = svd.left_vectors.ok_or(AftFitError::Covariance)?;
    let vt = svd
        .right_vectors_transposed
        .ok_or(AftFitError::Covariance)?;
    let cutoff = 1e-15 * svd.singular_values[0];
    for (k, &singular) in svd.singular_values.iter().enumerate() {
        if singular <= cutoff {
            continue;
        }
        for i in 0..d {
            for j in 0..d {
                inverse[i + j * d] += vt[k + i * d] * (u[j + k * d] / singular);
            }
        }
    }
    Ok((inverse, AftCovarianceMethod::PseudoInverse))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covariance_inverse_and_fallback_match_numpy() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../reproductions/clsa/data/aft-optimizer.json"
        ))
        .unwrap();
        for case in fixture["inverse_cases"].as_array().unwrap() {
            let rows = case["matrix"].as_array().unwrap();
            let matrix: Vec<f64> = rows
                .iter()
                .flat_map(|r| r.as_array().unwrap())
                .map(|v| v.as_f64().unwrap())
                .collect();
            let (actual, method) = covariance(&matrix, rows.len()).unwrap();
            assert_eq!(
                method,
                match case["method"].as_str().unwrap() {
                    "inverse" => AftCovarianceMethod::Inverse,
                    "pseudo_inverse" => AftCovarianceMethod::PseudoInverse,
                    other => panic!("unexpected method {other}"),
                }
            );
            for i in 0..rows.len() {
                for j in 0..rows.len() {
                    let expected = case["inverse"][i][j].as_f64().unwrap();
                    assert!(
                        (actual[i + j * rows.len()] - expected).abs() < 2e-13,
                        "{}",
                        case["name"]
                    );
                }
            }
        }
    }
}

/// Source normalization and penalty selection, retained with the design.
pub struct AftPreparation {
    data: NormalizedAftData,
    deviations: Vec<f64>,
    times: Vec<f64>,
    initial_column: usize,
}

impl AftPreparation {
    /// The design already includes the requested intercept columns. Like
    /// lifelines, divide by sample standard deviation without mean centering.
    pub fn new(
        primary: usize,
        columns: usize,
        mut design: Vec<f64>,
        times: Vec<f64>,
        events: Vec<bool>,
    ) -> Result<Self, AftError> {
        if times.len() < 2
            || primary == 0
            || primary >= columns
            || times.len().checked_mul(columns) != Some(design.len())
            || design.iter().any(|value| !value.is_finite())
        {
            return Err(AftError::InvalidDesign);
        }
        let n = times.len() as f64;
        let mut deviations = Vec::with_capacity(columns);
        let mut penalized = Vec::with_capacity(columns);
        let mut initial_column = None;
        for column in 0..columns {
            let mean = design
                .chunks_exact(columns)
                .map(|row| row[column])
                .sum::<f64>()
                / n;
            let variance = design
                .chunks_exact(columns)
                .map(|row| (row[column] - mean).powi(2))
                .sum::<f64>()
                / (n - 1.0);
            let deviation = variance.sqrt();
            if !deviation.is_finite() {
                return Err(AftError::InvalidDesign);
            }
            let constant = deviation < 1e-8;
            if constant && initial_column.is_none() {
                initial_column = Some(column);
            }
            let group_width = if column < primary {
                primary
            } else {
                columns - primary
            };
            penalized.push(!(group_width > 1 && constant));
            deviations.push(if constant { 1.0 } else { deviation });
        }
        for row in design.chunks_exact_mut(columns) {
            for column in 0..columns {
                row[column] /= deviations[column];
            }
        }
        let data = NormalizedAftData::new(primary, penalized, design, times.clone(), events)?;
        Ok(Self {
            data,
            deviations,
            times,
            initial_column: initial_column.unwrap_or(0),
        })
    }

    /// Fit the univariate distribution, then place its logged parameter in the
    /// first constant column. Source idxmax selects column zero if none is constant.
    pub fn automatic_start(&self, family: AftFamily) -> Result<Vec<f64>, AftFitError> {
        let scale = match family {
            AftFamily::Weibull => self.times.iter().sum::<f64>() / self.times.len() as f64,
            AftFamily::LogLogistic => {
                let mut sorted = self.times.clone();
                sorted.sort_by(f64::total_cmp);
                let middle = sorted.len() / 2;
                if sorted.len() % 2 == 0 {
                    (sorted[middle - 1] + sorted[middle]) / 2.0
                } else {
                    sorted[middle]
                }
            }
        };
        let univariate = NormalizedAftData::new(
            1,
            vec![false; 2],
            vec![1.0; self.times.len() * 2],
            self.times.clone(),
            self.data.events.clone(),
        )
        .map_err(AftFitError::Objective)?;
        let evaluate = |point: &[f64]| {
            univariate.evaluate_derivatives(family, &[point[0].ln(), point[1].ln()], 0.0, false)
        };
        let simplex = crate::nelder_mead::minimize(
            &[scale, 1.0],
            &[1e-9; 2],
            &[f64::INFINITY; 2],
            400,
            |point| evaluate(point).map(|value| value.objective),
            |_| {},
        )
        .map_err(|_| AftFitError::Initialization)?;
        let factr = match family {
            AftFamily::Weibull => 1e-14 / f64::EPSILON,
            AftFamily::LogLogistic => 1e7,
        };
        let mut evaluation_error = None;
        let refined = crate::lbfgsb::lbfgsb(
            &simplex.parameters,
            &[1e-9; 2],
            &[0.0; 2],
            &[1; 2],
            10,
            factr,
            1e-5,
            20,
            15000,
            |point| match evaluate(point) {
                Ok(value) => (
                    value.objective,
                    value
                        .gradient
                        .iter()
                        .zip(point)
                        .map(|(g, p)| g / p)
                        .collect(),
                ),
                Err(error) => {
                    evaluation_error = Some(error);
                    (f64::NAN, vec![f64::NAN; 2])
                }
            },
        );
        if let Some(error) = evaluation_error {
            return Err(AftFitError::Objective(error));
        }
        let simplex_valid = simplex.termination == crate::nelder_mead::Termination::Converged
            && simplex.value.is_finite();
        let refined_valid = refined.termination.converged() && refined.f.is_finite();
        let best = match (simplex_valid, refined_valid) {
            (true, true) if refined.f < simplex.value => &refined.x,
            (true, _) => &simplex.parameters,
            (false, true) => &refined.x,
            (false, false) => return Err(AftFitError::Initialization),
        };
        let group = usize::from(self.initial_column >= self.data.primary);
        let mut initial = vec![0.0; self.data.columns];
        initial[self.initial_column] = best[group].ln();
        Ok(initial)
    }

    pub fn fit(&self, family: AftFamily, penalty: f64, alpha: f64) -> Result<AftFit, AftFitError> {
        self.fit_from_normalized_start(family, self.automatic_start(family)?, penalty, alpha)
    }

    pub fn standard_deviations(&self) -> &[f64] {
        &self.deviations
    }

    pub fn penalty_mask(&self) -> &[bool] {
        &self.data.penalized
    }

    /// Source fit with a caller-supplied normalized starting vector. This does
    /// not run the default univariate-model initialization.
    pub fn fit_from_normalized_start(
        &self,
        family: AftFamily,
        initial: Vec<f64>,
        penalty: f64,
        alpha: f64,
    ) -> Result<AftFit, AftFitError> {
        if !penalty.is_finite()
            || penalty < 0.0
            || !alpha.is_finite()
            || alpha <= 0.0
            || alpha >= 1.0
        {
            return Err(AftFitError::InvalidSettings);
        }
        let d = self.data.columns;
        if initial.len() != d {
            return Err(AftFitError::InvalidSettings);
        }
        let tolerance = match family {
            AftFamily::Weibull => 1e-10,
            AftFamily::LogLogistic => 1e-6,
        };
        let problem = Problem::new(initial, &vec![Bound::Unbounded; d], 0, 0, tolerance, 200)
            .map_err(|_| AftFitError::OptimizerInput)?;
        let fit = problem
            .minimize(
                |parameters| {
                    let result = self
                        .data
                        .evaluate_derivatives(family, parameters, penalty, false)?;
                    Ok::<_, AftError>(Evaluation {
                        value: result.objective,
                        gradient: result.gradient,
                        constraints: vec![],
                        normals: vec![],
                    })
                },
                |_, _| {},
            )
            .map_err(|error| match error {
                crate::slsqp::driver::Error::Evaluation(error) => AftFitError::Objective(error),
                _ => AftFitError::OptimizerInput,
            })?;
        if fit.termination != Termination::Converged {
            return Err(AftFitError::OptimizerStopped(fit.termination.source_code()));
        }
        let n = self.data.events.len() as f64;
        let evaluated = self
            .data
            .evaluate(family, &fit.parameters, penalty)
            .map_err(AftFitError::Objective)?;
        let mut hessian = evaluated.hessian;
        for i in 0..d {
            for j in i..d {
                let entry = 0.5 * (hessian[i * d + j] + hessian[j * d + i]) * n;
                hessian[i * d + j] = entry;
                hessian[j * d + i] = entry;
            }
        }
        let (unit_covariance, covariance_method) = covariance(&hessian, d)?;
        let mut covariance = vec![0.0; d * d];
        for i in 0..d {
            for j in 0..d {
                covariance[i * d + j] =
                    unit_covariance[i + j * d] / (self.deviations[i] * self.deviations[j]);
            }
        }
        let quantile = ndtri(1.0 - alpha / 2.0);
        let mut coefficients = Vec::with_capacity(d);
        for i in 0..d {
            let estimate = fit.parameters[i] / self.deviations[i];
            let standard_error = covariance[i * d + i].sqrt();
            let z = estimate / standard_error;
            coefficients.push(AftCoefficient {
                estimate,
                standard_error,
                z,
                p_value: chdtrc(1.0, z * z),
                lower: estimate - quantile * standard_error,
                upper: estimate + quantile * standard_error,
            });
        }
        let log_likelihood = -n * fit.value;
        Ok(AftFit {
            family,
            primary: self.data.primary,
            coefficients,
            covariance,
            covariance_method,
            log_likelihood,
            aic: -2.0 * log_likelihood + 2.0 * d as f64,
            // Source uses the number of distribution parameter groups, not d.
            bic: -2.0 * log_likelihood + 2.0 * n.ln(),
            iterations: fit.iterations,
        })
    }

    pub fn evaluate(
        &self,
        family: AftFamily,
        parameters: &[f64],
        penalty: f64,
    ) -> Result<AftEvaluation, AftError> {
        self.data.evaluate(family, parameters, penalty)
    }
}

impl NormalizedAftData {
    /// Columns comprise the primary linear predictor followed by the ancillary
    /// predictor. The caller supplies lifelines' penalty mask and normalization.
    pub fn new(
        primary: usize,
        penalized: Vec<bool>,
        design: Vec<f64>,
        times: Vec<f64>,
        events: Vec<bool>,
    ) -> Result<Self, AftError> {
        let columns = penalized.len();
        if times.is_empty()
            || primary == 0
            || primary >= columns
            || events.len() != times.len()
            || times.len().checked_mul(columns) != Some(design.len())
            || design.iter().any(|v| !v.is_finite())
            || times.iter().any(|t| !t.is_finite() || *t <= 0.0)
        {
            return Err(AftError::InvalidDesign);
        }
        Ok(Self {
            primary,
            columns,
            design,
            log_times: times.iter().map(|t| t.ln()).collect(),
            events,
            penalized,
        })
    }

    pub fn evaluate(
        &self,
        family: AftFamily,
        parameters: &[f64],
        penalty: f64,
    ) -> Result<AftEvaluation, AftError> {
        self.evaluate_derivatives(family, parameters, penalty, true)
    }

    fn evaluate_derivatives(
        &self,
        family: AftFamily,
        parameters: &[f64],
        penalty: f64,
        second_order: bool,
    ) -> Result<AftEvaluation, AftError> {
        let d = self.columns;
        if parameters.len() != d
            || parameters.iter().any(|v| !v.is_finite())
            || !penalty.is_finite()
            || penalty < 0.0
        {
            return Err(AftError::InvalidParameters);
        }
        let mut result = AftEvaluation {
            objective: 0.0,
            gradient: vec![0.0; d],
            hessian: vec![0.0; if second_order { d * d } else { 0 }],
        };
        for (row, x) in self.design.chunks_exact(d).enumerate() {
            let eta = (0..self.primary).map(|j| x[j] * parameters[j]).sum::<f64>();
            let kappa = (self.primary..d).map(|j| x[j] * parameters[j]).sum::<f64>();
            let r = kappa.exp();
            let q = r * (self.log_times[row] - eta);
            let event = f64::from(self.events[row]);
            let (value, gradient, hessian) = match family {
                AftFamily::Weibull => {
                    let h = q.exp();
                    (
                        h - event * (kappa - self.log_times[row] + q),
                        [r * (event - h), h * q - event * (1.0 + q)],
                        [
                            r * r * h,
                            r * (event - h * (1.0 + q)),
                            h * (q * q + q) - event * q,
                        ],
                    )
                }
                AftFamily::LogLogistic => {
                    let softplus = q.max(0.0) + (-q.abs()).exp().ln_1p();
                    let s = if q >= 0.0 {
                        1.0 / (1.0 + (-q).exp())
                    } else {
                        q.exp() / (1.0 + q.exp())
                    };
                    let a = (1.0 + event) * s - event;
                    let b = (1.0 + event) * s * (1.0 - s);
                    (
                        (1.0 + event) * softplus - event * (kappa - self.log_times[row] + q),
                        [-r * a, a * q - event],
                        [r * r * b, -r * (a + b * q), b * q * q + a * q],
                    )
                }
            };
            result.objective += value;
            for i in 0..d {
                let group_i = usize::from(i >= self.primary);
                result.gradient[i] += x[i] * gradient[group_i];
                for j in 0..if second_order { d } else { 0 } {
                    let group_j = usize::from(j >= self.primary);
                    result.hessian[i * d + j] += x[i] * x[j] * hessian[group_i + group_j];
                }
            }
        }
        let n = self.log_times.len() as f64;
        result.objective /= n;
        for value in &mut result.gradient {
            *value /= n;
        }
        for value in &mut result.hessian {
            *value /= n;
        }
        for j in 0..d {
            if self.penalized[j] {
                result.objective += 0.5 * penalty * parameters[j] * parameters[j];
                result.gradient[j] += penalty * parameters[j];
                if second_order {
                    result.hessian[j * d + j] += penalty;
                }
            }
        }
        if !result.objective.is_finite()
            || result
                .gradient
                .iter()
                .chain(&result.hessian)
                .any(|v| !v.is_finite())
        {
            return Err(AftError::NonfiniteEvaluation);
        }
        Ok(result)
    }
}
