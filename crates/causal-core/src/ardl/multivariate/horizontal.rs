//! R ARDL's horizontal search with explicit starting orders and PSS AIC scoring.
use super::{fit_levels, Error, Input, Specification};
use crate::ardl::Trend;
use std::collections::{HashMap, HashSet};

pub struct HorizontalSearch {
    maximum: Vec<usize>,
    fixed: Vec<Option<usize>>,
    starting: Vec<usize>,
    trend: Trend,
    hold_back: usize,
    budget: usize,
}

pub struct RankedOrder {
    pub order: Vec<usize>,
    pub aic_pss: f64,
}

impl HorizontalSearch {
    pub fn new(
        maximum: Vec<usize>,
        fixed: Vec<Option<usize>>,
        starting: Vec<usize>,
        trend: Trend,
        hold_back: usize,
        budget: usize,
    ) -> Result<Self, Error> {
        if maximum.len() < 2
            || maximum.len() != fixed.len()
            || maximum.len() != starting.len()
            || starting[0] == 0
            || fixed[0] == Some(0)
            || maximum
                .iter()
                .zip(&starting)
                .any(|(max, start)| start > max)
            || maximum
                .iter()
                .zip(&fixed)
                .any(|(max, fixed)| fixed.is_some_and(|q| q > *max))
        {
            return Err(Error::OrderMismatch);
        }
        if maximum.iter().any(|q| *q > hold_back) {
            return Err(Error::HoldBackTooShort);
        }
        if budget == 0 {
            return Err(Error::SearchLimit);
        }
        let starting = starting
            .iter()
            .zip(&fixed)
            .map(|(start, fixed)| fixed.unwrap_or(*start))
            .collect();
        Ok(Self {
            maximum,
            fixed,
            starting,
            trend,
            hold_back,
            budget,
        })
    }

    pub fn run(
        &self,
        input: &Input<'_>,
        mut proceed: impl FnMut(usize) -> bool,
    ) -> Result<Vec<RankedOrder>, Error> {
        let mut cache = HashMap::<Vec<usize>, f64>::new();
        let mut seen = HashSet::new();
        let mut rows = Vec::new();
        let starts: Vec<usize> = match self.fixed[0] {
            Some(p) => vec![p],
            None => (self.starting[0]..=self.maximum[0]).collect(),
        };
        let mut evaluate = |order: &[usize]| -> Result<f64, Error> {
            if !proceed(cache.len()) {
                return Err(Error::Cancelled);
            }
            if let Some(score) = cache.get(order) {
                return Ok(*score);
            }
            if cache.len() >= self.budget {
                return Err(Error::SearchLimit);
            }
            let spec = Specification::new(
                order[0],
                order[1..].iter().copied().map(Some).collect(),
                self.trend,
                Some(self.hold_back),
            )?;
            let fit = fit_levels(input, &spec)?;
            let regression = fit.regression();
            let n = regression.nobs as f64;
            let variance = regression.resid.norm_squared() / n;
            let score = -0.5 * n * ((2.0 * std::f64::consts::PI * variance).ln() + 1.0)
                - regression.params.len() as f64;
            if !score.is_finite() {
                return Err(Error::NonFiniteResult);
            }
            cache.insert(order.to_vec(), score);
            Ok(score)
        };
        for p in starts {
            let mut current = self.starting.clone();
            current[0] = p;
            let mut score = evaluate(&current)?;
            if seen.insert(current.clone()) {
                rows.push(RankedOrder {
                    order: current.clone(),
                    aic_pss: score,
                });
            }
            let mut failed = HashSet::from([vec![0; current.len()]]);
            let mut converged = self.fixed[1..].iter().all(Option::is_some);
            while !converged {
                for j in 1..current.len() {
                    if self.fixed[j].is_some() || current[j] >= self.maximum[j] {
                        if current
                            .iter()
                            .zip(&self.fixed)
                            .zip(&self.maximum)
                            .skip(1)
                            .all(|((q, f), m)| Some(*q) == *f || q == m)
                        {
                            converged = true;
                        }
                        continue;
                    }
                    let mut back = current.clone();
                    let mut forth = current.clone();
                    back[j] = back[j].saturating_sub(1);
                    forth[j] += 1;
                    let back_score = evaluate(&back)?;
                    let forth_score = evaluate(&forth)?;
                    let (next, next_score) = if back_score >= forth_score {
                        (back, back_score)
                    } else {
                        (forth, forth_score)
                    };
                    if seen.insert(next.clone()) {
                        rows.push(RankedOrder {
                            order: next.clone(),
                            aic_pss: next_score,
                        });
                    }
                    if next_score > score {
                        current = next;
                        score = next_score;
                    } else if !failed.insert(next) {
                        converged = true;
                    }
                }
            }
        }
        rows.sort_by(|a, b| b.aic_pss.total_cmp(&a.aic_pss));
        rows.truncate(20);
        Ok(rows)
    }
}
