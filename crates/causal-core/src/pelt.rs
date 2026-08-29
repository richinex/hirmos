//! `ruptures.Pelt` with `CostL2` and `CostRbf`, following `Pelt._seg`.

/// Segment cost over a half open range.
pub trait SegmentCost {
    fn min_size(&self) -> usize;
    fn error(&self, start: usize, end: usize) -> f64;
}

/// `ruptures.costs.CostL2`: summed per feature variance times the segment length.
pub struct CostL2 {
    signal: Vec<Vec<f64>>,
    /// Cumulative sums and sums of squares per feature, so a segment costs O(features).
    cumsum: Vec<Vec<f64>>,
    cumsq: Vec<Vec<f64>>,
}

impl CostL2 {
    pub fn new(signal: &[Vec<f64>]) -> Self {
        let n = signal.len();
        let k = signal[0].len();
        let mut cumsum = vec![vec![0.0; k]; n + 1];
        let mut cumsq = vec![vec![0.0; k]; n + 1];
        for t in 0..n {
            for j in 0..k {
                cumsum[t + 1][j] = cumsum[t][j] + signal[t][j];
                cumsq[t + 1][j] = cumsq[t][j] + signal[t][j] * signal[t][j];
            }
        }
        CostL2 {
            signal: signal.to_vec(),
            cumsum,
            cumsq,
        }
    }
}

impl SegmentCost for CostL2 {
    fn min_size(&self) -> usize {
        1
    }
    fn error(&self, start: usize, end: usize) -> f64 {
        let len = (end - start) as f64;
        let k = self.signal[0].len();
        let mut total = 0.0;
        for j in 0..k {
            let sum = self.cumsum[end][j] - self.cumsum[start][j];
            let sq = self.cumsq[end][j] - self.cumsq[start][j];
            // Population variance times the length is the residual sum of squares.
            total += sq - sum * sum / len;
        }
        total
    }
}

/// `ruptures.costs.CostRbf`: a Gaussian kernel Gram matrix with the median heuristic for
/// gamma, the scaled squared distances clipped to [1e-2, 1e2] before exponentiating.
pub struct CostRbf {
    gram: Vec<Vec<f64>>,
    /// Prefix sums over the Gram matrix so a segment costs O(1).
    prefix: Vec<Vec<f64>>,
}

impl CostRbf {
    pub fn new(signal: &[Vec<f64>]) -> Self {
        let n = signal.len();
        let k = signal[0].len();
        let sqdist = |a: usize, b: usize| -> f64 {
            (0..k).map(|j| (signal[a][j] - signal[b][j]).powi(2)).sum()
        };
        // The median heuristic runs over the condensed upper triangle, as pdist returns it.
        let mut condensed = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in i + 1..n {
                condensed.push(sqdist(i, j));
            }
        }
        let mut sorted = condensed.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let m = sorted.len();
        let median = if m == 0 {
            0.0
        } else if m % 2 == 1 {
            sorted[m / 2]
        } else {
            (sorted[m / 2 - 1] + sorted[m / 2]) / 2.0
        };
        let gamma = if median != 0.0 { 1.0 / median } else { 1.0 };

        // squareform leaves the diagonal at zero, so the Gram diagonal is exp(0) = 1.
        let mut gram = vec![vec![1.0; n]; n];
        let mut idx = 0;
        for i in 0..n {
            for j in i + 1..n {
                let scaled = (condensed[idx] * gamma).clamp(1e-2, 1e2);
                let value = (-scaled).exp();
                gram[i][j] = value;
                gram[j][i] = value;
                idx += 1;
            }
        }
        let mut prefix = vec![vec![0.0; n + 1]; n + 1];
        for i in 0..n {
            for j in 0..n {
                prefix[i + 1][j + 1] =
                    gram[i][j] + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
            }
        }
        CostRbf { gram, prefix }
    }
}

impl SegmentCost for CostRbf {
    fn min_size(&self) -> usize {
        1
    }
    fn error(&self, start: usize, end: usize) -> f64 {
        let len = (end - start) as f64;
        // The Gram diagonal is all ones, so the diagonal sum is just the length.
        let block = self.prefix[end][end] - self.prefix[start][end] - self.prefix[end][start]
            + self.prefix[start][start];
        let _ = &self.gram;
        len - block / len
    }
}

/// PELT with a given penalty. Returns the breakpoints, ending with the sample count, in the
/// same form as `ruptures.Pelt.predict`.
pub fn pelt(
    cost: &dyn SegmentCost,
    n_samples: usize,
    min_size: usize,
    jump: usize,
    pen: f64,
) -> Vec<usize> {
    let min_size = min_size.max(cost.min_size());
    // Total cost of the best partition of [0, t), and the breakpoint that achieved it.
    let mut total: Vec<Option<f64>> = vec![None; n_samples + 1];
    let mut back: Vec<usize> = vec![0; n_samples + 1];
    total[0] = Some(0.0);

    let mut ind: Vec<usize> = (0..n_samples)
        .step_by(jump)
        .filter(|k| *k >= min_size)
        .collect();
    ind.push(n_samples);

    let mut admissible: Vec<usize> = Vec::new();
    for bkp in ind {
        let new_adm = ((bkp - min_size) / jump) * jump;
        admissible.push(new_adm);

        // Only candidates that already have a partition contribute a subproblem.
        let mut sums: Vec<f64> = Vec::new();
        let mut sources: Vec<usize> = Vec::new();
        for &t in &admissible {
            if let Some(base) = total[t] {
                sums.push(base + cost.error(t, bkp) + pen);
                sources.push(t);
            }
        }
        if sums.is_empty() {
            continue;
        }
        // Python's min keeps the first minimum.
        let mut best = 0usize;
        for i in 1..sums.len() {
            if sums[i] < sums[best] {
                best = i;
            }
        }
        total[bkp] = Some(sums[best]);
        back[bkp] = sources[best];

        // ruptures prunes by zipping the admissible list against the subproblems, which
        // stops at the shorter of the two and misaligns when a candidate was skipped.
        let threshold = sums[best] + pen;
        let kept: Vec<usize> = admissible
            .iter()
            .zip(&sums)
            .filter(|(_, &s)| s <= threshold)
            .map(|(&t, _)| t)
            .collect();
        admissible = kept;
    }

    // Backtrack from the end.
    let mut bkps = Vec::new();
    let mut at = n_samples;
    while at > 0 {
        bkps.push(at);
        at = back[at];
    }
    bkps.reverse();
    bkps
}

/// Convenience wrapper for the least squares cost.
pub fn pelt_l2(signal: &[Vec<f64>], min_size: usize, jump: usize, pen: f64) -> Vec<usize> {
    let cost = CostL2::new(signal);
    pelt(&cost, signal.len(), min_size, jump, pen)
}

/// Convenience wrapper for the RBF kernel cost.
pub fn pelt_rbf(signal: &[Vec<f64>], min_size: usize, jump: usize, pen: f64) -> Vec<usize> {
    let cost = CostRbf::new(signal);
    pelt(&cost, signal.len(), min_size, jump, pen)
}
