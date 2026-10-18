//! Planning and learning a policy that also sees the agent's intended action.

use std::fmt;

use super::decision_process::{DecisionProcessError, TabularMdp};

#[derive(Clone, Debug, PartialEq)]
pub enum CounterfactualPlanningError {
    EmptySpace { states: usize, actions: usize },
    ShapeMismatch { expected: usize, found: usize },
    InvalidProbability(f64),
    RowDoesNotSumToOne { cell: usize, total: f64 },
    ZeroHorizon,
    ZeroEpisodes,
    InvalidDelta(f64),
    IndexOutOfRange { value: usize, limit: usize },
}

impl fmt::Display for CounterfactualPlanningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySpace { states, actions } => {
                write!(f, "{states} states and {actions} actions leave nothing to plan")
            }
            Self::ShapeMismatch { expected, found } => {
                write!(f, "expected {expected} entries, found {found}")
            }
            Self::InvalidProbability(p) => write!(f, "probability {p} is outside [0, 1]"),
            Self::RowDoesNotSumToOne { cell, total } => {
                write!(f, "transition row {cell} sums to {total}, not 1")
            }
            Self::ZeroHorizon => write!(f, "the horizon must be at least one step"),
            Self::ZeroEpisodes => write!(f, "the planned number of episodes must be positive"),
            Self::InvalidDelta(value) => write!(f, "confidence parameter {value} is outside (0, 1)"),
            Self::IndexOutOfRange { value, limit } => write!(f, "index {value} is outside 0..{limit}"),
        }
    }
}

impl std::error::Error for CounterfactualPlanningError {}

/// A known counterfactual model.
#[derive(Clone, Debug, PartialEq)]
pub struct CounterfactualModel {
    states: usize,
    actions: usize,
    rewards: Vec<f64>,
    transitions: Vec<f64>,
}

impl CounterfactualModel {
    pub fn new(
        states: usize,
        actions: usize,
        rewards: Vec<f64>,
        transitions: Vec<f64>,
    ) -> Result<Self, CounterfactualPlanningError> {
        if states == 0 || actions == 0 {
            return Err(CounterfactualPlanningError::EmptySpace { states, actions });
        }
        let cells = states * actions * actions;
        let row = states * actions;
        if rewards.len() != cells {
            return Err(CounterfactualPlanningError::ShapeMismatch { expected: cells, found: rewards.len() });
        }
        if transitions.len() != cells * row {
            return Err(CounterfactualPlanningError::ShapeMismatch {
                expected: cells * row,
                found: transitions.len(),
            });
        }
        if let Some(p) = transitions.iter().find(|p| !(0.0..=1.0).contains(*p)) {
            return Err(CounterfactualPlanningError::InvalidProbability(*p));
        }
        for cell in 0..cells {
            let total: f64 = transitions[cell * row..(cell + 1) * row].iter().sum();
            if (total - 1.0).abs() > 1e-12 {
                return Err(CounterfactualPlanningError::RowDoesNotSumToOne { cell, total });
            }
        }
        Ok(Self { states, actions, rewards, transitions })
    }

    pub fn states(&self) -> usize {
        self.states
    }

    pub fn actions(&self) -> usize {
        self.actions
    }

    pub fn rewards(&self) -> &[f64] {
        &self.rewards
    }

    pub fn transitions(&self) -> &[f64] {
        &self.transitions
    }

    /// The model as a decision process whose states are (state, intention) pairs, numbered state *
    /// actions + intention.
    pub fn decision_process(&self, discount: f64) -> Result<TabularMdp, DecisionProcessError> {
        let pairs = self.states * self.actions;
        let mut transitions = vec![0.0; self.actions * pairs * pairs];
        let mut rewards = vec![0.0; pairs * self.actions];
        for pair in 0..pairs {
            for action in 0..self.actions {
                let cell = pair * self.actions + action;
                rewards[pair * self.actions + action] = self.rewards[cell];
                let row = &self.transitions[cell * pairs..(cell + 1) * pairs];
                transitions[(action * pairs + pair) * pairs..(action * pairs + pair + 1) * pairs].copy_from_slice(row);
            }
        }
        TabularMdp::new(pairs, self.actions, transitions, rewards, discount)
    }

}

/// One step of an episode: the state, the intention, the action played and its reward.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EpisodeStep {
    pub state: usize,
    pub intended: usize,
    pub played: usize,
    pub reward: f64,
}

/// A finished episode: its steps and the state and intention that follow the last step.
#[derive(Clone, Debug, PartialEq)]
pub struct Episode {
    pub steps: Vec<EpisodeStep>,
    pub final_state: usize,
    pub final_intended: usize,
}

/// Optimistic learning of a counterfactual policy over a fixed horizon.
#[derive(Clone, Debug)]
pub struct CounterfactualUcbvi {
    states: usize,
    actions: usize,
    horizon: usize,
    bonus_scale: f64,
    visits: Vec<u64>,
    reward_sums: Vec<f64>,
    transition_counts: Vec<u64>,
    values: Vec<f64>,
}

impl CounterfactualUcbvi {
    /// Create a learner for `episodes` episodes of `horizon` steps with confidence `delta`.
    pub fn new(
        states: usize,
        actions: usize,
        horizon: usize,
        episodes: usize,
        delta: f64,
    ) -> Result<Self, CounterfactualPlanningError> {
        if states == 0 || actions == 0 {
            return Err(CounterfactualPlanningError::EmptySpace { states, actions });
        }
        if horizon == 0 {
            return Err(CounterfactualPlanningError::ZeroHorizon);
        }
        if episodes == 0 {
            return Err(CounterfactualPlanningError::ZeroEpisodes);
        }
        if !(delta > 0.0 && delta < 1.0) {
            return Err(CounterfactualPlanningError::InvalidDelta(delta));
        }
        let cells = states * actions * actions;
        let height = horizon as f64;
        let log_term = (5.0 * cells as f64 * episodes as f64 / delta).ln();
        Ok(Self {
            states,
            actions,
            horizon,
            bonus_scale: 7.0 * height * log_term.sqrt(),
            visits: vec![0; cells],
            reward_sums: vec![0.0; cells],
            transition_counts: vec![0; cells * states * actions],
            values: vec![height; horizon * cells],
        })
    }

    fn cell(&self, state: usize, intended: usize, played: usize) -> usize {
        (state * self.actions + intended) * self.actions + played
    }

    fn check(&self, step: &EpisodeStep) -> Result<(), CounterfactualPlanningError> {
        for (value, limit) in [(step.state, self.states), (step.intended, self.actions), (step.played, self.actions)] {
            if value >= limit {
                return Err(CounterfactualPlanningError::IndexOutOfRange { value, limit });
            }
        }
        Ok(())
    }

    /// Action values for step `step` of the horizon, indexed by state, intention and action.
    pub fn action_values(&self, step: usize) -> &[f64] {
        let cells = self.visits.len();
        &self.values[step * cells..(step + 1) * cells]
    }

    pub fn visits(&self) -> &[u64] {
        &self.visits
    }

    /// Add one episode to the counts, every step with its successor.
    pub fn record_episode(&mut self, episode: &Episode) -> Result<(), CounterfactualPlanningError> {
        for step in &episode.steps {
            self.check(step)?;
        }
        for (value, limit) in [(episode.final_state, self.states), (episode.final_intended, self.actions)] {
            if value >= limit {
                return Err(CounterfactualPlanningError::IndexOutOfRange { value, limit });
            }
        }
        let row = self.states * self.actions;
        for (i, step) in episode.steps.iter().enumerate() {
            let cell = self.cell(step.state, step.intended, step.played);
            self.visits[cell] += 1;
            self.reward_sums[cell] += step.reward;
            let (next_state, next_intended) = match episode.steps.get(i + 1) {
                Some(next) => (next.state, next.intended),
                None => (episode.final_state, episode.final_intended),
            };
            self.transition_counts[cell * row + next_state * self.actions + next_intended] += 1;
        }
        Ok(())
    }

    /// Recompute the optimistic values from all recorded episodes.
    pub fn plan(&mut self) {
        let cells = self.visits.len();
        let row = self.states * self.actions;
        let height = self.horizon as f64;
        let mut next_values = vec![0.0; row];
        for step in (0..self.horizon).rev() {
            for cell in 0..cells {
                let visits = self.visits[cell];
                if visits == 0 {
                    continue;
                }
                let n = visits as f64;
                let counts = &self.transition_counts[cell * row..(cell + 1) * row];
                let expected: f64 =
                    counts.iter().zip(&next_values).map(|(count, v)| *count as f64 * v).sum::<f64>() / n;
                let estimate = self.reward_sums[cell] / n + expected + self.bonus_scale / n.sqrt();
                let slot = &mut self.values[step * cells + cell];
                *slot = slot.min(height).min(estimate);
            }
            let layer = &self.values[step * cells..(step + 1) * cells];
            next_values = layer
                .chunks(self.actions)
                .map(|choices| choices.iter().copied().fold(f64::NEG_INFINITY, f64::max))
                .collect();
        }
    }

    /// The action with the highest optimistic value at step `step`, the first one on ties.
    pub fn act(&self, step: usize, state: usize, intended: usize) -> usize {
        let start = self.cell(state, intended, 0);
        let row = &self.action_values(step)[start..start + self.actions];
        let mut best = 0;
        for (action, value) in row.iter().enumerate().skip(1) {
            if *value > row[best] {
                best = action;
            }
        }
        best
    }
}
