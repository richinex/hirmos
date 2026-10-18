//! Soft actor-critic imitation for continuous actions: SQIL's binary reward or IQ-Learn's implicit reward.

use burn::module::{AutodiffModule, Module, ModuleMapper, ModuleVisitor, Param};
use burn::optim::adaptor::OptimizerAdaptor;
use burn::optim::{Adam, AdamConfig, Optimizer};
use burn::tensor::backend::Backend;
use burn::tensor::{ElementConversion, Tensor, TensorData};

use crate::nprandom::NpRng;
use crate::reinforcement::policy_networks::{clipped_gradients, InferenceBackend, PolicyMlp, ResidualMlp, TrainingBackend};

/// An episodic environment with continuous actions that reports the imitator's inputs for the current step.
pub trait ContinuousEnvironment {
    fn reset(&mut self, seed: u64);
    /// Write the current step's inputs, already restricted to what the acting rule may read.
    fn observe(&self, inputs: &mut Vec<f32>);
    /// Play an action; returns the reward and whether the episode ended.
    fn step(&mut self, action: &[f64]) -> (f64, bool);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImitationReward {
    /// Reward 1 on demonstrated transitions and 0 on the learner's own, with the usual soft Bellman target.
    Binary,
    /// The chi-squared implicit reward Q(s, a) - discount * V(s'), raised on demonstrations and kept small everywhere.
    Implicit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CriticShape {
    /// Two ReLU layers with orthogonal weights.
    Plain,
    /// Residual blocks with layer norm, SiLU and dropout.
    Residual,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoftSettings {
    pub reward: ImitationReward,
    pub critic: CriticShape,
    pub hidden: usize,
    pub blocks: usize,
    pub dropout: f64,
    pub batch: usize,
    pub discount: f64,
    pub target_rate: f64,
    pub actor_learning_rate: f64,
    pub critic_learning_rate: f64,
    pub entropy_learning_rate: f64,
    pub capacity: usize,
    pub expert_share: f64,
    pub updates_per_step: usize,
    pub start_steps: usize,
    pub max_episode_steps: usize,
    pub max_gradient_norm: f64,
    pub action_low: f64,
    pub action_high: f64,
    pub seed: u64,
}

impl Default for SoftSettings {
    fn default() -> Self {
        Self {
            reward: ImitationReward::Binary,
            critic: CriticShape::Plain,
            hidden: 256,
            blocks: 3,
            dropout: 0.05,
            batch: 256,
            discount: 0.99,
            target_rate: 0.005,
            actor_learning_rate: 3e-4,
            critic_learning_rate: 3e-4,
            entropy_learning_rate: 3e-4,
            capacity: 1_000_000,
            expert_share: 0.5,
            updates_per_step: 1,
            start_steps: 5_000,
            max_episode_steps: 1_000,
            max_gradient_norm: 1.0,
            action_low: -1.0,
            action_high: 1.0,
            seed: 0,
        }
    }
}

/// One demonstrated or played step.
#[derive(Clone, Debug, PartialEq)]
pub struct Transition {
    pub state: Vec<f32>,
    pub action: Vec<f32>,
    pub next_state: Vec<f32>,
    pub done: bool,
}

/// A tanh-squashed Gaussian policy on a residual trunk, rescaled into [low, high].
#[derive(Module, Debug)]
pub struct GaussianActor<B: Backend> {
    trunk: ResidualMlp<B>,
    log_std: Param<Tensor<B, 1>>,
    low: f64,
    high: f64,
}

impl<B: Backend> GaussianActor<B> {
    fn new(inputs: usize, actions: usize, hidden: usize, low: f64, high: f64, rng: &mut NpRng, device: &B::Device) -> Self {
        Self {
            trunk: ResidualMlp::new(inputs, actions, hidden, 3, 0.05, rng, device),
            log_std: Param::from_data(TensorData::new(vec![0.0f32; actions], [actions]), device),
            low,
            high,
        }
    }

    /// Reparameterised actions and their log-densities; with no generator the mean action and no dropout.
    fn sample(&self, states: Tensor<B, 2>, rng: Option<&mut NpRng>) -> (Tensor<B, 2>, Tensor<B, 1>) {
        let [rows, _] = states.dims();
        let actions = self.log_std.dims()[0];
        let half_width = (self.high - self.low) / 2.0;
        let (mean, noise) = match rng {
            Some(rng) => {
                let mean = self.trunk.forward(states, Some(&mut *rng));
                let draws: Vec<f32> = (0..rows * actions).map(|_| rng.standard_normal() as f32).collect();
                (mean, Some(Tensor::<B, 2>::from_data(TensorData::new(draws, [rows, actions]), &self.log_std.device())))
            }
            None => (self.trunk.forward(states, None), None),
        };
        let log_std = self.log_std.val().unsqueeze_dim::<2>(0).expand([rows, actions]);
        let (u, squared) = match noise {
            Some(eps) => (mean + log_std.clone().exp() * eps.clone(), eps.powi_scalar(2)),
            None => (mean, Tensor::zeros([rows, actions], &self.log_std.device())),
        };
        let squashed = u.tanh();
        let gaussian = (squared.mul_scalar(-0.5) - log_std).sub_scalar(0.5 * (2.0 * std::f64::consts::PI).ln()).sum_dim(1);
        let jacobian = squashed.clone().powi_scalar(2).neg().add_scalar(1.0 + 1e-6).log().sum_dim(1);
        let log_density = (gaussian - jacobian).sub_scalar(actions as f64 * half_width.ln()).squeeze_dim::<1>(1);
        let action = squashed.add_scalar(1.0).mul_scalar(half_width).add_scalar(self.low);
        (action, log_density)
    }
}

#[derive(Module, Debug)]
enum Critic<B: Backend> {
    Plain(PolicyMlp<B>),
    Residual(ResidualMlp<B>),
}

impl<B: Backend> Critic<B> {
    fn new(shape: CriticShape, inputs: usize, settings: &SoftSettings, rng: &mut NpRng, device: &B::Device) -> Self {
        match shape {
            CriticShape::Plain => Self::Plain(PolicyMlp::orthogonal(inputs, 1, settings.hidden, rng, device)),
            CriticShape::Residual => Self::Residual(ResidualMlp::new(inputs, 1, settings.hidden, settings.blocks, settings.dropout, rng, device)),
        }
    }

    fn value(&self, states: Tensor<B, 2>, actions: Tensor<B, 2>, rng: Option<&mut NpRng>) -> Tensor<B, 1> {
        let x = Tensor::cat(vec![states, actions], 1);
        let out = match self {
            Self::Plain(net) => net.forward(x),
            Self::Residual(net) => net.forward(x, rng),
        };
        out.squeeze_dim::<1>(1)
    }
}

/// A preallocated ring of transitions sampled uniformly with replacement.
struct Ring {
    states: Vec<f32>,
    actions: Vec<f32>,
    next_states: Vec<f32>,
    dones: Vec<f32>,
    width: usize,
    action_width: usize,
    capacity: usize,
    len: usize,
    next: usize,
}

impl Ring {
    fn new(capacity: usize, width: usize, action_width: usize) -> Self {
        Self {
            states: Vec::new(),
            actions: Vec::new(),
            next_states: Vec::new(),
            dones: Vec::new(),
            width,
            action_width,
            capacity: capacity.max(1),
            len: 0,
            next: 0,
        }
    }

    fn push(&mut self, state: &[f32], action: &[f32], next_state: &[f32], done: bool) {
        if self.len < self.capacity {
            self.states.extend_from_slice(state);
            self.actions.extend_from_slice(action);
            self.next_states.extend_from_slice(next_state);
            self.dones.push(f32::from(u8::from(done)));
            self.len += 1;
        } else {
            let (w, a, i) = (self.width, self.action_width, self.next);
            self.states[i * w..(i + 1) * w].copy_from_slice(state);
            self.actions[i * a..(i + 1) * a].copy_from_slice(action);
            self.next_states[i * w..(i + 1) * w].copy_from_slice(next_state);
            self.dones[i] = f32::from(u8::from(done));
            self.next = (self.next + 1) % self.capacity;
        }
    }

    fn sample_into(&self, count: usize, rng: &mut NpRng, batch: &mut Batch) {
        let (w, a) = (self.width, self.action_width);
        for _ in 0..count.min(self.len) {
            let i = rng.bounded_uint64(self.len as u64) as usize;
            batch.states.extend_from_slice(&self.states[i * w..(i + 1) * w]);
            batch.actions.extend_from_slice(&self.actions[i * a..(i + 1) * a]);
            batch.next_states.extend_from_slice(&self.next_states[i * w..(i + 1) * w]);
            batch.dones.push(self.dones[i]);
        }
    }
}

#[derive(Default)]
struct Batch {
    states: Vec<f32>,
    actions: Vec<f32>,
    next_states: Vec<f32>,
    dones: Vec<f32>,
}

impl Batch {
    fn clear(&mut self) {
        self.states.clear();
        self.actions.clear();
        self.next_states.clear();
        self.dones.clear();
    }

    fn rows(&self) -> usize {
        self.dones.len()
    }
}

type Optim<M> = OptimizerAdaptor<Adam, M, TrainingBackend>;

/// A learner whose critics, target critics, actor and entropy weight are trained from demonstrations and its own play.
pub struct SoftImitationLearner {
    actor: GaussianActor<TrainingBackend>,
    critics: [Critic<TrainingBackend>; 2],
    targets: [Critic<InferenceBackend>; 2],
    log_alpha: f64,
    alpha_moments: (f64, f64, i32),
    actor_optimiser: Optim<GaussianActor<TrainingBackend>>,
    critic_optimisers: [Optim<Critic<TrainingBackend>>; 2],
    expert: Ring,
    played: Ring,
    batch: Batch,
    settings: SoftSettings,
    width: usize,
    action_width: usize,
    steps: usize,
    episodes: u64,
    rng: NpRng,
}

impl SoftImitationLearner {
    pub fn new(demonstrations: &[Transition], width: usize, action_width: usize, settings: SoftSettings) -> Self {
        let mut rng = NpRng::seeded(settings.seed);
        let device = Default::default();
        let actor = GaussianActor::new(width, action_width, settings.hidden, settings.action_low, settings.action_high, &mut rng, &device);
        let critics = [0, 1].map(|_| Critic::new(settings.critic, width + action_width, &settings, &mut rng, &device));
        let targets = [critics[0].valid(), critics[1].valid()];
        let expert_capacity = (settings.capacity as f64 * settings.expert_share) as usize;
        let mut expert = Ring::new(expert_capacity, width, action_width);
        for t in demonstrations {
            expert.push(&t.state, &t.action, &t.next_state, t.done);
        }
        let adam = || AdamConfig::new().with_epsilon(1e-8);
        Self {
            actor,
            critics,
            targets,
            log_alpha: 0.0,
            alpha_moments: (0.0, 0.0, 0),
            actor_optimiser: adam().init(),
            critic_optimisers: [adam().init(), adam().init()],
            expert,
            played: Ring::new(settings.capacity - expert_capacity, width, action_width),
            batch: Batch::default(),
            width,
            action_width,
            steps: 0,
            episodes: 0,
            settings,
            rng,
        }
    }

    /// Play one stochastic episode into the learner's buffer, then update once per step once enough play is stored; returns the episode's reward.
    pub fn episode(&mut self, environment: &mut impl ContinuousEnvironment) -> f64 {
        let seed = self.settings.seed + self.episodes;
        self.episodes += 1;
        environment.reset(seed);
        let actor = self.actor.valid();
        let (mut state, mut next) = (Vec::with_capacity(self.width), Vec::with_capacity(self.width));
        environment.observe(&mut state);
        let (mut total, mut length) = (0.0, 0);
        for _ in 0..self.settings.max_episode_steps {
            let (action, _) = actor.sample(row(&state), Some(&mut self.rng));
            let action: Vec<f32> = action.into_data().to_vec::<f32>().expect("f32 actions");
            let played: Vec<f64> = action.iter().map(|&a| f64::from(a)).collect();
            let (reward, done) = environment.step(&played);
            total += reward;
            length += 1;
            next.clear();
            environment.observe(&mut next);
            self.played.push(&state, &action, &next, done);
            std::mem::swap(&mut state, &mut next);
            if done {
                break;
            }
        }
        self.steps += length;
        if self.steps > self.settings.start_steps && self.played.len >= self.settings.batch / 2 {
            for _ in 0..length * self.settings.updates_per_step {
                self.update();
            }
        }
        total
    }

    /// The deterministic rule: the squashed mean action.
    pub fn act(&self, inputs: &[f32]) -> Vec<f64> {
        let (action, _) = self.actor.valid().sample(row(inputs), None);
        action.into_data().to_vec::<f32>().expect("f32 actions").into_iter().map(f64::from).collect()
    }

    fn update(&mut self) {
        let s = self.settings.clone();
        let alpha = self.log_alpha.exp();
        let expert_rows = (s.batch as f64 * s.expert_share) as usize;
        self.batch.clear();
        self.expert.sample_into(expert_rows, &mut self.rng, &mut self.batch);
        let demonstrated = self.batch.rows();
        self.played.sample_into(s.batch - expert_rows, &mut self.rng, &mut self.batch);
        let rows = self.batch.rows();
        let device = Default::default();
        let matrix = |v: &[f32], w: usize| Tensor::<TrainingBackend, 2>::from_data(TensorData::new(v.to_vec(), [rows, w]), &device);
        let (states, actions, next_states) = (matrix(&self.batch.states, self.width), matrix(&self.batch.actions, self.action_width), matrix(&self.batch.next_states, self.width));
        let continuing = Tensor::<InferenceBackend, 1>::from_data(TensorData::new(self.batch.dones.iter().map(|d| 1.0 - d).collect::<Vec<f32>>(), [rows]), &device);

        let actor = self.actor.valid();
        let (next_actions, next_log_density) = actor.sample(next_states.clone().inner(), Some(&mut self.rng));
        let next_values = |rng: &mut NpRng, target: &Critic<InferenceBackend>| {
            target.value(next_states.clone().inner(), next_actions.clone(), Some(rng)) - next_log_density.clone().mul_scalar(alpha)
        };
        let soft_next = next_values(&mut self.rng, &self.targets[0]).min_pair(next_values(&mut self.rng, &self.targets[1]));
        let bootstrap = Tensor::<TrainingBackend, 1>::from_inner(soft_next * continuing.mul_scalar(s.discount));

        for k in 0..2 {
            let q = self.critics[k].value(states.clone(), actions.clone(), Some(&mut self.rng));
            let loss = match s.reward {
                ImitationReward::Binary => {
                    let rewards: Vec<f32> = (0..rows).map(|i| if i < demonstrated { 1.0 } else { 0.0 }).collect();
                    let target = Tensor::<TrainingBackend, 1>::from_data(TensorData::new(rewards, [rows]), &device) + bootstrap.clone();
                    (q - target).powi_scalar(2).mean()
                }
                ImitationReward::Implicit => {
                    let implicit = q - bootstrap.clone();
                    let expert_part = implicit.clone().slice([0..demonstrated]).mean().neg();
                    expert_part + implicit.powi_scalar(2).mean().mul_scalar(0.5)
                }
            };
            let gradients = clipped_gradients(loss, &self.critics[k], s.max_gradient_norm);
            self.critics[k] = self.critic_optimisers[k].step(s.critic_learning_rate, self.critics[k].clone(), gradients);
        }

        let (policy_actions, log_density) = self.actor.sample(states.clone(), Some(&mut self.rng));
        let q_pi = self.critics[0]
            .value(states.clone(), policy_actions.clone(), Some(&mut self.rng))
            .min_pair(self.critics[1].value(states, policy_actions, Some(&mut self.rng)));
        let actor_loss = (log_density.clone().mul_scalar(alpha) - q_pi).mean();
        let gradients = clipped_gradients(actor_loss, &self.actor, s.max_gradient_norm);
        self.actor = self.actor_optimiser.step(s.actor_learning_rate, self.actor.clone(), gradients);

        let mean_log_density = f64::from(log_density.mean().into_scalar().elem::<f32>());
        let target_entropy = -(self.action_width as f64);
        self.adam_alpha(-(mean_log_density + target_entropy), s.entropy_learning_rate);

        for k in 0..2 {
            let mut collected = Collect::default();
            self.critics[k].valid().visit(&mut collected);
            let target = std::mem::replace(&mut self.targets[k], self.critics[k].valid());
            self.targets[k] = target.map(&mut Blend { sources: collected.values, next: 0, rate: s.target_rate });
        }
    }

    /// One Adam step on the scalar log entropy weight.
    fn adam_alpha(&mut self, gradient: f64, learning_rate: f64) {
        let (m, v, t) = &mut self.alpha_moments;
        *t += 1;
        *m = 0.9 * *m + 0.1 * gradient;
        *v = 0.999 * *v + 0.001 * gradient * gradient;
        let m_hat = *m / (1.0 - 0.9f64.powi(*t));
        let v_hat = *v / (1.0 - 0.999f64.powi(*t));
        self.log_alpha -= learning_rate * m_hat / (v_hat.sqrt() + 1e-8);
    }
}

fn row(inputs: &[f32]) -> Tensor<InferenceBackend, 2> {
    Tensor::from_data(TensorData::new(inputs.to_vec(), [1, inputs.len()]), &Default::default())
}

#[derive(Default)]
struct Collect {
    values: Vec<Tensor<InferenceBackend, 1>>,
}

impl ModuleVisitor<InferenceBackend> for Collect {
    fn visit_float<const D: usize>(&mut self, param: &Param<Tensor<InferenceBackend, D>>) {
        let tensor = param.val();
        let count = tensor.shape().num_elements();
        self.values.push(tensor.reshape([count]));
    }
}

/// Moves each parameter a fraction `rate` of the way to the matching source parameter, visited in the same order.
struct Blend {
    sources: Vec<Tensor<InferenceBackend, 1>>,
    next: usize,
    rate: f64,
}

impl ModuleMapper<InferenceBackend> for Blend {
    fn map_float<const D: usize>(&mut self, param: Param<Tensor<InferenceBackend, D>>) -> Param<Tensor<InferenceBackend, D>> {
        let (id, tensor, mapper) = param.consume();
        let source = self.sources[self.next].clone().reshape(tensor.shape());
        self.next += 1;
        Param::from_mapped_value(id, source.mul_scalar(self.rate) + tensor.mul_scalar(1.0 - self.rate), mapper)
    }
}
