//! Numerical redundancy diagnostics: correlation, complete-linkage clustering, and VIF.

use nalgebra::{DMatrix, DVector};

/// NumPy-compatible Pearson correlation matrix over columns.
pub fn correlation_matrix(arr: &DMatrix<f64>) -> DMatrix<f64> {
    let rows = arr.nrows();
    let columns = arr.ncols();
    let mut centered = arr.clone();
    for mut column in centered.column_iter_mut() {
        let mean = column.iter().sum::<f64>() / rows as f64;
        for value in column.iter_mut() {
            *value -= mean;
        }
    }
    let covariance = centered.transpose() * &centered / (rows as f64 - 1.0);
    let scales: Vec<f64> = (0..columns)
        .map(|column| covariance[(column, column)].sqrt())
        .collect();
    DMatrix::from_fn(columns, columns, |row, column| {
        (covariance[(row, column)] / scales[row] / scales[column]).clamp(-1.0, 1.0)
    })
}

/// Complete-linkage clustering of 1-|r| cut at 1-threshold: keep the lowest index per cluster.
pub fn cluster_redundant(
    corr: &DMatrix<f64>,
    threshold: f64,
) -> (Vec<usize>, Vec<usize>, Vec<Vec<usize>>) {
    let n = corr.nrows();
    let mut dist = DMatrix::<f64>::zeros(n, n);
    for i in 0..n {
        for j in 0..n {
            if i != j {
                let d = 1.0 - corr[(i, j)].abs();
                let dt = 1.0 - corr[(j, i)].abs();
                dist[(i, j)] = ((d + dt) / 2.0).max(0.0);
            }
        }
    }
    let cut = 1.0 - threshold;
    let mut clusters: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();
    loop {
        let mut best: Option<(usize, usize, f64)> = None;
        for i in 0..clusters.len() {
            for j in i + 1..clusters.len() {
                let mut link = f64::NEG_INFINITY;
                for &a in &clusters[i] {
                    for &b in &clusters[j] {
                        link = link.max(dist[(a, b)]);
                    }
                }
                if best.is_none_or(|(_, _, d)| link < d) {
                    best = Some((i, j, link));
                }
            }
        }
        match best {
            Some((i, j, d)) if d <= cut => {
                let merged = clusters.remove(j);
                clusters[i].extend(merged);
            }
            _ => break,
        }
    }
    let mut cluster_lists: Vec<Vec<usize>> = clusters
        .into_iter()
        .map(|mut c| {
            c.sort_unstable();
            c
        })
        .collect();
    cluster_lists.sort_by_key(|c| c[0]);
    let mut keep = Vec::new();
    let mut drop = Vec::new();
    for members in &cluster_lists {
        keep.push(members[0]);
        drop.extend(members[1..].iter().copied());
    }
    keep.sort_unstable();
    drop.sort_unstable();
    (keep, drop, cluster_lists)
}

fn rsquared(y: &DVector<f64>, x: &DMatrix<f64>) -> f64 {
    // statsmodels OLS uses its Moore-Penrose pseudo-inverse by default. This remains defined for
    // the exact-collinearity cases that VIF is specifically meant to expose.
    let ssr = crate::ols::Ols::fit(x, y).ssr;
    let mean = y.iter().sum::<f64>() / y.len() as f64;
    let tss: f64 = y.iter().map(|v| (v - mean) * (v - mean)).sum();
    1.0 - ssr / tss
}

/// Iterative VIF elimination: drop the worst above the threshold, higher index on ties.
pub fn vif_redundant(
    arr: &DMatrix<f64>,
    vif_threshold: f64,
) -> (Vec<usize>, Vec<usize>, Vec<(usize, f64)>) {
    let n = arr.ncols();
    let rows = arr.nrows();
    let mut keep: Vec<usize> = (0..n).collect();
    let mut dropped = Vec::new();
    let mut history = Vec::new();
    while keep.len() >= 2 {
        let k = keep.len();
        let mut vifs = Vec::with_capacity(k);
        for p in 0..k {
            let y = DVector::from_iterator(rows, (0..rows).map(|r| arr[(r, keep[p])]));
            let mut design = DMatrix::<f64>::zeros(rows, k);
            for r in 0..rows {
                design[(r, 0)] = 1.0;
                let mut c = 1;
                for (q, &col) in keep.iter().enumerate() {
                    if q != p {
                        design[(r, c)] = arr[(r, col)];
                        c += 1;
                    }
                }
            }
            let v = 1.0 / (1.0 - rsquared(&y, &design));
            vifs.push(if v.is_finite() { v } else { f64::INFINITY });
        }
        let mut worst = 0usize;
        for p in 1..k {
            if (vifs[p], keep[p]) > (vifs[worst], keep[worst]) {
                worst = p;
            }
        }
        if vifs[worst] < vif_threshold {
            break;
        }
        history.push((keep[worst], vifs[worst]));
        dropped.push(keep[worst]);
        keep.remove(worst);
    }
    dropped.sort_unstable();
    (keep, dropped, history)
}
