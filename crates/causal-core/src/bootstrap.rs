//! Shared Tigramite DataFrame block sampling (GPL-3.0).
//! Extracted from the existing CausalEffects bootstrap port.
use crate::nprandom::NpRng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockLength {
    Fixed(usize),
    CubeRoot,
}

impl BlockLength {
    /// Match DataFrame's `max(1, int(n ** (1/3)))`, not rounded cube root.
    pub fn resolve(self, n: usize) -> usize {
        match self {
            Self::Fixed(length) => length,
            Self::CubeRoot => (crate::arm_pow::cube_power(n as u64) as usize).max(1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockError {
    ZeroBlockLength,
    TooFewBlocks { blocks: usize },
}

/// DataFrame copies this RNG state separately for every dataset and query.
#[derive(Clone, Debug)]
pub struct Bootstrap {
    length: BlockLength,
    rng: NpRng,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockSummary {
    pub length: usize,
    pub blocks: usize,
}

impl BlockSummary {
    pub fn warns_few_blocks(self) -> bool {
        self.blocks < 10
    }
}

impl Bootstrap {
    pub fn new(length: BlockLength, rng: NpRng) -> Result<Self, BlockError> {
        if length == BlockLength::Fixed(0) {
            return Err(BlockError::ZeroBlockLength);
        }
        Ok(Self { length, rng })
    }

    pub fn draw(&self, n: usize) -> Result<(Vec<usize>, BlockSummary), BlockError> {
        let length = self.length.resolve(n);
        let starts = block_starts(n, length, &mut self.rng.clone())?;
        let summary = BlockSummary {
            length,
            blocks: starts.len(),
        };
        Ok((sample_indices(n, length, &starts), summary))
    }
}

/// `choice(arange(n_obs - block_length), size=ceil(n_obs/block_length))`.
/// The caller owns RNG lifetime; DataFrame copies its RNG before each dataset.
pub fn block_starts(
    n_obs: usize,
    block_length: usize,
    rng: &mut NpRng,
) -> Result<Vec<usize>, BlockError> {
    if block_length == 0 {
        return Err(BlockError::ZeroBlockLength);
    }
    let blocks = n_obs.div_ceil(block_length);
    if blocks < 2 {
        return Err(BlockError::TooFewBlocks { blocks });
    }
    let high = n_obs - block_length;
    Ok((0..blocks)
        .map(|_| rng.bounded_uint64(high as u64) as usize)
        .collect())
}

/// Expand sampled blocks in order and truncate to the original sample count.
pub fn sample_indices(n_obs: usize, block_length: usize, starts: &[usize]) -> Vec<usize> {
    let mut indices = Vec::with_capacity(starts.len() * block_length);
    for &start in starts {
        for offset in 0..block_length {
            indices.push(start + offset);
        }
    }
    indices.truncate(n_obs);
    indices
}
