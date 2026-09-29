// Source translation of DiceKriging 1.6.1's noisy constant-trend Matérn-5/2
// likelihood and simple-kriging mean, the restricted path used by GRF tuning.
// DiceKriging is licensed GPL-2 | GPL-3; this derived module uses GPL-3.0-or-later.
// SPDX-License-Identifier: GPL-3.0-or-later
use crate::lapack_cholesky::{dpotrf, dpotrs, Triangle};
use crate::{
    lbfgsb::{lbfgsb_r, LbfgsbTermination},
    survival::r_rng::RRng,
};

pub struct FittedSurface {
    pub surface: Surface,
    pub parameters: Vec<f64>,
    pub initial: Vec<f64>,
    pub termination: LbfgsbTermination,
}

/// The restricted kmNuggets.init path used by GRF: constant observation noise
/// var(errors)/2, twenty seeded starts, constant trend, one bounded optimization.
pub fn fit(
    design: &[Vec<f64>],
    response: &[f64],
    seed: u32,
) -> Result<FittedSurface, &'static str> {
    let n = response.len();
    let d = design.first().ok_or("Empty kriging design.")?.len();
    // Validate before indexing the design or constructing distance pairs.
    Surface::at(design, response, 1., &vec![1.; d + 1])?;
    let mean = response.iter().sum::<f64>() / n as f64;
    let variance = response.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    if variance <= 0. {
        return Err("Constant kriging response.");
    }
    let noise = variance / 2.;
    let mut lower = vec![1e-10; d];
    let mut upper: Vec<_> = (0..d)
        .map(|j| {
            2. * (design
                .iter()
                .map(|r| r[j])
                .fold(f64::NEG_INFINITY, f64::max)
                - design.iter().map(|r| r[j]).fold(f64::INFINITY, f64::min))
        })
        .collect();
    if upper.iter().any(|v| *v <= 1e-10) {
        return Err("Kriging design has a constant parameter dimension.");
    }
    let mut pairs = vec![];
    for j in 0..n {
        for i in j + 1..n {
            pairs.push((
                design[i]
                    .iter()
                    .zip(&design[j])
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>()
                    .sqrt(),
                (response[i] - response[j]).powi(2),
            ));
        }
    }
    let mut distances: Vec<_> = pairs.iter().map(|p| p.0).collect();
    distances.sort_by(f64::total_cmp);
    let m = distances.len();
    let median = if m % 2 == 0 {
        (distances[m / 2 - 1] + distances[m / 2]) / 2.
    } else {
        distances[m / 2]
    };
    let far: Vec<_> = pairs.iter().filter(|p| p.0 > median).collect();
    if far.is_empty() {
        return Err("No distances above the kriging initialization median.");
    }
    let variogram = 0.5 * far.iter().map(|p| p.1).sum::<f64>() / far.len() as f64;
    let vario_signal = if variogram - noise <= 1e-20 {
        variogram / 2.
    } else {
        variogram - noise
    };
    let init_variance = (variance - noise + vario_signal) / 2.;
    let vlo = if variogram - noise <= 1e-20 {
        1e-20
    } else {
        variogram - noise
    };
    let vhi = if variogram - noise <= 1e-20 {
        variogram
    } else {
        variogram - noise
    };
    let mut rng = RRng::new(seed);
    let mut starts: Vec<Vec<f64>> = (0..20)
        .map(|_| {
            (0..d)
                .map(|j| lower[j] + rng.uniform() * (upper[j] - lower[j]))
                .collect()
        })
        .collect();
    for p in &mut starts {
        p.push(0.5 * init_variance + rng.uniform() * init_variance);
    }
    lower.push(0.1 * (variance - noise).min(vlo));
    upper.push(10. * (variance - noise).max(vhi));
    let mut best = None;
    for p in starts {
        let value = Surface::at(design, response, noise, &p)?.log_likelihood;
        if best.as_ref().is_none_or(|(score, _)| value > *score) {
            best = Some((value, p));
        }
    }
    let initial = best.ok_or("No kriging initialization.")?.1;
    let mut failure = None;
    let result = lbfgsb_r(
        &initial,
        &lower,
        &upper,
        &vec![2; d + 1],
        5,
        1e7,
        0.,
        20,
        100,
        |p| match Surface::at(design, response, noise, p) {
            Ok(s) => (-s.log_likelihood, s.gradient.iter().map(|v| -v).collect()),
            Err(e) => {
                failure = Some(e);
                (f64::NAN, vec![f64::NAN; d + 1])
            }
        },
    );
    if let Some(e) = failure {
        return Err(e);
    }
    let surface = Surface::at(design, response, noise, &result.x)?;
    Ok(FittedSurface {
        surface,
        parameters: result.x,
        initial,
        termination: result.termination,
    })
}

pub struct Surface {
    design: Vec<Vec<f64>>,
    parameters: Vec<f64>,
    pub trend: f64,
    pub log_likelihood: f64,
    pub gradient: Vec<f64>,
    precision_residual: Vec<f64>,
}
fn correlation(a: &[f64], b: &[f64], ranges: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .zip(ranges)
        .map(|((&a, &b), &range)| {
            let z = 5_f64.sqrt() * (a - b).abs() / range;
            (1. + z + z * z / 3.) * (-z).exp()
        })
        .product()
}
impl Surface {
    /// Row-major design; range parameters followed by process variance. Noise
    /// variance is known, not another free nugget or an added numerical jitter.
    pub fn at(
        design: &[Vec<f64>],
        response: &[f64],
        noise: f64,
        parameters: &[f64],
    ) -> Result<Self, &'static str> {
        let n = design.len();
        let d = parameters
            .len()
            .checked_sub(1)
            .ok_or("Missing kriging parameters.")?;
        if n < 2
            || d == 0
            || response.len() != n
            || !noise.is_finite()
            || noise < 0.
            || parameters.iter().any(|p| !p.is_finite() || *p <= 0.)
            || response.iter().any(|v| !v.is_finite())
            || design
                .iter()
                .any(|row| row.len() != d || row.iter().any(|v| !v.is_finite()))
        {
            return Err("Invalid kriging design, response, noise or parameters.");
        }
        let variance = parameters[d];
        let ranges = &parameters[..d];
        let mut cov = vec![0.; n * n];
        for j in 0..n {
            for i in 0..n {
                cov[i + j * n] = variance * correlation(&design[i], &design[j], ranges);
            }
        }
        let mut factor = cov.clone();
        for i in 0..n {
            factor[i + i * n] += noise;
        }
        if dpotrf(Triangle::Upper, n, &mut factor, n)
            .map_err(|_| "Invalid kriging factor dimensions.")?
            != 0
        {
            return Err("Kriging covariance is not positive definite.");
        }
        let solve = |rhs: &mut [f64], count: usize| {
            dpotrs(Triangle::Upper, n, count, &factor, n, rhs, n)
                .map_err(|_| "Kriging solve failed.")
        };
        let mut cy = response.to_vec();
        solve(&mut cy, 1)?;
        let mut ones = vec![1.; n];
        solve(&mut ones, 1)?;
        let trend = cy.iter().sum::<f64>() / ones.iter().sum::<f64>();
        let residual: Vec<_> = response.iter().map(|v| v - trend).collect();
        let mut precision_residual = residual.clone();
        solve(&mut precision_residual, 1)?;
        let quadratic = residual
            .iter()
            .zip(&precision_residual)
            .map(|(a, b)| a * b)
            .sum::<f64>();
        let log_likelihood = -0.5
            * (n as f64 * (2. * std::f64::consts::PI).ln()
                + 2. * (0..n).map(|i| factor[i + i * n].ln()).sum::<f64>()
                + quadratic);
        let mut inverse = vec![0.; n * n];
        for i in 0..n {
            inverse[i + i * n] = 1.;
        }
        solve(&mut inverse, n)?;
        let mut gradient = vec![0.; d + 1];
        for k in 0..=d {
            for j in 0..n {
                for i in 0..n {
                    let derivative = if k == d {
                        cov[i + j * n] / variance
                    } else {
                        let z = 5_f64.sqrt() * (design[i][k] - design[j][k]).abs() / ranges[k];
                        cov[i + j * n]
                            * (z * z * (1. + z) / (3. * ranges[k] * (1. + z + z * z / 3.)))
                    };
                    gradient[k] += 0.5
                        * (precision_residual[i] * precision_residual[j] - inverse[i + j * n])
                        * derivative;
                }
            }
        }
        Ok(Self {
            design: design.to_vec(),
            parameters: parameters.to_vec(),
            trend,
            log_likelihood,
            gradient,
            precision_residual,
        })
    }
    pub fn predict(&self, rows: &[Vec<f64>]) -> Result<Vec<f64>, &'static str> {
        let d = self.parameters.len() - 1;
        if rows
            .iter()
            .any(|r| r.len() != d || r.iter().any(|v| !v.is_finite()))
        {
            return Err("Invalid kriging prediction design.");
        }
        Ok(rows
            .iter()
            .map(|row| {
                self.trend
                    + self
                        .design
                        .iter()
                        .zip(&self.precision_residual)
                        .map(|(x, a)| {
                            self.parameters[d] * correlation(x, row, &self.parameters[..d]) * a
                        })
                        .sum::<f64>()
            })
            .collect())
    }
}
