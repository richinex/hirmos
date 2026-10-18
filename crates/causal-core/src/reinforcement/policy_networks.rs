//! Small policy, value and discriminator networks, initialised and regularised from a seeded generator.

use burn::backend::{Autodiff, NdArray};
use burn::module::{AutodiffModule, Module, ModuleVisitor, Param, ParamId};
use burn::optim::GradientsParams;
use burn::tensor::backend::AutodiffBackend;
use burn::nn::{LayerNorm, LayerNormConfig, Linear, LinearConfig};
use burn::tensor::activation::{leaky_relu, relu, sigmoid, silu};
use burn::tensor::backend::Backend;
use burn::tensor::{Tensor, TensorData};
use nalgebra::DMatrix;

use crate::nprandom::NpRng;

pub type InferenceBackend = NdArray<f32>;
pub type TrainingBackend = Autodiff<InferenceBackend>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activation {
    Relu,
    LeakyRelu,
    Tanh,
    Sigmoid,
}

impl Activation {
    fn apply<B: Backend, const D: usize>(self, x: Tensor<B, D>) -> Tensor<B, D> {
        match self {
            Self::Relu => relu(x),
            Self::LeakyRelu => leaky_relu(x, 0.01),
            Self::Tanh => x.tanh(),
            Self::Sigmoid => sigmoid(x),
        }
    }

    fn slope<B: Backend, const D: usize>(self, pre: Tensor<B, D>) -> Tensor<B, D> {
        match self {
            Self::Relu => pre.greater_elem(0.0).float(),
            Self::LeakyRelu => pre.greater_elem(0.0).float().mul_scalar(0.99).add_scalar(0.01),
            Self::Tanh => pre.tanh().powi_scalar(2).neg().add_scalar(1.0),
            Self::Sigmoid => {
                let s = sigmoid(pre);
                s.clone() * s.neg().add_scalar(1.0)
            }
        }
    }
}

/// A linear layer with PyTorch's default uniform(-1/sqrt(inputs), 1/sqrt(inputs)) weights and biases.
pub fn dense<B: Backend>(inputs: usize, outputs: usize, rng: &mut NpRng, device: &B::Device) -> Linear<B> {
    let bound = 1.0 / (inputs as f64).sqrt();
    let weight = uniform(rng, inputs * outputs, -bound, bound);
    let bias = uniform(rng, outputs, -bound, bound);
    with_values(inputs, outputs, weight, bias, device)
}

/// A linear layer with orthogonal weights scaled by `gain` and zero biases.
pub fn orthogonal_dense<B: Backend>(inputs: usize, outputs: usize, gain: f64, rng: &mut NpRng, device: &B::Device) -> Linear<B> {
    let rows = outputs.max(inputs);
    let columns = outputs.min(inputs);
    let draws = DMatrix::from_fn(rows, columns, |_, _| rng.standard_normal());
    let qr = draws.qr();
    let (q, r) = (qr.q(), qr.r());
    let signed = DMatrix::from_fn(rows, columns, |i, j| q[(i, j)] * r[(j, j)].signum());
    let weight = (0..inputs)
        .flat_map(|i| (0..outputs).map(move |o| (i, o)))
        .map(|(i, o)| {
            let value = if outputs >= inputs { signed[(o, i)] } else { signed[(i, o)] };
            (gain * value) as f32
        })
        .collect();
    with_values(inputs, outputs, weight, vec![0.0; outputs], device)
}

fn uniform(rng: &mut NpRng, count: usize, low: f64, high: f64) -> Vec<f32> {
    (0..count).map(|_| (low + (high - low) * rng.next_f64()) as f32).collect()
}

fn with_values<B: Backend>(inputs: usize, outputs: usize, weight: Vec<f32>, bias: Vec<f32>, device: &B::Device) -> Linear<B> {
    let mut layer = LinearConfig::new(inputs, outputs).init(device);
    layer.weight = Param::from_data(TensorData::new(weight, [inputs, outputs]), device);
    layer.bias = Some(Param::from_data(TensorData::new(bias, [outputs]), device));
    layer
}

fn scaled<B: Backend>(layer: Linear<B>, weight_scale: f32) -> Linear<B> {
    let device = layer.weight.device();
    let [inputs, outputs] = layer.weight.dims();
    let weight = layer.weight.val().into_data().to_vec::<f32>().expect("f32 weights").into_iter().map(|w| w * weight_scale).collect();
    with_values(inputs, outputs, weight, vec![0.0; outputs], &device)
}

/// Inverted dropout whose mask is drawn from `rng`; with no generator the input passes unchanged.
pub fn dropout<B: Backend, const D: usize>(x: Tensor<B, D>, probability: f64, rng: Option<&mut NpRng>) -> Tensor<B, D> {
    match dropout_mask(&x, probability, rng) {
        Some(mask) => x * mask,
        None => x,
    }
}

fn dropout_mask<B: Backend, const D: usize>(x: &Tensor<B, D>, probability: f64, rng: Option<&mut NpRng>) -> Option<Tensor<B, D>> {
    let rng = rng.filter(|_| probability > 0.0)?;
    let shape = x.shape();
    let keep = 1.0 - probability;
    let mask: Vec<f32> = (0..shape.num_elements()).map(|_| if rng.next_f64() < keep { (1.0 / keep) as f32 } else { 0.0 }).collect();
    Some(Tensor::from_data(TensorData::new(mask, shape), &x.device()))
}

/// Three ReLU layers mapping inputs to action scores.
#[derive(Module, Debug)]
pub struct PolicyMlp<B: Backend> {
    first: Linear<B>,
    second: Linear<B>,
    head: Linear<B>,
}

impl<B: Backend> PolicyMlp<B> {
    pub fn new(inputs: usize, outputs: usize, hidden: usize, rng: &mut NpRng, device: &B::Device) -> Self {
        Self {
            first: dense(inputs, hidden, rng, device),
            second: dense(hidden, hidden, rng, device),
            head: dense(hidden, outputs, rng, device),
        }
    }

    /// The same shape with orthogonal weights scaled by sqrt(2) and zero biases in every layer.
    pub fn orthogonal(inputs: usize, outputs: usize, hidden: usize, rng: &mut NpRng, device: &B::Device) -> Self {
        let gain = std::f64::consts::SQRT_2;
        Self {
            first: orthogonal_dense(inputs, hidden, gain, rng, device),
            second: orthogonal_dense(hidden, hidden, gain, rng, device),
            head: orthogonal_dense(hidden, outputs, gain, rng, device),
        }
    }

    pub fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 2> {
        self.head.forward(relu(self.second.forward(relu(self.first.forward(x)))))
    }
}

/// Three hidden layers whose outputs are summed before the head, which starts at a tenth of its default weights and zero bias.
#[derive(Module, Debug)]
pub struct SkipMlp<B: Backend> {
    first: Linear<B>,
    second: Linear<B>,
    third: Linear<B>,
    head: Linear<B>,
    activation: Activation,
    dropout_rate: f64,
}

impl<B: Backend> SkipMlp<B> {
    pub fn new(inputs: usize, outputs: usize, hidden: usize, activation: Activation, rng: &mut NpRng, device: &B::Device) -> Self {
        Self {
            first: dense(inputs, hidden, rng, device),
            second: dense(hidden, hidden, rng, device),
            third: dense(hidden, hidden, rng, device),
            head: scaled(dense(hidden, outputs, rng, device), 0.1),
            activation,
            dropout_rate: 0.0,
        }
    }

    /// The same network with dropout after each hidden activation while training.
    pub fn with_dropout(self, dropout_rate: f64) -> Self {
        Self { dropout_rate, ..self }
    }

    pub fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 2> {
        self.forward_training(x, None)
    }

    /// The forward pass with dropout masks drawn from `rng`.
    pub fn forward_training(&self, x: Tensor<B, 2>, mut rng: Option<&mut NpRng>) -> Tensor<B, 2> {
        let one = dropout(self.activation.apply(self.first.forward(x)), self.dropout_rate, rng.as_deref_mut());
        let two = dropout(self.activation.apply(self.second.forward(one.clone())), self.dropout_rate, rng.as_deref_mut());
        let three = dropout(self.activation.apply(self.third.forward(one.clone() + two.clone())), self.dropout_rate, rng);
        self.head.forward(one + two + three)
    }

    /// The gradient of the single output with respect to each input row, built from tensor operations so that it stays differentiable in the weights.
    pub fn input_gradient(&self, x: Tensor<B, 2>, mut rng: Option<&mut NpRng>) -> Tensor<B, 2> {
        let first_pre = self.first.forward(x);
        let first_mask = dropout_mask(&first_pre, self.dropout_rate, rng.as_deref_mut());
        let one = masked(self.activation.apply(first_pre.clone()), &first_mask);
        let second_pre = self.second.forward(one.clone());
        let second_mask = dropout_mask(&second_pre, self.dropout_rate, rng.as_deref_mut());
        let two = masked(self.activation.apply(second_pre.clone()), &second_mask);
        let third_pre = self.third.forward(one + two);
        let third_mask = dropout_mask(&third_pre, self.dropout_rate, rng);

        let rows = third_pre.dims()[0];
        let summed = self.head.weight.val().transpose().expand([rows, self.head.weight.dims()[0]]);
        let through = |grad: Tensor<B, 2>, pre: Tensor<B, 2>, mask: &Option<Tensor<B, 2>>, layer: &Linear<B>| {
            (masked(grad, mask) * self.activation.slope(pre)).matmul(layer.weight.val().transpose())
        };
        let from_third = through(summed.clone(), third_pre, &third_mask, &self.third);
        let from_second = through(summed.clone() + from_third.clone(), second_pre, &second_mask, &self.second);
        through(summed + from_third + from_second, first_pre, &first_mask, &self.first)
    }
}

fn masked<B: Backend, const D: usize>(x: Tensor<B, D>, mask: &Option<Tensor<B, D>>) -> Tensor<B, D> {
    match mask {
        Some(m) => x * m.clone(),
        None => x,
    }
}

#[derive(Module, Debug)]
struct ResidualBlock<B: Backend> {
    first_norm: LayerNorm<B>,
    first: Linear<B>,
    second_norm: LayerNorm<B>,
    second: Linear<B>,
}

impl<B: Backend> ResidualBlock<B> {
    fn forward(&self, x: Tensor<B, 2>, dropout_rate: f64, rng: Option<&mut NpRng>) -> Tensor<B, 2> {
        let h = self.first.forward(silu(self.first_norm.forward(x.clone())));
        let h = dropout(silu(self.second_norm.forward(h)), dropout_rate, rng);
        x + self.second.forward(h)
    }
}

/// An orthogonally initialised trunk of residual blocks with layer norm and SiLU, and a head that starts near zero.
#[derive(Module, Debug)]
pub struct ResidualMlp<B: Backend> {
    input: Linear<B>,
    blocks: Vec<ResidualBlock<B>>,
    head: Linear<B>,
    dropout_rate: f64,
}

impl<B: Backend> ResidualMlp<B> {
    pub fn new(inputs: usize, outputs: usize, hidden: usize, blocks: usize, dropout_rate: f64, rng: &mut NpRng, device: &B::Device) -> Self {
        let gain = std::f64::consts::SQRT_2;
        let input = orthogonal_dense(inputs, hidden, gain, rng, device);
        let blocks = (0..blocks)
            .map(|_| ResidualBlock {
                first_norm: LayerNormConfig::new(hidden).init(device),
                first: orthogonal_dense(hidden, hidden, gain, rng, device),
                second_norm: LayerNormConfig::new(hidden).init(device),
                second: orthogonal_dense(hidden, hidden, gain, rng, device),
            })
            .collect();
        let head = with_values(hidden, outputs, uniform(rng, hidden * outputs, -1e-3, 1e-3), vec![0.0; outputs], device);
        Self { input, blocks, head, dropout_rate }
    }

    /// The trunk's features after a final SiLU; pass a generator only while training, to apply dropout.
    pub fn features(&self, x: Tensor<B, 2>, mut rng: Option<&mut NpRng>) -> Tensor<B, 2> {
        let mut h = self.input.forward(x);
        for block in &self.blocks {
            h = block.forward(h, self.dropout_rate, rng.as_deref_mut());
        }
        silu(h)
    }

    pub fn forward(&self, x: Tensor<B, 2>, rng: Option<&mut NpRng>) -> Tensor<B, 2> {
        self.head.forward(self.features(x, rng))
    }
}

/// Gradients scaled together so that their joint norm is at most `max_norm`.
pub fn clipped_gradients<M: AutodiffModule<TrainingBackend>>(loss: Tensor<TrainingBackend, 1>, module: &M, max_norm: f64) -> GradientsParams {
    let mut gradients = GradientsParams::from_grads(loss.backward(), module);
    let mut ids = ParamIds::default();
    module.visit(&mut ids);
    let matrices = ids.matrices.iter().filter_map(|id| gradients.get::<InferenceBackend, 2>(*id)).map(|g| g.powi_scalar(2).sum());
    let vectors = ids.vectors.iter().filter_map(|id| gradients.get::<InferenceBackend, 1>(*id)).map(|g| g.powi_scalar(2).sum());
    let parts: Vec<Tensor<InferenceBackend, 1>> = matrices.chain(vectors).collect();
    if parts.is_empty() {
        return gradients;
    }
    let squared = f64::from(Tensor::cat(parts, 0).sum().into_scalar());
    let scale = max_norm / (squared.sqrt() + 1e-6);
    if scale < 1.0 {
        for id in &ids.matrices {
            if let Some(g) = gradients.remove::<InferenceBackend, 2>(*id) {
                gradients.register(*id, g.mul_scalar(scale));
            }
        }
        for id in &ids.vectors {
            if let Some(g) = gradients.remove::<InferenceBackend, 1>(*id) {
                gradients.register(*id, g.mul_scalar(scale));
            }
        }
    }
    gradients
}

#[derive(Default)]
struct ParamIds {
    matrices: Vec<ParamId>,
    vectors: Vec<ParamId>,
}

impl<B: AutodiffBackend> ModuleVisitor<B> for ParamIds {
    fn visit_float<const D: usize>(&mut self, param: &Param<Tensor<B, D>>) {
        match D {
            2 => self.matrices.push(param.id),
            1 => self.vectors.push(param.id),
            _ => {}
        }
    }
}

