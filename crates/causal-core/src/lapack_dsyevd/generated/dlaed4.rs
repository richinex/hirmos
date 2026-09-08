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
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaed4_(
    mut n: *mut integer,
    mut i__: *mut integer,
    mut d__: *mut doublereal,
    mut z__: *mut doublereal,
    mut delta: *mut doublereal,
    mut rho: *mut doublereal,
    mut dlam: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut a: doublereal = 0.;
    let mut b: doublereal = 0.;
    let mut c__: doublereal = 0.;
    let mut j: integer = 0;
    let mut w: doublereal = 0.;
    let mut ii: integer = 0;
    let mut dw: doublereal = 0.;
    let mut zz: [doublereal; 3] = [0.; 3];
    let mut ip1: integer = 0;
    let mut del: doublereal = 0.;
    let mut eta: doublereal = 0.;
    let mut phi: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut tau: doublereal = 0.;
    let mut psi: doublereal = 0.;
    let mut iim1: integer = 0;
    let mut iip1: integer = 0;
    let mut dphi: doublereal = 0.;
    let mut dpsi: doublereal = 0.;
    let mut iter: integer = 0;
    let mut temp: doublereal = 0.;
    let mut prew: doublereal = 0.;
    let mut temp1: doublereal = 0.;
    let mut dltlb: doublereal = 0.;
    let mut dltub: doublereal = 0.;
    let mut midpt: doublereal = 0.;
    let mut niter: integer = 0;
    let mut swtch: logical = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dlaed5_"]
        fn dlaed5__0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlaed6_"]
        fn dlaed6__0(
            _: *mut integer,
            _: *mut logical,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut swtch3: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    let mut orgati: logical = 0;
    let mut erretm: doublereal = 0.;
    let mut rhoinv: doublereal = 0.;
    delta = delta.offset(-1);
    z__ = z__.offset(-1);
    d__ = d__.offset(-1);
    *info = 0 as integer;
    if *n == 1 as ::core::ffi::c_long {
        *dlam = *d__.offset(1 as ::core::ffi::c_int as isize)
            + *rho
                * *z__.offset(1 as ::core::ffi::c_int as isize)
                * *z__.offset(1 as ::core::ffi::c_int as isize);
        *delta.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    if *n == 2 as ::core::ffi::c_long {
        dlaed5__0(
            i__,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            delta.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            rho,
            dlam,
        );
        return 0 as ::core::ffi::c_int;
    }
    eps = dlamch__0(
        b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    rhoinv = 1.0f64 / *rho;
    if *i__ == *n {
        ii = (*n - 1 as ::core::ffi::c_long) as integer;
        niter = 1 as integer;
        midpt = (*rho / 2.0f64) as doublereal;
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            *delta.offset(j as isize) =
                *d__.offset(j as isize) - *d__.offset(*i__ as isize) - midpt;
            j += 1;
        }
        psi = 0.0f64 as doublereal;
        i__1 = (*n - 2 as ::core::ffi::c_long) as integer;
        j = 1 as integer;
        while j <= i__1 {
            psi += (*z__.offset(j as isize) * *z__.offset(j as isize) / *delta.offset(j as isize))
                as ::core::ffi::c_double;
            j += 1;
        }
        c__ = rhoinv + psi;
        w = c__
            + *z__.offset(ii as isize) * *z__.offset(ii as isize) / *delta.offset(ii as isize)
            + *z__.offset(*n as isize) * *z__.offset(*n as isize) / *delta.offset(*n as isize);
        if w <= 0.0f64 {
            temp = *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                * *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                / (*d__.offset(*n as isize)
                    - *d__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                    + *rho)
                + *z__.offset(*n as isize) * *z__.offset(*n as isize) / *rho;
            if c__ <= temp {
                tau = *rho;
            } else {
                del = *d__.offset(*n as isize)
                    - *d__.offset((*n - 1 as ::core::ffi::c_long) as isize);
                a = -c__ * del
                    + *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                        * *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                    + *z__.offset(*n as isize) * *z__.offset(*n as isize);
                b = *z__.offset(*n as isize) * *z__.offset(*n as isize) * del;
                if a < 0.0f64 {
                    tau = b * 2.0f64 / (sqrt(a * a + b * 4.0f64 * c__) as doublereal - a);
                } else {
                    tau = ((a as ::core::ffi::c_double + sqrt(a * a + b * 4.0f64 * c__))
                        / (c__ as ::core::ffi::c_double * 2.0f64))
                        as doublereal;
                }
            }
            dltlb = midpt;
            dltub = *rho;
        } else {
            del = *d__.offset(*n as isize) - *d__.offset((*n - 1 as ::core::ffi::c_long) as isize);
            a = -c__ * del
                + *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                    * *z__.offset((*n - 1 as ::core::ffi::c_long) as isize)
                + *z__.offset(*n as isize) * *z__.offset(*n as isize);
            b = *z__.offset(*n as isize) * *z__.offset(*n as isize) * del;
            if a < 0.0f64 {
                tau = b * 2.0f64 / (sqrt(a * a + b * 4.0f64 * c__) as doublereal - a);
            } else {
                tau = ((a as ::core::ffi::c_double + sqrt(a * a + b * 4.0f64 * c__))
                    / (c__ as ::core::ffi::c_double * 2.0f64)) as doublereal;
            }
            dltlb = 0.0f64 as doublereal;
            dltub = midpt;
        }
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            *delta.offset(j as isize) = *d__.offset(j as isize) - *d__.offset(*i__ as isize) - tau;
            j += 1;
        }
        dpsi = 0.0f64 as doublereal;
        psi = 0.0f64 as doublereal;
        erretm = 0.0f64 as doublereal;
        i__1 = ii;
        j = 1 as integer;
        while j <= i__1 {
            temp = *z__.offset(j as isize) / *delta.offset(j as isize);
            psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
            dpsi += (temp * temp) as ::core::ffi::c_double;
            erretm += psi as ::core::ffi::c_double;
            j += 1;
        }
        erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            erretm as ::core::ffi::c_double
        } else {
            -(erretm as ::core::ffi::c_double)
        }) as doublereal;
        temp = *z__.offset(*n as isize) / *delta.offset(*n as isize);
        phi = *z__.offset(*n as isize) * temp;
        dphi = temp * temp;
        erretm = (-phi - psi) * 8.0f64 + erretm - phi
            + rhoinv
            + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                tau
            } else {
                -tau
            }) * (dpsi + dphi);
        w = rhoinv + phi + psi;
        if (if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            w as ::core::ffi::c_double
        } else {
            -(w as ::core::ffi::c_double)
        }) <= eps * erretm
        {
            *dlam = *d__.offset(*i__ as isize) + tau;
        } else {
            if w <= 0.0f64 {
                dltlb = (if dltlb >= tau {
                    dltlb as ::core::ffi::c_double
                } else {
                    tau as ::core::ffi::c_double
                }) as doublereal;
            } else {
                dltub = (if dltub <= tau {
                    dltub as ::core::ffi::c_double
                } else {
                    tau as ::core::ffi::c_double
                }) as doublereal;
            }
            niter += 1;
            c__ = w
                - *delta.offset((*n - 1 as ::core::ffi::c_long) as isize) * dpsi
                - *delta.offset(*n as isize) * dphi;
            a = (*delta.offset((*n - 1 as ::core::ffi::c_long) as isize)
                + *delta.offset(*n as isize))
                * w
                - *delta.offset((*n - 1 as ::core::ffi::c_long) as isize)
                    * *delta.offset(*n as isize)
                    * (dpsi + dphi);
            b = *delta.offset((*n - 1 as ::core::ffi::c_long) as isize)
                * *delta.offset(*n as isize)
                * w;
            if c__ < 0.0f64 {
                c__ = (if c__ >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    c__ as ::core::ffi::c_double
                } else {
                    -(c__ as ::core::ffi::c_double)
                }) as doublereal;
            }
            if c__ == 0.0f64 {
                eta = dltub - tau;
            } else if a >= 0.0f64 {
                d__1 = a * a - b * 4.0f64 * c__;
                eta = ((a as ::core::ffi::c_double
                    + sqrt(
                        (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1
                        } else {
                            -d__1
                        }),
                    ))
                    / (c__ as ::core::ffi::c_double * 2.0f64)) as doublereal;
            } else {
                d__1 = a * a - b * 4.0f64 * c__;
                eta = (b as ::core::ffi::c_double * 2.0f64
                    / (a as ::core::ffi::c_double
                        - sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))) as doublereal;
            }
            if w * eta > 0.0f64 {
                eta = -w / (dpsi + dphi);
            }
            temp = tau + eta;
            if temp > dltub || temp < dltlb {
                if w < 0.0f64 {
                    eta = ((dltub as ::core::ffi::c_double - tau as ::core::ffi::c_double) / 2.0f64)
                        as doublereal;
                } else {
                    eta = ((dltlb as ::core::ffi::c_double - tau as ::core::ffi::c_double) / 2.0f64)
                        as doublereal;
                }
            }
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                let ref mut fresh0 = *delta.offset(j as isize);
                *fresh0 -= eta as ::core::ffi::c_double;
                j += 1;
            }
            tau += eta as ::core::ffi::c_double;
            dpsi = 0.0f64 as doublereal;
            psi = 0.0f64 as doublereal;
            erretm = 0.0f64 as doublereal;
            i__1 = ii;
            j = 1 as integer;
            while j <= i__1 {
                temp = *z__.offset(j as isize) / *delta.offset(j as isize);
                psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                dpsi += (temp * temp) as ::core::ffi::c_double;
                erretm += psi as ::core::ffi::c_double;
                j += 1;
            }
            erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                erretm as ::core::ffi::c_double
            } else {
                -(erretm as ::core::ffi::c_double)
            }) as doublereal;
            temp = *z__.offset(*n as isize) / *delta.offset(*n as isize);
            phi = *z__.offset(*n as isize) * temp;
            dphi = temp * temp;
            erretm = (-phi - psi) * 8.0f64 + erretm - phi
                + rhoinv
                + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    tau
                } else {
                    -tau
                }) * (dpsi + dphi);
            w = rhoinv + phi + psi;
            iter = (niter as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            niter = iter;
            loop {
                if !(niter <= 30 as ::core::ffi::c_long) {
                    current_block = 14957879348977399331;
                    break;
                }
                if (if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    w as ::core::ffi::c_double
                } else {
                    -(w as ::core::ffi::c_double)
                }) <= eps * erretm
                {
                    *dlam = *d__.offset(*i__ as isize) + tau;
                    current_block = 14409496533122754037;
                    break;
                } else {
                    if w <= 0.0f64 {
                        dltlb = (if dltlb >= tau {
                            dltlb as ::core::ffi::c_double
                        } else {
                            tau as ::core::ffi::c_double
                        }) as doublereal;
                    } else {
                        dltub = (if dltub <= tau {
                            dltub as ::core::ffi::c_double
                        } else {
                            tau as ::core::ffi::c_double
                        }) as doublereal;
                    }
                    c__ = w
                        - *delta.offset((*n - 1 as ::core::ffi::c_long) as isize) * dpsi
                        - *delta.offset(*n as isize) * dphi;
                    a = (*delta.offset((*n - 1 as ::core::ffi::c_long) as isize)
                        + *delta.offset(*n as isize))
                        * w
                        - *delta.offset((*n - 1 as ::core::ffi::c_long) as isize)
                            * *delta.offset(*n as isize)
                            * (dpsi + dphi);
                    b = *delta.offset((*n - 1 as ::core::ffi::c_long) as isize)
                        * *delta.offset(*n as isize)
                        * w;
                    if a >= 0.0f64 {
                        d__1 = a * a - b * 4.0f64 * c__;
                        eta = ((a as ::core::ffi::c_double
                            + sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ))
                            / (c__ as ::core::ffi::c_double * 2.0f64))
                            as doublereal;
                    } else {
                        d__1 = a * a - b * 4.0f64 * c__;
                        eta = (b as ::core::ffi::c_double * 2.0f64
                            / (a as ::core::ffi::c_double
                                - sqrt(
                                    (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__1
                                    } else {
                                        -d__1
                                    }),
                                ))) as doublereal;
                    }
                    if w * eta > 0.0f64 {
                        eta = -w / (dpsi + dphi);
                    }
                    temp = tau + eta;
                    if temp > dltub || temp < dltlb {
                        if w < 0.0f64 {
                            eta = ((dltub as ::core::ffi::c_double - tau as ::core::ffi::c_double)
                                / 2.0f64) as doublereal;
                        } else {
                            eta = ((dltlb as ::core::ffi::c_double - tau as ::core::ffi::c_double)
                                / 2.0f64) as doublereal;
                        }
                    }
                    i__1 = *n;
                    j = 1 as integer;
                    while j <= i__1 {
                        let ref mut fresh1 = *delta.offset(j as isize);
                        *fresh1 -= eta as ::core::ffi::c_double;
                        j += 1;
                    }
                    tau += eta as ::core::ffi::c_double;
                    dpsi = 0.0f64 as doublereal;
                    psi = 0.0f64 as doublereal;
                    erretm = 0.0f64 as doublereal;
                    i__1 = ii;
                    j = 1 as integer;
                    while j <= i__1 {
                        temp = *z__.offset(j as isize) / *delta.offset(j as isize);
                        psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                        dpsi += (temp * temp) as ::core::ffi::c_double;
                        erretm += psi as ::core::ffi::c_double;
                        j += 1;
                    }
                    erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        erretm as ::core::ffi::c_double
                    } else {
                        -(erretm as ::core::ffi::c_double)
                    }) as doublereal;
                    temp = *z__.offset(*n as isize) / *delta.offset(*n as isize);
                    phi = *z__.offset(*n as isize) * temp;
                    dphi = temp * temp;
                    erretm = (-phi - psi) * 8.0f64 + erretm - phi
                        + rhoinv
                        + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            tau
                        } else {
                            -tau
                        }) * (dpsi + dphi);
                    w = rhoinv + phi + psi;
                    niter += 1;
                }
            }
            match current_block {
                14409496533122754037 => {}
                _ => {
                    *info = 1 as integer;
                    *dlam = *d__.offset(*i__ as isize) + tau;
                }
            }
        }
    } else {
        niter = 1 as integer;
        ip1 = (*i__ + 1 as ::core::ffi::c_long) as integer;
        del = *d__.offset(ip1 as isize) - *d__.offset(*i__ as isize);
        midpt = (del as ::core::ffi::c_double / 2.0f64) as doublereal;
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            *delta.offset(j as isize) =
                *d__.offset(j as isize) - *d__.offset(*i__ as isize) - midpt;
            j += 1;
        }
        psi = 0.0f64 as doublereal;
        i__1 = (*i__ - 1 as ::core::ffi::c_long) as integer;
        j = 1 as integer;
        while j <= i__1 {
            psi += (*z__.offset(j as isize) * *z__.offset(j as isize) / *delta.offset(j as isize))
                as ::core::ffi::c_double;
            j += 1;
        }
        phi = 0.0f64 as doublereal;
        i__1 = (*i__ + 2 as ::core::ffi::c_long) as integer;
        j = *n;
        while j >= i__1 {
            phi += (*z__.offset(j as isize) * *z__.offset(j as isize) / *delta.offset(j as isize))
                as ::core::ffi::c_double;
            j -= 1;
        }
        c__ = rhoinv + psi + phi;
        w = c__
            + *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize)
                / *delta.offset(*i__ as isize)
            + *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize) / *delta.offset(ip1 as isize);
        if w > 0.0f64 {
            orgati = TRUE_ as logical;
            a = c__ * del
                + *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize)
                + *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize);
            b = *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize) * del;
            if a > 0.0f64 {
                d__1 = a * a - b * 4.0f64 * c__;
                tau = (b as ::core::ffi::c_double * 2.0f64
                    / (a as ::core::ffi::c_double
                        + sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))) as doublereal;
            } else {
                d__1 = a * a - b * 4.0f64 * c__;
                tau = ((a as ::core::ffi::c_double
                    - sqrt(
                        (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1
                        } else {
                            -d__1
                        }),
                    ))
                    / (c__ as ::core::ffi::c_double * 2.0f64)) as doublereal;
            }
            dltlb = 0.0f64 as doublereal;
            dltub = midpt;
        } else {
            orgati = FALSE_ as logical;
            a = c__ * del
                - *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize)
                - *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize);
            b = *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize) * del;
            if a < 0.0f64 {
                d__1 = a * a + b * 4.0f64 * c__;
                tau = (b as ::core::ffi::c_double * 2.0f64
                    / (a as ::core::ffi::c_double
                        - sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))) as doublereal;
            } else {
                d__1 = a * a + b * 4.0f64 * c__;
                tau = (-(a as ::core::ffi::c_double
                    + sqrt(
                        (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1
                        } else {
                            -d__1
                        }),
                    ))
                    / (c__ as ::core::ffi::c_double * 2.0f64)) as doublereal;
            }
            dltlb = -midpt;
            dltub = 0.0f64 as doublereal;
        }
        if orgati != 0 {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                *delta.offset(j as isize) =
                    *d__.offset(j as isize) - *d__.offset(*i__ as isize) - tau;
                j += 1;
            }
        } else {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                *delta.offset(j as isize) =
                    *d__.offset(j as isize) - *d__.offset(ip1 as isize) - tau;
                j += 1;
            }
        }
        if orgati != 0 {
            ii = *i__;
        } else {
            ii = (*i__ + 1 as ::core::ffi::c_long) as integer;
        }
        iim1 = (ii as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        iip1 = (ii as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dpsi = 0.0f64 as doublereal;
        psi = 0.0f64 as doublereal;
        erretm = 0.0f64 as doublereal;
        i__1 = iim1;
        j = 1 as integer;
        while j <= i__1 {
            temp = *z__.offset(j as isize) / *delta.offset(j as isize);
            psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
            dpsi += (temp * temp) as ::core::ffi::c_double;
            erretm += psi as ::core::ffi::c_double;
            j += 1;
        }
        erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            erretm as ::core::ffi::c_double
        } else {
            -(erretm as ::core::ffi::c_double)
        }) as doublereal;
        dphi = 0.0f64 as doublereal;
        phi = 0.0f64 as doublereal;
        i__1 = iip1;
        j = *n;
        while j >= i__1 {
            temp = *z__.offset(j as isize) / *delta.offset(j as isize);
            phi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
            dphi += (temp * temp) as ::core::ffi::c_double;
            erretm += phi as ::core::ffi::c_double;
            j -= 1;
        }
        w = rhoinv + phi + psi;
        swtch3 = FALSE_ as logical;
        if orgati != 0 {
            if w < 0.0f64 {
                swtch3 = TRUE_ as logical;
            }
        } else if w > 0.0f64 {
            swtch3 = TRUE_ as logical;
        }
        if ii == 1 as ::core::ffi::c_long || ii == *n {
            swtch3 = FALSE_ as logical;
        }
        temp = *z__.offset(ii as isize) / *delta.offset(ii as isize);
        dw = dpsi + dphi + temp * temp;
        temp = *z__.offset(ii as isize) * temp;
        w += temp as ::core::ffi::c_double;
        erretm = (phi - psi) * 8.0f64
            + erretm
            + rhoinv * 2.0f64
            + (if temp >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                temp
            } else {
                -temp
            }) * 3.0f64
            + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                tau
            } else {
                -tau
            }) * dw;
        if (if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            w as ::core::ffi::c_double
        } else {
            -(w as ::core::ffi::c_double)
        }) <= eps * erretm
        {
            if orgati != 0 {
                *dlam = *d__.offset(*i__ as isize) + tau;
            } else {
                *dlam = *d__.offset(ip1 as isize) + tau;
            }
        } else {
            if w <= 0.0f64 {
                dltlb = (if dltlb >= tau {
                    dltlb as ::core::ffi::c_double
                } else {
                    tau as ::core::ffi::c_double
                }) as doublereal;
            } else {
                dltub = (if dltub <= tau {
                    dltub as ::core::ffi::c_double
                } else {
                    tau as ::core::ffi::c_double
                }) as doublereal;
            }
            niter += 1;
            if swtch3 == 0 {
                if orgati != 0 {
                    d__1 = *z__.offset(*i__ as isize) / *delta.offset(*i__ as isize);
                    c__ = w
                        - *delta.offset(ip1 as isize) * dw
                        - (*d__.offset(*i__ as isize) - *d__.offset(ip1 as isize)) * (d__1 * d__1);
                } else {
                    d__1 = *z__.offset(ip1 as isize) / *delta.offset(ip1 as isize);
                    c__ = w
                        - *delta.offset(*i__ as isize) * dw
                        - (*d__.offset(ip1 as isize) - *d__.offset(*i__ as isize)) * (d__1 * d__1);
                }
                a = (*delta.offset(*i__ as isize) + *delta.offset(ip1 as isize)) * w
                    - *delta.offset(*i__ as isize) * *delta.offset(ip1 as isize) * dw;
                b = *delta.offset(*i__ as isize) * *delta.offset(ip1 as isize) * w;
                if c__ == 0.0f64 {
                    if a == 0.0f64 {
                        if orgati != 0 {
                            a = *z__.offset(*i__ as isize) * *z__.offset(*i__ as isize)
                                + *delta.offset(ip1 as isize)
                                    * *delta.offset(ip1 as isize)
                                    * (dpsi + dphi);
                        } else {
                            a = *z__.offset(ip1 as isize) * *z__.offset(ip1 as isize)
                                + *delta.offset(*i__ as isize)
                                    * *delta.offset(*i__ as isize)
                                    * (dpsi + dphi);
                        }
                    }
                    eta = b / a;
                } else if a <= 0.0f64 {
                    d__1 = a * a - b * 4.0f64 * c__;
                    eta = ((a as ::core::ffi::c_double
                        - sqrt(
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1
                            } else {
                                -d__1
                            }),
                        ))
                        / (c__ as ::core::ffi::c_double * 2.0f64))
                        as doublereal;
                } else {
                    d__1 = a * a - b * 4.0f64 * c__;
                    eta = (b as ::core::ffi::c_double * 2.0f64
                        / (a as ::core::ffi::c_double
                            + sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ))) as doublereal;
                }
                current_block = 1424623445371442388;
            } else {
                temp = rhoinv + psi + phi;
                if orgati != 0 {
                    temp1 = *z__.offset(iim1 as isize) / *delta.offset(iim1 as isize);
                    temp1 *= temp1 as ::core::ffi::c_double;
                    c__ = temp
                        - *delta.offset(iip1 as isize) * (dpsi + dphi)
                        - (*d__.offset(iim1 as isize) - *d__.offset(iip1 as isize)) * temp1;
                    zz[0 as ::core::ffi::c_int as usize] =
                        *z__.offset(iim1 as isize) * *z__.offset(iim1 as isize);
                    zz[2 as ::core::ffi::c_int as usize] = *delta.offset(iip1 as isize)
                        * *delta.offset(iip1 as isize)
                        * (dpsi - temp1 + dphi);
                } else {
                    temp1 = *z__.offset(iip1 as isize) / *delta.offset(iip1 as isize);
                    temp1 *= temp1 as ::core::ffi::c_double;
                    c__ = temp
                        - *delta.offset(iim1 as isize) * (dpsi + dphi)
                        - (*d__.offset(iip1 as isize) - *d__.offset(iim1 as isize)) * temp1;
                    zz[0 as ::core::ffi::c_int as usize] = *delta.offset(iim1 as isize)
                        * *delta.offset(iim1 as isize)
                        * (dpsi + (dphi - temp1));
                    zz[2 as ::core::ffi::c_int as usize] =
                        *z__.offset(iip1 as isize) * *z__.offset(iip1 as isize);
                }
                zz[1 as ::core::ffi::c_int as usize] =
                    *z__.offset(ii as isize) * *z__.offset(ii as isize);
                dlaed6__0(
                    &raw mut niter,
                    &raw mut orgati,
                    &raw mut c__,
                    delta.offset(iim1 as isize) as *mut doublereal,
                    &raw mut zz as *mut doublereal,
                    &raw mut w,
                    &raw mut eta,
                    info,
                );
                if *info != 0 as ::core::ffi::c_long {
                    current_block = 14409496533122754037;
                } else {
                    current_block = 1424623445371442388;
                }
            }
            match current_block {
                14409496533122754037 => {}
                _ => {
                    if w * eta >= 0.0f64 {
                        eta = -w / dw;
                    }
                    temp = tau + eta;
                    if temp > dltub || temp < dltlb {
                        if w < 0.0f64 {
                            eta = ((dltub as ::core::ffi::c_double - tau as ::core::ffi::c_double)
                                / 2.0f64) as doublereal;
                        } else {
                            eta = ((dltlb as ::core::ffi::c_double - tau as ::core::ffi::c_double)
                                / 2.0f64) as doublereal;
                        }
                    }
                    prew = w;
                    i__1 = *n;
                    j = 1 as integer;
                    while j <= i__1 {
                        let ref mut fresh2 = *delta.offset(j as isize);
                        *fresh2 -= eta as ::core::ffi::c_double;
                        j += 1;
                    }
                    dpsi = 0.0f64 as doublereal;
                    psi = 0.0f64 as doublereal;
                    erretm = 0.0f64 as doublereal;
                    i__1 = iim1;
                    j = 1 as integer;
                    while j <= i__1 {
                        temp = *z__.offset(j as isize) / *delta.offset(j as isize);
                        psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                        dpsi += (temp * temp) as ::core::ffi::c_double;
                        erretm += psi as ::core::ffi::c_double;
                        j += 1;
                    }
                    erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        erretm as ::core::ffi::c_double
                    } else {
                        -(erretm as ::core::ffi::c_double)
                    }) as doublereal;
                    dphi = 0.0f64 as doublereal;
                    phi = 0.0f64 as doublereal;
                    i__1 = iip1;
                    j = *n;
                    while j >= i__1 {
                        temp = *z__.offset(j as isize) / *delta.offset(j as isize);
                        phi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                        dphi += (temp * temp) as ::core::ffi::c_double;
                        erretm += phi as ::core::ffi::c_double;
                        j -= 1;
                    }
                    temp = *z__.offset(ii as isize) / *delta.offset(ii as isize);
                    dw = dpsi + dphi + temp * temp;
                    temp = *z__.offset(ii as isize) * temp;
                    w = rhoinv + phi + psi + temp;
                    d__1 = tau + eta;
                    erretm = (phi - psi) * 8.0f64
                        + erretm
                        + rhoinv * 2.0f64
                        + (if temp >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            temp
                        } else {
                            -temp
                        }) * 3.0f64
                        + (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1
                        } else {
                            -d__1
                        }) * dw;
                    swtch = FALSE_ as logical;
                    if orgati != 0 {
                        if -w
                            > (if prew >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                prew as ::core::ffi::c_double
                            } else {
                                -(prew as ::core::ffi::c_double)
                            }) / 10.0f64
                        {
                            swtch = TRUE_ as logical;
                        }
                    } else if w
                        > (if prew >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            prew as ::core::ffi::c_double
                        } else {
                            -(prew as ::core::ffi::c_double)
                        }) / 10.0f64
                    {
                        swtch = TRUE_ as logical;
                    }
                    tau += eta as ::core::ffi::c_double;
                    iter = (niter as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    niter = iter;
                    loop {
                        if !(niter <= 30 as ::core::ffi::c_long) {
                            current_block = 15882700527400876337;
                            break;
                        }
                        if (if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            w as ::core::ffi::c_double
                        } else {
                            -(w as ::core::ffi::c_double)
                        }) <= eps * erretm
                        {
                            if orgati != 0 {
                                *dlam = *d__.offset(*i__ as isize) + tau;
                            } else {
                                *dlam = *d__.offset(ip1 as isize) + tau;
                            }
                            current_block = 14409496533122754037;
                            break;
                        } else {
                            if w <= 0.0f64 {
                                dltlb = (if dltlb >= tau {
                                    dltlb as ::core::ffi::c_double
                                } else {
                                    tau as ::core::ffi::c_double
                                }) as doublereal;
                            } else {
                                dltub = (if dltub <= tau {
                                    dltub as ::core::ffi::c_double
                                } else {
                                    tau as ::core::ffi::c_double
                                }) as doublereal;
                            }
                            if swtch3 == 0 {
                                if swtch == 0 {
                                    if orgati != 0 {
                                        d__1 = *z__.offset(*i__ as isize)
                                            / *delta.offset(*i__ as isize);
                                        c__ = w
                                            - *delta.offset(ip1 as isize) * dw
                                            - (*d__.offset(*i__ as isize)
                                                - *d__.offset(ip1 as isize))
                                                * (d__1 * d__1);
                                    } else {
                                        d__1 =
                                            *z__.offset(ip1 as isize) / *delta.offset(ip1 as isize);
                                        c__ = w
                                            - *delta.offset(*i__ as isize) * dw
                                            - (*d__.offset(ip1 as isize)
                                                - *d__.offset(*i__ as isize))
                                                * (d__1 * d__1);
                                    }
                                } else {
                                    temp = *z__.offset(ii as isize) / *delta.offset(ii as isize);
                                    if orgati != 0 {
                                        dpsi += (temp * temp) as ::core::ffi::c_double;
                                    } else {
                                        dphi += (temp * temp) as ::core::ffi::c_double;
                                    }
                                    c__ = w
                                        - *delta.offset(*i__ as isize) * dpsi
                                        - *delta.offset(ip1 as isize) * dphi;
                                }
                                a = (*delta.offset(*i__ as isize) + *delta.offset(ip1 as isize))
                                    * w
                                    - *delta.offset(*i__ as isize)
                                        * *delta.offset(ip1 as isize)
                                        * dw;
                                b = *delta.offset(*i__ as isize) * *delta.offset(ip1 as isize) * w;
                                if c__ == 0.0f64 {
                                    if a == 0.0f64 {
                                        if swtch == 0 {
                                            if orgati != 0 {
                                                a = *z__.offset(*i__ as isize)
                                                    * *z__.offset(*i__ as isize)
                                                    + *delta.offset(ip1 as isize)
                                                        * *delta.offset(ip1 as isize)
                                                        * (dpsi + dphi);
                                            } else {
                                                a = *z__.offset(ip1 as isize)
                                                    * *z__.offset(ip1 as isize)
                                                    + *delta.offset(*i__ as isize)
                                                        * *delta.offset(*i__ as isize)
                                                        * (dpsi + dphi);
                                            }
                                        } else {
                                            a = *delta.offset(*i__ as isize)
                                                * *delta.offset(*i__ as isize)
                                                * dpsi
                                                + *delta.offset(ip1 as isize)
                                                    * *delta.offset(ip1 as isize)
                                                    * dphi;
                                        }
                                    }
                                    eta = b / a;
                                } else if a <= 0.0f64 {
                                    d__1 = a * a - b * 4.0f64 * c__;
                                    eta = ((a as ::core::ffi::c_double
                                        - sqrt(
                                            (if d__1
                                                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                            {
                                                d__1
                                            } else {
                                                -d__1
                                            }),
                                        ))
                                        / (c__ as ::core::ffi::c_double * 2.0f64))
                                        as doublereal;
                                } else {
                                    d__1 = a * a - b * 4.0f64 * c__;
                                    eta = (b as ::core::ffi::c_double * 2.0f64
                                        / (a as ::core::ffi::c_double
                                            + sqrt(
                                                (if d__1
                                                    >= 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                {
                                                    d__1
                                                } else {
                                                    -d__1
                                                }),
                                            )))
                                        as doublereal;
                                }
                            } else {
                                temp = rhoinv + psi + phi;
                                if swtch != 0 {
                                    c__ = temp
                                        - *delta.offset(iim1 as isize) * dpsi
                                        - *delta.offset(iip1 as isize) * dphi;
                                    zz[0 as ::core::ffi::c_int as usize] = *delta
                                        .offset(iim1 as isize)
                                        * *delta.offset(iim1 as isize)
                                        * dpsi;
                                    zz[2 as ::core::ffi::c_int as usize] = *delta
                                        .offset(iip1 as isize)
                                        * *delta.offset(iip1 as isize)
                                        * dphi;
                                } else if orgati != 0 {
                                    temp1 =
                                        *z__.offset(iim1 as isize) / *delta.offset(iim1 as isize);
                                    temp1 *= temp1 as ::core::ffi::c_double;
                                    c__ = temp
                                        - *delta.offset(iip1 as isize) * (dpsi + dphi)
                                        - (*d__.offset(iim1 as isize) - *d__.offset(iip1 as isize))
                                            * temp1;
                                    zz[0 as ::core::ffi::c_int as usize] =
                                        *z__.offset(iim1 as isize) * *z__.offset(iim1 as isize);
                                    zz[2 as ::core::ffi::c_int as usize] = *delta
                                        .offset(iip1 as isize)
                                        * *delta.offset(iip1 as isize)
                                        * (dpsi - temp1 + dphi);
                                } else {
                                    temp1 =
                                        *z__.offset(iip1 as isize) / *delta.offset(iip1 as isize);
                                    temp1 *= temp1 as ::core::ffi::c_double;
                                    c__ = temp
                                        - *delta.offset(iim1 as isize) * (dpsi + dphi)
                                        - (*d__.offset(iip1 as isize) - *d__.offset(iim1 as isize))
                                            * temp1;
                                    zz[0 as ::core::ffi::c_int as usize] = *delta
                                        .offset(iim1 as isize)
                                        * *delta.offset(iim1 as isize)
                                        * (dpsi + (dphi - temp1));
                                    zz[2 as ::core::ffi::c_int as usize] =
                                        *z__.offset(iip1 as isize) * *z__.offset(iip1 as isize);
                                }
                                dlaed6__0(
                                    &raw mut niter,
                                    &raw mut orgati,
                                    &raw mut c__,
                                    delta.offset(iim1 as isize) as *mut doublereal,
                                    &raw mut zz as *mut doublereal,
                                    &raw mut w,
                                    &raw mut eta,
                                    info,
                                );
                                if *info != 0 as ::core::ffi::c_long {
                                    current_block = 14409496533122754037;
                                    break;
                                }
                            }
                            if w * eta >= 0.0f64 {
                                eta = -w / dw;
                            }
                            temp = tau + eta;
                            if temp > dltub || temp < dltlb {
                                if w < 0.0f64 {
                                    eta = ((dltub as ::core::ffi::c_double
                                        - tau as ::core::ffi::c_double)
                                        / 2.0f64)
                                        as doublereal;
                                } else {
                                    eta = ((dltlb as ::core::ffi::c_double
                                        - tau as ::core::ffi::c_double)
                                        / 2.0f64)
                                        as doublereal;
                                }
                            }
                            i__1 = *n;
                            j = 1 as integer;
                            while j <= i__1 {
                                let ref mut fresh3 = *delta.offset(j as isize);
                                *fresh3 -= eta as ::core::ffi::c_double;
                                j += 1;
                            }
                            tau += eta as ::core::ffi::c_double;
                            prew = w;
                            dpsi = 0.0f64 as doublereal;
                            psi = 0.0f64 as doublereal;
                            erretm = 0.0f64 as doublereal;
                            i__1 = iim1;
                            j = 1 as integer;
                            while j <= i__1 {
                                temp = *z__.offset(j as isize) / *delta.offset(j as isize);
                                psi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                                dpsi += (temp * temp) as ::core::ffi::c_double;
                                erretm += psi as ::core::ffi::c_double;
                                j += 1;
                            }
                            erretm = (if erretm >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {
                                erretm as ::core::ffi::c_double
                            } else {
                                -(erretm as ::core::ffi::c_double)
                            }) as doublereal;
                            dphi = 0.0f64 as doublereal;
                            phi = 0.0f64 as doublereal;
                            i__1 = iip1;
                            j = *n;
                            while j >= i__1 {
                                temp = *z__.offset(j as isize) / *delta.offset(j as isize);
                                phi += (*z__.offset(j as isize) * temp) as ::core::ffi::c_double;
                                dphi += (temp * temp) as ::core::ffi::c_double;
                                erretm += phi as ::core::ffi::c_double;
                                j -= 1;
                            }
                            temp = *z__.offset(ii as isize) / *delta.offset(ii as isize);
                            dw = dpsi + dphi + temp * temp;
                            temp = *z__.offset(ii as isize) * temp;
                            w = rhoinv + phi + psi + temp;
                            erretm = (phi - psi) * 8.0f64
                                + erretm
                                + rhoinv * 2.0f64
                                + (if temp >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    temp
                                } else {
                                    -temp
                                }) * 3.0f64
                                + (if tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    tau
                                } else {
                                    -tau
                                }) * dw;
                            if w * prew > 0.0f64
                                && (if w >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    w as ::core::ffi::c_double
                                } else {
                                    -(w as ::core::ffi::c_double)
                                }) > (if prew >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    prew as ::core::ffi::c_double
                                } else {
                                    -(prew as ::core::ffi::c_double)
                                }) / 10.0f64
                            {
                                swtch = (swtch == 0) as ::core::ffi::c_int as logical;
                            }
                            niter += 1;
                        }
                    }
                    match current_block {
                        14409496533122754037 => {}
                        _ => {
                            *info = 1 as integer;
                            if orgati != 0 {
                                *dlam = *d__.offset(*i__ as isize) + tau;
                            } else {
                                *dlam = *d__.offset(ip1 as isize) + tau;
                            }
                        }
                    }
                }
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
