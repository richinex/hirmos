//! Copying a demonstrator's actions from the variables each action may read.

use std::fmt;

use burn::module::AutodiffModule;
use burn::nn::loss::{HuberLossConfig, Reduction};
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::tensor::activation::{log_softmax, softmax};
use burn::tensor::backend::Backend;
use burn::tensor::{ElementConversion, Int, Tensor, TensorData};

use crate::nprandom::NpRng;
use crate::reinforcement::policy_networks::{InferenceBackend, PolicyMlp, ResidualMlp, TrainingBackend};

#[derive(Clone, Debug, PartialEq)]
pub enum CloningError {
    NoExamples,
    ShapeMismatch { expected: usize, found: usize },
    ActionOutOfRange { action: usize, actions: usize },
    TooFewForValidation,
    NonFinite,
}

impl fmt::Display for CloningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoExamples => write!(f, "there are no demonstrations to copy"),
            Self::ShapeMismatch { expected, found } => write!(f, "expected {expected} values, found {found}"),
            Self::ActionOutOfRange { action, actions } => write!(f, "action {action} is outside 0..{actions}"),
            Self::TooFewForValidation => write!(f, "the validation share leaves no validation or no training examples"),
            Self::NonFinite => write!(f, "inputs and targets must be finite"),
        }
    }
}

impl std::error::Error for CloningError {}

#[derive(Clone, Debug, PartialEq)]
pub struct CloningSettings {
    pub hidden: usize,
    pub learning_rate: f64,
    pub epochs: usize,
    pub batch_size: usize,
    pub validation_share: f64,
    pub patience: usize,
    pub seed: u64,
}

impl Default for CloningSettings {
    fn default() -> Self {
        Self { hidden: 64, learning_rate: 1e-3, epochs: 100, batch_size: 64, validation_share: 0.2, patience: 10, seed: 0 }
    }
}

/// Demonstrations: one row of `width` inputs per example, row-major.
#[derive(Clone, Copy, Debug)]
pub struct Demonstrations<'a> {
    pub inputs: &'a [f32],
    pub width: usize,
}

impl Demonstrations<'_> {
    fn rows(&self) -> usize {
        if self.width == 0 { 0 } else { self.inputs.len() / self.width }
    }
}

/// A copied rule for discrete actions: the action frequencies when it reads nothing, otherwise a network.
#[derive(Debug)]
pub enum DiscreteClone {
    Constant(Vec<f64>),
    Network { model: PolicyMlp<InferenceBackend>, width: usize },
}

impl DiscreteClone {
    pub fn probabilities(&self, input: &[f32]) -> Vec<f64> {
        match self {
            Self::Constant(p) => p.clone(),
            Self::Network { model, width } => {
                let device = Default::default();
                let x = Tensor::<InferenceBackend, 2>::from_data(TensorData::new(input.to_vec(), [1, *width]), &device);
                values(softmax(model.forward(x), 1))
            }
        }
    }

    /// The highest-scoring action, or a draw from the frequencies when the rule reads nothing.
    pub fn act(&self, input: &[f32], rng: &mut NpRng) -> usize {
        match self {
            Self::Constant(p) => rng.choice_weighted(p),
            Self::Network { .. } => {
                let p = self.probabilities(input);
                (1..p.len()).fold(0, |best, a| if p[a] > p[best] { a } else { best })
            }
        }
    }
}

/// Fit a rule from inputs to `actions` in `0..action_count` by cross-entropy.
pub fn clone_discrete(
    demonstrations: Demonstrations,
    actions: &[usize],
    action_count: usize,
    settings: &CloningSettings,
) -> Result<DiscreteClone, CloningError> {
    if actions.is_empty() {
        return Err(CloningError::NoExamples);
    }
    if let Some(&action) = actions.iter().find(|&&a| a >= action_count) {
        return Err(CloningError::ActionOutOfRange { action, actions: action_count });
    }
    if demonstrations.width == 0 {
        let mut counts = vec![0.0; action_count];
        actions.iter().for_each(|&a| counts[a] += 1.0);
        let total = actions.len() as f64;
        return Ok(DiscreteClone::Constant(counts.into_iter().map(|c| c / total).collect()));
    }
    check(demonstrations, actions.len())?;
    let width = demonstrations.width;
    let targets: Vec<i64> = actions.iter().map(|&a| a as i64).collect();
    let mut rng = NpRng::seeded(settings.seed);
    let device = Default::default();
    let model = PolicyMlp::<TrainingBackend>::new(width, action_count, settings.hidden, &mut rng, &device);
    let model = train(
        model,
        demonstrations,
        settings,
        &mut rng,
        |model, x, rows, _| cross_entropy(model, x, &targets, rows),
        |model, x, rows| scalar(cross_entropy(model, x, &targets, rows)),
    )?;
    Ok(DiscreteClone::Network { model, width })
}

/// A copied rule for continuous actions: the mean action when it reads nothing, otherwise a network.
#[derive(Debug)]
pub enum ContinuousClone {
    Constant(Vec<f64>),
    Network { model: ResidualMlp<InferenceBackend>, width: usize, bounds: Option<Vec<(f64, f64)>> },
}

impl ContinuousClone {
    pub fn act(&self, input: &[f32]) -> Vec<f64> {
        match self {
            Self::Constant(mean) => mean.clone(),
            Self::Network { model, width, bounds } => {
                let device = Default::default();
                let x = Tensor::<InferenceBackend, 2>::from_data(TensorData::new(input.to_vec(), [1, *width]), &device);
                let squashed = values(model.forward(x, None).tanh());
                match bounds {
                    Some(b) => squashed.iter().zip(b).map(|(u, (low, high))| (high + low) / 2.0 + (high - low) / 2.0 * u).collect(),
                    None => squashed,
                }
            }
        }
    }
}

/// Fit a rule from inputs to actions of `action_width` values by Huber loss on a tanh-squashed output, rescaled into `bounds` when given.
pub fn clone_continuous(
    demonstrations: Demonstrations,
    targets: &[f32],
    action_width: usize,
    bounds: Option<Vec<(f64, f64)>>,
    settings: &CloningSettings,
) -> Result<ContinuousClone, CloningError> {
    let rows = if action_width == 0 { 0 } else { targets.len() / action_width };
    if rows == 0 {
        return Err(CloningError::NoExamples);
    }
    if targets.iter().any(|t| !t.is_finite()) {
        return Err(CloningError::NonFinite);
    }
    if demonstrations.width == 0 {
        let mean = (0..action_width).map(|j| (0..rows).map(|r| targets[r * action_width + j] as f64).sum::<f64>() / rows as f64).collect();
        return Ok(ContinuousClone::Constant(mean));
    }
    check(demonstrations, rows)?;
    let width = demonstrations.width;
    let (middle, half): (Vec<f32>, Vec<f32>) = match &bounds {
        Some(b) => b.iter().map(|(low, high)| (((high + low) / 2.0) as f32, ((high - low) / 2.0) as f32)).unzip(),
        None => (vec![0.0; action_width], vec![1.0; action_width]),
    };
    let mut rng = NpRng::seeded(settings.seed);
    let device = Default::default();
    let model = ResidualMlp::<TrainingBackend>::new(width, action_width, 256, 3, 0.05, &mut rng, &device);
    let fit = Rescaled { targets, width: action_width, middle: &middle, half: &half };
    let model = train(
        model,
        demonstrations,
        settings,
        &mut rng,
        |model, x, rows, dropout| fit.huber(model, x, rows, dropout),
        |model, x, rows| scalar(fit.huber(model, x, rows, None)),
    )?;
    Ok(ContinuousClone::Network { model, width, bounds })
}

fn check(demonstrations: Demonstrations, rows: usize) -> Result<(), CloningError> {
    if demonstrations.inputs.len() != rows * demonstrations.width {
        return Err(CloningError::ShapeMismatch { expected: rows * demonstrations.width, found: demonstrations.inputs.len() });
    }
    if demonstrations.inputs.iter().any(|v| !v.is_finite()) {
        return Err(CloningError::NonFinite);
    }
    Ok(())
}

fn cross_entropy<B: Backend>(model: &PolicyMlp<B>, x: Tensor<B, 2>, targets: &[i64], rows: &[usize]) -> Tensor<B, 1> {
    let y = Tensor::<B, 1, Int>::from_data(TensorData::new(rows.iter().map(|&r| targets[r]).collect::<Vec<_>>(), [rows.len()]), &x.device());
    log_softmax(model.forward(x), 1).gather(1, y.unsqueeze_dim(1)).mean().neg()
}

struct Rescaled<'a> {
    targets: &'a [f32],
    width: usize,
    middle: &'a [f32],
    half: &'a [f32],
}

impl Rescaled<'_> {
    fn huber<B: Backend>(&self, model: &ResidualMlp<B>, x: Tensor<B, 2>, rows: &[usize], dropout: Option<&mut NpRng>) -> Tensor<B, 1> {
        let device = x.device();
        let w = self.width;
        let y: Vec<f32> = rows.iter().flat_map(|&r| self.targets[r * w..(r + 1) * w].iter().copied()).collect();
        let y = Tensor::from_data(TensorData::new(y, [rows.len(), w]), &device);
        let scale = |v: &[f32]| Tensor::<B, 1>::from_data(TensorData::new(v.to_vec(), [w]), &device).unsqueeze_dim::<2>(0);
        let prediction = scale(self.middle) + scale(self.half) * model.forward(x, dropout).tanh();
        HuberLossConfig::new(1.0).init().forward(prediction, y, Reduction::Mean)
    }
}

fn scalar<B: Backend>(loss: Tensor<B, 1>) -> f64 {
    f64::from(loss.into_scalar().elem::<f32>())
}

fn values<B: Backend>(tensor: Tensor<B, 2>) -> Vec<f64> {
    tensor.into_data().to_vec::<f32>().expect("f32 backend").into_iter().map(f64::from).collect()
}

/// Shuffle once into validation and training rows, then train by Adam on reshuffled minibatches, keeping the model with the lowest validation loss.
fn train<M>(
    mut model: M,
    demonstrations: Demonstrations,
    settings: &CloningSettings,
    rng: &mut NpRng,
    loss: impl Fn(&M, Tensor<TrainingBackend, 2>, &[usize], Option<&mut NpRng>) -> Tensor<TrainingBackend, 1>,
    validation_loss: impl Fn(&M::InnerModule, Tensor<InferenceBackend, 2>, &[usize]) -> f64,
) -> Result<M::InnerModule, CloningError>
where
    M: AutodiffModule<TrainingBackend> + Clone,
{
    let rows = demonstrations.rows();
    let mut order = rng.permutation(rows);
    let split = (rows as f64 * settings.validation_share) as usize;
    if split == 0 || split == rows {
        return Err(CloningError::TooFewForValidation);
    }
    let validation: Vec<usize> = order.drain(..split).collect();
    let mut training = order;
    let device = Default::default();
    let batch = |picked: &[usize]| {
        let width = demonstrations.width;
        let x: Vec<f32> = picked.iter().flat_map(|&r| demonstrations.inputs[r * width..(r + 1) * width].iter().copied()).collect();
        Tensor::<TrainingBackend, 2>::from_data(TensorData::new(x, [picked.len(), width]), &device)
    };
    let validation_inputs = batch(&validation).inner();
    let mut optimizer = AdamConfig::new().with_epsilon(1e-8).init();
    let (mut best, mut best_loss, mut stale) = (model.clone(), f64::INFINITY, 0);
    for _ in 0..settings.epochs {
        rng.shuffle(&mut training);
        for picked in training.chunks(settings.batch_size.max(1)) {
            let objective = loss(&model, batch(picked), picked, Some(&mut *rng));
            let gradients = GradientsParams::from_grads(objective.backward(), &model);
            model = optimizer.step(settings.learning_rate, model, gradients);
        }
        let loss_now = validation_loss(&model.valid(), validation_inputs.clone(), &validation);
        if loss_now + 1e-6 < best_loss {
            best_loss = loss_now;
            best = model.clone();
            stale = 0;
        } else {
            stale += 1;
            if stale >= settings.patience {
                break;
            }
        }
    }
    Ok(best.valid())
}
