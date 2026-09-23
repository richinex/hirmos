//! BOOM 0.9.16 Bmath/rgamma.cpp and sexp.cpp, GPL-2.0-or-later.
//! Copyright Ross Ihaka, R Development Core Team and Steven L. Scott.
use super::Random;
use crate::Error;

impl Random {
    pub fn exponential(&mut self) -> f64 {
        let q = [
            0.6931471805599453,
            0.9333736875190459,
            0.9888777961838675,
            0.9984959252914960,
            0.9998292811061389,
            0.9999833164100727,
            0.9999985691438767,
            0.9999998906925558,
            0.9999999924734159,
            0.9999999995283275,
            0.9999999999728814,
            0.9999999999985598,
            0.9999999999999289,
            0.9999999999999968,
            0.9999999999999999,
            1.,
        ];
        let mut a = 0.;
        let mut u = self.uniform();
        while u <= 0. || u >= 1. {
            u = self.uniform();
        }
        loop {
            u += u;
            if u > 1. {
                break;
            }
            a += q[0];
        }
        u -= 1.;
        if u <= q[0] {
            return a + u;
        }
        let mut minimum = self.uniform();
        let mut i = 0;
        loop {
            minimum = minimum.min(self.uniform());
            i += 1;
            if u <= q[i] {
                return a + minimum * q[0];
            }
        }
    }

    /// Shape and scale convention, matching Rmath (not BOOM's rate wrapper).
    pub fn gamma(&mut self, a: f64, scale: f64) -> Result<f64, Error> {
        if !a.is_finite() || !scale.is_finite() || a <= 0. || scale <= 0. {
            return Err(Error::InvalidScale);
        }
        if a < 0.3 {
            let w = a / (1_f64.exp() * (1. - a));
            let r = 1. / (1. + w);
            let lambda = 1. / a - 1.;
            for _ in 0..1000 {
                let u = self.uniform();
                let z = if u <= r {
                    -(u / r).ln()
                } else {
                    self.uniform().ln() / lambda
                };
                let log_eta = if z >= 0. {
                    -z
                } else {
                    w.ln() + lambda.ln() + lambda * z
                };
                if -z - (-z / a).exp() >= self.uniform().ln() + log_eta {
                    return Ok((-z / a + scale.ln()).exp());
                }
            }
            return Err(Error::NonFinite);
        }
        if a < 1. {
            let e = 1. + 0.36787944117144232159 * a;
            loop {
                let p = e * self.uniform();
                let x;
                if p >= 1. {
                    x = -((e - p) / a).ln();
                    if self.exponential() < (1. - a) * x.ln() {
                        continue;
                    }
                } else {
                    x = (p.ln() / a).exp();
                    if self.exponential() < x {
                        continue;
                    }
                }
                if x > 0. {
                    return Ok(scale * x);
                }
            }
        }
        let s2 = a - 0.5;
        let s = s2.sqrt();
        let d = 5.656854 - s * 12.;
        let t = self.normal();
        let x = s + 0.5 * t;
        if t >= 0. {
            return Ok(scale * (x * x));
        }
        let u = self.uniform();
        if d * u <= t * t * t {
            return Ok(scale * (x * x));
        }
        let r = 1. / a;
        let q0 = ((((((2.424e-4 * r + 2.4511e-4) * r - 7.388e-5) * r + 0.00144121) * r
            + 0.00801191)
            * r
            + 0.02083148)
            * r
            + 0.04166669)
            * r;
        let (b, si, c) = if a <= 3.686 {
            (0.463 + s + 0.178 * s2, 1.235, 0.195 / s - 0.079 + 0.16 * s)
        } else if a <= 13.022 {
            (1.654 + 0.0076 * s2, 1.68 / s + 0.275, 0.062 / s + 0.024)
        } else {
            (1.77, 0.75, 0.1515 / s)
        };
        let quotient = |t: f64, stable: bool| {
            let v = t / (s + s);
            if v.abs() <= 0.25 {
                q0 + 0.5
                    * t
                    * t
                    * ((((((0.1233795 * v - 0.1367177) * v + 0.1423657) * v - 0.1662921) * v
                        + 0.2000062)
                        * v
                        - 0.250003)
                        * v
                        + 0.3333333)
                    * v
            } else {
                q0 - s * t
                    + 0.25 * t * t
                    + (s2 + s2) * if stable { v.ln_1p() } else { (1. + v).ln() }
            }
        };
        if x > 0. && (1. - u).ln() <= quotient(t, true) {
            return Ok(scale * (x * x));
        }
        loop {
            let e = self.exponential();
            let u = self.uniform();
            let u = u + u - 1.;
            let t = if u < 0. { b - si * e } else { b + si * e };
            if t >= -0.71874483771719 {
                let q = quotient(t, false);
                if q > 0. && c * u.abs() <= q.exp_m1() * (e - 0.5 * t * t).exp() {
                    let x = s + 0.5 * t;
                    return Ok(scale * x * x);
                }
            }
        }
    }
}
