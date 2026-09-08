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
pub type logical = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlaset_(
    mut uplo: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut alpha: *mut doublereal,
    mut beta: *mut doublereal,
    mut a: *mut doublereal,
    mut lda: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    if lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        i__1 = *n;
        j = 2 as integer;
        while j <= i__1 {
            i__3 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            i__2 = (if i__3 <= *m {
                i__3 as ::core::ffi::c_long
            } else {
                *m
            }) as integer;
            i__ = 1 as integer;
            while i__ <= i__2 {
                *a.offset((i__ + j * a_dim1) as isize) = *alpha;
                i__ += 1;
            }
            j += 1;
        }
    } else if lsame__0(
        uplo,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        i__1 = (if *m <= *n { *m } else { *n }) as integer;
        j = 1 as integer;
        while j <= i__1 {
            i__2 = *m;
            i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            while i__ <= i__2 {
                *a.offset((i__ + j * a_dim1) as isize) = *alpha;
                i__ += 1;
            }
            j += 1;
        }
    } else {
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            i__2 = *m;
            i__ = 1 as integer;
            while i__ <= i__2 {
                *a.offset((i__ + j * a_dim1) as isize) = *alpha;
                i__ += 1;
            }
            j += 1;
        }
    }
    i__1 = (if *m <= *n { *m } else { *n }) as integer;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *a.offset((i__ + i__ * a_dim1) as isize) = *beta;
        i__ += 1;
    }
    return 0 as ::core::ffi::c_int;
}
