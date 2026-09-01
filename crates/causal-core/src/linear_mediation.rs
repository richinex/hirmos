//! Focused parity port of Tigramite's `LinearMediation` response and block-bootstrap lane.
//!
//! The fitted equations come from [`DynamicLinearScm`]. Their lag coefficient matrices `Phi` are
//! converted to causal-response matrices `Psi` with Tigramite's recurrence. A point intervention
//! uses the ordinary response. A persistent hard intervention uses the joint response obtained after
//! removing all incoming treatment links, because every later treatment equation remains clamped.

use std::fmt;

use nalgebra::DMatrix;

use crate::causal_effects::{
    bootstrap_block_starts, bootstrap_sample_indices, numpy_percentile, BootstrapBlockLength,
};
use crate::dynamic_counterfactual::{
    DynamicLinearScm, DynamicParentSpec, DynamicScmError, InterventionTiming,
};

#[derive(Clone, Debug)]
pub struct LinearMediationModel {
    pub scm: DynamicLinearScm,
    /// `phi[lag][child, parent]`, matching Tigramite's coefficient matrices.
    pub phi: Vec<DMatrix<f64>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearMediationBootstrapOptions {
    pub samples: usize,
    pub block_length: BootstrapBlockLength,
    /// Tigramite seeds draw `b` with `seed + b` in `LinearMediation.fit_model_bootstrap`.
    pub seed: u64,
}

impl Default for LinearMediationBootstrapOptions {
    fn default() -> Self {
        Self {
            samples: 100,
            block_length: BootstrapBlockLength::CubeRoot,
            seed: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LinearMediationBootstrap {
    pub original_model: LinearMediationModel,
    pub bootstrap_models: Vec<LinearMediationModel>,
    pub block_starts: Vec<Vec<usize>>,
    pub resolved_block_length: usize,
    pub options: LinearMediationBootstrapOptions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicCounterfactualUncertainty {
    /// One high-minus-low contrast path per bootstrap refit.
    pub effect_draws: Vec<Vec<f64>>,
    /// Equal-tail pointwise bounds, ordered `[lower, upper][horizon]`.
    pub pointwise_interval: [Vec<f64>; 2],
    pub average_draws: Vec<f64>,
    pub average_interval: [f64; 2],
    pub cumulative_draws: Vec<f64>,
    pub cumulative_interval: [f64; 2],
    pub confidence_level: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LinearMediationError {
    DynamicScm(DynamicScmError),
    VariableOutOfRange { variable: usize },
    EmptyHorizon,
    ZeroSamples,
    ZeroBlockLength,
    TooFewBlocks { blocks: usize },
    SeedOverflow,
    InvalidConfidenceLevel,
    SingularContemporaneousSystem,
}

impl fmt::Display for LinearMediationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DynamicScm(error) => error.fmt(formatter),
            Self::VariableOutOfRange { variable } => {
                write!(
                    formatter,
                    "variable {variable} is outside the mediation model"
                )
            }
            Self::EmptyHorizon => formatter.write_str("a response horizon must be positive"),
            Self::ZeroSamples => formatter.write_str("bootstrap samples must be positive"),
            Self::ZeroBlockLength => formatter.write_str("bootstrap block length must be positive"),
            Self::TooFewBlocks { blocks } => write!(
                formatter,
                "only {blocks} block(s) are available; choose a shorter bootstrap block length"
            ),
            Self::SeedOverflow => formatter.write_str("bootstrap seed schedule overflowed"),
            Self::InvalidConfidenceLevel => {
                formatter.write_str("confidence level must lie strictly between zero and one")
            }
            Self::SingularContemporaneousSystem => {
                formatter.write_str("the contemporaneous coefficient system has no pseudo-inverse")
            }
        }
    }
}

impl std::error::Error for LinearMediationError {}

impl From<DynamicScmError> for LinearMediationError {
    fn from(error: DynamicScmError) -> Self {
        Self::DynamicScm(error)
    }
}

impl LinearMediationModel {
    pub fn fit(
        data: &[Vec<f64>],
        parents: &[Vec<DynamicParentSpec>],
    ) -> Result<Self, LinearMediationError> {
        Self::from_scm(DynamicLinearScm::fit(data, parents)?)
    }

    pub fn from_scm(scm: DynamicLinearScm) -> Result<Self, LinearMediationError> {
        let variables = scm.equations.len();
        let mut phi = vec![DMatrix::zeros(variables, variables); scm.max_lag + 1];
        for (child, equation) in scm.equations.iter().enumerate() {
            for parent in &equation.parents {
                phi[parent.lag][(child, parent.variable)] = parent.coefficient;
            }
        }
        // Compute psi[0] now so a malformed supplied contemporaneous system is refused at the
        // constructor boundary rather than during a later query.
        let model = Self { scm, phi };
        model.response_matrices(1, None)?;
        Ok(model)
    }

    pub fn point_response(
        &self,
        treatment: usize,
        outcome: usize,
        horizon: usize,
    ) -> Result<Vec<f64>, LinearMediationError> {
        self.validate_query(treatment, outcome, horizon)?;
        Ok(self
            .response_matrices(horizon, None)?
            .into_iter()
            .map(|response| response[(outcome, treatment)])
            .collect())
    }

    pub fn persistent_response(
        &self,
        treatment: usize,
        outcome: usize,
        horizon: usize,
    ) -> Result<Vec<f64>, LinearMediationError> {
        self.validate_query(treatment, outcome, horizon)?;
        let joint = self.response_matrices(horizon, Some(treatment))?;
        let mut cumulative = 0.0;
        Ok(joint
            .into_iter()
            .map(|response| {
                cumulative += response[(outcome, treatment)];
                cumulative
            })
            .collect())
    }

    pub fn contrast_response(
        &self,
        treatment: usize,
        outcome: usize,
        timing: InterventionTiming,
        horizon: usize,
        low: f64,
        high: f64,
    ) -> Result<Vec<f64>, LinearMediationError> {
        let response = match timing {
            InterventionTiming::Point { .. } => self.point_response(treatment, outcome, horizon)?,
            InterventionTiming::Persistent { .. } => {
                self.persistent_response(treatment, outcome, horizon)?
            }
        };
        let difference = high - low;
        Ok(response
            .into_iter()
            .map(|value| value * difference)
            .collect())
    }

    fn validate_query(
        &self,
        treatment: usize,
        outcome: usize,
        horizon: usize,
    ) -> Result<(), LinearMediationError> {
        if horizon == 0 {
            return Err(LinearMediationError::EmptyHorizon);
        }
        for variable in [treatment, outcome] {
            if variable >= self.scm.equations.len() {
                return Err(LinearMediationError::VariableOutOfRange { variable });
            }
        }
        Ok(())
    }

    /// Tigramite `_get_psi`, with zero-padding beyond the fitted maximum lag so a replay horizon
    /// may be longer than the largest parent lag.
    fn response_matrices(
        &self,
        horizon: usize,
        blocked_child: Option<usize>,
    ) -> Result<Vec<DMatrix<f64>>, LinearMediationError> {
        if horizon == 0 {
            return Err(LinearMediationError::EmptyHorizon);
        }
        let variables = self.scm.equations.len();
        if let Some(variable) = blocked_child {
            if variable >= variables {
                return Err(LinearMediationError::VariableOutOfRange { variable });
            }
        }
        let coefficient = |lag: usize| {
            let mut matrix = if lag < self.phi.len() {
                self.phi[lag].clone()
            } else {
                DMatrix::zeros(variables, variables)
            };
            if let Some(child) = blocked_child {
                matrix.row_mut(child).fill(0.0);
            }
            matrix
        };
        let contemporaneous = DMatrix::<f64>::identity(variables, variables) - coefficient(0);
        let decomposition = contemporaneous.svd(true, true);
        let tolerance = decomposition.singular_values.max() * 1e-15;
        let psi_zero = decomposition
            .pseudo_inverse(tolerance)
            .map_err(|_| LinearMediationError::SingularContemporaneousSystem)?;
        let mut psi = vec![DMatrix::zeros(variables, variables); horizon];
        psi[0] = psi_zero.clone();
        for tau in 1..horizon {
            for lag in 1..=tau.min(self.scm.max_lag) {
                let earlier = psi[tau - lag].clone();
                psi[tau] += &psi_zero * coefficient(lag) * earlier;
            }
        }
        Ok(psi)
    }
}

impl LinearMediationBootstrap {
    pub fn fit(
        data: &[Vec<f64>],
        parents: &[Vec<DynamicParentSpec>],
        options: LinearMediationBootstrapOptions,
    ) -> Result<Self, LinearMediationError> {
        Self::fit_with_progress(data, parents, options, |_, _| {})
    }

    pub fn fit_with_progress<F>(
        data: &[Vec<f64>],
        parents: &[Vec<DynamicParentSpec>],
        options: LinearMediationBootstrapOptions,
        mut progress: F,
    ) -> Result<Self, LinearMediationError>
    where
        F: FnMut(usize, usize),
    {
        if options.samples == 0 {
            return Err(LinearMediationError::ZeroSamples);
        }
        let original_model = LinearMediationModel::fit(data, parents)?;
        let rows = data.first().map_or(0, Vec::len);
        let observations = rows - original_model.scm.max_lag;
        let resolved_block_length = match options.block_length {
            BootstrapBlockLength::Fixed(0) => return Err(LinearMediationError::ZeroBlockLength),
            BootstrapBlockLength::Fixed(length) => length,
            BootstrapBlockLength::CubeRoot => {
                ((observations as f64).powf(1.0 / 3.0) as usize).max(1)
            }
        };
        let blocks = observations.div_ceil(resolved_block_length);
        if blocks < 2 {
            return Err(LinearMediationError::TooFewBlocks { blocks });
        }
        let mut bootstrap_models = Vec::with_capacity(options.samples);
        let mut all_starts = Vec::with_capacity(options.samples);
        for draw in 0..options.samples {
            let seed = options
                .seed
                .checked_add(draw as u64)
                .ok_or(LinearMediationError::SeedOverflow)?;
            let starts = bootstrap_block_starts(observations, resolved_block_length, seed)
                .map_err(|error| match error {
                    crate::causal_effects::TotalEffectBootstrapError::ZeroBlockLength => {
                        LinearMediationError::ZeroBlockLength
                    }
                    crate::causal_effects::TotalEffectBootstrapError::TooFewBlocks { blocks } => {
                        LinearMediationError::TooFewBlocks { blocks }
                    }
                    crate::causal_effects::TotalEffectBootstrapError::SeedOverflow => {
                        LinearMediationError::SeedOverflow
                    }
                    crate::causal_effects::TotalEffectBootstrapError::NotIdentifiable
                    | crate::causal_effects::TotalEffectBootstrapError::ZeroSamples => {
                        unreachable!("block-start helper cannot return this error")
                    }
                })?;
            let sampled = bootstrap_sample_indices(observations, resolved_block_length, &starts);
            let references: Vec<usize> = sampled
                .into_iter()
                .map(|position| position + original_model.scm.max_lag)
                .collect();
            let scm = DynamicLinearScm::fit_at_reference_points(data, parents, &references)?;
            bootstrap_models.push(LinearMediationModel::from_scm(scm)?);
            all_starts.push(starts);
            progress(draw + 1, options.samples);
        }
        Ok(Self {
            original_model,
            bootstrap_models,
            block_starts: all_starts,
            resolved_block_length,
            options,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn counterfactual_uncertainty(
        &self,
        treatment: usize,
        outcome: usize,
        timing: InterventionTiming,
        horizon: usize,
        low: f64,
        high: f64,
        confidence_level: f64,
    ) -> Result<DynamicCounterfactualUncertainty, LinearMediationError> {
        if !(0.0..1.0).contains(&confidence_level) {
            return Err(LinearMediationError::InvalidConfidenceLevel);
        }
        let mut effect_draws = Vec::with_capacity(self.bootstrap_models.len());
        let mut average_draws = Vec::with_capacity(self.bootstrap_models.len());
        let mut cumulative_draws = Vec::with_capacity(self.bootstrap_models.len());
        for model in &self.bootstrap_models {
            let effects =
                model.contrast_response(treatment, outcome, timing, horizon, low, high)?;
            let cumulative = effects.iter().sum::<f64>();
            average_draws.push(cumulative / horizon as f64);
            cumulative_draws.push(cumulative);
            effect_draws.push(effects);
        }
        let tail = (1.0 - confidence_level) / 2.0;
        let pointwise_interval = [
            (0..horizon)
                .map(|step| {
                    numpy_percentile(
                        &effect_draws
                            .iter()
                            .map(|draw| draw[step])
                            .collect::<Vec<_>>(),
                        tail,
                    )
                })
                .collect(),
            (0..horizon)
                .map(|step| {
                    numpy_percentile(
                        &effect_draws
                            .iter()
                            .map(|draw| draw[step])
                            .collect::<Vec<_>>(),
                        1.0 - tail,
                    )
                })
                .collect(),
        ];
        Ok(DynamicCounterfactualUncertainty {
            effect_draws,
            pointwise_interval,
            average_interval: [
                numpy_percentile(&average_draws, tail),
                numpy_percentile(&average_draws, 1.0 - tail),
            ],
            cumulative_interval: [
                numpy_percentile(&cumulative_draws, tail),
                numpy_percentile(&cumulative_draws, 1.0 - tail),
            ],
            average_draws,
            cumulative_draws,
            confidence_level,
        })
    }
}
