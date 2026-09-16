//! Unweighted continuous thresholds and bin lookup from scikit-learn 1.9.0.
// Derived from scikit-learn (BSD-3-Clause); see reference/sklearn-1.9.0-gcm/licenses/COPYING.

#[derive(Debug, PartialEq, Eq)]
pub enum BinError {
    Count,
    Empty,
    Infinite,
}

/// A fitted mapper reserves max_bins for missing values.
#[derive(Clone)]
pub struct Bins {
    thresholds: Vec<f64>,
    missing: u8,
}

impl Bins {
    pub fn fit(values: &[f64], max_bins: usize) -> Result<Self, BinError> {
        if !(2..=255).contains(&max_bins) {
            return Err(BinError::Count);
        }
        if values.is_empty() {
            return Err(BinError::Empty);
        }
        if values.iter().any(|v| v.is_infinite()) {
            return Err(BinError::Infinite);
        }
        let mut sorted: Vec<_> = values.iter().copied().filter(|v| !v.is_nan()).collect();
        sorted.sort_by(f64::total_cmp);
        let mut distinct = sorted.clone();
        distinct.dedup();
        let mut thresholds = if distinct.len() <= max_bins {
            distinct
                .windows(2)
                .map(|v| (v[0] + v[1]) / 2.0)
                .collect::<Vec<_>>()
        } else {
            (1..max_bins)
                .map(|i| {
                    let quantile = (i as f64 * (100.0 / max_bins as f64)) / 100.0;
                    let index = sorted.len() as f64 * quantile - 1.0;
                    let low = index.floor();
                    let a = sorted[(low.max(0.0) as usize).min(sorted.len() - 1)];
                    let b = sorted[((low + 1.0).max(0.0) as usize).min(sorted.len() - 1)];
                    if index - low == 0.0 {
                        b - (b - a) * 0.5
                    } else {
                        b
                    }
                })
                .collect::<Vec<_>>()
        };
        thresholds.dedup();
        for value in &mut thresholds {
            *value = value.min(1e300);
        }
        Ok(Self {
            thresholds,
            missing: max_bins as u8,
        })
    }
    pub fn thresholds(&self) -> &[f64] {
        &self.thresholds
    }
    pub fn map(&self, value: f64) -> u8 {
        if value.is_nan() {
            self.missing
        } else {
            self.thresholds
                .partition_point(|&threshold| threshold < value) as u8
        }
    }
}
