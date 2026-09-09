//! R 4.4.1's default Mersenne-Twister, inversion normal generator, and
//! `sample()` index path.

#[derive(Clone, Debug)]
pub(crate) struct RRng {
    state: [u32; 624],
    index: usize,
}

impl RRng {
    pub(crate) fn new(seed: u32) -> Self {
        let mut value = seed;
        for _ in 0..50 {
            value = value.wrapping_mul(69_069).wrapping_add(1);
        }
        // R initializes 625 seed words, then replaces the first with 624.
        value = value.wrapping_mul(69_069).wrapping_add(1);
        let mut state = [0_u32; 624];
        for word in &mut state {
            value = value.wrapping_mul(69_069).wrapping_add(1);
            *word = value;
        }
        Self { state, index: 624 }
    }

    pub(crate) fn uniform(&mut self) -> f64 {
        if self.index >= 624 {
            self.twist();
        }
        let mut value = self.state[self.index];
        self.index += 1;
        value ^= value >> 11;
        value ^= (value << 7) & 0x9d2c_5680;
        value ^= (value << 15) & 0xefc6_0000;
        value ^= value >> 18;
        value as f64 * 2.328_306_436_538_696_3e-10
    }

    /// R's default `N01_kind = INVERSION` path.
    pub(crate) fn normal(&mut self) -> f64 {
        const TWO_TO_27: f64 = 134_217_728.0;
        let first = self.uniform();
        let probability = ((TWO_TO_27 * first) as u32 as f64 + self.uniform()) / TWO_TO_27;
        standard_normal_quantile(probability)
    }

    /// R 4.4.1's `exp_rand` (Ahrens-Dieter), used by `rexp`.
    pub(crate) fn exponential(&mut self) -> f64 {
        const Q: [f64; 16] = [
            0.693_147_180_559_945_3,
            0.933_373_687_519_045_9,
            0.988_877_796_183_867_5,
            0.998_495_925_291_496,
            0.999_829_281_106_138_9,
            0.999_983_316_410_072_7,
            0.999_998_569_143_876_7,
            0.999_999_890_692_555_8,
            0.999_999_992_473_415_9,
            0.999_999_999_528_327_5,
            0.999_999_999_972_881_4,
            0.999_999_999_998_559_8,
            0.999_999_999_999_928_9,
            0.999_999_999_999_996_8,
            0.999_999_999_999_999_9,
            1.0,
        ];
        let mut a = 0.0;
        let mut u = self.uniform();
        loop {
            u += u;
            if u > 1.0 {
                break;
            }
            a += Q[0];
        }
        u -= 1.0;
        if u <= Q[0] {
            return a + u;
        }
        let mut index = 0;
        let mut minimum = self.uniform();
        loop {
            let candidate = self.uniform();
            if candidate < minimum {
                minimum = candidate;
            }
            index += 1;
            if u <= Q[index] {
                break;
            }
        }
        a + minimum * Q[0]
    }

    /// R 4.4.1's `rgamma`, including the Ahrens-Dieter GS and GD rejection
    /// algorithms. `scale` is R's scale parameter, not its rate parameter.
    pub(crate) fn gamma(&mut self, shape: f64, scale: f64) -> f64 {
        const SQRT_32: f64 = 5.656_854;
        const EXP_MINUS_ONE: f64 = 0.367_879_441_171_442_33;
        const Q1: f64 = 0.041_666_69;
        const Q2: f64 = 0.020_831_48;
        const Q3: f64 = 0.008_011_91;
        const Q4: f64 = 0.001_441_21;
        const Q5: f64 = -7.388e-5;
        const Q6: f64 = 2.451_1e-4;
        const Q7: f64 = 2.424e-4;
        const A1: f64 = 0.333_333_3;
        const A2: f64 = -0.250_003;
        const A3: f64 = 0.200_006_2;
        const A4: f64 = -0.166_292_1;
        const A5: f64 = 0.142_365_7;
        const A6: f64 = -0.136_717_7;
        const A7: f64 = 0.123_379_5;

        debug_assert!(shape.is_finite() && shape > 0.0);
        debug_assert!(scale.is_finite() && scale > 0.0);

        if shape < 1.0 {
            let e = 1.0 + EXP_MINUS_ONE * shape;
            loop {
                let p = e * self.uniform();
                if p >= 1.0 {
                    let x = -((e - p) / shape).ln();
                    if self.exponential() >= (1.0 - shape) * x.ln() {
                        return scale * x;
                    }
                } else {
                    let x = (p.ln() / shape).exp();
                    if self.exponential() >= x {
                        return scale * x;
                    }
                }
            }
        }

        let s2 = shape - 0.5;
        let s = s2.sqrt();
        let d = SQRT_32 - 12.0 * s;

        let mut t = self.normal();
        let mut x = s + 0.5 * t;
        let ret_val = x * x;
        if t >= 0.0 {
            return scale * ret_val;
        }

        let mut u = self.uniform();
        if d * u <= t * t * t {
            return scale * ret_val;
        }

        let reciprocal = 1.0 / shape;
        let q0 = ((((((Q7 * reciprocal + Q6) * reciprocal + Q5) * reciprocal + Q4) * reciprocal
            + Q3)
            * reciprocal
            + Q2)
            * reciprocal
            + Q1)
            * reciprocal;
        let (b, si, c) = if shape <= 3.686 {
            (0.463 + s + 0.178 * s2, 1.235, 0.195 / s - 0.079 + 0.16 * s)
        } else if shape <= 13.022 {
            (1.654 + 0.0076 * s2, 1.68 / s + 0.275, 0.062 / s + 0.024)
        } else {
            (1.77, 0.75, 0.1515 / s)
        };

        if x > 0.0 {
            let v = t / (s + s);
            let q = if v.abs() <= 0.25 {
                q0 + 0.5
                    * t
                    * t
                    * ((((((A7 * v + A6) * v + A5) * v + A4) * v + A3) * v + A2) * v + A1)
                    * v
            } else {
                q0 - s * t + 0.25 * t * t + (s2 + s2) * (1.0 + v).ln()
            };
            if (1.0 - u).ln() <= q {
                return scale * ret_val;
            }
        }

        loop {
            let e = self.exponential();
            u = self.uniform();
            u = u + u - 1.0;
            t = if u < 0.0 { b - si * e } else { b + si * e };
            if t >= -0.718_744_837_717_19 {
                let v = t / (s + s);
                let q = if v.abs() <= 0.25 {
                    q0 + 0.5
                        * t
                        * t
                        * ((((((A7 * v + A6) * v + A5) * v + A4) * v + A3) * v + A2) * v + A1)
                        * v
                } else {
                    q0 - s * t + 0.25 * t * t + (s2 + s2) * (1.0 + v).ln()
                };
                if q > 0.0 && c * u.abs() <= q.exp_m1() * (e - 0.5 * t * t).exp() {
                    break;
                }
            }
        }
        x = s + 0.5 * t;
        scale * x * x
    }

    /// R 4.4.1's `rf`: a ratio of two independently sampled chi-squares.
    pub(crate) fn f_ratio(&mut self, numerator_df: f64, denominator_df: f64) -> f64 {
        debug_assert!(numerator_df.is_finite() && numerator_df > 0.0);
        debug_assert!(denominator_df.is_finite() && denominator_df > 0.0);
        let numerator = self.gamma(0.5 * numerator_df, 2.0) / numerator_df;
        let denominator = self.gamma(0.5 * denominator_df, 2.0) / denominator_df;
        numerator / denominator
    }

    fn twist(&mut self) {
        const UPPER: u32 = 0x8000_0000;
        const LOWER: u32 = 0x7fff_ffff;
        for index in 0..624 {
            let next = (index + 1) % 624;
            let offset = (index + 397) % 624;
            let combined = (self.state[index] & UPPER) | (self.state[next] & LOWER);
            self.state[index] = self.state[offset]
                ^ (combined >> 1)
                ^ if combined & 1 == 0 { 0 } else { 0x9908_b0df };
        }
        self.index = 0;
    }

    fn bits(&mut self, count: u32) -> u64 {
        let mut value = 0_u64;
        for _ in (0..=count).step_by(16) {
            let word = (self.uniform() * 65_536.0).floor() as u64;
            value = 65_536 * value + word;
        }
        value & ((1_u64 << count) - 1)
    }

    fn sample_index(&mut self, upper: usize) -> usize {
        if upper == 0 {
            return 0;
        }
        let bits = (upper as f64).log2().ceil() as u32;
        loop {
            let value = self.bits(bits) as usize;
            if value < upper {
                return value;
            }
        }
    }

    pub(crate) fn permutation(&mut self, length: usize) -> Vec<usize> {
        let mut pool = (0..length).collect::<Vec<_>>();
        let mut remaining = length;
        let mut result = Vec::with_capacity(length);
        for _ in 0..length {
            let selected = self.sample_index(remaining);
            result.push(pool[selected]);
            remaining -= 1;
            pool[selected] = pool[remaining];
        }
        result
    }
}

// Wichura's AS 241 implementation used by R 4.4.1 qnorm. This is the
// lower-tail, non-logarithmic standard-normal path called by norm_rand().
fn standard_normal_quantile(probability: f64) -> f64 {
    if probability <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if probability >= 1.0 {
        return f64::INFINITY;
    }

    let centered = probability - 0.5;
    if centered.abs() <= 0.425 {
        let r = 0.180625 - centered * centered;
        return centered
            * (((((((r * 2509.0809287301226727 + 33430.575583588128105) * r
                + 67265.770927008700853)
                * r
                + 45921.953931549871457)
                * r
                + 13731.693765509461125)
                * r
                + 1971.5909503065514427)
                * r
                + 133.14166789178437745)
                * r
                + 3.387132872796366608)
            / (((((((r * 5226.495278852854561 + 28729.085735721942674) * r
                + 39307.89580009271061)
                * r
                + 21213.794301586595867)
                * r
                + 5394.1960214247511077)
                * r
                + 687.1870074920579083)
                * r
                + 42.313330701600911252)
                * r
                + 1.0);
    }

    let tail_probability = if centered > 0.0 {
        1.0 - probability
    } else {
        probability
    };
    let mut r = (-tail_probability.ln()).sqrt();
    let mut value = if r <= 5.0 {
        r -= 1.6;
        (((((((r * 7.7454501427834140764e-4 + 0.0227238449892691845833) * r
            + 0.24178072517745061177)
            * r
            + 1.27045825245236838258)
            * r
            + 3.64784832476320460504)
            * r
            + 5.7694972214606914055)
            * r
            + 4.6303378461565452959)
            * r
            + 1.42343711074968357734)
            / (((((((r * 1.05075007164441684324e-9 + 5.475938084995344946e-4) * r
                + 0.0151986665636164571966)
                * r
                + 0.14810397642748007459)
                * r
                + 0.68976733498510000455)
                * r
                + 1.6763848301838038494)
                * r
                + 2.05319162663775882187)
                * r
                + 1.0)
    } else {
        r -= 5.0;
        (((((((r * 2.01033439929228813265e-7 + 2.71155556874348757815e-5) * r
            + 0.0012426609473880784386)
            * r
            + 0.026532189526576123093)
            * r
            + 0.29656057182850489123)
            * r
            + 1.7848265399172913358)
            * r
            + 5.4637849111641143699)
            * r
            + 6.6579046435011035772)
            / (((((((r * 2.04426310338993978564e-15 + 1.4215117583164458887e-7) * r
                + 1.8463183175100546818e-5)
                * r
                + 7.868691311456132591e-4)
                * r
                + 0.0148753612908506148525)
                * r
                + 0.13692988092273580531)
                * r
                + 0.599832206555887969)
                * r
                + 1.0)
    };
    if centered < 0.0 {
        value = -value;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::RRng;

    #[test]
    fn uniforms_normals_and_sample_match_r_4_4_1() {
        let mut rng = RRng::new(12_345);
        let expected = [
            0.72090389626100659,
            0.87577319308184087,
            0.76098232832737267,
            0.88612456619739532,
            0.45648096012882888,
        ];
        for target in expected {
            assert_eq!(rng.uniform(), target);
        }

        let mut rng = RRng::new(12_345);
        let expected = [
            0.58552881784385558,
            0.70946601750952443,
            -0.10930331468105391,
            -0.45349717346276308,
            0.60588745584039339,
        ];
        for target in expected {
            assert!((rng.normal() - target).abs() <= 2.0e-15);
        }

        let mut rng = RRng::new(12_345);
        let source = [0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
        let actual = rng
            .permutation(source.len())
            .into_iter()
            .map(|index| source[index])
            .collect::<Vec<_>>();
        assert_eq!(actual, [0, 1, 0, 0, 1, 1, 1, 1, 0, 0]);

        let mut rng = RRng::new(24_680);
        let expected = [
            1.2698273929703996,
            0.12442440517526505,
            0.65868094097822905,
            0.07891150654433038,
            1.5068220402205232,
            0.00614909827709198,
            3.4314055217612274,
            0.11010073561938158,
            1.106461338656855,
            0.3914082059636712,
        ];
        for target in expected {
            assert_eq!(rng.exponential(), target);
        }
    }
}
