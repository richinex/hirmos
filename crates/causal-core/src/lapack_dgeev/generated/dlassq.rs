#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlassq_(
    mut n: *mut integer,
    mut x: *mut doublereal,
    mut incx: *mut integer,
    mut scale: *mut doublereal,
    mut sumsq: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut ix: integer = 0;
    let mut absxi: doublereal = 0.;
    x = x.offset(-1);
    if *n > 0 as ::core::ffi::c_long {
        i__1 = ((*n - 1 as ::core::ffi::c_long) * *incx + 1 as ::core::ffi::c_long) as integer;
        i__2 = *incx;
        ix = 1 as integer;
        while if i__2 < 0 as ::core::ffi::c_long {
            (ix >= i__1) as ::core::ffi::c_int
        } else {
            (ix <= i__1) as ::core::ffi::c_int
        } != 0
        {
            if *x.offset(ix as isize) != 0.0f64 {
                d__1 = *x.offset(ix as isize);
                absxi = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
                if *scale < absxi {
                    d__1 = *scale / absxi;
                    *sumsq = (*sumsq
                        * (d__1 as ::core::ffi::c_double * d__1 as ::core::ffi::c_double)
                        + 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                        as doublereal;
                    *scale = absxi;
                } else {
                    d__1 = absxi / *scale;
                    *sumsq += (d__1 * d__1) as ::core::ffi::c_double;
                }
            }
            ix += i__2 as ::core::ffi::c_long;
        }
    }
    return 0 as ::core::ffi::c_int;
}
