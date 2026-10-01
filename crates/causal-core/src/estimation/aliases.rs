//! Normalized column-pivoted Householder rank selection used by estimatr 2.0.0.
//! Kept separate from coefficient fitting and covariance estimation.
//! Algorithm reference: Eigen ColPivHouseholderQR (MPL-2.0),
//! Copyright (C) 2008-2009 Gael Guennebaud, 2006-2009 Benoit Jacob.
//! This Source Code Form is subject to the Mozilla Public License, v. 2.0.
//! Obtain a copy at https://mozilla.org/MPL/2.0/.
use nalgebra::DMatrix;

// Emulate the pinned ARM64 Eigen two-packet reduction, including contracted
// multiply-adds. A serial sum can change pivot ties after normalization.
fn squared_norm(v: &[f64]) -> f64 {
    if v.len() < 4 {
        return v.iter().fold(0., |s, x| x.mul_add(*x, s));
    }
    let end = v.len() / 4 * 4;
    let mut a = v[0] * v[0];
    let mut b = v[1] * v[1];
    let mut c = v[2] * v[2];
    let mut d = v[3] * v[3];
    for i in (4..end).step_by(4) {
        a = v[i].mul_add(v[i], a);
        b = v[i + 1].mul_add(v[i + 1], b);
        c = v[i + 2].mul_add(v[i + 2], c);
        d = v[i + 3].mul_add(v[i + 3], d);
    }
    a += c;
    b += d;
    let mut last = end;
    if v.len() - end >= 2 {
        a = v[end].mul_add(v[end], a);
        b = v[end + 1].mul_add(v[end + 1], b);
        last += 2;
    }
    v[last..].iter().fold(a + b, |s, x| x.mul_add(*x, s))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnStatus {
    Retained { coefficient: usize },
    Aliased,
}
#[derive(Debug)]
pub struct Selection {
    columns: Vec<ColumnStatus>,
    kept: Vec<usize>,
}
impl Selection {
    pub fn columns(&self) -> &[ColumnStatus] {
        &self.columns
    }
    pub fn kept(&self) -> &[usize] {
        &self.kept
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Empty,
    Shape,
    InvalidWeights,
    NonFinite,
    Numerical,
}

/// Same row sorting and square-root weight normalization as estimatr's clustered
/// regression preparation. Zero-weight rows remain in QR, as in the reference.
pub fn select_clustered(
    x: &DMatrix<f64>,
    weights: &[f64],
    clusters: &[u64],
) -> Result<Selection, Error> {
    let (n, p) = x.shape();
    if n == 0 || p == 0 {
        return Err(Error::Empty);
    }
    if weights.len() != n || clusters.len() != n {
        return Err(Error::Shape);
    }
    if x.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    if weights.iter().any(|w| !w.is_finite() || *w < 0.) {
        return Err(Error::InvalidWeights);
    }
    let mean = weights.iter().sum::<f64>() / n as f64;
    if !mean.is_finite() || mean <= 0. {
        return Err(Error::InvalidWeights);
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|i| clusters[*i]);
    let weighted = DMatrix::from_fn(n, p, |i, j| {
        x[(order[i], j)] * (weights[order[i]] / mean).sqrt()
    });
    select(&weighted)
}

/// Input is the weighted design in the reference's row order. The 1e-7 rank
/// threshold is the oracle's convention, not the SVD OLS threshold.
pub fn select(weighted: &DMatrix<f64>) -> Result<Selection, Error> {
    let (n, p) = weighted.shape();
    if n == 0 || p == 0 {
        return Err(Error::Empty);
    }
    if weighted.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let mut a = weighted.clone();
    for j in 0..p {
        let norm = squared_norm(a.column(j).as_slice()).sqrt();
        if !norm.is_finite() {
            return Err(Error::Numerical);
        }
        let scale = if norm > 0. { 1. / norm } else { 1. };
        for i in 0..n {
            a[(i, j)] *= scale;
        }
    }
    let mut norms: Vec<f64> = (0..p)
        .map(|j| squared_norm(a.column(j).as_slice()).sqrt())
        .collect();
    let mut direct = norms.clone();
    let mut permutation: Vec<usize> = (0..p).collect();
    let mut diagonal = Vec::new();
    for k in 0..n.min(p) {
        let mut pivot = k;
        for j in k + 1..p {
            if norms[j] > norms[pivot] {
                pivot = j;
            }
        }
        a.swap_columns(k, pivot);
        norms.swap(k, pivot);
        direct.swap(k, pivot);
        permutation.swap(k, pivot);
        let c0 = a[(k, k)];
        let tail = squared_norm(&a.column(k).as_slice()[k + 1..]);
        let (beta, tau) = if tail <= f64::MIN_POSITIVE {
            (c0, 0.)
        } else {
            let norm = (c0 * c0 + tail).sqrt();
            let beta = if c0 >= 0. { -norm } else { norm };
            for i in k + 1..n {
                a[(i, k)] /= c0 - beta;
            }
            (beta, (beta - c0) / beta)
        };
        a[(k, k)] = beta;
        diagonal.push(beta.abs());
        for j in k + 1..p {
            let dot = a[(k, j)] + (k + 1..n).map(|i| a[(i, k)] * a[(i, j)]).sum::<f64>();
            a[(k, j)] -= tau * dot;
            for i in k + 1..n {
                a[(i, j)] -= tau * a[(i, k)] * dot;
            }
            if norms[j] != 0. {
                let ratio = a[(k, j)].abs() / norms[j];
                let temp = ((1. + ratio) * (1. - ratio)).max(0.);
                let accuracy = temp * (norms[j] / direct[j]).powi(2);
                if accuracy <= f64::EPSILON.sqrt() {
                    direct[j] = squared_norm(&a.column(j).as_slice()[k + 1..]).sqrt();
                    norms[j] = direct[j];
                } else {
                    norms[j] *= temp.sqrt();
                }
            }
        }
    }
    if a.iter().any(|v| !v.is_finite()) {
        return Err(Error::Numerical);
    }
    let cutoff = diagonal.iter().copied().fold(0., f64::max) * 1e-7;
    let rank = diagonal.iter().filter(|v| **v > cutoff).count();
    let mut kept = permutation[..rank].to_vec();
    kept.sort_unstable();
    let mut columns = vec![ColumnStatus::Aliased; p];
    for (coefficient, &j) in kept.iter().enumerate() {
        columns[j] = ColumnStatus::Retained { coefficient };
    }
    Ok(Selection { columns, kept })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eigen_initial_norms() {
        let f: serde_json::Value = serde_json::from_str(include_str!(
            "../../oracle/fixtures/estimatr_rank_probe.json"
        ))
        .unwrap();
        let rows: Vec<Vec<f64>> = serde_json::from_value(f["x"].clone()).unwrap();
        let scales: Vec<f64> = serde_json::from_value(f["probe"]["scales"].clone()).unwrap();
        let mut misses = Vec::new();
        for j in 0..scales.len() {
            let v: Vec<f64> = rows.iter().map(|r| r[j]).collect();
            let actual = 1. / squared_norm(&v).sqrt();
            if actual != scales[j] {
                misses.push((j, actual, scales[j]));
            }
        }
        assert!(misses.is_empty(), "norm mismatch: {misses:?}");
    }
}
