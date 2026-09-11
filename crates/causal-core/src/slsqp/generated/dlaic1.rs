extern "C" {
    fn sqrt(_: doublereal) -> ::core::ffi::c_double;
}
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
static mut c__1: integer = 1 as integer;
static mut c_b5: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlaic1_(
    mut job: *mut integer,
    mut j: *mut integer,
    mut x: *mut doublereal,
    mut sest: *mut doublereal,
    mut w: *mut doublereal,
    mut gamma: *mut doublereal,
    mut sestpr: *mut doublereal,
    mut s: *mut doublereal,
    mut c__: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__4: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn dgelsd_closure_d_sign_0(_: *mut doublereal, _: *mut doublereal)
            -> ::core::ffi::c_double;
    }
    let mut b: doublereal = 0.;
    let mut t: doublereal = 0.;
    let mut s1: doublereal = 0.;
    let mut s2: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut tmp: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_ddot"]
        fn dgelsd_closure_f2c_ddot_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> doublereal;
    }
    let mut sine: doublereal = 0.;
    let mut test: doublereal = 0.;
    let mut zeta1: doublereal = 0.;
    let mut zeta2: doublereal = 0.;
    let mut alpha: doublereal = 0.;
    let mut norma: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dgelsd_closure_dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    let mut absgam: doublereal = 0.;
    let mut absalp: doublereal = 0.;
    let mut cosine: doublereal = 0.;
    let mut absest: doublereal = 0.;
    w = w.offset(-1);
    x = x.offset(-1);
    eps = dgelsd_closure_dlamch__0(
        b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    alpha = dgelsd_closure_f2c_ddot_0(
        j,
        x.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
        w.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
    );
    absalp = (if alpha >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        alpha as ::core::ffi::c_double
    } else {
        -(alpha as ::core::ffi::c_double)
    }) as doublereal;
    absgam = (if *gamma >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *gamma
    } else {
        -*gamma
    }) as doublereal;
    absest = (if *sest >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *sest
    } else {
        -*sest
    }) as doublereal;
    if *job == 1 as ::core::ffi::c_long {
        if *sest == 0.0f64 {
            s1 = (if absgam >= absalp {
                absgam as ::core::ffi::c_double
            } else {
                absalp as ::core::ffi::c_double
            }) as doublereal;
            if s1 == 0.0f64 {
                *s = 0.0f64 as doublereal;
                *c__ = 1.0f64 as doublereal;
                *sestpr = 0.0f64 as doublereal;
            } else {
                *s = alpha / s1;
                *c__ = *gamma / s1;
                tmp = sqrt(*s * *s + *c__ * *c__) as doublereal;
                *s /= tmp as ::core::ffi::c_double;
                *c__ /= tmp as ::core::ffi::c_double;
                *sestpr = s1 * tmp;
            }
            return 0 as ::core::ffi::c_int;
        } else if absgam <= eps * absest {
            *s = 1.0f64 as doublereal;
            *c__ = 0.0f64 as doublereal;
            tmp = (if absest >= absalp {
                absest as ::core::ffi::c_double
            } else {
                absalp as ::core::ffi::c_double
            }) as doublereal;
            s1 = absest / tmp;
            s2 = absalp / tmp;
            *sestpr = (tmp as ::core::ffi::c_double * sqrt(s1 * s1 + s2 * s2)) as doublereal;
            return 0 as ::core::ffi::c_int;
        } else if absalp <= eps * absest {
            s1 = absgam;
            s2 = absest;
            if s1 <= s2 {
                *s = 1.0f64 as doublereal;
                *c__ = 0.0f64 as doublereal;
                *sestpr = s2;
            } else {
                *s = 0.0f64 as doublereal;
                *c__ = 1.0f64 as doublereal;
                *sestpr = s1;
            }
            return 0 as ::core::ffi::c_int;
        } else if absest <= eps * absalp || absest <= eps * absgam {
            s1 = absgam;
            s2 = absalp;
            if s1 <= s2 {
                tmp = s1 / s2;
                *s = sqrt(tmp * tmp + 1.0f64) as doublereal;
                *sestpr = s2 * *s;
                *c__ = *gamma / s2 / *s;
                *s = dgelsd_closure_d_sign_0(&raw mut c_b5, &raw mut alpha) as doublereal / *s;
            } else {
                tmp = s2 / s1;
                *c__ = sqrt(tmp * tmp + 1.0f64) as doublereal;
                *sestpr = s1 * *c__;
                *s = alpha / s1 / *c__;
                *c__ = dgelsd_closure_d_sign_0(&raw mut c_b5, gamma) as doublereal / *c__;
            }
            return 0 as ::core::ffi::c_int;
        } else {
            zeta1 = alpha / absest;
            zeta2 = *gamma / absest;
            b = ((1.0f64
                - zeta1 as ::core::ffi::c_double * zeta1 as ::core::ffi::c_double
                - zeta2 as ::core::ffi::c_double * zeta2 as ::core::ffi::c_double)
                * 0.5f64) as doublereal;
            *c__ = zeta1 * zeta1;
            if b > 0.0f64 {
                t = (*c__ / (b as ::core::ffi::c_double + sqrt(b * b + *c__))) as doublereal;
            } else {
                t = sqrt(b * b + *c__) as doublereal - b;
            }
            sine = -zeta1 / t;
            cosine = (-(zeta2 as ::core::ffi::c_double) / (t as ::core::ffi::c_double + 1.0f64))
                as doublereal;
            tmp = sqrt(sine * sine + cosine * cosine) as doublereal;
            *s = sine / tmp;
            *c__ = cosine / tmp;
            *sestpr = sqrt(t + 1.0f64) as doublereal * absest;
            return 0 as ::core::ffi::c_int;
        }
    } else if *job == 2 as ::core::ffi::c_long {
        if *sest == 0.0f64 {
            *sestpr = 0.0f64 as doublereal;
            if (if absgam >= absalp {
                absgam as ::core::ffi::c_double
            } else {
                absalp as ::core::ffi::c_double
            }) == 0.0f64
            {
                sine = 1.0f64 as doublereal;
                cosine = 0.0f64 as doublereal;
            } else {
                sine = -*gamma;
                cosine = alpha;
            }
            d__1 = (if sine >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                sine as ::core::ffi::c_double
            } else {
                -(sine as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = (if cosine >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                cosine as ::core::ffi::c_double
            } else {
                -(cosine as ::core::ffi::c_double)
            }) as doublereal;
            s1 = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            *s = sine / s1;
            *c__ = cosine / s1;
            tmp = sqrt(*s * *s + *c__ * *c__) as doublereal;
            *s /= tmp as ::core::ffi::c_double;
            *c__ /= tmp as ::core::ffi::c_double;
            return 0 as ::core::ffi::c_int;
        } else if absgam <= eps * absest {
            *s = 0.0f64 as doublereal;
            *c__ = 1.0f64 as doublereal;
            *sestpr = absgam;
            return 0 as ::core::ffi::c_int;
        } else if absalp <= eps * absest {
            s1 = absgam;
            s2 = absest;
            if s1 <= s2 {
                *s = 0.0f64 as doublereal;
                *c__ = 1.0f64 as doublereal;
                *sestpr = s1;
            } else {
                *s = 1.0f64 as doublereal;
                *c__ = 0.0f64 as doublereal;
                *sestpr = s2;
            }
            return 0 as ::core::ffi::c_int;
        } else if absest <= eps * absalp || absest <= eps * absgam {
            s1 = absgam;
            s2 = absalp;
            if s1 <= s2 {
                tmp = s1 / s2;
                *c__ = sqrt(tmp * tmp + 1.0f64) as doublereal;
                *sestpr = absest * (tmp / *c__);
                *s = -(*gamma / s2) / *c__;
                *c__ = dgelsd_closure_d_sign_0(&raw mut c_b5, &raw mut alpha) as doublereal / *c__;
            } else {
                tmp = s2 / s1;
                *s = sqrt(tmp * tmp + 1.0f64) as doublereal;
                *sestpr = absest / *s;
                *c__ = alpha / s1 / *s;
                *s = -(dgelsd_closure_d_sign_0(&raw mut c_b5, gamma) as doublereal) / *s;
            }
            return 0 as ::core::ffi::c_int;
        } else {
            zeta1 = alpha / absest;
            zeta2 = *gamma / absest;
            d__1 = zeta1 * zeta2;
            d__3 = (zeta1 as ::core::ffi::c_double * zeta1 as ::core::ffi::c_double
                + 1.0f64
                + (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                })) as doublereal;
            d__2 = zeta1 * zeta2;
            d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2
            } else {
                -d__2
            }) + zeta2 * zeta2;
            norma = (if d__3 >= d__4 {
                d__3 as ::core::ffi::c_double
            } else {
                d__4 as ::core::ffi::c_double
            }) as doublereal;
            test = ((zeta1 as ::core::ffi::c_double - zeta2 as ::core::ffi::c_double)
                * 2.0f64
                * (zeta1 as ::core::ffi::c_double + zeta2 as ::core::ffi::c_double)
                + 1.0f64) as doublereal;
            if test >= 0.0f64 {
                b = ((zeta1 as ::core::ffi::c_double * zeta1 as ::core::ffi::c_double
                    + zeta2 as ::core::ffi::c_double * zeta2 as ::core::ffi::c_double
                    + 1.0f64)
                    * 0.5f64) as doublereal;
                *c__ = zeta2 * zeta2;
                d__1 = b * b - *c__;
                t = (*c__
                    / (b as ::core::ffi::c_double
                        + sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))) as doublereal;
                sine = zeta1 / (1.0f64 - t);
                cosine = -zeta2 / t;
                *sestpr = sqrt(t + eps * 4.0f64 * eps * norma) as doublereal * absest;
            } else {
                b = ((zeta2 as ::core::ffi::c_double * zeta2 as ::core::ffi::c_double
                    + zeta1 as ::core::ffi::c_double * zeta1 as ::core::ffi::c_double
                    - 1.0f64)
                    * 0.5f64) as doublereal;
                *c__ = zeta1 * zeta1;
                if b >= 0.0f64 {
                    t = (-*c__ / (b as ::core::ffi::c_double + sqrt(b * b + *c__))) as doublereal;
                } else {
                    t = (b as ::core::ffi::c_double - sqrt(b * b + *c__)) as doublereal;
                }
                sine = -zeta1 / t;
                cosine = (-(zeta2 as ::core::ffi::c_double) / (t as ::core::ffi::c_double + 1.0f64))
                    as doublereal;
                *sestpr = sqrt(t + 1.0f64 + eps * 4.0f64 * eps * norma) as doublereal * absest;
            }
            tmp = sqrt(sine * sine + cosine * cosine) as doublereal;
            *s = sine / tmp;
            *c__ = cosine / tmp;
            return 0 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
