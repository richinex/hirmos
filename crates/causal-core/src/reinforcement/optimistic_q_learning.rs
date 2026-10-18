//! Model-free optimistic Q-learning.

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum OptimisticQError {
    EmptySpace { states: usize, actions: usize },
    InvalidDelta(f64),
}

impl fmt::Display for OptimisticQError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySpace { states, actions } => {
                write!(f, "{states} states and {actions} actions leave nothing to learn")
            }
            Self::InvalidDelta(value) => write!(f, "confidence parameter {value} is outside (0, 1)"),
        }
    }
}

impl std::error::Error for OptimisticQError {}

/// Model-free optimistic Q-learning over (state, action).
#[derive(Clone, Debug)]
pub struct UcbQ {
    actions: usize,
    log_inv_delta: f64,
    values: Vec<f64>,
    visits: Vec<f64>,
}

impl UcbQ {
    pub fn new(states: usize, actions: usize, delta: f64) -> Result<Self, OptimisticQError> {
        if states == 0 || actions == 0 {
            return Err(OptimisticQError::EmptySpace { states, actions });
        }
        if !(delta > 0.0 && delta < 1.0) {
            return Err(OptimisticQError::InvalidDelta(delta));
        }
        Ok(Self {
            actions,
            log_inv_delta: (1.0 / delta).ln(),
            values: vec![0.0; states * actions],
            visits: vec![1.0; states * actions],
        })
    }

    pub fn action_values(&self) -> &[f64] {
        &self.values
    }

    pub fn visits(&self) -> &[f64] {
        &self.visits
    }

    /// The action with the highest value in `state`, the first one on ties.
    pub fn act(&self, state: usize) -> usize {
        let row = &self.values[state * self.actions..(state + 1) * self.actions];
        let mut best = 0;
        for (action, value) in row.iter().enumerate().skip(1) {
            if *value > row[best] {
                best = action;
            }
        }
        best
    }

    /// Move the action value towards reward + best next value + bonus with step 1 / visits.
    pub fn update(&mut self, state: usize, action: usize, reward: f64, next: usize) {
        let cell = state * self.actions + action;
        self.visits[cell] += 1.0;
        let rate = 1.0 / self.visits[cell];
        let bonus = (2.0 * self.log_inv_delta / self.visits[cell]).sqrt();
        let best_next = self.values[next * self.actions..(next + 1) * self.actions]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let target = reward + best_next + bonus;
        self.values[cell] = (1.0 - rate) * self.values[cell] + rate * target;
    }
}
