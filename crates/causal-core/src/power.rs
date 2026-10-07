//! Independent, common-variance two-sample t-test power, checked against
//! statsmodels 0.15.0 / SciPy 1.18.1. Not a power formula for arbitrary causal estimators.
use spec_math::cephes64::{incbet, incbi, lgam, ndtr, ndtri};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alternative {
    TwoSided,
    Larger,
    Smaller,
}

/// A paired t-test is a one-sample t-test on within-pair differences. Its effect
/// is standardized by the SD of those differences, and n is the number of pairs.
#[derive(Clone, Copy, Debug)]
pub enum PowerDesign {
    OneSampleT,
    IndependentT {
        ratio: f64,
    },
    /// NormalIndPower with ddof=0. This is an asymptotic z-test, not an exact
    /// binomial test or a Welch t-test.
    IndependentNormal {
        ratio: f64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerError {
    InvalidInput,
    NumericalRange,
    RootNotBracketed,
    DidNotConverge,
}

/// Both tails are evaluated directly: small upper tails are not `1 - cdf`.
#[derive(Clone, Copy, Debug)]
pub struct Tails {
    pub cdf: f64,
    pub sf: f64,
}

/// Noncentral-t beta-series representation. Sum around the Poisson mode to
/// avoid underflow of the first weight at large noncentrality. The omitted
/// weights lie more than 14 standard deviations beyond the mode, plus 60 terms.
/// This is a numerical implementation of the distribution, not a normal-power
/// approximation. Absolute-tail parity is tested separately from test power.
pub fn noncentral_t(t: f64, df: f64, nc: f64) -> Result<Tails, PowerError> {
    if !t.is_finite() || !df.is_finite() || df <= 0. || !nc.is_finite() {
        return Err(PowerError::InvalidInput);
    }
    if t < 0. {
        let p = noncentral_t(-t, df, -nc)?;
        return Ok(Tails {
            cdf: p.sf,
            sf: p.cdf,
        });
    }
    if t == 0. {
        return Ok(Tails {
            cdf: ndtr(-nc),
            sf: ndtr(nc),
        });
    }
    let lambda = nc * nc / 2.;
    if lambda > 1e6 || df > 1e7 {
        return Err(PowerError::NumericalRange);
    }
    let radius = 14. * lambda.sqrt() + 60.;
    let lo = (lambda - radius).max(0.) as usize;
    let hi = (lambda + radius).ceil() as usize;
    let x = t * t / (t * t + df);
    let xc = df / (t * t + df);
    if nc == 0. {
        let sf = 0.5 * incbet(df / 2., 0.5, xc);
        return Ok(Tails { cdf: 1. - sf, sf });
    }
    let mut cdf = ndtr(-nc);
    let mut sf = 0.;
    let log_weight = -lambda + lo as f64 * lambda.ln();
    let mut p = (log_weight - lgam(lo as f64 + 1.)).exp();
    let mut q = nc / 2_f64.sqrt() * (log_weight - lgam(lo as f64 + 1.5)).exp();
    for j in lo..=hi {
        let a = j as f64;
        cdf += 0.5 * (p * incbet(a + 0.5, df / 2., x) + q * incbet(a + 1., df / 2., x));
        sf += 0.5 * (p * incbet(df / 2., a + 0.5, xc) + q * incbet(df / 2., a + 1., xc));
        p *= lambda / (a + 1.);
        q *= lambda / (a + 1.5);
    }
    if !cdf.is_finite() || !sf.is_finite() || (cdf + sf - 1.).abs() > 1e-8 {
        return Err(PowerError::DidNotConverge);
    }
    Ok(Tails {
        cdf: cdf.clamp(0., 1.),
        sf: sf.clamp(0., 1.),
    })
}

/// For a symmetric two-sided rejection region the signed q-weights cancel:
/// P(|T|>c) = sum_j Poisson(nc^2/2,j) I_x(df/2,j+1/2).
/// Evaluate that identity directly instead of evaluating both signed tails.
fn two_sided_t(critical: f64, df: f64, nc: f64) -> Result<f64, PowerError> {
    let lambda = nc * nc / 2.;
    if !lambda.is_finite() || lambda > 1e6 || df > 1e7 {
        return Err(PowerError::NumericalRange);
    }
    let x = df / (df + critical * critical);
    if lambda == 0. {
        return Ok(incbet(df / 2., 0.5, x));
    }
    let radius = 14. * lambda.sqrt() + 60.;
    let lo = (lambda - radius).max(0.) as usize;
    let hi = (lambda + radius).ceil() as usize;
    let mut p = (-lambda + lo as f64 * lambda.ln() - lgam(lo as f64 + 1.)).exp();
    let mut sum = 0.;
    for j in lo..=hi {
        sum += p * incbet(df / 2., j as f64 + 0.5, x);
        p *= lambda / (j as f64 + 1.);
    }
    Ok(sum)
}

fn valid_alpha(alpha: f64) -> bool {
    alpha.is_finite() && alpha > 0. && alpha < 1.
}

pub(crate) fn student_t_upper_quantile(probability: f64, df: f64) -> f64 {
    let tail = probability.min(1. - probability);
    let beta = incbi(df / 2., 0.5, 2. * tail);
    (df * (1. - beta) / beta).sqrt() * if probability <= 0.5 { 1. } else { -1. }
}

/// Signed standardized mean difference; n2 = n1 * ratio. Fractional sizes are
/// supported for continuous root finding. Each group must contain at least two.
pub fn two_sample_power(
    d: f64,
    n1: f64,
    ratio: f64,
    alpha: f64,
    alternative: Alternative,
) -> Result<f64, PowerError> {
    let n2 = n1 * ratio;
    if !d.is_finite()
        || !n1.is_finite()
        || !n2.is_finite()
        || n1 < 2.
        || n2 < 2.
        || !valid_alpha(alpha)
    {
        return Err(PowerError::InvalidInput);
    }
    let df = n1 + n2 - 2.;
    t_rejection_power(
        d * (1. / (1. / n1 + 1. / n2)).sqrt(),
        df,
        alpha,
        alternative,
    )
}

fn t_rejection_power(
    nc: f64,
    df: f64,
    alpha: f64,
    alternative: Alternative,
) -> Result<f64, PowerError> {
    let probability = if alternative == Alternative::TwoSided {
        alpha / 2.
    } else {
        alpha
    };
    // Inverse beta gives a fractional-df Student-t quantile; the integer-only
    // Cephes stdtri API would round the df during sample-size root finding.
    let critical = student_t_upper_quantile(probability, df);
    let power = match alternative {
        Alternative::TwoSided => two_sided_t(critical, df, nc)?,
        Alternative::Larger => noncentral_t(critical, df, nc)?.sf,
        Alternative::Smaller => noncentral_t(-critical, df, nc)?.cdf,
    };
    // Summing independently evaluated tails can exceed one by roundoff.
    if !power.is_finite() || power > 1. + 1e-9 {
        return Err(PowerError::DidNotConverge);
    }
    Ok(power.clamp(0., 1.))
}

pub fn design_power(
    design: PowerDesign,
    d: f64,
    n: f64,
    alpha: f64,
    alternative: Alternative,
) -> Result<f64, PowerError> {
    if !d.is_finite() || !n.is_finite() || n < 2. || !valid_alpha(alpha) {
        return Err(PowerError::InvalidInput);
    }
    match design {
        PowerDesign::OneSampleT => t_rejection_power(d * n.sqrt(), n - 1., alpha, alternative),
        PowerDesign::IndependentT { ratio } => two_sample_power(d, n, ratio, alpha, alternative),
        PowerDesign::IndependentNormal { ratio } => {
            if !ratio.is_finite() || ratio <= 0. || !(n * ratio).is_finite() || n * ratio < 2. {
                return Err(PowerError::InvalidInput);
            }
            let shift = d * (1. / (1. / n + 1. / (n * ratio))).sqrt();
            if !shift.is_finite() {
                return Err(PowerError::NumericalRange);
            }
            let p = if alternative == Alternative::TwoSided {
                alpha / 2.
            } else {
                alpha
            };
            let critical = -ndtri(p);
            Ok(match alternative {
                Alternative::TwoSided => ndtr(shift - critical) + ndtr(-critical - shift),
                Alternative::Larger => ndtr(shift - critical),
                Alternative::Smaller => ndtr(-critical - shift),
            }
            .clamp(0., 1.))
        }
    }
}

/// Cohen's signed h. Use with IndependentNormal for the arcsine-transformed
/// two-proportion approximation. A raw risk difference is not Cohen's h.
pub fn proportion_effect_size(p1: f64, p2: f64) -> Result<f64, PowerError> {
    if !p1.is_finite() || !p2.is_finite() || !(0. ..=1.).contains(&p1) || !(0. ..=1.).contains(&p2)
    {
        return Err(PowerError::InvalidInput);
    }
    Ok(2. * (p1.sqrt().asin() - p2.sqrt().asin()))
}

fn solve<F: Fn(f64) -> Result<f64, PowerError>>(
    lower: f64,
    mut upper: f64,
    f: F,
) -> Result<f64, PowerError> {
    let low = f(lower)?;
    if low >= 0. {
        return Ok(lower);
    }
    for _ in 0..60 {
        if f(upper)? >= 0. {
            let root =
                crate::r_zeroin::r_zeroin2(lower, upper, 1e-10, 200, |x| f(x).unwrap_or(f64::NAN))
                    .map_err(|_| PowerError::DidNotConverge)?;
            // The shared solver does not signal iteration exhaustion; verify its
            // returned root rather than silently accepting an unfinished solve.
            if f(root)?.abs() > 1e-8 {
                return Err(PowerError::DidNotConverge);
            }
            return Ok(root);
        }
        upper *= 2.;
    }
    Err(PowerError::RootNotBracketed)
}

/// Smallest effect magnitude reaching target power. Smaller returns a negative
/// signed difference; TwoSided returns the positive branch. MDE in outcome units
/// equals this standardized effect times the assumed common standard deviation.
pub fn minimum_detectable_effect(
    n1: f64,
    ratio: f64,
    alpha: f64,
    target: f64,
    alternative: Alternative,
) -> Result<f64, PowerError> {
    minimum_detectable_effect_for(
        PowerDesign::IndependentT { ratio },
        n1,
        alpha,
        target,
        alternative,
    )
}

pub fn minimum_detectable_effect_for(
    design: PowerDesign,
    n: f64,
    alpha: f64,
    target: f64,
    alternative: Alternative,
) -> Result<f64, PowerError> {
    if !valid_alpha(target) || target <= alpha {
        return Err(PowerError::InvalidInput);
    }
    let sign = if alternative == Alternative::Smaller {
        -1.
    } else {
        1.
    };
    solve(0., 0.5, |d| {
        Ok(design_power(design, sign * d, n, alpha, alternative)? - target)
    })
    .map(|d| sign * d)
}

/// Continuous n1 solution, with n2 = ratio*n1. Actual study planning must round
/// group counts upward and recalculate power at those integer counts.
pub fn required_sample_size(
    d: f64,
    ratio: f64,
    alpha: f64,
    target: f64,
    alternative: Alternative,
) -> Result<f64, PowerError> {
    required_sample_size_for(
        PowerDesign::IndependentT { ratio },
        d,
        alpha,
        target,
        alternative,
    )
}

pub fn required_sample_size_for(
    design: PowerDesign,
    d: f64,
    alpha: f64,
    target: f64,
    alternative: Alternative,
) -> Result<f64, PowerError> {
    let ratio = match design {
        PowerDesign::OneSampleT => 1.,
        PowerDesign::IndependentT { ratio } | PowerDesign::IndependentNormal { ratio } => ratio,
    };
    if !ratio.is_finite()
        || ratio <= 0.
        || !valid_alpha(target)
        || target <= alpha
        || !d.is_finite()
        || d == 0.
        || (alternative == Alternative::Larger && d < 0.)
        || (alternative == Alternative::Smaller && d > 0.)
    {
        return Err(PowerError::InvalidInput);
    }
    let lower = 2_f64.max(2. / ratio);
    solve(lower, lower.max(10.) * 2., |n| {
        Ok(design_power(design, d, n, alpha, alternative)? - target)
    })
}
