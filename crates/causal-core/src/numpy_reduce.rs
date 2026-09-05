//! NumPy's `add.reduce` order for contiguous float64 data: pairwise summation inside each
//! 8,192-element buffer block, with the blocks accumulated in sequence from zero. `np.mean`,
//! `pandas.Series.mean` and the centred product inside `np.cov` all reduce this way, so a port that
//! has to match them to the last bit must sum in the same order rather than left to right.

const BUFFER_BLOCK: usize = 8192;
const PAIRWISE_BLOCK: usize = 128;

fn pairwise_sum(values: &[f64]) -> f64 {
    let n = values.len();
    if n < 8 {
        return values.iter().fold(0.0, |total, value| total + value);
    }
    if n <= PAIRWISE_BLOCK {
        let mut lanes = [0.0; 8];
        lanes.copy_from_slice(&values[..8]);
        let unrolled = n - n % 8;
        let mut index = 8;
        while index < unrolled {
            for (lane, value) in lanes.iter_mut().zip(&values[index..index + 8]) {
                *lane += value;
            }
            index += 8;
        }
        let mut total = ((lanes[0] + lanes[1]) + (lanes[2] + lanes[3]))
            + ((lanes[4] + lanes[5]) + (lanes[6] + lanes[7]));
        for value in &values[unrolled..] {
            total += value;
        }
        return total;
    }
    let mut half = n / 2;
    half -= half % 8;
    pairwise_sum(&values[..half]) + pairwise_sum(&values[half..])
}

/// `np.sum` / `np.add.reduce` over a contiguous float64 array.
pub fn numpy_sum(values: &[f64]) -> f64 {
    values
        .chunks(BUFFER_BLOCK)
        .fold(0.0, |total, block| total + pairwise_sum(block))
}

/// `np.mean` over a contiguous float64 array; an empty input is NaN, as in NumPy.
pub fn numpy_mean(values: &[f64]) -> f64 {
    numpy_sum(values) / values.len() as f64
}
