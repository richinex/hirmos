//! Clustered event increments and sandwich covariance for aareg.
use super::{Data, Error, Fit, Options, Qr, RiskSet, TestWeight};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Influence {
    /// Unique event time, cluster in first-appearance order, coefficient.
    pub increments: Vec<Vec<Vec<f64>>>,
    pub variance: Vec<Vec<f64>>,
}

fn group_indices(groups: impl Iterator<Item = usize>) -> (Vec<usize>, usize) {
    let mut indices = HashMap::new();
    let rows = groups
        .map(|id| {
            let next = indices.len();
            *indices.entry(id).or_insert(next)
        })
        .collect();
    (rows, indices.len())
}

pub(super) fn calculate(
    data: &Data,
    options: &Options,
    order: &[usize],
    sets: &[RiskSet],
    fit: &Fit,
) -> Result<Influence, Error> {
    let n = data.x.len();
    let p = data.x[0].len();
    let groups: Vec<usize> = data.clusters.clone().unwrap_or_else(|| (0..n).collect());
    let (sorted_groups, count) = group_indices(order.iter().map(|&row| groups[row]));
    let (original_groups, _) = group_indices(groups.into_iter());
    let mut increments = vec![vec![vec![0.0; p + 1]; count]; sets.len()];
    let mut tested = vec![vec![0.0; p + 1]; n];
    let mut event = 0;
    for (time, set) in sets.iter().enumerate() {
        let qr = Qr::new(set.covariance.clone(), p, options.tolerance);
        for &death in &set.deaths {
            let coef = &fit.coefficient[event];
            for (position, &row) in order.iter().enumerate() {
                let at_risk =
                    data.intervals[row][0] < set.time && data.intervals[row][1] >= set.time;
                if !at_risk {
                    continue;
                }
                let centered: Vec<f64> = (0..p).map(|j| data.x[row][j] - set.mean[j]).collect();
                let predicted = if p == 1 {
                    coef[1] * data.x[row][0] + coef[0]
                } else {
                    data.weights[death] / set.weight
                        + centered
                            .iter()
                            .zip(&coef[1..])
                            .map(|(x, b)| x * b)
                            .sum::<f64>()
                };
                let residual = f64::from(row == death) - predicted;
                let rhs: Vec<f64> = centered
                    .iter()
                    .map(|x| residual * data.weights[row] * x)
                    .collect();
                let mut value = vec![0.0; p + 1];
                let solution = qr.solve(&rhs).ok_or(Error::SingularRiskSet)?;
                for j in 0..p {
                    value[j + 1] = solution[j] / set.weight;
                }
                value[0] = residual * data.weights[row] / set.weight
                    - value[1..]
                        .iter()
                        .zip(&set.mean)
                        .map(|(v, m)| v * m)
                        .sum::<f64>();
                for j in 0..=p {
                    increments[time][sorted_groups[position]][j] += value[j];
                    let weight = match options.test {
                        TestWeight::Aalen => fit.tweight[event][j],
                        TestWeight::AtRisk => set.weight,
                    };
                    tested[position][j] += value[j] * weight;
                }
            }
            event += 1;
        }
    }
    let mut grouped = vec![vec![0.0; p + 1]; count];
    // aareg aggregates this sorted-row matrix using the original cluster vector.
    // Preserve that source behavior separately from the correctly reordered increments.
    for (position, row) in tested.iter().enumerate() {
        for j in 0..=p {
            grouped[original_groups[position]][j] += row[j];
        }
    }
    let mut variance = vec![vec![0.0; p + 1]; p + 1];
    for row in grouped {
        for i in 0..=p {
            for j in 0..=p {
                variance[i][j] += row[i] * row[j];
            }
        }
    }
    Ok(Influence {
        increments,
        variance,
    })
}
