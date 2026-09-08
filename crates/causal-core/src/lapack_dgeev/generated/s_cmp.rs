#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type integer = ::core::ffi::c_long;
pub type ftnlen = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_s_cmp(
    mut a0: *mut ::core::ffi::c_char,
    mut b0: *mut ::core::ffi::c_char,
    mut la: ftnlen,
    mut lb: ftnlen,
) -> integer {
    let mut a: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut aend: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut b: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bend: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    a = a0 as *mut ::core::ffi::c_uchar;
    b = b0 as *mut ::core::ffi::c_uchar;
    aend = a.offset(la as isize);
    bend = b.offset(lb as isize);
    if la <= lb {
        while a < aend {
            if *a as ::core::ffi::c_int != *b as ::core::ffi::c_int {
                return (*a as ::core::ffi::c_int - *b as ::core::ffi::c_int) as integer;
            } else {
                a = a.offset(1);
                b = b.offset(1);
            }
        }
        while b < bend {
            if *b as ::core::ffi::c_int != ' ' as i32 {
                return (' ' as i32 - *b as ::core::ffi::c_int) as integer;
            } else {
                b = b.offset(1);
            }
        }
    } else {
        while b < bend {
            if *a as ::core::ffi::c_int == *b as ::core::ffi::c_int {
                a = a.offset(1);
                b = b.offset(1);
            } else {
                return (*a as ::core::ffi::c_int - *b as ::core::ffi::c_int) as integer;
            }
        }
        while a < aend {
            if *a as ::core::ffi::c_int != ' ' as i32 {
                return (*a as ::core::ffi::c_int - ' ' as i32) as integer;
            } else {
                a = a.offset(1);
            }
        }
    }
    return 0 as integer;
}
