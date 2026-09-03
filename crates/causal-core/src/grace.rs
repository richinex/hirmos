//! GRACE gated refinement ported from causal-ts 0.26.0 at
//! `cd20a0d54054f7e852e584c9a0db31191ad56477`.

use burn::backend::{Autodiff, NdArray};
use burn::module::{Module, Param};
use burn::optim::decay::WeightDecayConfig;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::tensor::activation::{sigmoid, silu, softplus};
use burn::tensor::backend::Backend;
use burn::tensor::{Tensor, TensorData};
use std::fmt::{Display, Formatter};

use crate::causal_ts_preparation::GraceDenseWindows;
use crate::nprandom::NpRng;

type InnerBackend = NdArray<f32>;
type TrainingBackend = Autodiff<InnerBackend>;

const EPS: f32 = 1e-6;

#[derive(Clone, Debug, PartialEq)]
pub struct GraceConfiguration {
    pub max_lag: usize,
    pub hidden_size: usize,
    pub decoder_hidden: usize,
    pub decoder_layers: usize,
    pub lambda_l0: f64,
    pub temperature: f64,
    pub stretch_gamma: f64,
    pub stretch_zeta: f64,
    pub include_instantaneous: bool,
    pub learning_rate: f64,
    pub weight_decay: f64,
    pub epochs: usize,
    pub patience: usize,
    pub gate_threshold: f64,
    pub seed: u64,
}

impl Default for GraceConfiguration {
    fn default() -> Self {
        Self {
            max_lag: 3,
            hidden_size: 64,
            decoder_hidden: 64,
            decoder_layers: 2,
            lambda_l0: 0.1,
            temperature: 2.0 / 3.0,
            stretch_gamma: -0.1,
            stretch_zeta: 1.1,
            include_instantaneous: false,
            learning_rate: 1e-3,
            weight_decay: 1e-4,
            epochs: 150,
            patience: 20,
            gate_threshold: 0.5,
            seed: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraceLayerParameters {
    /// PyTorch linear layout `[output, input]`.
    pub weight: Vec<f32>,
    pub bias: Vec<f32>,
    pub outputs: usize,
    pub inputs: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraceDecoderParameters {
    pub layers: Vec<GraceLayerParameters>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraceParameters {
    /// Internal source layout `[cause, oldest..current lag, effect]`.
    pub log_alpha: Vec<f32>,
    /// Source layout `[cause, oldest..current lag, 3 * hidden]`.
    pub encoder_weight: Vec<f32>,
    /// Source layout `[cause, oldest..current lag, hidden]`.
    pub encoder_bias: Vec<f32>,
    pub decoders: Vec<GraceDecoderParameters>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraceProgress {
    pub epoch: usize,
    pub maximum_epochs: usize,
    pub loss: f64,
    pub rmse: f64,
    pub active_edges: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraceStep {
    pub epoch: usize,
    pub loss: f64,
    pub rmse: f64,
    /// Public causal-ts layout `[cause, effect, lag 0..max_lag]`.
    pub gate_values: Vec<f64>,
    pub log_alpha: Vec<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraceResult {
    pub variables: usize,
    pub max_lag: usize,
    pub skeleton: Vec<u8>,
    pub gate_values: Vec<f64>,
    pub time_graph: Vec<u8>,
    pub trajectory: Vec<GraceStep>,
    pub parameters: GraceParameters,
    pub epochs: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraceError {
    Shape,
    InvalidConfiguration,
    ParameterShape,
    UniformShape,
}

impl Display for GraceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Shape => "GRACE windows and skeleton have incompatible shapes",
            Self::InvalidConfiguration => "GRACE configuration is invalid",
            Self::ParameterShape => "GRACE parameters do not match the configured model",
            Self::UniformShape => "GRACE hard-concrete draws do not match the configured run",
        })
    }
}

impl std::error::Error for GraceError {}

#[derive(Module, Debug)]
struct Decoder<B: Backend> {
    weights: Vec<Param<Tensor<B, 2>>>,
    biases: Vec<Param<Tensor<B, 1>>>,
}

impl<B: Backend> Decoder<B> {
    fn forward(&self, mut input: Tensor<B, 2>) -> Tensor<B, 2> {
        for index in 0..self.weights.len() {
            input = input.matmul(self.weights[index].val().transpose())
                + self.biases[index].val().unsqueeze_dim::<2>(0);
            if index + 1 < self.weights.len() {
                input = silu(input);
            }
        }
        input
    }
}

#[derive(Module, Debug)]
struct GraceModel<B: Backend> {
    log_alpha: Param<Tensor<B, 3>>,
    encoder_weight: Param<Tensor<B, 3>>,
    encoder_bias: Param<Tensor<B, 3>>,
    decoders: Vec<Decoder<B>>,
}

fn parameter<B: Backend, const D: usize>(
    values: &[f32],
    shape: [usize; D],
    device: &B::Device,
) -> Param<Tensor<B, D>> {
    Param::from_data(TensorData::new(values.to_vec(), shape), device)
}

fn tensor_values<B: Backend, const D: usize>(tensor: Tensor<B, D>) -> Vec<f32> {
    tensor.into_data().to_vec::<f32>().expect("f32 backend")
}

fn normal(rng: &mut NpRng) -> f32 {
    let first = rng.next_f64().max(f64::MIN_POSITIVE);
    let second = rng.next_f64();
    (-2.0 * first.ln())
        .sqrt()
        .mul_add((2.0 * std::f64::consts::PI * second).cos(), 0.0) as f32
}

fn initialize_parameters(
    variables: usize,
    configuration: &GraceConfiguration,
    skeleton: &[u8],
) -> GraceParameters {
    let lags = configuration.max_lag + 1;
    let hidden = configuration.hidden_size;
    let mut rng = NpRng::seeded(configuration.seed);
    let log_alpha = (0..variables * lags * variables)
        .map(|index| {
            if skeleton_internal_at(skeleton, variables, lags, index) {
                0.5
            } else {
                -2.0
            }
        })
        .collect();
    let encoder_deviation = (2.0_f32 / 3.0).sqrt();
    let encoder_weight = (0..variables * lags * 3 * hidden)
        .map(|_| normal(&mut rng) * encoder_deviation)
        .collect();
    let encoder_bias = vec![0.0; variables * lags * hidden];
    let mut decoders = Vec::with_capacity(variables);
    for _ in 0..variables {
        let mut layers = Vec::with_capacity(configuration.decoder_layers + 1);
        let mut inputs = hidden;
        for _ in 0..configuration.decoder_layers {
            let deviation = (2.0_f32 / inputs as f32).sqrt();
            layers.push(GraceLayerParameters {
                weight: (0..configuration.decoder_hidden * inputs)
                    .map(|_| normal(&mut rng) * deviation)
                    .collect(),
                bias: vec![0.0; configuration.decoder_hidden],
                outputs: configuration.decoder_hidden,
                inputs,
            });
            inputs = configuration.decoder_hidden;
        }
        let bound = 0.1 * (6.0_f32 / (inputs + 2) as f32).sqrt();
        layers.push(GraceLayerParameters {
            weight: (0..2 * inputs)
                .map(|_| ((2.0 * rng.next_f64() - 1.0) as f32) * bound)
                .collect(),
            bias: vec![0.0; 2],
            outputs: 2,
            inputs,
        });
        decoders.push(GraceDecoderParameters { layers });
    }
    GraceParameters {
        log_alpha,
        encoder_weight,
        encoder_bias,
        decoders,
    }
}

fn skeleton_internal_at(skeleton: &[u8], variables: usize, lags: usize, index: usize) -> bool {
    let effect = index % variables;
    let remaining = index / variables;
    let internal_lag = remaining % lags;
    let cause = remaining / lags;
    let public_lag = lags - 1 - internal_lag;
    skeleton[(cause * variables + effect) * lags + public_lag] != 0
}

fn model<B: Backend>(
    variables: usize,
    configuration: &GraceConfiguration,
    parameters: &GraceParameters,
    device: &B::Device,
) -> Result<GraceModel<B>, GraceError> {
    let lags = configuration.max_lag + 1;
    let hidden = configuration.hidden_size;
    if parameters.log_alpha.len() != variables * lags * variables
        || parameters.encoder_weight.len() != variables * lags * 3 * hidden
        || parameters.encoder_bias.len() != variables * lags * hidden
        || parameters.decoders.len() != variables
    {
        return Err(GraceError::ParameterShape);
    }
    let mut decoders = Vec::with_capacity(variables);
    for supplied in &parameters.decoders {
        if supplied.layers.len() != configuration.decoder_layers + 1 {
            return Err(GraceError::ParameterShape);
        }
        let mut weights = Vec::with_capacity(supplied.layers.len());
        let mut biases = Vec::with_capacity(supplied.layers.len());
        for layer in &supplied.layers {
            if layer.weight.len() != layer.outputs * layer.inputs
                || layer.bias.len() != layer.outputs
            {
                return Err(GraceError::ParameterShape);
            }
            weights.push(parameter(
                &layer.weight,
                [layer.outputs, layer.inputs],
                device,
            ));
            biases.push(parameter(&layer.bias, [layer.outputs], device));
        }
        decoders.push(Decoder { weights, biases });
    }
    Ok(GraceModel {
        log_alpha: parameter(&parameters.log_alpha, [variables, lags, variables], device),
        encoder_weight: parameter(
            &parameters.encoder_weight,
            [variables, lags, 3 * hidden],
            device,
        ),
        encoder_bias: parameter(&parameters.encoder_bias, [variables, lags, hidden], device),
        decoders,
    })
}

fn extract_parameters<B: Backend>(
    model: &GraceModel<B>,
    variables: usize,
    configuration: &GraceConfiguration,
) -> GraceParameters {
    let decoders = model
        .decoders
        .iter()
        .map(|decoder| GraceDecoderParameters {
            layers: decoder
                .weights
                .iter()
                .zip(&decoder.biases)
                .map(|(weight, bias)| {
                    let [outputs, inputs] = weight.shape().dims();
                    GraceLayerParameters {
                        weight: tensor_values(weight.val()),
                        bias: tensor_values(bias.val()),
                        outputs,
                        inputs,
                    }
                })
                .collect(),
        })
        .collect();
    debug_assert_eq!(
        model.log_alpha.shape().dims(),
        [variables, configuration.max_lag + 1, variables]
    );
    GraceParameters {
        log_alpha: tensor_values(model.log_alpha.val()),
        encoder_weight: tensor_values(model.encoder_weight.val()),
        encoder_bias: tensor_values(model.encoder_bias.val()),
        decoders,
    }
}

fn mask_values(
    variables: usize,
    lags: usize,
    skeleton: &[u8],
    include_instantaneous: bool,
) -> Vec<f32> {
    (0..variables * lags * variables)
        .map(|index| {
            let effect = index % variables;
            let remaining = index / variables;
            let internal_lag = remaining % lags;
            let cause = remaining / lags;
            let allowed_by_time =
                internal_lag + 1 < lags || (include_instantaneous && cause != effect);
            (allowed_by_time && skeleton_internal_at(skeleton, variables, lags, index)) as u8 as f32
        })
        .collect()
}

fn forward<B: Backend>(
    model: &GraceModel<B>,
    input: Tensor<B, 3>,
    uniforms: Tensor<B, 3>,
    mask: Tensor<B, 3>,
    configuration: &GraceConfiguration,
) -> (Tensor<B, 2>, Tensor<B, 2>) {
    let [batch, variables, lags] = input.shape().dims();
    let hidden = configuration.hidden_size;
    let log_alpha = model.log_alpha.val();
    let gates = sigmoid(
        (uniforms.clone().log() - (uniforms.neg() + 1.0).log() + log_alpha)
            / configuration.temperature,
    );
    let gates = (gates * (configuration.stretch_zeta - configuration.stretch_gamma)
        + configuration.stretch_gamma)
        .clamp(0.0, 1.0)
        * mask;

    let mut encoded = Vec::with_capacity(variables * lags);
    for cause in 0..variables {
        for lag in 0..lags {
            let values = input
                .clone()
                .slice([0..batch, cause..cause + 1, lag..lag + 1])
                .reshape([batch, 1]);
            let basis = Tensor::cat(
                vec![
                    values.clone(),
                    values.clone().powf_scalar(2.0),
                    values.abs(),
                ],
                1,
            );
            let weight = model
                .encoder_weight
                .val()
                .slice([cause..cause + 1, lag..lag + 1, 0..3 * hidden])
                .reshape([3, hidden]);
            let bias = model
                .encoder_bias
                .val()
                .slice([cause..cause + 1, lag..lag + 1, 0..hidden])
                .reshape([1, hidden]);
            encoded.push(basis.matmul(weight) + bias);
        }
    }

    let mut outputs = Vec::with_capacity(variables);
    for effect in 0..variables {
        let mut aggregate = Tensor::<B, 2>::zeros([batch, hidden], &input.device());
        for cause in 0..variables {
            for lag in 0..lags {
                let gate = gates
                    .clone()
                    .slice([cause..cause + 1, lag..lag + 1, effect..effect + 1])
                    .reshape([1, 1]);
                aggregate = aggregate + encoded[cause * lags + lag].clone() * gate;
            }
        }
        outputs.push(
            model.decoders[effect]
                .forward(aggregate)
                .unsqueeze_dim::<3>(1),
        );
    }
    let output = Tensor::cat(outputs, 1);
    let mean = output
        .clone()
        .slice([0..batch, 0..variables, 0..1])
        .squeeze::<2>();
    let sigma = softplus(
        output.slice([0..batch, 0..variables, 1..2]).squeeze::<2>(),
        1.0,
    ) + EPS;
    (mean, sigma)
}

fn loss<B: Backend>(
    model: &GraceModel<B>,
    mean: Tensor<B, 2>,
    sigma: Tensor<B, 2>,
    input: Tensor<B, 3>,
    mask: Tensor<B, 3>,
    configuration: &GraceConfiguration,
) -> (Tensor<B, 1>, f64) {
    let [batch, variables, lags] = input.shape().dims();
    let target = input
        .slice([0..batch, 0..variables, lags - 1..lags])
        .squeeze::<2>();
    let squared = (target - mean).powf_scalar(2.0);
    let nll = (squared.clone() / (sigma.clone().powf_scalar(2.0) * 2.0)
        + sigma.log()
        + 0.5 * (2.0 * std::f64::consts::PI).ln())
    .mean();
    let rmse = tensor_values(squared.mean_dim(1).sqrt().mean())[0] as f64;
    let offset = configuration.temperature
        * (-configuration.stretch_gamma / configuration.stretch_zeta).ln();
    let l0 = (sigmoid(model.log_alpha.val() - offset) * mask).sum() * configuration.lambda_l0;
    (nll + l0, rmse)
}

fn deterministic_gates<B: Backend>(
    model: &GraceModel<B>,
    mask: Tensor<B, 3>,
    variables: usize,
    lags: usize,
    configuration: &GraceConfiguration,
) -> Vec<f64> {
    let internal = tensor_values(
        (sigmoid(model.log_alpha.val())
            * (configuration.stretch_zeta - configuration.stretch_gamma)
            + configuration.stretch_gamma)
            .clamp(0.0, 1.0)
            * mask,
    );
    let mut public = vec![0.0; internal.len()];
    for cause in 0..variables {
        for effect in 0..variables {
            for public_lag in 0..lags {
                let internal_lag = lags - 1 - public_lag;
                public[(cause * variables + effect) * lags + public_lag] =
                    internal[(cause * lags + internal_lag) * variables + effect] as f64;
            }
        }
    }
    public
}

fn validate(
    windows: &GraceDenseWindows,
    skeleton: &[u8],
    configuration: &GraceConfiguration,
) -> Result<(), GraceError> {
    let variables = windows.variables;
    let lags = configuration.max_lag + 1;
    if windows.max_lag != configuration.max_lag
        || windows.windows.is_empty()
        || skeleton.len() != variables * variables * lags
    {
        return Err(GraceError::Shape);
    }
    if configuration.hidden_size == 0
        || configuration.decoder_hidden == 0
        || configuration.decoder_layers == 0
        || configuration.epochs == 0
        || configuration.patience == 0
        || !(0.0..=1.0).contains(&configuration.gate_threshold)
        || configuration.lambda_l0 < 0.0
        || configuration.temperature <= 0.0
        || configuration.stretch_gamma >= 0.0
        || configuration.stretch_zeta <= 1.0
        || configuration.learning_rate <= 0.0
        || configuration.weight_decay < 0.0
    {
        return Err(GraceError::InvalidConfiguration);
    }
    Ok(())
}

fn fit_inner<F>(
    windows: &GraceDenseWindows,
    skeleton: &[u8],
    configuration: &GraceConfiguration,
    supplied: Option<&GraceParameters>,
    supplied_uniforms: Option<&[f32]>,
    mut progress: F,
) -> Result<GraceResult, GraceError>
where
    F: FnMut(GraceProgress),
{
    validate(windows, skeleton, configuration)?;
    let variables = windows.variables;
    let lags = configuration.max_lag + 1;
    let observations = windows.windows.len();
    let gate_count = variables * lags * variables;
    if supplied_uniforms.is_some_and(|values| values.len() != configuration.epochs * gate_count) {
        return Err(GraceError::UniformShape);
    }
    let parameters = supplied
        .cloned()
        .unwrap_or_else(|| initialize_parameters(variables, configuration, skeleton));
    let device = Default::default();
    let mut model = model::<TrainingBackend>(variables, configuration, &parameters, &device)?;
    let input_values = windows
        .windows
        .iter()
        .flat_map(|row| row.iter().flat_map(|variable| variable.iter().copied()))
        .collect::<Vec<_>>();
    let input = Tensor::<TrainingBackend, 3>::from_data(
        TensorData::new(input_values, [observations, variables, lags]),
        &device,
    );
    let mask_data = mask_values(
        variables,
        lags,
        skeleton,
        configuration.include_instantaneous,
    );
    let mask = Tensor::<TrainingBackend, 3>::from_data(
        TensorData::new(mask_data, [variables, lags, variables]),
        &device,
    );
    let mut rng = NpRng::seeded(configuration.seed.wrapping_add(1));
    let mut optimizer = AdamConfig::new()
        .with_epsilon(1e-8)
        .with_weight_decay(Some(WeightDecayConfig::new(
            configuration.weight_decay as f32,
        )))
        .init();
    let mut trajectory = Vec::with_capacity(configuration.epochs);
    let mut best_loss = f64::INFINITY;
    let mut stale = 0usize;

    for epoch in 0..configuration.epochs {
        let uniform_values = supplied_uniforms
            .map(|values| values[epoch * gate_count..(epoch + 1) * gate_count].to_vec())
            .unwrap_or_else(|| {
                (0..gate_count)
                    .map(|_| rng.next_f64().clamp(EPS as f64, 1.0 - EPS as f64) as f32)
                    .collect()
            });
        let uniforms = Tensor::<TrainingBackend, 3>::from_data(
            TensorData::new(uniform_values, [variables, lags, variables]),
            &device,
        );
        let (mean, sigma) = forward(&model, input.clone(), uniforms, mask.clone(), configuration);
        let (objective, rmse) = loss(
            &model,
            mean,
            sigma,
            input.clone(),
            mask.clone(),
            configuration,
        );
        let objective_value = tensor_values(objective.clone())[0] as f64;
        let gradients = GradientsParams::from_grads(objective.backward(), &model);
        model = optimizer.step(configuration.learning_rate, model, gradients);
        let gates = deterministic_gates(&model, mask.clone(), variables, lags, configuration);
        let active_edges = gates
            .iter()
            .filter(|value| **value >= configuration.gate_threshold)
            .count();
        trajectory.push(GraceStep {
            epoch: epoch + 1,
            loss: objective_value,
            rmse,
            gate_values: gates,
            log_alpha: tensor_values(model.log_alpha.val()),
        });
        progress(GraceProgress {
            epoch: epoch + 1,
            maximum_epochs: configuration.epochs,
            loss: objective_value,
            rmse,
            active_edges,
        });
        if objective_value < best_loss {
            best_loss = objective_value;
            stale = 0;
        } else {
            stale += 1;
            if stale >= configuration.patience {
                break;
            }
        }
    }

    let gate_values = trajectory
        .last()
        .map(|step| step.gate_values.clone())
        .unwrap_or_else(|| vec![0.0; gate_count]);
    let time_graph = gate_values
        .iter()
        .map(|value| (*value >= configuration.gate_threshold) as u8)
        .collect();
    let epochs = trajectory.len();
    Ok(GraceResult {
        variables,
        max_lag: configuration.max_lag,
        skeleton: skeleton.to_vec(),
        gate_values,
        time_graph,
        trajectory,
        parameters: extract_parameters(&model, variables, configuration),
        epochs,
    })
}

pub fn fit_grace(
    windows: &GraceDenseWindows,
    skeleton: &[u8],
    configuration: &GraceConfiguration,
) -> Result<GraceResult, GraceError> {
    fit_inner(windows, skeleton, configuration, None, None, |_| {})
}

pub fn fit_grace_with_progress<F>(
    windows: &GraceDenseWindows,
    skeleton: &[u8],
    configuration: &GraceConfiguration,
    progress: F,
) -> Result<GraceResult, GraceError>
where
    F: FnMut(GraceProgress),
{
    fit_inner(windows, skeleton, configuration, None, None, progress)
}

/// Parity entry point: import source parameters and hard-concrete uniform draws,
/// then compare the complete optimizer trajectory without relying on backend RNG identity.
pub fn fit_grace_from_parameters_and_uniforms(
    windows: &GraceDenseWindows,
    skeleton: &[u8],
    configuration: &GraceConfiguration,
    parameters: &GraceParameters,
    uniforms: &[f32],
) -> Result<GraceResult, GraceError> {
    fit_inner(
        windows,
        skeleton,
        configuration,
        Some(parameters),
        Some(uniforms),
        |_| {},
    )
}
