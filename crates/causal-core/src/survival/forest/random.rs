//! mt19937_64 and libstdc++'s integer sampling used by the pinned ranger oracle.

#[derive(Clone)]
pub(super) struct Random {
    engine: crate::mt19937_64::Mt19937_64,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            engine: crate::mt19937_64::Mt19937_64::new(seed),
        }
    }

    fn next(&mut self) -> u64 {
        self.engine.next()
    }

    pub fn index(&mut self, count: usize) -> usize {
        self.index64(count as u64) as usize
    }

    pub fn index64(&mut self, count: u64) -> u64 {
        assert!(count > 0);
        loop {
            let product = self.next() as u128 * count as u128;
            let low = product as u64;
            if low >= count || low >= count.wrapping_neg() % count {
                return (product >> 64) as u64;
            }
        }
    }

    pub fn unit(&mut self) -> f64 {
        (self.next() as f64 / 18446744073709551616.0).min(f64::from_bits(1.0_f64.to_bits() - 1))
    }

    pub fn features(&mut self, columns: usize, count: usize) -> Vec<usize> {
        if count < columns / 10 {
            let mut selected = vec![false; columns];
            let mut result = Vec::with_capacity(count);
            while result.len() < count {
                let column = self.index(columns);
                if !selected[column] {
                    selected[column] = true;
                    result.push(column);
                }
            }
            return result;
        }
        let mut result: Vec<usize> = (0..columns).collect();
        for i in 0..count {
            let j = i + (self.unit() * (columns - i) as f64) as usize;
            result.swap(i, j);
        }
        result.truncate(count);
        result
    }

    /// Match libstdc++'s paired-draw shuffle; NumPy uses a different permutation stream.
    pub fn shuffle<T>(&mut self, values: &mut [T]) {
        if values.is_empty() {
            return;
        }
        let mut i = 1;
        if values.len() % 2 == 0 {
            values.swap(i, self.index(2));
            i += 1;
        }
        while i < values.len() {
            let range = i + 1;
            let draw = self.index64(range as u64 * (range + 1) as u64);
            values.swap(i, (draw / (range + 1) as u64) as usize);
            values.swap(i + 1, (draw % (range + 1) as u64) as usize);
            i += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ranger_inbag_counts() {
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../../../oracle/fixtures/ranger.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            if case["sampling"]["kind"] != "uniform"
                || case["sampling"]["fraction"] != 1
                || case["sampling"]["replacement"] != true
            {
                continue;
            }
            let seed = case["seed"].as_u64().unwrap();
            for (tree, expected) in case["expected"]["inbag"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
            {
                let expected: Vec<usize> = serde_json::from_value(expected.clone()).unwrap();
                let mut random = Random::new(((tree + 1) as u64 * seed) as u32 as u64);
                let mut counts = vec![0; expected.len()];
                for _ in 0..expected.len() {
                    counts[random.index(expected.len())] += 1;
                }
                assert_eq!(counts, expected, "{} tree {tree}", case["name"]);
            }
        }
    }
}
