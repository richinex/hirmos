//! Ranger's bootstrap and subsampling plans.
use super::{random::Random, Error};

#[derive(Clone, Copy)]
pub enum Replacement {
    With,
    Without,
}

pub enum Sampling {
    Uniform {
        fraction: f64,
        replacement: Replacement,
    },
    Weighted {
        fraction: f64,
        replacement: Replacement,
        weights: Vec<f64>,
    },
    Manual {
        counts: Vec<Vec<usize>>,
    },
}

pub(super) enum Plan {
    Uniform {
        rows: usize,
        draws: usize,
        replacement: Replacement,
    },
    Weighted {
        cumulative: Vec<f64>,
        draws: usize,
        replacement: Replacement,
    },
    Manual(Vec<Vec<usize>>),
}

impl Plan {
    pub fn new(sampling: Sampling, rows: usize, trees: usize) -> Result<Self, Error> {
        let count = |fraction: f64| {
            if !fraction.is_finite()
                || fraction <= 0.0
                || fraction > 1.0
                || (rows as f64 * fraction) < 1.0
            {
                return Err(Error::InvalidSettings);
            }
            Ok((rows as f64 * fraction) as usize)
        };
        match sampling {
            Sampling::Uniform {
                fraction,
                replacement,
            } => Ok(Self::Uniform {
                rows,
                draws: count(fraction)?,
                replacement,
            }),
            Sampling::Weighted {
                fraction,
                replacement,
                weights,
            } => {
                let draws = count(fraction)?;
                let sum: f64 = weights.iter().sum();
                if weights.len() != rows
                    || weights.iter().any(|w| !w.is_finite() || *w < 0.0)
                    || !sum.is_finite()
                    || sum <= 0.0
                    || (matches!(replacement, Replacement::Without)
                        && weights.iter().filter(|&&w| w > 0.0).count() < draws)
                {
                    return Err(Error::InvalidSettings);
                }
                let mut total = 0.0;
                let mut cumulative: Vec<f64> = weights
                    .iter()
                    .map(|w| {
                        total += w / sum;
                        total
                    })
                    .collect();
                *cumulative.last_mut().unwrap() = 1.0;
                Ok(Self::Weighted {
                    cumulative,
                    draws,
                    replacement,
                })
            }
            Sampling::Manual { counts } => {
                if counts.len() != trees
                    || counts.iter().any(|tree| {
                        tree.len() != rows
                            || tree
                                .iter()
                                .try_fold(0usize, |a, &b| a.checked_add(b))
                                .is_none_or(|n| n == 0)
                    })
                {
                    return Err(Error::InvalidSettings);
                }
                Ok(Self::Manual(counts))
            }
        }
    }

    pub fn draw(&self, tree: usize, random: &mut Random) -> (Vec<usize>, Vec<usize>) {
        match self {
            Self::Uniform {
                rows,
                draws,
                replacement: Replacement::Without,
            } => {
                let mut samples: Vec<usize> = (0..*rows).collect();
                // shuffleAndSplit takes its generator by value in ranger.
                random.clone().shuffle(&mut samples);
                let oob = samples.split_off(*draws);
                (samples, oob)
            }
            Self::Uniform {
                rows,
                draws,
                replacement: Replacement::With,
            } => {
                let samples: Vec<usize> = (0..*draws).map(|_| random.index(*rows)).collect();
                let oob = outside(&samples, *rows);
                (samples, oob)
            }
            Self::Weighted {
                cumulative,
                draws,
                replacement,
            } => {
                let mut used = vec![false; cumulative.len()];
                let mut samples = Vec::with_capacity(*draws);
                while samples.len() < *draws {
                    let draw = random.unit();
                    let row = cumulative.partition_point(|v| *v < draw);
                    if matches!(replacement, Replacement::Without) && used[row] {
                        continue;
                    }
                    samples.push(row);
                    used[row] = true;
                }
                let oob = used
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &used)| (!used).then_some(i))
                    .collect();
                (samples, oob)
            }
            Self::Manual(counts) => {
                let mut samples = Vec::new();
                let mut oob = Vec::new();
                for (i, &n) in counts[tree].iter().enumerate() {
                    if n == 0 {
                        oob.push(i);
                    }
                    samples.extend(std::iter::repeat_n(i, n));
                }
                random.shuffle(&mut samples);
                (samples, oob)
            }
        }
    }
}

fn outside(samples: &[usize], rows: usize) -> Vec<usize> {
    let mut used = vec![false; rows];
    for &row in samples {
        used[row] = true;
    }
    used.iter()
        .enumerate()
        .filter_map(|(i, &used)| (!used).then_some(i))
        .collect()
}
