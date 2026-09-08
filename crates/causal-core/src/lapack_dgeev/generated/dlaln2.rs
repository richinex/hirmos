#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
pub const TRUE_: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE_: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlaln2_(
    mut ltrans: *mut logical,
    mut na: *mut integer,
    mut nw: *mut integer,
    mut smin: *mut doublereal,
    mut ca: *mut doublereal,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut d1: *mut doublereal,
    mut d2: *mut doublereal,
    mut b: *mut doublereal,
    mut ldb: *mut integer,
    mut wr: *mut doublereal,
    mut wi: *mut doublereal,
    mut x: *mut doublereal,
    mut ldx: *mut integer,
    mut scale: *mut doublereal,
    mut xnorm: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    static mut zswap: [logical; 4] = [
        FALSE_ as logical,
        FALSE_ as logical,
        TRUE_ as logical,
        TRUE_ as logical,
    ];
    static mut rswap: [logical; 4] = [
        FALSE_ as logical,
        TRUE_ as logical,
        FALSE_ as logical,
        TRUE_ as logical,
    ];
    static mut ipivot: [integer; 16] = [
        1 as ::core::ffi::c_int as integer,
        2 as ::core::ffi::c_int as integer,
        3 as ::core::ffi::c_int as integer,
        4 as ::core::ffi::c_int as integer,
        2 as ::core::ffi::c_int as integer,
        1 as ::core::ffi::c_int as integer,
        4 as ::core::ffi::c_int as integer,
        3 as ::core::ffi::c_int as integer,
        3 as ::core::ffi::c_int as integer,
        4 as ::core::ffi::c_int as integer,
        1 as ::core::ffi::c_int as integer,
        2 as ::core::ffi::c_int as integer,
        4 as ::core::ffi::c_int as integer,
        3 as ::core::ffi::c_int as integer,
        2 as ::core::ffi::c_int as integer,
        1 as ::core::ffi::c_int as integer,
    ];
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut b_dim1: integer = 0;
    let mut b_offset: integer = 0;
    let mut x_dim1: integer = 0;
    let mut x_offset: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__4: doublereal = 0.;
    let mut d__5: doublereal = 0.;
    let mut d__6: doublereal = 0.;
    static mut equiv_0: [doublereal; 4] = [0.; 4];
    static mut equiv_1: [doublereal; 4] = [0.; 4];
    let mut j: integer = 0;
    let mut bi1: doublereal = 0.;
    let mut bi2: doublereal = 0.;
    let mut br1: doublereal = 0.;
    let mut br2: doublereal = 0.;
    let mut xi1: doublereal = 0.;
    let mut xi2: doublereal = 0.;
    let mut xr1: doublereal = 0.;
    let mut xr2: doublereal = 0.;
    let mut ci21: doublereal = 0.;
    let mut ci22: doublereal = 0.;
    let mut cr21: doublereal = 0.;
    let mut cr22: doublereal = 0.;
    let mut li21: doublereal = 0.;
    let mut csi: doublereal = 0.;
    let mut ui11: doublereal = 0.;
    let mut lr21: doublereal = 0.;
    let mut ui12: doublereal = 0.;
    let mut ui22: doublereal = 0.;
    let mut csr: doublereal = 0.;
    let mut ur11: doublereal = 0.;
    let mut ur12: doublereal = 0.;
    let mut ur22: doublereal = 0.;
    let mut bbnd: doublereal = 0.;
    let mut cmax: doublereal = 0.;
    let mut ui11r: doublereal = 0.;
    let mut ui12s: doublereal = 0.;
    let mut temp: doublereal = 0.;
    let mut ur11r: doublereal = 0.;
    let mut ur12s: doublereal = 0.;
    let mut u22abs: doublereal = 0.;
    let mut icmax: integer = 0;
    let mut bnorm: doublereal = 0.;
    let mut cnorm: doublereal = 0.;
    let mut smini: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dladiv_"]
        fn dladiv__0(
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    let mut bignum: doublereal = 0.;
    let mut smlnum: doublereal = 0.;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    b_dim1 = *ldb;
    b_offset = 1 as integer + b_dim1;
    b = b.offset(-(b_offset as isize));
    x_dim1 = *ldx;
    x_offset = 1 as integer + x_dim1;
    x = x.offset(-(x_offset as isize));
    smlnum = 2.0f64
        * dlamch__0(
            b"Safe minimum\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    bignum = 1.0f64 / smlnum;
    smini = (if *smin >= smlnum {
        *smin
    } else {
        smlnum as ::core::ffi::c_double
    }) as doublereal;
    *info = 0 as integer;
    *scale = 1.0f64 as doublereal;
    if *na == 1 as ::core::ffi::c_long {
        if *nw == 1 as ::core::ffi::c_long {
            csr = *ca
                * *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                - *wr * *d1;
            cnorm = (if csr >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                csr as ::core::ffi::c_double
            } else {
                -(csr as ::core::ffi::c_double)
            }) as doublereal;
            if cnorm < smini {
                csr = smini;
                cnorm = smini;
                *info = 1 as integer;
            }
            d__1 = *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            bnorm = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            if cnorm < 1.0f64 && bnorm > 1.0f64 {
                if bnorm > bignum * cnorm {
                    *scale = 1.0f64 / bnorm;
                }
            }
            *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) = *b
                .offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                * *scale
                / csr;
            d__1 = *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            *xnorm = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
        } else {
            csr = *ca
                * *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                - *wr * *d1;
            csi = -*wi * *d1;
            cnorm = ((if csr >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                csr as ::core::ffi::c_double
            } else {
                -(csr as ::core::ffi::c_double)
            }) + (if csi >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                csi as ::core::ffi::c_double
            } else {
                -(csi as ::core::ffi::c_double)
            })) as doublereal;
            if cnorm < smini {
                csr = smini;
                csi = 0.0f64 as doublereal;
                cnorm = smini;
                *info = 1 as integer;
            }
            d__1 = *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__2 = *b.offset(
                (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            );
            bnorm = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            })) as doublereal;
            if cnorm < 1.0f64 && bnorm > 1.0f64 {
                if bnorm > bignum * cnorm {
                    *scale = 1.0f64 / bnorm;
                }
            }
            d__1 = *scale
                * *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__2 = *scale
                * *b.offset(
                    (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
            dladiv__0(
                &raw mut d__1,
                &raw mut d__2,
                &raw mut csr,
                &raw mut csi,
                x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                x.offset(
                    (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
            );
            d__1 = *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__2 = *x.offset(
                (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            );
            *xnorm = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            })) as doublereal;
        }
    } else {
        equiv_1[0 as ::core::ffi::c_int as usize] = *ca
            * *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
            - *wr * *d1;
        equiv_1[3 as ::core::ffi::c_int as usize] =
            *ca * *a.offset(
                (((a_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            ) - *wr * *d2;
        if *ltrans != 0 {
            equiv_1[2 as ::core::ffi::c_int as usize] = *ca
                * *a.offset((a_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            equiv_1[1 as ::core::ffi::c_int as usize] = *ca
                * *a.offset(
                    (((a_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
        } else {
            equiv_1[1 as ::core::ffi::c_int as usize] = *ca
                * *a.offset((a_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            equiv_1[2 as ::core::ffi::c_int as usize] = *ca
                * *a.offset(
                    (((a_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
        }
        if *nw == 1 as ::core::ffi::c_long {
            cmax = 0.0f64 as doublereal;
            icmax = 0 as integer;
            j = 1 as integer;
            while j <= 4 as ::core::ffi::c_long {
                d__1 = equiv_1[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
                if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) > cmax
                {
                    d__1 = equiv_1[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
                    cmax = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                    icmax = j;
                }
                j += 1;
            }
            if cmax < smini {
                d__1 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) as doublereal;
                d__2 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__2 as ::core::ffi::c_double
                } else {
                    -(d__2 as ::core::ffi::c_double)
                }) as doublereal;
                bnorm = (if d__3 >= d__4 {
                    d__3 as ::core::ffi::c_double
                } else {
                    d__4 as ::core::ffi::c_double
                }) as doublereal;
                if smini < 1.0f64 && bnorm > 1.0f64 {
                    if bnorm > bignum * smini {
                        *scale = 1.0f64 / bnorm;
                    }
                }
                temp = *scale / smini;
                *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                    temp * *b.offset(
                        (b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                    );
                *x.offset((x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) =
                    temp * *b.offset(
                        (b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    );
                *xnorm = temp * bnorm;
                *info = 1 as integer;
                return 0 as ::core::ffi::c_int;
            }
            ur11 = equiv_1[(icmax as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
            cr21 = equiv_1[(ipivot[(((icmax as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - 3 as ::core::ffi::c_long) as usize]
                - 1 as ::core::ffi::c_long) as usize];
            ur12 = equiv_1[(ipivot[(((icmax as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - 2 as ::core::ffi::c_long) as usize]
                - 1 as ::core::ffi::c_long) as usize];
            cr22 = equiv_1[(ipivot[(((icmax as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as usize]
                - 1 as ::core::ffi::c_long) as usize];
            ur11r = 1.0f64 / ur11;
            lr21 = ur11r * cr21;
            ur22 = cr22 - ur12 * lr21;
            if (if ur22 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                ur22 as ::core::ffi::c_double
            } else {
                -(ur22 as ::core::ffi::c_double)
            }) < smini
            {
                ur22 = smini;
                *info = 1 as integer;
            }
            if rswap[(icmax as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] != 0 {
                br1 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                br2 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            } else {
                br1 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                br2 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            }
            br2 -= (lr21 * br1) as ::core::ffi::c_double;
            d__1 = br1 * (ur22 * ur11r);
            d__2 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            d__3 = (if br2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                br2 as ::core::ffi::c_double
            } else {
                -(br2 as ::core::ffi::c_double)
            }) as doublereal;
            bbnd = (if d__2 >= d__3 {
                d__2 as ::core::ffi::c_double
            } else {
                d__3 as ::core::ffi::c_double
            }) as doublereal;
            if bbnd > 1.0f64
                && (if ur22 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    ur22 as ::core::ffi::c_double
                } else {
                    -(ur22 as ::core::ffi::c_double)
                }) < 1.0f64
            {
                if bbnd
                    >= bignum as ::core::ffi::c_double
                        * (if ur22 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            ur22 as ::core::ffi::c_double
                        } else {
                            -(ur22 as ::core::ffi::c_double)
                        })
                {
                    *scale = 1.0f64 / bbnd;
                }
            }
            xr2 = br2 * *scale / ur22;
            xr1 = *scale * br1 * ur11r - xr2 * (ur11r * ur12);
            if zswap[(icmax as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] != 0 {
                *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                    xr2;
                *x.offset((x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) =
                    xr1;
            } else {
                *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                    xr1;
                *x.offset((x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) =
                    xr2;
            }
            d__1 = (if xr1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                xr1 as ::core::ffi::c_double
            } else {
                -(xr1 as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = (if xr2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                xr2 as ::core::ffi::c_double
            } else {
                -(xr2 as ::core::ffi::c_double)
            }) as doublereal;
            *xnorm = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            if *xnorm > 1.0f64 && cmax > 1.0f64 {
                if *xnorm > bignum / cmax {
                    temp = cmax / bignum;
                    *x.offset(
                        (x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                    ) = temp
                        * *x.offset(
                            (x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        );
                    *x.offset(
                        (x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    ) = temp
                        * *x.offset(
                            (x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                        );
                    *xnorm = temp * *xnorm;
                    *scale = temp * *scale;
                }
            }
        } else {
            equiv_0[0 as ::core::ffi::c_int as usize] = -*wi * *d1;
            equiv_0[1 as ::core::ffi::c_int as usize] = 0.0f64 as doublereal;
            equiv_0[2 as ::core::ffi::c_int as usize] = 0.0f64 as doublereal;
            equiv_0[3 as ::core::ffi::c_int as usize] = -*wi * *d2;
            cmax = 0.0f64 as doublereal;
            icmax = 0 as integer;
            j = 1 as integer;
            while j <= 4 as ::core::ffi::c_long {
                d__1 = equiv_1[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
                d__2 = equiv_0[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
                if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__2 as ::core::ffi::c_double
                } else {
                    -(d__2 as ::core::ffi::c_double)
                }) > cmax
                {
                    d__1 = equiv_1[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
                    d__2 = equiv_0[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
                    cmax = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__2 as ::core::ffi::c_double
                    } else {
                        -(d__2 as ::core::ffi::c_double)
                    })) as doublereal;
                    icmax = j;
                }
                j += 1;
            }
            if cmax < smini {
                d__1 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                d__2 = *b.offset(
                    (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
                d__5 = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__2 as ::core::ffi::c_double
                } else {
                    -(d__2 as ::core::ffi::c_double)
                })) as doublereal;
                d__3 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                d__4 = *b.offset(
                    (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                );
                d__6 = ((if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__3 as ::core::ffi::c_double
                } else {
                    -(d__3 as ::core::ffi::c_double)
                }) + (if d__4 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__4 as ::core::ffi::c_double
                } else {
                    -(d__4 as ::core::ffi::c_double)
                })) as doublereal;
                bnorm = (if d__5 >= d__6 {
                    d__5 as ::core::ffi::c_double
                } else {
                    d__6 as ::core::ffi::c_double
                }) as doublereal;
                if smini < 1.0f64 && bnorm > 1.0f64 {
                    if bnorm > bignum * smini {
                        *scale = 1.0f64 / bnorm;
                    }
                }
                temp = *scale / smini;
                *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                    temp * *b.offset(
                        (b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                    );
                *x.offset((x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) =
                    temp * *b.offset(
                        (b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    );
                *x.offset(
                    (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) = temp
                    * *b.offset(
                        (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                *x.offset(
                    (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                ) = temp
                    * *b.offset(
                        (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                *xnorm = temp * bnorm;
                *info = 1 as integer;
                return 0 as ::core::ffi::c_int;
            }
            ur11 = equiv_1[(icmax as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
            ui11 = equiv_0[(icmax as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
            cr21 = equiv_1[(ipivot[(((icmax as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - 3 as ::core::ffi::c_long) as usize]
                - 1 as ::core::ffi::c_long) as usize];
            ci21 = equiv_0[(ipivot[(((icmax as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - 3 as ::core::ffi::c_long) as usize]
                - 1 as ::core::ffi::c_long) as usize];
            ur12 = equiv_1[(ipivot[(((icmax as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - 2 as ::core::ffi::c_long) as usize]
                - 1 as ::core::ffi::c_long) as usize];
            ui12 = equiv_0[(ipivot[(((icmax as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - 2 as ::core::ffi::c_long) as usize]
                - 1 as ::core::ffi::c_long) as usize];
            cr22 = equiv_1[(ipivot[(((icmax as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as usize]
                - 1 as ::core::ffi::c_long) as usize];
            ci22 = equiv_0[(ipivot[(((icmax as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as usize]
                - 1 as ::core::ffi::c_long) as usize];
            if icmax == 1 as ::core::ffi::c_long || icmax == 4 as ::core::ffi::c_long {
                if (if ur11 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    ur11 as ::core::ffi::c_double
                } else {
                    -(ur11 as ::core::ffi::c_double)
                }) > (if ui11 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    ui11 as ::core::ffi::c_double
                } else {
                    -(ui11 as ::core::ffi::c_double)
                }) {
                    temp = ui11 / ur11;
                    d__1 = temp;
                    ur11r = (1.0f64
                        / (ur11 as ::core::ffi::c_double
                            * (d__1 as ::core::ffi::c_double * d__1 as ::core::ffi::c_double
                                + 1.0f64))) as doublereal;
                    ui11r = -temp * ur11r;
                } else {
                    temp = ur11 / ui11;
                    d__1 = temp;
                    ui11r = (-1.0f64
                        / (ui11 as ::core::ffi::c_double
                            * (d__1 as ::core::ffi::c_double * d__1 as ::core::ffi::c_double
                                + 1.0f64))) as doublereal;
                    ur11r = -temp * ui11r;
                }
                lr21 = cr21 * ur11r;
                li21 = cr21 * ui11r;
                ur12s = ur12 * ur11r;
                ui12s = ur12 * ui11r;
                ur22 = cr22 - ur12 * lr21;
                ui22 = ci22 - ur12 * li21;
            } else {
                ur11r = 1.0f64 / ur11;
                ui11r = 0.0f64 as doublereal;
                lr21 = cr21 * ur11r;
                li21 = ci21 * ur11r;
                ur12s = ur12 * ur11r;
                ui12s = ui12 * ur11r;
                ur22 = cr22 - ur12 * lr21 + ui12 * li21;
                ui22 = -ur12 * li21 - ui12 * lr21;
            }
            u22abs = ((if ur22 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                ur22 as ::core::ffi::c_double
            } else {
                -(ur22 as ::core::ffi::c_double)
            }) + (if ui22 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                ui22 as ::core::ffi::c_double
            } else {
                -(ui22 as ::core::ffi::c_double)
            })) as doublereal;
            if u22abs < smini {
                ur22 = smini;
                ui22 = 0.0f64 as doublereal;
                *info = 1 as integer;
            }
            if rswap[(icmax as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] != 0 {
                br2 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                br1 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                bi2 = *b.offset(
                    (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
                bi1 = *b.offset(
                    (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                );
            } else {
                br1 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
                br2 =
                    *b.offset((b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                bi1 = *b.offset(
                    (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
                bi2 = *b.offset(
                    (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                );
            }
            br2 = br2 - lr21 * br1 + li21 * bi1;
            bi2 = bi2 - li21 * br1 - lr21 * bi1;
            d__1 = (((if br1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                br1 as ::core::ffi::c_double
            } else {
                -(br1 as ::core::ffi::c_double)
            }) + (if bi1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                bi1 as ::core::ffi::c_double
            } else {
                -(bi1 as ::core::ffi::c_double)
            })) * (u22abs as ::core::ffi::c_double
                * ((if ur11r >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    ur11r as ::core::ffi::c_double
                } else {
                    -(ur11r as ::core::ffi::c_double)
                }) + (if ui11r >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    ui11r as ::core::ffi::c_double
                } else {
                    -(ui11r as ::core::ffi::c_double)
                })))) as doublereal;
            d__2 = ((if br2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                br2 as ::core::ffi::c_double
            } else {
                -(br2 as ::core::ffi::c_double)
            }) + (if bi2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                bi2 as ::core::ffi::c_double
            } else {
                -(bi2 as ::core::ffi::c_double)
            })) as doublereal;
            bbnd = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            if bbnd > 1.0f64 && u22abs < 1.0f64 {
                if bbnd >= bignum * u22abs {
                    *scale = 1.0f64 / bbnd;
                    br1 = *scale * br1;
                    bi1 = *scale * bi1;
                    br2 = *scale * br2;
                    bi2 = *scale * bi2;
                }
            }
            dladiv__0(
                &raw mut br2,
                &raw mut bi2,
                &raw mut ur22,
                &raw mut ui22,
                &raw mut xr2,
                &raw mut xi2,
            );
            xr1 = ur11r * br1 - ui11r * bi1 - ur12s * xr2 + ui12s * xi2;
            xi1 = ui11r * br1 + ur11r * bi1 - ui12s * xr2 - ur12s * xi2;
            if zswap[(icmax as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] != 0 {
                *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                    xr2;
                *x.offset((x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) =
                    xr1;
                *x.offset(
                    (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) = xi2;
                *x.offset(
                    (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                ) = xi1;
            } else {
                *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                    xr1;
                *x.offset((x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) =
                    xr2;
                *x.offset(
                    (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) = xi1;
                *x.offset(
                    (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                ) = xi2;
            }
            d__1 = ((if xr1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                xr1 as ::core::ffi::c_double
            } else {
                -(xr1 as ::core::ffi::c_double)
            }) + (if xi1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                xi1 as ::core::ffi::c_double
            } else {
                -(xi1 as ::core::ffi::c_double)
            })) as doublereal;
            d__2 = ((if xr2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                xr2 as ::core::ffi::c_double
            } else {
                -(xr2 as ::core::ffi::c_double)
            }) + (if xi2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                xi2 as ::core::ffi::c_double
            } else {
                -(xi2 as ::core::ffi::c_double)
            })) as doublereal;
            *xnorm = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            if *xnorm > 1.0f64 && cmax > 1.0f64 {
                if *xnorm > bignum / cmax {
                    temp = cmax / bignum;
                    *x.offset(
                        (x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                    ) = temp
                        * *x.offset(
                            (x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        );
                    *x.offset(
                        (x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    ) = temp
                        * *x.offset(
                            (x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                        );
                    *x.offset(
                        (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    ) = temp
                        * *x.offset(
                            (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_long) as isize,
                        );
                    *x.offset(
                        (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_long) as isize,
                    ) = temp
                        * *x.offset(
                            (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                + 2 as ::core::ffi::c_long) as isize,
                        );
                    *xnorm = temp * *xnorm;
                    *scale = temp * *scale;
                }
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
