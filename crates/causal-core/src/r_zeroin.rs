//! Base R's `R_zeroin2`, used by `uniroot`.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ZeroinError {
    RootNotBracketed,
    NonFiniteValue,
}

pub(crate) fn r_zeroin2<F>(
    lower: f64,
    upper: f64,
    tolerance: f64,
    maximum_iterations: usize,
    function: F,
) -> Result<f64, ZeroinError>
where
    F: Fn(f64) -> f64,
{
    let mut a = lower;
    let mut b = upper;
    let mut fa = function(a);
    let mut fb = function(b);
    if !fa.is_finite() || !fb.is_finite() {
        return Err(ZeroinError::NonFiniteValue);
    }
    if fa == 0.0 {
        return Ok(a);
    }
    if fb == 0.0 {
        return Ok(b);
    }
    if fa.signum() == fb.signum() {
        return Err(ZeroinError::RootNotBracketed);
    }

    let mut c = a;
    let mut fc = fa;
    for _ in 0..=maximum_iterations {
        let previous_step = b - a;
        if fc.abs() < fb.abs() {
            a = b;
            b = c;
            c = a;
            fa = fb;
            fb = fc;
            fc = fa;
        }
        let actual_tolerance = 2.0 * f64::EPSILON * b.abs() + tolerance / 2.0;
        let mut new_step = (c - b) / 2.0;
        if new_step.abs() <= actual_tolerance || fb == 0.0 {
            return Ok(b);
        }
        if previous_step.abs() >= actual_tolerance && fa.abs() > fb.abs() {
            let cb = c - b;
            let (mut p, mut q) = if a == c {
                let ratio = fb / fa;
                (cb * ratio, 1.0 - ratio)
            } else {
                let first_ratio = fa / fc;
                let second_ratio = fb / fc;
                let third_ratio = fb / fa;
                (
                    third_ratio
                        * (cb * first_ratio * (first_ratio - second_ratio)
                            - (b - a) * (second_ratio - 1.0)),
                    (first_ratio - 1.0) * (second_ratio - 1.0) * (third_ratio - 1.0),
                )
            };
            if p > 0.0 {
                q = -q;
            } else {
                p = -p;
            }
            if p < 0.75 * cb * q - (actual_tolerance * q).abs() / 2.0
                && p < (previous_step * q / 2.0).abs()
            {
                new_step = p / q;
            }
        }
        if new_step.abs() < actual_tolerance {
            new_step = if new_step > 0.0 {
                actual_tolerance
            } else {
                -actual_tolerance
            };
        }
        a = b;
        fa = fb;
        b += new_step;
        fb = function(b);
        if !fb.is_finite() {
            return Err(ZeroinError::NonFiniteValue);
        }
        if (fb > 0.0 && fc > 0.0) || (fb < 0.0 && fc < 0.0) {
            c = a;
            fc = fa;
        }
    }
    Ok(b)
}
