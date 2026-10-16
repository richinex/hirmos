//! Scalar normal confidence-interval coverage conditional on passing a two-sided pre-test.

use spec_math::cephes64::ndtri;

/// Standardised estimator biases and their sampling correlation, not observed effect sizes.
#[derive(Clone, Copy, Debug)]
pub struct Scenario {
    pub target_bias: f64,
    pub pretest_bias: f64,
    pub correlation: f64,
    pub interval_alpha: f64,
    pub pretest_alpha: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    InvalidBias,
    InvalidCorrelation,
    InvalidLevel,
    NumericalRange,
    PassProbabilityTooSmall,
    IntegrationDidNotConverge,
}

#[derive(Clone, Copy, Debug)]
pub struct Coverage {
    pub unconditional: f64,
    pub conditional: f64,
    pub pass_probability: f64,
    pub joint_probability: f64,
    /// Quadrature error estimate for conditional coverage, not a statistical standard error.
    pub integration_error: f64,
    pub evaluations: usize,
}

fn sf(x: f64) -> f64 {
    0.5 * libm::erfc(x / std::f64::consts::SQRT_2)
}

fn mass(lower: f64, upper: f64) -> f64 {
    if lower >= upper {
        0.0
    } else if lower >= 0.0 {
        sf(lower) - sf(upper)
    } else if upper <= 0.0 {
        sf(-upper) - sf(-lower)
    } else {
        1.0 - sf(upper) - sf(-lower)
    }
}

/// Compute asymptotic coverage for one scalar pre-test and one scalar target.
/// Refuse numerically unresolvable scenarios rather than replace conditional coverage.
pub fn coverage(s: Scenario) -> Result<Coverage, Error> {
    if !s.target_bias.is_finite() || !s.pretest_bias.is_finite() {
        return Err(Error::InvalidBias);
    }
    if !s.correlation.is_finite() || !(-1.0..=1.0).contains(&s.correlation) {
        return Err(Error::InvalidCorrelation);
    }
    if [s.interval_alpha, s.pretest_alpha]
        .iter()
        .any(|a| !a.is_finite() || *a <= 0.0 || *a >= 1.0)
    {
        return Err(Error::InvalidLevel);
    }
    let target_critical = -ndtri(s.interval_alpha / 2.0);
    let pretest_critical = -ndtri(s.pretest_alpha / 2.0);
    let lower = -pretest_critical - s.pretest_bias;
    let upper = pretest_critical - s.pretest_bias;
    if !target_critical.is_finite() || !lower.is_finite() || !upper.is_finite() || lower >= upper {
        return Err(Error::NumericalRange);
    }
    let pass_probability = mass(lower, upper);
    // Normalising below this floor leaves too little floating-point range for reliable integration.
    if pass_probability < 1e-280 {
        return Err(Error::PassProbabilityTooSmall);
    }
    let unconditional = mass(
        -target_critical - s.target_bias,
        target_critical - s.target_bias,
    );
    let (conditional, integration_error, evaluations) = if s.correlation == 0.0 {
        (unconditional, 0.0, 0)
    } else if s.correlation.abs() == 1.0 {
        let a = (-target_critical - s.target_bias) / s.correlation;
        let b = (target_critical - s.target_bias) / s.correlation;
        (
            mass(lower.max(a.min(b)), upper.min(a.max(b))) / pass_probability,
            0.0,
            0,
        )
    } else {
        let sd = ((1.0 - s.correlation) * (1.0 + s.correlation)).sqrt();
        let integrand = |z: f64| {
            let mean = s.target_bias + s.correlation * z;
            let probability = mass(
                (-target_critical - mean) / sd,
                (target_critical - mean) / sd,
            );
            probability * (-0.5 * z * z - pass_probability.ln()).exp()
                / std::f64::consts::TAU.sqrt()
        };
        // Split at the normal mode and narrow transitions so near-singular correlations cannot hide a peak.
        let mut points = vec![lower, upper];
        for z in -40..=40 {
            let z = z as f64;
            if lower < z && z < upper {
                points.push(z);
            }
        }
        for boundary in [-target_critical, target_critical] {
            let center = (boundary - s.target_bias) / s.correlation;
            for offset in [-8.0, -1.0, 0.0, 1.0, 8.0] {
                let z = center + offset * sd / s.correlation.abs();
                if lower < z && z < upper {
                    points.push(z);
                }
            }
        }
        points.sort_by(f64::total_cmp);
        points.dedup();
        let tolerance = 1e-11 / (points.len() - 1) as f64;
        let mut evaluations = 0;
        let mut value = 0.0;
        let mut error = 0.0;
        for interval in points.windows(2) {
            let a = interval[0];
            let b = interval[1];
            let fa = integrand(a);
            let fm = integrand(a + (b - a) / 2.0);
            let fb = integrand(b);
            evaluations += 3;
            let (part, bound) = integrate(
                &integrand,
                a,
                b,
                [fa, fm, fb],
                tolerance,
                24,
                &mut evaluations,
            )?;
            value += part;
            error += bound;
        }
        (value, error, evaluations)
    };
    if !conditional.is_finite() || !(-1e-11..=1.0 + 1e-11).contains(&conditional) {
        return Err(Error::NumericalRange);
    }
    let conditional = conditional.clamp(0.0, 1.0);
    Ok(Coverage {
        unconditional,
        conditional,
        pass_probability,
        joint_probability: conditional * pass_probability,
        integration_error,
        evaluations,
    })
}

fn integrate<F: Fn(f64) -> f64>(
    f: &F,
    a: f64,
    b: f64,
    values: [f64; 3],
    tolerance: f64,
    depth: usize,
    evaluations: &mut usize,
) -> Result<(f64, f64), Error> {
    if *evaluations > 200_000 {
        return Err(Error::IntegrationDidNotConverge);
    }
    let [fa, fm, fb] = values;
    let m = a + (b - a) / 2.0;
    let fl = f(a + (m - a) / 2.0);
    let fr = f(m + (b - m) / 2.0);
    *evaluations += 2;
    let whole = (b - a) * (fa + 4.0 * fm + fb) / 6.0;
    let left = (m - a) * (fa + 4.0 * fl + fm) / 6.0;
    let right = (b - m) * (fm + 4.0 * fr + fb) / 6.0;
    let delta = left + right - whole;
    if !delta.is_finite() {
        return Err(Error::NumericalRange);
    }
    if delta.abs() <= 15.0 * tolerance {
        return Ok((left + right + delta / 15.0, delta.abs() / 15.0));
    }
    if depth == 0 || m == a || m == b {
        return Err(Error::IntegrationDidNotConverge);
    }
    let (l, le) = integrate(
        f,
        a,
        m,
        [fa, fl, fm],
        tolerance / 2.0,
        depth - 1,
        evaluations,
    )?;
    let (r, re) = integrate(
        f,
        m,
        b,
        [fm, fr, fb],
        tolerance / 2.0,
        depth - 1,
        evaluations,
    )?;
    Ok((l + r, le + re))
}
