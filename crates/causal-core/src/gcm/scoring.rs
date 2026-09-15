//! DoWhy v0.14 MedianCDFQuantileScorer, with exact rank counts.

/// A fitted scorer. NaNs participate only in equality counts, as in numpy.isclose.
pub struct MedianCdf {
    ordered: Vec<f64>,
    missing: usize,
}

impl MedianCdf {
    pub fn fit(samples: &[f64]) -> Self {
        let mut ordered: Vec<_> = samples.iter().copied().filter(|v| !v.is_nan()).collect();
        ordered.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self {
            missing: samples.len() - ordered.len(),
            ordered,
        }
    }

    pub fn score(&self, value: f64) -> f64 {
        let (below, above, equal) = if value.is_nan() {
            (0, 0, self.missing + 1)
        } else {
            let below = self.ordered.partition_point(|v| *v < value);
            let end = self.ordered.partition_point(|v| *v <= value);
            (below, self.ordered.len() - end, end - below + 1)
        };
        let left = below as f64 + equal as f64 / 2.0;
        let right = above as f64 + equal as f64 / 2.0;
        1.0 - 2.0 * left.min(right) / (self.ordered.len() + self.missing + 1) as f64
    }
}
