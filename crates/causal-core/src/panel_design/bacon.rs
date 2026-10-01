//! Unadjusted Goodman-Bacon decomposition, checked against bacondecomp 0.1.1.
//! Balanced regular panels, binary absorbing treatment, no observation weights.
use super::Panel;
use crate::estimation::{ols_two_way, WithinErrors, WithinProblem};
use nalgebra::DMatrix;
use std::collections::{BTreeMap, BTreeSet};
pub mod adjusted;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    TreatedVsNever { adoption: i64 },
    EarlierVsLater { earlier: i64, later: i64 },
    LaterVsEarlier { earlier: i64, later: i64 },
    LaterVsAlways { adoption: i64 },
}
#[derive(Debug)]
pub struct Component {
    pub comparison: Comparison,
    pub estimate: f64,
    pub weight: f64,
    pub observations: usize,
}
#[derive(Debug)]
pub struct Decomposition {
    pub components: Vec<Component>,
    pub twfe: f64,
    pub reconstructed: f64,
}
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidColumns,
    NonBinaryTreatment { row: usize },
    TreatmentReversal { unit: u64, period: i64 },
    NoComparisons,
    Regression(WithinProblem),
    Numerical,
    RankDeficientProjection,
    DegenerateWithin,
}

impl Panel {
    /// Outcome/treatment are distinct source columns. No covariate-adjusted or
    /// weighted request is accepted by this unadjusted entry point.
    pub fn bacon_unadjusted(
        &self,
        outcome: usize,
        treatment: usize,
    ) -> Result<Decomposition, Error> {
        if outcome == treatment
            || outcome >= self.values.ncols()
            || treatment >= self.values.ncols()
        {
            return Err(Error::InvalidColumns);
        }
        let first = self
            .keys
            .iter()
            .map(|k| k.1)
            .min()
            .ok_or(Error::NoComparisons)?;
        let adoption = self.bacon_adoption(treatment)?;
        let groups: BTreeSet<_> = adoption.values().copied().collect();
        let mut components = Vec::new();
        for g in groups.iter().flatten().copied().filter(|g| *g != first) {
            for &h in &groups {
                if h == Some(g) {
                    continue;
                }
                let comparison = match h {
                    None => Comparison::TreatedVsNever { adoption: g },
                    Some(h) if h == first => Comparison::LaterVsAlways { adoption: g },
                    Some(h) if g < h => Comparison::EarlierVsLater {
                        earlier: g,
                        later: h,
                    },
                    Some(h) => Comparison::LaterVsEarlier {
                        earlier: h,
                        later: g,
                    },
                };
                let rows: Vec<_> = self
                    .keys
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &(u, t))| {
                        let group = adoption[&u];
                        let window = match h {
                            None => true,
                            Some(h) if g < h => t < h,
                            Some(h) => t >= h,
                        };
                        ((group == Some(g) || group == h) && window).then_some(i)
                    })
                    .collect();
                let treated_rows: Vec<_> = rows
                    .iter()
                    .copied()
                    .filter(|i| adoption[&self.keys[*i].0] == Some(g))
                    .collect();
                let ng = treated_rows.len() as f64;
                let nh = (rows.len() - treated_rows.len()) as f64;
                let fraction = treated_rows
                    .iter()
                    .map(|i| self.values[(*i, treatment)])
                    .sum::<f64>()
                    / ng;
                let weight = ng * nh * fraction * (1. - fraction);
                if !weight.is_finite() || weight <= 0. {
                    return Err(Error::Numerical);
                }
                components.push(Component {
                    comparison,
                    estimate: self.bacon_twfe(&rows, outcome, treatment)?,
                    weight,
                    observations: rows.len(),
                });
            }
        }
        if components.is_empty() {
            return Err(Error::NoComparisons);
        }
        let total: f64 = components.iter().map(|c| c.weight).sum();
        if !total.is_finite() || total <= 0. {
            return Err(Error::Numerical);
        }
        for c in &mut components {
            c.weight /= total;
        }
        let reconstructed = components.iter().map(|c| c.weight * c.estimate).sum();
        let twfe = self.bacon_twfe(
            &(0..self.keys.len()).collect::<Vec<_>>(),
            outcome,
            treatment,
        )?;
        Ok(Decomposition {
            components,
            twfe,
            reconstructed,
        })
    }

    fn bacon_adoption(&self, treatment: usize) -> Result<BTreeMap<u64, Option<i64>>, Error> {
        let mut adoption = BTreeMap::<u64, Option<i64>>::new();
        for (i, &(unit, period)) in self.keys.iter().enumerate() {
            let d = self.values[(i, treatment)];
            if d != 0. && d != 1. {
                return Err(Error::NonBinaryTreatment {
                    row: self.original[i],
                });
            }
            let g = adoption.entry(unit).or_default();
            match (*g, d == 1.) {
                (None, true) => *g = Some(period),
                (Some(_), false) => return Err(Error::TreatmentReversal { unit, period }),
                _ => {}
            }
        }
        Ok(adoption)
    }

    fn bacon_twfe(&self, rows: &[usize], outcome: usize, treatment: usize) -> Result<f64, Error> {
        let y: Vec<_> = rows.iter().map(|i| self.values[(*i, outcome)]).collect();
        let x = DMatrix::from_fn(rows.len(), 1, |i, _| self.values[(rows[i], treatment)]);
        let units: Vec<_> = rows.iter().map(|i| self.keys[*i].0).collect();
        // Signed period labels become opaque unsigned labels, preserving equality.
        let times: Vec<_> = rows.iter().map(|i| self.keys[*i].1 as u64).collect();
        let fit = ols_two_way(&y, &x, &units, &times, WithinErrors::Classical)
            .map_err(Error::Regression)?;
        match fit.params.as_slice() {
            [value] if value.is_finite() => Ok(*value),
            _ => Err(Error::Numerical),
        }
    }
}
