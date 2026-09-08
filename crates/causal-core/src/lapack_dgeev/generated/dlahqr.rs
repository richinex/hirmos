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
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlahqr_(
    mut wantt: *mut logical,
    mut wantz: *mut logical,
    mut n: *mut integer,
    mut ilo: *mut integer,
    mut ihi: *mut integer,
    mut h__: *mut doublereal,
    mut ldh: *mut integer,
    mut wr: *mut doublereal,
    mut wi: *mut doublereal,
    mut iloz: *mut integer,
    mut ihiz: *mut integer,
    mut z__: *mut doublereal,
    mut ldz: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut h_dim1: integer = 0;
    let mut h_offset: integer = 0;
    let mut z_dim1: integer = 0;
    let mut z_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__4: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut l: integer = 0;
    let mut m: integer = 0;
    let mut s: doublereal = 0.;
    let mut v: [doublereal; 3] = [0.; 3];
    let mut i1: integer = 0;
    let mut i2: integer = 0;
    let mut t1: doublereal = 0.;
    let mut t2: doublereal = 0.;
    let mut t3: doublereal = 0.;
    let mut v2: doublereal = 0.;
    let mut v3: doublereal = 0.;
    let mut aa: doublereal = 0.;
    let mut ab: doublereal = 0.;
    let mut ba: doublereal = 0.;
    let mut bb: doublereal = 0.;
    let mut h11: doublereal = 0.;
    let mut h12: doublereal = 0.;
    let mut h21: doublereal = 0.;
    let mut h22: doublereal = 0.;
    let mut cs: doublereal = 0.;
    let mut nh: integer = 0;
    let mut sn: doublereal = 0.;
    let mut nr: integer = 0;
    let mut tr: doublereal = 0.;
    let mut nz: integer = 0;
    let mut det: doublereal = 0.;
    let mut h21s: doublereal = 0.;
    let mut its: integer = 0;
    let mut ulp: doublereal = 0.;
    let mut sum: doublereal = 0.;
    let mut tst: doublereal = 0.;
    let mut rt1i: doublereal = 0.;
    let mut rt2i: doublereal = 0.;
    let mut rt1r: doublereal = 0.;
    let mut rt2r: doublereal = 0.;
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
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dcopy"]
        fn f2c_dcopy_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlanv2_"]
        fn dlanv2__0(
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlabad_"]
        fn dlabad__0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlarfg_"]
        fn dlarfg__0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    let mut safmin: doublereal = 0.;
    let mut safmax: doublereal = 0.;
    let mut rtdisc: doublereal = 0.;
    let mut smlnum: doublereal = 0.;
    h_dim1 = *ldh;
    h_offset = 1 as integer + h_dim1;
    h__ = h__.offset(-(h_offset as isize));
    wr = wr.offset(-1);
    wi = wi.offset(-1);
    z_dim1 = *ldz;
    z_offset = 1 as integer + z_dim1;
    z__ = z__.offset(-(z_offset as isize));
    *info = 0 as integer;
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *ilo == *ihi {
        *wr.offset(*ilo as isize) = *h__.offset((*ilo + *ilo * h_dim1) as isize);
        *wi.offset(*ilo as isize) = 0.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    i__1 = (*ihi - 3 as ::core::ffi::c_long) as integer;
    j = *ilo;
    while j <= i__1 {
        *h__.offset((j + 2 as integer + j * h_dim1) as isize) = 0.0f64 as doublereal;
        *h__.offset((j + 3 as integer + j * h_dim1) as isize) = 0.0f64 as doublereal;
        j += 1;
    }
    if *ilo <= *ihi - 2 as ::core::ffi::c_long {
        *h__.offset((*ihi + (*ihi - 2 as integer) * h_dim1) as isize) = 0.0f64 as doublereal;
    }
    nh = (*ihi - *ilo + 1 as ::core::ffi::c_long) as integer;
    nz = (*ihiz - *iloz + 1 as ::core::ffi::c_long) as integer;
    safmin = dlamch__0(
        b"SAFE MINIMUM\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    safmax = 1.0f64 / safmin;
    dlabad__0(&raw mut safmin, &raw mut safmax);
    ulp = dlamch__0(
        b"PRECISION\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    smlnum = safmin * (nh as doublereal / ulp);
    if *wantt != 0 {
        i1 = 1 as integer;
        i2 = *n;
    }
    i__ = *ihi;
    loop {
        l = *ilo;
        if i__ < *ilo {
            return 0 as ::core::ffi::c_int;
        } else {
            its = 0 as integer;
            loop {
                if !(its <= 30 as ::core::ffi::c_long) {
                    current_block = 12094759598800435720;
                    break;
                }
                i__1 = (l as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                k = i__;
                while k >= i__1 {
                    d__1 = *h__.offset((k + (k - 1 as integer) * h_dim1) as isize);
                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) <= smlnum
                    {
                        break;
                    }
                    d__1 = *h__.offset((k - 1 as integer + (k - 1 as integer) * h_dim1) as isize);
                    d__2 = *h__.offset((k + k * h_dim1) as isize);
                    tst = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__2 as ::core::ffi::c_double
                    } else {
                        -(d__2 as ::core::ffi::c_double)
                    })) as doublereal;
                    if tst == 0.0f64 {
                        if k as ::core::ffi::c_long - 2 as ::core::ffi::c_long >= *ilo {
                            d__1 = *h__
                                .offset((k - 1 as integer + (k - 2 as integer) * h_dim1) as isize);
                            tst += (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            });
                        }
                        if k as ::core::ffi::c_long + 1 as ::core::ffi::c_long <= *ihi {
                            d__1 = *h__.offset((k + 1 as integer + k * h_dim1) as isize);
                            tst += (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            });
                        }
                    }
                    d__1 = *h__.offset((k + (k - 1 as integer) * h_dim1) as isize);
                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) <= ulp * tst
                    {
                        d__1 = *h__.offset((k + (k - 1 as integer) * h_dim1) as isize);
                        d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        d__2 = *h__.offset((k - 1 as integer + k * h_dim1) as isize);
                        d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) as doublereal;
                        ab = (if d__3 >= d__4 {
                            d__3 as ::core::ffi::c_double
                        } else {
                            d__4 as ::core::ffi::c_double
                        }) as doublereal;
                        d__1 = *h__.offset((k + (k - 1 as integer) * h_dim1) as isize);
                        d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        d__2 = *h__.offset((k - 1 as integer + k * h_dim1) as isize);
                        d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) as doublereal;
                        ba = (if d__3 <= d__4 {
                            d__3 as ::core::ffi::c_double
                        } else {
                            d__4 as ::core::ffi::c_double
                        }) as doublereal;
                        d__1 = *h__.offset((k + k * h_dim1) as isize);
                        d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        d__2 = *h__
                            .offset((k - 1 as integer + (k - 1 as integer) * h_dim1) as isize)
                            - *h__.offset((k + k * h_dim1) as isize);
                        d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) as doublereal;
                        aa = (if d__3 >= d__4 {
                            d__3 as ::core::ffi::c_double
                        } else {
                            d__4 as ::core::ffi::c_double
                        }) as doublereal;
                        d__1 = *h__.offset((k + k * h_dim1) as isize);
                        d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        d__2 = *h__
                            .offset((k - 1 as integer + (k - 1 as integer) * h_dim1) as isize)
                            - *h__.offset((k + k * h_dim1) as isize);
                        d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) as doublereal;
                        bb = (if d__3 <= d__4 {
                            d__3 as ::core::ffi::c_double
                        } else {
                            d__4 as ::core::ffi::c_double
                        }) as doublereal;
                        s = aa + ab;
                        d__1 = smlnum;
                        d__2 = ulp * (bb * (aa / s));
                        if ba * (ab / s)
                            <= (if d__1 >= d__2 {
                                d__1 as ::core::ffi::c_double
                            } else {
                                d__2 as ::core::ffi::c_double
                            })
                        {
                            break;
                        }
                    }
                    k -= 1;
                }
                l = k;
                if l > *ilo {
                    *h__.offset((l + (l - 1 as integer) * h_dim1) as isize) = 0.0f64 as doublereal;
                }
                if l >= i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                    current_block = 15957898596574006116;
                    break;
                }
                if *wantt == 0 {
                    i1 = l;
                    i2 = i__;
                }
                if its == 10 as ::core::ffi::c_long {
                    d__1 = *h__.offset((l + 1 as integer + l * h_dim1) as isize);
                    d__2 = *h__.offset((l + 2 as integer + (l + 1 as integer) * h_dim1) as isize);
                    s = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__2 as ::core::ffi::c_double
                    } else {
                        -(d__2 as ::core::ffi::c_double)
                    })) as doublereal;
                    h11 = s * 0.75f64 + *h__.offset((l + l * h_dim1) as isize);
                    h12 = (s as ::core::ffi::c_double * -0.4375f64) as doublereal;
                    h21 = s;
                    h22 = h11;
                } else if its == 20 as ::core::ffi::c_long {
                    d__1 = *h__.offset((i__ + (i__ - 1 as integer) * h_dim1) as isize);
                    d__2 =
                        *h__.offset((i__ - 1 as integer + (i__ - 2 as integer) * h_dim1) as isize);
                    s = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__2 as ::core::ffi::c_double
                    } else {
                        -(d__2 as ::core::ffi::c_double)
                    })) as doublereal;
                    h11 = s * 0.75f64 + *h__.offset((i__ + i__ * h_dim1) as isize);
                    h12 = (s as ::core::ffi::c_double * -0.4375f64) as doublereal;
                    h21 = s;
                    h22 = h11;
                } else {
                    h11 =
                        *h__.offset((i__ - 1 as integer + (i__ - 1 as integer) * h_dim1) as isize);
                    h21 = *h__.offset((i__ + (i__ - 1 as integer) * h_dim1) as isize);
                    h12 = *h__.offset((i__ - 1 as integer + i__ * h_dim1) as isize);
                    h22 = *h__.offset((i__ + i__ * h_dim1) as isize);
                }
                s = ((if h11 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    h11 as ::core::ffi::c_double
                } else {
                    -(h11 as ::core::ffi::c_double)
                }) + (if h12 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    h12 as ::core::ffi::c_double
                } else {
                    -(h12 as ::core::ffi::c_double)
                }) + (if h21 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    h21 as ::core::ffi::c_double
                } else {
                    -(h21 as ::core::ffi::c_double)
                }) + (if h22 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    h22 as ::core::ffi::c_double
                } else {
                    -(h22 as ::core::ffi::c_double)
                })) as doublereal;
                if s == 0.0f64 {
                    rt1r = 0.0f64 as doublereal;
                    rt1i = 0.0f64 as doublereal;
                    rt2r = 0.0f64 as doublereal;
                    rt2i = 0.0f64 as doublereal;
                } else {
                    h11 /= s as ::core::ffi::c_double;
                    h21 /= s as ::core::ffi::c_double;
                    h12 /= s as ::core::ffi::c_double;
                    h22 /= s as ::core::ffi::c_double;
                    tr = ((h11 as ::core::ffi::c_double + h22 as ::core::ffi::c_double) / 2.0f64)
                        as doublereal;
                    det = (h11 - tr) * (h22 - tr) - h12 * h21;
                    rtdisc = sqrt(if det >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        det
                    } else {
                        -det
                    }) as doublereal;
                    if det >= 0.0f64 {
                        rt1r = tr * s;
                        rt2r = rt1r;
                        rt1i = rtdisc * s;
                        rt2i = -rt1i;
                    } else {
                        rt1r = tr + rtdisc;
                        rt2r = tr - rtdisc;
                        d__1 = rt1r - h22;
                        d__2 = rt2r - h22;
                        if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) <= (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) {
                            rt1r *= s as ::core::ffi::c_double;
                            rt2r = rt1r;
                        } else {
                            rt2r *= s as ::core::ffi::c_double;
                            rt1r = rt2r;
                        }
                        rt1i = 0.0f64 as doublereal;
                        rt2i = 0.0f64 as doublereal;
                    }
                }
                i__1 = l;
                m = (i__ as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as integer;
                while m >= i__1 {
                    h21s = *h__.offset((m + 1 as integer + m * h_dim1) as isize);
                    d__1 = *h__.offset((m + m * h_dim1) as isize) - rt2r;
                    s = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) + (if rt2i >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        rt2i as ::core::ffi::c_double
                    } else {
                        -(rt2i as ::core::ffi::c_double)
                    }) + (if h21s >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        h21s as ::core::ffi::c_double
                    } else {
                        -(h21s as ::core::ffi::c_double)
                    })) as doublereal;
                    h21s = *h__.offset((m + 1 as integer + m * h_dim1) as isize) / s;
                    v[0 as ::core::ffi::c_int as usize] = h21s
                        * *h__.offset((m + (m + 1 as integer) * h_dim1) as isize)
                        + (*h__.offset((m + m * h_dim1) as isize) - rt1r)
                            * ((*h__.offset((m + m * h_dim1) as isize) - rt2r) / s)
                        - rt1i * (rt2i / s);
                    v[1 as ::core::ffi::c_int as usize] = h21s
                        * (*h__.offset((m + m * h_dim1) as isize)
                            + *h__
                                .offset((m + 1 as integer + (m + 1 as integer) * h_dim1) as isize)
                            - rt1r
                            - rt2r);
                    v[2 as ::core::ffi::c_int as usize] = h21s
                        * *h__.offset((m + 2 as integer + (m + 1 as integer) * h_dim1) as isize);
                    s = ((if v[0 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        v[0 as ::core::ffi::c_int as usize]
                    } else {
                        -v[0 as ::core::ffi::c_int as usize]
                    }) + (if v[1 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        v[1 as ::core::ffi::c_int as usize]
                    } else {
                        -v[1 as ::core::ffi::c_int as usize]
                    }) + (if v[2 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        v[2 as ::core::ffi::c_int as usize]
                    } else {
                        -v[2 as ::core::ffi::c_int as usize]
                    })) as doublereal;
                    v[0 as ::core::ffi::c_int as usize] /= s as ::core::ffi::c_double;
                    v[1 as ::core::ffi::c_int as usize] /= s as ::core::ffi::c_double;
                    v[2 as ::core::ffi::c_int as usize] /= s as ::core::ffi::c_double;
                    if m == l {
                        break;
                    }
                    d__1 = *h__.offset((m + (m - 1 as integer) * h_dim1) as isize);
                    d__2 = *h__.offset((m - 1 as integer + (m - 1 as integer) * h_dim1) as isize);
                    d__3 = *h__.offset((m + m * h_dim1) as isize);
                    d__4 = *h__.offset((m + 1 as integer + (m + 1 as integer) * h_dim1) as isize);
                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) * ((if v[1 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        v[1 as ::core::ffi::c_int as usize]
                    } else {
                        -v[1 as ::core::ffi::c_int as usize]
                    }) + (if v[2 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        v[2 as ::core::ffi::c_int as usize]
                    } else {
                        -v[2 as ::core::ffi::c_int as usize]
                    })) <= ulp as ::core::ffi::c_double
                        * (if v[0 as ::core::ffi::c_int as usize]
                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            v[0 as ::core::ffi::c_int as usize]
                        } else {
                            -v[0 as ::core::ffi::c_int as usize]
                        })
                        * ((if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) + (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__3 as ::core::ffi::c_double
                        } else {
                            -(d__3 as ::core::ffi::c_double)
                        }) + (if d__4 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__4 as ::core::ffi::c_double
                        } else {
                            -(d__4 as ::core::ffi::c_double)
                        }))
                    {
                        break;
                    }
                    m -= 1;
                }
                i__1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                k = m;
                while k <= i__1 {
                    i__2 = 3 as integer;
                    i__3 = (i__ as ::core::ffi::c_long - k as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                    nr = (if i__2 <= i__3 {
                        i__2 as ::core::ffi::c_long
                    } else {
                        i__3 as ::core::ffi::c_long
                    }) as integer;
                    if k > m {
                        f2c_dcopy_0(
                            &raw mut nr,
                            h__.offset((k + (k - 1 as integer) * h_dim1) as isize)
                                as *mut doublereal,
                            &raw mut c__1,
                            &raw mut v as *mut doublereal,
                            &raw mut c__1,
                        );
                    }
                    dlarfg__0(
                        &raw mut nr,
                        &raw mut v as *mut doublereal,
                        (&raw mut v as *mut doublereal).offset(1 as ::core::ffi::c_int as isize)
                            as *mut doublereal,
                        &raw mut c__1,
                        &raw mut t1,
                    );
                    if k > m {
                        *h__.offset((k + (k - 1 as integer) * h_dim1) as isize) =
                            v[0 as ::core::ffi::c_int as usize];
                        *h__.offset((k + 1 as integer + (k - 1 as integer) * h_dim1) as isize) =
                            0.0f64 as doublereal;
                        if k < i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                            *h__.offset(
                                (k + 2 as integer + (k - 1 as integer) * h_dim1) as isize,
                            ) = 0.0f64 as doublereal;
                        }
                    } else if m > l {
                        let ref mut fresh0 =
                            *h__.offset((k + (k - 1 as integer) * h_dim1) as isize);
                        *fresh0 *= (1.0f64 - t1) as ::core::ffi::c_double;
                    }
                    v2 = v[1 as ::core::ffi::c_int as usize];
                    t2 = t1 * v2;
                    if nr == 3 as ::core::ffi::c_long {
                        v3 = v[2 as ::core::ffi::c_int as usize];
                        t3 = t1 * v3;
                        i__2 = i2;
                        j = k;
                        while j <= i__2 {
                            sum = *h__.offset((k + j * h_dim1) as isize)
                                + v2 * *h__.offset((k + 1 as integer + j * h_dim1) as isize)
                                + v3 * *h__.offset((k + 2 as integer + j * h_dim1) as isize);
                            let ref mut fresh1 = *h__.offset((k + j * h_dim1) as isize);
                            *fresh1 -= (sum * t1) as ::core::ffi::c_double;
                            let ref mut fresh2 =
                                *h__.offset((k + 1 as integer + j * h_dim1) as isize);
                            *fresh2 -= (sum * t2) as ::core::ffi::c_double;
                            let ref mut fresh3 =
                                *h__.offset((k + 2 as integer + j * h_dim1) as isize);
                            *fresh3 -= (sum * t3) as ::core::ffi::c_double;
                            j += 1;
                        }
                        i__3 = (k as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as integer;
                        i__2 = (if i__3 <= i__ {
                            i__3 as ::core::ffi::c_long
                        } else {
                            i__ as ::core::ffi::c_long
                        }) as integer;
                        j = i1;
                        while j <= i__2 {
                            sum = *h__.offset((j + k * h_dim1) as isize)
                                + v2 * *h__.offset((j + (k + 1 as integer) * h_dim1) as isize)
                                + v3 * *h__.offset((j + (k + 2 as integer) * h_dim1) as isize);
                            let ref mut fresh4 = *h__.offset((j + k * h_dim1) as isize);
                            *fresh4 -= (sum * t1) as ::core::ffi::c_double;
                            let ref mut fresh5 =
                                *h__.offset((j + (k + 1 as integer) * h_dim1) as isize);
                            *fresh5 -= (sum * t2) as ::core::ffi::c_double;
                            let ref mut fresh6 =
                                *h__.offset((j + (k + 2 as integer) * h_dim1) as isize);
                            *fresh6 -= (sum * t3) as ::core::ffi::c_double;
                            j += 1;
                        }
                        if *wantz != 0 {
                            i__2 = *ihiz;
                            j = *iloz;
                            while j <= i__2 {
                                sum = *z__.offset((j + k * z_dim1) as isize)
                                    + v2 * *z__.offset((j + (k + 1 as integer) * z_dim1) as isize)
                                    + v3 * *z__.offset((j + (k + 2 as integer) * z_dim1) as isize);
                                let ref mut fresh7 = *z__.offset((j + k * z_dim1) as isize);
                                *fresh7 -= (sum * t1) as ::core::ffi::c_double;
                                let ref mut fresh8 =
                                    *z__.offset((j + (k + 1 as integer) * z_dim1) as isize);
                                *fresh8 -= (sum * t2) as ::core::ffi::c_double;
                                let ref mut fresh9 =
                                    *z__.offset((j + (k + 2 as integer) * z_dim1) as isize);
                                *fresh9 -= (sum * t3) as ::core::ffi::c_double;
                                j += 1;
                            }
                        }
                    } else if nr == 2 as ::core::ffi::c_long {
                        i__2 = i2;
                        j = k;
                        while j <= i__2 {
                            sum = *h__.offset((k + j * h_dim1) as isize)
                                + v2 * *h__.offset((k + 1 as integer + j * h_dim1) as isize);
                            let ref mut fresh10 = *h__.offset((k + j * h_dim1) as isize);
                            *fresh10 -= (sum * t1) as ::core::ffi::c_double;
                            let ref mut fresh11 =
                                *h__.offset((k + 1 as integer + j * h_dim1) as isize);
                            *fresh11 -= (sum * t2) as ::core::ffi::c_double;
                            j += 1;
                        }
                        i__2 = i__;
                        j = i1;
                        while j <= i__2 {
                            sum = *h__.offset((j + k * h_dim1) as isize)
                                + v2 * *h__.offset((j + (k + 1 as integer) * h_dim1) as isize);
                            let ref mut fresh12 = *h__.offset((j + k * h_dim1) as isize);
                            *fresh12 -= (sum * t1) as ::core::ffi::c_double;
                            let ref mut fresh13 =
                                *h__.offset((j + (k + 1 as integer) * h_dim1) as isize);
                            *fresh13 -= (sum * t2) as ::core::ffi::c_double;
                            j += 1;
                        }
                        if *wantz != 0 {
                            i__2 = *ihiz;
                            j = *iloz;
                            while j <= i__2 {
                                sum = *z__.offset((j + k * z_dim1) as isize)
                                    + v2 * *z__.offset((j + (k + 1 as integer) * z_dim1) as isize);
                                let ref mut fresh14 = *z__.offset((j + k * z_dim1) as isize);
                                *fresh14 -= (sum * t1) as ::core::ffi::c_double;
                                let ref mut fresh15 =
                                    *z__.offset((j + (k + 1 as integer) * z_dim1) as isize);
                                *fresh15 -= (sum * t2) as ::core::ffi::c_double;
                                j += 1;
                            }
                        }
                    }
                    k += 1;
                }
                its += 1;
            }
            match current_block {
                15957898596574006116 => {
                    if l == i__ {
                        *wr.offset(i__ as isize) = *h__.offset((i__ + i__ * h_dim1) as isize);
                        *wi.offset(i__ as isize) = 0.0f64 as doublereal;
                    } else if l == i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                        dlanv2__0(
                            h__.offset(
                                (i__ - 1 as integer + (i__ - 1 as integer) * h_dim1) as isize,
                            ) as *mut doublereal,
                            h__.offset((i__ - 1 as integer + i__ * h_dim1) as isize)
                                as *mut doublereal,
                            h__.offset((i__ + (i__ - 1 as integer) * h_dim1) as isize)
                                as *mut doublereal,
                            h__.offset((i__ + i__ * h_dim1) as isize) as *mut doublereal,
                            wr.offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) as *mut doublereal,
                            wi.offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) as *mut doublereal,
                            wr.offset(i__ as isize) as *mut doublereal,
                            wi.offset(i__ as isize) as *mut doublereal,
                            &raw mut cs,
                            &raw mut sn,
                        );
                        if *wantt != 0 {
                            if i2 > i__ {
                                i__1 = i2 - i__;
                                f2c_drot_0(
                                    &raw mut i__1,
                                    h__.offset(
                                        (i__ - 1 as integer + (i__ + 1 as integer) * h_dim1)
                                            as isize,
                                    ) as *mut doublereal,
                                    ldh,
                                    h__.offset((i__ + (i__ + 1 as integer) * h_dim1) as isize)
                                        as *mut doublereal,
                                    ldh,
                                    &raw mut cs,
                                    &raw mut sn,
                                );
                            }
                            i__1 = (i__ as ::core::ffi::c_long
                                - i1 as ::core::ffi::c_long
                                - 1 as ::core::ffi::c_long)
                                as integer;
                            f2c_drot_0(
                                &raw mut i__1,
                                h__.offset((i1 + (i__ - 1 as integer) * h_dim1) as isize)
                                    as *mut doublereal,
                                &raw mut c__1,
                                h__.offset((i1 + i__ * h_dim1) as isize) as *mut doublereal,
                                &raw mut c__1,
                                &raw mut cs,
                                &raw mut sn,
                            );
                        }
                        if *wantz != 0 {
                            f2c_drot_0(
                                &raw mut nz,
                                z__.offset((*iloz + (i__ - 1 as integer) * z_dim1) as isize)
                                    as *mut doublereal,
                                &raw mut c__1,
                                z__.offset((*iloz + i__ * z_dim1) as isize) as *mut doublereal,
                                &raw mut c__1,
                                &raw mut cs,
                                &raw mut sn,
                            );
                        }
                    }
                    i__ = (l as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                }
                _ => {
                    *info = i__;
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
}
