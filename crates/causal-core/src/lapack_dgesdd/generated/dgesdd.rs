#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sqrt(_: doublereal) -> ::core::ffi::c_double;
}
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
static mut c__1: integer = 1 as integer;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
static mut c__0: integer = 0 as integer;
static mut c_b227: doublereal = 0.0f64;
static mut c_b248: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dgesdd_(
    mut jobz: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut s: *mut doublereal,
    mut u: *mut doublereal,
    mut ldu: *mut integer,
    mut vt: *mut doublereal,
    mut ldvt: *mut integer,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut iwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut u_dim1: integer = 0;
    let mut u_offset: integer = 0;
    let mut vt_dim1: integer = 0;
    let mut vt_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
    let mut ie: integer = 0;
    let mut il: integer = 0;
    let mut ir: integer = 0;
    let mut iu: integer = 0;
    let mut blk: integer = 0;
    let mut dum: [doublereal; 1] = [0.; 1];
    let mut eps: doublereal = 0.;
    let mut ivt: integer = 0;
    let mut iscl: integer = 0;
    let mut anrm: doublereal = 0.;
    let mut idum: [integer; 1] = [0; 1];
    let mut ierr: integer = 0;
    let mut itau: integer = 0;
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
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut chunk: integer = 0;
    let mut minmn: integer = 0;
    let mut wrkbl: integer = 0;
    let mut itaup: integer = 0;
    let mut itauq: integer = 0;
    let mut mnthr: integer = 0;
    let mut wntqa: logical = 0;
    let mut nwork: integer = 0;
    let mut wntqn: logical = 0;
    let mut wntqo: logical = 0;
    let mut wntqs: logical = 0;
    extern "C" {
        #[link_name = "dgesdd_closure_dbdsdc_"]
        fn dbdsdc__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgesdd_closure_dgebrd_"]
        fn dgebrd__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgesdd_closure_dlange_"]
        fn dlange__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> doublereal;
    }
    let mut bdspac: integer = 0;
    extern "C" {
        #[link_name = "dgesdd_closure_dgelqf_"]
        fn dgelqf__0(
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
        #[link_name = "dgelsd_closure_dlascl_"]
        fn dlascl__0(
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
        #[link_name = "dgesdd_closure_dgeqrf_"]
        fn dgeqrf__0(
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
        #[link_name = "dgelsd_closure_dlacpy_"]
        fn dlacpy__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlaset_"]
        fn dlaset__0(
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
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgesdd_closure_dorgbr_"]
        fn dorgbr__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
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
    let mut bignum: doublereal = 0.;
    extern "C" {
        #[link_name = "dgesdd_closure_dormbr_"]
        fn dormbr__0(
            _: *mut ::core::ffi::c_char,
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
    extern "C" {
        #[link_name = "dgesdd_closure_dorglq_"]
        fn dorglq__0(
            _: *mut integer,
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
        #[link_name = "dgesdd_closure_dorgqr_"]
        fn dorgqr__0(
            _: *mut integer,
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
    let mut ldwrkl: integer = 0;
    let mut ldwrkr: integer = 0;
    let mut minwrk: integer = 0;
    let mut ldwrku: integer = 0;
    let mut maxwrk: integer = 0;
    let mut ldwkvt: integer = 0;
    let mut smlnum: doublereal = 0.;
    let mut wntqas: logical = 0;
    let mut lquery: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    s = s.offset(-1);
    u_dim1 = *ldu;
    u_offset = 1 as integer + u_dim1;
    u = u.offset(-(u_offset as isize));
    vt_dim1 = *ldvt;
    vt_offset = 1 as integer + vt_dim1;
    vt = vt.offset(-(vt_offset as isize));
    work = work.offset(-1);
    iwork = iwork.offset(-1);
    *info = 0 as integer;
    minmn = (if *m <= *n { *m } else { *n }) as integer;
    wntqa = lsame__0(
        jobz,
        b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    wntqs = lsame__0(
        jobz,
        b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    wntqas = (wntqa != 0 || wntqs != 0) as ::core::ffi::c_int as logical;
    wntqo = lsame__0(
        jobz,
        b"O\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    wntqn = lsame__0(
        jobz,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
    if !(wntqa != 0 || wntqs != 0 || wntqo != 0 || wntqn != 0) {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *m < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        *info = -(5 as ::core::ffi::c_int) as integer;
    } else if *ldu < 1 as ::core::ffi::c_long
        || wntqas != 0 && *ldu < *m
        || wntqo != 0 && *m < *n && *ldu < *m
    {
        *info = -(8 as ::core::ffi::c_int) as integer;
    } else if *ldvt < 1 as ::core::ffi::c_long
        || wntqa != 0 && *ldvt < *n
        || wntqs != 0 && *ldvt < minmn
        || wntqo != 0 && *m >= *n && *ldvt < *n
    {
        *info = -(10 as ::core::ffi::c_int) as integer;
    }
    if *info == 0 as ::core::ffi::c_long {
        minwrk = 1 as integer;
        maxwrk = 1 as integer;
        if *m >= *n && minmn > 0 as ::core::ffi::c_long {
            mnthr = (minmn as ::core::ffi::c_double * 11.0f64 / 6.0f64) as integer;
            if wntqn != 0 {
                bdspac = (*n * 7 as ::core::ffi::c_long) as integer;
            } else {
                bdspac = *n * 3 as integer * *n + (*n << 2 as ::core::ffi::c_int);
            }
            if *m >= mnthr {
                if wntqn != 0 {
                    wrkbl = *n
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DGEQRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + (*n << 1 as ::core::ffi::c_int)
                            * ilaenv__0(
                                &raw mut c__1,
                                b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b" \0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                n,
                                n,
                                &raw mut c_n1,
                                &raw mut c_n1,
                            );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = bdspac + *n;
                    maxwrk = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    minwrk = bdspac + *n;
                } else if wntqo != 0 {
                    wrkbl = *n
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DGEQRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                    i__1 = wrkbl;
                    i__2 = *n
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORGQR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + (*n << 1 as ::core::ffi::c_int)
                            * ilaenv__0(
                                &raw mut c__1,
                                b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b" \0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                n,
                                n,
                                &raw mut c_n1,
                                &raw mut c_n1,
                            );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *n * 3 as ::core::ffi::c_long) as integer;
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    maxwrk = wrkbl + (*n << 1 as ::core::ffi::c_int) * *n;
                    minwrk = (bdspac as ::core::ffi::c_long
                        + (*n << 1 as ::core::ffi::c_int) * *n
                        + *n * 3 as ::core::ffi::c_long) as integer;
                } else if wntqs != 0 {
                    wrkbl = *n
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DGEQRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                    i__1 = wrkbl;
                    i__2 = *n
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORGQR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + (*n << 1 as ::core::ffi::c_int)
                            * ilaenv__0(
                                &raw mut c__1,
                                b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b" \0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                n,
                                n,
                                &raw mut c_n1,
                                &raw mut c_n1,
                            );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *n * 3 as ::core::ffi::c_long) as integer;
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    maxwrk = wrkbl + *n * *n;
                    minwrk = (bdspac as ::core::ffi::c_long
                        + *n * *n
                        + *n * 3 as ::core::ffi::c_long) as integer;
                } else if wntqa != 0 {
                    wrkbl = *n
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DGEQRF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                    i__1 = wrkbl;
                    i__2 = *n
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORGQR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + (*n << 1 as ::core::ffi::c_int)
                            * ilaenv__0(
                                &raw mut c__1,
                                b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b" \0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                n,
                                n,
                                &raw mut c_n1,
                                &raw mut c_n1,
                            );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *n * 3 as ::core::ffi::c_long) as integer;
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    maxwrk = wrkbl + *n * *n;
                    minwrk = (bdspac as ::core::ffi::c_long
                        + *n * *n
                        + *n * 3 as ::core::ffi::c_long) as integer;
                }
            } else {
                wrkbl = *n * 3 as integer
                    + (*m + *n)
                        * ilaenv__0(
                            &raw mut c__1,
                            b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                if wntqn != 0 {
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *n * 3 as ::core::ffi::c_long) as integer;
                    maxwrk = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    minwrk = (*n * 3 as ::core::ffi::c_long
                        + (if *m >= bdspac {
                            *m
                        } else {
                            bdspac as ::core::ffi::c_long
                        })) as integer;
                } else if wntqo != 0 {
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *n * 3 as ::core::ffi::c_long) as integer;
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    maxwrk = wrkbl + *m * *n;
                    i__1 = *m;
                    i__2 = *n * *n + bdspac;
                    minwrk = (*n * 3 as ::core::ffi::c_long
                        + (if i__1 >= i__2 {
                            i__1 as ::core::ffi::c_long
                        } else {
                            i__2 as ::core::ffi::c_long
                        })) as integer;
                } else if wntqs != 0 {
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *n * 3 as ::core::ffi::c_long) as integer;
                    maxwrk = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    minwrk = (*n * 3 as ::core::ffi::c_long
                        + (if *m >= bdspac {
                            *m
                        } else {
                            bdspac as ::core::ffi::c_long
                        })) as integer;
                } else if wntqa != 0 {
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *n * 3 as integer
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = maxwrk;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *n * 3 as ::core::ffi::c_long) as integer;
                    maxwrk = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    minwrk = (*n * 3 as ::core::ffi::c_long
                        + (if *m >= bdspac {
                            *m
                        } else {
                            bdspac as ::core::ffi::c_long
                        })) as integer;
                }
            }
        } else if minmn > 0 as ::core::ffi::c_long {
            mnthr = (minmn as ::core::ffi::c_double * 11.0f64 / 6.0f64) as integer;
            if wntqn != 0 {
                bdspac = (*m * 7 as ::core::ffi::c_long) as integer;
            } else {
                bdspac = *m * 3 as integer * *m + (*m << 2 as ::core::ffi::c_int);
            }
            if *n >= mnthr {
                if wntqn != 0 {
                    wrkbl = *m
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DGELQF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + (*m << 1 as ::core::ffi::c_int)
                            * ilaenv__0(
                                &raw mut c__1,
                                b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b" \0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                m,
                                m,
                                &raw mut c_n1,
                                &raw mut c_n1,
                            );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = bdspac + *m;
                    maxwrk = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    minwrk = bdspac + *m;
                } else if wntqo != 0 {
                    wrkbl = *m
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DGELQF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                    i__1 = wrkbl;
                    i__2 = *m
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORGLQ\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + (*m << 1 as ::core::ffi::c_int)
                            * ilaenv__0(
                                &raw mut c__1,
                                b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b" \0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                m,
                                m,
                                &raw mut c_n1,
                                &raw mut c_n1,
                            );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *m * 3 as ::core::ffi::c_long) as integer;
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    maxwrk = wrkbl + (*m << 1 as ::core::ffi::c_int) * *m;
                    minwrk = (bdspac as ::core::ffi::c_long
                        + (*m << 1 as ::core::ffi::c_int) * *m
                        + *m * 3 as ::core::ffi::c_long) as integer;
                } else if wntqs != 0 {
                    wrkbl = *m
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DGELQF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                    i__1 = wrkbl;
                    i__2 = *m
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORGLQ\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + (*m << 1 as ::core::ffi::c_int)
                            * ilaenv__0(
                                &raw mut c__1,
                                b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b" \0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                m,
                                m,
                                &raw mut c_n1,
                                &raw mut c_n1,
                            );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *m * 3 as ::core::ffi::c_long) as integer;
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    maxwrk = wrkbl + *m * *m;
                    minwrk = (bdspac as ::core::ffi::c_long
                        + *m * *m
                        + *m * 3 as ::core::ffi::c_long) as integer;
                } else if wntqa != 0 {
                    wrkbl = *m
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DGELQF\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                    i__1 = wrkbl;
                    i__2 = *m
                        + *n * ilaenv__0(
                            &raw mut c__1,
                            b"DORGLQ\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + (*m << 1 as ::core::ffi::c_int)
                            * ilaenv__0(
                                &raw mut c__1,
                                b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b" \0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                m,
                                m,
                                &raw mut c_n1,
                                &raw mut c_n1,
                            );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *m * 3 as ::core::ffi::c_long) as integer;
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    maxwrk = wrkbl + *m * *m;
                    minwrk = (bdspac as ::core::ffi::c_long
                        + *m * *m
                        + *m * 3 as ::core::ffi::c_long) as integer;
                }
            } else {
                wrkbl = *m * 3 as integer
                    + (*m + *n)
                        * ilaenv__0(
                            &raw mut c__1,
                            b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            &raw mut c_n1,
                            &raw mut c_n1,
                        );
                if wntqn != 0 {
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *m * 3 as ::core::ffi::c_long) as integer;
                    maxwrk = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    minwrk = (*m * 3 as ::core::ffi::c_long
                        + (if *n >= bdspac {
                            *n
                        } else {
                            bdspac as ::core::ffi::c_long
                        })) as integer;
                } else if wntqo != 0 {
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *m * 3 as ::core::ffi::c_long) as integer;
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    maxwrk = wrkbl + *m * *n;
                    i__1 = *n;
                    i__2 = *m * *m + bdspac;
                    minwrk = (*m * 3 as ::core::ffi::c_long
                        + (if i__1 >= i__2 {
                            i__1 as ::core::ffi::c_long
                        } else {
                            i__2 as ::core::ffi::c_long
                        })) as integer;
                } else if wntqs != 0 {
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            n,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *m * 3 as ::core::ffi::c_long) as integer;
                    maxwrk = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    minwrk = (*m * 3 as ::core::ffi::c_long
                        + (if *n >= bdspac {
                            *n
                        } else {
                            bdspac as ::core::ffi::c_long
                        })) as integer;
                } else if wntqa != 0 {
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"QLN\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            m,
                            m,
                            n,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 = *m * 3 as integer
                        + *m * ilaenv__0(
                            &raw mut c__1,
                            b"DORMBR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"PRT\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            m,
                            &raw mut c_n1,
                        );
                    wrkbl = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    i__1 = wrkbl;
                    i__2 =
                        (bdspac as ::core::ffi::c_long + *m * 3 as ::core::ffi::c_long) as integer;
                    maxwrk = (if i__1 >= i__2 {
                        i__1 as ::core::ffi::c_long
                    } else {
                        i__2 as ::core::ffi::c_long
                    }) as integer;
                    minwrk = (*m * 3 as ::core::ffi::c_long
                        + (if *n >= bdspac {
                            *n
                        } else {
                            bdspac as ::core::ffi::c_long
                        })) as integer;
                }
            }
        }
        maxwrk = (if maxwrk >= minwrk {
            maxwrk as ::core::ffi::c_long
        } else {
            minwrk as ::core::ffi::c_long
        }) as integer;
        *work.offset(1 as ::core::ffi::c_int as isize) = maxwrk as doublereal;
        if *lwork < minwrk && lquery == 0 {
            *info = -(12 as ::core::ffi::c_int) as integer;
        }
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DGESDD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *m == 0 as ::core::ffi::c_long || *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    eps = dlamch__0(b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char);
    smlnum = sqrt(dlamch__0(
        b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    )) as doublereal
        / eps;
    bignum = 1.0f64 / smlnum;
    anrm = dlange__0(
        b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        m,
        n,
        a.offset(a_offset as isize) as *mut doublereal,
        lda,
        &raw mut dum as *mut doublereal,
    );
    iscl = 0 as integer;
    if anrm > 0.0f64 && anrm < smlnum {
        iscl = 1 as integer;
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut anrm,
            &raw mut smlnum,
            m,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            &raw mut ierr,
        );
    } else if anrm > bignum {
        iscl = 1 as integer;
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut anrm,
            &raw mut bignum,
            m,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            &raw mut ierr,
        );
    }
    if *m >= *n {
        if *m >= mnthr {
            if wntqn != 0 {
                itau = 1 as integer;
                nwork = itau + *n;
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dgeqrf__0(
                    m,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itau as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                i__2 = (*n - 1 as ::core::ffi::c_long) as integer;
                dlaset__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut i__1,
                    &raw mut i__2,
                    &raw mut c_b227,
                    &raw mut c_b227,
                    a.offset((a_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                    lda,
                );
                ie = 1 as integer;
                itauq = ie + *n;
                itaup = itauq + *n;
                nwork = itaup + *n;
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dgebrd__0(
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    work.offset(itauq as isize) as *mut doublereal,
                    work.offset(itaup as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
                nwork = ie + *n;
                dbdsdc__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    &raw mut dum as *mut doublereal,
                    &raw mut c__1,
                    &raw mut dum as *mut doublereal,
                    &raw mut c__1,
                    &raw mut dum as *mut doublereal,
                    &raw mut idum as *mut integer,
                    work.offset(nwork as isize) as *mut doublereal,
                    iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                    info,
                );
            } else if wntqo != 0 {
                ir = 1 as integer;
                if *lwork >= *lda * *n + *n * *n + *n * 3 as integer + bdspac {
                    ldwrkr = *lda;
                } else {
                    ldwrkr = (*lwork - *n * *n - *n * 3 as integer - bdspac) / *n;
                }
                itau = ir + ldwrkr * *n;
                nwork = itau + *n;
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dgeqrf__0(
                    m,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itau as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
                dlacpy__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                );
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                i__2 = (*n - 1 as ::core::ffi::c_long) as integer;
                dlaset__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut i__1,
                    &raw mut i__2,
                    &raw mut c_b227,
                    &raw mut c_b227,
                    work.offset((ir as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                    &raw mut ldwrkr,
                );
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dorgqr__0(
                    m,
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itau as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
                ie = itau;
                itauq = ie + *n;
                itaup = itauq + *n;
                nwork = itaup + *n;
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dgebrd__0(
                    n,
                    n,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    work.offset(itauq as isize) as *mut doublereal,
                    work.offset(itaup as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
                iu = nwork;
                nwork = iu + *n * *n;
                dbdsdc__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    work.offset(iu as isize) as *mut doublereal,
                    n,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    &raw mut dum as *mut doublereal,
                    &raw mut idum as *mut integer,
                    work.offset(nwork as isize) as *mut doublereal,
                    iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                    info,
                );
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    n,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                    work.offset(itauq as isize) as *mut doublereal,
                    work.offset(iu as isize) as *mut doublereal,
                    n,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    n,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                    work.offset(itaup as isize) as *mut doublereal,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
                i__1 = *m;
                i__2 = ldwrkr;
                i__ = 1 as integer;
                while if i__2 < 0 as ::core::ffi::c_long {
                    (i__ >= i__1) as ::core::ffi::c_int
                } else {
                    (i__ <= i__1) as ::core::ffi::c_int
                } != 0
                {
                    i__3 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    chunk = (if i__3 <= ldwrkr {
                        i__3 as ::core::ffi::c_long
                    } else {
                        ldwrkr as ::core::ffi::c_long
                    }) as integer;
                    f2c_dgemm_0(
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut chunk,
                        n,
                        n,
                        &raw mut c_b248,
                        a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                        lda,
                        work.offset(iu as isize) as *mut doublereal,
                        n,
                        &raw mut c_b227,
                        work.offset(ir as isize) as *mut doublereal,
                        &raw mut ldwrkr,
                    );
                    dlacpy__0(
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut chunk,
                        n,
                        work.offset(ir as isize) as *mut doublereal,
                        &raw mut ldwrkr,
                        a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                        lda,
                    );
                    i__ += i__2 as ::core::ffi::c_long;
                }
            } else if wntqs != 0 {
                ir = 1 as integer;
                ldwrkr = *n;
                itau = ir + ldwrkr * *n;
                nwork = itau + *n;
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dgeqrf__0(
                    m,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itau as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                dlacpy__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                );
                i__2 = (*n - 1 as ::core::ffi::c_long) as integer;
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                dlaset__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__1,
                    &raw mut c_b227,
                    &raw mut c_b227,
                    work.offset((ir as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                    &raw mut ldwrkr,
                );
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dorgqr__0(
                    m,
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itau as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                ie = itau;
                itauq = ie + *n;
                itaup = itauq + *n;
                nwork = itaup + *n;
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dgebrd__0(
                    n,
                    n,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    work.offset(itauq as isize) as *mut doublereal,
                    work.offset(itaup as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                dbdsdc__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    &raw mut dum as *mut doublereal,
                    &raw mut idum as *mut integer,
                    work.offset(nwork as isize) as *mut doublereal,
                    iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                    info,
                );
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    n,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                    work.offset(itauq as isize) as *mut doublereal,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    n,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                    work.offset(itaup as isize) as *mut doublereal,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                dlacpy__0(
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                );
                f2c_dgemm_0(
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    n,
                    &raw mut c_b248,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(ir as isize) as *mut doublereal,
                    &raw mut ldwrkr,
                    &raw mut c_b227,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                );
            } else if wntqa != 0 {
                iu = 1 as integer;
                ldwrku = *n;
                itau = iu + ldwrku * *n;
                nwork = itau + *n;
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dgeqrf__0(
                    m,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itau as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                dlacpy__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                );
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dorgqr__0(
                    m,
                    m,
                    n,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    work.offset(itau as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                i__2 = (*n - 1 as ::core::ffi::c_long) as integer;
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                dlaset__0(
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__1,
                    &raw mut c_b227,
                    &raw mut c_b227,
                    a.offset((a_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                    lda,
                );
                ie = itau;
                itauq = ie + *n;
                itaup = itauq + *n;
                nwork = itaup + *n;
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dgebrd__0(
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    work.offset(itauq as isize) as *mut doublereal,
                    work.offset(itaup as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                dbdsdc__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    work.offset(iu as isize) as *mut doublereal,
                    n,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    &raw mut dum as *mut doublereal,
                    &raw mut idum as *mut integer,
                    work.offset(nwork as isize) as *mut doublereal,
                    iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                    info,
                );
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itauq as isize) as *mut doublereal,
                    work.offset(iu as isize) as *mut doublereal,
                    &raw mut ldwrku,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itaup as isize) as *mut doublereal,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                f2c_dgemm_0(
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    n,
                    &raw mut c_b248,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    work.offset(iu as isize) as *mut doublereal,
                    &raw mut ldwrku,
                    &raw mut c_b227,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                );
                dlacpy__0(
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                );
            }
        } else {
            ie = 1 as integer;
            itauq = ie + *n;
            itaup = itauq + *n;
            nwork = itaup + *n;
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgebrd__0(
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                work.offset(itauq as isize) as *mut doublereal,
                work.offset(itaup as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            if wntqn != 0 {
                dbdsdc__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    &raw mut dum as *mut doublereal,
                    &raw mut c__1,
                    &raw mut dum as *mut doublereal,
                    &raw mut c__1,
                    &raw mut dum as *mut doublereal,
                    &raw mut idum as *mut integer,
                    work.offset(nwork as isize) as *mut doublereal,
                    iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                    info,
                );
            } else if wntqo != 0 {
                iu = nwork;
                if *lwork >= *m * *n + *n * 3 as integer + bdspac {
                    ldwrku = *m;
                    nwork = iu + ldwrku * *n;
                    dlaset__0(
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        m,
                        n,
                        &raw mut c_b227,
                        &raw mut c_b227,
                        work.offset(iu as isize) as *mut doublereal,
                        &raw mut ldwrku,
                    );
                } else {
                    ldwrku = *n;
                    nwork = iu + ldwrku * *n;
                    ir = nwork;
                    ldwrkr = (*lwork - *n * *n - *n * 3 as integer) / *n;
                }
                nwork = iu + ldwrku * *n;
                dbdsdc__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    work.offset(iu as isize) as *mut doublereal,
                    &raw mut ldwrku,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    &raw mut dum as *mut doublereal,
                    &raw mut idum as *mut integer,
                    work.offset(nwork as isize) as *mut doublereal,
                    iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                    info,
                );
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itaup as isize) as *mut doublereal,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                if *lwork >= *m * *n + *n * 3 as integer + bdspac {
                    i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                        as integer;
                    dormbr__0(
                        b"Q\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        m,
                        n,
                        n,
                        a.offset(a_offset as isize) as *mut doublereal,
                        lda,
                        work.offset(itauq as isize) as *mut doublereal,
                        work.offset(iu as isize) as *mut doublereal,
                        &raw mut ldwrku,
                        work.offset(nwork as isize) as *mut doublereal,
                        &raw mut i__2,
                        &raw mut ierr,
                    );
                    dlacpy__0(
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        m,
                        n,
                        work.offset(iu as isize) as *mut doublereal,
                        &raw mut ldwrku,
                        a.offset(a_offset as isize) as *mut doublereal,
                        lda,
                    );
                } else {
                    i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                        as integer;
                    dorgbr__0(
                        b"Q\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        m,
                        n,
                        n,
                        a.offset(a_offset as isize) as *mut doublereal,
                        lda,
                        work.offset(itauq as isize) as *mut doublereal,
                        work.offset(nwork as isize) as *mut doublereal,
                        &raw mut i__2,
                        &raw mut ierr,
                    );
                    i__2 = *m;
                    i__1 = ldwrkr;
                    i__ = 1 as integer;
                    while if i__1 < 0 as ::core::ffi::c_long {
                        (i__ >= i__2) as ::core::ffi::c_int
                    } else {
                        (i__ <= i__2) as ::core::ffi::c_int
                    } != 0
                    {
                        i__3 =
                            (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        chunk = (if i__3 <= ldwrkr {
                            i__3 as ::core::ffi::c_long
                        } else {
                            ldwrkr as ::core::ffi::c_long
                        }) as integer;
                        f2c_dgemm_0(
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut chunk,
                            n,
                            n,
                            &raw mut c_b248,
                            a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                            lda,
                            work.offset(iu as isize) as *mut doublereal,
                            &raw mut ldwrku,
                            &raw mut c_b227,
                            work.offset(ir as isize) as *mut doublereal,
                            &raw mut ldwrkr,
                        );
                        dlacpy__0(
                            b"F\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut chunk,
                            n,
                            work.offset(ir as isize) as *mut doublereal,
                            &raw mut ldwrkr,
                            a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                            lda,
                        );
                        i__ += i__1 as ::core::ffi::c_long;
                    }
                }
            } else if wntqs != 0 {
                dlaset__0(
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    &raw mut c_b227,
                    &raw mut c_b227,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                );
                dbdsdc__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    &raw mut dum as *mut doublereal,
                    &raw mut idum as *mut integer,
                    work.offset(nwork as isize) as *mut doublereal,
                    iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                    info,
                );
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itauq as isize) as *mut doublereal,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itaup as isize) as *mut doublereal,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
            } else if wntqa != 0 {
                dlaset__0(
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    m,
                    &raw mut c_b227,
                    &raw mut c_b227,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                );
                dbdsdc__0(
                    b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(ie as isize) as *mut doublereal,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    &raw mut dum as *mut doublereal,
                    &raw mut idum as *mut integer,
                    work.offset(nwork as isize) as *mut doublereal,
                    iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                    info,
                );
                if *m > *n {
                    i__1 = *m - *n;
                    i__2 = *m - *n;
                    dlaset__0(
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut i__1,
                        &raw mut i__2,
                        &raw mut c_b227,
                        &raw mut c_b248,
                        u.offset((*n + 1 as integer + (*n + 1 as integer) * u_dim1) as isize)
                            as *mut doublereal,
                        ldu,
                    );
                }
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    m,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itauq as isize) as *mut doublereal,
                    u.offset(u_offset as isize) as *mut doublereal,
                    ldu,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
                i__1 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    n,
                    m,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itaup as isize) as *mut doublereal,
                    vt.offset(vt_offset as isize) as *mut doublereal,
                    ldvt,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__1,
                    &raw mut ierr,
                );
            }
        }
    } else if *n >= mnthr {
        if wntqn != 0 {
            itau = 1 as integer;
            nwork = itau + *m;
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgelqf__0(
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itau as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
            i__1 = (*m - 1 as ::core::ffi::c_long) as integer;
            i__2 = (*m - 1 as ::core::ffi::c_long) as integer;
            dlaset__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
                &raw mut i__2,
                &raw mut c_b227,
                &raw mut c_b227,
                a.offset(
                    (((a_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                lda,
            );
            ie = 1 as integer;
            itauq = ie + *m;
            itaup = itauq + *m;
            nwork = itaup + *m;
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgebrd__0(
                m,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                work.offset(itauq as isize) as *mut doublereal,
                work.offset(itaup as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
            nwork = ie + *m;
            dbdsdc__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                &raw mut dum as *mut doublereal,
                &raw mut c__1,
                &raw mut dum as *mut doublereal,
                &raw mut c__1,
                &raw mut dum as *mut doublereal,
                &raw mut idum as *mut integer,
                work.offset(nwork as isize) as *mut doublereal,
                iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                info,
            );
        } else if wntqo != 0 {
            ivt = 1 as integer;
            il = ivt + *m * *m;
            if *lwork >= *m * *n + *m * *m + *m * 3 as integer + bdspac {
                ldwrkl = *m;
                chunk = *n;
            } else {
                ldwrkl = *m;
                chunk = (*lwork - *m * *m) / *m;
            }
            itau = il + ldwrkl * *m;
            nwork = itau + *m;
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgelqf__0(
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itau as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
            dlacpy__0(
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
            );
            i__1 = (*m - 1 as ::core::ffi::c_long) as integer;
            i__2 = (*m - 1 as ::core::ffi::c_long) as integer;
            dlaset__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
                &raw mut i__2,
                &raw mut c_b227,
                &raw mut c_b227,
                work.offset((il + ldwrkl) as isize) as *mut doublereal,
                &raw mut ldwrkl,
            );
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dorglq__0(
                m,
                n,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itau as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
            ie = itau;
            itauq = ie + *m;
            itaup = itauq + *m;
            nwork = itaup + *m;
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgebrd__0(
                m,
                m,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                work.offset(itauq as isize) as *mut doublereal,
                work.offset(itaup as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
            dbdsdc__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                work.offset(ivt as isize) as *mut doublereal,
                m,
                &raw mut dum as *mut doublereal,
                &raw mut idum as *mut integer,
                work.offset(nwork as isize) as *mut doublereal,
                iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                info,
            );
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                m,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
                work.offset(itauq as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                m,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
                work.offset(itaup as isize) as *mut doublereal,
                work.offset(ivt as isize) as *mut doublereal,
                m,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
            i__1 = *n;
            i__2 = chunk;
            i__ = 1 as integer;
            while if i__2 < 0 as ::core::ffi::c_long {
                (i__ >= i__1) as ::core::ffi::c_int
            } else {
                (i__ <= i__1) as ::core::ffi::c_int
            } != 0
            {
                i__3 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                blk = (if i__3 <= chunk {
                    i__3 as ::core::ffi::c_long
                } else {
                    chunk as ::core::ffi::c_long
                }) as integer;
                f2c_dgemm_0(
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    &raw mut blk,
                    m,
                    &raw mut c_b248,
                    work.offset(ivt as isize) as *mut doublereal,
                    m,
                    a.offset(
                        (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    lda,
                    &raw mut c_b227,
                    work.offset(il as isize) as *mut doublereal,
                    &raw mut ldwrkl,
                );
                dlacpy__0(
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    &raw mut blk,
                    work.offset(il as isize) as *mut doublereal,
                    &raw mut ldwrkl,
                    a.offset(
                        (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    lda,
                );
                i__ += i__2 as ::core::ffi::c_long;
            }
        } else if wntqs != 0 {
            il = 1 as integer;
            ldwrkl = *m;
            itau = il + ldwrkl * *m;
            nwork = itau + *m;
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgelqf__0(
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itau as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            dlacpy__0(
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
            );
            i__2 = (*m - 1 as ::core::ffi::c_long) as integer;
            i__1 = (*m - 1 as ::core::ffi::c_long) as integer;
            dlaset__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__1,
                &raw mut c_b227,
                &raw mut c_b227,
                work.offset((il + ldwrkl) as isize) as *mut doublereal,
                &raw mut ldwrkl,
            );
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dorglq__0(
                m,
                n,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itau as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            ie = itau;
            itauq = ie + *m;
            itaup = itauq + *m;
            nwork = itaup + *m;
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgebrd__0(
                m,
                m,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                work.offset(itauq as isize) as *mut doublereal,
                work.offset(itaup as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            dbdsdc__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
                &raw mut dum as *mut doublereal,
                &raw mut idum as *mut integer,
                work.offset(nwork as isize) as *mut doublereal,
                iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                info,
            );
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                m,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
                work.offset(itauq as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                m,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
                work.offset(itaup as isize) as *mut doublereal,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            dlacpy__0(
                b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
            );
            f2c_dgemm_0(
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                m,
                &raw mut c_b248,
                work.offset(il as isize) as *mut doublereal,
                &raw mut ldwrkl,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                &raw mut c_b227,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
            );
        } else if wntqa != 0 {
            ivt = 1 as integer;
            ldwkvt = *m;
            itau = ivt + ldwkvt * *m;
            nwork = itau + *m;
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgelqf__0(
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itau as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            dlacpy__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
            );
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dorglq__0(
                n,
                n,
                m,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
                work.offset(itau as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            i__2 = (*m - 1 as ::core::ffi::c_long) as integer;
            i__1 = (*m - 1 as ::core::ffi::c_long) as integer;
            dlaset__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__1,
                &raw mut c_b227,
                &raw mut c_b227,
                a.offset(
                    (((a_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                lda,
            );
            ie = itau;
            itauq = ie + *m;
            itaup = itauq + *m;
            nwork = itaup + *m;
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgebrd__0(
                m,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                work.offset(itauq as isize) as *mut doublereal,
                work.offset(itaup as isize) as *mut doublereal,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            dbdsdc__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                work.offset(ivt as isize) as *mut doublereal,
                &raw mut ldwkvt,
                &raw mut dum as *mut doublereal,
                &raw mut idum as *mut integer,
                work.offset(nwork as isize) as *mut doublereal,
                iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                info,
            );
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itauq as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itaup as isize) as *mut doublereal,
                work.offset(ivt as isize) as *mut doublereal,
                &raw mut ldwkvt,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            f2c_dgemm_0(
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                m,
                &raw mut c_b248,
                work.offset(ivt as isize) as *mut doublereal,
                &raw mut ldwkvt,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
                &raw mut c_b227,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
            );
            dlacpy__0(
                b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
            );
        }
    } else {
        ie = 1 as integer;
        itauq = ie + *m;
        itaup = itauq + *m;
        nwork = itaup + *m;
        i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dgebrd__0(
            m,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(ie as isize) as *mut doublereal,
            work.offset(itauq as isize) as *mut doublereal,
            work.offset(itaup as isize) as *mut doublereal,
            work.offset(nwork as isize) as *mut doublereal,
            &raw mut i__2,
            &raw mut ierr,
        );
        if wntqn != 0 {
            dbdsdc__0(
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                &raw mut dum as *mut doublereal,
                &raw mut c__1,
                &raw mut dum as *mut doublereal,
                &raw mut c__1,
                &raw mut dum as *mut doublereal,
                &raw mut idum as *mut integer,
                work.offset(nwork as isize) as *mut doublereal,
                iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                info,
            );
        } else if wntqo != 0 {
            ldwkvt = *m;
            ivt = nwork;
            if *lwork >= *m * *n + *m * 3 as integer + bdspac {
                dlaset__0(
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    &raw mut c_b227,
                    &raw mut c_b227,
                    work.offset(ivt as isize) as *mut doublereal,
                    &raw mut ldwkvt,
                );
                nwork = ivt + ldwkvt * *n;
            } else {
                nwork = ivt + ldwkvt * *m;
                il = nwork;
                chunk = (*lwork - *m * *m - *m * 3 as integer) / *m;
            }
            dbdsdc__0(
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                work.offset(ivt as isize) as *mut doublereal,
                &raw mut ldwkvt,
                &raw mut dum as *mut doublereal,
                &raw mut idum as *mut integer,
                work.offset(nwork as isize) as *mut doublereal,
                iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                info,
            );
            i__2 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itauq as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__2,
                &raw mut ierr,
            );
            if *lwork >= *m * *n + *m * 3 as integer + bdspac {
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dormbr__0(
                    b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    m,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itaup as isize) as *mut doublereal,
                    work.offset(ivt as isize) as *mut doublereal,
                    &raw mut ldwkvt,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                dlacpy__0(
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    work.offset(ivt as isize) as *mut doublereal,
                    &raw mut ldwkvt,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                );
            } else {
                i__2 =
                    (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dorgbr__0(
                    b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    m,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    work.offset(itaup as isize) as *mut doublereal,
                    work.offset(nwork as isize) as *mut doublereal,
                    &raw mut i__2,
                    &raw mut ierr,
                );
                i__2 = *n;
                i__1 = chunk;
                i__ = 1 as integer;
                while if i__1 < 0 as ::core::ffi::c_long {
                    (i__ >= i__2) as ::core::ffi::c_int
                } else {
                    (i__ <= i__2) as ::core::ffi::c_int
                } != 0
                {
                    i__3 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    blk = (if i__3 <= chunk {
                        i__3 as ::core::ffi::c_long
                    } else {
                        chunk as ::core::ffi::c_long
                    }) as integer;
                    f2c_dgemm_0(
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        m,
                        &raw mut blk,
                        m,
                        &raw mut c_b248,
                        work.offset(ivt as isize) as *mut doublereal,
                        &raw mut ldwkvt,
                        a.offset(
                            (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        lda,
                        &raw mut c_b227,
                        work.offset(il as isize) as *mut doublereal,
                        m,
                    );
                    dlacpy__0(
                        b"F\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        m,
                        &raw mut blk,
                        work.offset(il as isize) as *mut doublereal,
                        m,
                        a.offset(
                            (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        lda,
                    );
                    i__ += i__1 as ::core::ffi::c_long;
                }
            }
        } else if wntqs != 0 {
            dlaset__0(
                b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                &raw mut c_b227,
                &raw mut c_b227,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
            );
            dbdsdc__0(
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
                &raw mut dum as *mut doublereal,
                &raw mut idum as *mut integer,
                work.offset(nwork as isize) as *mut doublereal,
                iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                info,
            );
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itauq as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itaup as isize) as *mut doublereal,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
        } else if wntqa != 0 {
            dlaset__0(
                b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b227,
                &raw mut c_b227,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
            );
            dbdsdc__0(
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(ie as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
                &raw mut dum as *mut doublereal,
                &raw mut idum as *mut integer,
                work.offset(nwork as isize) as *mut doublereal,
                iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                info,
            );
            if *n > *m {
                i__1 = *n - *m;
                i__2 = *n - *m;
                dlaset__0(
                    b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut i__1,
                    &raw mut i__2,
                    &raw mut c_b227,
                    &raw mut c_b248,
                    vt.offset((*m + 1 as integer + (*m + 1 as integer) * vt_dim1) as isize)
                        as *mut doublereal,
                    ldvt,
                );
            }
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                m,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itauq as isize) as *mut doublereal,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
            i__1 = (*lwork - nwork as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dormbr__0(
                b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                m,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                work.offset(itaup as isize) as *mut doublereal,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
                work.offset(nwork as isize) as *mut doublereal,
                &raw mut i__1,
                &raw mut ierr,
            );
        }
    }
    if iscl == 1 as ::core::ffi::c_long {
        if anrm > bignum {
            dlascl__0(
                b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                &raw mut c__0,
                &raw mut bignum,
                &raw mut anrm,
                &raw mut minmn,
                &raw mut c__1,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut minmn,
                &raw mut ierr,
            );
        }
        if anrm < smlnum {
            dlascl__0(
                b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                &raw mut c__0,
                &raw mut smlnum,
                &raw mut anrm,
                &raw mut minmn,
                &raw mut c__1,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut minmn,
                &raw mut ierr,
            );
        }
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = maxwrk as doublereal;
    return 0 as ::core::ffi::c_int;
}
