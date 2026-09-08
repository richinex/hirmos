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
pub unsafe extern "C" fn dsyevd_closure_f2c_dsyr2(
    mut uplo: *mut ::core::ffi::c_char,
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
    let mut iy: integer = 0;
    let mut jx: integer = 0;
    let mut jy: integer = 0;
    let mut kx: integer = 0;
    let mut ky: integer = 0;
    let mut info: integer = 0;
    let mut temp1: doublereal = 0.;
    let mut temp2: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
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
    if lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 1 as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        info = 2 as integer;
    } else if *incx == 0 as ::core::ffi::c_long {
        info = 5 as integer;
    } else if *incy == 0 as ::core::ffi::c_long {
        info = 7 as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        info = 9 as integer;
    }
    if info != 0 as ::core::ffi::c_long {
        xerbla__0(
            b"DSYR2 \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut info,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long || *alpha == 0.0f64 {
        return 0 as ::core::ffi::c_int;
    }
    if *incx != 1 as ::core::ffi::c_long || *incy != 1 as ::core::ffi::c_long {
        if *incx > 0 as ::core::ffi::c_long {
            kx = 1 as integer;
        } else {
            kx = 1 as integer - (*n - 1 as integer) * *incx;
        }
        if *incy > 0 as ::core::ffi::c_long {
            ky = 1 as integer;
        } else {
            ky = 1 as integer - (*n - 1 as integer) * *incy;
        }
        jx = kx;
        jy = ky;
    }
    if lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if *incx == 1 as ::core::ffi::c_long && *incy == 1 as ::core::ffi::c_long {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                if *x.offset(j as isize) != 0.0f64 || *y.offset(j as isize) != 0.0f64 {
                    temp1 = *alpha * *y.offset(j as isize);
                    temp2 = *alpha * *x.offset(j as isize);
                    i__2 = j;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *a.offset((i__ + j * a_dim1) as isize) = *a
                            .offset((i__ + j * a_dim1) as isize)
                            + *x.offset(i__ as isize) * temp1
                            + *y.offset(i__ as isize) * temp2;
                        i__ += 1;
                    }
                }
                j += 1;
            }
        } else {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                if *x.offset(jx as isize) != 0.0f64 || *y.offset(jy as isize) != 0.0f64 {
                    temp1 = *alpha * *y.offset(jy as isize);
                    temp2 = *alpha * *x.offset(jx as isize);
                    ix = kx;
                    iy = ky;
                    i__2 = j;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *a.offset((i__ + j * a_dim1) as isize) = *a
                            .offset((i__ + j * a_dim1) as isize)
                            + *x.offset(ix as isize) * temp1
                            + *y.offset(iy as isize) * temp2;
                        ix += *incx as ::core::ffi::c_long;
                        iy += *incy as ::core::ffi::c_long;
                        i__ += 1;
                    }
                }
                jx += *incx as ::core::ffi::c_long;
                jy += *incy as ::core::ffi::c_long;
                j += 1;
            }
        }
    } else if *incx == 1 as ::core::ffi::c_long && *incy == 1 as ::core::ffi::c_long {
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            if *x.offset(j as isize) != 0.0f64 || *y.offset(j as isize) != 0.0f64 {
                temp1 = *alpha * *y.offset(j as isize);
                temp2 = *alpha * *x.offset(j as isize);
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    *a.offset((i__ + j * a_dim1) as isize) = *a.offset((i__ + j * a_dim1) as isize)
                        + *x.offset(i__ as isize) * temp1
                        + *y.offset(i__ as isize) * temp2;
                    i__ += 1;
                }
            }
            j += 1;
        }
    } else {
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            if *x.offset(jx as isize) != 0.0f64 || *y.offset(jy as isize) != 0.0f64 {
                temp1 = *alpha * *y.offset(jy as isize);
                temp2 = *alpha * *x.offset(jx as isize);
                ix = jx;
                iy = jy;
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    *a.offset((i__ + j * a_dim1) as isize) = *a.offset((i__ + j * a_dim1) as isize)
                        + *x.offset(ix as isize) * temp1
                        + *y.offset(iy as isize) * temp2;
                    ix += *incx as ::core::ffi::c_long;
                    iy += *incy as ::core::ffi::c_long;
                    i__ += 1;
                }
            }
            jx += *incx as ::core::ffi::c_long;
            jy += *incy as ::core::ffi::c_long;
            j += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
