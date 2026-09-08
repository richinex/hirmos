#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sqrt(_: doublereal) -> ::core::ffi::c_double;
}
pub type doublereal = ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlae2_(
    mut a: *mut doublereal,
    mut b: *mut doublereal,
    mut c__: *mut doublereal,
    mut rt1: *mut doublereal,
    mut rt2: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut d__1: doublereal = 0.;
    let mut ab: doublereal = 0.;
    let mut df: doublereal = 0.;
    let mut tb: doublereal = 0.;
    let mut sm: doublereal = 0.;
    let mut rt: doublereal = 0.;
    let mut adf: doublereal = 0.;
    let mut acmn: doublereal = 0.;
    let mut acmx: doublereal = 0.;
    sm = *a + *c__;
    df = *a - *c__;
    adf = (if df >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        df as ::core::ffi::c_double
    } else {
        -(df as ::core::ffi::c_double)
    }) as doublereal;
    tb = *b + *b;
    ab = (if tb >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        tb as ::core::ffi::c_double
    } else {
        -(tb as ::core::ffi::c_double)
    }) as doublereal;
    if (if *a >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *a
    } else {
        -*a
    }) > (if *c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *c__
    } else {
        -*c__
    }) {
        acmx = *a;
        acmn = *c__;
    } else {
        acmx = *c__;
        acmn = *a;
    }
    if adf > ab {
        d__1 = ab / adf;
        rt = (adf as ::core::ffi::c_double * sqrt(d__1 * d__1 + 1.0f64)) as doublereal;
    } else if adf < ab {
        d__1 = adf / ab;
        rt = (ab as ::core::ffi::c_double * sqrt(d__1 * d__1 + 1.0f64)) as doublereal;
    } else {
        rt = (ab as ::core::ffi::c_double * sqrt(2.0f64)) as doublereal;
    }
    if sm < 0.0f64 {
        *rt1 = ((sm as ::core::ffi::c_double - rt as ::core::ffi::c_double) * 0.5f64) as doublereal;
        *rt2 = acmx / *rt1 * acmn - *b / *rt1 * *b;
    } else if sm > 0.0f64 {
        *rt1 = ((sm as ::core::ffi::c_double + rt as ::core::ffi::c_double) * 0.5f64) as doublereal;
        *rt2 = acmx / *rt1 * acmn - *b / *rt1 * *b;
    } else {
        *rt1 = (rt as ::core::ffi::c_double * 0.5f64) as doublereal;
        *rt2 = (rt as ::core::ffi::c_double * -0.5f64) as doublereal;
    }
    return 0 as ::core::ffi::c_int;
}
