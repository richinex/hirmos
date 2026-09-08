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
pub unsafe extern "C" fn dsyevd_closure_iladlc_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
) -> integer {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut ret_val: integer = 0;
    let mut i__1: integer = 0;
    let mut i__: integer = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    if *n == 0 as ::core::ffi::c_long {
        ret_val = *n;
    } else if *a.offset((*n * a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
        != 0.0f64
        || *a.offset((*m + *n * a_dim1) as isize) != 0.0f64
    {
        ret_val = *n;
    } else {
        ret_val = *n;
        while ret_val >= 1 as ::core::ffi::c_long {
            i__1 = *m;
            i__ = 1 as integer;
            while i__ <= i__1 {
                if *a.offset((i__ + ret_val * a_dim1) as isize) != 0.0f64 {
                    return ret_val;
                }
                i__ += 1;
            }
            ret_val -= 1;
        }
    }
    return ret_val;
}
