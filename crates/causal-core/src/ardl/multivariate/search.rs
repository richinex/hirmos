//! Contiguous-order search from statsmodels ardl_select_order (glob=False).

use super::{design, deterministic, Error, Input, Specification, Term};
use crate::ardl::{numpy_lstsq, Trend};
use nalgebra::{DMatrix, DVector};

#[derive(Clone, Copy)]
pub enum Criterion {
    Aic,
    Bic,
    Hqic,
}

pub struct Search {
    maximum: Specification,
    criterion: Criterion,
    candidates: usize,
    counts: Vec<Vec<usize>>,
}

pub struct Candidate {
    pub outcome_lag: usize,
    pub predictor_lags: Vec<Option<usize>>,
    pub aic: f64,
    pub bic: f64,
    pub hqic: f64,
}

pub struct Selection {
    pub specification: Specification,
    pub candidates: Vec<Candidate>,
    pub start_row: usize,
}

impl Search {
    pub fn new(
        outcome_lag: usize,
        predictor_lags: Vec<usize>,
        trend: Trend,
        hold_back: Option<usize>,
        criterion: Criterion,
        budget: usize,
    ) -> Result<Self, Error> {
        let count = predictor_lags
            .iter()
            .try_fold(
                outcome_lag.checked_add(1).ok_or(Error::SearchLimit)?,
                |total, q| total.checked_mul(q.checked_add(2)?),
            )
            .ok_or(Error::SearchLimit)?;
        if count > budget {
            return Err(Error::SearchLimit);
        }
        Ok(Self {
            counts: std::iter::once((0..=outcome_lag).collect())
                .chain(predictor_lags.iter().map(|q| (0..=q + 1).collect()))
                .collect(),
            maximum: Specification::new(
                outcome_lag,
                predictor_lags.into_iter().map(Some).collect(),
                trend,
                hold_back,
            )?,
            criterion,
            candidates: count,
        })
    }

    /// Full grid with a minimum AR order and fixed predictor orders, as used by
    /// R ARDL's grid search. Every predictor remains included, even at lag zero.
    pub fn restricted(
        minimum_lag: usize,
        maximum_lag: usize,
        maximum_orders: Vec<usize>,
        fixed_orders: Vec<Option<usize>>,
        trend: Trend,
        hold_back: Option<usize>,
        criterion: Criterion,
        budget: usize,
    ) -> Result<Self, Error> {
        if minimum_lag == 0
            || minimum_lag > maximum_lag
            || maximum_orders.len() != fixed_orders.len()
            || maximum_orders
                .iter()
                .zip(&fixed_orders)
                .any(|(max, fixed)| fixed.is_some_and(|q| q > *max))
        {
            return Err(Error::OrderMismatch);
        }
        let counts: Vec<Vec<usize>> = std::iter::once((minimum_lag..=maximum_lag).collect())
            .chain(
                maximum_orders
                    .iter()
                    .zip(fixed_orders)
                    .map(|(max, fixed)| match fixed {
                        Some(q) => vec![q + 1],
                        None => (1..=max + 1).collect(),
                    }),
            )
            .collect();
        let candidates = counts
            .iter()
            .try_fold(1usize, |total, choices| total.checked_mul(choices.len()))
            .ok_or(Error::SearchLimit)?;
        if candidates > budget {
            return Err(Error::SearchLimit);
        }
        Ok(Self {
            maximum: Specification::new(
                maximum_lag,
                maximum_orders.into_iter().map(Some).collect(),
                trend,
                hold_back,
            )?,
            criterion,
            candidates,
            counts,
        })
    }

    pub fn run(
        &self,
        input: &Input<'_>,
        mut proceed: impl FnMut(usize, usize) -> bool,
    ) -> Result<Selection, Error> {
        let start = self.maximum.sample_start(input, false)?;
        let mut always_terms = deterministic(self.maximum.trend);
        always_terms.extend((0..input.fixed.len()).map(|column| Term::Fixed { column }));
        let always = design(input, &always_terms, start)?;
        let mut term_blocks = vec![(1..=self.maximum.outcome_lag)
            .map(|lag| Term::Outcome { lag })
            .collect::<Vec<_>>()];
        for (column, order) in self.maximum.predictor_lags.iter().enumerate() {
            term_blocks.push(
                (0..=order.unwrap())
                    .map(|lag| Term::Predictor { column, lag })
                    .collect(),
            );
        }
        let column_count = always.ncols() + term_blocks.iter().map(Vec::len).sum::<usize>();
        if input.outcome.len() - start <= column_count {
            return Err(Error::InsufficientRows);
        }
        let mut blocks: Vec<DMatrix<f64>> = term_blocks
            .iter()
            .map(|t| design(input, t, start))
            .collect::<Result<_, _>>()?;
        let mut y = DVector::from_column_slice(&input.outcome[start..]);
        if always.ncols() > 0 {
            let pinv = crate::linalg::pseudo_inverse(&always, 1e-15)
                .map_err(|_| Error::Regression(crate::ols::OlsError::DecompositionFailed))?
                .matrix;
            for block in &mut blocks {
                *block -= &always * (&pinv * &*block);
            }
            y -= &always * (&pinv * &y);
        }
        let radices: Vec<usize> = self.counts.iter().map(Vec::len).collect();
        let mut candidates = Vec::with_capacity(self.candidates);
        let mut best = None;
        let mut lowest = f64::INFINITY;
        for number in 0..self.candidates {
            if !proceed(number, self.candidates) {
                return Err(Error::Cancelled);
            }
            let mut remainder = number;
            let mut counts = vec![0; radices.len()];
            for index in (0..counts.len()).rev() {
                counts[index] = self.counts[index][remainder % radices[index]];
                remainder /= radices[index];
            }
            let selected: Vec<(usize, usize)> = counts
                .iter()
                .enumerate()
                .flat_map(|(block, count)| (0..*count).map(move |column| (block, column)))
                .collect();
            let x = DMatrix::from_fn(y.len(), selected.len(), |r, c| {
                blocks[selected[c].0][(r, selected[c].1)]
            });
            let residual = if selected.is_empty() {
                y.clone()
            } else {
                &y - &x * numpy_lstsq(&x, &y)
            };
            let n = y.len() as f64;
            let sigma2 = residual.iter().map(|v| v * v).sum::<f64>() / n;
            let deviance = n * ((2.0 * std::f64::consts::PI * sigma2).ln() + 1.0);
            let df = (always.ncols() + selected.len() + 1) as f64;
            let candidate = Candidate {
                outcome_lag: counts[0],
                predictor_lags: counts[1..].iter().map(|c| c.checked_sub(1)).collect(),
                aic: deviance + 2.0 * df,
                bic: deviance + n.ln() * df,
                hqic: deviance + 2.0 * n.ln().ln() * df,
            };
            let score = match self.criterion {
                Criterion::Aic => candidate.aic,
                Criterion::Bic => candidate.bic,
                Criterion::Hqic => candidate.hqic,
            };
            if !score.is_finite() {
                return Err(Error::NonFiniteResult);
            }
            if score < lowest {
                lowest = score;
                best = Some(number);
            }
            candidates.push(candidate);
        }
        let chosen = &candidates[best.ok_or(Error::NonFiniteResult)?];
        let specification = Specification::new(
            chosen.outcome_lag,
            chosen.predictor_lags.clone(),
            self.maximum.trend,
            self.maximum.hold_back,
        )?;
        Ok(Selection {
            specification,
            candidates,
            start_row: start,
        })
    }
}
