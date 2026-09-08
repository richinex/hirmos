#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaisnan_(
    mut din1: *mut doublereal,
    mut din2: *mut doublereal,
) -> logical {
    let mut ret_val: logical = 0;
    ret_val = (*din1 != *din2) as ::core::ffi::c_int as logical;
    return ret_val;
}
