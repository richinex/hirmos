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
static mut c__65: integer = 65 as integer;
static mut c_b25: doublereal = -1.0f64;
static mut c_b26: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dgehrd_(
    mut n: *mut integer,
    mut ilo: *mut integer,
    mut ihi: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
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
    let mut i__4: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut t: [doublereal; 4160] = [0.; 4160];
    let mut ib: integer = 0;
    let mut ei: doublereal = 0.;
    let mut nb: integer = 0;
    let mut nh: integer = 0;
    let mut nx: integer = 0;
    let mut iws: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dgemm"]
        fn f2c_dgemm_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
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
    let mut nbmin: integer = 0;
    let mut iinfo: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_dtrmm"]
        fn f2c_dtrmm_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_daxpy"]
        fn f2c_daxpy_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dgehd2_"]
        fn dgehd2__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlahr2_"]
        fn dlahr2__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlarfb_"]
        fn dlarfb__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_ilaenv_"]
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
    tau = tau.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    i__1 = 64 as integer;
    i__2 = ilaenv__0(
        &raw mut c__1,
        b"DGEHRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        n,
        ilo,
        ihi,
        &raw mut c_n1,
    );
    nb = (if i__1 <= i__2 {
        i__1 as ::core::ffi::c_long
    } else {
        i__2 as ::core::ffi::c_long
    }) as integer;
    lwkopt = *n * nb;
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
    if *n < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *ilo < 1 as ::core::ffi::c_long
        || *ilo
            > (if 1 as ::core::ffi::c_long >= *n {
                1 as ::core::ffi::c_long
            } else {
                *n
            })
    {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *ihi < (if *ilo <= *n { *ilo } else { *n }) || *ihi > *n {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(5 as ::core::ffi::c_int) as integer;
    } else if *lwork
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
        && lquery == 0
    {
        *info = -(8 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DGEHRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    i__1 = (*ilo - 1 as ::core::ffi::c_long) as integer;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *tau.offset(i__ as isize) = 0.0f64 as doublereal;
        i__ += 1;
    }
    i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
    i__ = (if 1 as ::core::ffi::c_long >= *ihi {
        1 as ::core::ffi::c_long
    } else {
        *ihi
    }) as integer;
    while i__ <= i__1 {
        *tau.offset(i__ as isize) = 0.0f64 as doublereal;
        i__ += 1;
    }
    nh = (*ihi - *ilo + 1 as ::core::ffi::c_long) as integer;
    if nh <= 1 as ::core::ffi::c_long {
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    i__1 = 64 as integer;
    i__2 = ilaenv__0(
        &raw mut c__1,
        b"DGEHRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        n,
        ilo,
        ihi,
        &raw mut c_n1,
    );
    nb = (if i__1 <= i__2 {
        i__1 as ::core::ffi::c_long
    } else {
        i__2 as ::core::ffi::c_long
    }) as integer;
    nbmin = 2 as integer;
    iws = 1 as integer;
    if nb > 1 as ::core::ffi::c_long && nb < nh {
        i__1 = nb;
        i__2 = ilaenv__0(
            &raw mut c__3,
            b"DGEHRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            ilo,
            ihi,
            &raw mut c_n1,
        );
        nx = (if i__1 >= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        if nx < nh {
            iws = *n * nb;
            if *lwork < iws {
                i__1 = 2 as integer;
                i__2 = ilaenv__0(
                    &raw mut c__2,
                    b"DGEHRD\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    ilo,
                    ihi,
                    &raw mut c_n1,
                );
                nbmin = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
                if *lwork >= *n * nbmin {
                    nb = *lwork / *n;
                } else {
                    nb = 1 as integer;
                }
            }
        }
    }
    ldwork = *n;
    if nb < nbmin || nb >= nh {
        i__ = *ilo;
    } else {
        i__1 = *ihi - 1 as integer - nx;
        i__2 = nb;
        i__ = *ilo;
        while if i__2 < 0 as ::core::ffi::c_long {
            (i__ >= i__1) as ::core::ffi::c_int
        } else {
            (i__ <= i__1) as ::core::ffi::c_int
        } != 0
        {
            i__3 = nb;
            i__4 = *ihi - i__;
            ib = (if i__3 <= i__4 {
                i__3 as ::core::ffi::c_long
            } else {
                i__4 as ::core::ffi::c_long
            }) as integer;
            dlahr2__0(
                ihi,
                &raw mut i__,
                &raw mut ib,
                a.offset(
                    (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                lda,
                tau.offset(i__ as isize) as *mut doublereal,
                &raw mut t as *mut doublereal,
                &raw mut c__65,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut ldwork,
            );
            ei = *a.offset((i__ + ib + (i__ + ib - 1 as integer) * a_dim1) as isize);
            *a.offset((i__ + ib + (i__ + ib - 1 as integer) * a_dim1) as isize) =
                1.0f64 as doublereal;
            i__3 = (*ihi - i__ as ::core::ffi::c_long - ib as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            f2c_dgemm_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                ihi,
                &raw mut i__3,
                &raw mut ib,
                &raw mut c_b25,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut ldwork,
                a.offset((i__ + ib + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
                &raw mut c_b26,
                a.offset(
                    ((i__ as ::core::ffi::c_long + ib as ::core::ffi::c_long)
                        * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                lda,
            );
            *a.offset((i__ + ib + (i__ + ib - 1 as integer) * a_dim1) as isize) = ei;
            i__3 = (ib as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dtrmm_0(
                b"Right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"Lower\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"Unit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__,
                &raw mut i__3,
                &raw mut c_b26,
                a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut ldwork,
            );
            i__3 = (ib as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as integer;
            j = 0 as integer;
            while j <= i__3 {
                f2c_daxpy_0(
                    &raw mut i__,
                    &raw mut c_b25,
                    work.offset(
                        (ldwork as ::core::ffi::c_long * j as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    a.offset(
                        ((i__ as ::core::ffi::c_long
                            + j as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long)
                            * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                j += 1;
            }
            i__3 = *ihi - i__;
            i__4 = (*n - i__ as ::core::ffi::c_long - ib as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            dlarfb__0(
                b"Left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"Forward\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"Columnwise\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__3,
                &raw mut i__4,
                &raw mut ib,
                a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
                &raw mut t as *mut doublereal,
                &raw mut c__65,
                a.offset((i__ + 1 as integer + (i__ + ib) * a_dim1) as isize) as *mut doublereal,
                lda,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut ldwork,
            );
            i__ += i__2 as ::core::ffi::c_long;
        }
    }
    dgehd2__0(
        n,
        &raw mut i__,
        ihi,
        a.offset(a_offset as isize) as *mut doublereal,
        lda,
        tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut iinfo,
    );
    *work.offset(1 as ::core::ffi::c_int as isize) = iws as doublereal;
    return 0 as ::core::ffi::c_int;
}
