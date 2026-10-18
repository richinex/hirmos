use hirmos_causal_core::nprandom::NpRng;
use hirmos_causal_core::reinforcement::soft_imitation::{ContinuousEnvironment, CriticShape, ImitationReward, SoftImitationLearner, SoftSettings, Transition};

/// One step: the state is uniform on [-1, 1]; the expert plays 0.6 times it.
struct Steer {
    state: f32,
}

impl ContinuousEnvironment for Steer {
    fn reset(&mut self, seed: u64) {
        self.state = (2.0 * NpRng::seeded(seed).next_f64() - 1.0) as f32;
    }

    fn observe(&self, inputs: &mut Vec<f32>) {
        inputs.push(self.state);
    }

    fn step(&mut self, action: &[f64]) -> (f64, bool) {
        (-(action[0] - 0.6 * f64::from(self.state)).abs(), true)
    }
}

fn largest_error(reward: ImitationReward, critic: CriticShape, blocks: usize, episodes: usize) -> f64 {
    let mut rng = NpRng::seeded(1);
    let demonstrations: Vec<Transition> = (0..2000)
        .map(|_| {
            let z = (2.0 * rng.next_f64() - 1.0) as f32;
            Transition { state: vec![z], action: vec![(0.6 * f64::from(z) + 0.1 * rng.standard_normal()).clamp(-0.999, 0.999) as f32], next_state: vec![z], done: true }
        })
        .collect();
    let settings = SoftSettings { reward, critic, hidden: 32, blocks, start_steps: 256, capacity: 50_000, max_episode_steps: 1, ..SoftSettings::default() };
    let mut learner = SoftImitationLearner::new(&demonstrations, 1, 1, settings);
    let mut environment = Steer { state: 0.0 };
    for _ in 0..episodes {
        learner.episode(&mut environment);
    }
    (0..=10).map(|i| {
        let z = -1.0 + 0.2 * i as f32;
        (learner.act(&[z])[0] - 0.6 * f64::from(z)).abs()
    }).fold(0.0, f64::max)
}

#[test]
fn the_binary_reward_learns_the_demonstrated_rule() {
    let error = largest_error(ImitationReward::Binary, CriticShape::Plain, 2, 6000);
    assert!(error < 0.1, "largest error {error}");
}

#[test]
fn the_implicit_reward_learns_the_demonstrated_rule() {
    let error = largest_error(ImitationReward::Implicit, CriticShape::Residual, 1, 6000);
    assert!(error < 0.1, "largest error {error}");
}
