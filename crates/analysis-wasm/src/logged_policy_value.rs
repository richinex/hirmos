//! Browser boundary for scoring plans from a log that records the logging policy's propensities.
use hirmos_causal_core::numpy_reduce::numpy_mean;
use hirmos_causal_core::reinforcement::off_policy_evaluation::{
    bootstrap_intervals, paired_differences, policy_value, round_rewards, weight_diagnostics, Estimator, LoggedRound,
};
use serde::{Deserialize, Serialize};

use crate::holdout::Holdout;

#[derive(Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub(crate) enum Rule {
    /// Every action with equal probability.
    Uniform,
    /// One action every time.
    Always { action: usize },
    /// Action probabilities for each input pattern, indexed by each row's pattern.
    Table { probabilities: Vec<Vec<f64>> },
}

/// The logged rows a plan is scored on: every row, or only the rows held out from training its rule.
#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Rows {
    All,
    HeldOut,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Plan {
    rule: Rule,
    rows: Rows,
    /// Bootstrap this plan's value and its paired difference from the observed average.
    compare: bool,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub(crate) enum EstimatorChoice {
    InverseProbability { max_weight: Option<f64> },
    SelfNormalized,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Bootstrap {
    rounds: usize,
    seed: u32,
    alpha: f64,
}

/// `values` holds four numbers per logged row: action, reward, the logging policy's propensity and the row's input pattern.
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    actions: Vec<f64>,
    plans: Vec<Plan>,
    estimator: EstimatorChoice,
    bootstrap: Bootstrap,
    holdout: Option<Holdout>,
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Interval {
    lower: f64,
    upper: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Estimate {
    value: f64,
    interval: Option<Interval>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Weights {
    support: f64,
    max_weight: f64,
    effective_rows: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Scored {
    rows: usize,
    /// The average logged outcome on the rows this plan is scored on.
    observed: f64,
    value: Estimate,
    /// The plan's value minus the observed average on the same rows, with a paired interval.
    difference: Estimate,
    weights: Weights,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Evidence {
    rows: usize,
    held_out_rows: Option<usize>,
    observed: Estimate,
    plans: Vec<Scored>,
}

/// Bootstrap samples waiting for shared draws over one set of rows.
struct Pending {
    samples: Vec<Vec<f64>>,
    owners: Vec<(usize, Slot)>,
}

#[derive(Clone, Copy)]
enum Slot {
    Observed,
    Value,
    Difference,
}

pub(crate) fn run(request: Request, values: &[f64], progress: impl Fn(&'static str, usize, usize)) -> Result<Evidence, String> {
    if values.len() % 4 != 0 {
        return Err("logged rows come in fours".to_owned());
    }
    let arms = request.actions.len();
    if arms < 2 {
        return Err("a rule needs at least two actions".to_owned());
    }
    let actions = values
        .chunks_exact(4)
        .map(|row| {
            request.actions.iter().position(|&value| value == row[0]).ok_or_else(|| format!("action {} is not among the listed actions", row[0]))
        })
        .collect::<Result<Vec<usize>, String>>()?;
    let estimator = match request.estimator {
        EstimatorChoice::InverseProbability { max_weight } => Estimator::InverseProbability { max_weight: max_weight.unwrap_or(f64::INFINITY) },
        EstimatorChoice::SelfNormalized => Estimator::SelfNormalized,
    };
    let held = match request.holdout {
        Some(holdout) => Some(holdout.mask(actions.len())?),
        None if request.plans.iter().any(|plan| plan.rows == Rows::HeldOut) => {
            return Err("a plan scored on held-out rows needs a held-out split".to_owned())
        }
        None => None,
    };
    let selected = |rows: Rows| -> Vec<usize> {
        match (rows, &held) {
            (Rows::HeldOut, Some(mask)) => (0..actions.len()).filter(|&row| mask[row]).collect(),
            _ => (0..actions.len()).collect(),
        }
    };
    let all: Vec<f64> = values.chunks_exact(4).map(|row| row[1]).collect();
    let mut groups = [
        Pending { samples: vec![all.clone()], owners: vec![(usize::MAX, Slot::Observed)] },
        Pending { samples: Vec::new(), owners: Vec::new() },
    ];

    let mut scored = Vec::with_capacity(request.plans.len());
    for (index, plan) in request.plans.iter().enumerate() {
        progress("Scoring plans", index, request.plans.len());
        let rows = selected(plan.rows);
        let mut rounds = Vec::with_capacity(rows.len());
        for &row in &rows {
            let (action, pattern) = (actions[row], values[row * 4 + 3]);
            let target = match &plan.rule {
                Rule::Uniform => 1.0 / arms as f64,
                Rule::Always { action: chosen } if *chosen >= arms => return Err(format!("rule action {chosen} is outside the {arms} actions")),
                Rule::Always { action: chosen } => f64::from(u8::from(action == *chosen)),
                Rule::Table { probabilities } => {
                    let row = (pattern >= 0.0 && pattern.fract() == 0.0)
                        .then(|| probabilities.get(pattern as usize))
                        .flatten()
                        .ok_or_else(|| format!("input pattern {pattern} has no row in the rule's table"))?;
                    *row.get(action).ok_or_else(|| format!("the rule's table has no probability for action {action}"))?
                }
            };
            rounds.push(LoggedRound { reward: values[row * 4 + 1], propensity: values[row * 4 + 2], target });
        }
        let value = policy_value(&rounds, estimator).map_err(|e| e.to_string())?;
        let observed = numpy_mean(&rounds.iter().map(|round| round.reward).collect::<Vec<f64>>());
        let weights = weight_diagnostics(&rounds).map_err(|e| e.to_string())?;
        let differences = paired_differences(&rounds, estimator).map_err(|e| e.to_string())?;
        let difference = numpy_mean(&differences);
        if plan.compare {
            let group = &mut groups[usize::from(plan.rows == Rows::HeldOut)];
            group.samples.push(round_rewards(&rounds, estimator).map_err(|e| e.to_string())?);
            group.owners.push((index, Slot::Value));
            group.samples.push(differences);
            group.owners.push((index, Slot::Difference));
        }
        scored.push(Scored {
            rows: rows.len(),
            observed,
            value: Estimate { value, interval: None },
            difference: Estimate { value: difference, interval: None },
            weights: Weights { support: weights.support, max_weight: weights.max_weight, effective_rows: weights.effective_sample_size },
        });
    }

    let b = &request.bootstrap;
    let busy: Vec<&Pending> = groups.iter().filter(|group| !group.samples.is_empty()).collect();
    let total = b.rounds * busy.len();
    let mut observed = Estimate { value: numpy_mean(&all), interval: None };
    for (step, group) in busy.into_iter().enumerate() {
        let slices: Vec<&[f64]> = group.samples.iter().map(Vec::as_slice).collect();
        let found = bootstrap_intervals(&slices, b.alpha, b.rounds, b.seed, |round| progress("Bootstrapping the compared plans", step * b.rounds + round, total))
            .map_err(|e| e.to_string())?;
        for (&(index, slot), interval) in group.owners.iter().zip(found) {
            let interval = Some(Interval { lower: interval.lower, upper: interval.upper });
            match slot {
                Slot::Observed => observed.interval = interval,
                Slot::Value => scored[index].value.interval = interval,
                Slot::Difference => scored[index].difference.interval = interval,
            }
        }
    }
    Ok(Evidence { rows: actions.len(), held_out_rows: held.map(|mask| mask.iter().filter(|&&h| h).count()), observed, plans: scored })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(rule: Rule, compare: bool) -> Plan {
        Plan { rule, rows: Rows::All, compare }
    }

    #[test]
    fn a_uniform_log_scored_for_one_action_gives_that_actions_mean() {
        // Uniform logging over two actions: action 0 pays 1 in two of four rows, action 1 never pays.
        let values = [0.0, 1.0, 0.5, 0.0, 0.0, 1.0, 0.5, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 0.0, 1.0, 0.0, 0.5, 0.0, 1.0, 0.0, 0.5, 0.0];
        let request = Request {
            actions: vec![0.0, 1.0],
            plans: vec![plan(Rule::Always { action: 0 }, true), plan(Rule::Uniform, false)],
            estimator: EstimatorChoice::SelfNormalized,
            bootstrap: Bootstrap { rounds: 50, seed: 1, alpha: 0.05 },
            holdout: None,
        };
        let evidence = run(request, &values, |_, _, _| {}).unwrap();
        assert_eq!(evidence.plans[0].value.value, 0.5);
        assert!(evidence.plans[0].value.interval.is_some() && evidence.plans[0].difference.interval.is_some());
        assert!(evidence.plans[1].value.interval.is_none() && evidence.plans[1].difference.interval.is_none());
        assert_eq!(evidence.plans[1].value.value, 2.0 / 6.0);
        assert!((evidence.plans[0].difference.value - (0.5 - 2.0 / 6.0)).abs() < 1e-15);
        assert!(evidence.observed.interval.is_some());
        // Action 0 was logged in four of six rows, each with weight 2.
        assert_eq!(evidence.plans[0].weights.support, 4.0 / 6.0);
        assert_eq!(evidence.plans[0].weights.effective_rows, 4.0);
    }

    #[test]
    fn a_table_rule_that_copies_the_logging_policy_recovers_the_logged_mean() {
        // Two patterns; the logging policy's propensity is the table's probability of the logged action.
        let values = [0.0, 1.0, 0.8, 0.0, 1.0, 0.0, 0.2, 0.0, 1.0, 1.0, 0.4, 1.0, 0.0, 0.0, 0.6, 1.0];
        let table = vec![vec![0.8, 0.2], vec![0.6, 0.4]];
        let request = Request {
            actions: vec![0.0, 1.0],
            plans: vec![plan(Rule::Table { probabilities: table }, false)],
            estimator: EstimatorChoice::InverseProbability { max_weight: None },
            bootstrap: Bootstrap { rounds: 10, seed: 1, alpha: 0.05 },
            holdout: None,
        };
        let evidence = run(request, &values, |_, _, _| {}).unwrap();
        assert_eq!(evidence.plans[0].value.value, 0.5);
        assert!(evidence.plans[0].difference.value.abs() < 1e-15);
    }

    #[test]
    fn a_held_out_plan_is_scored_only_on_the_held_out_rows() {
        let values: Vec<f64> = (0..100).flat_map(|row| [f64::from(row % 2), f64::from(row % 3 == 0), 0.5, 0.0]).collect();
        let request = Request {
            actions: vec![0.0, 1.0],
            plans: vec![Plan { rule: Rule::Uniform, rows: Rows::HeldOut, compare: true }, plan(Rule::Uniform, true)],
            estimator: EstimatorChoice::SelfNormalized,
            bootstrap: Bootstrap { rounds: 20, seed: 3, alpha: 0.05 },
            holdout: Some(serde_json::from_value(serde_json::json!({ "share": 0.25, "seed": 4 })).unwrap()),
        };
        let evidence = run(request, &values, |_, _, _| {}).unwrap();
        assert_eq!((evidence.held_out_rows, evidence.plans[0].rows, evidence.plans[1].rows), (Some(25), 25, 100));
        assert!(evidence.plans[0].difference.interval.is_some());
    }

    #[test]
    fn a_held_out_plan_without_a_split_is_refused() {
        let request = Request {
            actions: vec![0.0, 1.0],
            plans: vec![Plan { rule: Rule::Uniform, rows: Rows::HeldOut, compare: false }],
            estimator: EstimatorChoice::SelfNormalized,
            bootstrap: Bootstrap { rounds: 1, seed: 1, alpha: 0.05 },
            holdout: None,
        };
        assert!(run(request, &[0.0, 1.0, 0.5, 0.0, 1.0, 0.0, 0.5, 0.0], |_, _, _| {}).is_err());
    }
}
