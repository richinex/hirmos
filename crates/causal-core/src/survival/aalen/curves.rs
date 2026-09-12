//! Cumulative coefficients and model-based bands from plot.aareg.

use super::Fit;

#[derive(Clone, Debug)]
pub struct CoefficientPoint {
    pub time: f64,
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
}

/// Return one cumulative-coefficient curve per term, including the intercept.
/// Bands use the reference plot's fixed multiplier of 1.96.
pub fn coefficient_curves(fit: &Fit) -> Vec<Vec<CoefficientPoint>> {
    let terms = fit.statistic.len();
    let mut curves = vec![
        vec![CoefficientPoint {
            time: 0.0,
            estimate: 0.0,
            lower: 0.0,
            upper: 0.0
        }];
        terms
    ];
    let mut cumulative = vec![0.0; terms];
    let mut variance = vec![0.0; terms];
    let mut time_index = 0;
    for i in 0..fit.times.len() {
        for j in 0..terms {
            cumulative[j] += fit.coefficient[i][j];
            if fit.influence.is_none() {
                variance[j] += fit.coefficient[i][j] * fit.coefficient[i][j];
            }
        }
        if fit.times.get(i + 1) == Some(&fit.times[i]) {
            continue;
        }
        for j in 0..terms {
            if let Some(influence) = &fit.influence {
                variance[j] += influence.increments[time_index]
                    .iter()
                    .map(|row| row[j] * row[j])
                    .sum::<f64>();
            }
            let half_width = 1.96 * variance[j].sqrt();
            curves[j].push(CoefficientPoint {
                time: fit.times[i],
                estimate: cumulative[j],
                lower: cumulative[j] - half_width,
                upper: cumulative[j] + half_width,
            });
        }
        time_index += 1;
    }
    curves
}
