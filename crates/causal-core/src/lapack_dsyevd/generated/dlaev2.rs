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
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaev2_(
    mut a: *mut doublereal,
    mut b: *mut doublereal,
    mut c__: *mut doublereal,
    mut rt1: *mut doublereal,
    mut rt2: *mut doublereal,
    mut cs1: *mut doublereal,
    mut sn1: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut d__1: doublereal = 0.;
    let mut ab: doublereal = 0.;
    let mut df: doublereal = 0.;
    let mut cs: doublereal = 0.;
    let mut ct: doublereal = 0.;
    let mut tb: doublereal = 0.;
    let mut sm: doublereal = 0.;
    let mut tn: doublereal = 0.;
    let mut rt: doublereal = 0.;
    let mut adf: doublereal = 0.;
    let mut acs: doublereal = 0.;
    let mut sgn1: integer = 0;
    let mut sgn2: integer = 0;
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
        sgn1 = -(1 as ::core::ffi::c_int) as integer;
        *rt2 = acmx / *rt1 * acmn - *b / *rt1 * *b;
    } else if sm > 0.0f64 {
        *rt1 = ((sm as ::core::ffi::c_double + rt as ::core::ffi::c_double) * 0.5f64) as doublereal;
        sgn1 = 1 as integer;
        *rt2 = acmx / *rt1 * acmn - *b / *rt1 * *b;
    } else {
        *rt1 = (rt as ::core::ffi::c_double * 0.5f64) as doublereal;
        *rt2 = (rt as ::core::ffi::c_double * -0.5f64) as doublereal;
        sgn1 = 1 as integer;
    }
    if df >= 0.0f64 {
        cs = df + rt;
        sgn2 = 1 as integer;
    } else {
        cs = df - rt;
        sgn2 = -(1 as ::core::ffi::c_int) as integer;
    }
    acs = (if cs >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        cs as ::core::ffi::c_double
    } else {
        -(cs as ::core::ffi::c_double)
    }) as doublereal;
    if acs > ab {
        ct = -tb / cs;
        *sn1 = (1.0f64 / sqrt(ct * ct + 1.0f64)) as doublereal;
        *cs1 = ct * *sn1;
    } else if ab == 0.0f64 {
        *cs1 = 1.0f64 as doublereal;
        *sn1 = 0.0f64 as doublereal;
    } else {
        tn = -cs / tb;
        *cs1 = (1.0f64 / sqrt(tn * tn + 1.0f64)) as doublereal;
        *sn1 = tn * *cs1;
    }
    if sgn1 == sgn2 {
        tn = *cs1;
        *cs1 = -*sn1;
        *sn1 = tn;
    }
    return 0 as ::core::ffi::c_int;
}
