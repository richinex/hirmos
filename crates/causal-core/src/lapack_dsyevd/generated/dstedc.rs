#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn log(_: doublereal) -> ::core::ffi::c_double;
    fn sqrt(_: doublereal) -> ::core::ffi::c_double;
}
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
static mut c__9: integer = 9 as integer;
static mut c__0: integer = 0 as integer;
static mut c__2: integer = 2 as integer;
static mut c_b17: doublereal = 0.0f64;
static mut c_b18: doublereal = 1.0f64;
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dstedc_(
    mut compz: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut z__: *mut doublereal,
    mut ldz: *mut integer,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut iwork: *mut integer,
    mut liwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut z_dim1: integer = 0;
    let mut z_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_pow_ii"]
        fn pow_ii_0(_: *mut integer, _: *mut integer) -> integer;
    }
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut m: integer = 0;
    let mut p: doublereal = 0.;
    let mut ii: integer = 0;
    let mut lgn: integer = 0;
    let mut eps: doublereal = 0.;
    let mut tiny: doublereal = 0.;
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
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dswap"]
        fn f2c_dswap_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut lwmin: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dlaed0_"]
        fn dlaed0__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut start: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
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
    let mut finish: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlanst_"]
        fn dlanst__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> doublereal;
    }
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
        #[link_name = "dgelsd_closure_dlasrt_"]
        fn dlasrt__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut liwmin: integer = 0;
    let mut icompz: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dsteqr_"]
        fn dsteqr__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut orgnrm: doublereal = 0.;
    let mut lquery: logical = 0;
    let mut smlsiz: integer = 0;
    let mut storez: integer = 0;
    let mut strtrw: integer = 0;
    d__ = d__.offset(-1);
    e = e.offset(-1);
    z_dim1 = *ldz;
    z_offset = 1 as integer + z_dim1;
    z__ = z__.offset(-(z_offset as isize));
    work = work.offset(-1);
    iwork = iwork.offset(-1);
    *info = 0 as integer;
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
        || *liwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as ::core::ffi::c_int as logical;
    if lsame__0(
        compz,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        icompz = 0 as integer;
    } else if lsame__0(
        compz,
        b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        icompz = 1 as integer;
    } else if lsame__0(
        compz,
        b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        icompz = 2 as integer;
    } else {
        icompz = -(1 as ::core::ffi::c_int) as integer;
    }
    if icompz < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *ldz < 1 as ::core::ffi::c_long
        || icompz > 0 as ::core::ffi::c_long
            && *ldz
                < (if 1 as ::core::ffi::c_long >= *n {
                    1 as ::core::ffi::c_long
                } else {
                    *n
                })
    {
        *info = -(6 as ::core::ffi::c_int) as integer;
    }
    if *info == 0 as ::core::ffi::c_long {
        smlsiz = ilaenv__0(
            &raw mut c__9,
            b"DSTEDC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut c__0,
        );
        if *n <= 1 as ::core::ffi::c_long || icompz == 0 as ::core::ffi::c_long {
            liwmin = 1 as integer;
            lwmin = 1 as integer;
        } else if *n <= smlsiz {
            liwmin = 1 as integer;
            lwmin = ((*n - 1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int) as integer;
        } else {
            lgn = (log(*n as doublereal) / log(2.0f64)) as integer;
            if pow_ii_0(&raw mut c__2, &raw mut lgn) < *n {
                lgn += 1;
            }
            if pow_ii_0(&raw mut c__2, &raw mut lgn) < *n {
                lgn += 1;
            }
            if icompz == 1 as ::core::ffi::c_long {
                i__1 = *n;
                lwmin = (*n * 3 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long
                    + (*n << 1 as ::core::ffi::c_int) * lgn as ::core::ffi::c_long
                    + i__1 as ::core::ffi::c_long
                        * i__1 as ::core::ffi::c_long
                        * 3 as ::core::ffi::c_long) as integer;
                liwmin = *n * 6 as integer + 6 as integer + *n * 5 as integer * lgn;
            } else if icompz == 2 as ::core::ffi::c_long {
                i__1 = *n;
                lwmin = (*n << 2 as ::core::ffi::c_int) + 1 as integer + i__1 * i__1;
                liwmin = (*n * 5 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as integer;
            }
        }
        *work.offset(1 as ::core::ffi::c_int as isize) = lwmin as doublereal;
        *iwork.offset(1 as ::core::ffi::c_int as isize) = liwmin;
        if *lwork < lwmin && lquery == 0 {
            *info = -(8 as ::core::ffi::c_int) as integer;
        } else if *liwork < liwmin && lquery == 0 {
            *info = -(10 as ::core::ffi::c_int) as integer;
        }
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DSTEDC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
        if icompz != 0 as ::core::ffi::c_long {
            *z__.offset((z_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                1.0f64 as doublereal;
        }
        return 0 as ::core::ffi::c_int;
    }
    if icompz == 0 as ::core::ffi::c_long {
        dsterf__0(
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            info,
        );
    } else if *n <= smlsiz {
        dsteqr__0(
            compz,
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            z__.offset(z_offset as isize) as *mut doublereal,
            ldz,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            info,
        );
    } else {
        if icompz == 1 as ::core::ffi::c_long {
            storez = (*n * *n + 1 as ::core::ffi::c_long) as integer;
        } else {
            storez = 1 as integer;
        }
        if icompz == 2 as ::core::ffi::c_long {
            dlaset__0(
                b"Full\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b17,
                &raw mut c_b18,
                z__.offset(z_offset as isize) as *mut doublereal,
                ldz,
            );
        }
        orgnrm = dlanst__0(
            b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        );
        if !(orgnrm == 0.0f64) {
            eps = dlamch__0(
                b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            start = 1 as integer;
            loop {
                if start <= *n {
                    finish = start;
                    while finish < *n {
                        d__1 = *d__.offset(finish as isize);
                        d__2 = *d__.offset(
                            (finish as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        );
                        tiny = (eps as ::core::ffi::c_double
                            * sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            )
                            * sqrt(
                                (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__2
                                } else {
                                    -d__2
                                }),
                            )) as doublereal;
                        d__1 = *e.offset(finish as isize);
                        if !((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) > tiny)
                        {
                            break;
                        }
                        finish += 1;
                    }
                    m = (finish as ::core::ffi::c_long - start as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                    if m == 1 as ::core::ffi::c_long {
                        start =
                            (finish as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    } else {
                        if m > smlsiz {
                            orgnrm = dlanst__0(
                                b"M\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                &raw mut m,
                                d__.offset(start as isize) as *mut doublereal,
                                e.offset(start as isize) as *mut doublereal,
                            );
                            dlascl__0(
                                b"G\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                &raw mut c__0,
                                &raw mut c__0,
                                &raw mut orgnrm,
                                &raw mut c_b18,
                                &raw mut m,
                                &raw mut c__1,
                                d__.offset(start as isize) as *mut doublereal,
                                &raw mut m,
                                info,
                            );
                            i__1 = (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                            i__2 = (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                            dlascl__0(
                                b"G\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                &raw mut c__0,
                                &raw mut c__0,
                                &raw mut orgnrm,
                                &raw mut c_b18,
                                &raw mut i__1,
                                &raw mut c__1,
                                e.offset(start as isize) as *mut doublereal,
                                &raw mut i__2,
                                info,
                            );
                            if icompz == 1 as ::core::ffi::c_long {
                                strtrw = 1 as integer;
                            } else {
                                strtrw = start;
                            }
                            dlaed0__0(
                                &raw mut icompz,
                                n,
                                &raw mut m,
                                d__.offset(start as isize) as *mut doublereal,
                                e.offset(start as isize) as *mut doublereal,
                                z__.offset((strtrw + start * z_dim1) as isize) as *mut doublereal,
                                ldz,
                                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                                n,
                                work.offset(storez as isize) as *mut doublereal,
                                iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                                info,
                            );
                            if *info != 0 as ::core::ffi::c_long {
                                *info = ((*info
                                    / (m as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    + start as ::core::ffi::c_long
                                    - 1 as ::core::ffi::c_long)
                                    * (*n + 1 as ::core::ffi::c_long)
                                    + *info % (m as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    + start as ::core::ffi::c_long
                                    - 1 as ::core::ffi::c_long)
                                    as integer;
                                break;
                            } else {
                                dlascl__0(
                                    b"G\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    &raw mut c__0,
                                    &raw mut c__0,
                                    &raw mut c_b18,
                                    &raw mut orgnrm,
                                    &raw mut m,
                                    &raw mut c__1,
                                    d__.offset(start as isize) as *mut doublereal,
                                    &raw mut m,
                                    info,
                                );
                            }
                        } else {
                            if icompz == 1 as ::core::ffi::c_long {
                                dsteqr__0(
                                    b"I\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    &raw mut m,
                                    d__.offset(start as isize) as *mut doublereal,
                                    e.offset(start as isize) as *mut doublereal,
                                    work.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut doublereal,
                                    &raw mut m,
                                    work.offset(
                                        (m as ::core::ffi::c_long * m as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    info,
                                );
                                dlacpy__0(
                                    b"A\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    n,
                                    &raw mut m,
                                    z__.offset(
                                        (start as ::core::ffi::c_long
                                            * z_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    ldz,
                                    work.offset(storez as isize) as *mut doublereal,
                                    n,
                                );
                                f2c_dgemm_0(
                                    b"N\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"N\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    n,
                                    &raw mut m,
                                    &raw mut m,
                                    &raw mut c_b18,
                                    work.offset(storez as isize) as *mut doublereal,
                                    n,
                                    work.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut doublereal,
                                    &raw mut m,
                                    &raw mut c_b17,
                                    z__.offset(
                                        (start as ::core::ffi::c_long
                                            * z_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    ldz,
                                );
                            } else if icompz == 2 as ::core::ffi::c_long {
                                dsteqr__0(
                                    b"I\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    &raw mut m,
                                    d__.offset(start as isize) as *mut doublereal,
                                    e.offset(start as isize) as *mut doublereal,
                                    z__.offset((start + start * z_dim1) as isize)
                                        as *mut doublereal,
                                    ldz,
                                    work.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut doublereal,
                                    info,
                                );
                            } else {
                                dsterf__0(
                                    &raw mut m,
                                    d__.offset(start as isize) as *mut doublereal,
                                    e.offset(start as isize) as *mut doublereal,
                                    info,
                                );
                            }
                            if *info != 0 as ::core::ffi::c_long {
                                *info = start * (*n + 1 as integer) + finish;
                                break;
                            }
                        }
                        start =
                            (finish as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    }
                } else {
                    if m != *n {
                        if icompz == 0 as ::core::ffi::c_long {
                            dlasrt__0(
                                b"I\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                n,
                                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                                info,
                            );
                        } else {
                            i__1 = *n;
                            ii = 2 as integer;
                            while ii <= i__1 {
                                i__ = (ii as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                    as integer;
                                k = i__;
                                p = *d__.offset(i__ as isize);
                                i__2 = *n;
                                j = ii;
                                while j <= i__2 {
                                    if *d__.offset(j as isize) < p {
                                        k = j;
                                        p = *d__.offset(j as isize);
                                    }
                                    j += 1;
                                }
                                if k != i__ {
                                    *d__.offset(k as isize) = *d__.offset(i__ as isize);
                                    *d__.offset(i__ as isize) = p;
                                    f2c_dswap_0(
                                        n,
                                        z__.offset(
                                            (i__ as ::core::ffi::c_long
                                                * z_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                        z__.offset(
                                            (k as ::core::ffi::c_long
                                                * z_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                }
                                ii += 1;
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = lwmin as doublereal;
    *iwork.offset(1 as ::core::ffi::c_int as isize) = liwmin;
    return 0 as ::core::ffi::c_int;
}
