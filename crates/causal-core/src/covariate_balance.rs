//! Binary-treatment balance, following cobalt 5.0.0's col_w_smd.

#[derive(Clone, Copy, Debug)]
pub enum Denominator {
    Pooled,
    Treated,
    Control,
}
#[derive(Clone, Copy, Debug)]
pub enum CovariateKind {
    Continuous,
    Binary,
}
#[derive(Clone, Copy, Debug)]
pub enum Scale {
    Raw,
    Standardized(Denominator),
}
#[derive(Debug, PartialEq)]
pub enum Error {
    Length,
    NonFinite,
    InvalidBinary,
    InvalidWeights,
    MissingArm,
    NumericalRange,
}
#[derive(Debug)]
pub struct Balance {
    pub before: f64,
    pub after: Option<f64>,
    pub denominator: f64,
    pub used_full_sample_spread: bool,
}

fn variance(x: &[f64], kind: CovariateKind) -> f64 {
    let mean = x.iter().sum::<f64>() / x.len() as f64;
    match kind {
        CovariateKind::Binary => mean * (1.0 - mean),
        CovariateKind::Continuous => {
            x.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (x.len() as f64 - 1.0)
        }
    }
}

pub fn balance(
    x: &[f64],
    treated: &[bool],
    weights: Option<&[f64]>,
    kind: CovariateKind,
    scale: Scale,
) -> Result<Balance, Error> {
    if x.is_empty() || x.len() != treated.len() || weights.is_some_and(|w| w.len() != x.len()) {
        return Err(Error::Length);
    }
    if x.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    if matches!(kind, CovariateKind::Binary) && x.iter().any(|&v| v != 0.0 && v != 1.0) {
        return Err(Error::InvalidBinary);
    }
    if weights.is_some_and(|w| w.iter().any(|v| !v.is_finite() || *v < 0.0)) {
        return Err(Error::InvalidWeights);
    }
    let arm = |t| {
        x.iter()
            .zip(treated)
            .filter_map(|(&v, &a)| (a == t).then_some(v))
            .collect::<Vec<_>>()
    };
    let t = arm(true);
    let c = arm(false);
    if t.is_empty() || c.is_empty() {
        return Err(Error::MissingArm);
    }
    let tolerance = f64::EPSILON.sqrt();
    let mut denominator = match scale {
        Scale::Raw => 1.0,
        Scale::Standardized(d) => match d {
            Denominator::Pooled => ((variance(&t, kind) + variance(&c, kind)) / 2.0).sqrt(),
            Denominator::Treated => variance(&t, kind).sqrt(),
            Denominator::Control => variance(&c, kind).sqrt(),
        },
    };
    // cobalt substitutes full-sample spread for a degenerate reference arm.
    let used_full_sample_spread = !denominator.is_finite() || denominator.abs() < tolerance;
    if used_full_sample_spread {
        denominator = variance(x, kind).sqrt();
    }
    let standardize = |diff: f64| {
        if diff.abs() < tolerance {
            diff
        } else {
            diff / denominator
        }
    };
    let before = standardize(
        t.iter().sum::<f64>() / t.len() as f64 - c.iter().sum::<f64>() / c.len() as f64,
    );
    let after = if let Some(w) = weights {
        let mean = |arm| -> Result<f64, Error> {
            let max = w
                .iter()
                .zip(treated)
                .filter_map(|(&v, &a)| (a == arm).then_some(v))
                .fold(0.0, f64::max);
            if max == 0.0 {
                return Err(Error::MissingArm);
            }
            let (sum, total) = x
                .iter()
                .zip(treated)
                .zip(w)
                .filter(|((_, a), _)| **a == arm)
                .fold((0.0, 0.0), |(s, n), ((v, _), w)| (s + v * w, n + w));
            if !sum.is_finite() || !total.is_finite() {
                return Err(Error::NumericalRange);
            }
            Ok(sum / total)
        };
        Some(standardize(mean(true)? - mean(false)?))
    } else {
        None
    };
    if !before.is_finite() || after.is_some_and(|v| !v.is_finite()) || !denominator.is_finite() {
        return Err(Error::NumericalRange);
    }
    Ok(Balance {
        before,
        after,
        denominator,
        used_full_sample_spread,
    })
}
