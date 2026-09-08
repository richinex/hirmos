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
static mut c_b9: doublereal = 0.0f64;
static mut c_b10: doublereal = 1.0f64;
static mut c__0: integer = 0 as integer;
static mut c__1: integer = 1 as integer;
static mut c__2: integer = 2 as integer;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dsteqr_(
    mut compz: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut z__: *mut doublereal,
    mut ldz: *mut integer,
    mut work: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut z_dim1: integer = 0;
    let mut z_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
    }
    let mut b: doublereal = 0.;
    let mut c__: doublereal = 0.;
    let mut f: doublereal = 0.;
    let mut g: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut l: integer = 0;
    let mut m: integer = 0;
    let mut p: doublereal = 0.;
    let mut r__: doublereal = 0.;
    let mut s: doublereal = 0.;
    let mut l1: integer = 0;
    let mut ii: integer = 0;
    let mut mm: integer = 0;
    let mut lm1: integer = 0;
    let mut mm1: integer = 0;
    let mut nm1: integer = 0;
    let mut rt1: doublereal = 0.;
    let mut rt2: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut lsv: integer = 0;
    let mut tst: doublereal = 0.;
    let mut eps2: doublereal = 0.;
    let mut lend: integer = 0;
    let mut jtot: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dlae2_"]
        fn dlae2__0(
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlasr_"]
        fn dlasr__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut anorm: doublereal = 0.;
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
    extern "C" {
        #[link_name = "dsyevd_closure_dlaev2_"]
        fn dlaev2__0(
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    let mut lendm1: integer = 0;
    let mut lendp1: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlapy2_"]
        fn dlapy2__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
    }
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
    let mut safmin: doublereal = 0.;
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
    let mut safmax: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
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
        #[link_name = "dgelsd_closure_dlasrt_"]
        fn dlasrt__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut lendsv: integer = 0;
    let mut ssfmin: doublereal = 0.;
    let mut nmaxit: integer = 0;
    let mut icompz: integer = 0;
    let mut ssfmax: doublereal = 0.;
    d__ = d__.offset(-1);
    e = e.offset(-1);
    z_dim1 = *ldz;
    z_offset = 1 as integer + z_dim1;
    z__ = z__.offset(-(z_offset as isize));
    work = work.offset(-1);
    *info = 0 as integer;
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
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DSTEQR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *n == 1 as ::core::ffi::c_long {
        if icompz == 2 as ::core::ffi::c_long {
            *z__.offset((z_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                1.0f64 as doublereal;
        }
        return 0 as ::core::ffi::c_int;
    }
    eps = dlamch__0(b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char);
    d__1 = eps;
    eps2 = d__1 * d__1;
    safmin =
        dlamch__0(b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char);
    safmax = 1.0f64 / safmin;
    ssfmax = (sqrt(safmax) / 3.0f64) as doublereal;
    ssfmin = sqrt(safmin) as doublereal / eps2;
    if icompz == 2 as ::core::ffi::c_long {
        dlaset__0(
            b"Full\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            n,
            &raw mut c_b9,
            &raw mut c_b10,
            z__.offset(z_offset as isize) as *mut doublereal,
            ldz,
        );
    }
    nmaxit = (*n * 30 as ::core::ffi::c_long) as integer;
    jtot = 0 as integer;
    l1 = 1 as integer;
    nm1 = (*n - 1 as ::core::ffi::c_long) as integer;
    loop {
        if l1 > *n {
            if icompz == 0 as ::core::ffi::c_long {
                dlasrt__0(
                    b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n,
                    d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    info,
                );
            } else {
                i__1 = *n;
                ii = 2 as integer;
                while ii <= i__1 {
                    i__ = (ii as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
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
                                (i__ as ::core::ffi::c_long * z_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                            z__.offset(
                                (k as ::core::ffi::c_long * z_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                        );
                    }
                    ii += 1;
                }
            }
            break;
        } else {
            if l1 > 1 as ::core::ffi::c_long {
                *e.offset((l1 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                    0.0f64 as doublereal;
            }
            if l1 <= nm1 {
                i__1 = nm1;
                m = l1;
                loop {
                    if !(m <= i__1) {
                        current_block = 13125627826496529465;
                        break;
                    }
                    d__1 = *e.offset(m as isize);
                    tst = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                    if tst == 0.0f64 {
                        current_block = 10456737779699310722;
                        break;
                    }
                    d__1 = *d__.offset(m as isize);
                    d__2 =
                        *d__.offset((m as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                    if tst
                        <= sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ) as doublereal
                            * sqrt(
                                (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__2
                                } else {
                                    -d__2
                                }),
                            ) as doublereal
                            * eps
                    {
                        *e.offset(m as isize) = 0.0f64 as doublereal;
                        current_block = 10456737779699310722;
                        break;
                    } else {
                        m += 1;
                    }
                }
            } else {
                current_block = 13125627826496529465;
            }
            match current_block {
                13125627826496529465 => {
                    m = *n;
                }
                _ => {}
            }
            l = l1;
            lsv = l;
            lend = m;
            lendsv = lend;
            l1 = (m as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            if lend == l {
                continue;
            }
            i__1 = (lend as ::core::ffi::c_long - l as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            anorm = dlanst__0(
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
                d__.offset(l as isize) as *mut doublereal,
                e.offset(l as isize) as *mut doublereal,
            );
            iscale = 0 as integer;
            if anorm == 0.0f64 {
                continue;
            }
            if anorm > ssfmax {
                iscale = 1 as integer;
                i__1 = (lend as ::core::ffi::c_long - l as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut anorm,
                    &raw mut ssfmax,
                    &raw mut i__1,
                    &raw mut c__1,
                    d__.offset(l as isize) as *mut doublereal,
                    n,
                    info,
                );
                i__1 = lend - l;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut anorm,
                    &raw mut ssfmax,
                    &raw mut i__1,
                    &raw mut c__1,
                    e.offset(l as isize) as *mut doublereal,
                    n,
                    info,
                );
            } else if anorm < ssfmin {
                iscale = 2 as integer;
                i__1 = (lend as ::core::ffi::c_long - l as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut anorm,
                    &raw mut ssfmin,
                    &raw mut i__1,
                    &raw mut c__1,
                    d__.offset(l as isize) as *mut doublereal,
                    n,
                    info,
                );
                i__1 = lend - l;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut anorm,
                    &raw mut ssfmin,
                    &raw mut i__1,
                    &raw mut c__1,
                    e.offset(l as isize) as *mut doublereal,
                    n,
                    info,
                );
            }
            d__1 = *d__.offset(lend as isize);
            d__2 = *d__.offset(l as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) < (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            }) {
                lend = lsv;
                l = lendsv;
            }
            if lend > l {
                loop {
                    if l != lend {
                        lendm1 =
                            (lend as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        i__1 = lendm1;
                        m = l;
                        loop {
                            if !(m <= i__1) {
                                current_block = 18137396335907573669;
                                break;
                            }
                            d__1 = *e.offset(m as isize);
                            d__2 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) as doublereal;
                            tst = d__2 * d__2;
                            d__1 = *d__.offset(m as isize);
                            d__2 = *d__.offset(
                                (m as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                            );
                            if tst
                                <= eps2
                                    * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__1
                                    } else {
                                        -d__1
                                    })
                                    * (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__2
                                    } else {
                                        -d__2
                                    })
                                    + safmin
                            {
                                current_block = 17914251326417477582;
                                break;
                            }
                            m += 1;
                        }
                    } else {
                        current_block = 18137396335907573669;
                    }
                    match current_block {
                        18137396335907573669 => {
                            m = lend;
                        }
                        _ => {}
                    }
                    if m < lend {
                        *e.offset(m as isize) = 0.0f64 as doublereal;
                    }
                    p = *d__.offset(l as isize);
                    if m == l {
                        *d__.offset(l as isize) = p;
                        l += 1;
                        if !(l <= lend) {
                            break;
                        }
                    } else if m == l as ::core::ffi::c_long + 1 as ::core::ffi::c_long {
                        if icompz > 0 as ::core::ffi::c_long {
                            dlaev2__0(
                                d__.offset(l as isize) as *mut doublereal,
                                e.offset(l as isize) as *mut doublereal,
                                d__.offset(
                                    (l as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                                ) as *mut doublereal,
                                &raw mut rt1,
                                &raw mut rt2,
                                &raw mut c__,
                                &raw mut s,
                            );
                            *work.offset(l as isize) = c__;
                            *work.offset((*n - 1 as integer + l) as isize) = s;
                            dlasr__0(
                                b"R\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"V\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"B\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                n,
                                &raw mut c__2,
                                work.offset(l as isize) as *mut doublereal,
                                work.offset((*n - 1 as integer + l) as isize) as *mut doublereal,
                                z__.offset(
                                    (l as ::core::ffi::c_long * z_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                ldz,
                            );
                        } else {
                            dlae2__0(
                                d__.offset(l as isize) as *mut doublereal,
                                e.offset(l as isize) as *mut doublereal,
                                d__.offset(
                                    (l as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                                ) as *mut doublereal,
                                &raw mut rt1,
                                &raw mut rt2,
                            );
                        }
                        *d__.offset(l as isize) = rt1;
                        *d__.offset(
                            (l as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        ) = rt2;
                        *e.offset(l as isize) = 0.0f64 as doublereal;
                        l += 2 as ::core::ffi::c_long;
                        if !(l <= lend) {
                            break;
                        }
                    } else {
                        if jtot == nmaxit {
                            break;
                        }
                        jtot += 1;
                        g = ((*d__
                            .offset((l as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                            as ::core::ffi::c_double
                            - p as ::core::ffi::c_double)
                            / (*e.offset(l as isize) as ::core::ffi::c_double * 2.0f64))
                            as doublereal;
                        r__ = dlapy2__0(&raw mut g, &raw mut c_b10);
                        g = (*d__.offset(m as isize) as ::core::ffi::c_double
                            - p as ::core::ffi::c_double
                            + *e.offset(l as isize) as ::core::ffi::c_double
                                / (g as ::core::ffi::c_double + d_sign_0(&raw mut r__, &raw mut g)))
                            as doublereal;
                        s = 1.0f64 as doublereal;
                        c__ = 1.0f64 as doublereal;
                        p = 0.0f64 as doublereal;
                        mm1 = (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        i__1 = l;
                        i__ = mm1;
                        while i__ >= i__1 {
                            f = s * *e.offset(i__ as isize);
                            b = c__ * *e.offset(i__ as isize);
                            dlartg__0(
                                &raw mut g,
                                &raw mut f,
                                &raw mut c__,
                                &raw mut s,
                                &raw mut r__,
                            );
                            if i__ != m as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                                *e.offset(
                                    (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) = r__;
                            }
                            g = *d__.offset(
                                (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                            ) - p;
                            r__ = (*d__.offset(i__ as isize) - g) * s + c__ * 2.0f64 * b;
                            p = s * r__;
                            *d__.offset(
                                (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                            ) = g + p;
                            g = c__ * r__ - b;
                            if icompz > 0 as ::core::ffi::c_long {
                                *work.offset(i__ as isize) = c__;
                                *work.offset((*n - 1 as integer + i__) as isize) = -s;
                            }
                            i__ -= 1;
                        }
                        if icompz > 0 as ::core::ffi::c_long {
                            mm = (m as ::core::ffi::c_long - l as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long)
                                as integer;
                            dlasr__0(
                                b"R\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"V\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"B\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                n,
                                &raw mut mm,
                                work.offset(l as isize) as *mut doublereal,
                                work.offset((*n - 1 as integer + l) as isize) as *mut doublereal,
                                z__.offset(
                                    (l as ::core::ffi::c_long * z_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                ldz,
                            );
                        }
                        let ref mut fresh0 = *d__.offset(l as isize);
                        *fresh0 -= p as ::core::ffi::c_double;
                        *e.offset(l as isize) = g;
                    }
                }
            } else {
                loop {
                    if l != lend {
                        lendp1 =
                            (lend as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        i__1 = lendp1;
                        m = l;
                        loop {
                            if !(m >= i__1) {
                                current_block = 5250576585193495047;
                                break;
                            }
                            d__1 = *e.offset(
                                (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                            d__2 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) as doublereal;
                            tst = d__2 * d__2;
                            d__1 = *d__.offset(m as isize);
                            d__2 = *d__.offset(
                                (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                            if tst
                                <= eps2
                                    * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__1
                                    } else {
                                        -d__1
                                    })
                                    * (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__2
                                    } else {
                                        -d__2
                                    })
                                    + safmin
                            {
                                current_block = 5176173967456670406;
                                break;
                            }
                            m -= 1;
                        }
                    } else {
                        current_block = 5250576585193495047;
                    }
                    match current_block {
                        5250576585193495047 => {
                            m = lend;
                        }
                        _ => {}
                    }
                    if m > lend {
                        *e.offset((m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                            0.0f64 as doublereal;
                    }
                    p = *d__.offset(l as isize);
                    if m == l {
                        *d__.offset(l as isize) = p;
                        l -= 1;
                        if !(l >= lend) {
                            break;
                        }
                    } else if m == l as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                        if icompz > 0 as ::core::ffi::c_long {
                            dlaev2__0(
                                d__.offset(
                                    (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ) as *mut doublereal,
                                e.offset(
                                    (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ) as *mut doublereal,
                                d__.offset(l as isize) as *mut doublereal,
                                &raw mut rt1,
                                &raw mut rt2,
                                &raw mut c__,
                                &raw mut s,
                            );
                            *work.offset(m as isize) = c__;
                            *work.offset((*n - 1 as integer + m) as isize) = s;
                            dlasr__0(
                                b"R\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"V\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"F\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                n,
                                &raw mut c__2,
                                work.offset(m as isize) as *mut doublereal,
                                work.offset((*n - 1 as integer + m) as isize) as *mut doublereal,
                                z__.offset(
                                    ((l as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        * z_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                ldz,
                            );
                        } else {
                            dlae2__0(
                                d__.offset(
                                    (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ) as *mut doublereal,
                                e.offset(
                                    (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                                ) as *mut doublereal,
                                d__.offset(l as isize) as *mut doublereal,
                                &raw mut rt1,
                                &raw mut rt2,
                            );
                        }
                        *d__.offset(
                            (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) = rt1;
                        *d__.offset(l as isize) = rt2;
                        *e.offset((l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                            0.0f64 as doublereal;
                        l += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
                        if !(l >= lend) {
                            break;
                        }
                    } else {
                        if jtot == nmaxit {
                            break;
                        }
                        jtot += 1;
                        g = ((*d__
                            .offset((l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                            as ::core::ffi::c_double
                            - p as ::core::ffi::c_double)
                            / (*e.offset(
                                (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) as ::core::ffi::c_double
                                * 2.0f64)) as doublereal;
                        r__ = dlapy2__0(&raw mut g, &raw mut c_b10);
                        g = (*d__.offset(m as isize) as ::core::ffi::c_double
                            - p as ::core::ffi::c_double
                            + *e.offset(
                                (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) as ::core::ffi::c_double
                                / (g as ::core::ffi::c_double + d_sign_0(&raw mut r__, &raw mut g)))
                            as doublereal;
                        s = 1.0f64 as doublereal;
                        c__ = 1.0f64 as doublereal;
                        p = 0.0f64 as doublereal;
                        lm1 = (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        i__1 = lm1;
                        i__ = m;
                        while i__ <= i__1 {
                            f = s * *e.offset(i__ as isize);
                            b = c__ * *e.offset(i__ as isize);
                            dlartg__0(
                                &raw mut g,
                                &raw mut f,
                                &raw mut c__,
                                &raw mut s,
                                &raw mut r__,
                            );
                            if i__ != m {
                                *e.offset(
                                    (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as isize,
                                ) = r__;
                            }
                            g = *d__.offset(i__ as isize) - p;
                            r__ = (*d__.offset(
                                (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                            ) - g)
                                * s
                                + c__ * 2.0f64 * b;
                            p = s * r__;
                            *d__.offset(i__ as isize) = g + p;
                            g = c__ * r__ - b;
                            if icompz > 0 as ::core::ffi::c_long {
                                *work.offset(i__ as isize) = c__;
                                *work.offset((*n - 1 as integer + i__) as isize) = s;
                            }
                            i__ += 1;
                        }
                        if icompz > 0 as ::core::ffi::c_long {
                            mm = (l as ::core::ffi::c_long - m as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long)
                                as integer;
                            dlasr__0(
                                b"R\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"V\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"F\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                n,
                                &raw mut mm,
                                work.offset(m as isize) as *mut doublereal,
                                work.offset((*n - 1 as integer + m) as isize) as *mut doublereal,
                                z__.offset(
                                    (m as ::core::ffi::c_long * z_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                ldz,
                            );
                        }
                        let ref mut fresh1 = *d__.offset(l as isize);
                        *fresh1 -= p as ::core::ffi::c_double;
                        *e.offset(lm1 as isize) = g;
                    }
                }
            }
            if iscale == 1 as ::core::ffi::c_long {
                i__1 = (lendsv as ::core::ffi::c_long - lsv as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut ssfmax,
                    &raw mut anorm,
                    &raw mut i__1,
                    &raw mut c__1,
                    d__.offset(lsv as isize) as *mut doublereal,
                    n,
                    info,
                );
                i__1 = lendsv - lsv;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut ssfmax,
                    &raw mut anorm,
                    &raw mut i__1,
                    &raw mut c__1,
                    e.offset(lsv as isize) as *mut doublereal,
                    n,
                    info,
                );
            } else if iscale == 2 as ::core::ffi::c_long {
                i__1 = (lendsv as ::core::ffi::c_long - lsv as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut ssfmin,
                    &raw mut anorm,
                    &raw mut i__1,
                    &raw mut c__1,
                    d__.offset(lsv as isize) as *mut doublereal,
                    n,
                    info,
                );
                i__1 = lendsv - lsv;
                dlascl__0(
                    b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut c__0,
                    &raw mut c__0,
                    &raw mut ssfmin,
                    &raw mut anorm,
                    &raw mut i__1,
                    &raw mut c__1,
                    e.offset(lsv as isize) as *mut doublereal,
                    n,
                    info,
                );
            }
            if jtot < nmaxit {
                continue;
            }
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            i__ = 1 as integer;
            while i__ <= i__1 {
                if *e.offset(i__ as isize) != 0.0f64 {
                    *info += 1;
                }
                i__ += 1;
            }
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
