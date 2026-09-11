pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dtrsm_(
    mut side: *mut ::core::ffi::c_char,
    mut uplo: *mut ::core::ffi::c_char,
    mut transa: *mut ::core::ffi::c_char,
    mut diag: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut alpha: *mut doublereal,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut b: *mut doublereal,
    mut ldb: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut b_dim1: integer = 0;
    let mut b_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut info: integer = 0;
    let mut temp: doublereal = 0.;
    let mut lside: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn dgelsd_closure_lsame__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
        ) -> logical;
    }
    let mut nrowa: integer = 0;
    let mut upper: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn dgelsd_closure_xerbla__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut nounit: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    b_dim1 = *ldb;
    b_offset = 1 as integer + b_dim1;
    b = b.offset(-(b_offset as isize));
    lside = dgelsd_closure_lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if lside != 0 {
        nrowa = *m;
    } else {
        nrowa = *n;
    }
    nounit = dgelsd_closure_lsame__0(
        diag,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    upper = dgelsd_closure_lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    info = 0 as integer;
    if lside == 0
        && dgelsd_closure_lsame__0(
            side,
            b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 1 as integer;
    } else if upper == 0
        && dgelsd_closure_lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 2 as integer;
    } else if dgelsd_closure_lsame__0(
        transa,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && dgelsd_closure_lsame__0(
            transa,
            b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
        && dgelsd_closure_lsame__0(
            transa,
            b"C\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 3 as integer;
    } else if dgelsd_closure_lsame__0(
        diag,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && dgelsd_closure_lsame__0(
            diag,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 4 as integer;
    } else if *m < 0 as ::core::ffi::c_long {
        info = 5 as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        info = 6 as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= nrowa {
            1 as ::core::ffi::c_long
        } else {
            nrowa as ::core::ffi::c_long
        })
    {
        info = 9 as integer;
    } else if *ldb
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        info = 11 as integer;
    }
    if info != 0 as ::core::ffi::c_long {
        dgelsd_closure_xerbla__0(
            b"DTRSM \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut info,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *m == 0 as ::core::ffi::c_long || *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *alpha == 0.0f64 {
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            i__2 = *m;
            i__ = 1 as integer;
            while i__ <= i__2 {
                *b.offset((i__ + j * b_dim1) as isize) = 0.0f64 as doublereal;
                i__ += 1;
            }
            j += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    if lside != 0 {
        if dgelsd_closure_lsame__0(
            transa,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            if upper != 0 {
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    if *alpha != 1.0f64 {
                        i__2 = *m;
                        i__ = 1 as integer;
                        while i__ <= i__2 {
                            *b.offset((i__ + j * b_dim1) as isize) =
                                *alpha * *b.offset((i__ + j * b_dim1) as isize);
                            i__ += 1;
                        }
                    }
                    k = *m;
                    while k >= 1 as ::core::ffi::c_long {
                        if *b.offset((k + j * b_dim1) as isize) != 0.0f64 {
                            if nounit != 0 {
                                let ref mut fresh0 = *b.offset((k + j * b_dim1) as isize);
                                *fresh0 /=
                                    *a.offset((k + k * a_dim1) as isize) as ::core::ffi::c_double;
                            }
                            i__2 = (k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                            i__ = 1 as integer;
                            while i__ <= i__2 {
                                let ref mut fresh1 = *b.offset((i__ + j * b_dim1) as isize);
                                *fresh1 -= (*b.offset((k + j * b_dim1) as isize)
                                    * *a.offset((i__ + k * a_dim1) as isize))
                                    as ::core::ffi::c_double;
                                i__ += 1;
                            }
                        }
                        k -= 1;
                    }
                    j += 1;
                }
            } else {
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    if *alpha != 1.0f64 {
                        i__2 = *m;
                        i__ = 1 as integer;
                        while i__ <= i__2 {
                            *b.offset((i__ + j * b_dim1) as isize) =
                                *alpha * *b.offset((i__ + j * b_dim1) as isize);
                            i__ += 1;
                        }
                    }
                    i__2 = *m;
                    k = 1 as integer;
                    while k <= i__2 {
                        if *b.offset((k + j * b_dim1) as isize) != 0.0f64 {
                            if nounit != 0 {
                                let ref mut fresh2 = *b.offset((k + j * b_dim1) as isize);
                                *fresh2 /=
                                    *a.offset((k + k * a_dim1) as isize) as ::core::ffi::c_double;
                            }
                            i__3 = *m;
                            i__ = (k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                            while i__ <= i__3 {
                                let ref mut fresh3 = *b.offset((i__ + j * b_dim1) as isize);
                                *fresh3 -= (*b.offset((k + j * b_dim1) as isize)
                                    * *a.offset((i__ + k * a_dim1) as isize))
                                    as ::core::ffi::c_double;
                                i__ += 1;
                            }
                        }
                        k += 1;
                    }
                    j += 1;
                }
            }
        } else if upper != 0 {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                i__2 = *m;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    temp = *alpha * *b.offset((i__ + j * b_dim1) as isize);
                    i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    k = 1 as integer;
                    while k <= i__3 {
                        temp -= (*a.offset((k + i__ * a_dim1) as isize)
                            * *b.offset((k + j * b_dim1) as isize))
                            as ::core::ffi::c_double;
                        k += 1;
                    }
                    if nounit != 0 {
                        temp /= *a.offset((i__ + i__ * a_dim1) as isize) as ::core::ffi::c_double;
                    }
                    *b.offset((i__ + j * b_dim1) as isize) = temp;
                    i__ += 1;
                }
                j += 1;
            }
        } else {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                i__ = *m;
                while i__ >= 1 as ::core::ffi::c_long {
                    temp = *alpha * *b.offset((i__ + j * b_dim1) as isize);
                    i__2 = *m;
                    k = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    while k <= i__2 {
                        temp -= (*a.offset((k + i__ * a_dim1) as isize)
                            * *b.offset((k + j * b_dim1) as isize))
                            as ::core::ffi::c_double;
                        k += 1;
                    }
                    if nounit != 0 {
                        temp /= *a.offset((i__ + i__ * a_dim1) as isize) as ::core::ffi::c_double;
                    }
                    *b.offset((i__ + j * b_dim1) as isize) = temp;
                    i__ -= 1;
                }
                j += 1;
            }
        }
    } else if dgelsd_closure_lsame__0(
        transa,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if upper != 0 {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                if *alpha != 1.0f64 {
                    i__2 = *m;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *b.offset((i__ + j * b_dim1) as isize) =
                            *alpha * *b.offset((i__ + j * b_dim1) as isize);
                        i__ += 1;
                    }
                }
                i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                k = 1 as integer;
                while k <= i__2 {
                    if *a.offset((k + j * a_dim1) as isize) != 0.0f64 {
                        i__3 = *m;
                        i__ = 1 as integer;
                        while i__ <= i__3 {
                            let ref mut fresh4 = *b.offset((i__ + j * b_dim1) as isize);
                            *fresh4 -= (*a.offset((k + j * a_dim1) as isize)
                                * *b.offset((i__ + k * b_dim1) as isize))
                                as ::core::ffi::c_double;
                            i__ += 1;
                        }
                    }
                    k += 1;
                }
                if nounit != 0 {
                    temp = 1.0f64 / *a.offset((j + j * a_dim1) as isize);
                    i__2 = *m;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *b.offset((i__ + j * b_dim1) as isize) =
                            temp * *b.offset((i__ + j * b_dim1) as isize);
                        i__ += 1;
                    }
                }
                j += 1;
            }
        } else {
            j = *n;
            while j >= 1 as ::core::ffi::c_long {
                if *alpha != 1.0f64 {
                    i__1 = *m;
                    i__ = 1 as integer;
                    while i__ <= i__1 {
                        *b.offset((i__ + j * b_dim1) as isize) =
                            *alpha * *b.offset((i__ + j * b_dim1) as isize);
                        i__ += 1;
                    }
                }
                i__1 = *n;
                k = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                while k <= i__1 {
                    if *a.offset((k + j * a_dim1) as isize) != 0.0f64 {
                        i__2 = *m;
                        i__ = 1 as integer;
                        while i__ <= i__2 {
                            let ref mut fresh5 = *b.offset((i__ + j * b_dim1) as isize);
                            *fresh5 -= (*a.offset((k + j * a_dim1) as isize)
                                * *b.offset((i__ + k * b_dim1) as isize))
                                as ::core::ffi::c_double;
                            i__ += 1;
                        }
                    }
                    k += 1;
                }
                if nounit != 0 {
                    temp = 1.0f64 / *a.offset((j + j * a_dim1) as isize);
                    i__1 = *m;
                    i__ = 1 as integer;
                    while i__ <= i__1 {
                        *b.offset((i__ + j * b_dim1) as isize) =
                            temp * *b.offset((i__ + j * b_dim1) as isize);
                        i__ += 1;
                    }
                }
                j -= 1;
            }
        }
    } else if upper != 0 {
        k = *n;
        while k >= 1 as ::core::ffi::c_long {
            if nounit != 0 {
                temp = 1.0f64 / *a.offset((k + k * a_dim1) as isize);
                i__1 = *m;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    *b.offset((i__ + k * b_dim1) as isize) =
                        temp * *b.offset((i__ + k * b_dim1) as isize);
                    i__ += 1;
                }
            }
            i__1 = (k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            j = 1 as integer;
            while j <= i__1 {
                if *a.offset((j + k * a_dim1) as isize) != 0.0f64 {
                    temp = *a.offset((j + k * a_dim1) as isize);
                    i__2 = *m;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        let ref mut fresh6 = *b.offset((i__ + j * b_dim1) as isize);
                        *fresh6 -= (temp * *b.offset((i__ + k * b_dim1) as isize))
                            as ::core::ffi::c_double;
                        i__ += 1;
                    }
                }
                j += 1;
            }
            if *alpha != 1.0f64 {
                i__1 = *m;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    *b.offset((i__ + k * b_dim1) as isize) =
                        *alpha * *b.offset((i__ + k * b_dim1) as isize);
                    i__ += 1;
                }
            }
            k -= 1;
        }
    } else {
        i__1 = *n;
        k = 1 as integer;
        while k <= i__1 {
            if nounit != 0 {
                temp = 1.0f64 / *a.offset((k + k * a_dim1) as isize);
                i__2 = *m;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    *b.offset((i__ + k * b_dim1) as isize) =
                        temp * *b.offset((i__ + k * b_dim1) as isize);
                    i__ += 1;
                }
            }
            i__2 = *n;
            j = (k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            while j <= i__2 {
                if *a.offset((j + k * a_dim1) as isize) != 0.0f64 {
                    temp = *a.offset((j + k * a_dim1) as isize);
                    i__3 = *m;
                    i__ = 1 as integer;
                    while i__ <= i__3 {
                        let ref mut fresh7 = *b.offset((i__ + j * b_dim1) as isize);
                        *fresh7 -= (temp * *b.offset((i__ + k * b_dim1) as isize))
                            as ::core::ffi::c_double;
                        i__ += 1;
                    }
                }
                j += 1;
            }
            if *alpha != 1.0f64 {
                i__2 = *m;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    *b.offset((i__ + k * b_dim1) as isize) =
                        *alpha * *b.offset((i__ + k * b_dim1) as isize);
                    i__ += 1;
                }
            }
            k += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
