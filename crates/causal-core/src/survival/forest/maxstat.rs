//! Maximally selected log-rank scores and Lausen p-value approximations.
use super::Split;
use std::f64::consts::{PI, SQRT_2};

pub(super) fn split(
    x: &[Vec<f64>],
    time: &[f64],
    event: &[bool],
    rows: &[usize],
    features: &[usize],
    alpha: f64,
) -> Option<(Split, f64)> {
    let n = rows.len();
    let mut by_time: Vec<usize> = (0..n).collect();
    by_time.sort_by(|&a, &b| time[rows[a]].total_cmp(&time[rows[b]]));
    let mut scores = vec![0.0; n];
    let mut cumulative = 0.0;
    let mut start = 0;
    while start < n {
        let mut end = start + 1;
        while end < n && time[rows[by_time[end]]] == time[rows[by_time[start]]] {
            end += 1;
        }
        for &i in &by_time[start..end] {
            cumulative += f64::from(event[rows[i]]) / (n - end + 1) as f64;
        }
        for &i in &by_time[start..end] {
            scores[i] = f64::from(event[rows[i]]) - cumulative;
        }
        start = end;
    }
    let mut candidates = Vec::new();
    for &column in features {
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by(|&a, &b| x[rows[a]][column].total_cmp(&x[rows[b]][column]));
        let total: f64 = indices.iter().map(|&i| scores[i]).sum();
        let mean = total / n as f64;
        let variance = scores
            .iter()
            .fold(0.0, |sum, s| (s - mean).mul_add(s - mean, sum));
        let minsplit = (n as f64 * 0.1 - 1.0).max(0.0) as usize;
        let maxsplit = (n as f64 * 0.9 - 1.0) as usize;
        let mut sum = 0.0;
        let mut best_score = -1.0;
        let mut best_value = 0.0;
        for i in 0..=maxsplit {
            sum += scores[indices[i]];
            if i < minsplit
                || (i + 1 < n && x[rows[indices[i]]][column] == x[rows[indices[i + 1]]][column])
            {
                continue;
            }
            if x[rows[indices[i]]][column] == x[rows[indices[n - 1]]][column] {
                break;
            }
            let left = (i + 1) as f64;
            let expected = left / n as f64 * total;
            let v = left * (n - i - 1) as f64 / (n * (n - 1)) as f64 * variance;
            let score = ((sum - expected) / v.sqrt()).abs();
            if score > best_score {
                best_score = score;
                best_value = (x[rows[indices[i]]][column] + x[rows[indices[i + 1]]][column]) / 2.0;
            }
        }
        if best_score < 0.0 {
            continue;
        }
        let cuts: Vec<usize> = (0..n - 1)
            .filter_map(|i| {
                (x[rows[indices[i]]][column] != x[rows[indices[i + 1]]][column]).then_some(i + 1)
            })
            .collect();
        let p = if cuts.len() == 1 {
            1.0 + libm::erf(-best_score / SQRT_2)
        } else {
            lausen(best_score, n, &cuts)
        };
        candidates.push((column, best_value, best_score, p));
    }
    let best =
        (0..candidates.len()).min_by(|&a, &b| candidates[a].3.total_cmp(&candidates[b].3))?;
    let mut order: Vec<usize> = (0..candidates.len()).collect();
    order.sort_by(|&a, &b| candidates[b].3.total_cmp(&candidates[a].3));
    let mut adjusted = f64::INFINITY;
    for (rank, &i) in order.iter().enumerate() {
        adjusted = adjusted
            .min(candidates.len() as f64 / (candidates.len() - rank) as f64 * candidates[i].3);
        if i == best {
            break;
        }
    }
    if adjusted > alpha {
        return None;
    }
    let (column, threshold, score, _) = candidates[best];
    Some((Split::Ordered { column, threshold }, score))
}

fn lausen(b: f64, n: usize, cuts: &[usize]) -> f64 {
    let density = (-0.5 * b * b).exp() / (2.0 * PI).sqrt();
    let p92 = if b < 1.0 {
        1.0
    } else {
        (4.0 * density / b + density * (b - 1.0 / b) * (81.0_f64).ln()).max(0.0)
    };
    let mut correction = 0.0;
    for cut in cuts.windows(2) {
        let a = cut[0] as f64;
        let c = cut[1] as f64;
        let n = n as f64;
        let t = (1.0 - a * (n - c) / ((n - a) * c)).sqrt();
        correction +=
            (1.0 / PI) * (-b * b / 2.0).exp() * (t - (b * b / 4.0 - 1.0) * (t * t * t) / 6.0);
    }
    let p94 = 2.0 * (1.0 - 0.5 * (1.0 + libm::erf(b / SQRT_2))) + correction;
    p92.min(p94)
}
