#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type integer = ::core::ffi::c_long;
pub type real = ::core::ffi::c_float;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_i_nint(mut x: *mut real) -> integer {
    return (if *x >= 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        floor(*x as ::core::ffi::c_double + 0.5f64)
    } else {
        -floor(0.5f64 - *x as ::core::ffi::c_double)
    }) as integer;
}
