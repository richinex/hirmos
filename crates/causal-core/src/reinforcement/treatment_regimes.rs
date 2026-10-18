//! Online learning of a multi-stage treatment regime, optionally started from causal bounds on confounded logs.

use std::fmt;

use crate::reinforcement::sequential_evaluation::Trajectory;
use crate::simplex::{solve, Lp};

#[derive(Clone, Debug, PartialEq)]
pub enum RegimeError {
    NoStages,
    StageCountMismatch { states: usize, actions: usize },
    EmptyDomain { stage: usize },
    TrajectoryOutsideShape,
    OutcomeOutsideUnitInterval(f64),
    NoLoggedWeight,
}

impl fmt::Display for RegimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoStages => write!(f, "a regime needs at least one stage"),
            Self::StageCountMismatch { states, actions } => {
                write!(f, "{states} state domains but {actions} action domains")
            }
            Self::EmptyDomain { stage } => write!(f, "stage {stage} has an empty domain"),
            Self::TrajectoryOutsideShape => write!(f, "a trajectory does not fit the regime's stages and domains"),
            Self::OutcomeOutsideUnitInterval(y) => write!(f, "outcome {y} is outside [0, 1]"),
            Self::NoLoggedWeight => write!(f, "the log carries no weight"),
        }
    }
}

impl std::error::Error for RegimeError {}

/// The number of states and actions at each stage; every decision may read the whole history.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegimeShape {
    states: Vec<usize>,
    actions: Vec<usize>,
}

impl RegimeShape {
    pub fn new(states: Vec<usize>, actions: Vec<usize>) -> Result<Self, RegimeError> {
        if states.is_empty() {
            return Err(RegimeError::NoStages);
        }
        if states.len() != actions.len() {
            return Err(RegimeError::StageCountMismatch { states: states.len(), actions: actions.len() });
        }
        if let Some(stage) = (0..states.len()).find(|&k| states[k] == 0 || actions[k] == 0) {
            return Err(RegimeError::EmptyDomain { stage });
        }
        Ok(Self { states, actions })
    }

    pub fn stages(&self) -> usize {
        self.states.len()
    }

    fn histories(&self, stage: usize) -> usize {
        (0..stage).map(|k| self.states[k] * self.actions[k]).product::<usize>() * self.states[stage]
    }

    /// The history index before each decision and the index of the history with that decision.
    fn positions<'a>(&'a self, trajectory: &'a Trajectory) -> Result<impl Iterator<Item = (usize, usize)> + 'a, RegimeError> {
        let fits = trajectory.stages.len() == self.stages()
            && trajectory.stages.iter().enumerate().all(|(k, stage)| stage.state < self.states[k] && stage.action < self.actions[k]);
        if !fits {
            return Err(RegimeError::TrajectoryOutsideShape);
        }
        Ok(trajectory.stages.iter().enumerate().scan(0, |pair, (k, stage)| {
            let history = *pair * self.states[k] + stage.state;
            *pair = history * self.actions[k] + stage.action;
            Some((history, *pair))
        }))
    }

    fn widest_stage(&self) -> usize {
        self.states.iter().copied().max().unwrap_or(1)
    }
}

/// Intervals for every interventional transition P_x(s_k+1 | history) and final reward E_x[Y | history].
#[derive(Clone, Debug, PartialEq)]
pub struct CausalBounds {
    shape: RegimeShape,
    initial: Vec<f64>,
    transitions: Vec<Vec<(f64, f64)>>,
    rewards: Vec<(f64, f64)>,
}

impl CausalBounds {
    /// Bounds that say nothing: every probability and reward in [0, 1].
    pub fn uninformative(shape: &RegimeShape) -> Self {
        let stages = shape.stages();
        Self {
            initial: vec![1.0 / shape.states[0] as f64; shape.states[0]],
            transitions: (1..stages).map(|k| vec![(0.0, 1.0); shape.histories(k)]).collect(),
            rewards: vec![(0.0, 1.0); shape.histories(stages - 1) * shape.actions[stages - 1]],
            shape: shape.clone(),
        }
    }

    /// Bounds from a confounded log, with first states distributed as logged.
    pub fn from_log(shape: &RegimeShape, log: &[Trajectory]) -> Result<Self, RegimeError> {
        let stages = shape.stages();
        let mut reached: Vec<Vec<f64>> = (0..stages).map(|k| vec![0.0; shape.histories(k)]).collect();
        let mut chosen: Vec<Vec<f64>> = (0..stages).map(|k| vec![0.0; shape.histories(k) * shape.actions[k]]).collect();
        let mut rewarded = vec![0.0; chosen[stages - 1].len()];
        let mut total = 0.0;
        for trajectory in log {
            if !(0.0..=1.0).contains(&trajectory.outcome) {
                return Err(RegimeError::OutcomeOutsideUnitInterval(trajectory.outcome));
            }
            for (k, (history, pair)) in shape.positions(trajectory)?.enumerate() {
                reached[k][history] += trajectory.weight;
                chosen[k][pair] += trajectory.weight;
                if k == stages - 1 {
                    rewarded[pair] += trajectory.weight * trajectory.outcome;
                }
            }
            total += trajectory.weight;
        }
        if total <= 0.0 {
            return Err(RegimeError::NoLoggedWeight);
        }
        let scale = |masses: &mut Vec<Vec<f64>>| masses.iter_mut().flatten().for_each(|m| *m /= total);
        scale(&mut reached);
        scale(&mut chosen);
        rewarded.iter_mut().for_each(|m| *m /= total);

        let mut gamma = vec![reached[0].clone()];
        for k in 1..stages {
            let next: Vec<f64> = (0..shape.histories(k))
                .map(|h| {
                    let pair = h / shape.states[k];
                    let previous = pair / shape.actions[k - 1];
                    reached[k][h] - chosen[k - 1][pair] + gamma[k - 1][previous]
                })
                .collect();
            gamma.push(next);
        }
        let ratio = |numerator: f64, denominator: f64, otherwise: f64| {
            if denominator > 0.0 { numerator / denominator } else { otherwise }
        };
        let transitions = (1..stages)
            .map(|k| {
                (0..shape.histories(k))
                    .map(|h| {
                        let before = h / shape.states[k] / shape.actions[k - 1];
                        let denominator = gamma[k - 1][before];
                        (ratio(reached[k][h], denominator, 0.0), ratio(gamma[k][h], denominator, 1.0))
                    })
                    .collect()
            })
            .collect();
        let last = stages - 1;
        let rewards = (0..chosen[last].len())
            .map(|pair| {
                let denominator = gamma[last][pair / shape.actions[last]];
                let failed = chosen[last][pair] - rewarded[pair];
                (ratio(rewarded[pair], denominator, 0.0), 1.0 - ratio(failed, denominator, 0.0))
            })
            .collect();
        Ok(Self { shape: shape.clone(), initial: reached[0].clone(), transitions, rewards })
    }

    /// Bounds on P_x(s_k+1 | history), indexed by stage k + 1 and the history that ends in s_k+1.
    pub fn transition(&self, stage: usize, history: usize) -> (f64, f64) {
        self.transitions[stage - 1][history]
    }

    /// Bounds on E_x[Y | history] for the final history with its last action.
    pub fn reward(&self, pair: usize) -> (f64, f64) {
        self.rewards[pair]
    }

    /// The lowest and highest value of `policy` over all models within the bounds.
    pub fn policy_value(&self, policy: &RegimePolicy) -> (f64, f64) {
        let shape = &self.shape;
        let last = shape.stages() - 1;
        let mut low: Vec<f64> = (0..shape.histories(last))
            .map(|h| self.rewards[h * shape.actions[last] + policy.actions[last][h]].0)
            .collect();
        let mut high: Vec<f64> = (0..shape.histories(last))
            .map(|h| self.rewards[h * shape.actions[last] + policy.actions[last][h]].1)
            .collect();
        let mut scratch = Scratch::new(shape.widest_stage());
        for k in (0..last).rev() {
            let next_states = shape.states[k + 1];
            let mut stage_low = Vec::with_capacity(shape.histories(k));
            let mut stage_high = Vec::with_capacity(shape.histories(k));
            for h in 0..shape.histories(k) {
                let start = (h * shape.actions[k] + policy.actions[k][h]) * next_states;
                let bounds = &self.transitions[k][start..start + next_states];
                stage_low.push(scratch.extreme_in_box(&low[start..start + next_states], bounds, false));
                stage_high.push(scratch.extreme_in_box(&high[start..start + next_states], bounds, true));
            }
            low = stage_low;
            high = stage_high;
        }
        let weigh = |values: &[f64]| self.initial.iter().zip(values).map(|(p, v)| p * v).sum::<f64>();
        (weigh(&low), weigh(&high))
    }
}

/// A deterministic regime: one action for every history before every decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegimePolicy {
    shape: RegimeShape,
    actions: Vec<Vec<usize>>,
}

impl RegimePolicy {
    /// The action for the history `[s_1, x_1, ..., s_k]`.
    pub fn action(&self, history: &[usize]) -> usize {
        let stage = history.len() / 2;
        let mut index = 0;
        for k in 0..stage {
            index = (index * self.shape.states[k] + history[2 * k]) * self.shape.actions[k] + history[2 * k + 1];
        }
        index = index * self.shape.states[stage] + history[2 * stage];
        self.actions[stage][index]
    }
}

/// An optimistic learner over histories that plans only among models inside its causal bounds.
#[derive(Clone, Debug)]
pub struct RegimeLearner {
    shape: RegimeShape,
    bounds: CausalBounds,
    visits: Vec<Vec<f64>>,
    arrivals: Vec<Vec<f64>>,
    reward_total: Vec<f64>,
    episodes: usize,
}

impl RegimeLearner {
    pub fn new(bounds: CausalBounds) -> Self {
        let shape = bounds.shape.clone();
        let stages = shape.stages();
        Self {
            visits: (0..stages).map(|k| vec![0.0; shape.histories(k) * shape.actions[k]]).collect(),
            arrivals: (1..stages).map(|k| vec![0.0; shape.histories(k)]).collect(),
            reward_total: vec![0.0; shape.histories(stages - 1) * shape.actions[stages - 1]],
            episodes: 0,
            bounds,
            shape,
        }
    }

    /// The optimal regime of the most optimistic model consistent with the data and the bounds, with delta = t^-4.
    pub fn policy(&self) -> RegimePolicy {
        let shape = &self.shape;
        let stages = shape.stages();
        let last = stages - 1;
        let log_inverse_delta = 4.0 * ((self.episodes + 1) as f64).ln();
        let domain = |k: usize| (0..=k).map(|j| shape.states[j] * shape.actions[j]).product::<usize>() as f64;
        let mut actions = vec![Vec::new(); stages];

        let reward_log = (8.0 * stages as f64 * domain(last)).ln() + log_inverse_delta;
        let mut values = vec![0.0; shape.histories(last)];
        actions[last] = vec![0; shape.histories(last)];
        for h in 0..shape.histories(last) {
            let scores = (0..shape.actions[last]).map(|x| {
                let pair = h * shape.actions[last] + x;
                let count = self.visits[last][pair].max(1.0);
                let width = (reward_log / (2.0 * count)).sqrt();
                let (low, high) = self.bounds.rewards[pair];
                ((self.reward_total[pair] / count + width).min(1.0).max(low)).min(high)
            });
            (actions[last][h], values[h]) = first_best(scores);
        }

        let mut scratch = Scratch::new(shape.widest_stage());
        for k in (0..last).rev() {
            let next_states = shape.states[k + 1];
            let transition_log = (4.0 * stages as f64 * domain(k)).ln() + log_inverse_delta;
            let mut stage_values = vec![0.0; shape.histories(k)];
            actions[k] = vec![0; shape.histories(k)];
            for h in 0..shape.histories(k) {
                let mut best = (0, f64::NEG_INFINITY);
                for x in 0..shape.actions[k] {
                    let pair = h * shape.actions[k] + x;
                    let visits = self.visits[k][pair];
                    let count = visits.max(1.0);
                    let start = pair * next_states;
                    let width = (2.0 * next_states as f64 * transition_log / count).sqrt();
                    let bounds = &self.bounds.transitions[k][start..start + next_states];
                    let next_values = &values[start..start + next_states];
                    let score = if visits == 0.0 && width >= 1.0 {
                        scratch.extreme_in_box(next_values, bounds, true)
                    } else {
                        scratch.estimate.clear();
                        scratch.estimate.extend(self.arrivals[k][start..start + next_states].iter().map(|n| n / count));
                        scratch.optimistic(next_values, width, bounds)
                    };
                    if score > best.1 {
                        best = (x, score);
                    }
                }
                (actions[k][h], stage_values[h]) = best;
            }
            values = stage_values;
        }
        RegimePolicy { shape: shape.clone(), actions }
    }

    pub fn record(&mut self, trajectory: &Trajectory) -> Result<(), RegimeError> {
        if !(0.0..=1.0).contains(&trajectory.outcome) {
            return Err(RegimeError::OutcomeOutsideUnitInterval(trajectory.outcome));
        }
        let last = self.shape.stages() - 1;
        for (k, (history, pair)) in self.shape.positions(trajectory)?.enumerate() {
            self.visits[k][pair] += 1.0;
            if k > 0 {
                self.arrivals[k - 1][history] += 1.0;
            }
            if k == last {
                self.reward_total[pair] += trajectory.outcome;
            }
        }
        self.episodes += 1;
        Ok(())
    }
}

fn first_best(scores: impl Iterator<Item = f64>) -> (usize, f64) {
    scores.enumerate().fold((0, f64::NEG_INFINITY), |best, (i, s)| if s > best.1 { (i, s) } else { best })
}

/// The largest sum of p(s) values(s) over distributions inside `bounds` within L1 distance `width` of `estimate`, or of the nearest such distribution.
pub fn optimistic_expectation(values: &[f64], estimate: &[f64], width: f64, bounds: &[(f64, f64)]) -> f64 {
    let mut scratch = Scratch::new(values.len());
    scratch.estimate.extend_from_slice(estimate);
    scratch.optimistic(values, width, bounds)
}

/// Buffers reused across the many small optimisations of one planning pass.
struct Scratch {
    estimate: Vec<f64>,
    mass: Vec<f64>,
    order: Vec<usize>,
}

impl Scratch {
    fn new(size: usize) -> Self {
        Self { estimate: Vec::with_capacity(size), mass: Vec::with_capacity(size), order: Vec::with_capacity(size) }
    }

    fn sort_by_value(&mut self, values: &[f64], descending: bool) {
        self.order.clear();
        self.order.extend(0..values.len());
        self.order.sort_by(|&a, &b| if descending { values[b].total_cmp(&values[a]) } else { values[a].total_cmp(&values[b]) });
    }

    /// Moves probability from the lowest to the highest values, which is exact when the estimate is a distribution inside the bounds.
    fn optimistic(&mut self, values: &[f64], width: f64, bounds: &[(f64, f64)]) -> f64 {
        let inside = self.estimate.iter().zip(bounds).all(|(p, b)| b.0.max(0.0) <= *p && *p <= b.1.min(1.0))
            && (self.estimate.iter().sum::<f64>() - 1.0).abs() <= 1e-12;
        if !inside {
            return simplex_optimum(values, &self.estimate, width, bounds);
        }
        self.sort_by_value(values, true);
        self.mass.clear();
        self.mass.extend_from_slice(&self.estimate);
        let (mut top, mut bottom, mut budget) = (0, values.len() - 1, width / 2.0);
        while top < bottom && budget > 0.0 {
            let (high, low) = (self.order[top], self.order[bottom]);
            if values[high] <= values[low] {
                break;
            }
            let room = bounds[high].1.min(1.0) - self.mass[high];
            let spare = self.mass[low] - bounds[low].0.max(0.0);
            if room <= 0.0 {
                top += 1;
            } else if spare <= 0.0 {
                bottom -= 1;
            } else {
                let moved = room.min(spare).min(budget);
                self.mass[high] += moved;
                self.mass[low] -= moved;
                budget -= moved;
            }
        }
        self.mass.iter().zip(values).map(|(p, v)| p * v).sum()
    }

    fn extreme_in_box(&mut self, values: &[f64], bounds: &[(f64, f64)], maximise: bool) -> f64 {
        self.sort_by_value(values, maximise);
        self.mass.clear();
        self.mass.extend(bounds.iter().map(|b| b.0));
        let mut remaining = 1.0 - self.mass.iter().sum::<f64>();
        for &j in &self.order {
            let added = remaining.min(bounds[j].1 - bounds[j].0).max(0.0);
            self.mass[j] += added;
            remaining -= added;
        }
        self.mass.iter().zip(values).map(|(p, v)| p * v).sum()
    }
}

fn simplex_optimum(values: &[f64], estimate: &[f64], width: f64, bounds: &[(f64, f64)]) -> f64 {
    let cost: Vec<f64> = values.iter().map(|v| -v).collect();
    if let Some((_, objective)) = solve(&distance_lp(estimate, bounds, Some(width), &cost)) {
        return -objective;
    }
    let nearest = solve(&distance_lp(estimate, bounds, None, &vec![0.0; values.len()]))
        .expect("the bounds contain a distribution")
        .1;
    -solve(&distance_lp(estimate, bounds, Some(nearest + 1e-9), &cost)).expect("the nearest distribution is feasible").1
}

/// Variables p, d, then one slack per inequality; every row an equality, signed so the start at the lower bounds is feasible.
fn distance_lp(estimate: &[f64], bounds: &[(f64, f64)], width: Option<f64>, cost: &[f64]) -> Lp {
    let n = estimate.len();
    let mut rows: Vec<Vec<(usize, f64)>> = Vec::new();
    let mut rhs = Vec::new();
    for j in 0..n {
        rows.push(vec![(j, 1.0), (n + j, -1.0), (2 * n + j, 1.0)]);
        rhs.push(estimate[j]);
        rows.push(vec![(j, -1.0), (n + j, -1.0), (3 * n + j, 1.0)]);
        rhs.push(-estimate[j]);
    }
    rows.push((0..n).map(|j| (j, 1.0)).collect());
    rhs.push(1.0);
    let mut lower: Vec<f64> = bounds.iter().map(|b| b.0.max(0.0)).collect();
    let mut upper: Vec<f64> = bounds.iter().map(|b| b.1.min(1.0)).collect();
    lower.extend(std::iter::repeat(0.0).take(3 * n));
    upper.extend(std::iter::repeat(2.0).take(n));
    upper.extend(std::iter::repeat(4.0).take(2 * n));
    let mut costs = cost.to_vec();
    match width {
        Some(width) => {
            rows.push((n..2 * n).map(|j| (j, 1.0)).chain([(4 * n, 1.0)]).collect());
            rhs.push(width);
            lower.push(0.0);
            upper.push(width);
            costs.extend(std::iter::repeat(0.0).take(3 * n + 1));
        }
        None => {
            costs.extend(std::iter::repeat(1.0).take(n));
            costs.extend(std::iter::repeat(0.0).take(2 * n));
        }
    }
    for (row, value) in rows.iter_mut().zip(rhs.iter_mut()) {
        let start: f64 = row.iter().map(|&(column, coefficient)| coefficient * lower[column]).sum();
        if *value < start {
            row.iter_mut().for_each(|entry| entry.1 = -entry.1);
            *value = -*value;
        }
    }
    let eq = vec![true; rows.len()];
    Lp { rows, rhs, eq, lower, upper, cost: costs }
}
