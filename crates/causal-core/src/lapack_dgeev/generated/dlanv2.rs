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
pub type doublereal = ::core::ffi::c_double;
static mut c_b4: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlanv2_(
    mut a: *mut doublereal,
    mut b: *mut doublereal,
    mut c__: *mut doublereal,
    mut d__: *mut doublereal,
    mut rt1r: *mut doublereal,
    mut rt1i: *mut doublereal,
    mut rt2r: *mut doublereal,
    mut rt2i: *mut doublereal,
    mut cs: *mut doublereal,
    mut sn: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    extern "C" {
        #[link_name = "dgeev_closure_d_sign"]
        fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
    }
    let mut p: doublereal = 0.;
    let mut z__: doublereal = 0.;
    let mut aa: doublereal = 0.;
    let mut bb: doublereal = 0.;
    let mut cc: doublereal = 0.;
    let mut dd: doublereal = 0.;
    let mut cs1: doublereal = 0.;
    let mut sn1: doublereal = 0.;
    let mut sab: doublereal = 0.;
    let mut sac: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut tau: doublereal = 0.;
    let mut temp: doublereal = 0.;
    let mut scale: doublereal = 0.;
    let mut bcmax: doublereal = 0.;
    let mut bcmis: doublereal = 0.;
    let mut sigma: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_dlapy2_"]
        fn dlapy2__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    eps = dlamch__0(b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char);
    if *c__ == 0.0f64 {
        *cs = 1.0f64 as doublereal;
        *sn = 0.0f64 as doublereal;
    } else if *b == 0.0f64 {
        *cs = 0.0f64 as doublereal;
        *sn = 1.0f64 as doublereal;
        temp = *d__;
        *d__ = *a;
        *a = temp;
        *b = -*c__;
        *c__ = 0.0f64 as doublereal;
    } else if *a - *d__ == 0.0f64 && d_sign_0(&raw mut c_b4, b) != d_sign_0(&raw mut c_b4, c__) {
        *cs = 1.0f64 as doublereal;
        *sn = 0.0f64 as doublereal;
    } else {
        temp = *a - *d__;
        p = (temp as ::core::ffi::c_double * 0.5f64) as doublereal;
        d__1 = (if *b >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *b
        } else {
            -*b
        }) as doublereal;
        d__2 = (if *c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *c__
        } else {
            -*c__
        }) as doublereal;
        bcmax = (if d__1 >= d__2 {
            d__1 as ::core::ffi::c_double
        } else {
            d__2 as ::core::ffi::c_double
        }) as doublereal;
        d__1 = (if *b >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *b
        } else {
            -*b
        }) as doublereal;
        d__2 = (if *c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *c__
        } else {
            -*c__
        }) as doublereal;
        bcmis = ((if d__1 <= d__2 {
            d__1 as ::core::ffi::c_double
        } else {
            d__2 as ::core::ffi::c_double
        }) * d_sign_0(&raw mut c_b4, b)
            * d_sign_0(&raw mut c_b4, c__)) as doublereal;
        d__1 = (if p >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            p as ::core::ffi::c_double
        } else {
            -(p as ::core::ffi::c_double)
        }) as doublereal;
        scale = (if d__1 >= bcmax {
            d__1 as ::core::ffi::c_double
        } else {
            bcmax as ::core::ffi::c_double
        }) as doublereal;
        z__ = p / scale * p + bcmax / scale * bcmis;
        if z__ >= eps as ::core::ffi::c_double * 4.0f64 {
            d__1 = (sqrt(scale) * sqrt(z__)) as doublereal;
            z__ = (p as ::core::ffi::c_double + d_sign_0(&raw mut d__1, &raw mut p)) as doublereal;
            *a = *d__ + z__;
            *d__ -= (bcmax / z__ * bcmis) as ::core::ffi::c_double;
            tau = dlapy2__0(c__, &raw mut z__);
            *cs = z__ / tau;
            *sn = *c__ / tau;
            *b -= *c__ as ::core::ffi::c_double;
            *c__ = 0.0f64 as doublereal;
        } else {
            sigma = *b + *c__;
            tau = dlapy2__0(&raw mut sigma, &raw mut temp);
            *cs = sqrt(
                ((if sigma >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    sigma
                } else {
                    -sigma
                }) / tau
                    + 1.0f64)
                    * 0.5f64,
            ) as doublereal;
            *sn = (-(p as ::core::ffi::c_double / (tau as ::core::ffi::c_double * *cs))
                * d_sign_0(&raw mut c_b4, &raw mut sigma)) as doublereal;
            aa = *a * *cs + *b * *sn;
            bb = -*a * *sn + *b * *cs;
            cc = *c__ * *cs + *d__ * *sn;
            dd = -*c__ * *sn + *d__ * *cs;
            *a = aa * *cs + cc * *sn;
            *b = bb * *cs + dd * *sn;
            *c__ = -aa * *sn + cc * *cs;
            *d__ = -bb * *sn + dd * *cs;
            temp = ((*a + *d__) * 0.5f64) as doublereal;
            *a = temp;
            *d__ = temp;
            if *c__ != 0.0f64 {
                if *b != 0.0f64 {
                    if d_sign_0(&raw mut c_b4, b) == d_sign_0(&raw mut c_b4, c__) {
                        sab = sqrt(if *b >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            *b
                        } else {
                            -*b
                        }) as doublereal;
                        sac = sqrt(
                            if *c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                *c__
                            } else {
                                -*c__
                            },
                        ) as doublereal;
                        d__1 = sab * sac;
                        p = d_sign_0(&raw mut d__1, c__) as doublereal;
                        d__1 = *b + *c__;
                        tau = (1.0f64
                            / sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            )) as doublereal;
                        *a = temp + p;
                        *d__ = temp - p;
                        *b -= *c__ as ::core::ffi::c_double;
                        *c__ = 0.0f64 as doublereal;
                        cs1 = sab * tau;
                        sn1 = sac * tau;
                        temp = *cs * cs1 - *sn * sn1;
                        *sn = *cs * sn1 + *sn * cs1;
                        *cs = temp;
                    }
                } else {
                    *b = -*c__;
                    *c__ = 0.0f64 as doublereal;
                    temp = *cs;
                    *cs = -*sn;
                    *sn = temp;
                }
            }
        }
    }
    *rt1r = *a;
    *rt2r = *d__;
    if *c__ == 0.0f64 {
        *rt1i = 0.0f64 as doublereal;
        *rt2i = 0.0f64 as doublereal;
    } else {
        *rt1i = (sqrt(
            (if *b >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *b
            } else {
                -*b
            }),
        ) * sqrt(
            (if *c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *c__
            } else {
                -*c__
            }),
        )) as doublereal;
        *rt2i = -*rt1i;
    }
    return 0 as ::core::ffi::c_int;
}
