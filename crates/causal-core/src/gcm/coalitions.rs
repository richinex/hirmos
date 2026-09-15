//! Insertion-only CPython 3.12 64-bit set order for binary tuples.
//! GCM assigns random seeds in this order, so Rust's HashSet is not interchangeable.
//! Sources: reference/cpython-3.12.11-{setobject,tupleobject}.c (PSF license).

#[derive(Clone)]
struct Entry<K> {
    hash: u64,
    key: K,
}

pub(super) struct PythonSet<K> {
    slots: Vec<Option<Entry<K>>>,
    used: usize,
}

fn tuple_hash(key: &[bool]) -> u64 {
    const PRIME1: u64 = 11400714785074694791;
    const PRIME2: u64 = 14029467366897019727;
    const PRIME5: u64 = 2870177450012600261;
    let mut hash = PRIME5;
    for &value in key {
        hash = hash.wrapping_add((value as u64).wrapping_mul(PRIME2));
        hash = hash.rotate_left(31).wrapping_mul(PRIME1);
    }
    hash = hash.wrapping_add(key.len() as u64 ^ (PRIME5 ^ 3527539));
    if hash == u64::MAX {
        1546275796
    } else {
        hash
    }
}

impl<K: Clone + Eq> PythonSet<K> {
    pub fn new() -> Self {
        Self {
            slots: vec![None; 8],
            used: 0,
        }
    }

    fn slot(&self, hash: u64, key: &K) -> usize {
        let mask = self.slots.len() - 1;
        let mut index = (hash & mask as u64) as usize;
        let mut perturb = hash;
        loop {
            let probes = if index + 9 <= mask { 9 } else { 0 };
            for offset in 0..=probes {
                match &self.slots[index + offset] {
                    None => return index + offset,
                    Some(entry) if entry.hash == hash && &entry.key == key => {
                        return index + offset
                    }
                    Some(_) => {}
                }
            }
            perturb >>= 5;
            index = ((index as u64)
                .wrapping_mul(5)
                .wrapping_add(1)
                .wrapping_add(perturb)
                & mask as u64) as usize;
        }
    }

    pub fn insert(&mut self, key: K, hash: u64) {
        let index = self.slot(hash, &key);
        if self.slots[index].is_some() {
            return;
        }
        self.slots[index] = Some(Entry { hash, key });
        self.used += 1;
        if self.used * 5 < (self.slots.len() - 1) * 3 {
            return;
        }
        let minimum = self.used * if self.used > 50000 { 2 } else { 4 };
        let size = minimum.next_power_of_two();
        let size = if size == minimum { size * 2 } else { size };
        let previous = std::mem::replace(&mut self.slots, vec![None; size]);
        for entry in previous.into_iter().flatten() {
            let index = self.slot(entry.hash, &entry.key);
            self.slots[index] = Some(entry);
        }
    }

    pub fn difference_order(&self, excluded: &[K]) -> Vec<K> {
        if self.used >> 2 > excluded.len() {
            // Fewer than used/4 deletions cannot trigger a tombstone purge.
            return self
                .copy_set()
                .into_order()
                .into_iter()
                .filter(|k| !excluded.contains(k))
                .collect();
        }
        let mut result = Self::new();
        for entry in self.slots.iter().flatten() {
            if !excluded.contains(&entry.key) {
                result.insert(entry.key.clone(), entry.hash);
            }
        }
        result.into_order()
    }

    fn copy_set(&self) -> Self {
        let mut result = Self::new();
        if self.used * 5 >= 7 * 3 {
            let mut size = 8;
            while size <= self.used * 2 {
                size *= 2;
            }
            result.slots = vec![None; size];
        }
        result.used = self.used;
        if result.slots.len() == self.slots.len() {
            result.slots = self.slots.clone();
            return result;
        }
        for entry in self.slots.iter().flatten() {
            let index = result.slot(entry.hash, &entry.key);
            result.slots[index] = Some(entry.clone());
        }
        result
    }

    pub fn into_order(self) -> Vec<K> {
        self.slots
            .into_iter()
            .flatten()
            .map(|entry| entry.key)
            .collect()
    }
}

pub(super) struct Coalitions(PythonSet<Vec<bool>>);

impl Coalitions {
    pub fn new() -> Self {
        Self(PythonSet::new())
    }
    pub fn insert(&mut self, key: Vec<bool>) {
        let hash = tuple_hash(&key);
        self.0.insert(key, hash);
    }
    pub fn into_order(self) -> Vec<Vec<bool>> {
        self.0.into_order()
    }
}
