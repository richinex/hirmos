//! BOOM CorrelationMap and BregVsSampler::attempt_swap.
//! Copyright Steven L. Scott and Google LLC; LGPL-2.1-or-later.
use super::*;
#[derive(Clone)]
pub(super) enum Correlations {
    Disabled,
    Enabled(Vec<Vec<(usize, f64)>>),
}
impl Correlations {
    pub(super) fn new(x: &DMatrix<f64>, threshold: f64) -> Result<Self, Error> {
        if !threshold.is_finite() || !(0.0..=1.0).contains(&threshold) {
            return Err(Error::InvalidProbability);
        }
        if threshold == 1.0 {
            return Ok(Self::Disabled);
        }
        let n = x.nrows() as f64;
        let center = DVector::from_fn(x.ncols(), |j, _| x.column(j).sum() / n);
        let covariance = (x.transpose() * x - &center * center.transpose() * n) / (n - 1.0);
        let sd = covariance.diagonal().map(|v| {
            let s = v.sqrt();
            if s <= 0.0 {
                1.0
            } else {
                s
            }
        });
        let entries = (0..x.ncols())
            .map(|i| {
                (0..x.ncols())
                    .filter_map(|j| {
                        let r = (covariance[(i, j)] / (sd[i] * sd[j])).abs();
                        if i != j && r >= threshold {
                            Some((j, r))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();
        Ok(Self::Enabled(entries))
    }
    fn candidates(&self, included: &[bool], index: usize) -> Vec<(usize, f64)> {
        let Self::Enabled(entries) = self else {
            return vec![];
        };
        let total: f64 = entries[index]
            .iter()
            .filter(|(j, _)| !included[*j])
            .map(|(_, w)| w)
            .sum();
        if total == 0.0 {
            return vec![];
        }
        entries[index]
            .iter()
            .filter(|(j, _)| !included[*j])
            .map(|(j, w)| (*j, w / total))
            .collect()
    }
}
impl Regression {
    pub fn with_swap_threshold(mut self, threshold: f64) -> Result<Self, Error> {
        self.swaps = Correlations::new(&self.x, threshold)?;
        Ok(self)
    }
    pub(super) fn attempt_swap(
        &self,
        included: &mut Vec<bool>,
        current: &mut Conditional,
        rng: &mut Random,
    ) -> Result<(), Error> {
        if matches!(self.swaps, Correlations::Disabled) {
            return Ok(());
        }
        let active: Vec<_> = included
            .iter()
            .enumerate()
            .filter_map(|(i, on)| on.then_some(i))
            .collect();
        if active.is_empty() || active.len() == included.len() {
            return Ok(());
        }
        let index = active[(rng.next_f64() * active.len() as f64) as usize];
        let candidates = self.swaps.candidates(included, index);
        if candidates.is_empty() {
            return Ok(());
        }
        let u = rng.next_f64();
        let mut cumulative = 0.0;
        let mut selected = *candidates.last().expect("nonempty candidates");
        for candidate in candidates {
            cumulative += candidate.1;
            if u < cumulative {
                selected = candidate;
                break;
            }
        }
        let (candidate, forward) = selected;
        let mut proposal = included.clone();
        proposal[index] = false;
        proposal[candidate] = true;
        let reverse = self
            .swaps
            .candidates(&proposal, candidate)
            .iter()
            .find(|(j, _)| *j == index)
            .map(|(_, w)| *w)
            .ok_or(Error::InvalidProbability)?;
        let conditional = self.conditional(&proposal)?;
        let ratio = conditional.log_weight - current.log_weight + reverse.ln() - forward.ln();
        if rng.next_f64().ln() < ratio {
            *included = proposal;
            *current = conditional;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn swap_proposals_match_boom_and_satisfy_detailed_balance() {
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../../fixtures/swaps.json")).unwrap();
        let x = DMatrix::from_fn(40, 4, |i, j| {
            let a = (i as f64 * 0.3).sin();
            let b = (i as f64 * 0.71).cos();
            match j {
                0 => 1.,
                1 => a,
                2 => 0.98 * a + 0.1 * b,
                _ => a + 0.2 * b,
            }
        });
        let map = Correlations::new(&x, 0.8).unwrap();
        let prior = Slab::new(
            DVector::zeros(4),
            DMatrix::identity(4, 4),
            vec![Inclusion::new(0.4).unwrap(); 4],
            Scale::new(0.5).unwrap(),
            10.,
        )
        .unwrap();
        let model = Regression::new(x.clone(), x.column(1).into_owned(), prior, 42).unwrap();
        for mask in 0..16 {
            let inc: Vec<_> = (0..4).map(|j| mask & (1 << j) != 0).collect();
            for i in 0..4 {
                if !inc[i] {
                    continue;
                }
                let candidates = map.candidates(&inc, i);
                for j in 0..4 {
                    let forward = candidates
                        .iter()
                        .find(|(k, _)| *k == j)
                        .map_or(0., |(_, w)| *w);
                    let expected = fixture["weights"][mask][i * 4 + j].as_f64().unwrap();
                    assert!((forward - expected).abs() < 1e-12, "mask {mask} {i}->{j}");
                    if forward == 0. {
                        continue;
                    }
                    let mut proposal = inc.clone();
                    proposal[i] = false;
                    proposal[j] = true;
                    let reverse = map
                        .candidates(&proposal, j)
                        .into_iter()
                        .find(|(k, _)| *k == i)
                        .unwrap()
                        .1;
                    let a = model.conditional(&inc).unwrap().log_weight;
                    let b = model.conditional(&proposal).unwrap().log_weight;
                    let ratio = b - a + reverse.ln() - forward.ln();
                    let forward_mass = a + forward.ln() + ratio.min(0.);
                    let reverse_mass = b + reverse.ln() + (-ratio).min(0.);
                    assert!((forward_mass - reverse_mass).abs() < 1e-12);
                }
            }
        }
        assert!(matches!(
            Correlations::new(&x, 1.).unwrap(),
            Correlations::Disabled
        ));
        assert!(Correlations::new(&x, f64::NAN).is_err());
    }
}
