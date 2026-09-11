pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
static mut c__1: integer = 1 as integer;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
static mut c__3: integer = 3 as integer;
static mut c__2: integer = 2 as integer;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dgeqp3_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut jpvt: *mut integer,
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
    let mut j: integer = 0;
    let mut jb: integer = 0;
    let mut na: integer = 0;
    let mut nb: integer = 0;
    let mut sm: integer = 0;
    let mut sn: integer = 0;
    let mut nx: integer = 0;
    let mut fjb: integer = 0;
    let mut iws: integer = 0;
    let mut nfxd: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dnrm2"]
        fn dgelsd_closure_f2c_dnrm2_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> doublereal;
    }
    let mut nbmin: integer = 0;
    let mut minmn: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dswap"]
        fn dgelsd_closure_f2c_dswap_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut minws: integer = 0;
    extern "C" {
        #[link_name = "slsqp_closure_dlaqp2_"]
        fn slsqp_closure_dlaqp2__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgesdd_closure_dgeqrf_"]
        fn dgesdd_closure_dgeqrf__0(
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
    extern "C" {
        #[link_name = "slsqp_closure_dlaqps_"]
        fn slsqp_closure_dlaqps__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut topbmn: integer = 0;
    let mut sminmn: integer = 0;
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
    let mut lwkopt: integer = 0;
    let mut lquery: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    jpvt = jpvt.offset(-1);
    tau = tau.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
    if *m < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        *info = -(4 as ::core::ffi::c_int) as integer;
    }
    if *info == 0 as ::core::ffi::c_long {
        minmn = (if *m <= *n { *m } else { *n }) as integer;
        if minmn == 0 as ::core::ffi::c_long {
            iws = 1 as integer;
            lwkopt = 1 as integer;
        } else {
            iws = (*n * 3 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            nb = dgelsd_closure_ilaenv__0(
                &raw mut c__1,
                b"DGEQRF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                &raw mut c_n1,
                &raw mut c_n1,
            );
            lwkopt = (*n << 1 as ::core::ffi::c_int) + (*n + 1 as integer) * nb;
        }
        *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
        if *lwork < iws && lquery == 0 {
            *info = -(8 as ::core::ffi::c_int) as integer;
        }
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        dgelsd_closure_xerbla__0(
            b"DGEQP3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    nfxd = 1 as integer;
    i__1 = *n;
    j = 1 as integer;
    while j <= i__1 {
        if *jpvt.offset(j as isize) != 0 as ::core::ffi::c_long {
            if j != nfxd {
                dgelsd_closure_f2c_dswap_0(
                    m,
                    a.offset(
                        (j as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    a.offset(
                        (nfxd as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                *jpvt.offset(j as isize) = *jpvt.offset(nfxd as isize);
                *jpvt.offset(nfxd as isize) = j;
            } else {
                *jpvt.offset(j as isize) = j;
            }
            nfxd += 1;
        } else {
            *jpvt.offset(j as isize) = j;
        }
        j += 1;
    }
    nfxd -= 1;
    if nfxd > 0 as ::core::ffi::c_long {
        na = (if *m <= nfxd {
            *m
        } else {
            nfxd as ::core::ffi::c_long
        }) as integer;
        dgesdd_closure_dgeqrf__0(
            m,
            &raw mut na,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            lwork,
            info,
        );
        i__1 = iws;
        i__2 = *work.offset(1 as ::core::ffi::c_int as isize) as integer;
        iws = (if i__1 >= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        if na < *n {
            i__1 = *n - na;
            dgeev_closure_dormqr__0(
                b"Left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                m,
                &raw mut i__1,
                &raw mut na,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                a.offset(
                    ((na as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                        * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                lda,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                lwork,
                info,
            );
            i__1 = iws;
            i__2 = *work.offset(1 as ::core::ffi::c_int as isize) as integer;
            iws = (if i__1 >= i__2 {
                i__1 as ::core::ffi::c_long
            } else {
                i__2 as ::core::ffi::c_long
            }) as integer;
        }
    }
    if nfxd < minmn {
        sm = *m - nfxd;
        sn = *n - nfxd;
        sminmn = minmn - nfxd;
        nb = dgelsd_closure_ilaenv__0(
            &raw mut c__1,
            b"DGEQRF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut sm,
            &raw mut sn,
            &raw mut c_n1,
            &raw mut c_n1,
        );
        nbmin = 2 as integer;
        nx = 0 as integer;
        if nb > 1 as ::core::ffi::c_long && nb < sminmn {
            i__1 = 0 as integer;
            i__2 = dgelsd_closure_ilaenv__0(
                &raw mut c__3,
                b"DGEQRF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut sm,
                &raw mut sn,
                &raw mut c_n1,
                &raw mut c_n1,
            );
            nx = (if i__1 >= i__2 {
                i__1 as ::core::ffi::c_long
            } else {
                i__2 as ::core::ffi::c_long
            }) as integer;
            if nx < sminmn {
                minws = (sn << 1 as ::core::ffi::c_int) + (sn + 1 as integer) * nb;
                iws = (if iws >= minws {
                    iws as ::core::ffi::c_long
                } else {
                    minws as ::core::ffi::c_long
                }) as integer;
                if *lwork < minws {
                    nb = ((*lwork - ((sn as ::core::ffi::c_long) << 1 as ::core::ffi::c_int))
                        / (sn as ::core::ffi::c_long + 1 as ::core::ffi::c_long))
                        as integer;
                    i__1 = 2 as integer;
                    i__2 = dgelsd_closure_ilaenv__0(
                        &raw mut c__2,
                        b"DGEQRF\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b" \0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut sm,
                        &raw mut sn,
                        &raw mut c_n1,
                        &raw mut c_n1,
                    );
                    nbmin = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                }
            }
        }
        i__1 = *n;
        j = (nfxd as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        while j <= i__1 {
            *work.offset(j as isize) = dgelsd_closure_f2c_dnrm2_0(
                &raw mut sm,
                a.offset((nfxd + 1 as integer + j * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
            *work.offset((*n + j) as isize) = *work.offset(j as isize);
            j += 1;
        }
        if nb >= nbmin && nb < sminmn && nx < sminmn {
            j = (nfxd as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            topbmn = minmn - nx;
            while j <= topbmn {
                i__1 = nb;
                i__2 = (topbmn as ::core::ffi::c_long - j as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                jb = (if i__1 <= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
                i__1 = (*n - j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                i__3 = (*n - j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                slsqp_closure_dlaqps__0(
                    m,
                    &raw mut i__1,
                    &raw mut i__2,
                    &raw mut jb,
                    &raw mut fjb,
                    a.offset(
                        (j as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    lda,
                    jpvt.offset(j as isize) as *mut integer,
                    tau.offset(j as isize) as *mut doublereal,
                    work.offset(j as isize) as *mut doublereal,
                    work.offset((*n + j) as isize) as *mut doublereal,
                    work.offset(
                        ((*n << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    work.offset(
                        ((*n << 1 as ::core::ffi::c_int)
                            + jb as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut i__3,
                );
                j += fjb as ::core::ffi::c_long;
            }
        } else {
            j = (nfxd as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        }
        if j <= minmn {
            i__1 = (*n - j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            slsqp_closure_dlaqp2__0(
                m,
                &raw mut i__1,
                &raw mut i__2,
                a.offset(
                    (j as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                lda,
                jpvt.offset(j as isize) as *mut integer,
                tau.offset(j as isize) as *mut doublereal,
                work.offset(j as isize) as *mut doublereal,
                work.offset((*n + j) as isize) as *mut doublereal,
                work.offset(((*n << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
            );
        }
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = iws as doublereal;
    return 0 as ::core::ffi::c_int;
}
