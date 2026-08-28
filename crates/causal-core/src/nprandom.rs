//! numpy's default_rng ported 1:1: SeedSequence entropy pooling, the PCG64 XSL-RR bit generator
//! with its uint32 buffer, and Generator's masked-rejection shuffle.

const INIT_A: u32 = 0x43b0d7e5;
const MULT_A: u32 = 0x931e8875;
const INIT_B: u32 = 0x8b51f9dd;
const MULT_B: u32 = 0x58f38ded;
const MIX_MULT_L: u32 = 0xca01f9dd;
const MIX_MULT_R: u32 = 0x4973f715;
const XSHIFT: u32 = 16;
const PCG_MULT: u128 = 0x2360ed051fc65da44385df649fccf645;

fn hashmix(value: u32, hash_const: &mut u32) -> u32 {
    let mut value = value ^ *hash_const;
    *hash_const = hash_const.wrapping_mul(MULT_A);
    value = value.wrapping_mul(*hash_const);
    value ^= value >> XSHIFT;
    value
}

fn mix(x: u32, y: u32) -> u32 {
    let mut result = MIX_MULT_L
        .wrapping_mul(x)
        .wrapping_sub(MIX_MULT_R.wrapping_mul(y));
    result ^= result >> XSHIFT;
    result
}

pub struct SeedSequence {
    pool: [u32; 4],
}

impl SeedSequence {
    pub fn new(entropy: &[u32]) -> Self {
        let mut pool = [0u32; 4];
        let mut hash_const = INIT_A;
        for (i, slot) in pool.iter_mut().enumerate() {
            let v = if i < entropy.len() { entropy[i] } else { 0 };
            *slot = hashmix(v, &mut hash_const);
        }
        for i_src in 0..4 {
            for i_dst in 0..4 {
                if i_src != i_dst {
                    pool[i_dst] = mix(pool[i_dst], hashmix(pool[i_src], &mut hash_const));
                }
            }
        }
        for &word in entropy.iter().skip(4) {
            for slot in &mut pool {
                *slot = mix(*slot, hashmix(word, &mut hash_const));
            }
        }
        SeedSequence { pool }
    }

    pub fn generate_u64(&self, n_words: usize) -> Vec<u64> {
        let mut hash_const = INIT_B;
        let words: Vec<u32> = (0..n_words * 2)
            .map(|i| {
                let mut v = self.pool[i % 4];
                v ^= hash_const;
                hash_const = hash_const.wrapping_mul(MULT_B);
                v = v.wrapping_mul(hash_const);
                v ^= v >> XSHIFT;
                v
            })
            .collect();
        words
            .chunks(2)
            .map(|c| c[0] as u64 | (c[1] as u64) << 32)
            .collect()
    }
}

/// PCG64 (setseq 128 XSL-RR) exactly as numpy seeds and steps it, uint32 buffer included.
pub struct NpRng {
    state: u128,
    inc: u128,
    has_u32: bool,
    buffered: u32,
}

impl NpRng {
    pub fn seeded(seed: u64) -> Self {
        let mut entropy = Vec::new();
        let mut v = seed;
        while v > 0 {
            entropy.push(v as u32);
            v >>= 32;
        }
        if entropy.is_empty() {
            entropy.push(0);
        }
        let words = SeedSequence::new(&entropy).generate_u64(4);
        let initstate = (words[0] as u128) << 64 | words[1] as u128;
        let initseq = (words[2] as u128) << 64 | words[3] as u128;
        let mut rng = NpRng {
            state: 0,
            inc: (initseq << 1) | 1,
            has_u32: false,
            buffered: 0,
        };
        rng.step();
        rng.state = rng.state.wrapping_add(initstate);
        rng.step();
        rng
    }

    fn step(&mut self) {
        self.state = self.state.wrapping_mul(PCG_MULT).wrapping_add(self.inc);
    }

    pub fn next_u64(&mut self) -> u64 {
        self.step();
        let rot = (self.state >> 122) as u32;
        let xored = ((self.state >> 64) as u64) ^ (self.state as u64);
        xored.rotate_right(rot)
    }

    fn next_u32(&mut self) -> u32 {
        if self.has_u32 {
            self.has_u32 = false;
            return self.buffered;
        }
        let v = self.next_u64();
        self.has_u32 = true;
        self.buffered = (v >> 32) as u32;
        v as u32
    }

    /// numpy's random_interval: masked rejection, drawing 32-bit words for small ranges.
    fn random_interval(&mut self, max: u64) -> u64 {
        if max == 0 {
            return 0;
        }
        let mut mask = max;
        mask |= mask >> 1;
        mask |= mask >> 2;
        mask |= mask >> 4;
        mask |= mask >> 8;
        mask |= mask >> 16;
        mask |= mask >> 32;
        if max <= 0xffff_ffff {
            loop {
                let value = (self.next_u32() as u64) & mask;
                if value <= max {
                    return value;
                }
            }
        } else {
            loop {
                let value = self.next_u64() & mask;
                if value <= max {
                    return value;
                }
            }
        }
    }

    pub fn shuffle<T>(&mut self, x: &mut [T]) {
        for i in (1..x.len()).rev() {
            let j = self.random_interval(i as u64) as usize;
            x.swap(i, j);
        }
    }

    pub fn permutation(&mut self, n: usize) -> Vec<usize> {
        let mut v: Vec<usize> = (0..n).collect();
        self.shuffle(&mut v);
        v
    }

    pub fn permutation_of<T: Clone>(&mut self, values: &[T]) -> Vec<T> {
        let mut v = values.to_vec();
        self.shuffle(&mut v);
        v
    }

    /// Generator.standard_normal: numpy's 256-layer ziggurat over raw 64-bit draws.
    pub fn standard_normal(&mut self) -> f64 {
        use crate::ziggurat::{FI_D, KI_D, NOR_INV_R, NOR_R, WI_D};
        loop {
            let r = self.next_u64();
            let idx = (r & 0xff) as usize;
            let r = r >> 8;
            let sign = r & 0x1;
            let rabs = (r >> 1) & 0x000f_ffff_ffff_ffff;
            let mut x = rabs as f64 * WI_D[idx];
            if sign & 0x1 == 1 {
                x = -x;
            }
            if rabs < KI_D[idx] {
                return x;
            }
            if idx == 0 {
                loop {
                    let xx = -NOR_INV_R * libm::log1p(-self.next_f64());
                    let yy = -libm::log1p(-self.next_f64());
                    if yy + yy > xx * xx {
                        return if (rabs >> 8) & 0x1 == 1 {
                            -(NOR_R + xx)
                        } else {
                            NOR_R + xx
                        };
                    }
                }
            } else if (FI_D[idx - 1] - FI_D[idx]) * self.next_f64() + FI_D[idx]
                < libm::exp(-0.5 * x * x)
            {
                return x;
            }
        }
    }

    /// Generator.random / uniform(0, 1): 53-bit doubles from single 64-bit draws.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / 9007199254740992.0
    }
}

/// The legacy MT19937 behind numpy's global RandomState, seeded via init_genrand.
pub struct Mt19937 {
    key: [u32; 624],
    pos: usize,
}

impl Mt19937 {
    pub fn seeded(seed: u32) -> Self {
        let mut key = [0u32; 624];
        key[0] = seed;
        for i in 1..624 {
            key[i] = 1812433253u32
                .wrapping_mul(key[i - 1] ^ (key[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        Mt19937 { key, pos: 624 }
    }

    fn twist(&mut self) {
        const M: usize = 397;
        const MATRIX_A: u32 = 0x9908b0df;
        const UPPER: u32 = 0x80000000;
        const LOWER: u32 = 0x7fffffff;
        for i in 0..624 {
            let y = (self.key[i] & UPPER) | (self.key[(i + 1) % 624] & LOWER);
            let mut next = y >> 1;
            if y & 1 != 0 {
                next ^= MATRIX_A;
            }
            self.key[i] = self.key[(i + M) % 624] ^ next;
        }
        self.pos = 0;
    }

    pub fn next_u32(&mut self) -> u32 {
        if self.pos >= 624 {
            self.twist();
        }
        let mut y = self.key[self.pos];
        self.pos += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^ (y >> 18)
    }

    pub fn next_u64(&mut self) -> u64 {
        let high = self.next_u32() as u64;
        let low = self.next_u32() as u64;
        (high << 32) | low
    }

    /// RandomState.randint(0, high): masked rejection, raw 32-bit draws for small ranges.
    pub fn randint(&mut self, high: u64) -> u64 {
        let rng = high - 1;
        if rng == 0 {
            return 0;
        }
        let mut mask = rng;
        mask |= mask >> 1;
        mask |= mask >> 2;
        mask |= mask >> 4;
        mask |= mask >> 8;
        mask |= mask >> 16;
        mask |= mask >> 32;
        if rng <= 0xffff_ffff {
            loop {
                let v = (self.next_u32() as u64) & mask;
                if v <= rng {
                    return v;
                }
            }
        }
        loop {
            let v = self.next_u64() & mask;
            if v <= rng {
                return v;
            }
        }
    }
}

impl Mt19937 {
    /// randomkit rk_double: 53-bit uniform from two 32-bit words.
    pub fn next_f64(&mut self) -> f64 {
        let a = (self.next_u32() >> 5) as f64;
        let b = (self.next_u32() >> 6) as f64;
        (a * 67108864.0 + b) / 9007199254740992.0
    }

    /// randomkit rk_gauss: polar Box-Muller with the second value cached.
    pub fn standard_normal(&mut self, cache: &mut Option<f64>) -> f64 {
        if let Some(v) = cache.take() {
            return v;
        }
        loop {
            let x1 = 2.0 * self.next_f64() - 1.0;
            let x2 = 2.0 * self.next_f64() - 1.0;
            let r2 = x1 * x1 + x2 * x2;
            if r2 < 1.0 && r2 != 0.0 {
                let f = (-2.0 * r2.ln() / r2).sqrt();
                *cache = Some(f * x1);
                return f * x2;
            }
        }
    }

    fn interval(&mut self, max: u64) -> u64 {
        if max == 0 {
            return 0;
        }
        let mut mask = max;
        mask |= mask >> 1;
        mask |= mask >> 2;
        mask |= mask >> 4;
        mask |= mask >> 8;
        mask |= mask >> 16;
        mask |= mask >> 32;
        if max <= 0xffff_ffff {
            loop {
                let v = (self.next_u32() as u64) & mask;
                if v <= max {
                    return v;
                }
            }
        }
        loop {
            let v = self.next_u64() & mask;
            if v <= max {
                return v;
            }
        }
    }

    pub fn shuffle(&mut self, x: &mut [usize]) {
        for i in (1..x.len()).rev() {
            let j = self.interval(i as u64) as usize;
            x.swap(i, j);
        }
    }

    pub fn permutation(&mut self, n: usize) -> Vec<usize> {
        let mut v: Vec<usize> = (0..n).collect();
        self.shuffle(&mut v);
        v
    }
}
