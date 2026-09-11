pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
static mut c__1: integer = 1 as integer;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
static mut c__0: integer = 0 as integer;
static mut c_b31: doublereal = 0.0f64;
static mut c__2: integer = 2 as integer;
static mut c_b54: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dgelsy_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut nrhs: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut b: *mut doublereal,
    mut ldb: *mut integer,
    mut jpvt: *mut integer,
    mut rcond: *mut doublereal,
    mut rank: *mut integer,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut b_dim1: integer = 0;
    let mut b_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut c1: doublereal = 0.;
    let mut c2: doublereal = 0.;
    let mut s1: doublereal = 0.;
    let mut s2: doublereal = 0.;
    let mut nb: integer = 0;
    let mut mn: integer = 0;
    let mut nb1: integer = 0;
    let mut nb2: integer = 0;
    let mut nb3: integer = 0;
    let mut nb4: integer = 0;
    let mut anrm: doublereal = 0.;
    let mut bnrm: doublereal = 0.;
    let mut smin: doublereal = 0.;
    let mut smax: doublereal = 0.;
    let mut iascl: integer = 0;
    let mut ibscl: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dcopy"]
        fn dgelsd_closure_f2c_dcopy_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut ismin: integer = 0;
    let mut ismax: integer = 0;
    extern "C" {
        #[link_name = "slsqp_closure_dtrsm_"]
        fn slsqp_closure_dtrsm__0(
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
        #[link_name = "slsqp_closure_dlaic1_"]
        fn slsqp_closure_dlaic1__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    let mut wsize: doublereal = 0.;
    extern "C" {
        #[link_name = "slsqp_closure_dgeqp3_"]
        fn slsqp_closure_dgeqp3__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dgelsd_closure_dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlange_"]
        fn dgeev_closure_dlange__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlascl_"]
        fn dgelsd_closure_dlascl__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlaset_"]
        fn dgelsd_closure_dlaset__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn dgelsd_closure_xerbla__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_ilaenv_"]
        fn dgelsd_closure_ilaenv__0(
            _: *mut integer,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
        ) -> integer;
    }
    let mut bignum: doublereal = 0.;
    let mut lwkmin: integer = 0;
    extern "C" {
        #[link_name = "dgeev_closure_dormqr_"]
        fn dgeev_closure_dormqr__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
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
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut sminpr: doublereal = 0.;
    let mut smaxpr: doublereal = 0.;
    let mut smlnum: doublereal = 0.;
    extern "C" {
        #[link_name = "slsqp_closure_dormrz_"]
        fn slsqp_closure_dormrz__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
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
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut lwkopt: integer = 0;
    let mut lquery: logical = 0;
    extern "C" {
        #[link_name = "slsqp_closure_dtzrzf_"]
        fn slsqp_closure_dtzrzf__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    b_dim1 = *ldb;
    b_offset = 1 as integer + b_dim1;
    b = b.offset(-(b_offset as isize));
    jpvt = jpvt.offset(-1);
    work = work.offset(-1);
    mn = (if *m <= *n { *m } else { *n }) as integer;
    ismin = (mn as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    ismax = (((mn as ::core::ffi::c_long) << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long)
        as integer;
    *info = 0 as integer;
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
    if *m < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *nrhs < 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        *info = -(5 as ::core::ffi::c_int) as integer;
    } else {
        i__1 = (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        }) as integer;
        if *ldb
            < (if i__1 >= *n {
                i__1 as ::core::ffi::c_long
            } else {
                *n
            })
        {
            *info = -(7 as ::core::ffi::c_int) as integer;
        }
    }
    if *info == 0 as ::core::ffi::c_long {
        if mn == 0 as ::core::ffi::c_long || *nrhs == 0 as ::core::ffi::c_long {
            lwkmin = 1 as integer;
            lwkopt = 1 as integer;
        } else {
            nb1 = dgelsd_closure_ilaenv__0(
                &raw mut c__1,
                b"DGEQRF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                &raw mut c_n1,
                &raw mut c_n1,
            );
            nb2 = dgelsd_closure_ilaenv__0(
                &raw mut c__1,
                b"DGERQF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                &raw mut c_n1,
                &raw mut c_n1,
            );
            nb3 = dgelsd_closure_ilaenv__0(
                &raw mut c__1,
                b"DORMQR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                nrhs,
                &raw mut c_n1,
            );
            nb4 = dgelsd_closure_ilaenv__0(
                &raw mut c__1,
                b"DORMRQ\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                nrhs,
                &raw mut c_n1,
            );
            i__1 = (if nb1 >= nb2 {
                nb1 as ::core::ffi::c_long
            } else {
                nb2 as ::core::ffi::c_long
            }) as integer;
            i__1 = (if i__1 >= nb3 {
                i__1 as ::core::ffi::c_long
            } else {
                nb3 as ::core::ffi::c_long
            }) as integer;
            nb = (if i__1 >= nb4 {
                i__1 as ::core::ffi::c_long
            } else {
                nb4 as ::core::ffi::c_long
            }) as integer;
            i__1 = mn << 1 as ::core::ffi::c_int;
            i__2 = (*n + 1 as ::core::ffi::c_long) as integer;
            i__1 = (if i__1 >= i__2 {
                i__1 as ::core::ffi::c_long
            } else {
                i__2 as ::core::ffi::c_long
            }) as integer;
            i__2 = mn + *nrhs;
            lwkmin = (mn as ::core::ffi::c_long
                + (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                })) as integer;
            i__1 = lwkmin;
            i__2 = (mn as ::core::ffi::c_long
                + (*n << 1 as ::core::ffi::c_int)
                + nb as ::core::ffi::c_long * (*n + 1 as ::core::ffi::c_long))
                as integer;
            i__1 = (if i__1 >= i__2 {
                i__1 as ::core::ffi::c_long
            } else {
                i__2 as ::core::ffi::c_long
            }) as integer;
            i__2 = (mn << 1 as ::core::ffi::c_int) + nb * *nrhs;
            lwkopt = (if i__1 >= i__2 {
                i__1 as ::core::ffi::c_long
            } else {
                i__2 as ::core::ffi::c_long
            }) as integer;
        }
        *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
        if *lwork < lwkmin && lquery == 0 {
            *info = -(12 as ::core::ffi::c_int) as integer;
        }
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        dgelsd_closure_xerbla__0(
            b"DGELSY\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if mn == 0 as ::core::ffi::c_long || *nrhs == 0 as ::core::ffi::c_long {
        *rank = 0 as integer;
        return 0 as ::core::ffi::c_int;
    }
    smlnum = dgelsd_closure_dlamch__0(
        b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) / dgelsd_closure_dlamch__0(
        b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    bignum = 1.0f64 / smlnum;
    anrm = dgeev_closure_dlange__0(
        b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        m,
        n,
        a.offset(a_offset as isize) as *mut doublereal,
        lda,
        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
    );
    iascl = 0 as integer;
    if anrm > 0.0f64 && anrm < smlnum {
        dgelsd_closure_dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut anrm,
            &raw mut smlnum,
            m,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            info,
        );
        iascl = 1 as integer;
        current_block = 317151059986244064;
    } else if anrm > bignum {
        dgelsd_closure_dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut anrm,
            &raw mut bignum,
            m,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            info,
        );
        iascl = 2 as integer;
        current_block = 317151059986244064;
    } else if anrm == 0.0f64 {
        i__1 = (if *m >= *n { *m } else { *n }) as integer;
        dgelsd_closure_dlaset__0(
            b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
            nrhs,
            &raw mut c_b31,
            &raw mut c_b31,
            b.offset(b_offset as isize) as *mut doublereal,
            ldb,
        );
        *rank = 0 as integer;
        current_block = 11528314641311018142;
    } else {
        current_block = 317151059986244064;
    }
    match current_block {
        317151059986244064 => {
            bnrm = dgeev_closure_dlange__0(
                b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                nrhs,
                b.offset(b_offset as isize) as *mut doublereal,
                ldb,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            );
            ibscl = 0 as integer;
            if bnrm > 0.0f64 && bnrm < smlnum {
                dgelsd_closure_dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut bnrm,
                    &raw mut smlnum,
                    m,
                    nrhs,
                    b.offset(b_offset as isize) as *mut doublereal,
                    ldb,
                    info,
                );
                ibscl = 1 as integer;
            } else if bnrm > bignum {
                dgelsd_closure_dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut bnrm,
                    &raw mut bignum,
                    m,
                    nrhs,
                    b.offset(b_offset as isize) as *mut doublereal,
                    ldb,
                    info,
                );
                ibscl = 2 as integer;
            }
            i__1 = *lwork - mn;
            slsqp_closure_dgeqp3__0(
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                jpvt.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset((mn as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut i__1,
                info,
            );
            wsize = mn as doublereal
                + *work.offset((mn as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            *work.offset(ismin as isize) = 1.0f64 as doublereal;
            *work.offset(ismax as isize) = 1.0f64 as doublereal;
            d__1 = *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            smax = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            smin = smax;
            d__1 = *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) == 0.0f64
            {
                *rank = 0 as integer;
                i__1 = (if *m >= *n { *m } else { *n }) as integer;
                dgelsd_closure_dlaset__0(
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut i__1,
                    nrhs,
                    &raw mut c_b31,
                    &raw mut c_b31,
                    b.offset(b_offset as isize) as *mut doublereal,
                    ldb,
                );
            } else {
                *rank = 1 as integer;
                while *rank < mn {
                    i__ = (*rank + 1 as ::core::ffi::c_long) as integer;
                    slsqp_closure_dlaic1__0(
                        &raw mut c__2,
                        rank,
                        work.offset(ismin as isize) as *mut doublereal,
                        &raw mut smin,
                        a.offset(
                            (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                        &raw mut sminpr,
                        &raw mut s1,
                        &raw mut c1,
                    );
                    slsqp_closure_dlaic1__0(
                        &raw mut c__1,
                        rank,
                        work.offset(ismax as isize) as *mut doublereal,
                        &raw mut smax,
                        a.offset(
                            (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                        &raw mut smaxpr,
                        &raw mut s2,
                        &raw mut c2,
                    );
                    if !(smaxpr * *rcond <= sminpr) {
                        break;
                    }
                    i__1 = *rank;
                    i__ = 1 as integer;
                    while i__ <= i__1 {
                        *work.offset(
                            (ismin as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                - 1 as ::core::ffi::c_long) as isize,
                        ) = s1
                            * *work.offset(
                                (ismin as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                    - 1 as ::core::ffi::c_long)
                                    as isize,
                            );
                        *work.offset(
                            (ismax as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                - 1 as ::core::ffi::c_long) as isize,
                        ) = s2
                            * *work.offset(
                                (ismax as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                    - 1 as ::core::ffi::c_long)
                                    as isize,
                            );
                        i__ += 1;
                    }
                    *work.offset((ismin + *rank) as isize) = c1;
                    *work.offset((ismax + *rank) as isize) = c2;
                    smin = sminpr;
                    smax = smaxpr;
                    *rank += 1;
                }
                if *rank < *n {
                    i__1 = *lwork - (mn << 1 as ::core::ffi::c_int);
                    slsqp_closure_dtzrzf__0(
                        rank,
                        n,
                        a.offset(a_offset as isize) as *mut doublereal,
                        lda,
                        work.offset((mn as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                            as *mut doublereal,
                        work.offset(
                            (((mn as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut i__1,
                        info,
                    );
                }
                i__1 = *lwork - (mn << 1 as ::core::ffi::c_int);
                dgeev_closure_dormqr__0(
                    b"Left\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    m,
                    nrhs,
                    &raw mut mn,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    b.offset(b_offset as isize) as *mut doublereal,
                    ldb,
                    work.offset(
                        (((mn as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut i__1,
                    info,
                );
                d__1 = wsize;
                d__2 = (mn << 1 as ::core::ffi::c_int) as doublereal
                    + *work.offset(
                        (((mn as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                wsize = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                slsqp_closure_dtrsm__0(
                    b"Left\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"Upper\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"Non-unit\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    rank,
                    nrhs,
                    &raw mut c_b54,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    b.offset(b_offset as isize) as *mut doublereal,
                    ldb,
                );
                i__1 = *nrhs;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = *n;
                    i__ = (*rank + 1 as ::core::ffi::c_long) as integer;
                    while i__ <= i__2 {
                        *b.offset((i__ + j * b_dim1) as isize) = 0.0f64 as doublereal;
                        i__ += 1;
                    }
                    j += 1;
                }
                if *rank < *n {
                    i__1 = *n - *rank;
                    i__2 = *lwork - (mn << 1 as ::core::ffi::c_int);
                    slsqp_closure_dormrz__0(
                        b"Left\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        n,
                        nrhs,
                        rank,
                        &raw mut i__1,
                        a.offset(a_offset as isize) as *mut doublereal,
                        lda,
                        work.offset((mn as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                            as *mut doublereal,
                        b.offset(b_offset as isize) as *mut doublereal,
                        ldb,
                        work.offset(
                            (((mn as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut i__2,
                        info,
                    );
                }
                i__1 = *nrhs;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = *n;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *work.offset(*jpvt.offset(i__ as isize) as isize) =
                            *b.offset((i__ + j * b_dim1) as isize);
                        i__ += 1;
                    }
                    dgelsd_closure_f2c_dcopy_0(
                        n,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        &raw mut c__1,
                        b.offset(
                            (j as ::core::ffi::c_long * b_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    j += 1;
                }
                if iascl == 1 as ::core::ffi::c_long {
                    dgelsd_closure_dlascl__0(
                        b"G\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__0,
                        &raw mut c__0,
                        &raw mut anrm,
                        &raw mut smlnum,
                        n,
                        nrhs,
                        b.offset(b_offset as isize) as *mut doublereal,
                        ldb,
                        info,
                    );
                    dgelsd_closure_dlascl__0(
                        b"U\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__0,
                        &raw mut c__0,
                        &raw mut smlnum,
                        &raw mut anrm,
                        rank,
                        rank,
                        a.offset(a_offset as isize) as *mut doublereal,
                        lda,
                        info,
                    );
                } else if iascl == 2 as ::core::ffi::c_long {
                    dgelsd_closure_dlascl__0(
                        b"G\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__0,
                        &raw mut c__0,
                        &raw mut anrm,
                        &raw mut bignum,
                        n,
                        nrhs,
                        b.offset(b_offset as isize) as *mut doublereal,
                        ldb,
                        info,
                    );
                    dgelsd_closure_dlascl__0(
                        b"U\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__0,
                        &raw mut c__0,
                        &raw mut bignum,
                        &raw mut anrm,
                        rank,
                        rank,
                        a.offset(a_offset as isize) as *mut doublereal,
                        lda,
                        info,
                    );
                }
                if ibscl == 1 as ::core::ffi::c_long {
                    dgelsd_closure_dlascl__0(
                        b"G\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__0,
                        &raw mut c__0,
                        &raw mut smlnum,
                        &raw mut bnrm,
                        n,
                        nrhs,
                        b.offset(b_offset as isize) as *mut doublereal,
                        ldb,
                        info,
                    );
                } else if ibscl == 2 as ::core::ffi::c_long {
                    dgelsd_closure_dlascl__0(
                        b"G\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__0,
                        &raw mut c__0,
                        &raw mut bignum,
                        &raw mut bnrm,
                        n,
                        nrhs,
                        b.offset(b_offset as isize) as *mut doublereal,
                        ldb,
                        info,
                    );
                }
            }
        }
        _ => {}
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    return 0 as ::core::ffi::c_int;
}
