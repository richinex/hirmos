//! BOOM 0.9.16 random stream compatibility, verified against the pinned Linux oracle.
//! Normal algorithm: BOOM Bmath/snorm.cpp, Kinderman-Ramage, GPL-2.0-or-later.
#[path = "../../causal-core/src/survival/forest/random.rs"]
mod engine;
mod gamma;
mod truncated;

#[derive(Clone)]
pub struct Random(engine::Random);

impl Random {
    pub fn new(seed: u64) -> Self {
        Self(engine::Random::new(seed))
    }
    pub fn uniform(&mut self) -> f64 {
        self.0.unit()
    }
    pub fn shuffle<T>(&mut self, values: &mut [T]) {
        // BOOM cpputil/shuffle.hpp uses random_int_mt, not std::shuffle.
        for i in (1..values.len()).rev() {
            let j = (self.uniform() * (i + 1) as f64).floor() as usize;
            values.swap(i, j);
        }
    }

    pub fn normal(&mut self) -> f64 {
        const A: f64 = 2.216035867166471;
        let g = |x: f64| 0.398942280401433 * (-x * x / 2.).exp() - 0.180025191068563 * (A - x);
        let u1 = self.uniform();
        if u1 < 0.884070402298758 {
            return A * (1.131131635444180 * u1 + self.uniform() - 1.);
        }
        if u1 >= 0.973310954173898 {
            loop {
                let u2 = self.uniform();
                let u3 = self.uniform();
                let t = A * A - 2. * u3.ln();
                if u2 * u2 < A * A / t {
                    return if u1 < 0.986655477086949 {
                        t.sqrt()
                    } else {
                        -t.sqrt()
                    };
                }
            }
        }
        loop {
            let u2 = self.uniform();
            let u3 = self.uniform();
            let (t, accepted) = if u1 >= 0.958720824790463 {
                let t = A - 0.630834801921960 * u2.min(u3);
                (
                    t,
                    u2.max(u3) <= 0.755591531667601 || 0.034240503750111 * (u2 - u3).abs() <= g(t),
                )
            } else if u1 >= 0.911312780288703 {
                let t = 0.479727404222441 + 1.105473661022070 * u2.min(u3);
                (
                    t,
                    u2.max(u3) <= 0.872834976671790 || 0.049264496373128 * (u2 - u3).abs() <= g(t),
                )
            } else {
                let t = 0.479727404222441 - 0.595507138015940 * u2.min(u3);
                (
                    t,
                    t >= 0.
                        && (u2.max(u3) <= 0.805577924423817
                            || 0.053377549506886 * (u2 - u3).abs() <= g(t)),
                )
            };
            if accepted {
                return if u2 < u3 { t } else { -t };
            }
        }
    }
}

/// Normal and uniform draws used by the shared Gaussian numerical kernels.
pub trait NormalDraw {
    fn standard_normal(&mut self) -> f64;
    fn next_f64(&mut self) -> f64;
}
impl NormalDraw for hirmos_causal_core::nprandom::NpRng {
    fn standard_normal(&mut self) -> f64 {
        self.standard_normal()
    }
    fn next_f64(&mut self) -> f64 {
        self.next_f64()
    }
}
impl NormalDraw for Random {
    fn standard_normal(&mut self) -> f64 {
        self.normal()
    }
    fn next_f64(&mut self) -> f64 {
        self.uniform()
    }
}
impl Random {
    pub fn standard_normal(&mut self) -> f64 {
        self.normal()
    }
    pub fn next_f64(&mut self) -> f64 {
        self.uniform()
    }
}
