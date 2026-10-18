//! Reward shaping whose potential is an upper bound on optimal values learned from confounded logs.

use crate::nprandom::NpRng;
use crate::reinforcement::windy_grid::round;

/// One transition of an episodic environment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Step {
    pub next: usize,
    pub reward: f64,
    pub terminated: bool,
    pub truncated: bool,
}

/// An environment whose states and actions are numbered from zero.
pub trait EpisodicEnvironment {
    fn reset(&mut self, seed: u64) -> usize;
    fn step(&mut self, action: usize) -> Step;
}

/// One logged transition; the action was chosen by a behaviour policy that may have seen more than the state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoggedStep {
    pub state: usize,
    pub action: usize,
    pub reward: f64,
    pub next: usize,
}

/// An upper bound on each state's optimal value, rounded to three decimals.
///
/// Actions the log rarely took in a state are credited with `reward_upper_bound` per step;
/// states never logged and states where the agent cannot act keep zero.
pub fn optimal_value_upper_bound(
    log: &[LoggedStep],
    actionable: &[bool],
    actions: usize,
    horizon: usize,
    reward_upper_bound: f64,
) -> Vec<f64> {
    let states = actionable.len();
    let mut state_count = vec![0usize; states];
    let mut pair_count = vec![0usize; states * actions];
    let mut reward_total = vec![0.0; states * actions];
    let mut transitions = vec![0usize; states * actions * states];
    for step in log {
        let pair = step.state * actions + step.action;
        state_count[step.state] += 1;
        pair_count[pair] += 1;
        reward_total[pair] += step.reward;
        transitions[pair * states + step.next] += 1;
    }
    let support: Vec<usize> = (0..states).filter(|&s| state_count[s] > 0).collect();
    let propensity: Vec<f64> = (0..states * actions)
        .map(|pair| match state_count[pair / actions] {
            0 => 1.0 / actions as f64,
            seen => pair_count[pair] as f64 / seen as f64,
        })
        .collect();

    let mut values = vec![0.0; states];
    let mut previous = vec![0.0; states];
    let mut action_values = vec![0.0; states * actions];
    let mut continuation = reward_upper_bound;
    loop {
        for state in 0..states {
            if !actionable[state] || state_count[state] == 0 {
                continue;
            }
            for action in 0..actions {
                let pair = state * actions + action;
                if pair_count[pair] == 0 {
                    continue;
                }
                let reward = reward_total[pair] / pair_count[pair] as f64;
                let mut next_value = 0.0;
                for next in 0..states {
                    if !actionable[next] || state_count[next] == 0 {
                        continue;
                    }
                    let probability = transitions[pair * states + next] as f64 / pair_count[pair] as f64;
                    if probability > 0.0 {
                        next_value += probability * previous[next];
                    }
                }
                action_values[pair] = propensity[pair] * (reward + next_value)
                    + (1.0 - propensity[pair]) * (reward_upper_bound + continuation);
            }
            values[state] = (0..actions)
                .filter(|&action| pair_count[state * actions + action] > 0)
                .map(|action| action_values[state * actions + action])
                .fold(f64::NEG_INFINITY, f64::max);
        }
        if values.iter().zip(&previous).all(|(a, b)| (a - b).abs() < 1e-3) {
            break;
        }
        previous.copy_from_slice(&values);
        let best = support.iter().map(|&s| previous[s]).fold(f64::NEG_INFINITY, f64::max);
        continuation = (reward_upper_bound * (horizon - 1) as f64).min(best);
    }
    values.iter().map(|value| round(*value, 3)).collect()
}

/// How the learner uses the upper bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shaping {
    /// Add the bound, rounded to one decimal, as a potential: Y + phi(s') - phi(s).
    pub potential: bool,
    /// Cap state values and scale the bonus by the bound instead of the horizon.
    pub clip_to_bound: bool,
}

/// What an online run produced.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapedRun {
    pub action_values: Vec<f64>,
    pub visits: Vec<f64>,
    pub rewards: Vec<f64>,
    /// Every (state, action) pair played, in order.
    pub steps: Vec<(usize, usize)>,
    /// Where each episode's steps end in `steps`.
    pub episode_ends: Vec<usize>,
}

impl ShapedRun {
    /// Regret summed over every step up to the end of each episode, each step's gap rounded to four decimals.
    pub fn cumulative_regret(&self, optimal_values: &[f64], optimal_action_values: &[f64], actions: usize) -> Vec<f64> {
        let mut total = 0.0;
        let mut start = 0;
        self.episode_ends
            .iter()
            .map(|&end| {
                for &(state, action) in &self.steps[start..end] {
                    total += round(optimal_values[state] - optimal_action_values[state * actions + action], 4);
                }
                start = end;
                total
            })
            .collect()
    }
}

/// Optimistic Q-learning with step size (H + 1) / (H + n) over `episodes` episodes of at most `horizon` steps.
///
/// Each episode resets the environment with a seed drawn from a generator seeded with `seed`;
/// action values are rounded to `precision` decimals after every update.
pub fn shaped_q_ucb(
    environment: &mut impl EpisodicEnvironment,
    upper_bound: &[f64],
    actions: usize,
    horizon: usize,
    episodes: usize,
    seed: u64,
    precision: i32,
    shaping: Shaping,
) -> ShapedRun {
    let states = upper_bound.len();
    let start = if shaping.potential { 0.0 } else { horizon as f64 };
    let mut action_values = vec![start; states * actions];
    let mut values = vec![start; states];
    let mut visits = vec![0.0; states * actions];
    let largest = upper_bound.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let scale = if shaping.clip_to_bound { horizon as f64 * largest.powi(2) } else { (horizon * horizon * horizon) as f64 };
    let log_term = ((states * actions * horizon * episodes * 10) as f64).ln();
    let potential = |state: usize| if shaping.potential { round(upper_bound[state], 1) } else { 0.0 };
    let mut seeds = NpRng::seeded(seed);
    let mut rewards = Vec::with_capacity(episodes);
    let mut steps = Vec::with_capacity(episodes * horizon);
    let mut episode_ends = Vec::with_capacity(episodes);
    for _ in 0..episodes {
        let mut state = environment.reset(seeds.bounded_uint64(10000));
        let mut total = 0.0;
        let start = steps.len();
        loop {
            let row = &action_values[state * actions..(state + 1) * actions];
            let action = (1..actions).fold(0, |best, a| if row[a] > row[best] { a } else { best });
            let step = environment.step(action);
            let cell = state * actions + action;
            visits[cell] += 1.0;
            let bonus = 4.0 * (scale * log_term / visits[cell]).sqrt();
            let rate = (horizon + 1) as f64 / (horizon as f64 + visits[cell]);
            let done = step.terminated || step.truncated;
            let shaped = if done {
                step.reward - potential(state)
            } else {
                step.reward + potential(step.next) - potential(state)
            };
            let next_value = if steps.len() - start + 1 < horizon { values[step.next] } else { 0.0 };
            action_values[cell] = round((1.0 - rate) * action_values[cell] + rate * (shaped + next_value + bonus), precision);
            let cap = if shaping.clip_to_bound { upper_bound[state] } else { horizon as f64 };
            let best = action_values[state * actions..(state + 1) * actions].iter().copied().fold(f64::NEG_INFINITY, f64::max);
            values[state] = best.min(cap);
            total += step.reward;
            steps.push((state, action));
            state = step.next;
            if done {
                break;
            }
        }
        rewards.push(total);
        episode_ends.push(steps.len());
    }
    ShapedRun { action_values, visits, rewards, steps, episode_ends }
}
