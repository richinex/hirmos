pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dtrsv(
    mut uplo: *mut ::core::ffi::c_char,
    mut trans: *mut ::core::ffi::c_char,
    mut diag: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut x: *mut doublereal,
    mut incx: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut ix: integer = 0;
    let mut jx: integer = 0;
    let mut kx: integer = 0;
    let mut info: integer = 0;
    let mut temp: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn dgelsd_closure_lsame__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
        ) -> logical;
    }
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
    x = x.offset(-1);
    info = 0 as integer;
    if dgelsd_closure_lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && dgelsd_closure_lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 1 as integer;
    } else if dgelsd_closure_lsame__0(
        trans,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && dgelsd_closure_lsame__0(
            trans,
            b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
        && dgelsd_closure_lsame__0(
            trans,
            b"C\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 2 as integer;
    } else if dgelsd_closure_lsame__0(
        diag,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && dgelsd_closure_lsame__0(
            diag,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        info = 3 as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        info = 4 as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        info = 6 as integer;
    } else if *incx == 0 as ::core::ffi::c_long {
        info = 8 as integer;
    }
    if info != 0 as ::core::ffi::c_long {
        dgelsd_closure_xerbla__0(
            b"DTRSV \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut info,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    nounit = dgelsd_closure_lsame__0(
        diag,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if *incx <= 0 as ::core::ffi::c_long {
        kx = 1 as integer - (*n - 1 as integer) * *incx;
    } else if *incx != 1 as ::core::ffi::c_long {
        kx = 1 as integer;
    }
    if dgelsd_closure_lsame__0(
        trans,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if dgelsd_closure_lsame__0(
            uplo,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            if *incx == 1 as ::core::ffi::c_long {
                j = *n;
                while j >= 1 as ::core::ffi::c_long {
                    if *x.offset(j as isize) != 0.0f64 {
                        if nounit != 0 {
                            let ref mut fresh0 = *x.offset(j as isize);
                            *fresh0 /=
                                *a.offset((j + j * a_dim1) as isize) as ::core::ffi::c_double;
                        }
                        temp = *x.offset(j as isize);
                        i__ = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        while i__ >= 1 as ::core::ffi::c_long {
                            let ref mut fresh1 = *x.offset(i__ as isize);
                            *fresh1 -= (temp * *a.offset((i__ + j * a_dim1) as isize))
                                as ::core::ffi::c_double;
                            i__ -= 1;
                        }
                    }
                    j -= 1;
                }
            } else {
                jx = kx + (*n - 1 as integer) * *incx;
                j = *n;
                while j >= 1 as ::core::ffi::c_long {
                    if *x.offset(jx as isize) != 0.0f64 {
                        if nounit != 0 {
                            let ref mut fresh2 = *x.offset(jx as isize);
                            *fresh2 /=
                                *a.offset((j + j * a_dim1) as isize) as ::core::ffi::c_double;
                        }
                        temp = *x.offset(jx as isize);
                        ix = jx;
                        i__ = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        while i__ >= 1 as ::core::ffi::c_long {
                            ix -= *incx as ::core::ffi::c_long;
                            let ref mut fresh3 = *x.offset(ix as isize);
                            *fresh3 -= (temp * *a.offset((i__ + j * a_dim1) as isize))
                                as ::core::ffi::c_double;
                            i__ -= 1;
                        }
                    }
                    jx -= *incx as ::core::ffi::c_long;
                    j -= 1;
                }
            }
        } else if *incx == 1 as ::core::ffi::c_long {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                if *x.offset(j as isize) != 0.0f64 {
                    if nounit != 0 {
                        let ref mut fresh4 = *x.offset(j as isize);
                        *fresh4 /= *a.offset((j + j * a_dim1) as isize) as ::core::ffi::c_double;
                    }
                    temp = *x.offset(j as isize);
                    i__2 = *n;
                    i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    while i__ <= i__2 {
                        let ref mut fresh5 = *x.offset(i__ as isize);
                        *fresh5 -= (temp * *a.offset((i__ + j * a_dim1) as isize))
                            as ::core::ffi::c_double;
                        i__ += 1;
                    }
                }
                j += 1;
            }
        } else {
            jx = kx;
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                if *x.offset(jx as isize) != 0.0f64 {
                    if nounit != 0 {
                        let ref mut fresh6 = *x.offset(jx as isize);
                        *fresh6 /= *a.offset((j + j * a_dim1) as isize) as ::core::ffi::c_double;
                    }
                    temp = *x.offset(jx as isize);
                    ix = jx;
                    i__2 = *n;
                    i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    while i__ <= i__2 {
                        ix += *incx as ::core::ffi::c_long;
                        let ref mut fresh7 = *x.offset(ix as isize);
                        *fresh7 -= (temp * *a.offset((i__ + j * a_dim1) as isize))
                            as ::core::ffi::c_double;
                        i__ += 1;
                    }
                }
                jx += *incx as ::core::ffi::c_long;
                j += 1;
            }
        }
    } else if dgelsd_closure_lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if *incx == 1 as ::core::ffi::c_long {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                temp = *x.offset(j as isize);
                i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    temp -= (*a.offset((i__ + j * a_dim1) as isize) * *x.offset(i__ as isize))
                        as ::core::ffi::c_double;
                    i__ += 1;
                }
                if nounit != 0 {
                    temp /= *a.offset((j + j * a_dim1) as isize) as ::core::ffi::c_double;
                }
                *x.offset(j as isize) = temp;
                j += 1;
            }
        } else {
            jx = kx;
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                temp = *x.offset(jx as isize);
                ix = kx;
                i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    temp -= (*a.offset((i__ + j * a_dim1) as isize) * *x.offset(ix as isize))
                        as ::core::ffi::c_double;
                    ix += *incx as ::core::ffi::c_long;
                    i__ += 1;
                }
                if nounit != 0 {
                    temp /= *a.offset((j + j * a_dim1) as isize) as ::core::ffi::c_double;
                }
                *x.offset(jx as isize) = temp;
                jx += *incx as ::core::ffi::c_long;
                j += 1;
            }
        }
    } else if *incx == 1 as ::core::ffi::c_long {
        j = *n;
        while j >= 1 as ::core::ffi::c_long {
            temp = *x.offset(j as isize);
            i__1 = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__ = *n;
            while i__ >= i__1 {
                temp -= (*a.offset((i__ + j * a_dim1) as isize) * *x.offset(i__ as isize))
                    as ::core::ffi::c_double;
                i__ -= 1;
            }
            if nounit != 0 {
                temp /= *a.offset((j + j * a_dim1) as isize) as ::core::ffi::c_double;
            }
            *x.offset(j as isize) = temp;
            j -= 1;
        }
    } else {
        kx += ((*n - 1 as integer) * *incx) as ::core::ffi::c_long;
        jx = kx;
        j = *n;
        while j >= 1 as ::core::ffi::c_long {
            temp = *x.offset(jx as isize);
            ix = kx;
            i__1 = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__ = *n;
            while i__ >= i__1 {
                temp -= (*a.offset((i__ + j * a_dim1) as isize) * *x.offset(ix as isize))
                    as ::core::ffi::c_double;
                ix -= *incx as ::core::ffi::c_long;
                i__ -= 1;
            }
            if nounit != 0 {
                temp /= *a.offset((j + j * a_dim1) as isize) as ::core::ffi::c_double;
            }
            *x.offset(jx as isize) = temp;
            jx -= *incx as ::core::ffi::c_long;
            j -= 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
