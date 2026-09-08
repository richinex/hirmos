#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn log(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type doublereal = ::core::ffi::c_double;
pub const log10e: ::core::ffi::c_double = 0.43429448190325182765f64;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_d_lg10(mut x: *mut doublereal) -> ::core::ffi::c_double {
    return log10e * log(*x);
}
