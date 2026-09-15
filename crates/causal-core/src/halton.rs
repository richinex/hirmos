//! SciPy 1.13.1's scrambled Halton sequence for integer seeds.
//! Sources: stats/_qmc.py and reference/scipy-qmc-1.13.1.pyx (BSD-3-Clause).

use crate::nprandom::NpRng;

#[derive(Clone, Debug)]
struct Digits {
    base: usize,
    permutations: Vec<Vec<usize>>,
}

/// Scrambled, non-optimized sequence with a private position and digit permutations.
#[derive(Clone, Debug)]
pub struct Halton {
    digits: Vec<Digits>,
    position: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SequenceExhausted;

impl Halton {
    pub fn seeded(dimensions: usize, seed: u64) -> Self {
        let mut rng = NpRng::seeded(seed);
        let mut primes = Vec::with_capacity(dimensions);
        let mut candidate = 2;
        while primes.len() < dimensions {
            if primes
                .iter()
                .take_while(|&&p| p <= candidate / p)
                .all(|p| candidate % p != 0)
            {
                primes.push(candidate);
            }
            candidate += 1;
        }
        let digits = primes
            .into_iter()
            .map(|base| {
                let count = (54.0 / (base as f64).log2()).ceil() as usize - 1;
                let permutations = (0..count).map(|_| rng.permutation(base)).collect();
                Digits { base, permutations }
            })
            .collect();
        Self {
            digits,
            position: 0,
        }
    }

    /// Return rows in source order; repeated draws continue the same sequence.
    pub fn draw(&mut self, count: usize) -> Result<Vec<Vec<f64>>, SequenceExhausted> {
        let end = self.position.checked_add(count).ok_or(SequenceExhausted)?;
        let rows = (self.position..end)
            .map(|index| {
                self.digits
                    .iter()
                    .map(|digits| {
                        let mut quotient = index;
                        let mut factor = 1.0 / digits.base as f64;
                        let mut value = 0.0;
                        for permutation in &digits.permutations {
                            value =
                                (permutation[quotient % digits.base] as f64).mul_add(factor, value);
                            factor /= digits.base as f64;
                            quotient /= digits.base;
                        }
                        value
                    })
                    .collect()
            })
            .collect();
        self.position = end;
        Ok(rows)
    }
}
