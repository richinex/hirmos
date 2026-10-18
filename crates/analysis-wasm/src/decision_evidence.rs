//! Browser boundary for what a decision log and its randomised rows say about each action.
use hirmos_causal_core::reinforcement::offline_to_online::{
    learn_online, natural_bounds_from_cells, ConfoundedBandit, LoggedCell, OfflineUse, ReplayedBandit, UnitReward,
};
use serde::{Deserialize, Serialize};

const CHECKPOINTS: usize = 60;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Learning {
    horizon: usize,
    seeds: Vec<u64>,
}

/// `values` holds four numbers per cell: action, outcome, sample (1 randomised, 0 logged, 2 neither) and count.
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    actions: Vec<f64>,
    learning: Option<Learning>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Sample {
    rows: u64,
    mean: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Action {
    value: f64,
    logged: Sample,
    share: f64,
    lower: f64,
    upper: f64,
    randomised: Option<Sample>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Curve {
    /// Regret against the best randomised mean after each checkpoint step, averaged over seeds.
    mean: Vec<f64>,
    low: Vec<f64>,
    high: Vec<f64>,
    /// Final regret for each seed, in seed order.
    finals: Vec<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LearningEvidence {
    steps: Vec<usize>,
    best: f64,
    scratch: Curve,
    clipped: Curve,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Evidence {
    actions: Vec<Action>,
    logged_rows: u64,
    randomised_rows: u64,
    excluded_rows: u64,
    learning: Option<LearningEvidence>,
}

fn cells(request: &Request, values: &[f64]) -> Result<(Vec<LoggedCell>, Vec<LoggedCell>, u64), String> {
    if values.len() % 4 != 0 {
        return Err("decision cells come in fours".to_owned());
    }
    let (mut logged, mut randomised, mut excluded) = (Vec::new(), Vec::new(), 0);
    for cell in values.chunks_exact(4) {
        let count = cell[3];
        if !(count >= 0.0 && count.fract() == 0.0) {
            return Err(format!("cell count {count} is not a whole number"));
        }
        let count = count as u64;
        if cell[2] == 2.0 {
            excluded += count;
            continue;
        }
        let arm = request
            .actions
            .iter()
            .position(|&value| value == cell[0])
            .ok_or_else(|| format!("action {} is not among the listed actions", cell[0]))?;
        let reward = UnitReward::new(cell[1]).map_err(|e| e.to_string())?;
        match cell[2] {
            0.0 => logged.push(LoggedCell { arm, reward, count }),
            1.0 => randomised.push(LoggedCell { arm, reward, count }),
            other => return Err(format!("sample code {other} is not 0, 1 or 2")),
        }
    }
    Ok((logged, randomised, excluded))
}

fn summary(cells: &[LoggedCell], arm: usize) -> Option<Sample> {
    let (rows, total) = cells
        .iter()
        .filter(|cell| cell.arm == arm)
        .fold((0u64, 0.0), |(rows, total), cell| (rows + cell.count, total + cell.reward.value() * cell.count as f64));
    (rows > 0).then(|| Sample { rows, mean: total / rows as f64 })
}

fn curve(regrets: &[Vec<f64>]) -> Curve {
    let points = regrets[0].len();
    let column = |i: usize| regrets.iter().map(move |run| run[i]);
    Curve {
        mean: (0..points).map(|i| column(i).sum::<f64>() / regrets.len() as f64).collect(),
        low: (0..points).map(|i| column(i).fold(f64::INFINITY, f64::min)).collect(),
        high: (0..points).map(|i| column(i).fold(f64::NEG_INFINITY, f64::max)).collect(),
        finals: regrets.iter().map(|run| run[points - 1]).collect(),
    }
}

pub(crate) fn run(request: Request, values: &[f64]) -> Result<Evidence, String> {
    let arms = request.actions.len();
    if arms < 2 {
        return Err("a decision needs at least two actions".to_owned());
    }
    let (logged, randomised, excluded_rows) = cells(&request, values)?;
    let bounds = natural_bounds_from_cells(arms, &logged).map_err(|e| e.to_string())?;
    let actions: Vec<Action> = bounds
        .iter()
        .enumerate()
        .map(|(arm, bound)| Action {
            value: request.actions[arm],
            logged: summary(&logged, arm).expect("bounds require every action in the log"),
            share: bound.pulls as f64 / logged.iter().map(|cell| cell.count).sum::<u64>() as f64,
            lower: bound.interval.lower,
            upper: bound.interval.upper,
            randomised: summary(&randomised, arm),
        })
        .collect();

    let learning = match &request.learning {
        None => None,
        Some(settings) => {
            if settings.horizon == 0 || settings.seeds.is_empty() {
                return Err("learning needs a positive horizon and at least one seed".to_owned());
            }
            if let Some(action) = actions.iter().find(|action| action.randomised.is_none()) {
                return Err(format!("action {} has no randomised rows to learn from", action.value));
            }
            let means: Vec<f64> = actions.iter().map(|action| action.randomised.as_ref().unwrap().mean).collect();
            let best = means.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let every = settings.horizon.div_ceil(CHECKPOINTS);
            let steps: Vec<usize> = (1..=settings.horizon).filter(|t| t % every == 0 || *t == settings.horizon).collect();
            let regret = |offline: &OfflineUse, seed: u64| -> Result<Vec<f64>, String> {
                let mut bandit = ReplayedBandit::new(arms, &logged, &randomised, seed).map_err(|e| e.to_string())?;
                debug_assert_eq!(bandit.arms(), arms);
                let pulls = learn_online(&mut bandit, offline, settings.horizon).map_err(|e| e.to_string())?;
                let mut total = 0.0;
                let mut points = Vec::with_capacity(steps.len());
                for (t, step) in pulls.iter().enumerate() {
                    total += best - means[step.arm];
                    if steps.binary_search(&(t + 1)).is_ok() {
                        points.push(total);
                    }
                }
                Ok(points)
            };
            let clip = OfflineUse::Clip(bounds.iter().map(|bound| bound.interval).collect());
            let scratch = settings.seeds.iter().map(|&seed| regret(&OfflineUse::Ignore, seed)).collect::<Result<Vec<_>, _>>()?;
            let clipped = settings.seeds.iter().map(|&seed| regret(&clip, seed)).collect::<Result<Vec<_>, _>>()?;
            Some(LearningEvidence { steps, best, scratch: curve(&scratch), clipped: curve(&clipped) })
        }
    };

    Ok(Evidence {
        actions,
        logged_rows: logged.iter().map(|cell| cell.count).sum(),
        randomised_rows: randomised.iter().map(|cell| cell.count).sum(),
        excluded_rows,
        learning,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_contain_the_randomised_mean_and_clipping_lowers_regret() {
        // Logged: arm 0 chosen half the time with mean 1, arm 1 with mean 0.1; randomised means 0.9 and 0.1.
        let values = [0.0, 1.0, 0.0, 50.0, 1.0, 1.0, 0.0, 5.0, 1.0, 0.0, 0.0, 45.0, 0.0, 1.0, 1.0, 9.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 9.0, 0.0, 1.0, 2.0, 7.0];
        let request = Request { actions: vec![0.0, 1.0], learning: Some(Learning { horizon: 2_000, seeds: vec![1, 2, 3] }) };
        let evidence = run(request, &values).unwrap();
        assert_eq!((evidence.logged_rows, evidence.randomised_rows, evidence.excluded_rows), (100, 20, 7));
        for action in &evidence.actions {
            let truth = action.randomised.as_ref().unwrap().mean;
            assert!(action.lower <= truth && truth <= action.upper);
        }
        let learning = evidence.learning.unwrap();
        assert!(learning.clipped.mean.last() < learning.scratch.mean.last());
    }

    #[test]
    fn learning_without_randomised_rows_for_every_action_is_refused() {
        let values = [0.0, 1.0, 0.0, 5.0, 1.0, 0.0, 0.0, 5.0, 0.0, 1.0, 1.0, 3.0];
        let request = Request { actions: vec![0.0, 1.0], learning: Some(Learning { horizon: 10, seeds: vec![1] }) };
        assert!(run(request, &values).is_err());
    }
}
