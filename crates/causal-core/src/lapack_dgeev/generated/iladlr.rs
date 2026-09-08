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
pub unsafe extern "C" fn dgeev_closure_iladlr_(
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
    let mut j: integer = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    if *m == 0 as ::core::ffi::c_long {
        ret_val = *m;
    } else if *a.offset((*m + a_dim1) as isize) != 0.0f64
        || *a.offset((*m + *n * a_dim1) as isize) != 0.0f64
    {
        ret_val = *m;
    } else {
        ret_val = 0 as integer;
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            i__ = *m;
            while i__ >= 1 as ::core::ffi::c_long {
                if *a.offset((i__ + j * a_dim1) as isize) != 0.0f64 {
                    break;
                }
                i__ -= 1;
            }
            ret_val = (if ret_val >= i__ {
                ret_val as ::core::ffi::c_long
            } else {
                i__ as ::core::ffi::c_long
            }) as integer;
            j += 1;
        }
    }
    return ret_val;
}
