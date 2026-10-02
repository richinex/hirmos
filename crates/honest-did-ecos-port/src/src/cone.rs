use crate::runtime::{sqrt};
extern "C" {
    fn eddot(n: idxint, x: *mut pfloat, y: *mut pfloat) -> pfloat;
}
pub type pfloat = ::core::ffi::c_double;
pub type idxint = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lpcone {
    pub p: idxint,
    pub w: *mut pfloat,
    pub v: *mut pfloat,
    pub kkt_idx: *mut idxint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct socone {
    pub p: idxint,
    pub skbar: *mut pfloat,
    pub zkbar: *mut pfloat,
    pub a: pfloat,
    pub d1: pfloat,
    pub w: pfloat,
    pub eta: pfloat,
    pub eta_square: pfloat,
    pub q: *mut pfloat,
    pub Didx: *mut idxint,
    pub u0: pfloat,
    pub u1: pfloat,
    pub v1: pfloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cone {
    pub lpc: *mut lpcone,
    pub soc: *mut socone,
    pub nsoc: idxint,
}
pub const INSIDE_CONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OUTSIDE_CONE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GAMMA: ::core::ffi::c_double = 0.99f64;
pub const EPS: ::core::ffi::c_double = 1E-13f64;
#[no_mangle]
pub unsafe extern "C" fn socres(mut u: *mut pfloat, mut p: idxint) -> pfloat {
    let mut res: pfloat =
        *u.offset(0 as ::core::ffi::c_int as isize) * *u.offset(0 as ::core::ffi::c_int as isize);
    let mut i: idxint = 0;
    i = 1 as idxint;
    while i < p {
        res -= (*u.offset(i as isize) * *u.offset(i as isize)) as ::core::ffi::c_double;
        i += 1;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn bring2cone(mut C: *mut cone, mut r: *mut pfloat, mut s: *mut pfloat) {
    let mut alpha: pfloat = -GAMMA;
    let mut cres: pfloat = 0.;
    let mut r1square: pfloat = 0.;
    let mut i: idxint = 0;
    let mut l: idxint = 0;
    let mut j: idxint = 0;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        if *r.offset(i as isize) <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && -*r.offset(i as isize) > alpha
        {
            alpha = -*r.offset(i as isize);
        }
        i += 1;
    }
    l = 0 as idxint;
    while l < (*C).nsoc {
        let fresh0 = i;
        i = i + 1;
        cres = *r.offset(fresh0 as isize);
        r1square = 0 as ::core::ffi::c_int as pfloat;
        j = 1 as idxint;
        while j < (*(*C).soc.offset(l as isize)).p {
            r1square += (*r.offset(i as isize) * *r.offset(i as isize)) as ::core::ffi::c_double;
            i += 1;
            j += 1;
        }
        cres -= sqrt(r1square as ::core::ffi::c_double);
        if cres <= 0 as ::core::ffi::c_int as ::core::ffi::c_double && -cres > alpha {
            alpha = -cres;
        }
        l += 1;
    }
    alpha += 1.0f64;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        *s.offset(i as isize) = *r.offset(i as isize) + alpha;
        i += 1;
    }
    l = 0 as idxint;
    while l < (*C).nsoc {
        *s.offset(i as isize) = *r.offset(i as isize) + alpha;
        i += 1;
        j = 1 as idxint;
        while j < (*(*C).soc.offset(l as isize)).p {
            *s.offset(i as isize) = *r.offset(i as isize);
            i += 1;
            j += 1;
        }
        l += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn updateScalings(
    mut C: *mut cone,
    mut s: *mut pfloat,
    mut z: *mut pfloat,
    mut lambda: *mut pfloat,
) -> idxint {
    let mut i: idxint = 0;
    let mut l: idxint = 0;
    let mut k: idxint = 0;
    let mut p: idxint = 0;
    let mut sres: pfloat = 0.;
    let mut zres: pfloat = 0.;
    let mut snorm: pfloat = 0.;
    let mut znorm: pfloat = 0.;
    let mut gamma: pfloat = 0.;
    let mut one_over_2gamma: pfloat = 0.;
    let mut sk: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut zk: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut a: pfloat = 0.;
    let mut c: pfloat = 0.;
    let mut d: pfloat = 0.;
    let mut w: pfloat = 0.;
    let mut temp: pfloat = 0.;
    let mut divisor: pfloat = 0.;
    let mut u0: pfloat = 0.;
    let mut u0_square: pfloat = 0.;
    let mut u1: pfloat = 0.;
    let mut v1: pfloat = 0.;
    let mut d1: pfloat = 0.;
    let mut c2byu02_d: pfloat = 0.;
    let mut c2byu02: pfloat = 0.;
    let mut c_square: pfloat = 0.;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        *(*(*C).lpc).v.offset(i as isize) = (if *z.offset(i as isize) < EPS {
            *s.offset(i as isize) as ::core::ffi::c_double / EPS
        } else {
            *s.offset(i as isize) as ::core::ffi::c_double
                / *z.offset(i as isize) as ::core::ffi::c_double
        }) as pfloat;
        *(*(*C).lpc).w.offset(i as isize) =
            sqrt(*(*(*C).lpc).v.offset(i as isize) as ::core::ffi::c_double) as pfloat;
        i += 1;
    }
    k = (*(*C).lpc).p;
    l = 0 as idxint;
    while l < (*C).nsoc {
        sk = s.offset(k as isize);
        zk = z.offset(k as isize);
        p = (*(*C).soc.offset(l as isize)).p;
        sres = socres(sk, p);
        zres = socres(zk, p);
        if sres <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || zres <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            return OUTSIDE_CONE as idxint;
        }
        snorm = sqrt(sres as ::core::ffi::c_double) as pfloat;
        znorm = sqrt(zres as ::core::ffi::c_double) as pfloat;
        i = 0 as idxint;
        while i < p {
            *(*(*C).soc.offset(l as isize)).skbar.offset(i as isize) = (if snorm < EPS {
                *sk.offset(i as isize) as ::core::ffi::c_double / EPS
            } else {
                *sk.offset(i as isize) as ::core::ffi::c_double / snorm as ::core::ffi::c_double
            }) as pfloat;
            i += 1;
        }
        i = 0 as idxint;
        while i < p {
            *(*(*C).soc.offset(l as isize)).zkbar.offset(i as isize) = (if znorm < EPS {
                *zk.offset(i as isize) as ::core::ffi::c_double / EPS
            } else {
                *zk.offset(i as isize) as ::core::ffi::c_double / znorm as ::core::ffi::c_double
            }) as pfloat;
            i += 1;
        }
        (*(*C).soc.offset(l as isize)).eta_square = (if znorm < EPS {
            snorm as ::core::ffi::c_double / EPS
        } else {
            snorm as ::core::ffi::c_double / znorm as ::core::ffi::c_double
        }) as pfloat;
        (*(*C).soc.offset(l as isize)).eta =
            sqrt((*(*C).soc.offset(l as isize)).eta_square as ::core::ffi::c_double) as pfloat;
        gamma = 1.0f64 as pfloat;
        i = 0 as idxint;
        while i < p {
            gamma += (*(*(*C).soc.offset(l as isize)).skbar.offset(i as isize)
                * *(*(*C).soc.offset(l as isize)).zkbar.offset(i as isize))
                as ::core::ffi::c_double;
            i += 1;
        }
        gamma = sqrt(0.5f64 * gamma as ::core::ffi::c_double) as pfloat;
        one_over_2gamma = (if gamma < EPS {
            0.5f64 / EPS
        } else {
            0.5f64 / gamma as ::core::ffi::c_double
        }) as pfloat;
        a = one_over_2gamma
            * (*(*(*C).soc.offset(l as isize))
                .skbar
                .offset(0 as ::core::ffi::c_int as isize)
                + *(*(*C).soc.offset(l as isize))
                    .zkbar
                    .offset(0 as ::core::ffi::c_int as isize));
        w = 0 as ::core::ffi::c_int as pfloat;
        i = 1 as idxint;
        while i < p {
            *(*(*C).soc.offset(l as isize))
                .q
                .offset((i as i64 - 1 as i64) as isize) =
                one_over_2gamma
                    * (*(*(*C).soc.offset(l as isize)).skbar.offset(i as isize)
                        - *(*(*C).soc.offset(l as isize)).zkbar.offset(i as isize));
            w += (*(*(*C).soc.offset(l as isize))
                .q
                .offset((i as i64 - 1 as i64) as isize)
                * *(*(*C).soc.offset(l as isize))
                    .q
                    .offset((i as i64 - 1 as i64) as isize))
                as ::core::ffi::c_double;
            i += 1;
        }
        (*(*C).soc.offset(l as isize)).w = w;
        (*(*C).soc.offset(l as isize)).a = a;
        temp = 1.0f64 + a;
        c = (1.0f64
            + a as ::core::ffi::c_double
            + (if temp < EPS {
                w as ::core::ffi::c_double / EPS
            } else {
                w as ::core::ffi::c_double / temp as ::core::ffi::c_double
            })) as pfloat;
        divisor = temp * temp;
        d = (1 as ::core::ffi::c_int as ::core::ffi::c_double
            + (if temp < EPS {
                2 as ::core::ffi::c_int as ::core::ffi::c_double / EPS
            } else {
                2 as ::core::ffi::c_int as ::core::ffi::c_double / temp as ::core::ffi::c_double
            })
            + (if divisor < EPS {
                w as ::core::ffi::c_double / EPS
            } else {
                w as ::core::ffi::c_double / divisor as ::core::ffi::c_double
            })) as pfloat;
        c_square = c * c;
        divisor = 1.0f64 + w * d;
        d1 = (0.5f64
            * (a as ::core::ffi::c_double * a as ::core::ffi::c_double
                + w as ::core::ffi::c_double
                    * (1.0f64
                        - (if divisor < EPS {
                            c_square as ::core::ffi::c_double / EPS
                        } else {
                            c_square as ::core::ffi::c_double / divisor as ::core::ffi::c_double
                        })))) as pfloat;
        if d1 < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d1 = 0 as ::core::ffi::c_int as pfloat;
        }
        u0_square = a * a + w - d1;
        u0 = sqrt(u0_square as ::core::ffi::c_double) as pfloat;
        c2byu02 = (if u0_square < EPS {
            c as ::core::ffi::c_double * c as ::core::ffi::c_double / EPS
        } else {
            c as ::core::ffi::c_double * c as ::core::ffi::c_double
                / u0_square as ::core::ffi::c_double
        }) as pfloat;
        c2byu02_d = c2byu02 - d;
        if c2byu02_d <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            return OUTSIDE_CONE as idxint;
        }
        v1 = sqrt(c2byu02_d as ::core::ffi::c_double) as pfloat;
        u1 = sqrt(c2byu02 as ::core::ffi::c_double) as pfloat;
        (*(*C).soc.offset(l as isize)).d1 = d1;
        (*(*C).soc.offset(l as isize)).u0 = u0;
        (*(*C).soc.offset(l as isize)).u1 = u1;
        (*(*C).soc.offset(l as isize)).v1 = v1;
        k += (*(*C).soc.offset(l as isize)).p as i64;
        l += 1;
    }
    scale(z, C, lambda);
    return INSIDE_CONE as idxint;
}
#[no_mangle]
pub unsafe extern "C" fn scale(mut z: *mut pfloat, mut C: *mut cone, mut lambda: *mut pfloat) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut l: idxint = 0;
    let mut cone_start: idxint = 0;
    let mut zeta: pfloat = 0.;
    let mut factor: pfloat = 0.;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        *lambda.offset(i as isize) = *(*(*C).lpc).w.offset(i as isize) * *z.offset(i as isize);
        i += 1;
    }
    cone_start = (*(*C).lpc).p;
    l = 0 as idxint;
    while l < (*C).nsoc {
        zeta = 0 as ::core::ffi::c_int as pfloat;
        i = 1 as idxint;
        while i < (*(*C).soc.offset(l as isize)).p {
            zeta += (*(*(*C).soc.offset(l as isize))
                .q
                .offset((i as i64 - 1 as i64) as isize)
                * *z.offset((cone_start + i) as isize))
                as ::core::ffi::c_double;
            i += 1;
        }
        factor = (*z.offset(cone_start as isize) as ::core::ffi::c_double
            + (if 1 as ::core::ffi::c_int as pfloat + (*(*C).soc.offset(l as isize)).a < EPS {
                zeta as ::core::ffi::c_double / EPS
            } else {
                zeta as ::core::ffi::c_double
                    / (1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + (*(*C).soc.offset(l as isize)).a as ::core::ffi::c_double)
            })) as pfloat;
        *lambda.offset(cone_start as isize) = (*(*C).soc.offset(l as isize)).eta
            * ((*(*C).soc.offset(l as isize)).a * *z.offset(cone_start as isize) + zeta);
        i = 1 as idxint;
        while i < (*(*C).soc.offset(l as isize)).p {
            j = cone_start + i;
            *lambda.offset(j as isize) = (*(*C).soc.offset(l as isize)).eta
                * (*z.offset(j as isize)
                    + factor
                        * *(*(*C).soc.offset(l as isize)).q.offset(
                            (i as i64 - 1 as i64) as isize,
                        ));
            i += 1;
        }
        cone_start += (*(*C).soc.offset(l as isize)).p as i64;
        l += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn scale2add(mut x: *mut pfloat, mut y: *mut pfloat, mut C: *mut cone) {
    let mut i: idxint = 0;
    let mut l: idxint = 0;
    let mut cone_start: idxint = 0;
    let mut conesize: idxint = 0;
    let mut conesize_m1: idxint = 0;
    let mut x1: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut x2: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut y1: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut y2: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut eta_square: pfloat = 0.;
    let mut q: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut x3: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut x4: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut y3: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut y4: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut d1: pfloat = 0.;
    let mut u0: pfloat = 0.;
    let mut u1: pfloat = 0.;
    let mut v1: pfloat = 0.;
    let mut v1x3_plus_u1x4: pfloat = 0.;
    let mut qtx2: pfloat = 0.;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        let ref mut fresh1 = *y.offset(i as isize);
        *fresh1 +=
            (*(*(*C).lpc).v.offset(i as isize) * *x.offset(i as isize)) as ::core::ffi::c_double;
        i += 1;
    }
    cone_start = (*(*C).lpc).p;
    l = 0 as idxint;
    while l < (*C).nsoc {
        getSOCDetails(
            (*C).soc.offset(l as isize) as *mut socone,
            &raw mut conesize,
            &raw mut eta_square,
            &raw mut d1,
            &raw mut u0,
            &raw mut u1,
            &raw mut v1,
            &raw mut q,
        );
        conesize_m1 = (conesize as i64 - 1 as i64) as idxint;
        x1 = x.offset(cone_start as isize);
        x2 = x1.offset(1 as ::core::ffi::c_int as isize);
        x3 = x2
            .offset(conesize as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        x4 = x3.offset(1 as ::core::ffi::c_int as isize);
        y1 = y.offset(cone_start as isize);
        y2 = y1.offset(1 as ::core::ffi::c_int as isize);
        y3 = y2
            .offset(conesize as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        y4 = y3.offset(1 as ::core::ffi::c_int as isize);
        let ref mut fresh2 = *y1.offset(0 as ::core::ffi::c_int as isize);
        *fresh2 += (eta_square
            * (d1 * *x1.offset(0 as ::core::ffi::c_int as isize)
                + u0 * *x4.offset(0 as ::core::ffi::c_int as isize)))
            as ::core::ffi::c_double;
        v1x3_plus_u1x4 = v1 * *x3.offset(0 as ::core::ffi::c_int as isize)
            + u1 * *x4.offset(0 as ::core::ffi::c_int as isize);
        qtx2 = 0 as ::core::ffi::c_int as pfloat;
        i = 0 as idxint;
        while i < conesize_m1 {
            let ref mut fresh3 = *y2.offset(i as isize);
            *fresh3 += (eta_square
                * (*x2.offset(i as isize) + v1x3_plus_u1x4 * *q.offset(i as isize)))
                as ::core::ffi::c_double;
            qtx2 += (*q.offset(i as isize) * *x2.offset(i as isize)) as ::core::ffi::c_double;
            i += 1;
        }
        let ref mut fresh4 = *y3.offset(0 as ::core::ffi::c_int as isize);
        *fresh4 += (eta_square * (v1 * qtx2 + *x3.offset(0 as ::core::ffi::c_int as isize)))
            as ::core::ffi::c_double;
        let ref mut fresh5 = *y4.offset(0 as ::core::ffi::c_int as isize);
        *fresh5 += (eta_square
            * (u0 * *x1.offset(0 as ::core::ffi::c_int as isize) + u1 * qtx2
                - *x4.offset(0 as ::core::ffi::c_int as isize)))
            as ::core::ffi::c_double;
        cone_start += conesize as i64 + 2 as i64;
        l += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn unscale(mut lambda: *mut pfloat, mut C: *mut cone, mut z: *mut pfloat) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut l: idxint = 0;
    let mut cone_start: idxint = 0;
    let mut zeta: pfloat = 0.;
    let mut factor: pfloat = 0.;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        *z.offset(i as isize) = (if *(*(*C).lpc).w.offset(i as isize) < EPS {
            *lambda.offset(i as isize) as ::core::ffi::c_double / EPS
        } else {
            *lambda.offset(i as isize) as ::core::ffi::c_double
                / *(*(*C).lpc).w.offset(i as isize) as ::core::ffi::c_double
        }) as pfloat;
        i += 1;
    }
    cone_start = (*(*C).lpc).p;
    l = 0 as idxint;
    while l < (*C).nsoc {
        zeta = 0 as ::core::ffi::c_int as pfloat;
        i = 1 as idxint;
        while i < (*(*C).soc.offset(l as isize)).p {
            zeta += (*(*(*C).soc.offset(l as isize))
                .q
                .offset((i as i64 - 1 as i64) as isize)
                * *lambda.offset((cone_start + i) as isize))
                as ::core::ffi::c_double;
            i += 1;
        }
        factor = (-(*lambda.offset(cone_start as isize) as ::core::ffi::c_double)
            + (if 1 as ::core::ffi::c_int as pfloat + (*(*C).soc.offset(l as isize)).a < EPS {
                zeta as ::core::ffi::c_double / EPS
            } else {
                zeta as ::core::ffi::c_double
                    / (1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + (*(*C).soc.offset(l as isize)).a as ::core::ffi::c_double)
            })) as pfloat;
        *z.offset(cone_start as isize) = (if (*(*C).soc.offset(l as isize)).eta < EPS {
            ((*(*C).soc.offset(l as isize)).a as ::core::ffi::c_double
                * *lambda.offset(cone_start as isize) as ::core::ffi::c_double
                - zeta as ::core::ffi::c_double)
                / EPS
        } else {
            ((*(*C).soc.offset(l as isize)).a as ::core::ffi::c_double
                * *lambda.offset(cone_start as isize) as ::core::ffi::c_double
                - zeta as ::core::ffi::c_double)
                / (*(*C).soc.offset(l as isize)).eta as ::core::ffi::c_double
        }) as pfloat;
        i = 1 as idxint;
        while i < (*(*C).soc.offset(l as isize)).p {
            j = cone_start + i;
            *z.offset(j as isize) =
                (if (*(*C).soc.offset(l as isize)).eta < EPS {
                    (*lambda.offset(j as isize) as ::core::ffi::c_double
                        + factor as ::core::ffi::c_double
                            * *(*(*C).soc.offset(l as isize)).q.offset(
                                (i as i64 - 1 as i64) as isize,
                            ) as ::core::ffi::c_double)
                        / EPS
                } else {
                    (*lambda.offset(j as isize) as ::core::ffi::c_double
                        + factor as ::core::ffi::c_double
                            * *(*(*C).soc.offset(l as isize)).q.offset(
                                (i as i64 - 1 as i64) as isize,
                            ) as ::core::ffi::c_double)
                        / (*(*C).soc.offset(l as isize)).eta as ::core::ffi::c_double
                }) as pfloat;
            i += 1;
        }
        cone_start += (*(*C).soc.offset(l as isize)).p as i64;
        l += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn conicProduct(
    mut u: *mut pfloat,
    mut v: *mut pfloat,
    mut C: *mut cone,
    mut w: *mut pfloat,
) -> pfloat {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    let mut cone_start: idxint = 0;
    let mut conesize: idxint = 0;
    let mut u0: pfloat = 0.;
    let mut v0: pfloat = 0.;
    let mut mu: pfloat = 0.;
    mu = 0 as ::core::ffi::c_int as pfloat;
    k = 0 as idxint;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        *w.offset(k as isize) = *u.offset(i as isize) * *v.offset(i as isize);
        mu += if *w.offset(k as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -(*w.offset(k as isize) as ::core::ffi::c_double)
        } else {
            *w.offset(k as isize) as ::core::ffi::c_double
        };
        k += 1;
        i += 1;
    }
    cone_start = (*(*C).lpc).p;
    i = 0 as idxint;
    while i < (*C).nsoc {
        conesize = (*(*C).soc.offset(i as isize)).p;
        u0 = *u.offset(cone_start as isize);
        v0 = *v.offset(cone_start as isize);
        *w.offset(k as isize) = eddot(
            conesize,
            u.offset(cone_start as isize),
            v.offset(cone_start as isize),
        );
        mu += if *w.offset(k as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -(*w.offset(k as isize) as ::core::ffi::c_double)
        } else {
            *w.offset(k as isize) as ::core::ffi::c_double
        };
        k += 1;
        j = 1 as idxint;
        while j < conesize {
            let fresh6 = k;
            k = k + 1;
            *w.offset(fresh6 as isize) = u0 * *v.offset((cone_start + j) as isize)
                + v0 * *u.offset((cone_start + j) as isize);
            j += 1;
        }
        cone_start += conesize as i64;
        i += 1;
    }
    return mu;
}
#[no_mangle]
pub unsafe extern "C" fn conicDivision(
    mut u: *mut pfloat,
    mut w: *mut pfloat,
    mut C: *mut cone,
    mut v: *mut pfloat,
) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    let mut cone_start: idxint = 0;
    let mut conesize: idxint = 0;
    let mut rho: pfloat = 0.;
    let mut zeta: pfloat = 0.;
    let mut u0: pfloat = 0.;
    let mut w0: pfloat = 0.;
    let mut factor: pfloat = 0.;
    let mut temp: pfloat = 0.;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        *v.offset(i as isize) = (if *u.offset(i as isize) < EPS {
            *w.offset(i as isize) as ::core::ffi::c_double / EPS
        } else {
            *w.offset(i as isize) as ::core::ffi::c_double
                / *u.offset(i as isize) as ::core::ffi::c_double
        }) as pfloat;
        i += 1;
    }
    cone_start = (*(*C).lpc).p;
    i = 0 as idxint;
    while i < (*C).nsoc {
        conesize = (*(*C).soc.offset(i as isize)).p;
        u0 = *u.offset(cone_start as isize);
        w0 = *w.offset(cone_start as isize);
        rho = u0 * u0;
        zeta = 0 as ::core::ffi::c_int as pfloat;
        j = 1 as idxint;
        while j < conesize {
            k = cone_start + j;
            rho -= (*u.offset(k as isize) * *u.offset(k as isize)) as ::core::ffi::c_double;
            zeta += (*u.offset(k as isize) * *w.offset(k as isize)) as ::core::ffi::c_double;
            j += 1;
        }
        temp = (if u0 < EPS { zeta / EPS } else { zeta / u0 }) - w0;
        factor = (if rho < EPS {
            temp as ::core::ffi::c_double / EPS
        } else {
            temp as ::core::ffi::c_double / rho as ::core::ffi::c_double
        }) as pfloat;
        temp = u0 * w0 - zeta;
        *v.offset(cone_start as isize) = (if rho < EPS {
            temp as ::core::ffi::c_double / EPS
        } else {
            temp as ::core::ffi::c_double / rho as ::core::ffi::c_double
        }) as pfloat;
        j = 1 as idxint;
        while j < conesize {
            k = cone_start + j;
            *v.offset((cone_start + j) as isize) = (factor as ::core::ffi::c_double
                * *u.offset(k as isize) as ::core::ffi::c_double
                + (if u0 < EPS {
                    *w.offset(k as isize) as ::core::ffi::c_double / EPS
                } else {
                    *w.offset(k as isize) as ::core::ffi::c_double / u0 as ::core::ffi::c_double
                })) as pfloat;
            j += 1;
        }
        cone_start += (*(*C).soc.offset(i as isize)).p as i64;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getSOCDetails(
    mut soc: *mut socone,
    mut conesize: *mut idxint,
    mut eta_square: *mut pfloat,
    mut d1: *mut pfloat,
    mut u0: *mut pfloat,
    mut u1: *mut pfloat,
    mut v1: *mut pfloat,
    mut q: *mut *mut pfloat,
) {
    *conesize = (*soc).p;
    *eta_square = (*soc).eta_square;
    *d1 = (*soc).d1;
    *u0 = (*soc).u0;
    *u1 = (*soc).u1;
    *v1 = (*soc).v1;
    *q = (*soc).q;
}
#[no_mangle]
pub unsafe extern "C" fn unstretch(
    mut n: idxint,
    mut p: idxint,
    mut C: *mut cone,
    mut Pinv: *mut idxint,
    mut Px: *mut pfloat,
    mut dx: *mut pfloat,
    mut dy: *mut pfloat,
    mut dz: *mut pfloat,
) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    let mut l: idxint = 0;
    k = 0 as idxint;
    i = 0 as idxint;
    while i < n {
        let fresh7 = k;
        k = k + 1;
        *dx.offset(i as isize) = *Px.offset(*Pinv.offset(fresh7 as isize) as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < p {
        let fresh8 = k;
        k = k + 1;
        *dy.offset(i as isize) = *Px.offset(*Pinv.offset(fresh8 as isize) as isize);
        i += 1;
    }
    j = 0 as idxint;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        let fresh9 = k;
        k = k + 1;
        let fresh10 = j;
        j = j + 1;
        *dz.offset(fresh10 as isize) = *Px.offset(*Pinv.offset(fresh9 as isize) as isize);
        i += 1;
    }
    l = 0 as idxint;
    while l < (*C).nsoc {
        i = 0 as idxint;
        while i < (*(*C).soc.offset(l as isize)).p {
            let fresh11 = k;
            k = k + 1;
            let fresh12 = j;
            j = j + 1;
            *dz.offset(fresh12 as isize) = *Px.offset(*Pinv.offset(fresh11 as isize) as isize);
            i += 1;
        }
        k += 2 as i64;
        l += 1;
    }
}
