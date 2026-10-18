use burn::tensor::{Tensor, TensorData};
use hirmos_causal_core::reinforcement::adversarial_imitation::{AdversarialLearner, AdversarialPolicy, AdversarialSettings, ImitationEnvironment};
use hirmos_causal_core::nprandom::NpRng;
use hirmos_causal_core::reinforcement::policy_networks::{Activation, InferenceBackend, SkipMlp};

/// Experiment 9.11: U1, U2 ~ Bern(0.8), U3 ~ Bern(0.2), U4 ~ Bern(0.1); Z0 = U1 ^ U3, Z1 = Z0 ^ U2 ^ U4.
#[derive(Clone, Copy)]
struct World {
    u: [u8; 4],
}

impl World {
    fn z(&self) -> [u8; 2] {
        let z0 = self.u[0] ^ self.u[2];
        [z0, z0 ^ self.u[1] ^ self.u[3]]
    }

    fn reward(&self, x0: u8, x1: u8) -> f64 {
        let [z0, z1] = self.z();
        f64::from(1 - (x0 ^ x1 ^ z0 ^ z1 ^ self.u[2]))
    }

    fn all() -> impl Iterator<Item = (World, f64)> {
        let p = [0.8, 0.8, 0.2, 0.1];
        (0..16u8).map(move |bits| {
            let u = [0, 1, 2, 3].map(|i| (bits >> i) & 1);
            let weight = (0..4).map(|i| if u[i] == 1 { p[i] } else { 1.0 - p[i] }).product();
            (World { u }, weight)
        })
    }

    fn sample(rng: &mut NpRng) -> Self {
        let p = [0.8, 0.8, 0.2, 0.1];
        World { u: p.map(|q| u8::from(rng.next_f64() < q)) }
    }
}

/// Which of Z0, X0, Z1 each step's rule reads, one-hot encoded over the union in time order (X0, Z0, Z1).
#[derive(Clone, Copy)]
struct Reads {
    first: [bool; 3],
    second: [bool; 3],
}

impl Reads {
    fn used(&self) -> Vec<usize> {
        (0..3).filter(|&i| self.first[i] || self.second[i]).collect()
    }

    fn encode(&self, step: usize, x0: Option<u8>, z: [u8; 2], out: &mut Vec<f32>) {
        let keep = if step == 0 { self.first } else if step == 1 { self.second } else { [true; 3] };
        let values = [x0, Some(z[0]), if step >= 1 { Some(z[1]) } else { None }];
        for i in self.used() {
            let value = values[i].filter(|_| keep[i]);
            out.extend([0u8, 1].map(|v| if value == Some(v) { 1.0 } else { 0.0 }));
        }
    }
}

struct Inventory {
    reads: Reads,
    world: World,
    step: usize,
    x0: Option<u8>,
    rng: NpRng,
}

impl ImitationEnvironment for Inventory {
    fn reset(&mut self, seed: u64) {
        self.rng = NpRng::seeded(seed);
        self.world = World::sample(&mut self.rng);
        self.step = 0;
        self.x0 = None;
    }

    fn observe(&self, inputs: &mut Vec<f32>) {
        self.reads.encode(self.step, self.x0, self.world.z(), inputs);
    }

    fn step(&mut self, action: usize) -> (f64, bool) {
        self.step += 1;
        match self.x0 {
            None => {
                self.x0 = Some(action as u8);
                (0.0, false)
            }
            Some(x0) => (self.world.reward(x0, action as u8), true),
        }
    }
}

/// The value of copying P(X0) and P(X1 | Z1) exactly; the expert's own value is 0.4352 because the reward reads Z0 directly.
const PERFECT_COPY: f64 = 0.4289;

fn value(policy: &AdversarialPolicy, reads: Reads) -> f64 {
    let mut total = 0.0;
    for (world, weight) in World::all() {
        let mut first = Vec::new();
        reads.encode(0, None, world.z(), &mut first);
        let p0 = policy.probabilities(&first);
        for x0 in 0..2u8 {
            let mut second = Vec::new();
            reads.encode(1, Some(x0), world.z(), &mut second);
            let p1 = policy.probabilities(&second);
            for x1 in 0..2u8 {
                total += weight * p0[x0 as usize] * p1[x1 as usize] * world.reward(x0, x1);
            }
        }
    }
    total
}

/// The mean, over the final `averaged` rounds, of the exact value and the probabilities of acting 1 at each probed input.
fn learn(reads: Reads, rounds: usize, averaged: usize) -> (f64, Vec<f64>) {
    let mut rng = NpRng::seeded(0);
    let (mut inputs, mut actions) = (Vec::new(), Vec::new());
    for _ in 0..2000 {
        let world = World::sample(&mut rng);
        let x0 = u8::from(rng.next_f64() < 0.68);
        let x1 = world.u[1] ^ world.z()[1];
        reads.encode(0, None, world.z(), &mut inputs);
        actions.push(x0 as usize);
        reads.encode(1, Some(x0), world.z(), &mut inputs);
        actions.push(x1 as usize);
    }
    let width = 2 * reads.used().len();
    let settings = AdversarialSettings {
        hidden: 64,
        actor_learning_rate: 1e-4,
        discriminator_learning_rate: 1e-3,
        ppo_epochs: 4,
        minibatch: 256,
        normalise_advantages: true,
        gradient_penalty: None,
        discriminator_updates: 5,
        discriminator_minibatch: 256,
        discriminator_dropout: 0.0,
        max_steps: 2,
        episodes_per_round: 200,
        ..AdversarialSettings::default()
    };
    let mut learner = AdversarialLearner::new(&inputs, width, &actions, 2, settings);
    let mut environment = Inventory { reads, world: World { u: [0; 4] }, step: 0, x0: None, rng: NpRng::seeded(0) };
    let probes: Vec<Vec<f32>> = [(0, None, [0, 0]), (1, Some(0), [0, 0]), (1, Some(0), [0, 1])]
        .iter()
        .map(|&(step, x0, z)| {
            let mut v = Vec::new();
            reads.encode(step, x0, z, &mut v);
            v
        })
        .collect();
    let (mut value_total, mut probabilities) = (0.0, vec![0.0; probes.len()]);
    for round in 0..rounds {
        learner.round(&mut environment);
        if round + averaged >= rounds {
            let policy = learner.policy();
            value_total += value(&policy, reads);
            for (total, probe) in probabilities.iter_mut().zip(&probes) {
                *total += policy.probabilities(probe)[1];
            }
        }
    }
    (value_total / averaged as f64, probabilities.into_iter().map(|p| p / averaged as f64).collect())
}

#[test]
fn the_explicit_input_gradient_matches_finite_differences() {
    for activation in [Activation::Relu, Activation::Tanh, Activation::Sigmoid, Activation::LeakyRelu] {
        let mut rng = NpRng::seeded(4);
        let device = Default::default();
        let net = SkipMlp::<InferenceBackend>::new(5, 1, 16, activation, &mut rng, &device);
        let x: Vec<f32> = (0..15).map(|_| rng.standard_normal() as f32).collect();
        let tensor = |v: Vec<f32>| Tensor::<InferenceBackend, 2>::from_data(TensorData::new(v, [3, 5]), &device);
        let gradient = net.input_gradient(tensor(x.clone()), None).into_data().to_vec::<f32>().unwrap();
        let output = |v: Vec<f32>| net.forward(tensor(v)).into_data().to_vec::<f32>().unwrap();
        for i in 0..15 {
            let (mut up, mut down) = (x.clone(), x.clone());
            up[i] += 1e-2;
            down[i] -= 1e-2;
            let numeric = (output(up)[i / 5] - output(down)[i / 5]) / 2e-2;
            assert!((numeric - gradient[i]).abs() < 2e-3, "{activation:?} input {i}: {numeric} against {}", gradient[i]);
        }
    }
}

#[test]
fn adversarial_imitation_matches_the_demonstrated_choices_on_average() {
    let causal = Reads { first: [false; 3], second: [true, false, true] };
    let (value, probabilities) = learn(causal, 300, 100);
    for (learned, expert) in probabilities.iter().zip([0.68, 0.879, 0.311]) {
        assert!((learned - expert).abs() < 0.05, "probabilities {probabilities:?}");
    }
    assert!((value - PERFECT_COPY).abs() < 0.02, "value {value}");
}
