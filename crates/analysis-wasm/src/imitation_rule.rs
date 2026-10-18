//! Browser boundary for copying a demonstrator's rule from the variables an action may read.
use hirmos_causal_core::reinforcement::behaviour_cloning::{clone_continuous, clone_discrete, CloningSettings, Demonstrations};
use hirmos_causal_core::nprandom::NpRng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub(crate) enum Actions {
    Discrete { count: usize, chosen: Vec<usize> },
    Continuous { width: usize, chosen: Vec<f64>, bounds: Option<Vec<(f64, f64)>> },
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Settings {
    hidden: usize,
    learning_rate: f64,
    epochs: usize,
    batch_size: usize,
    validation_share: f64,
    patience: usize,
    seed: u64,
}

/// Train on this many demonstrations drawn without replacement by a generator seeded with `seed`.
#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Sample {
    rows: usize,
    seed: u64,
}

/// One rule: `values` holds one row of `width` inputs per demonstration; `probes` are the input rows to evaluate.
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    width: usize,
    actions: Actions,
    probes: Vec<Vec<f64>>,
    settings: Settings,
    #[serde(default)]
    sample: Option<Sample>,
    /// Rows kept back for scoring the copy; the copy never trains on them.
    #[serde(default)]
    holdout: Option<crate::holdout::Holdout>,
}

/// The demonstrations to train on, in drawn order, or every one when the sample covers them all.
fn drawn(demonstrations: usize, sample: Option<Sample>) -> Option<Vec<usize>> {
    let sample = sample.filter(|s| s.rows < demonstrations)?;
    let mut order = NpRng::seeded(sample.seed).permutation(demonstrations);
    order.truncate(sample.rows);
    Some(order)
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum Evidence {
    /// Action probabilities for each probe.
    Discrete { probabilities: Vec<Vec<f64>> },
    /// The chosen action for each probe.
    Continuous { actions: Vec<Vec<f64>> },
}

pub(crate) fn run(request: Request, values: &[f64]) -> Result<Evidence, String> {
    let width = request.width;
    let examples = match &request.actions {
        Actions::Discrete { chosen, .. } => chosen.len(),
        Actions::Continuous { width: action_width, chosen, .. } => chosen.len() / (*action_width).max(1),
    };
    if values.len() != examples * width {
        return Err(format!("expected {} input values for {examples} demonstrations, found {}", examples * width, values.len()));
    }
    let candidates: Option<Vec<usize>> = match request.holdout {
        None => None,
        Some(holdout) => Some(holdout.mask(examples)?.iter().enumerate().filter(|(_, &held)| !held).map(|(row, _)| row).collect()),
    };
    let rows = match candidates {
        None => drawn(examples, request.sample),
        Some(candidates) => Some(match drawn(candidates.len(), request.sample) {
            None => candidates,
            Some(order) => order.into_iter().map(|i| candidates[i]).collect(),
        }),
    };
    let inputs: Vec<f32> = match &rows {
        None => values.iter().map(|&v| v as f32).collect(),
        Some(rows) => rows.iter().flat_map(|&r| values[r * width..(r + 1) * width].iter().map(|&v| v as f32)).collect(),
    };
    let demonstrations = Demonstrations { inputs: &inputs, width };
    let s = request.settings;
    let settings = CloningSettings {
        hidden: s.hidden,
        learning_rate: s.learning_rate,
        epochs: s.epochs,
        batch_size: s.batch_size,
        validation_share: s.validation_share,
        patience: s.patience,
        seed: s.seed,
    };
    if request.probes.iter().any(|p| p.len() != request.width) {
        return Err(format!("every probe needs {} inputs", request.width));
    }
    let probe = |p: &Vec<f64>| p.iter().map(|&v| v as f32).collect::<Vec<f32>>();
    match request.actions {
        Actions::Discrete { count, chosen } => {
            let chosen: Vec<usize> = match &rows {
                None => chosen,
                Some(rows) => rows.iter().map(|&r| chosen[r]).collect(),
            };
            let rule = clone_discrete(demonstrations, &chosen, count, &settings).map_err(|e| e.to_string())?;
            Ok(Evidence::Discrete { probabilities: request.probes.iter().map(|p| rule.probabilities(&probe(p))).collect() })
        }
        Actions::Continuous { width, chosen, bounds } => {
            let targets: Vec<f32> = match &rows {
                None => chosen.iter().map(|&v| v as f32).collect(),
                Some(rows) => rows.iter().flat_map(|&r| chosen[r * width..(r + 1) * width].iter().map(|&v| v as f32)).collect(),
            };
            let rule = clone_continuous(demonstrations, &targets, width, bounds, &settings).map_err(|e| e.to_string())?;
            Ok(Evidence::Continuous { actions: request.probes.iter().map(|p| rule.act(&probe(p))).collect() })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> Settings {
        Settings { hidden: 16, learning_rate: 1e-2, epochs: 60, batch_size: 64, validation_share: 0.2, patience: 10, seed: 3 }
    }

    #[test]
    fn a_rule_trained_on_a_sample_still_copies_a_deterministic_demonstrator() {
        let inputs: Vec<f64> = (0..2_000).map(|i| (i % 2) as f64).collect();
        let chosen: Vec<usize> = (0..2_000).map(|i| i % 2).collect();
        let request = Request {
            width: 1,
            actions: Actions::Discrete { count: 2, chosen },
            probes: vec![vec![0.0], vec![1.0]],
            settings: settings(),
            sample: Some(Sample { rows: 400, seed: 7 }),
            holdout: None,
        };
        let Evidence::Discrete { probabilities } = run(request, &inputs).unwrap() else { panic!("discrete") };
        assert!(probabilities[0][0] > 0.9 && probabilities[1][1] > 0.9, "{probabilities:?}");
    }

    #[test]
    fn a_rule_that_reads_nothing_returns_the_sampled_action_frequencies() {
        let chosen: Vec<usize> = (0..1_000).map(|i| usize::from(i < 250)).collect();
        let request = Request {
            width: 0,
            actions: Actions::Discrete { count: 2, chosen: chosen.clone() },
            probes: vec![vec![]],
            settings: settings(),
            sample: Some(Sample { rows: 200, seed: 11 }),
            holdout: None,
        };
        let Evidence::Discrete { probabilities } = run(request, &[]).unwrap() else { panic!("discrete") };
        let order = drawn(1_000, Some(Sample { rows: 200, seed: 11 })).unwrap();
        let ones = order.iter().filter(|&&r| chosen[r] == 1).count() as f64 / 200.0;
        assert_eq!(probabilities[0][1], ones);
    }
}
