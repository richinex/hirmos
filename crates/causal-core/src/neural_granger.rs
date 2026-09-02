//! Neural Granger-causality models ported from Neural-GC.
//!
//! Neural-GC fits one forecasting network per output series and applies a
//! proximal sparsity update to the input weights. Zeroed input groups encode
//! Granger non-causality. The public inputs are `f64` because that is the
//! surrounding crate's data boundary; they are converted to `f32` before any
//! model operation, matching the reference wrappers.

use burn::backend::{Autodiff, NdArray};
use burn::module::{Module, Param};
use burn::nn::conv::{Conv1d, Conv1dConfig};
use burn::optim::{GradientsParams, Optimizer, SgdConfig};
use burn::tensor::activation::{leaky_relu, relu, sigmoid};
use burn::tensor::backend::{AutodiffBackend, Backend};
use burn::tensor::{Tensor, TensorData};
use std::fmt::{Display, Formatter};

use crate::nprandom::NpRng;

type InnerBackend = NdArray<f32>;
type TrainingBackend = Autodiff<InnerBackend>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activation {
    Sigmoid,
    Tanh,
    Relu,
    LeakyRelu,
    Identity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmlpPenalty {
    GroupLasso,
    GroupSparseGroupLasso,
    Hierarchical,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CmlpConfig {
    pub lag: usize,
    pub hidden: Vec<usize>,
    pub activation: Activation,
    pub penalty: CmlpPenalty,
    pub lambda: f64,
    pub ridge_lambda: f64,
    pub learning_rate: f64,
    pub max_iter: usize,
    pub check_every: usize,
    pub lookback: usize,
    pub seed: u64,
}

impl Default for CmlpConfig {
    fn default() -> Self {
        Self {
            lag: 3,
            hidden: vec![100],
            activation: Activation::Relu,
            penalty: CmlpPenalty::Hierarchical,
            lambda: 0.005,
            ridge_lambda: 0.01,
            learning_rate: 0.01,
            max_iter: 50_000,
            check_every: 100,
            lookback: 5,
            seed: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conv1dParameters {
    pub out_channels: usize,
    pub in_channels: usize,
    pub kernel_size: usize,
    pub weight: Vec<f32>,
    pub bias: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CmlpParameters {
    /// Networks are ordered by predicted/output series.
    pub networks: Vec<Vec<Conv1dParameters>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NeuralGrangerProgress {
    pub iteration: usize,
    pub max_iterations: usize,
    pub loss: f64,
    pub active_fraction: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CmlpResult {
    pub series: usize,
    pub lag: usize,
    /// `[effect, cause]`, matching `cMLP.GC(ignore_lag=True)`.
    pub summary_scores: Vec<f64>,
    /// Source `GC(threshold=true)`, in `[effect, cause]` order.
    pub summary_active: Vec<bool>,
    /// `[effect, cause, lag_index]`, matching `cMLP.GC(ignore_lag=False)`.
    /// Neural-GC index zero is the oldest lag in the input window.
    pub lag_scores: Vec<f64>,
    /// Source zero/nonzero graph by lag, in `[effect, cause, lag_index]` order.
    pub lag_active: Vec<bool>,
    /// Maps each lag-score axis position to the corresponding causal lag.
    pub lag_order: Vec<usize>,
    pub loss: Vec<f64>,
    pub iterations: usize,
    pub parameters: CmlpParameters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeuralGrangerError {
    EmptyData,
    InvalidShape,
    NonFiniteData,
    InvalidLag,
    EmptyHiddenLayers,
    InvalidHiddenWidth,
    InvalidTrainingConfiguration,
    NonFiniteTraining,
    ParameterShape,
}

impl Display for NeuralGrangerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyData => "neural Granger input must not be empty",
            Self::InvalidShape => "neural Granger values must match rows times columns",
            Self::NonFiniteData => "neural Granger input must contain only finite values",
            Self::InvalidLag => "cMLP lag must be positive and smaller than the row count",
            Self::EmptyHiddenLayers => "cMLP requires at least one hidden layer",
            Self::InvalidHiddenWidth => "neural Granger hidden widths must be positive",
            Self::InvalidTrainingConfiguration => {
                "neural Granger training counts and numeric controls must be positive and finite"
            }
            Self::NonFiniteTraining => "neural Granger training produced a non-finite loss",
            Self::ParameterShape => "neural Granger parameters do not match the model shape",
        };
        f.write_str(message)
    }
}

impl std::error::Error for NeuralGrangerError {}

#[derive(Module, Debug)]
struct MlpNetwork<B: Backend> {
    layers: Vec<Conv1d<B>>,
    activation: Activation,
}

impl<B: Backend> MlpNetwork<B> {
    fn forward(&self, mut input: Tensor<B, 3>) -> Tensor<B, 3> {
        for (index, layer) in self.layers.iter().enumerate() {
            if index > 0 {
                input = activate(input, self.activation);
            }
            input = layer.forward(input);
        }
        input
    }
}

#[derive(Module, Debug)]
struct CmlpModel<B: Backend> {
    networks: Vec<MlpNetwork<B>>,
}

fn activate<B: Backend>(input: Tensor<B, 3>, activation: Activation) -> Tensor<B, 3> {
    match activation {
        Activation::Sigmoid => sigmoid(input),
        Activation::Tanh => input.tanh(),
        Activation::Relu => relu(input),
        Activation::LeakyRelu => leaky_relu(input, 0.01),
        Activation::Identity => input,
    }
}

fn tensor_values<B: Backend, const D: usize>(tensor: Tensor<B, D>) -> Vec<f32> {
    tensor.into_data().to_vec::<f32>().expect("f32 backend")
}

fn parameter_tensor<B: Backend, const D: usize>(
    values: &[f32],
    shape: [usize; D],
    device: &B::Device,
) -> Param<Tensor<B, D>> {
    Param::from_data(TensorData::new(values.to_vec(), shape), device)
}

fn uniform_values(rng: &mut NpRng, count: usize, bound: f64) -> Vec<f32> {
    (0..count)
        .map(|_| {
            let unit = (rng.next_u64() >> 11) as f64 * (1.0 / ((1u64 << 53) as f64));
            ((2.0 * unit - 1.0) * bound) as f32
        })
        .collect()
}

fn initialized_parameter<B: Backend, const D: usize>(
    supplied: Option<&[f32]>,
    shape: [usize; D],
    bound: f64,
    rng: &mut NpRng,
    device: &B::Device,
) -> Param<Tensor<B, D>> {
    if let Some(values) = supplied {
        parameter_tensor(values, shape, device)
    } else {
        parameter_tensor(
            &uniform_values(rng, shape.iter().product(), bound),
            shape,
            device,
        )
    }
}

fn validate_matrix(values: &[f64], rows: usize, columns: usize) -> Result<(), NeuralGrangerError> {
    if rows == 0 || columns == 0 {
        return Err(NeuralGrangerError::EmptyData);
    }
    if values.len() != rows * columns {
        return Err(NeuralGrangerError::InvalidShape);
    }
    if values.iter().any(|value| !value.is_finite()) {
        return Err(NeuralGrangerError::NonFiniteData);
    }
    Ok(())
}

fn validate_cmlp(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &CmlpConfig,
) -> Result<(), NeuralGrangerError> {
    validate_matrix(values, rows, columns)?;
    if config.lag == 0 || config.lag >= rows {
        return Err(NeuralGrangerError::InvalidLag);
    }
    if config.hidden.is_empty() {
        return Err(NeuralGrangerError::EmptyHiddenLayers);
    }
    if config.hidden.contains(&0) {
        return Err(NeuralGrangerError::InvalidHiddenWidth);
    }
    if config.max_iter == 0
        || config.check_every == 0
        || config.lookback == 0
        || !config.lambda.is_finite()
        || config.lambda < 0.0
        || !config.ridge_lambda.is_finite()
        || config.ridge_lambda < 0.0
        || !config.learning_rate.is_finite()
        || config.learning_rate <= 0.0
    {
        return Err(NeuralGrangerError::InvalidTrainingConfiguration);
    }
    Ok(())
}

fn cmlp_model<B: Backend>(
    series: usize,
    config: &CmlpConfig,
    parameters: Option<&CmlpParameters>,
    seed: u64,
    device: &B::Device,
) -> Result<CmlpModel<B>, NeuralGrangerError> {
    if let Some(parameters) = parameters {
        if parameters.networks.len() != series {
            return Err(NeuralGrangerError::ParameterShape);
        }
    }

    let widths = config
        .hidden
        .iter()
        .copied()
        .chain(std::iter::once(1))
        .collect::<Vec<_>>();
    let mut networks = Vec::with_capacity(series);
    let mut rng = NpRng::seeded(seed);

    for output in 0..series {
        let mut layers = Vec::with_capacity(widths.len());
        let mut in_channels = series;
        for (index, out_channels) in widths.iter().copied().enumerate() {
            let kernel_size = if index == 0 { config.lag } else { 1 };
            let mut layer = Conv1dConfig::new(in_channels, out_channels, kernel_size).init(device);

            if let Some(parameters) = parameters {
                let network = &parameters.networks[output];
                if network.len() != widths.len() {
                    return Err(NeuralGrangerError::ParameterShape);
                }
                let supplied = &network[index];
                if supplied.out_channels != out_channels
                    || supplied.in_channels != in_channels
                    || supplied.kernel_size != kernel_size
                    || supplied.weight.len() != out_channels * in_channels * kernel_size
                    || supplied.bias.len() != out_channels
                {
                    return Err(NeuralGrangerError::ParameterShape);
                }
                layer.weight = parameter_tensor(
                    &supplied.weight,
                    [out_channels, in_channels, kernel_size],
                    device,
                );
                layer.bias = Some(parameter_tensor(&supplied.bias, [out_channels], device));
            } else {
                let bound = 1.0 / ((in_channels * kernel_size) as f64).sqrt();
                layer.weight = parameter_tensor(
                    &uniform_values(&mut rng, out_channels * in_channels * kernel_size, bound),
                    [out_channels, in_channels, kernel_size],
                    device,
                );
                layer.bias = Some(parameter_tensor(
                    &uniform_values(&mut rng, out_channels, bound),
                    [out_channels],
                    device,
                ));
            }

            layers.push(layer);
            in_channels = out_channels;
        }
        networks.push(MlpNetwork {
            layers,
            activation: config.activation,
        });
    }

    Ok(CmlpModel { networks })
}

fn cmlp_parameters<B: Backend>(model: &CmlpModel<B>) -> CmlpParameters {
    let networks = model
        .networks
        .iter()
        .map(|network| {
            network
                .layers
                .iter()
                .map(|layer| {
                    let [out_channels, in_channels, kernel_size] = layer.weight.shape().dims();
                    Conv1dParameters {
                        out_channels,
                        in_channels,
                        kernel_size,
                        weight: tensor_values(layer.weight.val()),
                        bias: tensor_values(layer.bias.as_ref().expect("configured bias").val()),
                    }
                })
                .collect()
        })
        .collect();
    CmlpParameters { networks }
}

fn cmlp_tensors<B: Backend>(
    values: &[f64],
    rows: usize,
    columns: usize,
    lag: usize,
    device: &B::Device,
) -> (Tensor<B, 3>, Vec<Tensor<B, 3>>) {
    let input_values = values[..(rows - 1) * columns]
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>();
    let input = Tensor::from_data(
        TensorData::new(input_values, [1, rows - 1, columns]),
        device,
    )
    .swap_dims(1, 2);

    let targets = (0..columns)
        .map(|column| {
            let target = (lag..rows)
                .map(|row| values[row * columns + column] as f32)
                .collect::<Vec<_>>();
            Tensor::from_data(TensorData::new(target, [1, 1, rows - lag]), device)
        })
        .collect();
    (input, targets)
}

fn cmlp_smooth_loss<B: AutodiffBackend>(
    model: &CmlpModel<B>,
    input: &Tensor<B, 3>,
    targets: &[Tensor<B, 3>],
    ridge_lambda: f64,
) -> Tensor<B, 1> {
    let mut terms = Vec::with_capacity(model.networks.len());
    for (network, target) in model.networks.iter().zip(targets) {
        let residual = network.forward(input.clone()) - target.clone();
        let mse = residual.powf_scalar(2.0).mean();
        let ridge = network
            .layers
            .iter()
            .skip(1)
            .fold(Tensor::<B, 1>::zeros([1], &input.device()), |sum, layer| {
                sum + layer.weight.val().powf_scalar(2.0).sum()
            });
        terms.push(mse + ridge * ridge_lambda);
    }
    terms
        .into_iter()
        .reduce(|left, right| left + right)
        .unwrap()
}

fn cmlp_regularization(parameters: &CmlpParameters, lambda: f32, penalty: CmlpPenalty) -> f32 {
    parameters
        .networks
        .iter()
        .map(|network| {
            let layer = &network[0];
            let h = layer.out_channels;
            let p = layer.in_channels;
            let lag = layer.kernel_size;
            let index = |hidden: usize, input: usize, lag_index: usize| {
                (hidden * p + input) * lag + lag_index
            };
            let mut penalty_sum = 0.0f32;
            match penalty {
                CmlpPenalty::GroupLasso => {
                    for input in 0..p {
                        let mut squared = 0.0;
                        for hidden in 0..h {
                            for lag_index in 0..lag {
                                squared += layer.weight[index(hidden, input, lag_index)].powi(2);
                            }
                        }
                        penalty_sum += squared.sqrt();
                    }
                }
                CmlpPenalty::GroupSparseGroupLasso => {
                    for input in 0..p {
                        let mut overall = 0.0;
                        for lag_index in 0..lag {
                            let mut squared = 0.0;
                            for hidden in 0..h {
                                let value = layer.weight[index(hidden, input, lag_index)];
                                squared += value * value;
                                overall += value * value;
                            }
                            penalty_sum += squared.sqrt();
                        }
                        penalty_sum += overall.sqrt();
                    }
                }
                CmlpPenalty::Hierarchical => {
                    for prefix in 1..=lag {
                        for input in 0..p {
                            let mut squared = 0.0;
                            for hidden in 0..h {
                                for lag_index in 0..prefix {
                                    squared +=
                                        layer.weight[index(hidden, input, lag_index)].powi(2);
                                }
                            }
                            penalty_sum += squared.sqrt();
                        }
                    }
                }
            }
            lambda * penalty_sum
        })
        .sum()
}

fn cmlp_prox(
    parameters: &mut CmlpParameters,
    lambda: f32,
    learning_rate: f32,
    penalty: CmlpPenalty,
) {
    let threshold = lambda * learning_rate;
    if threshold == 0.0 {
        return;
    }
    for network in &mut parameters.networks {
        let layer = &mut network[0];
        let h = layer.out_channels;
        let p = layer.in_channels;
        let lag = layer.kernel_size;
        let index =
            |hidden: usize, input: usize, lag_index: usize| (hidden * p + input) * lag + lag_index;

        let shrink = |weight: &mut [f32], indices: &[usize]| {
            let norm = indices
                .iter()
                .map(|position| weight[*position] * weight[*position])
                .sum::<f32>()
                .sqrt();
            let scale = (norm - threshold).max(0.0) / norm.max(threshold);
            for position in indices {
                weight[*position] *= scale;
            }
        };

        match penalty {
            CmlpPenalty::GroupLasso => {
                for input in 0..p {
                    let mut positions = Vec::with_capacity(h * lag);
                    for hidden in 0..h {
                        for lag_index in 0..lag {
                            positions.push(index(hidden, input, lag_index));
                        }
                    }
                    shrink(&mut layer.weight, &positions);
                }
            }
            CmlpPenalty::GroupSparseGroupLasso => {
                for input in 0..p {
                    for lag_index in 0..lag {
                        let positions = (0..h)
                            .map(|hidden| index(hidden, input, lag_index))
                            .collect::<Vec<_>>();
                        shrink(&mut layer.weight, &positions);
                    }
                }
                for input in 0..p {
                    let mut positions = Vec::with_capacity(h * lag);
                    for hidden in 0..h {
                        for lag_index in 0..lag {
                            positions.push(index(hidden, input, lag_index));
                        }
                    }
                    shrink(&mut layer.weight, &positions);
                }
            }
            CmlpPenalty::Hierarchical => {
                for prefix in 1..=lag {
                    for input in 0..p {
                        let mut positions = Vec::with_capacity(h * prefix);
                        for hidden in 0..h {
                            for lag_index in 0..prefix {
                                positions.push(index(hidden, input, lag_index));
                            }
                        }
                        shrink(&mut layer.weight, &positions);
                    }
                }
            }
        }
    }
}

fn cmlp_prox_model<B: Backend>(
    model: &mut CmlpModel<B>,
    lambda: f64,
    learning_rate: f64,
    penalty: CmlpPenalty,
) {
    for network in &mut model.networks {
        let layer = &mut network.layers[0];
        let [out_channels, in_channels, kernel_size] = layer.weight.shape().dims();
        let mut parameters = CmlpParameters {
            networks: vec![vec![Conv1dParameters {
                out_channels,
                in_channels,
                kernel_size,
                weight: tensor_values(layer.weight.val()),
                bias: Vec::new(),
            }]],
        };
        cmlp_prox(
            &mut parameters,
            lambda as f32,
            learning_rate as f32,
            penalty,
        );
        let device = layer.weight.device();
        let weight = Tensor::from_data(
            TensorData::new(
                parameters.networks.pop().unwrap().pop().unwrap().weight,
                [out_channels, in_channels, kernel_size],
            ),
            &device,
        );
        layer.weight = layer.weight.clone().map(|_| weight.detach().require_grad());
    }
}

pub fn cmlp_proximal_update(
    parameters: &mut CmlpParameters,
    lambda: f64,
    learning_rate: f64,
    penalty: CmlpPenalty,
) -> Result<(), NeuralGrangerError> {
    if !lambda.is_finite()
        || lambda < 0.0
        || !learning_rate.is_finite()
        || learning_rate <= 0.0
        || parameters.networks.iter().any(|network| network.is_empty())
    {
        return Err(NeuralGrangerError::InvalidTrainingConfiguration);
    }
    cmlp_prox(parameters, lambda as f32, learning_rate as f32, penalty);
    Ok(())
}

fn cmlp_scores(parameters: &CmlpParameters) -> (Vec<f64>, Vec<f64>) {
    let mut summary = Vec::with_capacity(parameters.networks.len().pow(2));
    let mut lagged = Vec::new();
    for network in &parameters.networks {
        let layer = &network[0];
        for input in 0..layer.in_channels {
            let mut overall = 0.0f32;
            for lag_index in 0..layer.kernel_size {
                let mut squared = 0.0f32;
                for hidden in 0..layer.out_channels {
                    let position =
                        (hidden * layer.in_channels + input) * layer.kernel_size + lag_index;
                    let value = layer.weight[position];
                    squared += value * value;
                    overall += value * value;
                }
                lagged.push(squared.sqrt() as f64);
            }
            summary.push(overall.sqrt() as f64);
        }
    }
    (summary, lagged)
}

pub fn cmlp_initial_parameters(
    series: usize,
    config: &CmlpConfig,
) -> Result<CmlpParameters, NeuralGrangerError> {
    if series == 0 || config.hidden.is_empty() || config.hidden.contains(&0) || config.lag == 0 {
        return Err(NeuralGrangerError::InvalidTrainingConfiguration);
    }
    let device = Default::default();
    <TrainingBackend as Backend>::seed(&device, config.seed);
    let model = cmlp_model::<TrainingBackend>(series, config, None, config.seed, &device)?;
    Ok(cmlp_parameters(&model))
}

pub fn cmlp_predict(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &CmlpConfig,
    parameters: &CmlpParameters,
) -> Result<Vec<f64>, NeuralGrangerError> {
    validate_cmlp(values, rows, columns, config)?;
    let device = Default::default();
    let model =
        cmlp_model::<TrainingBackend>(columns, config, Some(parameters), config.seed, &device)?;
    let (input, _) = cmlp_tensors(values, rows, columns, config.lag, &device);
    let outputs = model
        .networks
        .iter()
        .map(|network| network.forward(input.clone()))
        .collect::<Vec<_>>();
    let output = Tensor::cat(outputs, 1).swap_dims(1, 2);
    Ok(tensor_values(output)
        .into_iter()
        .map(|value| value as f64)
        .collect())
}

pub fn fit_cmlp(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &CmlpConfig,
) -> Result<CmlpResult, NeuralGrangerError> {
    fit_cmlp_with(values, rows, columns, config, None, |_| {})
}

pub fn fit_cmlp_from_parameters(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &CmlpConfig,
    parameters: &CmlpParameters,
) -> Result<CmlpResult, NeuralGrangerError> {
    fit_cmlp_with(values, rows, columns, config, Some(parameters), |_| {})
}

pub fn fit_cmlp_with<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &CmlpConfig,
    parameters: Option<&CmlpParameters>,
    mut progress: F,
) -> Result<CmlpResult, NeuralGrangerError>
where
    F: FnMut(NeuralGrangerProgress),
{
    validate_cmlp(values, rows, columns, config)?;
    let device = Default::default();
    <TrainingBackend as Backend>::seed(&device, config.seed);
    let mut model =
        cmlp_model::<TrainingBackend>(columns, config, parameters, config.seed, &device)?;
    let (input, targets) = cmlp_tensors(values, rows, columns, config.lag, &device);
    let mut optimizer = SgdConfig::new().init();
    let mut smooth = cmlp_smooth_loss(&model, &input, &targets, config.ridge_lambda);
    let mut losses = Vec::new();
    let mut best = None;
    let mut best_loss = f64::INFINITY;
    let mut best_iteration = 0usize;
    let mut completed = 0usize;

    for iteration in 0..config.max_iter {
        let gradients = GradientsParams::from_grads(smooth.backward(), &model);
        model = optimizer.step(config.learning_rate, model, gradients);
        cmlp_prox_model(
            &mut model,
            config.lambda,
            config.learning_rate,
            config.penalty,
        );

        smooth = cmlp_smooth_loss(&model, &input, &targets, config.ridge_lambda);
        completed = iteration + 1;

        if completed % config.check_every == 0 {
            let smooth_value = tensor_values(smooth.clone())[0];
            let parameters = cmlp_parameters(&model);
            let nonsmooth = cmlp_regularization(&parameters, config.lambda as f32, config.penalty);
            let loss = ((smooth_value + nonsmooth) / columns as f32) as f64;
            if !loss.is_finite() {
                return Err(NeuralGrangerError::NonFiniteTraining);
            }
            losses.push(loss);
            let (scores, _) = cmlp_scores(&parameters);
            let active = scores.iter().filter(|score| **score > 0.0).count();
            progress(NeuralGrangerProgress {
                iteration: completed,
                max_iterations: config.max_iter,
                loss,
                active_fraction: active as f64 / scores.len() as f64,
            });

            if loss < best_loss {
                best_loss = loss;
                best_iteration = iteration;
                best = Some(model.clone());
            } else if iteration - best_iteration == config.lookback * config.check_every {
                break;
            }
        }
    }

    if let Some(best) = best {
        model = best;
    }
    let parameters = cmlp_parameters(&model);
    let (summary_scores, lag_scores) = cmlp_scores(&parameters);
    let summary_active = summary_scores.iter().map(|score| *score > 0.0).collect();
    let lag_active = lag_scores.iter().map(|score| *score > 0.0).collect();
    Ok(CmlpResult {
        series: columns,
        lag: config.lag,
        summary_scores,
        summary_active,
        lag_scores,
        lag_active,
        lag_order: (1..=config.lag).rev().collect(),
        loss: losses,
        iterations: completed,
        parameters,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClstmConfig {
    pub context: usize,
    pub hidden: usize,
    pub lambda: f64,
    pub ridge_lambda: f64,
    pub learning_rate: f64,
    pub max_iter: usize,
    pub check_every: usize,
    pub lookback: usize,
    pub seed: u64,
}

impl Default for ClstmConfig {
    fn default() -> Self {
        Self {
            context: 10,
            hidden: 100,
            lambda: 0.005,
            ridge_lambda: 0.01,
            learning_rate: 0.01,
            max_iter: 20_000,
            check_every: 50,
            lookback: 5,
            seed: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LstmNetworkParameters {
    /// PyTorch layout `[input, forget, cell, output]`, shape `[4h, p]`.
    pub weight_ih: Vec<f32>,
    /// PyTorch layout `[input, forget, cell, output]`, shape `[4h, h]`.
    pub weight_hh: Vec<f32>,
    pub bias_ih: Vec<f32>,
    pub bias_hh: Vec<f32>,
    /// Shape `[1, h, 1]`, matching the reference's one-by-one convolution.
    pub output_weight: Vec<f32>,
    pub output_bias: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClstmParameters {
    pub series: usize,
    pub hidden: usize,
    /// Networks are ordered by predicted/output series.
    pub networks: Vec<LstmNetworkParameters>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClstmResult {
    pub series: usize,
    pub context: usize,
    /// `[effect, cause]`, matching `cLSTM.GC(threshold=False)`.
    pub summary_scores: Vec<f64>,
    /// Source `GC(threshold=true)`, in `[effect, cause]` order.
    pub summary_active: Vec<bool>,
    pub loss: Vec<f64>,
    pub iterations: usize,
    pub parameters: ClstmParameters,
}

#[derive(Module, Debug)]
struct LstmNetwork<B: Backend> {
    weight_ih: Param<Tensor<B, 2>>,
    weight_hh: Param<Tensor<B, 2>>,
    bias_ih: Param<Tensor<B, 1>>,
    bias_hh: Param<Tensor<B, 1>>,
    output_weight: Param<Tensor<B, 2>>,
    output_bias: Param<Tensor<B, 1>>,
    hidden: usize,
}

impl<B: Backend> LstmNetwork<B> {
    fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 3> {
        let [batch, steps, input_columns] = input.shape().dims();
        let device = input.device();
        let mut hidden_state = Tensor::<B, 2>::zeros([batch, self.hidden], &device);
        let mut cell_state = Tensor::<B, 2>::zeros([batch, self.hidden], &device);
        let mut predictions = Vec::with_capacity(steps);

        for step in 0..steps {
            let current = input
                .clone()
                .slice([0..batch, step..step + 1, 0..input_columns])
                .reshape([batch, input_columns]);
            let gates = current.matmul(self.weight_ih.val().transpose())
                + hidden_state
                    .clone()
                    .matmul(self.weight_hh.val().transpose())
                + self.bias_ih.val().unsqueeze_dim::<2>(0)
                + self.bias_hh.val().unsqueeze_dim::<2>(0);
            let input_gate = sigmoid(gates.clone().slice([0..batch, 0..self.hidden]));
            let forget_gate = sigmoid(
                gates
                    .clone()
                    .slice([0..batch, self.hidden..2 * self.hidden]),
            );
            let cell_gate = gates
                .clone()
                .slice([0..batch, 2 * self.hidden..3 * self.hidden])
                .tanh();
            let output_gate = sigmoid(gates.slice([0..batch, 3 * self.hidden..4 * self.hidden]));

            cell_state = forget_gate * cell_state + input_gate * cell_gate;
            hidden_state = output_gate * cell_state.clone().tanh();
            let prediction = hidden_state
                .clone()
                .matmul(self.output_weight.val().transpose())
                + self.output_bias.val().unsqueeze_dim::<2>(0);
            predictions.push(prediction.unsqueeze_dim::<3>(1));
        }

        Tensor::cat(predictions, 1)
    }
}

#[derive(Module, Debug)]
struct ClstmModel<B: Backend> {
    networks: Vec<LstmNetwork<B>>,
}

fn validate_clstm(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &ClstmConfig,
) -> Result<(), NeuralGrangerError> {
    validate_matrix(values, rows, columns)?;
    if config.context == 0 || config.context >= rows {
        return Err(NeuralGrangerError::InvalidLag);
    }
    if config.hidden == 0 {
        return Err(NeuralGrangerError::InvalidHiddenWidth);
    }
    if config.max_iter == 0
        || config.check_every == 0
        || config.lookback == 0
        || !config.lambda.is_finite()
        || config.lambda < 0.0
        || !config.ridge_lambda.is_finite()
        || config.ridge_lambda < 0.0
        || !config.learning_rate.is_finite()
        || config.learning_rate <= 0.0
    {
        return Err(NeuralGrangerError::InvalidTrainingConfiguration);
    }
    Ok(())
}

fn validate_clstm_parameters(
    parameters: &ClstmParameters,
    series: usize,
    hidden: usize,
) -> Result<(), NeuralGrangerError> {
    if parameters.series != series
        || parameters.hidden != hidden
        || parameters.networks.len() != series
        || parameters.networks.iter().any(|network| {
            network.weight_ih.len() != 4 * hidden * series
                || network.weight_hh.len() != 4 * hidden * hidden
                || network.bias_ih.len() != 4 * hidden
                || network.bias_hh.len() != 4 * hidden
                || network.output_weight.len() != hidden
        })
    {
        return Err(NeuralGrangerError::ParameterShape);
    }
    Ok(())
}

fn clstm_model<B: Backend>(
    series: usize,
    hidden: usize,
    parameters: Option<&ClstmParameters>,
    seed: u64,
    device: &B::Device,
) -> Result<ClstmModel<B>, NeuralGrangerError> {
    if let Some(parameters) = parameters {
        validate_clstm_parameters(parameters, series, hidden)?;
    }
    let bound = 1.0 / (hidden as f64).sqrt();
    let mut networks = Vec::with_capacity(series);
    let mut rng = NpRng::seeded(seed);
    for output in 0..series {
        let supplied = parameters.map(|parameters| &parameters.networks[output]);
        networks.push(LstmNetwork {
            weight_ih: initialized_parameter(
                supplied.map(|item| item.weight_ih.as_slice()),
                [4 * hidden, series],
                bound,
                &mut rng,
                device,
            ),
            weight_hh: initialized_parameter(
                supplied.map(|item| item.weight_hh.as_slice()),
                [4 * hidden, hidden],
                bound,
                &mut rng,
                device,
            ),
            bias_ih: initialized_parameter(
                supplied.map(|item| item.bias_ih.as_slice()),
                [4 * hidden],
                bound,
                &mut rng,
                device,
            ),
            bias_hh: initialized_parameter(
                supplied.map(|item| item.bias_hh.as_slice()),
                [4 * hidden],
                bound,
                &mut rng,
                device,
            ),
            output_weight: initialized_parameter(
                supplied.map(|item| item.output_weight.as_slice()),
                [1, hidden],
                bound,
                &mut rng,
                device,
            ),
            output_bias: if let Some(supplied) = supplied {
                parameter_tensor(&[supplied.output_bias], [1], device)
            } else {
                parameter_tensor(&uniform_values(&mut rng, 1, bound), [1], device)
            },
            hidden,
        });
    }
    Ok(ClstmModel { networks })
}

fn clstm_parameters<B: Backend>(model: &ClstmModel<B>, series: usize) -> ClstmParameters {
    let hidden = model.networks[0].hidden;
    let networks = model
        .networks
        .iter()
        .map(|network| LstmNetworkParameters {
            weight_ih: tensor_values(network.weight_ih.val()),
            weight_hh: tensor_values(network.weight_hh.val()),
            bias_ih: tensor_values(network.bias_ih.val()),
            bias_hh: tensor_values(network.bias_hh.val()),
            output_weight: tensor_values(network.output_weight.val()),
            output_bias: tensor_values(network.output_bias.val())[0],
        })
        .collect();
    ClstmParameters {
        series,
        hidden,
        networks,
    }
}

fn clstm_arrange<B: Backend>(
    values: &[f64],
    rows: usize,
    columns: usize,
    context: usize,
    device: &B::Device,
) -> (Tensor<B, 3>, Vec<Tensor<B, 3>>) {
    let samples = rows - context;
    let mut input = Vec::with_capacity(samples * context * columns);
    let mut targets = vec![Vec::with_capacity(samples * context); columns];
    for sample in 0..samples {
        for offset in 0..context {
            for column in 0..columns {
                input.push(values[(sample + offset) * columns + column] as f32);
                targets[column].push(values[(sample + offset + 1) * columns + column] as f32);
            }
        }
    }
    let input = Tensor::from_data(TensorData::new(input, [samples, context, columns]), device);
    let targets = targets
        .into_iter()
        .map(|target| Tensor::from_data(TensorData::new(target, [samples, context, 1]), device))
        .collect();
    (input, targets)
}

fn clstm_smooth_loss<B: AutodiffBackend>(
    model: &ClstmModel<B>,
    input: &Tensor<B, 3>,
    targets: &[Tensor<B, 3>],
    ridge_lambda: f64,
) -> Tensor<B, 1> {
    model
        .networks
        .iter()
        .zip(targets)
        .map(|(network, target)| {
            let residual = network.forward(input.clone()) - target.clone();
            let mse = residual.powf_scalar(2.0).mean();
            let ridge = network.weight_hh.val().powf_scalar(2.0).sum()
                + network.output_weight.val().powf_scalar(2.0).sum();
            mse + ridge * ridge_lambda
        })
        .reduce(|left, right| left + right)
        .unwrap()
}

fn clstm_regularization(parameters: &ClstmParameters, lambda: f32) -> f32 {
    parameters
        .networks
        .iter()
        .map(|network| {
            (0..parameters.series)
                .map(|input| {
                    let squared = (0..4 * parameters.hidden)
                        .map(|row| {
                            let value = network.weight_ih[row * parameters.series + input];
                            value * value
                        })
                        .sum::<f32>();
                    squared.sqrt()
                })
                .sum::<f32>()
        })
        .sum::<f32>()
        * lambda
}

fn clstm_prox(parameters: &mut ClstmParameters, lambda: f32, learning_rate: f32) {
    let threshold = lambda * learning_rate;
    if threshold == 0.0 {
        return;
    }
    for network in &mut parameters.networks {
        for input in 0..parameters.series {
            let norm = (0..4 * parameters.hidden)
                .map(|row| {
                    let value = network.weight_ih[row * parameters.series + input];
                    value * value
                })
                .sum::<f32>()
                .sqrt();
            let scale = (norm - threshold).max(0.0) / norm.max(threshold);
            for row in 0..4 * parameters.hidden {
                network.weight_ih[row * parameters.series + input] *= scale;
            }
        }
    }
}

fn clstm_prox_model<B: Backend>(model: &mut ClstmModel<B>, lambda: f64, learning_rate: f64) {
    let threshold = lambda * learning_rate;
    if threshold == 0.0 {
        return;
    }
    for network in &mut model.networks {
        let weight = network.weight_ih.val();
        let norm = weight.clone().powf_scalar(2.0).sum_dim(0).sqrt();
        let scale = (norm.clone() - threshold).clamp_min(0.0) / norm.clamp_min(threshold);
        let updated = weight * scale;
        network.weight_ih = network
            .weight_ih
            .clone()
            .map(|_| updated.detach().require_grad());
    }
}

pub fn clstm_proximal_update(
    parameters: &mut ClstmParameters,
    lambda: f64,
    learning_rate: f64,
) -> Result<(), NeuralGrangerError> {
    validate_clstm_parameters(parameters, parameters.series, parameters.hidden)?;
    if !lambda.is_finite() || lambda < 0.0 || !learning_rate.is_finite() || learning_rate <= 0.0 {
        return Err(NeuralGrangerError::InvalidTrainingConfiguration);
    }
    clstm_prox(parameters, lambda as f32, learning_rate as f32);
    Ok(())
}

fn clstm_scores(parameters: &ClstmParameters) -> Vec<f64> {
    let mut scores = Vec::with_capacity(parameters.series * parameters.series);
    for network in &parameters.networks {
        for input in 0..parameters.series {
            let norm = (0..4 * parameters.hidden)
                .map(|row| {
                    let value = network.weight_ih[row * parameters.series + input];
                    value * value
                })
                .sum::<f32>()
                .sqrt();
            scores.push(norm as f64);
        }
    }
    scores
}

pub fn clstm_initial_parameters(
    series: usize,
    config: &ClstmConfig,
) -> Result<ClstmParameters, NeuralGrangerError> {
    if series == 0 || config.hidden == 0 {
        return Err(NeuralGrangerError::InvalidTrainingConfiguration);
    }
    let device = Default::default();
    <TrainingBackend as Backend>::seed(&device, config.seed);
    let model = clstm_model::<TrainingBackend>(series, config.hidden, None, config.seed, &device)?;
    Ok(clstm_parameters(&model, series))
}

pub fn clstm_predict(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &ClstmConfig,
    parameters: &ClstmParameters,
) -> Result<Vec<f64>, NeuralGrangerError> {
    validate_clstm(values, rows, columns, config)?;
    let device = Default::default();
    let model = clstm_model::<TrainingBackend>(
        columns,
        config.hidden,
        Some(parameters),
        config.seed,
        &device,
    )?;
    let input = Tensor::from_data(
        TensorData::new(
            values.iter().map(|value| *value as f32).collect::<Vec<_>>(),
            [1, rows, columns],
        ),
        &device,
    );
    let output = Tensor::cat(
        model
            .networks
            .iter()
            .map(|network| network.forward(input.clone()))
            .collect(),
        2,
    );
    Ok(tensor_values(output)
        .into_iter()
        .map(|value| value as f64)
        .collect())
}

pub fn fit_clstm(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &ClstmConfig,
) -> Result<ClstmResult, NeuralGrangerError> {
    fit_clstm_with(values, rows, columns, config, None, |_| {})
}

pub fn fit_clstm_from_parameters(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &ClstmConfig,
    parameters: &ClstmParameters,
) -> Result<ClstmResult, NeuralGrangerError> {
    fit_clstm_with(values, rows, columns, config, Some(parameters), |_| {})
}

pub fn fit_clstm_with<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    config: &ClstmConfig,
    parameters: Option<&ClstmParameters>,
    mut progress: F,
) -> Result<ClstmResult, NeuralGrangerError>
where
    F: FnMut(NeuralGrangerProgress),
{
    validate_clstm(values, rows, columns, config)?;
    let device = Default::default();
    <TrainingBackend as Backend>::seed(&device, config.seed);
    let mut model =
        clstm_model::<TrainingBackend>(columns, config.hidden, parameters, config.seed, &device)?;
    let (input, targets) = clstm_arrange(values, rows, columns, config.context, &device);
    let mut optimizer = SgdConfig::new().init();
    let mut smooth = clstm_smooth_loss(&model, &input, &targets, config.ridge_lambda);
    let mut losses = Vec::new();
    let mut best = None;
    let mut best_loss = f64::INFINITY;
    let mut best_iteration = 0usize;
    let mut completed = 0usize;

    for iteration in 0..config.max_iter {
        let gradients = GradientsParams::from_grads(smooth.backward(), &model);
        model = optimizer.step(config.learning_rate, model, gradients);
        clstm_prox_model(&mut model, config.lambda, config.learning_rate);
        smooth = clstm_smooth_loss(&model, &input, &targets, config.ridge_lambda);
        completed = iteration + 1;

        if completed % config.check_every == 0 {
            let smooth_value = tensor_values(smooth.clone())[0];
            let parameters = clstm_parameters(&model, columns);
            let nonsmooth = clstm_regularization(&parameters, config.lambda as f32);
            let loss = ((smooth_value + nonsmooth) / columns as f32) as f64;
            if !loss.is_finite() {
                return Err(NeuralGrangerError::NonFiniteTraining);
            }
            losses.push(loss);
            let scores = clstm_scores(&parameters);
            let active = scores.iter().filter(|score| **score > 0.0).count();
            progress(NeuralGrangerProgress {
                iteration: completed,
                max_iterations: config.max_iter,
                loss,
                active_fraction: active as f64 / scores.len() as f64,
            });

            if loss < best_loss {
                best_loss = loss;
                best_iteration = iteration;
                best = Some(model.clone());
            } else if iteration - best_iteration == config.lookback * config.check_every {
                break;
            }
        }
    }

    if let Some(best) = best {
        model = best;
    }
    let parameters = clstm_parameters(&model, columns);
    let summary_scores = clstm_scores(&parameters);
    let summary_active = summary_scores.iter().map(|score| *score > 0.0).collect();
    Ok(ClstmResult {
        series: columns,
        context: config.context,
        summary_scores,
        summary_active,
        loss: losses,
        iterations: completed,
        parameters,
    })
}
