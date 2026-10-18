//! Finite tabular decision processes whose transitions are interventional, P(S_next | S, do(A)).

use crate::discrete_bn::{DiscreteBn, DiscreteBnError, Intervention};
use crate::nprandom::Mt19937;
use std::collections::HashMap;

const TRANSITION_SUM_TOLERANCE: f64 = 1e-12;

#[derive(Clone, Debug, PartialEq)]
pub struct TabularMdp {
    states: usize,
    actions: usize,
    transitions: Vec<f64>,
    rewards: RewardModel,
    discount: f64,
}

#[derive(Clone, Debug, PartialEq)]
enum RewardModel {
    StateAction(Vec<f64>),
    Transition(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DecisionProcessError {
    Network(DiscreteBnError),
    TwoSliceVariablesMustBeDistinct,
    StateLabelsDiffer {
        current: Vec<String>,
        next: Vec<String>,
    },
    NoStates,
    NoActions,
    TransitionLength {
        expected: usize,
        actual: usize,
    },
    RewardLength {
        expected: usize,
        actual: usize,
    },
    InvalidTransition {
        action: usize,
        state: usize,
        next_state: usize,
        value: f64,
    },
    TransitionRowNotNormalized {
        action: usize,
        state: usize,
        sum: f64,
    },
    InvalidReward {
        state: usize,
        action: usize,
        value: f64,
    },
    InvalidDiscount(f64),
    InvalidValueLength {
        expected: usize,
        actual: usize,
    },
    InvalidEpsilon(f64),
    InvalidMaximumIterations(usize),
    TooFewLearningIterations(usize),
    InvalidHorizon,
    InvalidValue {
        state: usize,
        value: f64,
    },
    InvalidPolicyLength {
        stage: usize,
        expected: usize,
        actual: usize,
    },
    InvalidPolicyAction {
        stage: usize,
        state: usize,
        action: usize,
    },
    NonFinitePolicyValue {
        stage: usize,
        state: usize,
    },
}

impl std::fmt::Display for DecisionProcessError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for DecisionProcessError {}

impl From<DiscreteBnError> for DecisionProcessError {
    fn from(error: DiscreteBnError) -> Self {
        Self::Network(error)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterventionalMdp {
    pub model: TabularMdp,
    pub state_labels: Vec<String>,
    pub action_labels: Vec<String>,
    pub current_state_variable: String,
    pub action_variable: String,
    pub next_state_variable: String,
}

/// Construct a finite MDP by graph surgery on a two-slice causal model.
pub fn interventional_mdp_from_discrete_bn(
    network: &DiscreteBn,
    current_state_variable: &str,
    action_variable: &str,
    next_state_variable: &str,
    rewards: Vec<f64>,
    discount: f64,
) -> Result<InterventionalMdp, DecisionProcessError> {
    if current_state_variable == action_variable
        || current_state_variable == next_state_variable
        || action_variable == next_state_variable
    {
        return Err(DecisionProcessError::TwoSliceVariablesMustBeDistinct);
    }

    let states = network
        .state_names
        .get(current_state_variable)
        .ok_or_else(|| DiscreteBnError::UnknownVariable(current_state_variable.to_string()))?
        .clone();
    let next_states = network
        .state_names
        .get(next_state_variable)
        .ok_or_else(|| DiscreteBnError::UnknownVariable(next_state_variable.to_string()))?
        .clone();
    if states != next_states {
        return Err(DecisionProcessError::StateLabelsDiffer {
            current: states,
            next: next_states,
        });
    }
    let actions = network
        .state_names
        .get(action_variable)
        .ok_or_else(|| DiscreteBnError::UnknownVariable(action_variable.to_string()))?
        .clone();

    let mut transitions = vec![0.0; actions.len() * states.len() * states.len()];
    for (action, action_label) in actions.iter().enumerate() {
        let intervention = Intervention {
            variable: action_variable.to_string(),
            state: action_label.clone(),
        };
        for (state, state_label) in states.iter().enumerate() {
            let evidence =
                HashMap::from([(current_state_variable.to_string(), state_label.clone())]);
            let distribution = network.query_under_intervention(
                &[next_state_variable.to_string()],
                std::slice::from_ref(&intervention),
                &evidence,
            )?;
            for (assignment, probability) in
                distribution.states.iter().zip(&distribution.probabilities)
            {
                let next_state = states
                    .iter()
                    .position(|label| label == &assignment[0])
                    .expect("query states were checked against the model state labels");
                transitions[(action * states.len() + state) * states.len() + next_state] =
                    *probability;
            }
        }
    }

    let model = TabularMdp::new(states.len(), actions.len(), transitions, rewards, discount)?;
    Ok(InterventionalMdp {
        model,
        state_labels: states,
        action_labels: actions,
        current_state_variable: current_state_variable.to_string(),
        action_variable: action_variable.to_string(),
        next_state_variable: next_state_variable.to_string(),
    })
}

impl TabularMdp {
    /// Construct an MDP from action-major transition rows and state-major rewards.
    pub fn new(
        states: usize,
        actions: usize,
        transitions: Vec<f64>,
        rewards: Vec<f64>,
        discount: f64,
    ) -> Result<Self, DecisionProcessError> {
        if states == 0 {
            return Err(DecisionProcessError::NoStates);
        }
        if actions == 0 {
            return Err(DecisionProcessError::NoActions);
        }

        let expected_transitions = actions * states * states;
        if transitions.len() != expected_transitions {
            return Err(DecisionProcessError::TransitionLength {
                expected: expected_transitions,
                actual: transitions.len(),
            });
        }
        validate_reward_length(states * actions, rewards.len())?;
        if !discount.is_finite() || !(0.0 < discount && discount <= 1.0) {
            return Err(DecisionProcessError::InvalidDiscount(discount));
        }

        for action in 0..actions {
            for state in 0..states {
                let mut sum = 0.0;
                for next_state in 0..states {
                    let value = transitions[(action * states + state) * states + next_state];
                    if !value.is_finite() || value < 0.0 {
                        return Err(DecisionProcessError::InvalidTransition {
                            action,
                            state,
                            next_state,
                            value,
                        });
                    }
                    sum += value;
                }
                if (sum - 1.0).abs() > TRANSITION_SUM_TOLERANCE {
                    return Err(DecisionProcessError::TransitionRowNotNormalized { action, state, sum });
                }
            }
        }

        validate_state_action_rewards(states, actions, &rewards)?;

        Ok(Self {
            states,
            actions,
            transitions,
            rewards: RewardModel::StateAction(rewards),
            discount,
        })
    }

    /// Construct an MDP with a reward for each `(action, state, next_state)`.
    pub fn with_transition_rewards(
        states: usize,
        actions: usize,
        transitions: Vec<f64>,
        rewards: Vec<f64>,
        discount: f64,
    ) -> Result<Self, DecisionProcessError> {
        let mut model = Self::new(
            states,
            actions,
            transitions,
            vec![0.0; states * actions],
            discount,
        )?;
        validate_reward_length(actions * states * states, rewards.len())?;
        for action in 0..actions {
            for state in 0..states {
                for next_state in 0..states {
                    let value = rewards[(action * states + state) * states + next_state];
                    if !value.is_finite() {
                        return Err(DecisionProcessError::InvalidReward {
                            state,
                            action,
                            value,
                        });
                    }
                }
            }
        }
        model.rewards = RewardModel::Transition(rewards);
        Ok(model)
    }

    pub fn states(&self) -> usize {
        self.states
    }

    pub fn actions(&self) -> usize {
        self.actions
    }

    pub fn discount(&self) -> f64 {
        self.discount
    }

    pub fn transition(&self, action: usize, state: usize, next_state: usize) -> f64 {
        self.transitions[(action * self.states + state) * self.states + next_state]
    }

    pub fn reward(&self, state: usize, action: usize) -> f64 {
        match &self.rewards {
            RewardModel::StateAction(rewards) => rewards[state * self.actions + action],
            RewardModel::Transition(rewards) => (0..self.states)
                .map(|next_state| {
                    self.transition(action, state, next_state)
                        * rewards[(action * self.states + state) * self.states + next_state]
                })
                .sum(),
        }
    }

    fn realized_reward(&self, state: usize, action: usize, next_state: usize) -> f64 {
        match &self.rewards {
            RewardModel::StateAction(rewards) => rewards[state * self.actions + action],
            RewardModel::Transition(rewards) => {
                rewards[(action * self.states + state) * self.states + next_state]
            }
        }
    }
}

fn validate_reward_length(expected: usize, actual: usize) -> Result<(), DecisionProcessError> {
    if actual == expected {
        Ok(())
    } else {
        Err(DecisionProcessError::RewardLength { expected, actual })
    }
}

fn validate_state_action_rewards(
    states: usize,
    actions: usize,
    rewards: &[f64],
) -> Result<(), DecisionProcessError> {
    for state in 0..states {
        for action in 0..actions {
            let value = rewards[state * actions + action];
            if !value.is_finite() {
                return Err(DecisionProcessError::InvalidReward {
                    state,
                    action,
                    value,
                });
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub struct BellmanResult {
    pub policy: Vec<usize>,
    pub value: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FiniteHorizonResult {
    /// Stage-major values, including the terminal stage.
    pub values: Vec<Vec<f64>>,
    /// One action per state at each nonterminal stage.
    pub policy: Vec<Vec<usize>>,
}

fn validate_terminal(mdp: &TabularMdp, terminal: &[f64]) -> Result<(), DecisionProcessError> {
    if terminal.len() != mdp.states {
        return Err(DecisionProcessError::InvalidValueLength {
            expected: mdp.states,
            actual: terminal.len(),
        });
    }
    for (state, &value) in terminal.iter().enumerate() {
        if !value.is_finite() {
            return Err(DecisionProcessError::InvalidValue { state, value });
        }
    }
    Ok(())
}

/// Backward induction with an explicit terminal reward.
pub fn finite_horizon(
    mdp: &TabularMdp,
    horizon: usize,
    terminal: &[f64],
) -> Result<FiniteHorizonResult, DecisionProcessError> {
    if horizon == 0 || horizon == usize::MAX {
        return Err(DecisionProcessError::InvalidHorizon);
    }
    validate_terminal(mdp, terminal)?;
    let mut values = vec![vec![0.0; mdp.states]; horizon + 1];
    let mut policy = vec![vec![0; mdp.states]; horizon];
    values[horizon].copy_from_slice(terminal);
    for stage in (0..horizon).rev() {
        let step = bellman_operator(mdp, &values[stage + 1])?;
        for (state, value) in step.value.iter().enumerate() {
            if !value.is_finite() {
                return Err(DecisionProcessError::NonFinitePolicyValue { stage, state });
            }
        }
        values[stage] = step.value;
        policy[stage] = step.policy;
    }
    Ok(FiniteHorizonResult { values, policy })
}

/// Evaluate a specified stage-dependent deterministic policy in the supplied model.
pub fn evaluate_finite_policy(
    mdp: &TabularMdp,
    policy: &[Vec<usize>],
    terminal: &[f64],
) -> Result<Vec<Vec<f64>>, DecisionProcessError> {
    if policy.is_empty() {
        return Err(DecisionProcessError::InvalidHorizon);
    }
    validate_terminal(mdp, terminal)?;
    for (stage, actions) in policy.iter().enumerate() {
        if actions.len() != mdp.states {
            return Err(DecisionProcessError::InvalidPolicyLength {
                stage,
                expected: mdp.states,
                actual: actions.len(),
            });
        }
        for (state, &action) in actions.iter().enumerate() {
            if action >= mdp.actions {
                return Err(DecisionProcessError::InvalidPolicyAction {
                    stage,
                    state,
                    action,
                });
            }
        }
    }
    let mut values = vec![vec![0.0; mdp.states]; policy.len() + 1];
    values[policy.len()].copy_from_slice(terminal);
    for stage in (0..policy.len()).rev() {
        for state in 0..mdp.states {
            let value = action_value(mdp, state, policy[stage][state], &values[stage + 1]);
            if !value.is_finite() {
                return Err(DecisionProcessError::NonFinitePolicyValue { stage, state });
            }
            values[stage][state] = value;
        }
    }
    Ok(values)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValueIterationStep {
    pub iteration: usize,
    pub policy: Vec<usize>,
    pub value: Vec<f64>,
    pub variation: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValueIterationResult {
    pub policy: Vec<usize>,
    pub value: Vec<f64>,
    pub iterations: usize,
    pub computed_max_iterations: usize,
    pub threshold: f64,
    pub trace: Vec<ValueIterationStep>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QUpdateCheckpoint {
    pub iteration: usize,
    pub state: usize,
    pub action: usize,
    pub next_state: usize,
    pub reward: f64,
    pub delta: f64,
    pub update: f64,
    pub updated_value: f64,
    pub q: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QLearningResult {
    pub initial_state: usize,
    pub q: Vec<f64>,
    pub value: Vec<f64>,
    pub policy: Vec<usize>,
    pub mean_discrepancy: Vec<f64>,
    pub checkpoints: Vec<QUpdateCheckpoint>,
}

fn first_argmax(values: &[f64]) -> (usize, f64) {
    let mut index = 0;
    let mut maximum = values[0];
    for (candidate, &value) in values.iter().enumerate().skip(1) {
        if value > maximum {
            index = candidate;
            maximum = value;
        }
    }
    (index, maximum)
}

fn span(values: &[f64]) -> f64 {
    let mut minimum = values[0];
    let mut maximum = values[0];
    for &value in &values[1..] {
        minimum = minimum.min(value);
        maximum = maximum.max(value);
    }
    maximum - minimum
}

fn action_value(mdp: &TabularMdp, state: usize, action: usize, value: &[f64]) -> f64 {
    let mut continuation = 0.0;
    for (next_state, next_value) in value.iter().enumerate() {
        continuation += mdp.transition(action, state, next_state) * next_value;
    }
    mdp.reward(state, action) + mdp.discount * continuation
}

/// Action values Q(s, a) = R(s, a) + discount * sum over s' of P(s' | s, do(a)) V(s'), laid out
/// state by state.
pub fn action_values(mdp: &TabularMdp, value: &[f64]) -> Result<Vec<f64>, DecisionProcessError> {
    if value.len() != mdp.states {
        return Err(DecisionProcessError::InvalidValueLength {
            expected: mdp.states,
            actual: value.len(),
        });
    }
    Ok((0..mdp.states)
        .flat_map(|state| (0..mdp.actions).map(move |action| (state, action)))
        .map(|(state, action)| action_value(mdp, state, action, value))
        .collect())
}

pub fn bellman_operator(mdp: &TabularMdp, value: &[f64]) -> Result<BellmanResult, DecisionProcessError> {
    if value.len() != mdp.states {
        return Err(DecisionProcessError::InvalidValueLength {
            expected: mdp.states,
            actual: value.len(),
        });
    }

    let mut q = vec![0.0; mdp.actions * mdp.states];
    for action in 0..mdp.actions {
        for state in 0..mdp.states {
            q[action * mdp.states + state] = action_value(mdp, state, action, value);
        }
    }

    let mut policy = Vec::with_capacity(mdp.states);
    let mut updated = Vec::with_capacity(mdp.states);
    let mut action_values = vec![0.0; mdp.actions];
    for state in 0..mdp.states {
        for (action, slot) in action_values.iter_mut().enumerate() {
            *slot = q[action * mdp.states + state];
        }
        let (action, maximum) = first_argmax(&action_values);
        policy.push(action);
        updated.push(maximum);
    }
    Ok(BellmanResult {
        policy,
        value: updated,
    })
}

fn bounded_value_iterations(
    mdp: &TabularMdp,
    epsilon: f64,
    initial_value: &[f64],
) -> Result<usize, DecisionProcessError> {
    let mut column_minima = vec![0.0; mdp.states];
    for next_state in 0..mdp.states {
        let mut minimum = f64::INFINITY;
        for action in 0..mdp.actions {
            for state in 0..mdp.states {
                minimum = minimum.min(mdp.transition(action, state, next_state));
            }
        }
        column_minima[next_state] = minimum;
    }
    let k = 1.0 - column_minima.iter().sum::<f64>();
    let first = bellman_operator(mdp, initial_value)?;
    let differences = first
        .value
        .iter()
        .zip(initial_value)
        .map(|(next, previous)| next - previous)
        .collect::<Vec<_>>();
    let first_span = span(&differences);

    if first_span == 0.0 || mdp.discount * k == 0.0 {
        return Ok(1);
    }
    let numerator = epsilon * (1.0 - mdp.discount) / mdp.discount;
    let bound = (numerator / first_span).ln() / (mdp.discount * k).ln();
    Ok(bound.ceil().max(1.0) as usize)
}

pub fn value_iteration(
    mdp: &TabularMdp,
    epsilon: f64,
    max_iterations: usize,
    initial_value: Option<&[f64]>,
) -> Result<ValueIterationResult, DecisionProcessError> {
    if !epsilon.is_finite() || epsilon <= 0.0 {
        return Err(DecisionProcessError::InvalidEpsilon(epsilon));
    }
    if max_iterations == 0 {
        return Err(DecisionProcessError::InvalidMaximumIterations(max_iterations));
    }

    let mut value = match initial_value {
        Some(initial) if initial.len() != mdp.states => {
            return Err(DecisionProcessError::InvalidValueLength {
                expected: mdp.states,
                actual: initial.len(),
            })
        }
        Some(initial) => initial.to_vec(),
        None => vec![0.0; mdp.states],
    };
    let computed_max_iterations = if mdp.discount < 1.0 {
        bounded_value_iterations(mdp, epsilon, &value)?
    } else {
        max_iterations
    };
    let threshold = if mdp.discount < 1.0 {
        epsilon * (1.0 - mdp.discount) / mdp.discount
    } else {
        epsilon
    };

    let mut trace = Vec::new();
    let mut policy = vec![0; mdp.states];
    loop {
        let previous = value;
        let bellman = bellman_operator(mdp, &previous)?;
        let differences = bellman
            .value
            .iter()
            .zip(&previous)
            .map(|(next, old)| next - old)
            .collect::<Vec<_>>();
        let variation = span(&differences);
        value = bellman.value;
        policy = bellman.policy;
        trace.push(ValueIterationStep {
            iteration: trace.len() + 1,
            policy: policy.clone(),
            value: value.clone(),
            variation,
        });

        if variation < threshold || trace.len() == computed_max_iterations {
            break;
        }
    }

    Ok(ValueIterationResult {
        policy,
        value,
        iterations: trace.len(),
        computed_max_iterations,
        threshold,
        trace,
    })
}

pub fn q_learning(
    mdp: &TabularMdp,
    iterations: usize,
    seed: u32,
    checkpoint_iterations: &[usize],
) -> Result<QLearningResult, DecisionProcessError> {
    if iterations < 10_000 {
        return Err(DecisionProcessError::TooFewLearningIterations(iterations));
    }

    let mut rng = Mt19937::seeded(seed);
    let mut q = vec![0.0; mdp.states * mdp.actions];
    let mut discrepancies = Vec::with_capacity(100);
    let mut mean_discrepancy = Vec::with_capacity(iterations / 100);
    let mut checkpoints = Vec::new();
    let mut state = rng.randint(mdp.states as u64) as usize;
    let initial_state = state;

    for iteration in 1..=iterations {
        if iteration % 100 == 0 {
            state = rng.randint(mdp.states as u64) as usize;
        }

        let exploration_draw = rng.next_f64();
        let greedy_probability = 1.0 - 1.0 / ((iteration + 2) as f64).ln();
        let action = if exploration_draw < greedy_probability {
            first_argmax(&q[state * mdp.actions..(state + 1) * mdp.actions]).0
        } else {
            rng.randint(mdp.actions as u64) as usize
        };

        let transition_draw = rng.next_f64();
        let mut cumulative = 0.0;
        let mut sampled_state = -1_isize;
        while cumulative < transition_draw && sampled_state < mdp.states as isize - 1 {
            sampled_state += 1;
            cumulative += mdp.transition(action, state, sampled_state as usize);
        }
        // A draw of exactly zero selects no state, and the last state is used.
        let next_state = if sampled_state < 0 {
            mdp.states - 1
        } else {
            sampled_state as usize
        };

        let reward = mdp.realized_reward(state, action, next_state);
        let current = q[state * mdp.actions + action];
        let next_maximum =
            first_argmax(&q[next_state * mdp.actions..(next_state + 1) * mdp.actions]).1;
        let delta = reward + mdp.discount * next_maximum - current;
        let update = delta / ((iteration + 2) as f64).sqrt();
        let updated_value = current + update;
        q[state * mdp.actions + action] = updated_value;

        discrepancies.push(update.abs());
        if discrepancies.len() == 100 {
            mean_discrepancy.push(discrepancies.iter().sum::<f64>() / 100.0);
            discrepancies.clear();
        }

        if checkpoint_iterations.contains(&iteration) {
            checkpoints.push(QUpdateCheckpoint {
                iteration,
                state,
                action,
                next_state,
                reward,
                delta,
                update,
                updated_value,
                q: q.clone(),
            });
        }
        state = next_state;
    }

    let mut value = Vec::with_capacity(mdp.states);
    let mut policy = Vec::with_capacity(mdp.states);
    for state in 0..mdp.states {
        let (action, maximum) = first_argmax(&q[state * mdp.actions..(state + 1) * mdp.actions]);
        policy.push(action);
        value.push(maximum);
    }

    Ok(QLearningResult {
        initial_state,
        q,
        value,
        policy,
        mean_discrepancy,
        checkpoints,
    })
}
