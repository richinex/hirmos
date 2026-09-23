//! Interval meaning is explicit; unavailable uniform bands never become pointwise.
use super::{bootstrap, Summary};
use nalgebra::DMatrix;

#[derive(Debug, Clone, Copy)]
pub struct Confidence(f64);
impl Confidence {
    pub fn new(level: f64) -> Result<Self, Error> {
        let tail = 0.5 + level / 2.0;
        if !level.is_finite() || level <= 0.0 || level >= 1.0 || tail >= 1.0 {
            return Err(Error::Confidence);
        }
        Ok(Self(level))
    }
}

pub enum Method {
    Analytical,
    BootstrapPointwise { iterations: usize, seed: u32 },
    BootstrapSimultaneous { iterations: usize, seed: u32 },
}

pub enum Point<'a> {
    Reference,
    Estimated(&'a Summary),
}

#[derive(Debug, PartialEq)]
pub enum Interval {
    Reference,
    Unavailable {
        att: f64,
    },
    Estimated {
        att: f64,
        se: f64,
        lower: f64,
        upper: f64,
    },
}

#[derive(Debug, PartialEq)]
pub enum Coverage {
    Pointwise,
    Simultaneous { critical: f64, large_critical: bool },
}

#[derive(Debug)]
pub struct Inference {
    pub confidence: f64,
    pub coverage: Coverage,
    pub intervals: Vec<Interval>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Confidence,
    Shape,
    NonFinite,
    AnalyticalClustering,
    Bootstrap(bootstrap::Error),
    UnavailableSimultaneousBand,
    BandNarrowerThanPointwise,
}

/// The caller supplies one inferential family, with unit-aligned influence rows.
/// Extra clustering requires bootstrap, matching did's analytical restriction.
pub fn infer(
    points: &[Point<'_>],
    clusters: &[u64],
    confidence: Confidence,
    method: Method,
) -> Result<Inference, Error> {
    let n = clusters.len();
    if n < 2 || points.is_empty() {
        return Err(Error::Shape);
    }
    let mut influence = DMatrix::zeros(n, points.len());
    for (j, point) in points.iter().enumerate() {
        if let Point::Estimated(s) = point {
            if s.influence.len() != n {
                return Err(Error::Shape);
            }
            if !s.att.is_finite() || s.influence.iter().any(|v| !v.is_finite()) {
                return Err(Error::NonFinite);
            }
            for (i, v) in s.influence.iter().enumerate() {
                influence[(i, j)] = *v;
            }
        }
    }
    let normal = spec_math::cephes64::ndtri(0.5 + confidence.0 / 2.0);
    let (standard_errors, critical, coverage) = match method {
        Method::Analytical => {
            let distinct: std::collections::BTreeSet<_> = clusters.iter().collect();
            if distinct.len() != n {
                return Err(Error::AnalyticalClustering);
            }
            let se = (0..points.len())
                .map(|j| {
                    let value = influence.column(j).norm() / n as f64;
                    (value > f64::EPSILON.sqrt() * 10.0).then_some(value)
                })
                .collect::<Vec<_>>();
            (se, normal, Coverage::Pointwise)
        }
        Method::BootstrapPointwise { iterations, seed } => {
            let result = bootstrap::run(&influence, clusters, iterations, 1.0 - confidence.0, seed)
                .map_err(Error::Bootstrap)?;
            (result.se, normal, Coverage::Pointwise)
        }
        Method::BootstrapSimultaneous { iterations, seed } => {
            let result = bootstrap::run(&influence, clusters, iterations, 1.0 - confidence.0, seed)
                .map_err(Error::Bootstrap)?;
            let critical = result.critical.ok_or(Error::UnavailableSimultaneousBand)?;
            if critical < normal {
                return Err(Error::BandNarrowerThanPointwise);
            }
            (
                result.se,
                critical,
                Coverage::Simultaneous {
                    critical,
                    large_critical: critical >= 7.0,
                },
            )
        }
    };
    let intervals = points
        .iter()
        .zip(standard_errors)
        .map(|(point, se)| match point {
            Point::Reference => Interval::Reference,
            Point::Estimated(s) => match se {
                Some(se) if se.is_finite() && se > 0.0 => Interval::Estimated {
                    att: s.att,
                    se,
                    lower: s.att - critical * se,
                    upper: s.att + critical * se,
                },
                _ => Interval::Unavailable { att: s.att },
            },
        })
        .collect::<Vec<_>>();
    if intervals.iter().any(|i| matches!(i,Interval::Estimated{lower,upper,..} if !lower.is_finite() || !upper.is_finite())) { return Err(Error::NonFinite); }
    Ok(Inference {
        confidence: confidence.0,
        coverage,
        intervals,
    })
}
