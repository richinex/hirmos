//! The value of a multi-stage policy estimated from logged trajectories by weighting or backward plug-in.

use std::collections::BTreeMap;
use std::fmt;

use crate::propensity::Normalization;

#[derive(Clone, Debug, PartialEq)]
pub enum SequentialEvaluationError {
    NoTrajectories,
    NoStages,
    StageCountMismatch { expected: usize, found: usize },
    ActionOutOfRange { action: usize, actions: usize },
    InvalidWeight(f64),
    NonFiniteOutcome(f64),
    InvalidPolicyProbability { stage: usize, probability: f64 },
    OutsideSupport { stage: usize, history: Vec<usize>, action: usize },
}

impl fmt::Display for SequentialEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoTrajectories => write!(f, "no trajectories were given"),
            Self::NoStages => write!(f, "trajectories need at least one stage"),
            Self::StageCountMismatch { expected, found } => {
                write!(f, "a trajectory has {found} stages where the first has {expected}")
            }
            Self::ActionOutOfRange { action, actions } => write!(f, "action {action} is outside 0..{actions}"),
            Self::InvalidWeight(weight) => write!(f, "weight {weight} is not a positive number"),
            Self::NonFiniteOutcome(outcome) => write!(f, "outcome {outcome} is not finite"),
            Self::InvalidPolicyProbability { stage, probability } => {
                write!(f, "the policy gives probability {probability} at stage {stage}")
            }
            Self::OutsideSupport { stage, history, action } => write!(
                f,
                "the policy plays action {action} at stage {stage} after history {history:?}, which the log never shows"
            ),
        }
    }
}

impl std::error::Error for SequentialEvaluationError {}

/// What was recorded before and at one decision: the covariate summary and the action taken.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stage {
    pub state: usize,
    pub action: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Trajectory {
    pub stages: Vec<Stage>,
    pub outcome: f64,
    /// How many units, or how much probability, this record stands for.
    pub weight: f64,
}

/// A decision rule: the probability of `action` at `stage` given the history `[state_1, action_1,
/// ..., state_stage]`.
pub type Policy<'a> = dyn Fn(usize, &[usize], usize) -> f64 + 'a;

/// Backward plug-in estimates: the policy value and Q(history, action) for every recorded history
/// at every stage.
#[derive(Clone, Debug, PartialEq)]
pub struct PlugInEvaluation {
    pub value: f64,
    pub action_values: Vec<BTreeMap<Vec<usize>, f64>>,
}

struct Log {
    stages: usize,
    actions: usize,
    totals: BTreeMap<Vec<usize>, f64>,
    outcome_sums: BTreeMap<Vec<usize>, f64>,
    total: f64,
}

impl Log {
    fn new(trajectories: &[Trajectory], actions: usize) -> Result<Self, SequentialEvaluationError> {
        let first = trajectories.first().ok_or(SequentialEvaluationError::NoTrajectories)?;
        let stages = first.stages.len();
        if stages == 0 {
            return Err(SequentialEvaluationError::NoStages);
        }
        let mut totals = BTreeMap::new();
        let mut outcome_sums = BTreeMap::new();
        let mut total = 0.0;
        for trajectory in trajectories {
            if trajectory.stages.len() != stages {
                return Err(SequentialEvaluationError::StageCountMismatch {
                    expected: stages,
                    found: trajectory.stages.len(),
                });
            }
            if !(trajectory.weight.is_finite() && trajectory.weight > 0.0) {
                return Err(SequentialEvaluationError::InvalidWeight(trajectory.weight));
            }
            if !trajectory.outcome.is_finite() {
                return Err(SequentialEvaluationError::NonFiniteOutcome(trajectory.outcome));
            }
            let mut prefix = Vec::with_capacity(2 * stages);
            for stage in &trajectory.stages {
                if stage.action >= actions {
                    return Err(SequentialEvaluationError::ActionOutOfRange { action: stage.action, actions });
                }
                prefix.push(stage.state);
                *totals.entry(prefix.clone()).or_insert(0.0) += trajectory.weight;
                prefix.push(stage.action);
                *totals.entry(prefix.clone()).or_insert(0.0) += trajectory.weight;
            }
            *outcome_sums.entry(prefix).or_insert(0.0) += trajectory.weight * trajectory.outcome;
            total += trajectory.weight;
        }
        Ok(Self { stages, actions, totals, outcome_sums, total })
    }

    fn weight(&self, prefix: &[usize]) -> f64 {
        self.totals.get(prefix).copied().unwrap_or(0.0)
    }

    fn policy_probability(
        policy: &Policy<'_>,
        stage: usize,
        history: &[usize],
        action: usize,
    ) -> Result<f64, SequentialEvaluationError> {
        let probability = policy(stage, history, action);
        if (0.0..=1.0).contains(&probability) {
            Ok(probability)
        } else {
            Err(SequentialEvaluationError::InvalidPolicyProbability { stage, probability })
        }
    }

    fn check_support(&self, policy: &Policy<'_>) -> Result<(), SequentialEvaluationError> {
        for history in self.totals.keys().filter(|prefix| prefix.len() % 2 == 1) {
            let stage = history.len() / 2;
            for action in 0..self.actions {
                if Self::policy_probability(policy, stage, history, action)? > 0.0 {
                    let mut chosen = history.clone();
                    chosen.push(action);
                    if self.weight(&chosen) == 0.0 {
                        return Err(SequentialEvaluationError::OutsideSupport {
                            stage,
                            history: history.clone(),
                            action,
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

/// Estimate the policy's expected outcome by inverse propensity weighting.
pub fn weighting_value(
    trajectories: &[Trajectory],
    actions: usize,
    policy: &Policy<'_>,
    normalization: Normalization,
) -> Result<f64, SequentialEvaluationError> {
    let log = Log::new(trajectories, actions)?;
    log.check_support(policy)?;
    let mut weighted_outcomes = 0.0;
    let mut weighted_ratios = 0.0;
    for trajectory in trajectories {
        let mut ratio = 1.0;
        let mut history = Vec::with_capacity(2 * log.stages);
        for (stage, step) in trajectory.stages.iter().enumerate() {
            history.push(step.state);
            let logged = log.weight(&[history.as_slice(), &[step.action]].concat()) / log.weight(&history);
            ratio *= Log::policy_probability(policy, stage, &history, step.action)? / logged;
            history.push(step.action);
        }
        weighted_outcomes += trajectory.weight * trajectory.outcome * ratio;
        weighted_ratios += trajectory.weight * ratio;
    }
    Ok(match normalization {
        Normalization::HorvitzThompson => weighted_outcomes / log.total,
        Normalization::Hajek => weighted_outcomes / weighted_ratios,
    })
}

/// Estimate the policy's expected outcome by backward plug-in of conditional means.
pub fn plug_in_value(
    trajectories: &[Trajectory],
    actions: usize,
    policy: &Policy<'_>,
) -> Result<PlugInEvaluation, SequentialEvaluationError> {
    let log = Log::new(trajectories, actions)?;
    log.check_support(policy)?;
    let mut action_values: Vec<BTreeMap<Vec<usize>, f64>> = vec![BTreeMap::new(); log.stages];
    for stage in (0..log.stages).rev() {
        let length = 2 * stage + 2;
        let chosen: Vec<Vec<usize>> = log.totals.keys().filter(|p| p.len() == length).cloned().collect();
        for prefix in chosen {
            let value = if stage + 1 == log.stages {
                log.outcome_sums[&prefix] / log.weight(&prefix)
            } else {
                let mut expected = 0.0;
                for (next, weight) in log.totals.range(prefix.clone()..) {
                    if !next.starts_with(&prefix) {
                        break;
                    }
                    if next.len() != length + 1 {
                        continue;
                    }
                    let mut continuation = 0.0;
                    for action in 0..log.actions {
                        let probability = Log::policy_probability(policy, stage + 1, next, action)?;
                        if probability > 0.0 {
                            let mut key = next.clone();
                            key.push(action);
                            continuation += probability * action_values[stage + 1][&key];
                        }
                    }
                    expected += weight / log.weight(&prefix) * continuation;
                }
                expected
            };
            action_values[stage].insert(prefix, value);
        }
    }
    let mut value = 0.0;
    for (start, weight) in log.totals.iter().filter(|(p, _)| p.len() == 1) {
        for action in 0..log.actions {
            let probability = Log::policy_probability(policy, 0, start, action)?;
            if probability > 0.0 {
                value += weight / log.total * probability * action_values[0][&[start[0], action][..]];
            }
        }
    }
    Ok(PlugInEvaluation { value, action_values })
}
