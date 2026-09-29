//! setup_pte_cont: first-period positive doses, type-7 quantiles.
use crate::continuous_did::{panel, quantile::type_seven_quantile};

#[derive(Debug, PartialEq)]
pub enum Error {
    NoPositiveDose,
    InvalidDose,
    Allocation,
}
pub struct Design {
    pub grid: Vec<f64>,
    pub knots: Vec<f64>,
}

pub fn from_first_period(doses: &[f64], num_knots: usize) -> Result<Design, Error> {
    if doses.iter().any(|d| !d.is_finite() || *d < 0.0) {
        return Err(Error::InvalidDose);
    }
    let mut sorted: Vec<_> = doses.iter().copied().filter(|d| *d > 0.0).collect();
    if sorted.is_empty() {
        return Err(Error::NoPositiveDose);
    }
    sorted.sort_by(f64::total_cmp);
    let denominator = num_knots.checked_add(1).ok_or(Error::Allocation)?;
    let mut knots = Vec::new();
    knots
        .try_reserve_exact(num_knots)
        .map_err(|_| Error::Allocation)?;
    for i in 1..=num_knots {
        knots.push(type_seven_quantile(&sorted, i as f64 / denominator as f64));
    }
    // Upstream executes seq(.1,.99,.01), despite its "1st percentile" comment.
    let grid = (0..90)
        .map(|i| type_seven_quantile(&sorted, 0.1 + i as f64 * 0.01))
        .collect();
    Ok(Design { grid, knots })
}

#[derive(Debug)]
pub enum FitError {
    Design(Error),
    Panel(panel::Error),
    Empty,
}
pub struct PreparedFit {
    pub design: Design,
    pub fit: panel::Fit,
}

/// The package default is cubic with no interior knots.
pub fn fit(rows: &[panel::Row], degree: usize, num_knots: usize) -> Result<PreparedFit, FitError> {
    let first = rows
        .iter()
        .map(|r| r.observation.period)
        .min()
        .ok_or(FitError::Empty)?;
    let doses: Vec<_> = rows
        .iter()
        .filter(|r| r.observation.period == first)
        .map(|r| r.dose)
        .collect();
    let design = from_first_period(&doses, num_knots).map_err(FitError::Design)?;
    let fit =
        panel::fit(rows, &design.grid, degree, design.knots.clone()).map_err(FitError::Panel)?;
    Ok(PreparedFit { design, fit })
}
