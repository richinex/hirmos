#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type doublereal = ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dladiv_(
    mut a: *mut doublereal,
    mut b: *mut doublereal,
    mut c__: *mut doublereal,
    mut d__: *mut doublereal,
    mut p: *mut doublereal,
    mut q: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut e: doublereal = 0.;
    let mut f: doublereal = 0.;
    if (if *d__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *d__
    } else {
        -*d__
    }) < (if *c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *c__
    } else {
        -*c__
    }) {
        e = *d__ / *c__;
        f = *c__ + *d__ * e;
        *p = (*a + *b * e) / f;
        *q = (*b - *a * e) / f;
    } else {
        e = *c__ / *d__;
        f = *d__ + *c__ * e;
        *p = (*b + *a * e) / f;
        *q = (-*a + *b * e) / f;
    }
    return 0 as ::core::ffi::c_int;
}
