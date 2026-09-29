// Copyright (c) 2024 GRF Contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
// rank_average_treatment_effect.fit, estimate_rate and boot_grf.
use crate::survival::r_rng::RRng;
use std::collections::BTreeMap;
#[derive(Clone, Copy)]
pub enum Target {
    Autoc,
    Qini,
}
pub struct RuleResult {
    pub estimate: f64,
    pub standard_error: f64,
    pub toc: Vec<f64>,
    pub toc_standard_error: Vec<f64>,
}
pub struct RankResult {
    pub rules: Vec<RuleResult>,
    pub difference: Option<RuleResult>,
}
fn estimate(
    scores: &[f64],
    priority: &[f64],
    weights: &[f64],
    rows: &[usize],
    q: &[f64],
    target: Target,
) -> Vec<f64> {
    let mut order = rows.to_vec();
    order.sort_by(|&i, &j| priority[j].partial_cmp(&priority[i]).unwrap());
    let n = order.len();
    let mut sorted = vec![0.0; n];
    let mut first = 0;
    while first < n {
        let mut end = first + 1;
        while end < n && priority[order[first]] == priority[order[end]] {
            end += 1;
        }
        let mut sum = 0.0;
        let mut mass = 0.0;
        for &i in &order[first..end] {
            sum += scores[i] * weights[i];
            mass += weights[i];
        }
        for v in &mut sorted[first..end] {
            *v = sum / mass;
        }
        first = end;
    }
    let mut cw = vec![0.0; n];
    let mut cy = vec![0.0; n];
    let (mut w, mut y) = (0.0, 0.0);
    for (i, &row) in order.iter().enumerate() {
        w += weights[row];
        y += weights[row] * sorted[i];
        cw[i] = w;
        cy[i] = y;
    }
    let ate = y / w;
    let mut rate = 0.0;
    for (i, &row) in order.iter().enumerate() {
        let toc = cy[i] / cw[i] - ate;
        let factor = match target {
            Target::Autoc => 1.0,
            Target::Qini => cw[i] / w,
        };
        rate += factor * weights[row] * toc;
    }
    let mut result = vec![rate / w];
    for &fraction in q {
        let nw = fraction * w;
        let idx = cw.partition_point(|v| *v <= nw + 1e-15);
        let base = idx.max(1) - 1;
        let next = idx.min(n - 1);
        let adj = nw - cw[base];
        result.push((cy[base] + adj * sorted[next]) / (cw[base] + adj) - ate);
    }
    result
}
pub fn fit(
    scores: &[f64],
    priorities: &[Vec<f64>],
    sample_weights: Option<&[f64]>,
    labels: &[usize],
    q: &[f64],
    target: Target,
    replications: usize,
    seed: u32,
) -> Result<RankResult, &'static str> {
    let n = scores.len();
    if n < 2
        || labels.len() != n
        || priorities.is_empty()
        || priorities.len() > 2
        || priorities
            .iter()
            .any(|p| p.len() != n || p.iter().any(|v| !v.is_finite()))
        || scores.iter().any(|v| !v.is_finite())
        || q.is_empty()
        || q.iter().any(|v| !v.is_finite() || *v <= 0.0)
        || q.last() != Some(&1.0)
        || q.windows(2).any(|v| v[0] >= v[1])
        || sample_weights
            .is_some_and(|w| w.len() != n || w.iter().any(|v| !v.is_finite() || *v <= 0.0))
    {
        return Err("Invalid RATE inputs, weights or grid.");
    }
    let mut weights = sample_weights.map(|v| v.to_vec()).unwrap_or(vec![1.0; n]);
    if sample_weights.is_some() {
        let sum: f64 = weights.iter().sum();
        for w in &mut weights {
            *w /= sum;
        }
    }
    let mut groups = BTreeMap::<usize, Vec<usize>>::new();
    for (i, &g) in labels.iter().enumerate() {
        groups.entry(g).or_default().push(i);
    }
    let groups: Vec<_> = groups.into_values().collect();
    let g = groups.len();
    if g < 2 {
        return Err("RATE requires more than one effective unit.");
    }
    let all: Vec<_> = (0..n).collect();
    let calculate = |rows: &[usize]| {
        let mut values: Vec<_> = priorities
            .iter()
            .map(|p| estimate(scores, p, &weights, rows, q, target))
            .collect();
        if values.len() == 2 {
            values.push(
                values[0]
                    .iter()
                    .zip(&values[1])
                    .map(|(a, b)| a - b)
                    .collect(),
            );
        }
        values
    };
    let point = calculate(&all);
    let mut draws = Vec::with_capacity(replications);
    let mut rng = RRng::new(seed);
    for _ in 0..replications {
        let mut pool: Vec<_> = (0..g).collect();
        let mut remaining = g;
        let mut rows = vec![];
        for _ in 0..g / 2 {
            let selected = rng.sample_index(remaining);
            rows.extend_from_slice(&groups[pool[selected]]);
            remaining -= 1;
            pool[selected] = pool[remaining];
        }
        draws.push(calculate(&rows));
    }
    let clean = |v: f64| if v.abs() < 1e-15 { 0.0 } else { v };
    let mut results = Vec::new();
    for (rule, point) in point.iter().enumerate() {
        let mut se = vec![0.0; point.len()];
        if replications >= 2 {
            for j in 0..point.len() {
                let mean = draws.iter().map(|d| d[rule][j]).sum::<f64>() / replications as f64;
                se[j] = clean(
                    (draws
                        .iter()
                        .map(|d| (d[rule][j] - mean).powi(2))
                        .sum::<f64>()
                        / (replications - 1) as f64)
                        .sqrt(),
                );
            }
        }
        results.push(RuleResult {
            estimate: clean(point[0]),
            standard_error: se[0],
            toc: point[1..].iter().map(|v| clean(*v)).collect(),
            toc_standard_error: se[1..].to_vec(),
        });
    }
    let difference = if results.len() == 3 {
        results.pop()
    } else {
        None
    };
    Ok(RankResult {
        rules: results,
        difference,
    })
}
