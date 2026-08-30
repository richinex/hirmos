//! Bayesian Gaussian causal regression for continuous outcomes.
//!
//! The model matches the two-equation Lalonde SCM used by the PathMC/PyMC example:
//!
//! ```text
//! treatment ~ Bernoulli(logit^{-1}(alpha_t + X beta_t))
//! outcome   ~ Normal(alpha_y + tau * treatment + X beta_y, sigma)
//! ```
//!
//! Both intercepts have `Normal(0, 10)` priors, all slopes have `Normal(0, 1)`
//! priors, and `sigma` has a `HalfNormal(10)` prior. Sampling uses the crate's
//! shared multinomial NUTS implementation. The parameterization is unconstrained:
//! `[treatment coefficients, outcome coefficients, log_sigma]`.

use crate::negbin_nuts::quantile;
use crate::nprandom::NpRng;
use crate::nuts::{multinomial_nuts, NutsOptions, NutsResult};

const LOG_2_PI: f64 = 1.8378770664093453;
const INTERCEPT_PRIOR_SD: f64 = 10.0;
const SLOPE_PRIOR_SD: f64 = 1.0;
const SIGMA_PRIOR_SD: f64 = 10.0;

#[derive(Clone, Debug, PartialEq)]
pub enum GaussianScmError {
    EmptyData,
    LengthMismatch,
    RaggedCovariates,
    NonFiniteData,
    NonBinaryTreatment { row: usize, value: f64 },
    ParameterLength { expected: usize, actual: usize },
    EmptyPosterior,
    EmptyPopulation,
    RowOutOfBounds { row: usize, rows: usize },
    InvalidProbabilityMass(f64),
}

#[derive(Clone, Debug)]
pub struct BayesianGaussianScm {
    treatment: Vec<f64>,
    covariates: Vec<Vec<f64>>,
    outcome: Vec<f64>,
    covariate_count: usize,
}

impl BayesianGaussianScm {
    pub fn new(
        treatment: Vec<f64>,
        covariates: Vec<Vec<f64>>,
        outcome: Vec<f64>,
    ) -> Result<Self, GaussianScmError> {
        if treatment.is_empty() {
            return Err(GaussianScmError::EmptyData);
        }
        if treatment.len() != covariates.len() || treatment.len() != outcome.len() {
            return Err(GaussianScmError::LengthMismatch);
        }
        let covariate_count = covariates.first().map_or(0, Vec::len);
        if covariates.iter().any(|row| row.len() != covariate_count) {
            return Err(GaussianScmError::RaggedCovariates);
        }
        if treatment
            .iter()
            .chain(&outcome)
            .chain(covariates.iter().flatten())
            .any(|value| !value.is_finite())
        {
            return Err(GaussianScmError::NonFiniteData);
        }
        for (row, &value) in treatment.iter().enumerate() {
            if value != 0.0 && value != 1.0 {
                return Err(GaussianScmError::NonBinaryTreatment { row, value });
            }
        }
        Ok(Self {
            treatment,
            covariates,
            outcome,
            covariate_count,
        })
    }

    pub fn rows(&self) -> usize {
        self.outcome.len()
    }

    pub fn covariate_count(&self) -> usize {
        self.covariate_count
    }

    /// Intercept plus one coefficient per adjustment variable.
    pub fn treatment_parameter_count(&self) -> usize {
        self.covariate_count + 1
    }

    /// Intercept, treatment effect, then one coefficient per adjustment variable.
    pub fn outcome_parameter_count(&self) -> usize {
        self.covariate_count + 2
    }

    pub fn parameter_count(&self) -> usize {
        self.treatment_parameter_count() + self.outcome_parameter_count() + 1
    }

    pub fn outcome_parameter_start(&self) -> usize {
        self.treatment_parameter_count()
    }

    pub fn treatment_effect_index(&self) -> usize {
        self.outcome_parameter_start() + 1
    }

    pub fn log_sigma_index(&self) -> usize {
        self.parameter_count() - 1
    }

    /// Parameter count for the identified outcome-regression adjustment model:
    /// `[outcome intercept, treatment effect, covariate slopes, log_sigma]`.
    pub fn adjustment_parameter_count(&self) -> usize {
        self.outcome_parameter_count() + 1
    }

    fn validate_parameters(&self, parameters: &[f64]) -> Result<(), GaussianScmError> {
        if parameters.len() != self.parameter_count() {
            return Err(GaussianScmError::ParameterLength {
                expected: self.parameter_count(),
                actual: parameters.len(),
            });
        }
        Ok(())
    }

    /// A deterministic, finite starting point for the unconstrained sampler.
    pub fn initial_position(&self) -> Vec<f64> {
        let mut initial = vec![0.0; self.parameter_count()];
        let mean = self.outcome.iter().sum::<f64>() / self.rows() as f64;
        let variance = self
            .outcome
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / self.rows().max(2).saturating_sub(1) as f64;
        initial[self.outcome_parameter_start()] = mean;
        initial[self.log_sigma_index()] = variance.sqrt().max(1e-6).ln();
        initial
    }

    pub fn adjustment_initial_position(&self) -> Vec<f64> {
        let mut initial = vec![0.0; self.adjustment_parameter_count()];
        let mean = self.outcome.iter().sum::<f64>() / self.rows() as f64;
        let variance = self
            .outcome
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / self.rows().max(2).saturating_sub(1) as f64;
        initial[0] = mean;
        initial[self.adjustment_parameter_count() - 1] = variance.sqrt().max(1e-6).ln();
        initial
    }

    /// Joint log posterior and analytic gradient, including the
    /// `sigma = exp(log_sigma)` transform Jacobian used by PyMC/Pyro.
    pub fn try_log_prob_grad(
        &self,
        parameters: &[f64],
    ) -> Result<(f64, Vec<f64>), GaussianScmError> {
        self.validate_parameters(parameters)?;
        let outcome_start = self.outcome_parameter_start();
        let log_sigma_index = self.log_sigma_index();
        let log_sigma = parameters[log_sigma_index];
        let sigma = log_sigma.exp();
        let sigma_squared = sigma * sigma;

        let mut log_prob = 0.0;
        let mut gradient = vec![0.0; parameters.len()];

        for index in 0..parameters.len() - 1 {
            let is_intercept = index == 0 || index == outcome_start;
            let prior_sd = if is_intercept {
                INTERCEPT_PRIOR_SD
            } else {
                SLOPE_PRIOR_SD
            };
            let prior_variance = prior_sd * prior_sd;
            log_prob += -0.5 * LOG_2_PI
                - prior_sd.ln()
                - 0.5 * parameters[index] * parameters[index] / prior_variance;
            gradient[index] = -parameters[index] / prior_variance;
        }

        // HalfNormal(10) prior on sigma plus the exponential-transform Jacobian.
        log_prob += 0.5 * std::f64::consts::LN_2
            - 0.5 * std::f64::consts::PI.ln()
            - SIGMA_PRIOR_SD.ln()
            - 0.5 * sigma_squared / (SIGMA_PRIOR_SD * SIGMA_PRIOR_SD)
            + log_sigma;
        gradient[log_sigma_index] = 1.0 - sigma_squared / (SIGMA_PRIOR_SD * SIGMA_PRIOR_SD);

        for row in 0..self.rows() {
            let mut treatment_eta = parameters[0];
            for column in 0..self.covariate_count {
                treatment_eta += parameters[column + 1] * self.covariates[row][column];
            }
            let treatment_probability = sigmoid(treatment_eta);
            log_prob += self.treatment[row] * treatment_eta - softplus(treatment_eta);
            let treatment_score = self.treatment[row] - treatment_probability;
            gradient[0] += treatment_score;
            for column in 0..self.covariate_count {
                gradient[column + 1] += treatment_score * self.covariates[row][column];
            }

            let mut outcome_mean =
                parameters[outcome_start] + parameters[outcome_start + 1] * self.treatment[row];
            for column in 0..self.covariate_count {
                outcome_mean +=
                    parameters[outcome_start + 2 + column] * self.covariates[row][column];
            }
            let residual = self.outcome[row] - outcome_mean;
            log_prob += -0.5 * LOG_2_PI - log_sigma - 0.5 * residual * residual / sigma_squared;
            let outcome_score = residual / sigma_squared;
            gradient[outcome_start] += outcome_score;
            gradient[outcome_start + 1] += outcome_score * self.treatment[row];
            for column in 0..self.covariate_count {
                gradient[outcome_start + 2 + column] +=
                    outcome_score * self.covariates[row][column];
            }
            gradient[log_sigma_index] += -1.0 + residual * residual / sigma_squared;
        }

        Ok((log_prob, gradient))
    }

    /// Infallible callback form for the shared NUTS engine. The model and sampler
    /// construct all positions with the required dimension.
    pub fn log_prob_grad(&self, parameters: &[f64]) -> (f64, Vec<f64>) {
        self.try_log_prob_grad(parameters)
            .expect("NUTS supplied the model's parameter dimension")
    }

    pub fn sample(&self, options: NutsOptions, seed: u64) -> NutsResult {
        let mut rng = NpRng::seeded(seed);
        multinomial_nuts(
            &|parameters| self.log_prob_grad(parameters),
            &self.initial_position(),
            options,
            &mut rng,
        )
    }

    /// Outcome-only backdoor adjustment model used when the query is the effect
    /// of treatment on outcome. The full joint posterior factorizes, so omitting
    /// the independent treatment-assignment equation leaves this effect posterior
    /// unchanged while sampling ten rather than eighteen parameters on Lalonde.
    pub fn try_adjustment_log_prob_grad(
        &self,
        parameters: &[f64],
    ) -> Result<(f64, Vec<f64>), GaussianScmError> {
        if parameters.len() != self.adjustment_parameter_count() {
            return Err(GaussianScmError::ParameterLength {
                expected: self.adjustment_parameter_count(),
                actual: parameters.len(),
            });
        }
        let log_sigma_index = parameters.len() - 1;
        let log_sigma = parameters[log_sigma_index];
        let sigma = log_sigma.exp();
        let sigma_squared = sigma * sigma;
        let mut log_prob = 0.0;
        let mut gradient = vec![0.0; parameters.len()];

        for index in 0..log_sigma_index {
            let prior_sd = if index == 0 {
                INTERCEPT_PRIOR_SD
            } else {
                SLOPE_PRIOR_SD
            };
            let prior_variance = prior_sd * prior_sd;
            log_prob += -0.5 * LOG_2_PI
                - prior_sd.ln()
                - 0.5 * parameters[index] * parameters[index] / prior_variance;
            gradient[index] = -parameters[index] / prior_variance;
        }
        log_prob += 0.5 * std::f64::consts::LN_2
            - 0.5 * std::f64::consts::PI.ln()
            - SIGMA_PRIOR_SD.ln()
            - 0.5 * sigma_squared / (SIGMA_PRIOR_SD * SIGMA_PRIOR_SD)
            + log_sigma;
        gradient[log_sigma_index] = 1.0 - sigma_squared / (SIGMA_PRIOR_SD * SIGMA_PRIOR_SD);

        for row in 0..self.rows() {
            let mut mean = parameters[0] + parameters[1] * self.treatment[row];
            for column in 0..self.covariate_count {
                mean += parameters[2 + column] * self.covariates[row][column];
            }
            let residual = self.outcome[row] - mean;
            log_prob += -0.5 * LOG_2_PI - log_sigma - 0.5 * residual * residual / sigma_squared;
            let score = residual / sigma_squared;
            gradient[0] += score;
            gradient[1] += score * self.treatment[row];
            for column in 0..self.covariate_count {
                gradient[2 + column] += score * self.covariates[row][column];
            }
            gradient[log_sigma_index] += -1.0 + residual * residual / sigma_squared;
        }
        Ok((log_prob, gradient))
    }

    pub fn sample_adjustment(&self, options: NutsOptions, seed: u64) -> NutsResult {
        let mut rng = NpRng::seeded(seed);
        multinomial_nuts(
            &|parameters| {
                self.try_adjustment_log_prob_grad(parameters)
                    .expect("NUTS supplied the adjustment model's parameter dimension")
            },
            &self.adjustment_initial_position(),
            options,
            &mut rng,
        )
    }

    pub fn adjustment_effect_draws(
        &self,
        samples: &[Vec<f64>],
        low: f64,
        high: f64,
    ) -> Result<Vec<f64>, GaussianScmError> {
        if samples.is_empty() {
            return Err(GaussianScmError::EmptyPosterior);
        }
        samples
            .iter()
            .map(|parameters| {
                if parameters.len() != self.adjustment_parameter_count() {
                    return Err(GaussianScmError::ParameterLength {
                        expected: self.adjustment_parameter_count(),
                        actual: parameters.len(),
                    });
                }
                let mut low_sum = 0.0;
                let mut high_sum = 0.0;
                for row in &self.covariates {
                    let mut low_mean = parameters[0] + parameters[1] * low;
                    let mut high_mean = parameters[0] + parameters[1] * high;
                    for column in 0..self.covariate_count {
                        let contribution = parameters[2 + column] * row[column];
                        low_mean += contribution;
                        high_mean += contribution;
                    }
                    low_sum += low_mean;
                    high_sum += high_mean;
                }
                Ok((high_sum - low_sum) / self.rows() as f64)
            })
            .collect()
    }

    /// Expected outcome for every observed adjustment row under `do(treatment=value)`.
    /// Residual noise is intentionally excluded: this is posterior g-computation of
    /// the conditional mean, matching PathMC's `do(kind="mean")`.
    pub fn expected_outcomes(
        &self,
        parameters: &[f64],
        intervention: f64,
    ) -> Result<Vec<f64>, GaussianScmError> {
        self.validate_parameters(parameters)?;
        let outcome_start = self.outcome_parameter_start();
        Ok(self
            .covariates
            .iter()
            .map(|row| {
                let mut value =
                    parameters[outcome_start] + parameters[outcome_start + 1] * intervention;
                for column in 0..self.covariate_count {
                    value += parameters[outcome_start + 2 + column] * row[column];
                }
                value
            })
            .collect())
    }

    fn selected_rows<'a>(
        &'a self,
        rows: Option<&'a [usize]>,
    ) -> Result<Vec<usize>, GaussianScmError> {
        let selected = rows
            .map(<[usize]>::to_vec)
            .unwrap_or_else(|| (0..self.rows()).collect());
        if selected.is_empty() {
            return Err(GaussianScmError::EmptyPopulation);
        }
        if let Some(&row) = selected.iter().find(|&&row| row >= self.rows()) {
            return Err(GaussianScmError::RowOutOfBounds {
                row,
                rows: self.rows(),
            });
        }
        Ok(selected)
    }

    /// Posterior draws of the standardized mean under one intervention.
    pub fn intervention_mean_draws(
        &self,
        samples: &[Vec<f64>],
        intervention: f64,
        rows: Option<&[usize]>,
    ) -> Result<Vec<f64>, GaussianScmError> {
        if samples.is_empty() {
            return Err(GaussianScmError::EmptyPosterior);
        }
        let selected = self.selected_rows(rows)?;
        samples
            .iter()
            .map(|parameters| {
                let expected = self.expected_outcomes(parameters, intervention)?;
                Ok(selected.iter().map(|&row| expected[row]).sum::<f64>() / selected.len() as f64)
            })
            .collect()
    }

    /// Posterior draws of `E[Y | do(high)] - E[Y | do(low)]` over a chosen
    /// target population. In this additive model the result equals
    /// `(high - low) * tau` for every draw; the explicit standardization is
    /// retained so the API generalizes to interactions and non-linear links.
    pub fn effect_draws(
        &self,
        samples: &[Vec<f64>],
        low: f64,
        high: f64,
        rows: Option<&[usize]>,
    ) -> Result<Vec<f64>, GaussianScmError> {
        let low_draws = self.intervention_mean_draws(samples, low, rows)?;
        let high_draws = self.intervention_mean_draws(samples, high, rows)?;
        Ok(high_draws
            .iter()
            .zip(low_draws)
            .map(|(high, low)| high - low)
            .collect())
    }

    pub fn ate_draws(&self, samples: &[Vec<f64>]) -> Result<Vec<f64>, GaussianScmError> {
        self.effect_draws(samples, 0.0, 1.0, None)
    }

    pub fn att_draws(&self, samples: &[Vec<f64>]) -> Result<Vec<f64>, GaussianScmError> {
        let rows: Vec<usize> = self
            .treatment
            .iter()
            .enumerate()
            .filter_map(|(row, &value)| (value == 1.0).then_some(row))
            .collect();
        self.effect_draws(samples, 0.0, 1.0, Some(&rows))
    }

    pub fn atu_draws(&self, samples: &[Vec<f64>]) -> Result<Vec<f64>, GaussianScmError> {
        let rows: Vec<usize> = self
            .treatment
            .iter()
            .enumerate()
            .filter_map(|(row, &value)| (value == 0.0).then_some(row))
            .collect();
        self.effect_draws(samples, 0.0, 1.0, Some(&rows))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PosteriorEffectSummary {
    pub mean: f64,
    pub standard_deviation: f64,
    pub median: f64,
    pub hdi_lower: f64,
    pub hdi_upper: f64,
    pub probability_positive: f64,
}

pub fn posterior_effect_summary(
    draws: &[f64],
    probability_mass: f64,
) -> Result<PosteriorEffectSummary, GaussianScmError> {
    if draws.is_empty() {
        return Err(GaussianScmError::EmptyPosterior);
    }
    if !(0.0 < probability_mass && probability_mass < 1.0) {
        return Err(GaussianScmError::InvalidProbabilityMass(probability_mass));
    }
    let mean = draws.iter().sum::<f64>() / draws.len() as f64;
    let standard_deviation = if draws.len() == 1 {
        0.0
    } else {
        (draws
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / (draws.len() - 1) as f64)
            .sqrt()
    };
    let (hdi_lower, hdi_upper) = highest_density_interval(draws, probability_mass)?;
    Ok(PosteriorEffectSummary {
        mean,
        standard_deviation,
        median: quantile(draws, 0.5),
        hdi_lower,
        hdi_upper,
        probability_positive: draws.iter().filter(|&&value| value > 0.0).count() as f64
            / draws.len() as f64,
    })
}

/// Minimum-width interval containing `floor(probability_mass * n)` ordered draws,
/// matching ArviZ's one-dimensional highest-density interval algorithm.
pub fn highest_density_interval(
    draws: &[f64],
    probability_mass: f64,
) -> Result<(f64, f64), GaussianScmError> {
    if draws.is_empty() {
        return Err(GaussianScmError::EmptyPosterior);
    }
    if !(0.0 < probability_mass && probability_mass < 1.0) {
        return Err(GaussianScmError::InvalidProbabilityMass(probability_mass));
    }
    let mut sorted = draws.to_vec();
    sorted.sort_by(|left, right| left.partial_cmp(right).expect("finite posterior draw"));
    let interval_increment = ((probability_mass * sorted.len() as f64).floor() as usize)
        .clamp(1, sorted.len() - usize::from(sorted.len() > 1));
    if sorted.len() == 1 {
        return Ok((sorted[0], sorted[0]));
    }
    let mut best_start = 0usize;
    let mut best_width = f64::INFINITY;
    for start in 0..sorted.len() - interval_increment {
        let width = sorted[start + interval_increment] - sorted[start];
        if width < best_width {
            best_width = width;
            best_start = start;
        }
    }
    Ok((sorted[best_start], sorted[best_start + interval_increment]))
}

fn sigmoid(value: f64) -> f64 {
    if value >= 0.0 {
        1.0 / (1.0 + (-value).exp())
    } else {
        let exponential = value.exp();
        exponential / (1.0 + exponential)
    }
}

fn softplus(value: f64) -> f64 {
    value.max(0.0) + (-value.abs()).exp().ln_1p()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_binary_treatment() {
        let error = BayesianGaussianScm::new(vec![0.5], vec![vec![1.0]], vec![2.0])
            .expect_err("treatment must be binary");
        assert_eq!(
            error,
            GaussianScmError::NonBinaryTreatment { row: 0, value: 0.5 }
        );
    }

    #[test]
    fn hdi_finds_shortest_interval() {
        let interval = highest_density_interval(&[0.0, 1.0, 2.0, 100.0], 0.5).unwrap();
        assert_eq!(interval, (0.0, 2.0));
    }
}
