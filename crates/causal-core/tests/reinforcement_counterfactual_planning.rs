use hirmos_causal_core::reinforcement::decision_process::{
    action_values, evaluate_finite_policy, finite_horizon,
};
use hirmos_causal_core::reinforcement::counterfactual_planning::{
    CounterfactualModel, CounterfactualUcbvi, Episode, EpisodeStep,
};
use hirmos_causal_core::nprandom::NpRng;

const ERROR: f64 = 0.9;
const DEMAND: f64 = 0.1;
const VALUE: f64 = 0.1;

fn bernoulli(p: f64, outcome: usize) -> f64 {
    if outcome == 1 { p } else { 1.0 - p }
}

fn cell(s: usize, x: usize, a: usize) -> usize {
    (s * 2 + x) * 2 + a
}

// The inventory example: S' = (S or A) xor U1 xor U2, intention X = S xor U1, reward
// Y = S xor A xor U1 xor U3. The intention reveals U1 = X xor S.
fn inventory_model() -> CounterfactualModel {
    let mut rewards = vec![0.0; 8];
    let mut transitions = vec![0.0; 32];
    for s in 0..2 {
        for x in 0..2 {
            let u1 = x ^ s;
            for a in 0..2 {
                rewards[cell(s, x, a)] = (0..2)
                    .filter(|u3| s ^ a ^ u1 ^ u3 == 1)
                    .map(|u3| bernoulli(VALUE, u3))
                    .sum();
                for u2 in 0..2 {
                    let next = (s | a) ^ u1 ^ u2;
                    for u1_next in 0..2 {
                        let intended = next ^ u1_next;
                        transitions[cell(s, x, a) * 4 + next * 2 + intended] +=
                            bernoulli(DEMAND, u2) * bernoulli(ERROR, u1_next);
                    }
                }
            }
        }
    }
    CounterfactualModel::new(2, 2, rewards, transitions).unwrap()
}

// Long enough at discount 0.9 for the truncated tail, 9 * 0.9^400, to vanish below 1e-15.
const LONG_HORIZON: usize = 400;

fn discounted_action_values(model: &CounterfactualModel, policy: Option<&[usize]>) -> Vec<f64> {
    let process = model.decision_process(0.9).unwrap();
    let terminal = vec![0.0; 4];
    let first_stage = match policy {
        None => finite_horizon(&process, LONG_HORIZON, &terminal).unwrap().values.remove(0),
        Some(rule) => evaluate_finite_policy(&process, &vec![rule.to_vec(); LONG_HORIZON], &terminal)
            .unwrap()
            .remove(0),
    };
    action_values(&process, &first_stage).unwrap()
}

#[test]
fn switching_is_worth_nine_and_following_eight_point_two_at_discount_point_nine() {
    let model = inventory_model();
    let optimal = discounted_action_values(&model, None);
    let follow: Vec<usize> = (0..4).map(|pair| pair % 2).collect();
    let switch: Vec<usize> = (0..4).map(|pair| 1 - pair % 2).collect();
    let always_follow = discounted_action_values(&model, Some(&follow));
    let always_switch = discounted_action_values(&model, Some(&switch));
    for s in 0..2 {
        for x in 0..2 {
            assert!((optimal[cell(s, x, 1 - x)] - 9.0).abs() < 1e-9);
            assert!((optimal[cell(s, x, x)] - 8.2).abs() < 1e-9);
            assert!((always_follow[cell(s, x, x)] - 1.0).abs() < 1e-9);
            assert!((always_switch[cell(s, x, 1 - x)] - 9.0).abs() < 1e-9);
        }
    }
}

struct Inventory {
    rng: NpRng,
}

impl Inventory {
    fn coin(&mut self, p: f64) -> usize {
        usize::from(self.rng.next_f64() < p)
    }

    fn episode(&mut self, horizon: usize, choose: &mut dyn FnMut(usize, usize, usize) -> usize) -> Episode {
        let mut state = self.coin(ERROR) ^ self.coin(DEMAND);
        let mut steps = Vec::with_capacity(horizon);
        for step in 0..horizon {
            let (error, demand, value) = (self.coin(ERROR), self.coin(DEMAND), self.coin(VALUE));
            let intended = state ^ error;
            let played = choose(step, state, intended);
            let reward = (state ^ played ^ error ^ value) as f64;
            steps.push(EpisodeStep { state, intended, played, reward });
            state = (state | played) ^ error ^ demand;
        }
        let final_intended = state ^ self.coin(ERROR);
        Episode { steps, final_state: state, final_intended }
    }
}

#[test]
fn the_learner_stays_optimistic_capped_and_monotone_and_learns_to_switch() {
    let (horizon, episodes) = (3, 200_000);
    let process = inventory_model().decision_process(1.0).unwrap();
    let stages = finite_horizon(&process, horizon, &[0.0; 4]).unwrap().values;
    let optimal: Vec<f64> = (0..horizon)
        .flat_map(|step| action_values(&process, &stages[step + 1]).unwrap())
        .collect();
    let mut learner = CounterfactualUcbvi::new(2, 2, horizon, episodes, 0.1).unwrap();
    let mut inventory = Inventory { rng: NpRng::seeded(1) };
    let mut previous: Vec<f64> = (0..horizon).flat_map(|h| learner.action_values(h).to_vec()).collect();
    let mut late_reward = 0.0;
    let late_start = episodes - episodes / 10;
    for e in 0..episodes {
        learner.plan();
        let current: Vec<f64> = (0..horizon).flat_map(|h| learner.action_values(h).to_vec()).collect();
        for (i, value) in current.iter().enumerate() {
            assert!(*value <= horizon as f64 && *value <= previous[i]);
            assert!(*value >= optimal[i] - 1e-12, "optimism lost at episode {e}, entry {i}");
        }
        previous = current;
        let episode = inventory.episode(horizon, &mut |step, state, intended| learner.act(step, state, intended));
        if e >= late_start {
            late_reward += episode.steps.iter().map(|step| step.reward).sum::<f64>();
        }
        learner.record_episode(&episode).unwrap();
    }
    learner.plan();
    for step in 0..horizon {
        let values = learner.action_values(step);
        for s in 0..2 {
            for x in 0..2 {
                let (follow, switch) = (values[cell(s, x, x)], values[cell(s, x, 1 - x)]);
                assert!(follow == switch || switch > follow, "step {step}, state {s}, intention {x}");
            }
        }
    }
    let per_step = late_reward / ((episodes - late_start) * horizon) as f64;
    assert!(per_step > 0.85, "late reward per step {per_step}");
}
