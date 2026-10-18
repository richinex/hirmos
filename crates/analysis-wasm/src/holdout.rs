//! A seeded split of logged rows into rows a copier trains on and rows its copy is scored on.
use hirmos_causal_core::nprandom::NpRng;
use serde::Deserialize;

/// Hold out `share` of the rows, drawn without replacement by a generator seeded with `seed`.
#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Holdout {
    share: f64,
    seed: u64,
}

impl Holdout {
    /// For each row, whether it is held out; the same rows, share and seed always give the same split.
    pub(crate) fn mask(self, rows: usize) -> Result<Vec<bool>, String> {
        if !(self.share > 0.0 && self.share < 1.0) {
            return Err(format!("the held-out share {} must lie strictly between 0 and 1", self.share));
        }
        let held = (rows as f64 * self.share).round() as usize;
        if held == 0 || held == rows {
            return Err(format!("holding out {} of {rows} rows leaves no rows on one side", self.share));
        }
        let mut mask = vec![false; rows];
        for row in NpRng::seeded(self.seed).permutation(rows).into_iter().take(held) {
            mask[row] = true;
        }
        Ok(mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_split_is_repeatable_and_holds_out_the_requested_share() {
        let split = Holdout { share: 0.2, seed: 9 };
        let first = split.mask(1_000).unwrap();
        assert_eq!(first, split.mask(1_000).unwrap());
        assert_eq!(first.iter().filter(|&&held| held).count(), 200);
    }
}
