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
pub unsafe extern "C" fn dsyevd_closure_f2c_dsymv(
    mut uplo: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut alpha: *mut doublereal,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut x: *mut doublereal,
    mut incx: *mut integer,
    mut beta: *mut doublereal,
    mut y: *mut doublereal,
    mut incy: *mut integer,
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
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    x = x.offset(-1);
    y = y.offset(-1);
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
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        info = 5 as integer;
    } else if *incx == 0 as ::core::ffi::c_long {
        info = 7 as integer;
    } else if *incy == 0 as ::core::ffi::c_long {
        info = 10 as integer;
    }
    if info != 0 as ::core::ffi::c_long {
        xerbla__0(
            b"DSYMV \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut info,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long || *alpha == 0.0f64 && *beta == 1.0f64 {
        return 0 as ::core::ffi::c_int;
    }
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
    if *beta != 1.0f64 {
        if *incy == 1 as ::core::ffi::c_long {
            if *beta == 0.0f64 {
                i__1 = *n;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    *y.offset(i__ as isize) = 0.0f64 as doublereal;
                    i__ += 1;
                }
            } else {
                i__1 = *n;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    *y.offset(i__ as isize) = *beta * *y.offset(i__ as isize);
                    i__ += 1;
                }
            }
        } else {
            iy = ky;
            if *beta == 0.0f64 {
                i__1 = *n;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    *y.offset(iy as isize) = 0.0f64 as doublereal;
                    iy += *incy as ::core::ffi::c_long;
                    i__ += 1;
                }
            } else {
                i__1 = *n;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    *y.offset(iy as isize) = *beta * *y.offset(iy as isize);
                    iy += *incy as ::core::ffi::c_long;
                    i__ += 1;
                }
            }
        }
    }
    if *alpha == 0.0f64 {
        return 0 as ::core::ffi::c_int;
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
                temp1 = *alpha * *x.offset(j as isize);
                temp2 = 0.0f64 as doublereal;
                i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    let ref mut fresh0 = *y.offset(i__ as isize);
                    *fresh0 +=
                        (temp1 * *a.offset((i__ + j * a_dim1) as isize)) as ::core::ffi::c_double;
                    temp2 += (*a.offset((i__ + j * a_dim1) as isize) * *x.offset(i__ as isize))
                        as ::core::ffi::c_double;
                    i__ += 1;
                }
                *y.offset(j as isize) = *y.offset(j as isize)
                    + temp1 * *a.offset((j + j * a_dim1) as isize)
                    + *alpha * temp2;
                j += 1;
            }
        } else {
            jx = kx;
            jy = ky;
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                temp1 = *alpha * *x.offset(jx as isize);
                temp2 = 0.0f64 as doublereal;
                ix = kx;
                iy = ky;
                i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    let ref mut fresh1 = *y.offset(iy as isize);
                    *fresh1 +=
                        (temp1 * *a.offset((i__ + j * a_dim1) as isize)) as ::core::ffi::c_double;
                    temp2 += (*a.offset((i__ + j * a_dim1) as isize) * *x.offset(ix as isize))
                        as ::core::ffi::c_double;
                    ix += *incx as ::core::ffi::c_long;
                    iy += *incy as ::core::ffi::c_long;
                    i__ += 1;
                }
                *y.offset(jy as isize) = *y.offset(jy as isize)
                    + temp1 * *a.offset((j + j * a_dim1) as isize)
                    + *alpha * temp2;
                jx += *incx as ::core::ffi::c_long;
                jy += *incy as ::core::ffi::c_long;
                j += 1;
            }
        }
    } else if *incx == 1 as ::core::ffi::c_long && *incy == 1 as ::core::ffi::c_long {
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            temp1 = *alpha * *x.offset(j as isize);
            temp2 = 0.0f64 as doublereal;
            let ref mut fresh2 = *y.offset(j as isize);
            *fresh2 += (temp1 * *a.offset((j + j * a_dim1) as isize)) as ::core::ffi::c_double;
            i__2 = *n;
            i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            while i__ <= i__2 {
                let ref mut fresh3 = *y.offset(i__ as isize);
                *fresh3 +=
                    (temp1 * *a.offset((i__ + j * a_dim1) as isize)) as ::core::ffi::c_double;
                temp2 += (*a.offset((i__ + j * a_dim1) as isize) * *x.offset(i__ as isize))
                    as ::core::ffi::c_double;
                i__ += 1;
            }
            let ref mut fresh4 = *y.offset(j as isize);
            *fresh4 += (*alpha * temp2) as ::core::ffi::c_double;
            j += 1;
        }
    } else {
        jx = kx;
        jy = ky;
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            temp1 = *alpha * *x.offset(jx as isize);
            temp2 = 0.0f64 as doublereal;
            let ref mut fresh5 = *y.offset(jy as isize);
            *fresh5 += (temp1 * *a.offset((j + j * a_dim1) as isize)) as ::core::ffi::c_double;
            ix = jx;
            iy = jy;
            i__2 = *n;
            i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            while i__ <= i__2 {
                ix += *incx as ::core::ffi::c_long;
                iy += *incy as ::core::ffi::c_long;
                let ref mut fresh6 = *y.offset(iy as isize);
                *fresh6 +=
                    (temp1 * *a.offset((i__ + j * a_dim1) as isize)) as ::core::ffi::c_double;
                temp2 += (*a.offset((i__ + j * a_dim1) as isize) * *x.offset(ix as isize))
                    as ::core::ffi::c_double;
                i__ += 1;
            }
            let ref mut fresh7 = *y.offset(jy as isize);
            *fresh7 += (*alpha * temp2) as ::core::ffi::c_double;
            jx += *incx as ::core::ffi::c_long;
            jy += *incy as ::core::ffi::c_long;
            j += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
