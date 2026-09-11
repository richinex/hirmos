extern "C" {
    fn sqrt(_: doublereal) -> ::core::ffi::c_double;
    fn log(_: doublereal) -> ::core::ffi::c_double;
}
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
static mut c_b6: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlartgp_(
    mut f: *mut doublereal,
    mut g: *mut doublereal,
    mut cs: *mut doublereal,
    mut sn: *mut doublereal,
    mut r__: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_pow_di"]
        fn dgelsd_closure_pow_di_0(_: *mut doublereal, _: *mut integer) -> ::core::ffi::c_double;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn dgelsd_closure_d_sign_0(_: *mut doublereal, _: *mut doublereal)
            -> ::core::ffi::c_double;
    }
    let mut i__: integer = 0;
    let mut f1: doublereal = 0.;
    let mut g1: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut scale: doublereal = 0.;
    let mut count: integer = 0;
    let mut safmn2: doublereal = 0.;
    let mut safmx2: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dgelsd_closure_dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    let mut safmin: doublereal = 0.;
    safmin = dgelsd_closure_dlamch__0(
        b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    eps = dgelsd_closure_dlamch__0(
        b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    d__1 = dgelsd_closure_dlamch__0(
        b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    i__1 = (log(safmin / eps)
        / log(dgelsd_closure_dlamch__0(
            b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ))
        / 2.0f64) as integer;
    safmn2 = dgelsd_closure_pow_di_0(&raw mut d__1, &raw mut i__1) as doublereal;
    safmx2 = 1.0f64 / safmn2;
    if *g == 0.0f64 {
        *cs = dgelsd_closure_d_sign_0(&raw mut c_b6, f) as doublereal;
        *sn = 0.0f64 as doublereal;
        *r__ = (if *f >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *f
        } else {
            -*f
        }) as doublereal;
    } else if *f == 0.0f64 {
        *cs = 0.0f64 as doublereal;
        *sn = dgelsd_closure_d_sign_0(&raw mut c_b6, g) as doublereal;
        *r__ = (if *g >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *g
        } else {
            -*g
        }) as doublereal;
    } else {
        f1 = *f;
        g1 = *g;
        d__1 = (if f1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            f1 as ::core::ffi::c_double
        } else {
            -(f1 as ::core::ffi::c_double)
        }) as doublereal;
        d__2 = (if g1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            g1 as ::core::ffi::c_double
        } else {
            -(g1 as ::core::ffi::c_double)
        }) as doublereal;
        scale = (if d__1 >= d__2 {
            d__1 as ::core::ffi::c_double
        } else {
            d__2 as ::core::ffi::c_double
        }) as doublereal;
        if scale >= safmx2 {
            count = 0 as integer;
            loop {
                count += 1;
                f1 *= safmn2 as ::core::ffi::c_double;
                g1 *= safmn2 as ::core::ffi::c_double;
                d__1 = (if f1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    f1 as ::core::ffi::c_double
                } else {
                    -(f1 as ::core::ffi::c_double)
                }) as doublereal;
                d__2 = (if g1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    g1 as ::core::ffi::c_double
                } else {
                    -(g1 as ::core::ffi::c_double)
                }) as doublereal;
                scale = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                if !(scale >= safmx2 && count < 20 as ::core::ffi::c_long) {
                    break;
                }
            }
            d__1 = f1;
            d__2 = g1;
            *r__ = sqrt(d__1 * d__1 + d__2 * d__2) as doublereal;
            *cs = f1 / *r__;
            *sn = g1 / *r__;
            i__1 = count;
            i__ = 1 as integer;
            while i__ <= i__1 {
                *r__ *= safmx2 as ::core::ffi::c_double;
                i__ += 1;
            }
        } else if scale <= safmn2 {
            count = 0 as integer;
            loop {
                count += 1;
                f1 *= safmx2 as ::core::ffi::c_double;
                g1 *= safmx2 as ::core::ffi::c_double;
                d__1 = (if f1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    f1 as ::core::ffi::c_double
                } else {
                    -(f1 as ::core::ffi::c_double)
                }) as doublereal;
                d__2 = (if g1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    g1 as ::core::ffi::c_double
                } else {
                    -(g1 as ::core::ffi::c_double)
                }) as doublereal;
                scale = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                if !(scale <= safmn2) {
                    break;
                }
            }
            d__1 = f1;
            d__2 = g1;
            *r__ = sqrt(d__1 * d__1 + d__2 * d__2) as doublereal;
            *cs = f1 / *r__;
            *sn = g1 / *r__;
            i__1 = count;
            i__ = 1 as integer;
            while i__ <= i__1 {
                *r__ *= safmn2 as ::core::ffi::c_double;
                i__ += 1;
            }
        } else {
            d__1 = f1;
            d__2 = g1;
            *r__ = sqrt(d__1 * d__1 + d__2 * d__2) as doublereal;
            *cs = f1 / *r__;
            *sn = g1 / *r__;
        }
        if *r__ < 0.0f64 {
            *cs = -*cs;
            *sn = -*sn;
            *r__ = -*r__;
        }
    }
    return 0 as ::core::ffi::c_int;
}
