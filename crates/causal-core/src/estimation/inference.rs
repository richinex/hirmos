//! Linear restrictions on an already fitted coefficient vector and covariance.
//! Callers choose the reference distribution and degrees of freedom explicitly.
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Copy, Debug)]
pub enum Reference {
    Asymptotic,
    Student { degrees_of_freedom: f64 },
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    Confidence,
    DegreesOfFreedom,
    InvalidCovariance,
    SingularRestriction,
    Numerical,
}

pub struct Contrast {
    pub estimate: f64,
    pub standard_error: f64,
    pub statistic: f64,
    pub p_value: f64,
    pub interval: [f64; 2],
}

pub enum JointTest {
    ChiSquared {
        statistic: f64,
        degrees_of_freedom: usize,
        p_value: f64,
    },
    F {
        statistic: f64,
        numerator_df: usize,
        denominator_df: f64,
        p_value: f64,
    },
}

fn validate_reference(reference: Reference) -> Result<(), Error> {
    match reference {
        Reference::Asymptotic => Ok(()),
        Reference::Student {
            degrees_of_freedom: df,
        } if df.is_finite() && df > 0.0 => Ok(()),
        Reference::Student { .. } => Err(Error::DegreesOfFreedom),
    }
}

fn validate(beta: &[f64], covariance: &DMatrix<f64>) -> Result<(), Error> {
    let p = beta.len();
    if p == 0 || covariance.shape() != (p, p) {
        return Err(Error::Shape);
    }
    if beta.iter().chain(covariance.iter()).any(|x| !x.is_finite()) {
        return Err(Error::NonFinite);
    }
    let scale = covariance.iter().fold(0.0_f64, |a, x| a.max(x.abs()));
    let tolerance = scale * 128.0 * p as f64 * f64::EPSILON;
    for i in 0..p {
        for j in 0..p {
            if (covariance[(i, j)] - covariance[(j, i)]).abs() > tolerance {
                return Err(Error::InvalidCovariance);
            }
        }
    }
    if crate::linalg::eigenvalues_symmetric_lower(covariance)
        .map_err(|_| Error::Numerical)?
        .iter()
        .any(|x| *x < -tolerance)
    {
        return Err(Error::InvalidCovariance);
    }
    Ok(())
}

/// Test c'beta = null. The interval is for c'beta, not its deviation from null.
pub fn contrast(
    beta: &[f64],
    covariance: &DMatrix<f64>,
    c: &[f64],
    null: f64,
    confidence: f64,
    reference: Reference,
) -> Result<Contrast, Error> {
    validate(beta, covariance)?;
    validate_reference(reference)?;
    if c.len() != beta.len() {
        return Err(Error::Shape);
    }
    if !null.is_finite() || c.iter().any(|x| !x.is_finite()) {
        return Err(Error::NonFinite);
    }
    if !confidence.is_finite() || confidence <= 0.0 || confidence >= 1.0 {
        return Err(Error::Confidence);
    }
    let c = DVector::from_column_slice(c);
    let estimate = c.dot(&DVector::from_column_slice(beta));
    let variance = (c.transpose() * covariance * &c)[(0, 0)];
    if !variance.is_finite() {
        return Err(Error::Numerical);
    }
    if variance <= 0.0 {
        return Err(Error::SingularRestriction);
    }
    let standard_error = variance.sqrt();
    let statistic = (estimate - null) / standard_error;
    let (critical, p_value) = match reference {
        Reference::Asymptotic => (
            spec_math::cephes64::ndtri(0.5 + confidence / 2.0),
            libm::erfc(statistic.abs() / std::f64::consts::SQRT_2),
        ),
        Reference::Student {
            degrees_of_freedom: df,
        } => {
            let z = spec_math::cephes64::incbi(df / 2.0, 0.5, 1.0 - confidence);
            (
                (df * (1.0 - z) / z).sqrt(),
                spec_math::cephes64::incbet(df / 2.0, 0.5, df / (df + statistic * statistic)),
            )
        }
    };
    let interval = [
        estimate - critical * standard_error,
        estimate + critical * standard_error,
    ];
    if [estimate, statistic, p_value, interval[0], interval[1]]
        .iter()
        .any(|x| !x.is_finite())
    {
        return Err(Error::Numerical);
    }
    Ok(Contrast {
        estimate,
        standard_error,
        statistic,
        p_value,
        interval,
    })
}

/// Test R beta = q. Redundant or covariance-singular restrictions are refused,
/// not silently assigned a reduced rank or a different reference distribution.
pub fn joint(
    beta: &[f64],
    covariance: &DMatrix<f64>,
    restrictions: &DMatrix<f64>,
    null: &[f64],
    reference: Reference,
) -> Result<JointTest, Error> {
    validate(beta, covariance)?;
    validate_reference(reference)?;
    let q = restrictions.nrows();
    if q == 0 || q > beta.len() || restrictions.ncols() != beta.len() || null.len() != q {
        return Err(Error::Shape);
    }
    if restrictions.iter().chain(null).any(|x| !x.is_finite()) {
        return Err(Error::NonFinite);
    }
    let delta = restrictions * DVector::from_column_slice(beta) - DVector::from_column_slice(null);
    let v = restrictions * covariance * restrictions.transpose();
    if v.iter().chain(delta.iter()).any(|x| !x.is_finite()) {
        return Err(Error::Numerical);
    }
    let eigen = crate::linalg::eigenvalues_symmetric_lower(&v).map_err(|_| Error::Numerical)?;
    let largest = eigen.iter().copied().fold(0.0_f64, f64::max);
    if eigen
        .iter()
        .any(|x| *x <= largest * q as f64 * f64::EPSILON)
    {
        return Err(Error::SingularRestriction);
    }
    let rhs = DMatrix::from_column_slice(q, 1, delta.as_slice());
    let solved = crate::linalg::solve(&v, &rhs).map_err(|_| Error::SingularRestriction)?;
    let wald = delta.dot(&solved.column(0));
    if !wald.is_finite() || wald < 0.0 {
        return Err(Error::Numerical);
    }
    Ok(match reference {
        Reference::Asymptotic => JointTest::ChiSquared {
            statistic: wald,
            degrees_of_freedom: q,
            p_value: spec_math::cephes64::chdtrc(q as f64, wald),
        },
        Reference::Student {
            degrees_of_freedom: df,
        } => JointTest::F {
            statistic: wald / q as f64,
            numerator_df: q,
            denominator_df: df,
            p_value: spec_math::cephes64::incbet(df / 2.0, q as f64 / 2.0, df / (df + wald)),
        },
    })
}
