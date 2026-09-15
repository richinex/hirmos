//! Node-label hashing for a reproducible CPython 3.12 PYTHONHASHSEED=0 oracle.
//! SipHash13 source: reference/cpython-3.12.11-pyhash.c (PSF license).

use super::coalitions::PythonSet;

pub fn difference_order(names: &[String], excluded: &[usize]) -> Vec<usize> {
    let mut set = PythonSet::new();
    for (index, name) in names.iter().enumerate() {
        set.insert(index, label_hash(name));
    }
    set.difference_order(excluded)
}

pub fn label_hash(value: &str) -> u64 {
    if value.is_empty() {
        return 0;
    }
    let maximum = value.chars().map(|c| c as u32).max().unwrap();
    let bytes: Vec<_> = if maximum <= 255 {
        value.chars().map(|c| c as u8).collect()
    } else if maximum <= 65535 {
        value
            .chars()
            .flat_map(|c| (c as u16).to_le_bytes())
            .collect()
    } else {
        value
            .chars()
            .flat_map(|c| (c as u32).to_le_bytes())
            .collect()
    };
    let mut v = [
        0x736f6d6570736575_u64,
        0x646f72616e646f6d,
        0x6c7967656e657261,
        0x7465646279746573,
    ];
    let mut chunks = bytes.chunks_exact(8);
    for chunk in &mut chunks {
        let word = u64::from_le_bytes(chunk.try_into().unwrap());
        v[3] ^= word;
        round(&mut v);
        v[0] ^= word;
    }
    let mut tail = (bytes.len() as u64) << 56;
    for (i, &byte) in chunks.remainder().iter().enumerate() {
        tail |= (byte as u64) << (i * 8);
    }
    v[3] ^= tail;
    round(&mut v);
    v[0] ^= tail;
    v[2] ^= 0xff;
    for _ in 0..3 {
        round(&mut v);
    }
    let hash = v.into_iter().fold(0, |a, b| a ^ b);
    if hash == u64::MAX {
        u64::MAX - 1
    } else {
        hash
    }
}

fn round(v: &mut [u64; 4]) {
    v[0] = v[0].wrapping_add(v[1]);
    v[2] = v[2].wrapping_add(v[3]);
    v[1] = v[1].rotate_left(13) ^ v[0];
    v[3] = v[3].rotate_left(16) ^ v[2];
    v[0] = v[0].rotate_left(32);
    v[2] = v[2].wrapping_add(v[1]);
    v[0] = v[0].wrapping_add(v[3]);
    v[1] = v[1].rotate_left(17) ^ v[2];
    v[3] = v[3].rotate_left(21) ^ v[0];
    v[2] = v[2].rotate_left(32);
}
