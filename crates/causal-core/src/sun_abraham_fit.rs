//! Reuse audit: the production Hirmos fitter, not another regression engine.
use crate::estimation::{self, WithinErrors};
use nalgebra::{DMatrix, DVector};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use crate::sun_abraham::{self as sa, Cell};

#[derive(Debug)]
pub enum Error {
    Design(sa::Error),
    Regression(estimation::WithinProblem),
    Shape,
    DegreesOfFreedom,
    Singular,
    DuplicatePanelKey,
    InconsistentCohort,
}
pub struct Fit {
    pub cells: Vec<Cell>,
    pub dropped_cells: Vec<Cell>,
    pub coefficients: Vec<f64>,
    pub covariance: DMatrix<f64>,
    pub support: Vec<f64>,
    pub retained_rows: Vec<usize>,
    pub removed_rows: Vec<usize>,
    pub source_gram: DMatrix<f64>,
    pub source_rhs: Vec<f64>,
}
/// fixest lm_related.cpp: rank-revealing cpp_cholesky, invert_tri, tproduct_tri.
/// Its triangular-inverse accumulation differs from LAPACK's blocked solve.
/// This source-specific finalization reuses Hirmos's fixed-effects projection;
/// it is not a second fixed-effects estimator. Exclusions follow the source's
/// ordered Cholesky rule rather than silently substituting a pseudoinverse.
fn source_inverse(gram: &DMatrix<f64>) -> Result<(Vec<usize>, DMatrix<f64>), Error> {
    let k = gram.nrows();
    let mut r = DMatrix::<f64>::zeros(k, k);
    let mut kept = Vec::new();
    for j in 0..k {
        let mut d = gram[(j, j)];
        for &a in &kept {
            d = (-r[(a, j)]).mul_add(r[(a, j)], d);
        }
        if !d.is_finite() {
            return Err(Error::Singular);
        }
        if d < 1e-9 {
            continue;
        }
        kept.push(j);
        r[(j, j)] = d.sqrt();
        for i in j + 1..k {
            let mut v = gram[(i, j)];
            for &a in kept.iter().filter(|&&a| a < j) {
                v = (-r[(a, i)]).mul_add(r[(a, j)], v);
            }
            r[(j, i)] = v / r[(j, j)];
        }
    }
    if kept.is_empty() {
        return Err(Error::Singular);
    }
    let mut r = DMatrix::from_fn(kept.len(), kept.len(), |i, j| r[(kept[i], kept[j])]);
    let k = kept.len();
    for i in 0..k {
        for j in i + 1..k {
            r[(j, i)] = r[(i, j)];
        }
        r[(i, i)] = 1. / r[(i, i)];
    }
    for band in 1..k {
        for i in 0..k - band {
            let col = i + band;
            let mut v = 0.;
            for a in i + 1..=col {
                v = (-r[(a, i)]).mul_add(r[(a, col)], v);
            }
            r[(i, col)] = v * r[(i, i)];
        }
    }
    let mut inverse = DMatrix::<f64>::zeros(k, k);
    for i in 0..k {
        for j in i..k {
            let mut v = 0.;
            for a in j..k {
                v = r[(j, a)].mul_add(r[(i, a)], v);
            }
            inverse[(i, j)] = v;
            inverse[(j, i)] = v;
        }
    }
    Ok((kept, inverse))
}
/// Unit-clustered Sun–Abraham fit using the existing Hirmos two-way projection
/// and OLS. fixest's default nonnested-FE correction counts period effects and
/// excludes unit effects nested in unit clusters. The underlying sandwich is
/// unchanged; the correction convention differs from PanelOLS.
pub fn fit(
    units: &[u64],
    periods: &[i64],
    cohorts: &[i64],
    outcome: &[f64],
    references: &[i64],
    reference_cohorts: &[i64],
) -> Result<Fit, Error> {
    if units.len() != outcome.len() || periods.len() != outcome.len() {
        return Err(Error::Shape);
    }
    let design = sa::calendar_design(cohorts, periods, references, reference_cohorts)
        .map_err(Error::Design)?;
    let mut keys = BTreeSet::new();
    let mut unit_cohorts = BTreeMap::new();
    for i in 0..units.len() {
        if !keys.insert((units[i], periods[i])) {
            return Err(Error::DuplicatePanelKey);
        }
        if unit_cohorts
            .insert(units[i], cohorts[i])
            .is_some_and(|g| g != cohorts[i])
        {
            return Err(Error::InconsistentCohort);
        }
    }
    // Canonical keyed accumulation keeps numerical rounding independent of
    // uploaded row order. Row identities remain the original source indices.
    let mut order: Vec<_> = (0..design.retained_rows.len()).collect();
    order.sort_by_key(|r| {
        let i = design.retained_rows[*r];
        (units[i], periods[i])
    });
    let ids: Vec<_> = order
        .iter()
        .map(|r| units[design.retained_rows[*r]])
        .collect();
    // Bitwise mapping retains equality, including negative calendar indices.
    let times: Vec<_> = design
        .retained_rows
        .iter()
        .map(|i| periods[*i] as u64)
        .collect();
    let times: Vec<_> = order.iter().map(|r| times[*r]).collect();
    let y: Vec<_> = order
        .iter()
        .map(|r| outcome[design.retained_rows[*r]])
        .collect();
    let matrix = DMatrix::from_fn(order.len(), design.matrix.ncols(), |r, c| {
        design.matrix[(order[r], c)]
    });
    let regression = estimation::ols_two_way(&y, &matrix, &ids, &times, WithinErrors::Cluster)
        .map_err(Error::Regression)?;
    let n = y.len();
    let k = design.cells.len();
    // fixest accumulates cross-products row by row, rather than computing an
    // SVD pseudoinverse as statsmodels does. Retain the shared FE projection,
    // then apply the source's triangular-inverse accumulation for that path.
    let x = &regression.projected_design;
    let y = &regression.within_outcome;
    let mut gram = DMatrix::<f64>::zeros(k, k);
    let mut rhs = DVector::<f64>::zeros(k);
    for j in 0..k {
        for i in 0..n {
            rhs[j] = x[(i, j)].mul_add(y[i], rhs[j]);
        }
        for l in j..k {
            let mut value = 0.;
            for i in 0..n {
                value = x[(i, j)].mul_add(x[(i, l)], value);
            }
            gram[(j, l)] = value;
            gram[(l, j)] = value;
        }
    }
    let (kept, inverse) = source_inverse(&gram)?;
    let source_gram = DMatrix::from_fn(kept.len(), kept.len(), |i, j| gram[(kept[i], kept[j])]);
    let source_rhs = kept.iter().map(|j| rhs[*j]).collect();
    let rhs = DVector::from_iterator(kept.len(), kept.iter().map(|j| rhs[*j]));
    let x = DMatrix::from_fn(n, kept.len(), |i, j| x[(i, kept[j])]);
    let k = kept.len();
    let t = times.iter().copied().collect::<BTreeSet<_>>().len();
    let counted = n
        .checked_sub(k + t)
        .filter(|v| *v > 0)
        .ok_or(Error::DegreesOfFreedom)?;
    let coefficients = &inverse * rhs;
    let residual = DVector::from_column_slice(y) - &x * &coefficients;
    let groups: Vec<_> = ids
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut scores = DMatrix::<f64>::zeros(groups.len(), k);
    for i in 0..n {
        let g = groups.binary_search(&ids[i]).unwrap();
        for j in 0..k {
            scores[(g, j)] += x[(i, j)] * residual[i];
        }
    }
    let g = groups.len() as f64;
    let correction = g / (g - 1.) * (n - 1) as f64 / counted as f64;
    let covariance = &inverse * scores.transpose() * scores * inverse.transpose() * correction;
    let cells = kept.iter().map(|j| design.cells[*j]).collect();
    let dropped_cells = design
        .cells
        .iter()
        .enumerate()
        .filter(|(j, _)| !kept.contains(j))
        .map(|(_, c)| *c)
        .collect();
    let support = kept
        .iter()
        .map(|j| design.matrix.column(*j).sum())
        .collect();
    Ok(Fit {
        cells,
        dropped_cells,
        coefficients: coefficients.as_slice().to_vec(),
        covariance,
        support,
        retained_rows: design.retained_rows,
        removed_rows: design.removed_rows,
        source_gram,
        source_rhs,
    })
}
