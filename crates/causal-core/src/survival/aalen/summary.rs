//! Weighted coefficient summaries and the overall chi-squared test.

use super::{Error, Fit, TestWeight};
use crate::lapack_lu::{dgetrf, dgetrs, Transpose};
use spec_math::cephes64::chdtrc;

#[derive(Debug)]
pub struct Summary {
    /// Columns: slope, weighted coefficient, standard error, z, two-sided p.
    pub table: SummaryTable,
    pub chisq: f64,
    pub degrees_of_freedom: usize,
    pub p_value: f64,
}

#[derive(Debug)]
pub enum SummaryTable {
    Model(Vec<[f64; 5]>),
    /// Slope, coefficient, model SE, robust SE, z, p.
    Robust(Vec<[f64; 6]>),
}

/// Summarize the full fitted interval with the fit's original test weighting.
pub fn summarize(fit: &Fit) -> Result<Summary, Error> {
    summarize_scaled(fit, 1.0)
}

fn summarize_scaled(fit: &Fit, scale_factor: f64) -> Result<Summary, Error> {
    let test = fit.test;
    let terms = fit.statistic.len();
    let mut table = Vec::with_capacity(terms);
    let mut robust_table = Vec::with_capacity(terms);
    let covariance = fit
        .influence
        .as_ref()
        .map_or(&fit.variance, |v| &v.variance);
    for j in 0..terms {
        let mut cumulative = 0.0;
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        let mut total_weight = 0.0;
        for i in 0..fit.times.len() {
            let weight = match test {
                TestWeight::Aalen => fit.tweight[i][j],
                TestWeight::AtRisk => fit.nrisk[i],
            };
            cumulative += weight * fit.coefficient[i][j];
            numerator += cumulative * fit.times[i];
            denominator += weight * fit.times[i] * fit.times[i];
            total_weight += weight;
        }
        let scale = match test {
            TestWeight::Aalen => total_weight,
            TestWeight::AtRisk => fit.times.len() as f64,
        } / scale_factor;
        let se = fit.variance[j][j].sqrt();
        let se2 = covariance[j][j].sqrt();
        let z = fit.statistic[j] / se2;
        table.push([
            numerator / denominator,
            fit.statistic[j] / scale,
            se / scale,
            z,
            chdtrc(1.0, z * z),
        ]);
        robust_table.push([
            numerator / denominator,
            fit.statistic[j] / scale,
            se / scale,
            se2 / scale,
            z,
            chdtrc(1.0, z * z),
        ]);
    }
    let p = terms - 1;
    let mut matrix = vec![0.0; p * p];
    for j in 0..p {
        for i in 0..p {
            matrix[i + j * p] = covariance[i + 1][j + 1];
        }
    }
    let mut pivots = vec![0; p];
    let info = dgetrf(p, p, &mut matrix, p, &mut pivots).map_err(|_| Error::SingularRiskSet)?;
    if info != 0 {
        return Err(Error::SingularRiskSet);
    }
    let mut solution = fit.statistic[1..].to_vec();
    dgetrs(Transpose::None, p, 1, &matrix, p, &pivots, &mut solution, p)
        .map_err(|_| Error::SingularRiskSet)?;
    let chisq = solution
        .iter()
        .zip(&fit.statistic[1..])
        .map(|(a, b)| a * b)
        .sum();
    Ok(Summary {
        table: if fit.influence.is_some() {
            SummaryTable::Robust(robust_table)
        } else {
            SummaryTable::Model(table)
        },
        chisq,
        degrees_of_freedom: p,
        p_value: chdtrc(p as f64, chisq),
    })
}

/// Recompute a summary through a supplied time, optionally changing its test weights.
pub fn summarize_through(
    fit: &Fit,
    maxtime: f64,
    test: TestWeight,
    scale: f64,
) -> Result<Summary, Error> {
    if !maxtime.is_finite() || !scale.is_finite() || scale <= 0.0 {
        return Err(Error::InvalidOptions);
    }
    let count = fit.times.partition_point(|t| *t <= maxtime);
    if count == 0 {
        return Err(Error::TooFewEvents);
    }
    let mut cut = fit.clone();
    cut.test = test;
    cut.times.truncate(count);
    cut.nrisk.truncate(count);
    cut.coefficient.truncate(count);
    cut.tweight.truncate(count);
    let p = fit.statistic.len();
    cut.statistic.fill(0.0);
    cut.variance.iter_mut().for_each(|r| r.fill(0.0));
    for i in 0..count {
        let weights: Vec<f64> = (0..p)
            .map(|j| match test {
                TestWeight::Aalen => cut.tweight[i][j],
                TestWeight::AtRisk => cut.nrisk[i],
            })
            .collect();
        for j in 0..p {
            let value = weights[j] * cut.coefficient[i][j];
            cut.statistic[j] += value;
            for k in 0..p {
                cut.variance[j][k] += value * weights[k] * cut.coefficient[i][k];
            }
        }
    }
    if let Some(influence) = &mut cut.influence {
        let mut grouped = vec![vec![0.0; p]; influence.increments[0].len()];
        let mut time = 0;
        for i in 0..count {
            if i > 0 && cut.times[i] == cut.times[i - 1] {
                continue;
            }
            for (cluster, row) in influence.increments[time].iter().enumerate() {
                for j in 0..p {
                    let weight = match test {
                        TestWeight::Aalen => cut.tweight[i][j],
                        TestWeight::AtRisk => cut.nrisk[i],
                    };
                    grouped[cluster][j] += row[j] * weight;
                }
            }
            time += 1;
        }
        influence.variance.iter_mut().for_each(|r| r.fill(0.0));
        for row in grouped {
            for j in 0..p {
                for k in 0..p {
                    influence.variance[j][k] += row[j] * row[k];
                }
            }
        }
        influence.increments.truncate(time);
    }
    summarize_scaled(&cut, scale)
}
