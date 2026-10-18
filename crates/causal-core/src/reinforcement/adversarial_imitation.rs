//! Adversarial imitation: a policy learns from a discriminator that tells its behaviour from the demonstrator's.

use burn::module::AutodiffModule;
use burn::nn::loss::{HuberLossConfig, Reduction};
use burn::optim::adaptor::OptimizerAdaptor;
use burn::optim::{Adam, AdamConfig, GradientsParams, Optimizer};
use burn::tensor::activation::{log_softmax, softmax, softplus};
use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor, TensorData};

use crate::nprandom::NpRng;
use crate::reinforcement::policy_networks::{clipped_gradients, Activation, InferenceBackend, SkipMlp, TrainingBackend};

/// An episodic environment that reports the imitator's inputs for the current step.
pub trait ImitationEnvironment {
    fn reset(&mut self, seed: u64);
    /// Write the current step's inputs, already restricted to what the acting rule may read.
    fn observe(&self, inputs: &mut Vec<f32>);
    /// Play a discrete action; returns the reward and whether the episode ended.
    fn step(&mut self, action: usize) -> (f64, bool);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscriminatorLoss {
    /// Cross-entropy on logits; the policy's reward is -softplus(-score).
    CrossEntropy,
    /// Score difference; the policy's reward is the score.
    Linear,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdversarialSettings {
    pub hidden: usize,
    pub actor_learning_rate: f64,
    pub critic_learning_rate: f64,
    pub discriminator_learning_rate: f64,
    pub discount: f64,
    pub gae_lambda: f64,
    pub clip: f64,
    pub ppo_epochs: usize,
    pub minibatch: usize,
    pub entropy_weight: f64,
    pub value_weight: f64,
    pub max_gradient_norm: f64,
    pub normalise_advantages: bool,
    pub loss: DiscriminatorLoss,
    /// Weight of the penalty on the discriminator's input-gradient norm, if any.
    pub gradient_penalty: Option<f64>,
    pub discriminator_updates: usize,
    pub discriminator_minibatch: usize,
    pub discriminator_dropout: f64,
    pub instance_noise: f64,
    pub label_smoothing: f64,
    pub max_steps: usize,
    pub episodes_per_round: usize,
    pub seed: u64,
}

impl Default for AdversarialSettings {
    fn default() -> Self {
        Self {
            hidden: 1024,
            actor_learning_rate: 3e-4,
            critic_learning_rate: 1e-4,
            discriminator_learning_rate: 5e-5,
            discount: 0.99,
            gae_lambda: 0.95,
            clip: 0.2,
            ppo_epochs: 10,
            minibatch: 2048,
            entropy_weight: 1e-2,
            value_weight: 0.5,
            max_gradient_norm: 0.5,
            normalise_advantages: false,
            loss: DiscriminatorLoss::CrossEntropy,
            gradient_penalty: Some(10.0),
            discriminator_updates: 3,
            discriminator_minibatch: 4096,
            discriminator_dropout: 0.2,
            instance_noise: 0.0,
            label_smoothing: 0.0,
            max_steps: 10,
            episodes_per_round: 10,
            seed: 0,
        }
    }
}

/// What one round of rollouts and updates produced.
#[derive(Clone, Debug, PartialEq)]
pub struct RoundSummary {
    pub mean_return: f64,
    pub discriminator_loss: f64,
    pub discriminator_accuracy: f64,
}

type Optim<M> = OptimizerAdaptor<Adam, M, TrainingBackend>;

/// A discrete-action learner with its actor, critic and discriminator.
pub struct AdversarialLearner {
    actor: SkipMlp<TrainingBackend>,
    critic: SkipMlp<TrainingBackend>,
    discriminator: SkipMlp<TrainingBackend>,
    actor_optimiser: Optim<SkipMlp<TrainingBackend>>,
    critic_optimiser: Optim<SkipMlp<TrainingBackend>>,
    discriminator_optimiser: Optim<SkipMlp<TrainingBackend>>,
    expert: Tensor<TrainingBackend, 2>,
    width: usize,
    actions: usize,
    settings: AdversarialSettings,
    rng: NpRng,
}

impl AdversarialLearner {
    /// Start from demonstrations: `inputs` holds one row of `width` values per demonstrated action.
    pub fn new(inputs: &[f32], width: usize, demonstrated: &[usize], actions: usize, settings: AdversarialSettings) -> Self {
        let mut rng = NpRng::seeded(settings.seed);
        let device = Default::default();
        let actor = SkipMlp::new(width, actions, settings.hidden, Activation::Relu, &mut rng, &device);
        let critic = SkipMlp::new(width, 1, settings.hidden, Activation::Relu, &mut rng, &device);
        let discriminator =
            SkipMlp::new(width + actions, 1, settings.hidden, Activation::Relu, &mut rng, &device).with_dropout(settings.discriminator_dropout);
        let mut rows = Vec::with_capacity(demonstrated.len() * (width + actions));
        for (row, &action) in inputs.chunks(width.max(1)).zip(demonstrated) {
            push_pair(&mut rows, row, action, actions);
        }
        let expert = Tensor::from_data(TensorData::new(rows, [demonstrated.len(), width + actions]), &device);
        let adam = || AdamConfig::new().with_epsilon(1e-8).init();
        Self {
            actor,
            critic,
            discriminator,
            actor_optimiser: adam(),
            critic_optimiser: adam(),
            discriminator_optimiser: adam(),
            expert,
            width,
            actions,
            settings,
            rng,
        }
    }

    /// Roll out the current policy, reward it by the discriminator, update it by PPO, then update the discriminator.
    pub fn round(&mut self, environment: &mut impl ImitationEnvironment) -> RoundSummary {
        let rollout = self.rollout(environment);
        let device = Default::default();
        let pairs = Tensor::<TrainingBackend, 2>::from_data(
            TensorData::new(rollout.pairs.clone(), [rollout.actions.len(), self.width + self.actions]),
            &device,
        );
        let scores = column(self.discriminator.valid().forward_training(pairs.clone().inner(), Some(&mut self.rng)));
        let mut rewards: Vec<f64> = scores
            .iter()
            .map(|&s| match self.settings.loss {
                DiscriminatorLoss::Linear => s,
                DiscriminatorLoss::CrossEntropy => -softplus_scalar(-s),
            })
            .collect();
        normalise(&mut rewards);
        let (advantages, returns) = advantages(&rewards, &rollout, self.settings.discount, self.settings.gae_lambda);
        self.ppo(&rollout, advantages, returns);
        let (loss, accuracy) = self.train_discriminator(pairs);
        RoundSummary { mean_return: rollout.returns.iter().sum::<f64>() / rollout.returns.len() as f64, discriminator_loss: loss, discriminator_accuracy: accuracy }
    }

    /// The trained rule.
    pub fn policy(&self) -> AdversarialPolicy {
        AdversarialPolicy { actor: self.actor.valid(), width: self.width }
    }

    fn rollout(&mut self, environment: &mut impl ImitationEnvironment) -> Rollout {
        let actor = self.actor.valid();
        let critic = self.critic.valid();
        let mut rollout = Rollout::default();
        let mut inputs = Vec::with_capacity(self.width);
        for episode in 0..self.settings.episodes_per_round {
            environment.reset(self.settings.seed + episode as u64);
            let (mut steps, mut total, mut done) = (0, 0.0, false);
            while !done && steps < self.settings.max_steps {
                inputs.clear();
                environment.observe(&mut inputs);
                let x = row_tensor(&inputs);
                let probabilities = column(softmax(actor.forward(x.clone()), 1));
                let action = self.rng.choice_weighted(&probabilities);
                rollout.values.push(column(critic.forward(x))[0]);
                rollout.log_probabilities.push(probabilities[action].ln());
                let (reward, ended) = environment.step(action);
                done = ended;
                total += reward;
                rollout.inputs.extend_from_slice(&inputs);
                push_pair(&mut rollout.pairs, &inputs, action, self.actions);
                rollout.actions.push(action as i64);
                rollout.dones.push(done);
                steps += 1;
            }
            inputs.clear();
            environment.observe(&mut inputs);
            rollout.last_values.push(column(critic.forward(row_tensor(&inputs)))[0]);
            rollout.lengths.push(steps);
            rollout.returns.push(total);
        }
        rollout
    }

    fn ppo(&mut self, rollout: &Rollout, mut advantages: Vec<f64>, returns: Vec<f64>) {
        if self.settings.normalise_advantages {
            normalise(&mut advantages);
        }
        let n = rollout.actions.len();
        let device = Default::default();
        let s = &self.settings;
        let huber = HuberLossConfig::new(1.0).init();
        for _ in 0..s.ppo_epochs {
            let order = self.rng.permutation(n);
            for picked in order.chunks(s.minibatch.max(1)) {
                let x = gather_rows(&rollout.inputs, self.width, picked);
                let a = Tensor::<TrainingBackend, 1, Int>::from_data(TensorData::new(picked.iter().map(|&i| rollout.actions[i]).collect::<Vec<_>>(), [picked.len()]), &device);
                let pick = |v: &[f64]| Tensor::<TrainingBackend, 1>::from_data(TensorData::new(picked.iter().map(|&i| v[i] as f32).collect::<Vec<_>>(), [picked.len()]), &device);
                let (old, advantage, target) = (pick(&rollout.log_probabilities), pick(&advantages), pick(&returns));

                let log_p = log_softmax(self.actor.forward(x.clone()), 1);
                let entropy = (log_p.clone().exp() * log_p.clone()).sum_dim(1).neg().mean();
                let new = log_p.gather(1, a.unsqueeze_dim(1)).squeeze_dim::<1>(1);
                let ratio = (new - old).exp();
                let clipped = ratio.clone().clamp(1.0 - s.clip, 1.0 + s.clip) * advantage.clone();
                let policy_loss = (ratio * advantage).min_pair(clipped).mean().neg();
                let actor_loss = policy_loss - entropy.mul_scalar(s.entropy_weight);
                let gradients = clipped_gradients(actor_loss, &self.actor, s.max_gradient_norm);
                self.actor = self.actor_optimiser.step(s.actor_learning_rate, self.actor.clone(), gradients);

                let value = self.critic.forward(x).squeeze_dim::<1>(1);
                let critic_loss = huber.forward(value, target, Reduction::Mean).mul_scalar(s.value_weight);
                let gradients = clipped_gradients(critic_loss, &self.critic, s.max_gradient_norm);
                self.critic = self.critic_optimiser.step(s.critic_learning_rate, self.critic.clone(), gradients);
            }
        }
    }

    fn train_discriminator(&mut self, policy_pairs: Tensor<TrainingBackend, 2>) -> (f64, f64) {
        let device = Default::default();
        let s = self.settings.clone();
        let (real_count, fake_count) = (self.expert.dims()[0], policy_pairs.dims()[0]);
        let n = real_count.min(fake_count);
        if n == 0 {
            return (0.0, 0.0);
        }
        let index = |order: Vec<usize>| Tensor::<TrainingBackend, 1, Int>::from_data(TensorData::new(order.into_iter().take(n).map(|i| i as i64).collect::<Vec<_>>(), [n]), &device);
        let real_all = self.expert.clone().select(0, index(self.rng.permutation(real_count)));
        let fake_all = policy_pairs.select(0, index(self.rng.permutation(fake_count)));
        let (mut total_loss, mut total_accuracy, mut batches) = (0.0, 0.0, 0);
        for _ in 0..s.discriminator_updates {
            let order = self.rng.permutation(n);
            for picked in order.chunks(s.discriminator_minibatch.max(1)) {
                let rows = Tensor::<TrainingBackend, 1, Int>::from_data(TensorData::new(picked.iter().map(|&i| i as i64).collect::<Vec<_>>(), [picked.len()]), &device);
                let mut real = real_all.clone().select(0, rows.clone()).detach();
                let mut fake = fake_all.clone().select(0, rows).detach();
                if s.instance_noise > 0.0 {
                    real = real.clone() + self.noise(real.dims(), s.instance_noise);
                    fake = fake.clone() + self.noise(fake.dims(), s.instance_noise);
                }
                let real_scores = self.discriminator.forward_training(real.clone(), Some(&mut self.rng)).squeeze_dim::<1>(1);
                let fake_scores = self.discriminator.forward_training(fake.clone(), Some(&mut self.rng)).squeeze_dim::<1>(1);
                let mut loss = match s.loss {
                    DiscriminatorLoss::Linear => fake_scores.clone().mean() - real_scores.clone().mean(),
                    DiscriminatorLoss::CrossEntropy => {
                        logistic_loss(real_scores.clone(), 1.0 - s.label_smoothing) + logistic_loss(fake_scores.clone(), s.label_smoothing)
                    }
                };
                if let Some(weight) = s.gradient_penalty {
                    loss = loss + self.gradient_penalty(real, fake).mul_scalar(weight);
                }
                total_loss += f64::from(loss.clone().into_scalar());
                let correct = |scores: Tensor<TrainingBackend, 1>, real: bool| {
                    let v = column1(scores.inner());
                    v.iter().filter(|&&x| (x >= 0.0) == real).count() as f64 / v.len() as f64
                };
                total_accuracy += 0.5 * (correct(real_scores, true) + correct(fake_scores, false));
                batches += 1;
                let gradients = GradientsParams::from_grads(loss.backward(), &self.discriminator);
                self.discriminator = self.discriminator_optimiser.step(s.discriminator_learning_rate, self.discriminator.clone(), gradients);
            }
        }
        (total_loss / batches as f64, total_accuracy / batches as f64)
    }

    fn noise(&mut self, [rows, columns]: [usize; 2], scale: f64) -> Tensor<TrainingBackend, 2> {
        let values: Vec<f32> = (0..rows * columns).map(|_| (scale * self.rng.standard_normal()) as f32).collect();
        Tensor::from_data(TensorData::new(values, [rows, columns]), &Default::default())
    }

    fn gradient_penalty(&mut self, real: Tensor<TrainingBackend, 2>, fake: Tensor<TrainingBackend, 2>) -> Tensor<TrainingBackend, 1> {
        let [rows, columns] = real.dims();
        let weights: Vec<f32> = (0..rows).map(|_| self.rng.next_f64() as f32).collect();
        let eps = Tensor::<TrainingBackend, 1>::from_data(TensorData::new(weights, [rows]), &Default::default()).unsqueeze_dim::<2>(1).expand([rows, columns]);
        let mixed = eps.clone() * real + eps.neg().add_scalar(1.0) * fake;
        let gradient = self.discriminator.input_gradient(mixed, Some(&mut self.rng));
        (gradient.powi_scalar(2).sum_dim(1).sqrt().sub_scalar(1.0)).powi_scalar(2).mean()
    }
}

/// A trained discrete rule.
#[derive(Debug)]
pub struct AdversarialPolicy {
    actor: SkipMlp<InferenceBackend>,
    width: usize,
}

impl AdversarialPolicy {
    pub fn probabilities(&self, inputs: &[f32]) -> Vec<f64> {
        let x = Tensor::<InferenceBackend, 2>::from_data(TensorData::new(inputs.to_vec(), [1, self.width]), &Default::default());
        column(softmax(self.actor.forward(x), 1))
    }

    /// The most probable action.
    pub fn act(&self, inputs: &[f32]) -> usize {
        let p = self.probabilities(inputs);
        (1..p.len()).fold(0, |best, a| if p[a] > p[best] { a } else { best })
    }
}

#[derive(Default)]
struct Rollout {
    inputs: Vec<f32>,
    pairs: Vec<f32>,
    actions: Vec<i64>,
    log_probabilities: Vec<f64>,
    values: Vec<f64>,
    dones: Vec<bool>,
    last_values: Vec<f64>,
    lengths: Vec<usize>,
    returns: Vec<f64>,
}

/// Generalised advantage estimates per episode, bootstrapped from each episode's final value unless it ended.
fn advantages(rewards: &[f64], rollout: &Rollout, discount: f64, lambda: f64) -> (Vec<f64>, Vec<f64>) {
    let n = rewards.len();
    let (mut advantages, mut returns) = (vec![0.0; n], vec![0.0; n]);
    let mut start = 0;
    for (episode, &length) in rollout.lengths.iter().enumerate() {
        let mut running = 0.0;
        for t in (start..start + length).rev() {
            let next = if t + 1 < start + length { rollout.values[t + 1] } else { rollout.last_values[episode] };
            let continuing = if rollout.dones[t] { 0.0 } else { 1.0 };
            let delta = rewards[t] + discount * next * continuing - rollout.values[t];
            running = delta + discount * lambda * continuing * running;
            advantages[t] = running;
            returns[t] = running + rollout.values[t];
        }
        start += length;
    }
    (advantages, returns)
}

fn normalise(values: &mut [f64]) {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let deviation = (values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n).sqrt();
    values.iter_mut().for_each(|v| *v = (*v - mean) / (deviation + 1e-8));
}

fn softplus_scalar(x: f64) -> f64 {
    if x > 0.0 { x + (-x).exp().ln_1p() } else { x.exp().ln_1p() }
}

fn logistic_loss(scores: Tensor<TrainingBackend, 1>, target: f64) -> Tensor<TrainingBackend, 1> {
    (softplus(scores.clone(), 1.0) - scores.mul_scalar(target)).mean()
}

fn push_pair(rows: &mut Vec<f32>, inputs: &[f32], action: usize, actions: usize) {
    rows.extend_from_slice(inputs);
    rows.extend((0..actions).map(|a| if a == action { 1.0 } else { 0.0 }));
}

fn row_tensor(inputs: &[f32]) -> Tensor<InferenceBackend, 2> {
    Tensor::from_data(TensorData::new(inputs.to_vec(), [1, inputs.len()]), &Default::default())
}

fn gather_rows(inputs: &[f32], width: usize, picked: &[usize]) -> Tensor<TrainingBackend, 2> {
    let rows: Vec<f32> = picked.iter().flat_map(|&i| inputs[i * width..(i + 1) * width].iter().copied()).collect();
    Tensor::from_data(TensorData::new(rows, [picked.len(), width]), &Default::default())
}

fn column<B: Backend>(tensor: Tensor<B, 2>) -> Vec<f64> {
    tensor.into_data().to_vec::<f32>().expect("f32 backend").into_iter().map(f64::from).collect()
}

fn column1<B: Backend>(tensor: Tensor<B, 1>) -> Vec<f64> {
    tensor.into_data().to_vec::<f32>().expect("f32 backend").into_iter().map(f64::from).collect()
}
