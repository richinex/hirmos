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
pub unsafe extern "C" fn dgeev_closure_dlabad_(
    mut small: *mut doublereal,
    mut large: *mut doublereal,
) -> ::core::ffi::c_int {
    extern "C" {
        #[link_name = "dgeev_closure_d_lg10"]
        fn d_lg10_0(_: *mut doublereal) -> ::core::ffi::c_double;
    }
    if d_lg10_0(large) > 2e3f64 {
        *small = sqrt(*small) as doublereal;
        *large = sqrt(*large) as doublereal;
    }
    return 0 as ::core::ffi::c_int;
}
