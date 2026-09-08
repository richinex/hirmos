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
static mut c__0: integer = 0 as integer;
static mut c__1: integer = 1 as integer;
static mut c_b32: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dsterf_(
    mut n: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
    }
    let mut c__: doublereal = 0.;
    let mut i__: integer = 0;
    let mut l: integer = 0;
    let mut m: integer = 0;
    let mut p: doublereal = 0.;
    let mut r__: doublereal = 0.;
    let mut s: doublereal = 0.;
    let mut l1: integer = 0;
    let mut bb: doublereal = 0.;
    let mut rt1: doublereal = 0.;
    let mut rt2: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut rte: doublereal = 0.;
    let mut lsv: integer = 0;
    let mut eps2: doublereal = 0.;
    let mut oldc: doublereal = 0.;
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
    let mut gamma: doublereal = 0.;
    let mut alpha: doublereal = 0.;
    let mut sigma: doublereal = 0.;
    let mut anorm: doublereal = 0.;
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
    let mut oldgam: doublereal = 0.;
    let mut safmin: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut safmax: doublereal = 0.;
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
    let mut ssfmax: doublereal = 0.;
    e = e.offset(-1);
    d__ = d__.offset(-1);
    *info = 0 as integer;
    if *n < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
        i__1 = -*info;
        xerbla__0(
            b"DSTERF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n <= 1 as ::core::ffi::c_long {
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
    nmaxit = (*n * 30 as ::core::ffi::c_long) as integer;
    sigma = 0.0f64 as doublereal;
    jtot = 0 as integer;
    l1 = 1 as integer;
    loop {
        if l1 > *n {
            dlasrt__0(
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                info,
            );
            break;
        } else {
            if l1 > 1 as ::core::ffi::c_long {
                *e.offset((l1 as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                    0.0f64 as doublereal;
            }
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            m = l1;
            loop {
                if !(m <= i__1) {
                    current_block = 8704759739624374314;
                    break;
                }
                d__3 = *e.offset(m as isize);
                d__1 = *d__.offset(m as isize);
                d__2 = *d__.offset((m as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                if (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__3 as ::core::ffi::c_double
                } else {
                    -(d__3 as ::core::ffi::c_double)
                }) <= sqrt(
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
                    current_block = 1484043903557446564;
                    break;
                } else {
                    m += 1;
                }
            }
            match current_block {
                8704759739624374314 => {
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
            i__1 = (lend as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            i__ = l;
            while i__ <= i__1 {
                d__1 = *e.offset(i__ as isize);
                *e.offset(i__ as isize) = d__1 * d__1;
                i__ += 1;
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
            if lend >= l {
                loop {
                    if l != lend {
                        i__1 = (lend as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        m = l;
                        loop {
                            if !(m <= i__1) {
                                current_block = 317151059986244064;
                                break;
                            }
                            d__2 = *e.offset(m as isize);
                            d__1 = *d__.offset(m as isize)
                                * *d__.offset(
                                    (m as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                                );
                            if (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__2 as ::core::ffi::c_double
                            } else {
                                -(d__2 as ::core::ffi::c_double)
                            }) <= eps2 as ::core::ffi::c_double
                                * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    -(d__1 as ::core::ffi::c_double)
                                })
                            {
                                current_block = 3973111354378468622;
                                break;
                            }
                            m += 1;
                        }
                    } else {
                        current_block = 317151059986244064;
                    }
                    match current_block {
                        317151059986244064 => {
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
                        rte = sqrt(*e.offset(l as isize)) as doublereal;
                        dlae2__0(
                            d__.offset(l as isize) as *mut doublereal,
                            &raw mut rte,
                            d__.offset(
                                (l as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                            ) as *mut doublereal,
                            &raw mut rt1,
                            &raw mut rt2,
                        );
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
                        rte = sqrt(*e.offset(l as isize)) as doublereal;
                        sigma = ((*d__
                            .offset((l as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                            as ::core::ffi::c_double
                            - p as ::core::ffi::c_double)
                            / (rte as ::core::ffi::c_double * 2.0f64))
                            as doublereal;
                        r__ = dlapy2__0(&raw mut sigma, &raw mut c_b32);
                        sigma = (p as ::core::ffi::c_double
                            - rte as ::core::ffi::c_double
                                / (sigma as ::core::ffi::c_double
                                    + d_sign_0(&raw mut r__, &raw mut sigma)))
                            as doublereal;
                        c__ = 1.0f64 as doublereal;
                        s = 0.0f64 as doublereal;
                        gamma = *d__.offset(m as isize) - sigma;
                        p = gamma * gamma;
                        i__1 = l;
                        i__ = (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        while i__ >= i__1 {
                            bb = *e.offset(i__ as isize);
                            r__ = p + bb;
                            if i__ != m as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                                *e.offset(
                                    (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) = s * r__;
                            }
                            oldc = c__;
                            c__ = p / r__;
                            s = bb / r__;
                            oldgam = gamma;
                            alpha = *d__.offset(i__ as isize);
                            gamma = c__ * (alpha - sigma) - s * oldgam;
                            *d__.offset(
                                (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                            ) = oldgam + (alpha - gamma);
                            if c__ != 0.0f64 {
                                p = gamma * gamma / c__;
                            } else {
                                p = oldc * bb;
                            }
                            i__ -= 1;
                        }
                        *e.offset(l as isize) = s * p;
                        *d__.offset(l as isize) = sigma + gamma;
                    }
                }
            } else {
                loop {
                    i__1 = (lend as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    m = l;
                    loop {
                        if !(m >= i__1) {
                            current_block = 7639320476250304355;
                            break;
                        }
                        d__2 = *e
                            .offset((m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize);
                        d__1 = *d__.offset(m as isize)
                            * *d__.offset(
                                (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                        if (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) <= eps2 as ::core::ffi::c_double
                            * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            })
                        {
                            current_block = 12205351494314812295;
                            break;
                        }
                        m -= 1;
                    }
                    match current_block {
                        7639320476250304355 => {
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
                        rte =
                            sqrt(*e.offset(
                                (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            )) as doublereal;
                        dlae2__0(
                            d__.offset(l as isize) as *mut doublereal,
                            &raw mut rte,
                            d__.offset(
                                (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) as *mut doublereal,
                            &raw mut rt1,
                            &raw mut rt2,
                        );
                        *d__.offset(l as isize) = rt1;
                        *d__.offset(
                            (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) = rt2;
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
                        rte =
                            sqrt(*e.offset(
                                (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            )) as doublereal;
                        sigma = ((*d__
                            .offset((l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                            as ::core::ffi::c_double
                            - p as ::core::ffi::c_double)
                            / (rte as ::core::ffi::c_double * 2.0f64))
                            as doublereal;
                        r__ = dlapy2__0(&raw mut sigma, &raw mut c_b32);
                        sigma = (p as ::core::ffi::c_double
                            - rte as ::core::ffi::c_double
                                / (sigma as ::core::ffi::c_double
                                    + d_sign_0(&raw mut r__, &raw mut sigma)))
                            as doublereal;
                        c__ = 1.0f64 as doublereal;
                        s = 0.0f64 as doublereal;
                        gamma = *d__.offset(m as isize) - sigma;
                        p = gamma * gamma;
                        i__1 = (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        i__ = m;
                        while i__ <= i__1 {
                            bb = *e.offset(i__ as isize);
                            r__ = p + bb;
                            if i__ != m {
                                *e.offset(
                                    (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as isize,
                                ) = s * r__;
                            }
                            oldc = c__;
                            c__ = p / r__;
                            s = bb / r__;
                            oldgam = gamma;
                            alpha = *d__.offset(
                                (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                            );
                            gamma = c__ * (alpha - sigma) - s * oldgam;
                            *d__.offset(i__ as isize) = oldgam + (alpha - gamma);
                            if c__ != 0.0f64 {
                                p = gamma * gamma / c__;
                            } else {
                                p = oldc * bb;
                            }
                            i__ += 1;
                        }
                        *e.offset((l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                            s * p;
                        *d__.offset(l as isize) = sigma + gamma;
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
            }
            if iscale == 2 as ::core::ffi::c_long {
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
