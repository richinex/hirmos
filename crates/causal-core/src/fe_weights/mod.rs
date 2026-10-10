//! Identifying variation for a single regressor with one absorbed grouping.

use std::collections::BTreeMap;

pub mod density;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    EmptySample,
    LengthMismatch,
    NonFiniteInput,
    TooFewGroups,
    NoWithinVariation,
    NumericalRange,
    InvalidDropCount,
    Cancelled,
}

#[derive(Clone, Debug)]
pub struct GroupVariation {
    pub group: u64,
    pub observations: usize,
    pub variance: f64,
    pub sum_squares: f64,
    pub weight: f64,
    pub relative_weight: f64,
    pub rank_share: f64,
    pub cumulative_weight: f64,
}

#[derive(Clone, Debug)]
pub struct Concentration {
    pub gini_variance: f64,
    pub maximum_weight: f64,
    pub effective_groups: f64,
    pub top_five_share: f64,
    pub top_ten_share: f64,
    pub top_decile_share: f64,
}

#[derive(Clone, Debug)]
pub struct Diagnostics {
    pub observations: usize,
    /// Descending identifying weight, then ascending group ID for ties.
    pub groups: Vec<GroupVariation>,
    pub concentration: Concentration,
}

#[derive(Clone, Debug)]
pub enum DropoutFit {
    Estimated {
        coefficient: f64,
        diagnostics: Diagnostics,
    },
    Unavailable(Error),
}

#[derive(Clone, Debug)]
pub struct DropoutPoint {
    pub removed_groups: Vec<u64>,
    pub original_weight_removed: f64,
    pub remaining_observations: usize,
    pub fit: DropoutFit,
}

struct Moments {
    group: u64,
    count: usize,
    sum_squares: f64,
    cross_product: f64,
}

pub struct OneWaySample {
    groups: Vec<Moments>,
}

impl OneWaySample {
    pub fn new(x: &[f64], y: &[f64], groups: &[u64]) -> Result<Self, Error> {
        if x.len() != y.len() || x.len() != groups.len() {
            return Err(Error::LengthMismatch);
        }
        if x.is_empty() {
            return Err(Error::EmptySample);
        }
        if x.iter().chain(y).any(|v| !v.is_finite()) {
            return Err(Error::NonFiniteInput);
        }
        let mut rows = BTreeMap::<u64, Vec<usize>>::new();
        for (row, group) in groups.iter().enumerate() {
            rows.entry(*group).or_default().push(row);
        }
        if rows.len() < 2 {
            return Err(Error::TooFewGroups);
        }
        let mut moments = Vec::with_capacity(rows.len());
        for (group, indices) in rows {
            let count = indices.len();
            let first = indices[0];
            let mean_x = indices.iter().map(|&i| x[i] - x[first]).sum::<f64>() / count as f64;
            let mean_y = indices.iter().map(|&i| y[i] - y[first]).sum::<f64>() / count as f64;
            let mut sum_squares = 0.0;
            let mut cross_product = 0.0;
            for &i in &indices {
                let dx = (x[i] - x[first]) - mean_x;
                let dy = (y[i] - y[first]) - mean_y;
                sum_squares += dx * dx;
                cross_product += dx * dy;
            }
            if !sum_squares.is_finite()
                || !cross_product.is_finite()
                || (sum_squares == 0.0 && indices.iter().any(|&i| x[i] != x[first]))
            {
                return Err(Error::NumericalRange);
            }
            moments.push(Moments {
                group,
                count,
                sum_squares,
                cross_product,
            });
        }
        Ok(Self { groups: moments })
    }

    pub fn diagnostics(&self) -> Result<Diagnostics, Error> {
        summarize(&self.groups.iter().collect::<Vec<_>>())
    }

    /// Remove prefixes of the original ranking; retain at least two groups.
    pub fn dropout(&self, counts: &[usize]) -> Result<Vec<DropoutPoint>, Error> {
        self.dropout_with_cancel(counts, || false)
    }

    pub fn dropout_with_cancel(
        &self,
        counts: &[usize],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Vec<DropoutPoint>, Error> {
        if cancelled() {
            return Err(Error::Cancelled);
        }
        let original = self.diagnostics()?;
        if counts.iter().any(|&k| k > self.groups.len() - 2) {
            return Err(Error::InvalidDropCount);
        }
        counts
            .iter()
            .map(|&k| {
                if cancelled() {
                    return Err(Error::Cancelled);
                }
                let removed_groups = original.groups[..k]
                    .iter()
                    .map(|g| g.group)
                    .collect::<Vec<_>>();
                let retained = self
                    .groups
                    .iter()
                    .filter(|g| !removed_groups.contains(&g.group))
                    .collect::<Vec<_>>();
                let remaining_observations = retained.iter().map(|g| g.count).sum();
                let fit = match summarize(&retained) {
                    Ok(diagnostics) => {
                        // For this bounded design, within-OLS equals the ratio of pooled cross-products.
                        let numerator = retained.iter().map(|g| g.cross_product).sum::<f64>();
                        let denominator = retained.iter().map(|g| g.sum_squares).sum::<f64>();
                        let coefficient = numerator / denominator;
                        if coefficient.is_finite() {
                            DropoutFit::Estimated {
                                coefficient,
                                diagnostics,
                            }
                        } else {
                            DropoutFit::Unavailable(Error::NumericalRange)
                        }
                    }
                    Err(problem) => DropoutFit::Unavailable(problem),
                };
                Ok(DropoutPoint {
                    removed_groups,
                    original_weight_removed: original.groups[..k].iter().map(|g| g.weight).sum(),
                    remaining_observations,
                    fit,
                })
            })
            .collect()
    }
}

fn summarize(moments: &[&Moments]) -> Result<Diagnostics, Error> {
    let total = moments.iter().map(|g| g.sum_squares).sum::<f64>();
    if !total.is_finite() {
        return Err(Error::NumericalRange);
    }
    if total == 0.0 {
        return Err(Error::NoWithinVariation);
    }
    let count = moments.len();
    let mut groups = moments
        .iter()
        .map(|g| GroupVariation {
            group: g.group,
            observations: g.count,
            variance: g.sum_squares / g.count as f64,
            sum_squares: g.sum_squares,
            weight: g.sum_squares / total,
            relative_weight: count as f64 * (g.sum_squares / total),
            rank_share: 0.0,
            cumulative_weight: 0.0,
        })
        .collect::<Vec<_>>();
    groups.sort_by(|a, b| b.weight.total_cmp(&a.weight).then(a.group.cmp(&b.group)));
    let mut cumulative = 0.0;
    for (index, group) in groups.iter_mut().enumerate() {
        cumulative += group.weight;
        group.cumulative_weight = cumulative;
        group.rank_share = (index + 1) as f64 / count as f64;
    }
    let mut variances = groups.iter().map(|g| g.variance).collect::<Vec<_>>();
    variances.sort_by(f64::total_cmp);
    let variance_scale = *variances.last().unwrap();
    if variance_scale == 0.0 {
        return Err(Error::NumericalRange);
    }
    let scaled_sum = variances.iter().map(|v| v / variance_scale).sum::<f64>();
    let weighted_sum = variances
        .iter()
        .enumerate()
        .map(|(i, v)| (i + 1) as f64 * (v / variance_scale))
        .sum::<f64>();
    let top = |n| groups.iter().take(n).map(|g| g.weight).sum();
    let concentration = Concentration {
        gini_variance: 2.0 * weighted_sum / (count as f64 * scaled_sum)
            - (count + 1) as f64 / count as f64,
        maximum_weight: groups[0].weight,
        effective_groups: 1.0 / groups.iter().map(|g| g.weight * g.weight).sum::<f64>(),
        top_five_share: top(5),
        top_ten_share: top(10),
        top_decile_share: top(count.div_ceil(10)),
    };
    Ok(Diagnostics {
        observations: moments.iter().map(|g| g.count).sum(),
        groups,
        concentration,
    })
}
