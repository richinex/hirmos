//! Policies that consult a person's intended action in at most a given discounted share of the time.

use std::fmt;

use crate::reinforcement::counterfactual_planning::CounterfactualModel;
use crate::simplex::{solve, Lp};

#[derive(Clone, Debug, PartialEq)]
pub enum HybridError {
    ShapeMismatch { expected: usize, found: usize },
    NotADistribution { row: usize, total: f64 },
    IntentionNotFresh { cell: usize },
    InvalidDiscount(f64),
    InvalidBudget(f64),
    NoSolution,
}

impl fmt::Display for HybridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShapeMismatch { expected, found } => write!(f, "expected {expected} entries, found {found}"),
            Self::NotADistribution { row, total } => write!(f, "row {row} sums to {total}, not 1"),
            Self::IntentionNotFresh { cell } => {
                write!(f, "transition row {cell} makes the next intention depend on more than the next state")
            }
            Self::InvalidDiscount(value) => write!(f, "discount {value} is outside [0, 1)"),
            Self::InvalidBudget(value) => write!(f, "budget {value} is outside [0, 1]"),
            Self::NoSolution => write!(f, "the linear program has no solution"),
        }
    }
}

impl std::error::Error for HybridError {}

/// A counterfactual model whose next intention depends only on the next state, through P(x | s).
#[derive(Clone, Debug, PartialEq)]
pub struct FreshIntentionModel {
    states: usize,
    actions: usize,
    rewards: Vec<f64>,
    next_states: Vec<f64>,
    intention: Vec<f64>,
}

impl FreshIntentionModel {
    pub fn new(model: &CounterfactualModel, intention: Vec<f64>) -> Result<Self, HybridError> {
        let (states, actions) = (model.states(), model.actions());
        if intention.len() != states * actions {
            return Err(HybridError::ShapeMismatch { expected: states * actions, found: intention.len() });
        }
        for row in 0..states {
            let total: f64 = intention[row * actions..(row + 1) * actions].iter().sum();
            if (total - 1.0).abs() > 1e-12 {
                return Err(HybridError::NotADistribution { row, total });
            }
        }
        let pairs = states * actions;
        let mut next_states = Vec::with_capacity(pairs * actions * states);
        for (cell, row) in model.transitions().chunks(pairs).enumerate() {
            for next in 0..states {
                let reached: f64 = row[next * actions..(next + 1) * actions].iter().sum();
                let fresh = (0..actions)
                    .all(|x| (row[next * actions + x] - reached * intention[next * actions + x]).abs() <= 1e-12);
                if !fresh {
                    return Err(HybridError::IntentionNotFresh { cell });
                }
                next_states.push(reached);
            }
        }
        Ok(Self { states, actions, rewards: model.rewards().to_vec(), next_states, intention })
    }
}

/// A stationary hybrid policy and what it achieves from the initial distribution.
#[derive(Clone, Debug, PartialEq)]
pub struct HybridPolicy {
    /// P(consult | s).
    pub consult: Vec<f64>,
    /// P(x' | s) when acting alone, state-major.
    pub autonomous: Vec<f64>,
    /// P(x' | s, intended x) when consulting, indexed (s * actions + x) * actions + x'.
    pub assisted: Vec<f64>,
    pub value: f64,
    /// Discounted share of time spent consulting.
    pub consulted_share: f64,
}

/// The best hybrid policy whose discounted share of consulting is at most `budget`; unreached states never consult and act uniformly.
pub fn plan_hybrid(
    model: &FreshIntentionModel,
    initial: &[f64],
    discount: f64,
    budget: f64,
) -> Result<HybridPolicy, HybridError> {
    let (states, actions) = (model.states, model.actions);
    if initial.len() != states {
        return Err(HybridError::ShapeMismatch { expected: states, found: initial.len() });
    }
    let total: f64 = initial.iter().sum();
    if (total - 1.0).abs() > 1e-12 {
        return Err(HybridError::NotADistribution { row: 0, total });
    }
    if !(0.0..1.0).contains(&discount) {
        return Err(HybridError::InvalidDiscount(discount));
    }
    if !(0.0..=1.0).contains(&budget) {
        return Err(HybridError::InvalidBudget(budget));
    }

    let rules = actions.pow(actions as u32);
    let choices = actions + rules;
    let realised = |choice: usize, x: usize| {
        if choice < actions { choice } else { (choice - actions) / actions.pow(x as u32) % actions }
    };
    let mut rewards = vec![0.0; states * choices];
    let mut moves = vec![0.0; states * choices * states];
    for s in 0..states {
        for choice in 0..choices {
            let column = s * choices + choice;
            for x in 0..actions {
                let p = model.intention[s * actions + x];
                let cell = (s * actions + x) * actions + realised(choice, x);
                rewards[column] += p * model.rewards[cell];
                for next in 0..states {
                    moves[column * states + next] += p * model.next_states[cell * states + next];
                }
            }
        }
    }

    let columns = states * choices;
    let mut rows: Vec<Vec<(usize, f64)>> = vec![Vec::new(); states];
    for column in 0..columns {
        for next in 0..states {
            let coefficient = f64::from(u8::from(column / choices == next)) - discount * moves[column * states + next];
            if coefficient != 0.0 {
                rows[next].push((column, coefficient));
            }
        }
    }
    rows.push((0..columns).map(|c| (c, if c % choices >= actions { 1.0 - budget } else { -budget })).collect());
    let mut rhs = initial.to_vec();
    rhs.push(0.0);
    let mut eq = vec![true; states];
    eq.push(false);
    let lp = Lp {
        rows,
        rhs,
        eq,
        lower: vec![0.0; columns],
        upper: vec![1.0 / (1.0 - discount); columns],
        cost: rewards.iter().map(|r| -r).collect(),
    };
    let (occupancy, objective) = solve(&lp).ok_or(HybridError::NoSolution)?;

    let mut consult = vec![0.0; states];
    let mut autonomous = vec![1.0 / actions as f64; states * actions];
    let mut assisted = vec![1.0 / actions as f64; states * actions * actions];
    for s in 0..states {
        let row = &occupancy[s * choices..(s + 1) * choices];
        let alone: f64 = row[..actions].iter().sum();
        let helped: f64 = row[actions..].iter().sum();
        if alone + helped > 0.0 {
            consult[s] = helped / (alone + helped);
        }
        if alone > 0.0 {
            for x in 0..actions {
                autonomous[s * actions + x] = row[x] / alone;
            }
        }
        if helped > 0.0 {
            for x in 0..actions {
                let start = (s * actions + x) * actions;
                assisted[start..start + actions].iter_mut().for_each(|p| *p = 0.0);
                for rule in 0..rules {
                    assisted[start + realised(actions + rule, x)] += row[actions + rule] / helped;
                }
            }
        }
    }
    let consulted: f64 = (0..columns).filter(|c| c % choices >= actions).map(|c| occupancy[c]).sum();
    Ok(HybridPolicy { consult, autonomous, assisted, value: -objective, consulted_share: consulted * (1.0 - discount) })
}
