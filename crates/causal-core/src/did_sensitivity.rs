//! Riesz-representation sensitivity for observational panel DiD.
//! DoubleMLDID / DoubleMLDIDBinary 0.11.4: nuisance predictions are cross-fitted
//! outcome differences, not outcome levels. Learner selection is separate.
use crate::{did, dml, fminbound::fminbound};

pub struct CrossFitted {
    pub sensitivity: Fit,
    pub propensity_termination: Vec<crate::lbfgsb::LbfgsbTermination>,
}

pub struct BenchmarkRefit {
    pub values: Benchmark,
    pub long: CrossFitted,
    pub short: CrossFitted,
}

/// Refit after omitting a nonempty subset of baseline covariates, keeping the
/// same units, fold seed, score normalisation and learners. At least one remains.
pub fn benchmark_without(plan: &did::Plan<'_>, omitted: &[usize]) -> Result<BenchmarkRefit, Error> {
    let width = plan.sample.x[0].len();
    let mut selected = std::collections::HashSet::new();
    if omitted.is_empty()
        || omitted.len() >= width
        || omitted.iter().any(|&j| j >= width || !selected.insert(j))
    {
        return Err(Error::Input(did::Error::Shape));
    }
    let long = fit(plan)?;
    let short_sample = did::PairedSample {
        x: plan
            .sample
            .x
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &v)| (!selected.contains(&j)).then_some(v))
                    .collect()
            })
            .collect(),
        differences: plan.sample.differences.clone(),
        group: plan.sample.group.clone(),
    };
    let short_plan = did::Plan {
        sample: &short_sample,
        folds: plan.folds,
        seed: plan.seed,
        trimming: plan.trimming,
        normalization: plan.normalization,
    };
    let short = fit(&short_plan)?;
    let values = benchmark(&long.sensitivity, &short.sensitivity)?;
    Ok(BenchmarkRefit {
        values,
        long,
        short,
    })
}

/// Reuse the current DiD estimator and its folds. The extra treated-group regression
/// is used only for sensitivity; it cannot change the reported DiD point estimate.
pub fn fit(plan: &did::Plan<'_>) -> Result<CrossFitted, Error> {
    let fitted = did::fit(plan).map_err(Error::Input)?;
    let sample = plan.sample;
    let mut g1 = vec![0.0; sample.group.len()];
    for (train, test) in &fitted.folds {
        let treated: Vec<_> = train
            .iter()
            .copied()
            .filter(|&i| sample.group[i] == 1.0)
            .collect();
        let x = nalgebra::DMatrix::from_fn(treated.len(), sample.x[0].len(), |i, j| {
            sample.x[treated[i]][j]
        });
        let y = nalgebra::DVector::from_iterator(
            treated.len(),
            treated.iter().map(|&i| sample.differences[i]),
        );
        let regression = crate::sklearn_linear::fit_sklearn_linear_regression(
            &x,
            &y,
            treated.len().max(sample.x[0].len()) as f64 * f64::EPSILON,
        )
        .map_err(|_| Error::Input(did::Error::Regression))?;
        for &i in test {
            g1[i] = regression.intercept
                + sample.x[i]
                    .iter()
                    .zip(regression.coefficients.iter())
                    .map(|(x, b)| x * b)
                    .sum::<f64>();
        }
    }
    let sensitivity = from_predictions(
        &sample.differences,
        &sample.group,
        &fitted.g0,
        &g1,
        &fitted.propensity,
        plan.normalization,
        Units::All,
    )?;
    Ok(CrossFitted {
        sensitivity,
        propensity_termination: fitted.propensity_termination,
    })
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Input(did::Error),
    InvalidScenario,
    InvalidEmbedding,
    DegenerateOutcome,
    InvalidGrid,
}

/// The full panel's unit coordinates are required for comparing group-time cells.
pub enum Units<'a> {
    All,
    Subset {
        total: usize,
        positions: &'a [usize],
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RieszEstimate {
    Orthogonal,
    NonOrthogonalReferenceRecovery,
}

pub struct Fit {
    pub estimate: dml::DmlResult,
    pub sigma2: f64,
    pub nu2: f64,
    pub riesz_estimate: RieszEstimate,
    pub psi_sigma2: Vec<f64>,
    pub psi_nu2: Vec<f64>,
    pub riesz: Vec<f64>,
    positions: Vec<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct Scenario {
    cf_y: f64,
    cf_d: f64,
    rho: f64,
    level: f64,
    null: f64,
}
impl Scenario {
    pub fn new(cf_y: f64, cf_d: f64, rho: f64, level: f64, null: f64) -> Result<Self, Error> {
        if ![cf_y, cf_d, rho, level, null].iter().all(|v| v.is_finite())
            || !(0.0..1.0).contains(&cf_y)
            || !(0.0..1.0).contains(&cf_d)
            || rho.abs() > 1.0
            || level <= 0.0
            || level >= 1.0
        {
            return Err(Error::InvalidScenario);
        }
        Ok(Self {
            cf_y,
            cf_d,
            rho,
            level,
            null,
        })
    }
}

#[derive(Debug)]
pub struct Bounds {
    pub effect: [f64; 2],
    pub interval: [f64; 2],
}
#[derive(Debug)]
pub struct Robustness {
    pub value: f64,
    pub interval_value: f64,
}

impl Fit {
    pub fn bounds(&self, scenario: Scenario) -> Bounds {
        let (lo, hi, cl, ch) = self.estimate.sensitivity_bounds(
            scenario.cf_y,
            scenario.cf_d,
            scenario.rho,
            scenario.level,
        );
        Bounds {
            effect: [lo, hi],
            interval: [cl, ch],
        }
    }
    /// Same bounded minimisation and zero-null convention as DoubleMLFramework.
    pub fn robustness(&self, scenario: Scenario) -> Robustness {
        let side = usize::from(scenario.null > self.estimate.coef);
        let objective = |share, interval| {
            let b = self.bounds(Scenario {
                cf_y: share,
                cf_d: share,
                ..scenario
            });
            let v = if interval {
                b.interval[side]
            } else {
                b.effect[side]
            };
            (v - scenario.null).powi(2)
        };
        Robustness {
            value: fminbound(|v| objective(v, false), 0.0, 0.9999, 1e-5, 500),
            interval_value: fminbound(|v| objective(v, true), 0.0, 0.9999, 1e-5, 500),
        }
    }
    /// Row-major values for a contour plot; no rendering-specific interpolation.
    pub fn grid(
        &self,
        y_shares: &[f64],
        d_shares: &[f64],
        scenario: Scenario,
    ) -> Result<Vec<Vec<Bounds>>, Error> {
        if y_shares.is_empty() || d_shares.is_empty() {
            return Err(Error::InvalidGrid);
        }
        y_shares
            .iter()
            .map(|&y| {
                d_shares
                    .iter()
                    .map(|&d| {
                        Scenario::new(y, d, scenario.rho, scenario.level, scenario.null)
                            .map(|s| self.bounds(s))
                    })
                    .collect()
            })
            .collect()
    }
}

/// Predictions must have been obtained without using each row in its training fold.
/// This function cannot infer or certify that provenance from numeric arrays.
pub fn from_predictions(
    differences: &[f64],
    group: &[f64],
    g0: &[f64],
    g1: &[f64],
    propensity: &[f64],
    normalization: did::Normalization,
    units: Units<'_>,
) -> Result<Fit, Error> {
    let n = differences.len();
    let (a, b) = did::observational_score(differences, group, g0, propensity, normalization)
        .map_err(Error::Input)?;
    if g1.len() != n {
        return Err(Error::Input(did::Error::Shape));
    }
    if g1.iter().any(|v| !v.is_finite()) {
        return Err(Error::Input(did::Error::NonFinite));
    }
    let (total, positions) = match units {
        Units::All => (n, (0..n).collect::<Vec<_>>()),
        Units::Subset { total, positions } => {
            let mut seen = std::collections::HashSet::new();
            if total < n
                || positions.len() != n
                || positions.iter().any(|&i| i >= total || !seen.insert(i))
            {
                return Err(Error::InvalidEmbedding);
            }
            (total, positions.to_vec())
        }
    };
    let p = group.iter().sum::<f64>() / n as f64;
    let odds: Vec<_> = propensity.iter().map(|m| m / (1.0 - m)).collect();
    let mean_w = group
        .iter()
        .zip(&odds)
        .map(|(d, o)| (1.0 - d) * o)
        .sum::<f64>()
        / n as f64;
    let sigma_el: Vec<_> = (0..n)
        .map(|i| (differences[i] - group[i] * g1[i] - (1.0 - group[i]) * g0[i]).powi(2))
        .collect();
    let sigma2 = sigma_el.iter().sum::<f64>() / n as f64;
    if !sigma2.is_finite() || sigma2 <= 0.0 {
        return Err(Error::DegenerateOutcome);
    }
    let rr: Vec<_> = (0..n)
        .map(|i| match normalization {
            did::Normalization::InSample => group[i] / p - (1.0 - group[i]) * odds[i] / mean_w,
            did::Normalization::Population => group[i] / p - (1.0 - group[i]) * odds[i] / p,
        })
        .collect();
    let nu_el: Vec<_> = (0..n)
        .map(|i| {
            let ma = match normalization {
                did::Normalization::InSample => group[i] / p * (1.0 / p + odds[i] / mean_w),
                did::Normalization::Population => group[i] / p.powi(2) * (1.0 + odds[i]),
            };
            2.0 * ma - rr[i].powi(2)
        })
        .collect();
    let mut nu2 = nu_el.iter().sum::<f64>() / n as f64;
    let embed = |values: &[f64]| {
        let mut out = vec![0.0; total];
        for (j, &i) in positions.iter().enumerate() {
            out[i] = values[j] * total as f64 / n as f64;
        }
        out
    };
    let psi_sigma2 = embed(&sigma_el.iter().map(|v| v - sigma2).collect::<Vec<_>>());
    let mut psi_nu2 = embed(&nu_el.iter().map(|v| v - nu2).collect::<Vec<_>>());
    let riesz = embed(&rr);
    let riesz_estimate = if nu2 <= 0.0 {
        // Upstream validates after embedding; preserve this order and report recovery.
        psi_nu2 = riesz.iter().map(|r| r * r).collect();
        nu2 = psi_nu2.iter().sum::<f64>() / total as f64;
        RieszEstimate::NonOrthogonalReferenceRecovery
    } else {
        RieszEstimate::Orthogonal
    };
    if !nu2.is_finite() || nu2 <= 0.0 {
        return Err(Error::DegenerateOutcome);
    }
    let mut estimate = dml::solve_score(&a, &b);
    dml::attach_sensitivity(&mut estimate, &a, &b, sigma2, &psi_sigma2, nu2, &psi_nu2);
    estimate.scaled_psi = embed(&estimate.scaled_psi);
    Ok(Fit {
        estimate,
        sigma2,
        nu2,
        riesz_estimate,
        psi_sigma2,
        psi_nu2,
        riesz,
        positions,
    })
}

#[derive(Debug)]
pub struct Benchmark {
    pub cf_y: f64,
    pub cf_d: f64,
    pub rho: f64,
    pub delta_theta: f64,
}
/// One-repetition gain_statistics: short omits the benchmark covariate(s).
/// Caller fits long and short models on identical observations and folds.
pub fn benchmark(long: &Fit, short: &Fit) -> Result<Benchmark, Error> {
    if long.estimate.scaled_psi.len() != short.estimate.scaled_psi.len()
        || long.positions != short.positions
    {
        return Err(Error::InvalidEmbedding);
    }
    let dy = short.sigma2 - long.sigma2;
    let dn = long.nu2 - short.nu2;
    let delta = short.estimate.coef - long.estimate.coef;
    let denom = if dy > 0.0 && dn > 0.0 {
        (dy * dn).sqrt()
    } else {
        0.0
    };
    let rho = if denom == 0.0 {
        1.0
    } else {
        (delta.abs() / denom).clamp(0.0, 1.0)
    } * if delta == 0.0 { 0.0 } else { delta.signum() };
    Ok(Benchmark {
        cf_y: (dy / long.sigma2).clamp(0.0, 1.0),
        cf_d: (dn / short.nu2).clamp(0.0, 1.0),
        rho,
        delta_theta: delta,
    })
}
