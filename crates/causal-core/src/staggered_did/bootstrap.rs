//! did::mboot with BMisc 1.4.10's packed Rademacher stream.
use crate::survival::r_rng::RRng;
use nalgebra::DMatrix;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    InvalidIterations,
    InvalidAlpha,
}

pub struct Bootstrap {
    /// Columns retain input order, including degenerate dimensions.
    pub draws: DMatrix<f64>,
    pub se: Vec<Option<f64>>,
    pub critical: Option<f64>,
}

fn quantile(sorted: &[f64], probability: f64) -> f64 {
    sorted[((sorted.len() as f64 * probability).ceil() as usize)
        .saturating_sub(1)
        .min(sorted.len() - 1)]
}

/// Unique cluster labels give unit-level clustering; repeated labels aggregate
/// cluster sums. Labels must be aligned with influence-function rows.
pub fn run(
    influence: &DMatrix<f64>,
    clusters: &[u64],
    iterations: usize,
    alpha: f64,
    seed: u32,
) -> Result<Bootstrap, Error> {
    let n = influence.nrows();
    let k = influence.ncols();
    if n == 0 || k == 0 || clusters.len() != n {
        return Err(Error::Shape);
    }
    if iterations == 0 {
        return Err(Error::InvalidIterations);
    }
    if !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
        return Err(Error::InvalidAlpha);
    }
    if influence.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let mut sums = BTreeMap::new();
    for i in 0..n {
        let row = sums.entry(clusters[i]).or_insert_with(|| vec![0.0; k]);
        for j in 0..k {
            row[j] += influence[(i, j)];
        }
    }
    let groups: Vec<_> = sums.into_values().collect();
    let g = groups.len();
    let mut rng = RRng::new(seed);
    let mut draws = DMatrix::zeros(iterations, k);
    let mut signs = vec![0.0; g];
    for b in 0..iterations {
        for chunk in signs.chunks_mut(31) {
            // Rcpp's size-one sample uses floor(n*unif_rand())+1, not base R's
            // rejection-based sample.int. BMisc unpacks each word high bit first.
            let u = rng.uniform().max(0.5 / 4294967296.0);
            let word = (2147483647.0 * u) as u32 + 1;
            for (j, s) in chunk.iter_mut().enumerate() {
                *s = if (word >> (30 - j)) & 1 == 1 {
                    1.0
                } else {
                    -1.0
                };
            }
        }
        for j in 0..k {
            draws[(b, j)] = groups
                .iter()
                .zip(&signs)
                .map(|(row, s)| row[j] * s)
                .sum::<f64>()
                / (g as f64).sqrt();
        }
    }
    if draws.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let mut scales = vec![None; k];
    for (j, scale) in scales.iter_mut().enumerate() {
        if draws.column(j).iter().map(|v| v * v).sum::<f64>() > f64::EPSILON.sqrt() * 10.0 {
            let mut column: Vec<_> = draws.column(j).iter().copied().collect();
            column.sort_by(f64::total_cmp);
            *scale = Some((quantile(&column, 0.75) - quantile(&column, 0.25)) / 1.3489795003921634);
        }
    }
    let mut maxima = Vec::new();
    for b in 0..iterations {
        let mut largest = f64::NEG_INFINITY;
        for (j, s) in scales.iter().enumerate() {
            if let Some(s) = s {
                let value = (draws[(b, j)] / s).abs();
                if !value.is_nan() {
                    largest = largest.max(value);
                }
            }
        }
        if largest.is_finite() {
            maxima.push(largest);
        }
    }
    maxima.sort_by(f64::total_cmp);
    let critical = if maxima.is_empty() {
        None
    } else {
        Some(quantile(&maxima, 1.0 - alpha))
    };
    let se = scales
        .iter()
        .map(|s| s.map(|s| s * (g as f64).sqrt() / n as f64))
        .collect();
    Ok(Bootstrap {
        draws,
        se,
        critical,
    })
}
