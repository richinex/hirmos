//! CondIndTest's content-key cache and persistent shuffle RNG.
use super::*;
use crate::{nprandom::NpRng, parcorr_mult::ShufflePlan};
use sha1::{Digest, Sha1};
use std::{cell::RefCell, collections::BTreeMap};

type Key = [[u8; 20]; 3];

pub struct ShuffleCi<'a> {
    pub(crate) data: &'a JointData,
    correlation: Correlation,
    plan: ShufflePlan,
    state: RefCell<State>,
    residuals: Residuals,
}

struct State {
    rng: NpRng,
    results: BTreeMap<Key, (f64, f64)>,
}

impl JointArray {
    /// Source `_get_array_hash`: sort nodes within each role, hash contiguous
    /// row bytes, then sort the two X/Y hashes to allow symmetric cache hits.
    pub fn ci_key(&self) -> Key {
        let mut hashes = [[0; 20]; 3];
        for (slot, (role, nodes)) in [
            (Role::X, &self.cleaned.x),
            (Role::Y, &self.cleaned.y),
            (Role::Z, &self.cleaned.z),
        ]
        .into_iter()
        .enumerate()
        {
            let rows: Vec<_> = self
                .values
                .iter()
                .zip(&self.roles)
                .filter_map(|(row, r)| (*r == role).then_some(row))
                .collect();
            let mut order: Vec<_> = (0..nodes.len()).collect();
            order.sort_by_key(|&i| nodes[i]);
            let mut hash = Sha1::new();
            for i in order {
                for value in rows[i] {
                    hash.update(value.to_le_bytes());
                }
            }
            hashes[slot] = hash.finalize().into();
        }
        if hashes[0] > hashes[1] {
            hashes.swap(0, 1);
        }
        hashes
    }
}

impl JointData {
    pub fn shuffle(&self, correlation: Correlation, plan: ShufflePlan, seed: u64) -> ShuffleCi<'_> {
        ShuffleCi {
            data: self,
            correlation,
            plan,
            state: RefCell::new(State {
                rng: NpRng::seeded(seed),
                results: BTreeMap::new(),
            }),
            residuals: Residuals::Fresh,
        }
    }
}

impl ShuffleCi<'_> {
    pub fn with_residual_recycling(mut self) -> Self {
        self.residuals = self.data.recycling_policy();
        self
    }
    pub fn cached_tests(&self) -> usize {
        self.state.borrow().results.len()
    }

    pub fn test(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau: usize,
    ) -> Result<(f64, f64), JointError> {
        let array = self.data.test_array(x, y, z, tau)?;
        let key = array.ci_key();
        let mut state = self.state.borrow_mut();
        if let Some(result) = state.results.get(&key) {
            return Ok(*result);
        }
        let samples = array.samples()?;
        self.data.check_recycling(&samples, self.residuals)?;
        let result = samples
            .run_shuffle_test(self.correlation, &self.plan, &mut state.rng)
            .map_err(JointError::Samples)?;
        state.results.insert(key, result);
        Ok(result)
    }
}
