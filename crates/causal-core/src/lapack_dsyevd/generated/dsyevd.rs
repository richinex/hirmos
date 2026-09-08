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
static mut c_b17: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dsyevd_(
    mut jobz: *mut ::core::ffi::c_char,
    mut uplo: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut w: *mut doublereal,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut iwork: *mut integer,
    mut liwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut inde: integer = 0;
    let mut anrm: doublereal = 0.;
    let mut rmin: doublereal = 0.;
    let mut rmax: doublereal = 0.;
    let mut lopt: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dscal"]
        fn f2c_dscal_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut sigma: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut iinfo: integer = 0;
    let mut lwmin: integer = 0;
    let mut liopt: integer = 0;
    let mut lower: logical = 0;
    let mut wantz: logical = 0;
    let mut indwk2: integer = 0;
    let mut llwrk2: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    let mut iscale: integer = 0;
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
        #[link_name = "dsyevd_closure_dstedc_"]
        fn dstedc__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
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
    let mut safmin: doublereal = 0.;
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
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut bignum: doublereal = 0.;
    let mut indtau: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dsterf_"]
        fn dsterf__0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_dlansy_"]
        fn dlansy__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> doublereal;
    }
    let mut indwrk: integer = 0;
    let mut liwmin: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dormtr_"]
        fn dormtr__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
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
        #[link_name = "dsyevd_closure_dsytrd_"]
        fn dsytrd__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut llwork: integer = 0;
    let mut smlnum: doublereal = 0.;
    let mut lquery: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    w = w.offset(-1);
    work = work.offset(-1);
    iwork = iwork.offset(-1);
    wantz = lsame__0(
        jobz,
        b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    lower = lsame__0(
        uplo,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
        || *liwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as ::core::ffi::c_int as logical;
    *info = 0 as integer;
    if !(wantz != 0
        || lsame__0(
            jobz,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0)
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if !(lower != 0
        || lsame__0(
            uplo,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0)
    {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(5 as ::core::ffi::c_int) as integer;
    }
    if *info == 0 as ::core::ffi::c_long {
        if *n <= 1 as ::core::ffi::c_long {
            liwmin = 1 as integer;
            lwmin = 1 as integer;
            lopt = lwmin;
            liopt = liwmin;
        } else {
            if wantz != 0 {
                liwmin = (*n * 5 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as integer;
                i__1 = *n;
                lwmin = *n * 6 as integer + 1 as integer + (i__1 * i__1 << 1 as ::core::ffi::c_int);
            } else {
                liwmin = 1 as integer;
                lwmin = ((*n << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long) as integer;
            }
            i__1 = lwmin;
            i__2 = (*n << 1 as ::core::ffi::c_int)
                + ilaenv__0(
                    &raw mut c__1,
                    b"DSYTRD\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    uplo,
                    n,
                    &raw mut c_n1,
                    &raw mut c_n1,
                    &raw mut c_n1,
                );
            lopt = (if i__1 >= i__2 {
                i__1 as ::core::ffi::c_long
            } else {
                i__2 as ::core::ffi::c_long
            }) as integer;
            liopt = liwmin;
        }
        *work.offset(1 as ::core::ffi::c_int as isize) = lopt as doublereal;
        *iwork.offset(1 as ::core::ffi::c_int as isize) = liopt;
        if *lwork < lwmin && lquery == 0 {
            *info = -(8 as ::core::ffi::c_int) as integer;
        } else if *liwork < liwmin && lquery == 0 {
            *info = -(10 as ::core::ffi::c_int) as integer;
        }
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DSYEVD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *n == 1 as ::core::ffi::c_long {
        *w.offset(1 as ::core::ffi::c_int as isize) =
            *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
        if wantz != 0 {
            *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                1.0f64 as doublereal;
        }
        return 0 as ::core::ffi::c_int;
    }
    safmin = dlamch__0(
        b"Safe minimum\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    eps = dlamch__0(
        b"Precision\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    smlnum = safmin / eps;
    bignum = 1.0f64 / smlnum;
    rmin = sqrt(smlnum) as doublereal;
    rmax = sqrt(bignum) as doublereal;
    anrm = dlansy__0(
        b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        uplo,
        n,
        a.offset(a_offset as isize) as *mut doublereal,
        lda,
        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
    );
    iscale = 0 as integer;
    if anrm > 0.0f64 && anrm < rmin {
        iscale = 1 as integer;
        sigma = rmin / anrm;
    } else if anrm > rmax {
        iscale = 1 as integer;
        sigma = rmax / anrm;
    }
    if iscale == 1 as ::core::ffi::c_long {
        dlascl__0(
            uplo,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut c_b17,
            &raw mut sigma,
            n,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            info,
        );
    }
    inde = 1 as integer;
    indtau = inde + *n;
    indwrk = indtau + *n;
    llwork = (*lwork - indwrk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    indwk2 = indwrk + *n * *n;
    llwrk2 = (*lwork - indwk2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    dsytrd__0(
        uplo,
        n,
        a.offset(a_offset as isize) as *mut doublereal,
        lda,
        w.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        work.offset(inde as isize) as *mut doublereal,
        work.offset(indtau as isize) as *mut doublereal,
        work.offset(indwrk as isize) as *mut doublereal,
        &raw mut llwork,
        &raw mut iinfo,
    );
    lopt =
        ((*n << 1 as ::core::ffi::c_int) as doublereal + *work.offset(indwrk as isize)) as integer;
    if wantz == 0 {
        dsterf__0(
            n,
            w.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(inde as isize) as *mut doublereal,
            info,
        );
    } else {
        dstedc__0(
            b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            w.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(inde as isize) as *mut doublereal,
            work.offset(indwrk as isize) as *mut doublereal,
            n,
            work.offset(indwk2 as isize) as *mut doublereal,
            &raw mut llwrk2,
            iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
            liwork,
            info,
        );
        dormtr__0(
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            uplo,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            work.offset(indtau as isize) as *mut doublereal,
            work.offset(indwrk as isize) as *mut doublereal,
            n,
            work.offset(indwk2 as isize) as *mut doublereal,
            &raw mut llwrk2,
            &raw mut iinfo,
        );
        dlacpy__0(
            b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            n,
            work.offset(indwrk as isize) as *mut doublereal,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
        );
        i__3 = *n;
        i__1 = lopt;
        i__2 = *n * 6 as integer + 1 as integer + (i__3 * i__3 << 1 as ::core::ffi::c_int);
        lopt = (if i__1 >= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
    }
    if iscale == 1 as ::core::ffi::c_long {
        d__1 = 1.0f64 / sigma;
        f2c_dscal_0(
            n,
            &raw mut d__1,
            w.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = lopt as doublereal;
    *iwork.offset(1 as ::core::ffi::c_int as isize) = liopt;
    return 0 as ::core::ffi::c_int;
}
