//! Safe drivers for the mechanically translated original BOOM kernels.
//! Callbacks borrow per-call state; no global objective or mutable work arrays.
use crate::Error;
use core::ffi::c_void;
#[path = "generated/integral.rs"]
mod integral;
#[path = "generated/powell.rs"]
mod powell;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntegrationStatus {
    Converged,
    SubdivisionLimit,
    Roundoff,
    BadIntegrand,
    ExtrapolationFailure,
    Divergent,
    InvalidInput,
}
#[derive(Clone, Copy, Debug)]
pub struct Integral {
    pub value: f64,
    pub absolute_error: f64,
    pub evaluations: usize,
    pub partitions: usize,
    pub status: IntegrationStatus,
}
struct Scalar<F> {
    f: F,
    failed: bool,
}
unsafe extern "C" fn scalar<F: Fn(f64) -> f64>(x: *mut f64, n: i32, context: *mut c_void) {
    let state = &mut *context.cast::<Scalar<F>>();
    for v in core::slice::from_raw_parts_mut(x, n as usize) {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (state.f)(*v)));
        *v = match result {
            Ok(v) if v.is_finite() => v,
            _ => {
                state.failed = true;
                0.
            }
        };
    }
}
/// Retains QUADPACK's status alongside its best estimate, as BOOM does when
/// throw_on_error(false) is selected. Nonfinite callbacks are errors, not data.
pub fn integrate<F: Fn(f64) -> f64>(f: F, lo: f64, hi: f64) -> Result<Integral, Error> {
    integrate_with_tolerance(f, lo, hi, f64::EPSILON.powf(0.25))
}
/// Explicit tolerance for independent checks. Production mixture fitting uses
/// `integrate`, which preserves the original BOOM defaults.
pub fn integrate_with_tolerance<F: Fn(f64) -> f64>(
    f: F,
    mut lo: f64,
    mut hi: f64,
    tolerance: f64,
) -> Result<Integral, Error> {
    if !lo.is_finite() || !hi.is_finite() {
        return Err(Error::NonFinite);
    }
    if !tolerance.is_finite() || tolerance <= 0. {
        return Err(Error::InvalidScale);
    }
    let mut context = Scalar { f, failed: false };
    let mut epsabs = tolerance;
    let mut epsrel = tolerance;
    let mut limit = 1000;
    let mut lenw = 4 * limit;
    let (mut value, mut error) = (0., 0.);
    let (mut evaluations, mut status, mut partitions) = (0, 0, 0);
    // One prefix slot keeps source's one-based pointer adjustments in-bounds.
    let mut work = vec![0.; lenw as usize + 1];
    let mut iwork = vec![0; limit as usize + 1];
    unsafe {
        integral::Rdqags(
            Some(scalar::<F>),
            (&mut context as *mut Scalar<F>).cast(),
            &mut lo,
            &mut hi,
            &mut epsabs,
            &mut epsrel,
            &mut value,
            &mut error,
            &mut evaluations,
            &mut status,
            &mut limit,
            &mut lenw,
            &mut partitions,
            iwork.as_mut_ptr().add(1),
            work.as_mut_ptr().add(1),
        );
    }
    if context.failed || !value.is_finite() || !error.is_finite() {
        return Err(Error::NonFinite);
    }
    let status = match status {
        0 => IntegrationStatus::Converged,
        1 => IntegrationStatus::SubdivisionLimit,
        2 => IntegrationStatus::Roundoff,
        3 => IntegrationStatus::BadIntegrand,
        4 => IntegrationStatus::ExtrapolationFailure,
        5 => IntegrationStatus::Divergent,
        6 => IntegrationStatus::InvalidInput,
        _ => return Err(Error::NonFinite),
    };
    Ok(Integral {
        value,
        absolute_error: error,
        evaluations: evaluations as usize,
        partitions: partitions as usize,
        status,
    })
}

#[derive(Debug)]
pub struct Minimum {
    pub point: Vec<f64>,
    pub value: f64,
    pub evaluations: usize,
}
struct Vector<F> {
    f: F,
    failed: bool,
    evaluations: usize,
}
unsafe extern "C" fn vector<F: Fn(&[f64]) -> f64>(
    n: i64,
    x: *const f64,
    context: *mut c_void,
) -> f64 {
    let state = &mut *context.cast::<Vector<F>>();
    state.evaluations += 1;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        (state.f)(core::slice::from_raw_parts(x, n as usize))
    }));
    match result {
        Ok(v) if v.is_finite() => v,
        _ => {
            state.failed = true;
            f64::MAX / 16.
        }
    }
}
pub fn minimize<F: Fn(&[f64]) -> f64>(
    f: F,
    initial: &[f64],
    mut step: f64,
    mut precision: f64,
    max_evals: usize,
) -> Result<Minimum, Error> {
    let size = initial.len();
    if size < 2 || max_evals < 1 {
        return Err(Error::Shape);
    }
    if initial.iter().any(|v| !v.is_finite()) || !step.is_finite() || !precision.is_finite() {
        return Err(Error::NonFinite);
    }
    if precision <= 0. || step < precision {
        return Err(Error::InvalidScale);
    }
    let npt = size
        .checked_mul(2)
        .and_then(|v| v.checked_add(1))
        .ok_or(Error::Shape)?;
    let ndim = npt.checked_add(size).ok_or(Error::Shape)?;
    let work_size = (npt + 13)
        .checked_mul(ndim)
        .and_then(|w| {
            size.checked_mul(size + 3)
                .and_then(|v| v.checked_mul(3))
                .and_then(|v| w.checked_add(v / 2 + 1))
        })
        .ok_or(Error::Shape)?;
    if size > i32::MAX as usize
        || max_evals > i32::MAX as usize
        || work_size > isize::MAX as usize / 8 - ndim - 2
    {
        return Err(Error::Shape);
    }
    let pad = ndim + 2;
    let mut work = Vec::new();
    work.try_reserve_exact(work_size + pad)
        .map_err(|_| Error::Shape)?;
    work.resize(work_size + pad, 0.);
    let mut x = vec![0.; size + 1];
    x[1..].copy_from_slice(initial);
    let mut context = Vector {
        f,
        failed: false,
        evaluations: 0,
    };
    let mut target = powell::Target {
        eval: Some(vector::<F>),
        context: (&mut context as *mut Vector<F>).cast(),
    };
    let (mut n, mut npt, mut maxfun, mut print) =
        (size as i64, npt as i64, max_evals as i64, 0_i64);
    // Additional prefix covers the source's npt/ndim-leading matrix pointers.
    unsafe {
        powell::newuoa_(
            &mut target,
            &mut n,
            &mut npt,
            x.as_mut_ptr().add(1),
            &mut step,
            &mut precision,
            &mut print,
            &mut maxfun,
            work.as_mut_ptr().add(pad),
        );
    }
    if context.failed || x.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    let value = (context.f)(&x[1..]);
    if !value.is_finite() {
        return Err(Error::NonFinite);
    }
    Ok(Minimum {
        point: x[1..].to_vec(),
        value,
        evaluations: context.evaluations,
    })
}
