//! BOOM 0.9.16 Brent.cpp bracketing and Netlib FMINBR.
//! Source copyright Steven L. Scott / Google LLC, LGPL-2.1-or-later.
//! Keeps the reference steps, with bounded failure for malformed targets.
use crate::Error;

pub fn minimize(
    f: impl Fn(f64) -> f64,
    start: f64,
    second: f64,
    tolerance: f64,
) -> Result<(f64, f64), Error> {
    if !start.is_finite() || !second.is_finite() || !tolerance.is_finite() {
        return Err(Error::NonFinite);
    }
    if start == second || tolerance <= 0. {
        return Err(Error::InvalidScale);
    }
    let checked = |x: f64| {
        let y = f(x);
        if y.is_finite() {
            Ok(y)
        } else {
            Err(Error::NonFinite)
        }
    };
    let (mut a, mut b) = (start.min(second), start.max(second));
    let fa = checked(a)?;
    let fb = checked(b)?;
    let c = 0.5 * (a + b);
    let fc = checked(c)?;
    let mut left = (fc - fa) / (c - a);
    let mut right = (fb - fc) / (b - c);
    let sign = |x: f64| {
        if x == 0. {
            0
        } else if x > 0. {
            1
        } else {
            -1
        }
    };
    let mut bracketed = false;
    for _ in 0..1024 {
        if sign(left) != sign(right) {
            bracketed = true;
            break;
        }
        let dx = b - a;
        if right < 0. {
            b += dx;
            right = (checked(b)? - fc) / (b - c);
        } else if left > 0. {
            a -= dx;
            left = (fc - checked(a)?) / (c - a);
        } else {
            return Err(Error::SamplingLimit);
        }
        if !a.is_finite() || !b.is_finite() {
            return Err(Error::NonFinite);
        }
    }
    if !bracketed {
        return Err(Error::SamplingLimit);
    }
    let ratio = (3. - 5_f64.sqrt()) / 2.;
    let mut v = a + ratio * (b - a);
    let mut x = v;
    let mut w = v;
    let mut fv = checked(v)?;
    let mut fx = fv;
    let mut fw = fv;
    for _ in 0..10000 {
        let range = b - a;
        let middle = (a + b) / 2.;
        let tol = f64::EPSILON.sqrt() * x.abs() + tolerance / 3.;
        if (x - middle).abs() + range / 2. <= 2. * tol {
            return Ok((x, fx));
        }
        let mut step = ratio * if x < middle { b - x } else { a - x };
        if (x - w).abs() >= tol {
            let t = (x - w) * (fx - fv);
            let mut q = (x - v) * (fx - fw);
            let mut p = (x - v) * q - (x - w) * t;
            q = 2. * (q - t);
            if q > 0. {
                p = -p;
            } else {
                q = -q;
            }
            if p.abs() < (step * q).abs()
                && p > q * (a - x + 2. * tol)
                && p < q * (b - x - 2. * tol)
            {
                step = p / q;
            }
        }
        if step.abs() < tol {
            step = if step > 0. { tol } else { -tol };
        }
        let t = x + step;
        let ft = checked(t)?;
        if ft <= fx {
            if t < x {
                b = x;
            } else {
                a = x;
            }
            v = w;
            w = x;
            x = t;
            fv = fw;
            fw = fx;
            fx = ft;
        } else {
            if t < x {
                a = t;
            } else {
                b = t;
            }
            if ft <= fw || w == x {
                v = w;
                w = t;
                fv = fw;
                fw = ft;
            } else if ft <= fv || v == x || v == w {
                v = t;
                fv = ft;
            }
        }
    }
    Err(Error::SamplingLimit)
}
