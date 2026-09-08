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
pub const TRUE_: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE_: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut c__1: integer = 1 as integer;
static mut c__0: integer = 0 as integer;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dgeev_(
    mut jobvl: *mut ::core::ffi::c_char,
    mut jobvr: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut wr: *mut doublereal,
    mut wi: *mut doublereal,
    mut vl: *mut doublereal,
    mut ldvl: *mut integer,
    mut vr: *mut doublereal,
    mut ldvr: *mut integer,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut vl_dim1: integer = 0;
    let mut vl_offset: integer = 0;
    let mut vr_dim1: integer = 0;
    let mut vr_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut i__: integer = 0;
    let mut k: integer = 0;
    let mut r__: doublereal = 0.;
    let mut cs: doublereal = 0.;
    let mut sn: doublereal = 0.;
    let mut ihi: integer = 0;
    let mut scl: doublereal = 0.;
    let mut ilo: integer = 0;
    let mut dum: [doublereal; 1] = [0.; 1];
    let mut eps: doublereal = 0.;
    let mut ibal: integer = 0;
    let mut side: [::core::ffi::c_char; 1] = [0; 1];
    let mut anrm: doublereal = 0.;
    let mut ierr: integer = 0;
    let mut itau: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_drot"]
        fn f2c_drot_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    let mut iwrk: integer = 0;
    let mut nout: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dnrm2"]
        fn f2c_dnrm2_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dscal"]
        fn f2c_dscal_0(
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
    extern "C" {
        #[link_name = "dgelsd_closure_dlapy2_"]
        fn dlapy2__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlabad_"]
        fn dlabad__0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dgebak_"]
        fn dgebak__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dgebal_"]
        fn dgebal__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut scalea: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    let mut cscale: doublereal = 0.;
    extern "C" {
        #[link_name = "dgeev_closure_dlange_"]
        fn dlange__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dgehrd_"]
        fn dgehrd__0(
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
        #[link_name = "dgelsd_closure_f2c_idamax"]
        fn f2c_idamax_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> integer;
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
        #[link_name = "dgelsd_closure_dlartg_"]
        fn dlartg__0(
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut select: [logical; 1] = [0; 1];
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
    let mut bignum: doublereal = 0.;
    extern "C" {
        #[link_name = "dgeev_closure_dorghr_"]
        fn dorghr__0(
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
        #[link_name = "dgeev_closure_dhseqr_"]
        fn dhseqr__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dtrevc_"]
        fn dtrevc__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut logical,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut minwrk: integer = 0;
    let mut maxwrk: integer = 0;
    let mut wantvl: logical = 0;
    let mut smlnum: doublereal = 0.;
    let mut hswork: integer = 0;
    let mut lquery: logical = 0;
    let mut wantvr: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    wr = wr.offset(-1);
    wi = wi.offset(-1);
    vl_dim1 = *ldvl;
    vl_offset = 1 as integer + vl_dim1;
    vl = vl.offset(-(vl_offset as isize));
    vr_dim1 = *ldvr;
    vr_offset = 1 as integer + vr_dim1;
    vr = vr.offset(-(vr_offset as isize));
    work = work.offset(-1);
    *info = 0 as integer;
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
    wantvl = lsame__0(
        jobvl,
        b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    wantvr = lsame__0(
        jobvr,
        b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if wantvl == 0
        && lsame__0(
            jobvl,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if wantvr == 0
        && lsame__0(
            jobvr,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
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
    } else if *ldvl < 1 as ::core::ffi::c_long || wantvl != 0 && *ldvl < *n {
        *info = -(9 as ::core::ffi::c_int) as integer;
    } else if *ldvr < 1 as ::core::ffi::c_long || wantvr != 0 && *ldvr < *n {
        *info = -(11 as ::core::ffi::c_int) as integer;
    }
    if *info == 0 as ::core::ffi::c_long {
        if *n == 0 as ::core::ffi::c_long {
            minwrk = 1 as integer;
            maxwrk = 1 as integer;
        } else {
            maxwrk = (*n << 1 as ::core::ffi::c_int)
                + *n * ilaenv__0(
                    &raw mut c__1,
                    b"DGEHRD\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    &raw mut c__1,
                    n,
                    &raw mut c__0,
                );
            if wantvl != 0 {
                minwrk = *n << 2 as ::core::ffi::c_int;
                i__1 = maxwrk;
                i__2 = (*n << 1 as ::core::ffi::c_int)
                    + (*n - 1 as integer)
                        * ilaenv__0(
                            &raw mut c__1,
                            b"DORGHR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            &raw mut c__1,
                            n,
                            &raw mut c_n1,
                        );
                maxwrk = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
                dhseqr__0(
                    b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    &raw mut c__1,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    vl.offset(vl_offset as isize) as *mut doublereal,
                    ldvl,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c_n1,
                    info,
                );
                hswork = *work.offset(1 as ::core::ffi::c_int as isize) as integer;
                i__1 = maxwrk;
                i__2 = (*n + 1 as ::core::ffi::c_long) as integer;
                i__1 = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
                i__2 = *n + hswork;
                maxwrk = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
                i__1 = maxwrk;
                i__2 = *n << 2 as ::core::ffi::c_int;
                maxwrk = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
            } else if wantvr != 0 {
                minwrk = *n << 2 as ::core::ffi::c_int;
                i__1 = maxwrk;
                i__2 = (*n << 1 as ::core::ffi::c_int)
                    + (*n - 1 as integer)
                        * ilaenv__0(
                            &raw mut c__1,
                            b"DORGHR\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b" \0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            &raw mut c__1,
                            n,
                            &raw mut c_n1,
                        );
                maxwrk = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
                dhseqr__0(
                    b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    &raw mut c__1,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    vr.offset(vr_offset as isize) as *mut doublereal,
                    ldvr,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c_n1,
                    info,
                );
                hswork = *work.offset(1 as ::core::ffi::c_int as isize) as integer;
                i__1 = maxwrk;
                i__2 = (*n + 1 as ::core::ffi::c_long) as integer;
                i__1 = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
                i__2 = *n + hswork;
                maxwrk = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
                i__1 = maxwrk;
                i__2 = *n << 2 as ::core::ffi::c_int;
                maxwrk = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
            } else {
                minwrk = (*n * 3 as ::core::ffi::c_long) as integer;
                dhseqr__0(
                    b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    &raw mut c__1,
                    n,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    vr.offset(vr_offset as isize) as *mut doublereal,
                    ldvr,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c_n1,
                    info,
                );
                hswork = *work.offset(1 as ::core::ffi::c_int as isize) as integer;
                i__1 = maxwrk;
                i__2 = (*n + 1 as ::core::ffi::c_long) as integer;
                i__1 = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
                i__2 = *n + hswork;
                maxwrk = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
            }
            maxwrk = (if maxwrk >= minwrk {
                maxwrk as ::core::ffi::c_long
            } else {
                minwrk as ::core::ffi::c_long
            }) as integer;
        }
        *work.offset(1 as ::core::ffi::c_int as isize) = maxwrk as doublereal;
        if *lwork < minwrk && lquery == 0 {
            *info = -(13 as ::core::ffi::c_int) as integer;
        }
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DGEEV \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    eps = dlamch__0(b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char);
    smlnum =
        dlamch__0(b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char);
    bignum = 1.0f64 / smlnum;
    dlabad__0(&raw mut smlnum, &raw mut bignum);
    smlnum = sqrt(smlnum) as doublereal / eps;
    bignum = 1.0f64 / smlnum;
    anrm = dlange__0(
        b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        n,
        n,
        a.offset(a_offset as isize) as *mut doublereal,
        lda,
        &raw mut dum as *mut doublereal,
    );
    scalea = FALSE_ as logical;
    if anrm > 0.0f64 && anrm < smlnum {
        scalea = TRUE_ as logical;
        cscale = smlnum;
    } else if anrm > bignum {
        scalea = TRUE_ as logical;
        cscale = bignum;
    }
    if scalea != 0 {
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut anrm,
            &raw mut cscale,
            n,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            &raw mut ierr,
        );
    }
    ibal = 1 as integer;
    dgebal__0(
        b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        n,
        a.offset(a_offset as isize) as *mut doublereal,
        lda,
        &raw mut ilo,
        &raw mut ihi,
        work.offset(ibal as isize) as *mut doublereal,
        &raw mut ierr,
    );
    itau = ibal + *n;
    iwrk = itau + *n;
    i__1 = (*lwork - iwrk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    dgehrd__0(
        n,
        &raw mut ilo,
        &raw mut ihi,
        a.offset(a_offset as isize) as *mut doublereal,
        lda,
        work.offset(itau as isize) as *mut doublereal,
        work.offset(iwrk as isize) as *mut doublereal,
        &raw mut i__1,
        &raw mut ierr,
    );
    if wantvl != 0 {
        *(&raw mut side as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
            'L' as i32 as ::core::ffi::c_uchar;
        dlacpy__0(
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            vl.offset(vl_offset as isize) as *mut doublereal,
            ldvl,
        );
        i__1 = (*lwork - iwrk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dorghr__0(
            n,
            &raw mut ilo,
            &raw mut ihi,
            vl.offset(vl_offset as isize) as *mut doublereal,
            ldvl,
            work.offset(itau as isize) as *mut doublereal,
            work.offset(iwrk as isize) as *mut doublereal,
            &raw mut i__1,
            &raw mut ierr,
        );
        iwrk = itau;
        i__1 = (*lwork - iwrk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dhseqr__0(
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            &raw mut ilo,
            &raw mut ihi,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            vl.offset(vl_offset as isize) as *mut doublereal,
            ldvl,
            work.offset(iwrk as isize) as *mut doublereal,
            &raw mut i__1,
            info,
        );
        if wantvr != 0 {
            *(&raw mut side as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                'B' as i32 as ::core::ffi::c_uchar;
            dlacpy__0(
                b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                vl.offset(vl_offset as isize) as *mut doublereal,
                ldvl,
                vr.offset(vr_offset as isize) as *mut doublereal,
                ldvr,
            );
        }
    } else if wantvr != 0 {
        *(&raw mut side as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
            'R' as i32 as ::core::ffi::c_uchar;
        dlacpy__0(
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            vr.offset(vr_offset as isize) as *mut doublereal,
            ldvr,
        );
        i__1 = (*lwork - iwrk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dorghr__0(
            n,
            &raw mut ilo,
            &raw mut ihi,
            vr.offset(vr_offset as isize) as *mut doublereal,
            ldvr,
            work.offset(itau as isize) as *mut doublereal,
            work.offset(iwrk as isize) as *mut doublereal,
            &raw mut i__1,
            &raw mut ierr,
        );
        iwrk = itau;
        i__1 = (*lwork - iwrk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dhseqr__0(
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            &raw mut ilo,
            &raw mut ihi,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            vr.offset(vr_offset as isize) as *mut doublereal,
            ldvr,
            work.offset(iwrk as isize) as *mut doublereal,
            &raw mut i__1,
            info,
        );
    } else {
        iwrk = itau;
        i__1 = (*lwork - iwrk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dhseqr__0(
            b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            &raw mut ilo,
            &raw mut ihi,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            vr.offset(vr_offset as isize) as *mut doublereal,
            ldvr,
            work.offset(iwrk as isize) as *mut doublereal,
            &raw mut i__1,
            info,
        );
    }
    if !(*info > 0 as ::core::ffi::c_long) {
        if wantvl != 0 || wantvr != 0 {
            dtrevc__0(
                &raw mut side as *mut ::core::ffi::c_char,
                b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut select as *mut logical,
                n,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                vl.offset(vl_offset as isize) as *mut doublereal,
                ldvl,
                vr.offset(vr_offset as isize) as *mut doublereal,
                ldvr,
                n,
                &raw mut nout,
                work.offset(iwrk as isize) as *mut doublereal,
                &raw mut ierr,
            );
        }
        if wantvl != 0 {
            dgebak__0(
                b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                &raw mut ilo,
                &raw mut ihi,
                work.offset(ibal as isize) as *mut doublereal,
                n,
                vl.offset(vl_offset as isize) as *mut doublereal,
                ldvl,
                &raw mut ierr,
            );
            i__1 = *n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                if *wi.offset(i__ as isize) == 0.0f64 {
                    scl = 1.0f64
                        / f2c_dnrm2_0(
                            n,
                            vl.offset(
                                (i__ as ::core::ffi::c_long * vl_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                        );
                    f2c_dscal_0(
                        n,
                        &raw mut scl,
                        vl.offset(
                            (i__ as ::core::ffi::c_long * vl_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                } else if *wi.offset(i__ as isize) > 0.0f64 {
                    d__1 = f2c_dnrm2_0(
                        n,
                        vl.offset(
                            (i__ as ::core::ffi::c_long * vl_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    d__2 = f2c_dnrm2_0(
                        n,
                        vl.offset(
                            ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * vl_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    scl = 1.0f64 / dlapy2__0(&raw mut d__1, &raw mut d__2);
                    f2c_dscal_0(
                        n,
                        &raw mut scl,
                        vl.offset(
                            (i__ as ::core::ffi::c_long * vl_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    f2c_dscal_0(
                        n,
                        &raw mut scl,
                        vl.offset(
                            ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * vl_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    i__2 = *n;
                    k = 1 as integer;
                    while k <= i__2 {
                        d__1 = *vl.offset((k + i__ * vl_dim1) as isize);
                        d__2 = *vl.offset((k + (i__ + 1 as integer) * vl_dim1) as isize);
                        *work.offset(
                            (iwrk as ::core::ffi::c_long + k as ::core::ffi::c_long
                                - 1 as ::core::ffi::c_long) as isize,
                        ) = d__1 * d__1 + d__2 * d__2;
                        k += 1;
                    }
                    k = f2c_idamax_0(
                        n,
                        work.offset(iwrk as isize) as *mut doublereal,
                        &raw mut c__1,
                    );
                    dlartg__0(
                        vl.offset((k + i__ * vl_dim1) as isize) as *mut doublereal,
                        vl.offset((k + (i__ + 1 as integer) * vl_dim1) as isize) as *mut doublereal,
                        &raw mut cs,
                        &raw mut sn,
                        &raw mut r__,
                    );
                    f2c_drot_0(
                        n,
                        vl.offset(
                            (i__ as ::core::ffi::c_long * vl_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        vl.offset(
                            ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * vl_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut cs,
                        &raw mut sn,
                    );
                    *vl.offset((k + (i__ + 1 as integer) * vl_dim1) as isize) =
                        0.0f64 as doublereal;
                }
                i__ += 1;
            }
        }
        if wantvr != 0 {
            dgebak__0(
                b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                &raw mut ilo,
                &raw mut ihi,
                work.offset(ibal as isize) as *mut doublereal,
                n,
                vr.offset(vr_offset as isize) as *mut doublereal,
                ldvr,
                &raw mut ierr,
            );
            i__1 = *n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                if *wi.offset(i__ as isize) == 0.0f64 {
                    scl = 1.0f64
                        / f2c_dnrm2_0(
                            n,
                            vr.offset(
                                (i__ as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                        );
                    f2c_dscal_0(
                        n,
                        &raw mut scl,
                        vr.offset(
                            (i__ as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                } else if *wi.offset(i__ as isize) > 0.0f64 {
                    d__1 = f2c_dnrm2_0(
                        n,
                        vr.offset(
                            (i__ as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    d__2 = f2c_dnrm2_0(
                        n,
                        vr.offset(
                            ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * vr_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    scl = 1.0f64 / dlapy2__0(&raw mut d__1, &raw mut d__2);
                    f2c_dscal_0(
                        n,
                        &raw mut scl,
                        vr.offset(
                            (i__ as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    f2c_dscal_0(
                        n,
                        &raw mut scl,
                        vr.offset(
                            ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * vr_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    i__2 = *n;
                    k = 1 as integer;
                    while k <= i__2 {
                        d__1 = *vr.offset((k + i__ * vr_dim1) as isize);
                        d__2 = *vr.offset((k + (i__ + 1 as integer) * vr_dim1) as isize);
                        *work.offset(
                            (iwrk as ::core::ffi::c_long + k as ::core::ffi::c_long
                                - 1 as ::core::ffi::c_long) as isize,
                        ) = d__1 * d__1 + d__2 * d__2;
                        k += 1;
                    }
                    k = f2c_idamax_0(
                        n,
                        work.offset(iwrk as isize) as *mut doublereal,
                        &raw mut c__1,
                    );
                    dlartg__0(
                        vr.offset((k + i__ * vr_dim1) as isize) as *mut doublereal,
                        vr.offset((k + (i__ + 1 as integer) * vr_dim1) as isize) as *mut doublereal,
                        &raw mut cs,
                        &raw mut sn,
                        &raw mut r__,
                    );
                    f2c_drot_0(
                        n,
                        vr.offset(
                            (i__ as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        vr.offset(
                            ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * vr_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut cs,
                        &raw mut sn,
                    );
                    *vr.offset((k + (i__ + 1 as integer) * vr_dim1) as isize) =
                        0.0f64 as doublereal;
                }
                i__ += 1;
            }
        }
    }
    if scalea != 0 {
        i__1 = *n - *info;
        i__3 = *n - *info;
        i__2 = (if i__3 >= 1 as ::core::ffi::c_long {
            i__3 as ::core::ffi::c_long
        } else {
            1 as ::core::ffi::c_long
        }) as integer;
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut cscale,
            &raw mut anrm,
            &raw mut i__1,
            &raw mut c__1,
            wr.offset((*info + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            &raw mut i__2,
            &raw mut ierr,
        );
        i__1 = *n - *info;
        i__3 = *n - *info;
        i__2 = (if i__3 >= 1 as ::core::ffi::c_long {
            i__3 as ::core::ffi::c_long
        } else {
            1 as ::core::ffi::c_long
        }) as integer;
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut cscale,
            &raw mut anrm,
            &raw mut i__1,
            &raw mut c__1,
            wi.offset((*info + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            &raw mut i__2,
            &raw mut ierr,
        );
        if *info > 0 as ::core::ffi::c_long {
            i__1 = (ilo as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            dlascl__0(
                b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                &raw mut c__0,
                &raw mut cscale,
                &raw mut anrm,
                &raw mut i__1,
                &raw mut c__1,
                wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                n,
                &raw mut ierr,
            );
            i__1 = (ilo as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            dlascl__0(
                b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                &raw mut c__0,
                &raw mut cscale,
                &raw mut anrm,
                &raw mut i__1,
                &raw mut c__1,
                wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                n,
                &raw mut ierr,
            );
        }
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = maxwrk as doublereal;
    return 0 as ::core::ffi::c_int;
}
