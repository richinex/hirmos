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
pub unsafe extern "C" fn dsyevd_closure_f2c_dsyr2k(
    mut uplo: *mut ::core::ffi::c_char,
    mut trans: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut k: *mut integer,
    mut alpha: *mut doublereal,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut b: *mut doublereal,
    mut ldb: *mut integer,
    mut beta: *mut doublereal,
    mut c__: *mut doublereal,
    mut ldc: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut b_dim1: integer = 0;
    let mut b_offset: integer = 0;
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut l: integer = 0;
    let mut info: integer = 0;
    let mut temp1: doublereal = 0.;
    let mut temp2: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut nrowa: integer = 0;
    let mut upper: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    b_dim1 = *ldb;
    b_offset = 1 as integer + b_dim1;
    b = b.offset(-(b_offset as isize));
    c_dim1 = *ldc;
    c_offset = 1 as integer + c_dim1;
    c__ = c__.offset(-(c_offset as isize));
    if lsame__0(
        trans,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        nrowa = *n;
    } else {
        nrowa = *k;
    }
    upper = lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    info = 0 as integer;
    if upper == 0
        && lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 1 as integer;
    } else if lsame__0(
        trans,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && lsame__0(
            trans,
            b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
        && lsame__0(
            trans,
            b"C\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 2 as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        info = 3 as integer;
    } else if *k < 0 as ::core::ffi::c_long {
        info = 4 as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= nrowa {
            1 as ::core::ffi::c_long
        } else {
            nrowa as ::core::ffi::c_long
        })
    {
        info = 7 as integer;
    } else if *ldb
        < (if 1 as ::core::ffi::c_long >= nrowa {
            1 as ::core::ffi::c_long
        } else {
            nrowa as ::core::ffi::c_long
        })
    {
        info = 9 as integer;
    } else if *ldc
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        info = 12 as integer;
    }
    if info != 0 as ::core::ffi::c_long {
        xerbla__0(
            b"DSYR2K\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut info,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long
        || (*alpha == 0.0f64 || *k == 0 as ::core::ffi::c_long) && *beta == 1.0f64
    {
        return 0 as ::core::ffi::c_int;
    }
    if *alpha == 0.0f64 {
        if upper != 0 {
            if *beta == 0.0f64 {
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = j;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64 as doublereal;
                        i__ += 1;
                    }
                    j += 1;
                }
            } else {
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = j;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *beta * *c__.offset((i__ + j * c_dim1) as isize);
                        i__ += 1;
                    }
                    j += 1;
                }
            }
        } else if *beta == 0.0f64 {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64 as doublereal;
                    i__ += 1;
                }
                j += 1;
            }
        } else {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    *c__.offset((i__ + j * c_dim1) as isize) =
                        *beta * *c__.offset((i__ + j * c_dim1) as isize);
                    i__ += 1;
                }
                j += 1;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    if lsame__0(
        trans,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if upper != 0 {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                if *beta == 0.0f64 {
                    i__2 = j;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64 as doublereal;
                        i__ += 1;
                    }
                } else if *beta != 1.0f64 {
                    i__2 = j;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *beta * *c__.offset((i__ + j * c_dim1) as isize);
                        i__ += 1;
                    }
                }
                i__2 = *k;
                l = 1 as integer;
                while l <= i__2 {
                    if *a.offset((j + l * a_dim1) as isize) != 0.0f64
                        || *b.offset((j + l * b_dim1) as isize) != 0.0f64
                    {
                        temp1 = *alpha * *b.offset((j + l * b_dim1) as isize);
                        temp2 = *alpha * *a.offset((j + l * a_dim1) as isize);
                        i__3 = j;
                        i__ = 1 as integer;
                        while i__ <= i__3 {
                            *c__.offset((i__ + j * c_dim1) as isize) = *c__
                                .offset((i__ + j * c_dim1) as isize)
                                + *a.offset((i__ + l * a_dim1) as isize) * temp1
                                + *b.offset((i__ + l * b_dim1) as isize) * temp2;
                            i__ += 1;
                        }
                    }
                    l += 1;
                }
                j += 1;
            }
        } else {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                if *beta == 0.0f64 {
                    i__2 = *n;
                    i__ = j;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) = 0.0f64 as doublereal;
                        i__ += 1;
                    }
                } else if *beta != 1.0f64 {
                    i__2 = *n;
                    i__ = j;
                    while i__ <= i__2 {
                        *c__.offset((i__ + j * c_dim1) as isize) =
                            *beta * *c__.offset((i__ + j * c_dim1) as isize);
                        i__ += 1;
                    }
                }
                i__2 = *k;
                l = 1 as integer;
                while l <= i__2 {
                    if *a.offset((j + l * a_dim1) as isize) != 0.0f64
                        || *b.offset((j + l * b_dim1) as isize) != 0.0f64
                    {
                        temp1 = *alpha * *b.offset((j + l * b_dim1) as isize);
                        temp2 = *alpha * *a.offset((j + l * a_dim1) as isize);
                        i__3 = *n;
                        i__ = j;
                        while i__ <= i__3 {
                            *c__.offset((i__ + j * c_dim1) as isize) = *c__
                                .offset((i__ + j * c_dim1) as isize)
                                + *a.offset((i__ + l * a_dim1) as isize) * temp1
                                + *b.offset((i__ + l * b_dim1) as isize) * temp2;
                            i__ += 1;
                        }
                    }
                    l += 1;
                }
                j += 1;
            }
        }
    } else if upper != 0 {
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            i__2 = j;
            i__ = 1 as integer;
            while i__ <= i__2 {
                temp1 = 0.0f64 as doublereal;
                temp2 = 0.0f64 as doublereal;
                i__3 = *k;
                l = 1 as integer;
                while l <= i__3 {
                    temp1 += (*a.offset((l + i__ * a_dim1) as isize)
                        * *b.offset((l + j * b_dim1) as isize))
                        as ::core::ffi::c_double;
                    temp2 += (*b.offset((l + i__ * b_dim1) as isize)
                        * *a.offset((l + j * a_dim1) as isize))
                        as ::core::ffi::c_double;
                    l += 1;
                }
                if *beta == 0.0f64 {
                    *c__.offset((i__ + j * c_dim1) as isize) = *alpha * temp1 + *alpha * temp2;
                } else {
                    *c__.offset((i__ + j * c_dim1) as isize) = *beta
                        * *c__.offset((i__ + j * c_dim1) as isize)
                        + *alpha * temp1
                        + *alpha * temp2;
                }
                i__ += 1;
            }
            j += 1;
        }
    } else {
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            i__2 = *n;
            i__ = j;
            while i__ <= i__2 {
                temp1 = 0.0f64 as doublereal;
                temp2 = 0.0f64 as doublereal;
                i__3 = *k;
                l = 1 as integer;
                while l <= i__3 {
                    temp1 += (*a.offset((l + i__ * a_dim1) as isize)
                        * *b.offset((l + j * b_dim1) as isize))
                        as ::core::ffi::c_double;
                    temp2 += (*b.offset((l + i__ * b_dim1) as isize)
                        * *a.offset((l + j * a_dim1) as isize))
                        as ::core::ffi::c_double;
                    l += 1;
                }
                if *beta == 0.0f64 {
                    *c__.offset((i__ + j * c_dim1) as isize) = *alpha * temp1 + *alpha * temp2;
                } else {
                    *c__.offset((i__ + j * c_dim1) as isize) = *beta
                        * *c__.offset((i__ + j * c_dim1) as isize)
                        + *alpha * temp1
                        + *alpha * temp2;
                }
                i__ += 1;
            }
            j += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
