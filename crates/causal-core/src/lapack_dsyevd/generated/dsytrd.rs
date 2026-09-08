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
static mut c__1: integer = 1 as integer;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
static mut c__3: integer = 3 as integer;
static mut c__2: integer = 2 as integer;
static mut c_b22: doublereal = -1.0f64;
static mut c_b23: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dsytrd_(
    mut uplo: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut tau: *mut doublereal,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut nb: integer = 0;
    let mut kk: integer = 0;
    let mut nx: integer = 0;
    let mut iws: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut nbmin: integer = 0;
    let mut iinfo: integer = 0;
    let mut upper: logical = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dsytd2_"]
        fn dsytd2__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_dsyr2k"]
        fn f2c_dsyr2k_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_dlatrd_"]
        fn dlatrd__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_ilaenv_"]
        fn ilaenv__0(
            _: *mut integer,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
        ) -> integer;
    }
    let mut ldwork: integer = 0;
    let mut lwkopt: integer = 0;
    let mut lquery: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    d__ = d__.offset(-1);
    e = e.offset(-1);
    tau = tau.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    upper = lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
    if upper == 0
        && lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *lwork < 1 as ::core::ffi::c_long && lquery == 0 {
        *info = -(9 as ::core::ffi::c_int) as integer;
    }
    if *info == 0 as ::core::ffi::c_long {
        nb = ilaenv__0(
            &raw mut c__1,
            b"DSYTRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            uplo,
            n,
            &raw mut c_n1,
            &raw mut c_n1,
            &raw mut c_n1,
        );
        lwkopt = *n * nb;
        *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DSYTRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    nx = *n;
    iws = 1 as integer;
    if nb > 1 as ::core::ffi::c_long && nb < *n {
        i__1 = nb;
        i__2 = ilaenv__0(
            &raw mut c__3,
            b"DSYTRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            uplo,
            n,
            &raw mut c_n1,
            &raw mut c_n1,
            &raw mut c_n1,
        );
        nx = (if i__1 >= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        if nx < *n {
            ldwork = *n;
            iws = ldwork * nb;
            if *lwork < iws {
                i__1 = *lwork / ldwork;
                nb = (if i__1 >= 1 as ::core::ffi::c_long {
                    i__1 as ::core::ffi::c_long
                } else {
                    1 as ::core::ffi::c_long
                }) as integer;
                nbmin = ilaenv__0(
                    &raw mut c__2,
                    b"DSYTRD\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    uplo,
                    n,
                    &raw mut c_n1,
                    &raw mut c_n1,
                    &raw mut c_n1,
                );
                if nb < nbmin {
                    nx = *n;
                }
            }
        } else {
            nx = *n;
        }
    } else {
        nb = 1 as integer;
    }
    if upper != 0 {
        kk = *n - (*n - nx + nb - 1 as integer) / nb * nb;
        i__1 = (kk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        i__2 = -nb;
        i__ = (*n - nb as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        while if i__2 < 0 as ::core::ffi::c_long {
            (i__ >= i__1) as ::core::ffi::c_int
        } else {
            (i__ <= i__1) as ::core::ffi::c_int
        } != 0
        {
            i__3 = (i__ as ::core::ffi::c_long + nb as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as integer;
            dlatrd__0(
                uplo,
                &raw mut i__3,
                &raw mut nb,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut ldwork,
            );
            i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dsyr2k_0(
                uplo,
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__3,
                &raw mut nb,
                &raw mut c_b22,
                a.offset(
                    (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                lda,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut ldwork,
                &raw mut c_b23,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
            );
            i__3 = (i__ as ::core::ffi::c_long + nb as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as integer;
            j = i__;
            while j <= i__3 {
                *a.offset((j - 1 as integer + j * a_dim1) as isize) =
                    *e.offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                *d__.offset(j as isize) = *a.offset((j + j * a_dim1) as isize);
                j += 1;
            }
            i__ += i__2 as ::core::ffi::c_long;
        }
        dsytd2__0(
            uplo,
            &raw mut kk,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut iinfo,
        );
    } else {
        i__2 = *n - nx;
        i__1 = nb;
        i__ = 1 as integer;
        while if i__1 < 0 as ::core::ffi::c_long {
            (i__ >= i__2) as ::core::ffi::c_int
        } else {
            (i__ <= i__2) as ::core::ffi::c_int
        } != 0
        {
            i__3 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dlatrd__0(
                uplo,
                &raw mut i__3,
                &raw mut nb,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
                e.offset(i__ as isize) as *mut doublereal,
                tau.offset(i__ as isize) as *mut doublereal,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut ldwork,
            );
            i__3 = (*n - i__ as ::core::ffi::c_long - nb as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            f2c_dsyr2k_0(
                uplo,
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__3,
                &raw mut nb,
                &raw mut c_b22,
                a.offset((i__ + nb + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
                work.offset((nb as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut ldwork,
                &raw mut c_b23,
                a.offset((i__ + nb + (i__ + nb) * a_dim1) as isize) as *mut doublereal,
                lda,
            );
            i__3 = (i__ as ::core::ffi::c_long + nb as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as integer;
            j = i__;
            while j <= i__3 {
                *a.offset((j + 1 as integer + j * a_dim1) as isize) = *e.offset(j as isize);
                *d__.offset(j as isize) = *a.offset((j + j * a_dim1) as isize);
                j += 1;
            }
            i__ += i__1 as ::core::ffi::c_long;
        }
        i__1 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dsytd2__0(
            uplo,
            &raw mut i__1,
            a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
            lda,
            d__.offset(i__ as isize) as *mut doublereal,
            e.offset(i__ as isize) as *mut doublereal,
            tau.offset(i__ as isize) as *mut doublereal,
            &raw mut iinfo,
        );
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    return 0 as ::core::ffi::c_int;
}
