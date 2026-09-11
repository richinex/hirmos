//! SciPy SLSQP translation and numerical dependencies (safe driver pending).
//! Generated BLAS routines reuse the existing DGELSD character/error helpers.

#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
mod generated {
    pub mod __nnls;
    pub mod __slsqp;
    pub mod dgelsy;
    pub mod dgeqp3;
    pub mod dgerq2;
    pub mod dlaic1;
    pub mod dlaqp2;
    pub mod dlaqps;
    pub mod dlarf1f;
    pub mod dlarf1l;
    pub mod dlarfgp;
    pub mod dlartgp;
    pub mod dlarz;
    pub mod dlarzb;
    pub mod dlarzt;
    pub mod dlatrz;
    pub mod dormr2;
    pub mod dormr3;
    pub mod dormrz;
    pub mod dtpmv;
    pub mod dtpsv;
    pub mod dtrsm;
    pub mod dtrsv;
    pub mod dtzrzf;
    pub mod s_cat;
}

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub(crate) mod driver;

// C99 nextafter, which SciPy's __nnls.c takes from libm: the double adjacent to x toward y.
// Defined here so the translation carries every symbol it calls on every target.
#[no_mangle]
unsafe extern "C" fn slsqp_closure_nextafter(x: f64, y: f64) -> f64 {
    if x.is_nan() || y.is_nan() {
        return f64::NAN;
    }
    if x == y {
        return y;
    }
    if x == 0.0 {
        let smallest = f64::from_bits(1);
        return if y > 0.0 { smallest } else { -smallest };
    }
    let bits = x.to_bits();
    let toward_larger_magnitude = (y > x) == (x > 0.0);
    f64::from_bits(if toward_larger_magnitude { bits + 1 } else { bits - 1 })
}

// f2c's NINT(double) helper: ties round away from zero.
// Source: reference/CLAPACK-3.2.1/F2CLIBS/libf2c/i_dnnt.c.
#[no_mangle]
unsafe extern "C" fn slsqp_closure_i_dnnt(x: *const f64) -> core::ffi::c_long {
    let value = *x;
    if value >= 0.0 {
        (value + 0.5).floor() as core::ffi::c_long
    } else {
        -(0.5 - value).floor() as core::ffi::c_long
    }
}
