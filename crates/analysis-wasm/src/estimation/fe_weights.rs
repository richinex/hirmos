use crate::protocol::*;
use hirmos_causal_core::fe_weights::{self, DropoutFit, OneWaySample};

pub(super) fn calculate(x: &[f64], y: &[f64], groups: &[u64], labels: &[f64]) -> FeWeightEvidence {
    match calculate_available(x, y, groups, labels) {
        Ok(result) => result,
        Err(reason) => FeWeightEvidence::Unavailable { reason },
    }
}

fn calculate_available(x: &[f64], y: &[f64], groups: &[u64], labels: &[f64]) -> Result<FeWeightEvidence, String> {
    let sample = OneWaySample::new(x, y, groups).map_err(problem)?;
    let diagnostic = sample.diagnostics().map_err(problem)?;
    let counts = [0, 1, 2, 3, 4, 5, 10].into_iter().filter(|&k| k <= diagnostic.groups.len() - 2).collect::<Vec<_>>();
    let dropout = sample.dropout(&counts).map_err(problem)?.into_iter().map(|point| {
        let fit = match point.fit {
            DropoutFit::Estimated { coefficient, .. } => FeDropoutFit::Estimated { coefficient },
            DropoutFit::Unavailable(error) => FeDropoutFit::Unavailable { reason: problem(error) },
        };
        FeDropoutPoint {
            removed_groups: point.removed_groups,
            original_weight_removed: point.original_weight_removed,
            remaining_observations: point.remaining_observations,
            fit,
        }
    }).collect();
    let variances = diagnostic.groups.iter().map(|g| g.variance).collect::<Vec<_>>();
    let min = variances.iter().copied().fold(f64::INFINITY, f64::min);
    let max = variances.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let out_of_range = |_| "The variance density exceeds the supported numerical range.".to_owned();
    let bandwidth = fe_weights::density::bandwidth(&variances).map_err(out_of_range)?;
    let density = fe_weights::density::gaussian(&variances, min - 3.0 * bandwidth, max + 3.0 * bandwidth)
        .map_err(out_of_range)?;
    let c = diagnostic.concentration;
    Ok(FeWeightEvidence::Available {
        observations: diagnostic.observations,
        groups: diagnostic.groups.into_iter().map(|g| FeWeightGroup {
            group: g.group,
            label: labels[g.group as usize],
            observations: g.observations,
            variance: g.variance,
            sum_squares: g.sum_squares,
            weight: g.weight,
            relative_weight: g.relative_weight,
            rank_share: g.rank_share,
            cumulative_weight: g.cumulative_weight,
        }).collect(),
        gini_variance: c.gini_variance,
        maximum_weight: c.maximum_weight,
        effective_groups: c.effective_groups,
        top_five_share: c.top_five_share,
        top_ten_share: c.top_ten_share,
        top_decile_share: c.top_decile_share,
        bandwidth: density.bandwidth,
        density: density.points,
        dropout,
    })
}

fn problem(error: fe_weights::Error) -> String {
    use fe_weights::Error::*;
    match error {
        NoWithinVariation => "No treatment variation remains within the retained groups.",
        NumericalRange => "The group calculations exceed the supported numerical range.",
        TooFewGroups | InvalidDropCount => "The diagnostic requires at least two retained groups.",
        EmptySample | LengthMismatch | NonFiniteInput => "The diagnostic requires aligned, finite values from the fitted sample.",
        Cancelled => "The diagnostic was cancelled.",
    }.to_owned()
}
