//! Design-specific Monte Carlo power with classical OLS inference: independent
//! two-arm experiments, equal-sized randomized clusters analyzed as cluster
//! means, and two-period DiD analyzed as unit changes. Not a generic panel or
//! observational-design power approximation.
use crate::{
    nprandom::NpRng, ols::Ols, parcorr::analytic_pvalue_t, power::student_t_upper_quantile,
};
use nalgebra::{DMatrix, DVector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationError {
    InvalidDesign,
    InvalidData,
    DegenerateFit,
    InvalidRecord,
    FitFailed,
    EmptySimulation,
}

#[derive(Debug, Clone, Copy)]
pub struct Replicate {
    pub truth: f64,
    pub estimate: f64,
    pub standard_error: f64,
    pub p_value: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Diagnosis {
    pub replications: usize,
    pub rejected: usize,
    /// Under a zero true effect this is the Type-I rejection rate, not power.
    pub rejection_rate: f64,
    /// Binomial Monte Carlo standard error for independent replicates. A zero
    /// plug-in value at 0/1 does not establish certainty; counts are also retained.
    pub rejection_mcse: f64,
    pub coverage: f64,
    pub bias: f64,
    pub rmse: f64,
}

fn alpha_valid(alpha: f64) -> bool {
    alpha.is_finite() && alpha > 0. && alpha < 1.
}

/// Reuse the existing statsmodels-compatible OLS solver and Student-t inference.
pub fn fit_two_arm(
    y: &[f64],
    treatment: &[u8],
    truth: f64,
    alpha: f64,
) -> Result<Replicate, SimulationError> {
    if y.len() != treatment.len()
        || !truth.is_finite()
        || !alpha_valid(alpha)
        || y.iter().any(|v| !v.is_finite())
        || treatment.iter().any(|&v| v > 1)
    {
        return Err(SimulationError::InvalidData);
    }
    let treated = treatment.iter().filter(|&&v| v == 1).count();
    if treated < 2 || y.len() - treated < 2 {
        return Err(SimulationError::InvalidDesign);
    }
    let x = DMatrix::from_fn(
        y.len(),
        2,
        |i, j| if j == 0 { 1. } else { treatment[i] as f64 },
    );
    let fit =
        Ols::try_fit(&x, &DVector::from_column_slice(y)).map_err(|_| SimulationError::FitFailed)?;
    if fit.rank != 2 || fit.ssr <= 0. {
        return Err(SimulationError::DegenerateFit);
    }
    let df = (y.len() - 2) as f64;
    let se = (fit.ssr / df * fit.xtx_inverse()[(1, 1)]).sqrt();
    if !se.is_finite() || se <= 0. {
        return Err(SimulationError::DegenerateFit);
    }
    let estimate = fit.params[1];
    let margin = student_t_upper_quantile(alpha / 2., df) * se;
    Ok(Replicate {
        truth,
        estimate,
        standard_error: se,
        p_value: analytic_pvalue_t((estimate / se).abs(), df),
        lower: estimate - margin,
        upper: estimate + margin,
    })
}

/// Matches DeclareDesign's mean rejection, coverage, bias and RMSE definitions.
/// Invalid replicates are refused rather than silently excluded from denominators.
pub fn diagnose(records: &[Replicate], alpha: f64) -> Result<Diagnosis, SimulationError> {
    if records.is_empty() {
        return Err(SimulationError::EmptySimulation);
    }
    if !alpha_valid(alpha) {
        return Err(SimulationError::InvalidDesign);
    }
    let (mut rejected, mut covered, mut error, mut squared) = (0, 0, 0., 0.);
    for r in records {
        if [
            r.truth,
            r.estimate,
            r.standard_error,
            r.p_value,
            r.lower,
            r.upper,
        ]
        .iter()
        .any(|v| !v.is_finite())
            || r.standard_error <= 0.
            || !(0. ..=1.).contains(&r.p_value)
            || r.lower > r.upper
        {
            return Err(SimulationError::InvalidRecord);
        }
        rejected += usize::from(r.p_value <= alpha);
        covered += usize::from(r.truth >= r.lower && r.truth <= r.upper);
        let e = r.estimate - r.truth;
        error += e;
        squared += e * e;
    }
    let n = records.len() as f64;
    let p = rejected as f64 / n;
    Ok(Diagnosis {
        replications: records.len(),
        rejected,
        rejection_rate: p,
        rejection_mcse: (p * (1. - p) / n).sqrt(),
        coverage: covered as f64 / n,
        bias: error / n,
        rmse: (squared / n).sqrt(),
    })
}

#[derive(Debug, Clone, Copy)]
pub struct TwoArmScenario {
    pub treated: usize,
    pub controls: usize,
    pub effect: f64,
    pub noise_sd: f64,
}

/// Equal-sized cluster-randomized design. Every member of a cluster has the
/// same treatment. Analyze cluster means with one independent observation per
/// randomized cluster. This is not a row-level clustered sandwich estimator.
pub fn fit_equal_clusters(
    y: &[f64],
    z: &[u8],
    clusters: &[u64],
    truth: f64,
    alpha: f64,
) -> Result<Replicate, SimulationError> {
    use std::collections::BTreeMap;
    if y.len() != z.len()
        || y.len() != clusters.len()
        || y.iter().any(|v| !v.is_finite())
        || z.iter().any(|&v| v > 1)
    {
        return Err(SimulationError::InvalidData);
    }
    let mut groups = BTreeMap::<u64, (u8, usize, f64)>::new();
    for i in 0..y.len() {
        let entry = groups.entry(clusters[i]).or_insert((z[i], 0, 0.));
        if entry.0 != z[i] {
            return Err(SimulationError::InvalidDesign);
        }
        entry.1 += 1;
        entry.2 += y[i];
    }
    let size = groups
        .values()
        .next()
        .ok_or(SimulationError::InvalidDesign)?
        .1;
    if groups.values().any(|g| g.1 != size) {
        return Err(SimulationError::InvalidDesign);
    }
    let means: Vec<_> = groups.values().map(|g| g.2 / g.1 as f64).collect();
    let treatment: Vec<_> = groups.values().map(|g| g.0).collect();
    fit_two_arm(&means, &treatment, truth, alpha)
}

/// Two-period, simultaneous-adoption DiD with independent units and classical
/// common-variance inference on changes. Input z is the treated-group indicator,
/// not a long-panel treatment column. Parallel trends is an assumption, not a
/// property inferred by this adapter.
pub fn fit_two_period_did(
    before: &[f64],
    after: &[f64],
    z: &[u8],
    truth: f64,
    alpha: f64,
) -> Result<Replicate, SimulationError> {
    if before.len() != after.len() {
        return Err(SimulationError::InvalidData);
    }
    let changes: Vec<_> = before.iter().zip(after).map(|(a, b)| b - a).collect();
    fit_two_arm(&changes, z, truth, alpha)
}

#[derive(Debug, Clone, Copy)]
pub enum DependentScenario {
    EqualClusters {
        treated_clusters: usize,
        control_clusters: usize,
        members: usize,
        effect: f64,
        noise_sd: f64,
        icc: f64,
    },
    TwoPeriodDiD {
        treated: usize,
        controls: usize,
        effect: f64,
        noise_sd: f64,
        correlation: f64,
        untreated_trend_difference: f64,
    },
}

/// Gaussian scenario generator with explicit dependence and, for DiD, an
/// explicit parallel-trends violation parameter. Reuses the existing RNG and
/// fit adapters. Truth remains the treatment effect, not effect plus confounding.
pub fn simulate_dependent(
    scenario: DependentScenario,
    replications: usize,
    seed: u64,
    alpha: f64,
) -> Result<(Vec<Replicate>, Diagnosis), SimulationError> {
    if replications == 0 || !alpha_valid(alpha) {
        return Err(SimulationError::InvalidDesign);
    }
    let (treated, controls, effect, sd) = match scenario {
        DependentScenario::EqualClusters {
            treated_clusters,
            control_clusters,
            effect,
            noise_sd,
            members,
            icc,
        } => {
            if members == 0 || !icc.is_finite() || !(0. ..=1.).contains(&icc) {
                return Err(SimulationError::InvalidDesign);
            }
            (treated_clusters, control_clusters, effect, noise_sd)
        }
        DependentScenario::TwoPeriodDiD {
            treated,
            controls,
            effect,
            noise_sd,
            correlation,
            untreated_trend_difference,
        } => {
            if !correlation.is_finite()
                || correlation.abs() >= 1.
                || !untreated_trend_difference.is_finite()
            {
                return Err(SimulationError::InvalidDesign);
            }
            (treated, controls, effect, noise_sd)
        }
    };
    if treated < 2 || controls < 2 || !effect.is_finite() || !sd.is_finite() || sd <= 0. {
        return Err(SimulationError::InvalidDesign);
    }
    let n = treated
        .checked_add(controls)
        .ok_or(SimulationError::InvalidDesign)?;
    if let DependentScenario::EqualClusters { members, .. } = scenario {
        n.checked_mul(members)
            .ok_or(SimulationError::InvalidDesign)?;
    }
    let mut rng = NpRng::seeded(seed);
    let mut records = Vec::with_capacity(replications);
    for _ in 0..replications {
        let fit = match scenario {
            DependentScenario::EqualClusters { members, icc, .. } => {
                let (mut y, mut z, mut groups) = (Vec::new(), Vec::new(), Vec::new());
                for i in 0..n {
                    let shared = sd * icc.sqrt() * rng.standard_normal();
                    let group = u8::from(i < treated);
                    for _ in 0..members {
                        y.push(
                            effect * group as f64
                                + shared
                                + sd * (1. - icc).sqrt() * rng.standard_normal(),
                        );
                        z.push(group);
                        groups.push(i as u64);
                    }
                }
                fit_equal_clusters(&y, &z, &groups, effect, alpha)?
            }
            DependentScenario::TwoPeriodDiD {
                correlation,
                untreated_trend_difference,
                ..
            } => {
                let (mut before, mut after, mut z) = (Vec::new(), Vec::new(), Vec::new());
                for i in 0..n {
                    let group = u8::from(i < treated);
                    let baseline = rng.standard_normal() + 2. * group as f64;
                    let e0 = rng.standard_normal();
                    let e1 = rng.standard_normal();
                    before.push(baseline + sd * e0);
                    after.push(
                        baseline
                            + 1.
                            + (effect + untreated_trend_difference) * group as f64
                            + sd * (correlation * e0
                                + (1. - correlation * correlation).sqrt() * e1),
                    );
                    z.push(group);
                }
                fit_two_period_did(&before, &after, &z, effect, alpha)?
            }
        };
        records.push(fit);
    }
    let summary = diagnose(&records, alpha)?;
    Ok((records, summary))
}

/// Independent repeated samples with fixed group sizes. For this iid Gaussian
/// constant-effect model, fixing the row labels has the same sampling law as
/// randomly assigning exactly that many units. Does not reproduce R's RNG stream.
pub fn simulate_two_arm(
    scenario: TwoArmScenario,
    replications: usize,
    seed: u64,
    alpha: f64,
) -> Result<(Vec<Replicate>, Diagnosis), SimulationError> {
    let n = scenario
        .treated
        .checked_add(scenario.controls)
        .ok_or(SimulationError::InvalidDesign)?;
    if scenario.treated < 2
        || scenario.controls < 2
        || replications == 0
        || !alpha_valid(alpha)
        || !scenario.effect.is_finite()
        || !scenario.noise_sd.is_finite()
        || scenario.noise_sd <= 0.
    {
        return Err(SimulationError::InvalidDesign);
    }
    let treatment: Vec<u8> = (0..n).map(|i| u8::from(i < scenario.treated)).collect();
    let mut rng = NpRng::seeded(seed);
    let mut records = Vec::with_capacity(replications);
    for _ in 0..replications {
        let y: Vec<f64> = treatment
            .iter()
            .map(|&z| scenario.effect * z as f64 + scenario.noise_sd * rng.standard_normal())
            .collect();
        records.push(fit_two_arm(&y, &treatment, scenario.effect, alpha)?);
    }
    let summary = diagnose(&records, alpha)?;
    Ok((records, summary))
}
