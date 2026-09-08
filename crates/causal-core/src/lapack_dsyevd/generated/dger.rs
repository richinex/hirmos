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
pub unsafe extern "C" fn dsyevd_closure_f2c_dger(
    mut m: *mut integer,
    mut n: *mut integer,
    mut alpha: *mut doublereal,
    mut x: *mut doublereal,
    mut incx: *mut integer,
    mut y: *mut doublereal,
    mut incy: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut ix: integer = 0;
    let mut jy: integer = 0;
    let mut kx: integer = 0;
    let mut info: integer = 0;
    let mut temp: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    x = x.offset(-1);
    y = y.offset(-1);
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    info = 0 as integer;
    if *m < 0 as ::core::ffi::c_long {
        info = 1 as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        info = 2 as integer;
    } else if *incx == 0 as ::core::ffi::c_long {
        info = 5 as integer;
    } else if *incy == 0 as ::core::ffi::c_long {
        info = 7 as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        info = 9 as integer;
    }
    if info != 0 as ::core::ffi::c_long {
        xerbla__0(
            b"DGER  \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut info,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *m == 0 as ::core::ffi::c_long || *n == 0 as ::core::ffi::c_long || *alpha == 0.0f64 {
        return 0 as ::core::ffi::c_int;
    }
    if *incy > 0 as ::core::ffi::c_long {
        jy = 1 as integer;
    } else {
        jy = 1 as integer - (*n - 1 as integer) * *incy;
    }
    if *incx == 1 as ::core::ffi::c_long {
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            if *y.offset(jy as isize) != 0.0f64 {
                temp = *alpha * *y.offset(jy as isize);
                i__2 = *m;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    let ref mut fresh0 = *a.offset((i__ + j * a_dim1) as isize);
                    *fresh0 += (*x.offset(i__ as isize) * temp) as ::core::ffi::c_double;
                    i__ += 1;
                }
            }
            jy += *incy as ::core::ffi::c_long;
            j += 1;
        }
    } else {
        if *incx > 0 as ::core::ffi::c_long {
            kx = 1 as integer;
        } else {
            kx = 1 as integer - (*m - 1 as integer) * *incx;
        }
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            if *y.offset(jy as isize) != 0.0f64 {
                temp = *alpha * *y.offset(jy as isize);
                ix = kx;
                i__2 = *m;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    let ref mut fresh1 = *a.offset((i__ + j * a_dim1) as isize);
                    *fresh1 += (*x.offset(ix as isize) * temp) as ::core::ffi::c_double;
                    ix += *incx as ::core::ffi::c_long;
                    i__ += 1;
                }
            }
            jy += *incy as ::core::ffi::c_long;
            j += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
