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
pub unsafe extern "C" fn dgeev_closure_d_sign(
    mut a: *mut doublereal,
    mut b: *mut doublereal,
) -> ::core::ffi::c_double {
    let mut x: ::core::ffi::c_double = 0.;
    x = if *a >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *a
    } else {
        -*a
    };
    return if *b >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        x
    } else {
        -x
    };
}
