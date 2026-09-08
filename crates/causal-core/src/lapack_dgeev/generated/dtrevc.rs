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
static mut c_false: logical = FALSE_ as logical;
static mut c__1: integer = 1 as integer;
static mut c_b22: doublereal = 1.0f64;
static mut c_b25: doublereal = 0.0f64;
static mut c__2: integer = 2 as integer;
static mut c_true: logical = TRUE_ as logical;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dtrevc_(
    mut side: *mut ::core::ffi::c_char,
    mut howmny: *mut ::core::ffi::c_char,
    mut select: *mut logical,
    mut n: *mut integer,
    mut t: *mut doublereal,
    mut ldt: *mut integer,
    mut vl: *mut doublereal,
    mut ldvl: *mut integer,
    mut vr: *mut doublereal,
    mut ldvr: *mut integer,
    mut mm: *mut integer,
    mut m: *mut integer,
    mut work: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut t_dim1: integer = 0;
    let mut t_offset: integer = 0;
    let mut vl_dim1: integer = 0;
    let mut vl_offset: integer = 0;
    let mut vr_dim1: integer = 0;
    let mut vr_offset: integer = 0;
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
    let mut x: [doublereal; 4] = [0.; 4];
    let mut j1: integer = 0;
    let mut j2: integer = 0;
    let mut n2: integer = 0;
    let mut ii: integer = 0;
    let mut ki: integer = 0;
    let mut ip: integer = 0;
    let mut is: integer = 0;
    let mut wi: doublereal = 0.;
    let mut wr: doublereal = 0.;
    let mut rec: doublereal = 0.;
    let mut ulp: doublereal = 0.;
    let mut beta: doublereal = 0.;
    let mut emax: doublereal = 0.;
    let mut pair: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_ddot"]
        fn f2c_ddot_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> doublereal;
    }
    let mut allv: logical = 0;
    let mut ierr: integer = 0;
    let mut unfl: doublereal = 0.;
    let mut ovfl: doublereal = 0.;
    let mut smin: doublereal = 0.;
    let mut over: logical = 0;
    let mut vmax: doublereal = 0.;
    let mut jnxt: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dscal"]
        fn f2c_dscal_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut scale: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dgemv"]
        fn f2c_dgemv_0(
            _: *mut ::core::ffi::c_char,
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
    let mut remax: doublereal = 0.;
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
    let mut leftv: logical = 0;
    let mut bothv: logical = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_daxpy"]
        fn f2c_daxpy_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut vcrit: doublereal = 0.;
    let mut somev: logical = 0;
    let mut xnorm: doublereal = 0.;
    extern "C" {
        #[link_name = "dgeev_closure_dlaln2_"]
        fn dlaln2__0(
            _: *mut logical,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
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
        #[link_name = "dgelsd_closure_f2c_idamax"]
        fn f2c_idamax_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> integer;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut bignum: doublereal = 0.;
    let mut rightv: logical = 0;
    let mut smlnum: doublereal = 0.;
    select = select.offset(-1);
    t_dim1 = *ldt;
    t_offset = 1 as integer + t_dim1;
    t = t.offset(-(t_offset as isize));
    vl_dim1 = *ldvl;
    vl_offset = 1 as integer + vl_dim1;
    vl = vl.offset(-(vl_offset as isize));
    vr_dim1 = *ldvr;
    vr_offset = 1 as integer + vr_dim1;
    vr = vr.offset(-(vr_offset as isize));
    work = work.offset(-1);
    bothv = lsame__0(
        side,
        b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    rightv = (lsame__0(
        side,
        b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
        || bothv != 0) as ::core::ffi::c_int as logical;
    leftv = (lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
        || bothv != 0) as ::core::ffi::c_int as logical;
    allv = lsame__0(
        howmny,
        b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    over = lsame__0(
        howmny,
        b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    somev = lsame__0(
        howmny,
        b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    *info = 0 as integer;
    if rightv == 0 && leftv == 0 {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if allv == 0 && over == 0 && somev == 0 {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *ldt
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(6 as ::core::ffi::c_int) as integer;
    } else if *ldvl < 1 as ::core::ffi::c_long || leftv != 0 && *ldvl < *n {
        *info = -(8 as ::core::ffi::c_int) as integer;
    } else if *ldvr < 1 as ::core::ffi::c_long || rightv != 0 && *ldvr < *n {
        *info = -(10 as ::core::ffi::c_int) as integer;
    } else {
        if somev != 0 {
            *m = 0 as integer;
            pair = FALSE_ as logical;
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                if pair != 0 {
                    pair = FALSE_ as logical;
                    *select.offset(j as isize) = FALSE_ as logical;
                } else if j < *n {
                    if *t.offset((j + 1 as integer + j * t_dim1) as isize) == 0.0f64 {
                        if *select.offset(j as isize) != 0 {
                            *m += 1;
                        }
                    } else {
                        pair = TRUE_ as logical;
                        if *select.offset(j as isize) != 0
                            || *select.offset(
                                (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                            ) != 0
                        {
                            *select.offset(j as isize) = TRUE_ as logical;
                            *m += 2 as ::core::ffi::c_long;
                        }
                    }
                } else if *select.offset(*n as isize) != 0 {
                    *m += 1;
                }
                j += 1;
            }
        } else {
            *m = *n;
        }
        if *mm < *m {
            *info = -(11 as ::core::ffi::c_int) as integer;
        }
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DTREVC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    unfl = dlamch__0(
        b"Safe minimum\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    ovfl = 1.0f64 / unfl;
    dlabad__0(&raw mut unfl, &raw mut ovfl);
    ulp = dlamch__0(
        b"Precision\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    smlnum = unfl * (*n as doublereal / ulp);
    bignum = (1.0f64 - ulp) / smlnum;
    *work.offset(1 as ::core::ffi::c_int as isize) = 0.0f64 as doublereal;
    i__1 = *n;
    j = 2 as integer;
    while j <= i__1 {
        *work.offset(j as isize) = 0.0f64 as doublereal;
        i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        i__ = 1 as integer;
        while i__ <= i__2 {
            d__1 = *t.offset((i__ + j * t_dim1) as isize);
            let ref mut fresh0 = *work.offset(j as isize);
            *fresh0 += (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            });
            i__ += 1;
        }
        j += 1;
    }
    n2 = *n << 1 as ::core::ffi::c_int;
    if rightv != 0 {
        ip = 0 as integer;
        is = *m;
        ki = *n;
        while ki >= 1 as ::core::ffi::c_long {
            if !(ip == 1 as ::core::ffi::c_long) {
                if !(ki == 1 as ::core::ffi::c_long) {
                    if !(*t.offset((ki + (ki - 1 as integer) * t_dim1) as isize) == 0.0f64) {
                        ip = -(1 as ::core::ffi::c_int) as integer;
                    }
                }
                if somev != 0 {
                    if ip == 0 as ::core::ffi::c_long {
                        if *select.offset(ki as isize) == 0 {
                            current_block = 8248608242100809048;
                        } else {
                            current_block = 2220405792722996547;
                        }
                    } else if *select
                        .offset((ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                        == 0
                    {
                        current_block = 8248608242100809048;
                    } else {
                        current_block = 2220405792722996547;
                    }
                } else {
                    current_block = 2220405792722996547;
                }
                match current_block {
                    8248608242100809048 => {}
                    _ => {
                        wr = *t.offset((ki + ki * t_dim1) as isize);
                        wi = 0.0f64 as doublereal;
                        if ip != 0 as ::core::ffi::c_long {
                            d__1 = *t.offset((ki + (ki - 1 as integer) * t_dim1) as isize);
                            d__2 = *t.offset((ki - 1 as integer + ki * t_dim1) as isize);
                            wi = (sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ) * sqrt(
                                (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__2
                                } else {
                                    -d__2
                                }),
                            )) as doublereal;
                        }
                        d__1 = (ulp as ::core::ffi::c_double
                            * ((if wr >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                wr as ::core::ffi::c_double
                            } else {
                                -(wr as ::core::ffi::c_double)
                            }) + (if wi >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                wi as ::core::ffi::c_double
                            } else {
                                -(wi as ::core::ffi::c_double)
                            }))) as doublereal;
                        smin = (if d__1 >= smlnum {
                            d__1 as ::core::ffi::c_double
                        } else {
                            smlnum as ::core::ffi::c_double
                        }) as doublereal;
                        if ip == 0 as ::core::ffi::c_long {
                            *work.offset((ki + *n) as isize) = 1.0f64 as doublereal;
                            i__1 =
                                (ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                            k = 1 as integer;
                            while k <= i__1 {
                                *work.offset((k + *n) as isize) =
                                    -*t.offset((k + ki * t_dim1) as isize);
                                k += 1;
                            }
                            jnxt =
                                (ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                            j = (ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                            while j >= 1 as ::core::ffi::c_long {
                                if !(j > jnxt) {
                                    j1 = j;
                                    j2 = j;
                                    jnxt = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as integer;
                                    if j > 1 as ::core::ffi::c_long {
                                        if *t.offset((j + (j - 1 as integer) * t_dim1) as isize)
                                            != 0.0f64
                                        {
                                            j1 = (j as ::core::ffi::c_long
                                                - 1 as ::core::ffi::c_long)
                                                as integer;
                                            jnxt = (j as ::core::ffi::c_long
                                                - 2 as ::core::ffi::c_long)
                                                as integer;
                                        }
                                    }
                                    if j1 == j2 {
                                        dlaln2__0(
                                            &raw mut c_false,
                                            &raw mut c__1,
                                            &raw mut c__1,
                                            &raw mut smin,
                                            &raw mut c_b22,
                                            t.offset((j + j * t_dim1) as isize) as *mut doublereal,
                                            ldt,
                                            &raw mut c_b22,
                                            &raw mut c_b22,
                                            work.offset((j + *n) as isize) as *mut doublereal,
                                            n,
                                            &raw mut wr,
                                            &raw mut c_b25,
                                            &raw mut x as *mut doublereal,
                                            &raw mut c__2,
                                            &raw mut scale,
                                            &raw mut xnorm,
                                            &raw mut ierr,
                                        );
                                        if xnorm > 1.0f64 {
                                            if *work.offset(j as isize) > bignum / xnorm {
                                                x[0 as ::core::ffi::c_int as usize] /=
                                                    xnorm as ::core::ffi::c_double;
                                                scale /= xnorm as ::core::ffi::c_double;
                                            }
                                        }
                                        if scale != 1.0f64 {
                                            f2c_dscal_0(
                                                &raw mut ki,
                                                &raw mut scale,
                                                work.offset(
                                                    (*n + 1 as ::core::ffi::c_long) as isize,
                                                )
                                                    as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                        }
                                        *work.offset((j + *n) as isize) =
                                            x[0 as ::core::ffi::c_int as usize];
                                        i__1 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as integer;
                                        d__1 = -x[0 as ::core::ffi::c_int as usize];
                                        f2c_daxpy_0(
                                            &raw mut i__1,
                                            &raw mut d__1,
                                            t.offset(
                                                (j as ::core::ffi::c_long
                                                    * t_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                    } else {
                                        dlaln2__0(
                                            &raw mut c_false,
                                            &raw mut c__2,
                                            &raw mut c__1,
                                            &raw mut smin,
                                            &raw mut c_b22,
                                            t.offset(
                                                (j - 1 as integer + (j - 1 as integer) * t_dim1)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            ldt,
                                            &raw mut c_b22,
                                            &raw mut c_b22,
                                            work.offset((j - 1 as integer + *n) as isize)
                                                as *mut doublereal,
                                            n,
                                            &raw mut wr,
                                            &raw mut c_b25,
                                            &raw mut x as *mut doublereal,
                                            &raw mut c__2,
                                            &raw mut scale,
                                            &raw mut xnorm,
                                            &raw mut ierr,
                                        );
                                        if xnorm > 1.0f64 {
                                            d__1 = *work.offset(
                                                (j as ::core::ffi::c_long
                                                    - 1 as ::core::ffi::c_long)
                                                    as isize,
                                            );
                                            d__2 = *work.offset(j as isize);
                                            beta = (if d__1 >= d__2 {
                                                d__1 as ::core::ffi::c_double
                                            } else {
                                                d__2 as ::core::ffi::c_double
                                            })
                                                as doublereal;
                                            if beta > bignum / xnorm {
                                                x[0 as ::core::ffi::c_int as usize] /=
                                                    xnorm as ::core::ffi::c_double;
                                                x[1 as ::core::ffi::c_int as usize] /=
                                                    xnorm as ::core::ffi::c_double;
                                                scale /= xnorm as ::core::ffi::c_double;
                                            }
                                        }
                                        if scale != 1.0f64 {
                                            f2c_dscal_0(
                                                &raw mut ki,
                                                &raw mut scale,
                                                work.offset(
                                                    (*n + 1 as ::core::ffi::c_long) as isize,
                                                )
                                                    as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                        }
                                        *work.offset((j - 1 as integer + *n) as isize) =
                                            x[0 as ::core::ffi::c_int as usize];
                                        *work.offset((j + *n) as isize) =
                                            x[1 as ::core::ffi::c_int as usize];
                                        i__1 = (j as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                            as integer;
                                        d__1 = -x[0 as ::core::ffi::c_int as usize];
                                        f2c_daxpy_0(
                                            &raw mut i__1,
                                            &raw mut d__1,
                                            t.offset(
                                                ((j as ::core::ffi::c_long
                                                    - 1 as ::core::ffi::c_long)
                                                    * t_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                        i__1 = (j as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                            as integer;
                                        d__1 = -x[1 as ::core::ffi::c_int as usize];
                                        f2c_daxpy_0(
                                            &raw mut i__1,
                                            &raw mut d__1,
                                            t.offset(
                                                (j as ::core::ffi::c_long
                                                    * t_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                    }
                                }
                                j -= 1;
                            }
                            if over == 0 {
                                f2c_dcopy_0(
                                    &raw mut ki,
                                    work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                        as *mut doublereal,
                                    &raw mut c__1,
                                    vr.offset(
                                        (is as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                ii = f2c_idamax_0(
                                    &raw mut ki,
                                    vr.offset(
                                        (is as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                d__1 = *vr.offset((ii + is * vr_dim1) as isize);
                                remax = (1.0f64
                                    / (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    })) as doublereal;
                                f2c_dscal_0(
                                    &raw mut ki,
                                    &raw mut remax,
                                    vr.offset(
                                        (is as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                i__1 = *n;
                                k = (ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                while k <= i__1 {
                                    *vr.offset((k + is * vr_dim1) as isize) = 0.0f64 as doublereal;
                                    k += 1;
                                }
                            } else {
                                if ki > 1 as ::core::ffi::c_long {
                                    i__1 = (ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as integer;
                                    f2c_dgemv_0(
                                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        n,
                                        &raw mut i__1,
                                        &raw mut c_b22,
                                        vr.offset(vr_offset as isize) as *mut doublereal,
                                        ldvr,
                                        work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                            as *mut doublereal,
                                        &raw mut c__1,
                                        work.offset((ki + *n) as isize) as *mut doublereal,
                                        vr.offset(
                                            (ki as ::core::ffi::c_long
                                                * vr_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                }
                                ii = f2c_idamax_0(
                                    n,
                                    vr.offset(
                                        (ki as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                d__1 = *vr.offset((ii + ki * vr_dim1) as isize);
                                remax = (1.0f64
                                    / (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    })) as doublereal;
                                f2c_dscal_0(
                                    n,
                                    &raw mut remax,
                                    vr.offset(
                                        (ki as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                            }
                        } else {
                            d__1 = *t.offset((ki - 1 as integer + ki * t_dim1) as isize);
                            d__2 = *t.offset((ki + (ki - 1 as integer) * t_dim1) as isize);
                            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) >= (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__2 as ::core::ffi::c_double
                            } else {
                                -(d__2 as ::core::ffi::c_double)
                            }) {
                                *work.offset((ki - 1 as integer + *n) as isize) =
                                    1.0f64 as doublereal;
                                *work.offset((ki + n2) as isize) =
                                    wi / *t.offset((ki - 1 as integer + ki * t_dim1) as isize);
                            } else {
                                *work.offset((ki - 1 as integer + *n) as isize) =
                                    -wi / *t.offset((ki + (ki - 1 as integer) * t_dim1) as isize);
                                *work.offset((ki + n2) as isize) = 1.0f64 as doublereal;
                            }
                            *work.offset((ki + *n) as isize) = 0.0f64 as doublereal;
                            *work.offset((ki - 1 as integer + n2) as isize) = 0.0f64 as doublereal;
                            i__1 =
                                (ki as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as integer;
                            k = 1 as integer;
                            while k <= i__1 {
                                *work.offset((k + *n) as isize) = -*work
                                    .offset((ki - 1 as integer + *n) as isize)
                                    * *t.offset((k + (ki - 1 as integer) * t_dim1) as isize);
                                *work.offset((k + n2) as isize) = -*work.offset((ki + n2) as isize)
                                    * *t.offset((k + ki * t_dim1) as isize);
                                k += 1;
                            }
                            jnxt =
                                (ki as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as integer;
                            j = (ki as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as integer;
                            while j >= 1 as ::core::ffi::c_long {
                                if !(j > jnxt) {
                                    j1 = j;
                                    j2 = j;
                                    jnxt = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as integer;
                                    if j > 1 as ::core::ffi::c_long {
                                        if *t.offset((j + (j - 1 as integer) * t_dim1) as isize)
                                            != 0.0f64
                                        {
                                            j1 = (j as ::core::ffi::c_long
                                                - 1 as ::core::ffi::c_long)
                                                as integer;
                                            jnxt = (j as ::core::ffi::c_long
                                                - 2 as ::core::ffi::c_long)
                                                as integer;
                                        }
                                    }
                                    if j1 == j2 {
                                        dlaln2__0(
                                            &raw mut c_false,
                                            &raw mut c__1,
                                            &raw mut c__2,
                                            &raw mut smin,
                                            &raw mut c_b22,
                                            t.offset((j + j * t_dim1) as isize) as *mut doublereal,
                                            ldt,
                                            &raw mut c_b22,
                                            &raw mut c_b22,
                                            work.offset((j + *n) as isize) as *mut doublereal,
                                            n,
                                            &raw mut wr,
                                            &raw mut wi,
                                            &raw mut x as *mut doublereal,
                                            &raw mut c__2,
                                            &raw mut scale,
                                            &raw mut xnorm,
                                            &raw mut ierr,
                                        );
                                        if xnorm > 1.0f64 {
                                            if *work.offset(j as isize) > bignum / xnorm {
                                                x[0 as ::core::ffi::c_int as usize] /=
                                                    xnorm as ::core::ffi::c_double;
                                                x[2 as ::core::ffi::c_int as usize] /=
                                                    xnorm as ::core::ffi::c_double;
                                                scale /= xnorm as ::core::ffi::c_double;
                                            }
                                        }
                                        if scale != 1.0f64 {
                                            f2c_dscal_0(
                                                &raw mut ki,
                                                &raw mut scale,
                                                work.offset(
                                                    (*n + 1 as ::core::ffi::c_long) as isize,
                                                )
                                                    as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            f2c_dscal_0(
                                                &raw mut ki,
                                                &raw mut scale,
                                                work.offset(
                                                    (n2 as ::core::ffi::c_long
                                                        + 1 as ::core::ffi::c_long)
                                                        as isize,
                                                )
                                                    as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                        }
                                        *work.offset((j + *n) as isize) =
                                            x[0 as ::core::ffi::c_int as usize];
                                        *work.offset((j + n2) as isize) =
                                            x[2 as ::core::ffi::c_int as usize];
                                        i__1 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as integer;
                                        d__1 = -x[0 as ::core::ffi::c_int as usize];
                                        f2c_daxpy_0(
                                            &raw mut i__1,
                                            &raw mut d__1,
                                            t.offset(
                                                (j as ::core::ffi::c_long
                                                    * t_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                        i__1 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as integer;
                                        d__1 = -x[2 as ::core::ffi::c_int as usize];
                                        f2c_daxpy_0(
                                            &raw mut i__1,
                                            &raw mut d__1,
                                            t.offset(
                                                (j as ::core::ffi::c_long
                                                    * t_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset(
                                                (n2 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                    } else {
                                        dlaln2__0(
                                            &raw mut c_false,
                                            &raw mut c__2,
                                            &raw mut c__2,
                                            &raw mut smin,
                                            &raw mut c_b22,
                                            t.offset(
                                                (j - 1 as integer + (j - 1 as integer) * t_dim1)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            ldt,
                                            &raw mut c_b22,
                                            &raw mut c_b22,
                                            work.offset((j - 1 as integer + *n) as isize)
                                                as *mut doublereal,
                                            n,
                                            &raw mut wr,
                                            &raw mut wi,
                                            &raw mut x as *mut doublereal,
                                            &raw mut c__2,
                                            &raw mut scale,
                                            &raw mut xnorm,
                                            &raw mut ierr,
                                        );
                                        if xnorm > 1.0f64 {
                                            d__1 = *work.offset(
                                                (j as ::core::ffi::c_long
                                                    - 1 as ::core::ffi::c_long)
                                                    as isize,
                                            );
                                            d__2 = *work.offset(j as isize);
                                            beta = (if d__1 >= d__2 {
                                                d__1 as ::core::ffi::c_double
                                            } else {
                                                d__2 as ::core::ffi::c_double
                                            })
                                                as doublereal;
                                            if beta > bignum / xnorm {
                                                rec = 1.0f64 / xnorm;
                                                x[0 as ::core::ffi::c_int as usize] *=
                                                    rec as ::core::ffi::c_double;
                                                x[2 as ::core::ffi::c_int as usize] *=
                                                    rec as ::core::ffi::c_double;
                                                x[1 as ::core::ffi::c_int as usize] *=
                                                    rec as ::core::ffi::c_double;
                                                x[3 as ::core::ffi::c_int as usize] *=
                                                    rec as ::core::ffi::c_double;
                                                scale *= rec as ::core::ffi::c_double;
                                            }
                                        }
                                        if scale != 1.0f64 {
                                            f2c_dscal_0(
                                                &raw mut ki,
                                                &raw mut scale,
                                                work.offset(
                                                    (*n + 1 as ::core::ffi::c_long) as isize,
                                                )
                                                    as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            f2c_dscal_0(
                                                &raw mut ki,
                                                &raw mut scale,
                                                work.offset(
                                                    (n2 as ::core::ffi::c_long
                                                        + 1 as ::core::ffi::c_long)
                                                        as isize,
                                                )
                                                    as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                        }
                                        *work.offset((j - 1 as integer + *n) as isize) =
                                            x[0 as ::core::ffi::c_int as usize];
                                        *work.offset((j + *n) as isize) =
                                            x[1 as ::core::ffi::c_int as usize];
                                        *work.offset((j - 1 as integer + n2) as isize) =
                                            x[2 as ::core::ffi::c_int as usize];
                                        *work.offset((j + n2) as isize) =
                                            x[3 as ::core::ffi::c_int as usize];
                                        i__1 = (j as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                            as integer;
                                        d__1 = -x[0 as ::core::ffi::c_int as usize];
                                        f2c_daxpy_0(
                                            &raw mut i__1,
                                            &raw mut d__1,
                                            t.offset(
                                                ((j as ::core::ffi::c_long
                                                    - 1 as ::core::ffi::c_long)
                                                    * t_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                        i__1 = (j as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                            as integer;
                                        d__1 = -x[1 as ::core::ffi::c_int as usize];
                                        f2c_daxpy_0(
                                            &raw mut i__1,
                                            &raw mut d__1,
                                            t.offset(
                                                (j as ::core::ffi::c_long
                                                    * t_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                        i__1 = (j as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                            as integer;
                                        d__1 = -x[2 as ::core::ffi::c_int as usize];
                                        f2c_daxpy_0(
                                            &raw mut i__1,
                                            &raw mut d__1,
                                            t.offset(
                                                ((j as ::core::ffi::c_long
                                                    - 1 as ::core::ffi::c_long)
                                                    * t_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset(
                                                (n2 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                        i__1 = (j as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                            as integer;
                                        d__1 = -x[3 as ::core::ffi::c_int as usize];
                                        f2c_daxpy_0(
                                            &raw mut i__1,
                                            &raw mut d__1,
                                            t.offset(
                                                (j as ::core::ffi::c_long
                                                    * t_dim1 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset(
                                                (n2 as ::core::ffi::c_long
                                                    + 1 as ::core::ffi::c_long)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        );
                                    }
                                }
                                j -= 1;
                            }
                            if over == 0 {
                                f2c_dcopy_0(
                                    &raw mut ki,
                                    work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                        as *mut doublereal,
                                    &raw mut c__1,
                                    vr.offset(
                                        ((is as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                f2c_dcopy_0(
                                    &raw mut ki,
                                    work.offset(
                                        (n2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                    vr.offset(
                                        (is as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                emax = 0.0f64 as doublereal;
                                i__1 = ki;
                                k = 1 as integer;
                                while k <= i__1 {
                                    d__3 = emax;
                                    d__1 = *vr.offset((k + (is - 1 as integer) * vr_dim1) as isize);
                                    d__2 = *vr.offset((k + is * vr_dim1) as isize);
                                    d__4 = ((if d__1
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    }) + (if d__2
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__2 as ::core::ffi::c_double
                                    } else {
                                        -(d__2 as ::core::ffi::c_double)
                                    })) as doublereal;
                                    emax = (if d__3 >= d__4 {
                                        d__3 as ::core::ffi::c_double
                                    } else {
                                        d__4 as ::core::ffi::c_double
                                    }) as doublereal;
                                    k += 1;
                                }
                                remax = 1.0f64 / emax;
                                f2c_dscal_0(
                                    &raw mut ki,
                                    &raw mut remax,
                                    vr.offset(
                                        ((is as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                f2c_dscal_0(
                                    &raw mut ki,
                                    &raw mut remax,
                                    vr.offset(
                                        (is as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                i__1 = *n;
                                k = (ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                while k <= i__1 {
                                    *vr.offset((k + (is - 1 as integer) * vr_dim1) as isize) =
                                        0.0f64 as doublereal;
                                    *vr.offset((k + is * vr_dim1) as isize) = 0.0f64 as doublereal;
                                    k += 1;
                                }
                            } else {
                                if ki > 2 as ::core::ffi::c_long {
                                    i__1 = (ki as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                        as integer;
                                    f2c_dgemv_0(
                                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        n,
                                        &raw mut i__1,
                                        &raw mut c_b22,
                                        vr.offset(vr_offset as isize) as *mut doublereal,
                                        ldvr,
                                        work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                            as *mut doublereal,
                                        &raw mut c__1,
                                        work.offset((ki - 1 as integer + *n) as isize)
                                            as *mut doublereal,
                                        vr.offset(
                                            ((ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                                * vr_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                    i__1 = (ki as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
                                        as integer;
                                    f2c_dgemv_0(
                                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        n,
                                        &raw mut i__1,
                                        &raw mut c_b22,
                                        vr.offset(vr_offset as isize) as *mut doublereal,
                                        ldvr,
                                        work.offset(
                                            (n2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                        work.offset((ki + n2) as isize) as *mut doublereal,
                                        vr.offset(
                                            (ki as ::core::ffi::c_long
                                                * vr_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                } else {
                                    f2c_dscal_0(
                                        n,
                                        work.offset((ki - 1 as integer + *n) as isize)
                                            as *mut doublereal,
                                        vr.offset(
                                            ((ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                                * vr_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                    f2c_dscal_0(
                                        n,
                                        work.offset((ki + n2) as isize) as *mut doublereal,
                                        vr.offset(
                                            (ki as ::core::ffi::c_long
                                                * vr_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                }
                                emax = 0.0f64 as doublereal;
                                i__1 = *n;
                                k = 1 as integer;
                                while k <= i__1 {
                                    d__3 = emax;
                                    d__1 = *vr.offset((k + (ki - 1 as integer) * vr_dim1) as isize);
                                    d__2 = *vr.offset((k + ki * vr_dim1) as isize);
                                    d__4 = ((if d__1
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    }) + (if d__2
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__2 as ::core::ffi::c_double
                                    } else {
                                        -(d__2 as ::core::ffi::c_double)
                                    })) as doublereal;
                                    emax = (if d__3 >= d__4 {
                                        d__3 as ::core::ffi::c_double
                                    } else {
                                        d__4 as ::core::ffi::c_double
                                    }) as doublereal;
                                    k += 1;
                                }
                                remax = 1.0f64 / emax;
                                f2c_dscal_0(
                                    n,
                                    &raw mut remax,
                                    vr.offset(
                                        ((ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                f2c_dscal_0(
                                    n,
                                    &raw mut remax,
                                    vr.offset(
                                        (ki as ::core::ffi::c_long * vr_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                            }
                        }
                        is -= 1;
                        if ip != 0 as ::core::ffi::c_long {
                            is -= 1;
                        }
                    }
                }
            }
            if ip == 1 as ::core::ffi::c_long {
                ip = 0 as integer;
            }
            if ip == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
                ip = 1 as integer;
            }
            ki -= 1;
        }
    }
    if leftv != 0 {
        ip = 0 as integer;
        is = 1 as integer;
        i__1 = *n;
        ki = 1 as integer;
        while ki <= i__1 {
            if !(ip == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) {
                if !(ki == *n) {
                    if !(*t.offset((ki + 1 as integer + ki * t_dim1) as isize) == 0.0f64) {
                        ip = 1 as integer;
                    }
                }
                if somev != 0 {
                    if *select.offset(ki as isize) == 0 {
                        current_block = 1682960031492931211;
                    } else {
                        current_block = 7991679940794782184;
                    }
                } else {
                    current_block = 7991679940794782184;
                }
                match current_block {
                    1682960031492931211 => {}
                    _ => {
                        wr = *t.offset((ki + ki * t_dim1) as isize);
                        wi = 0.0f64 as doublereal;
                        if ip != 0 as ::core::ffi::c_long {
                            d__1 = *t.offset((ki + (ki + 1 as integer) * t_dim1) as isize);
                            d__2 = *t.offset((ki + 1 as integer + ki * t_dim1) as isize);
                            wi = (sqrt(
                                (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1
                                } else {
                                    -d__1
                                }),
                            ) * sqrt(
                                (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__2
                                } else {
                                    -d__2
                                }),
                            )) as doublereal;
                        }
                        d__1 = (ulp as ::core::ffi::c_double
                            * ((if wr >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                wr as ::core::ffi::c_double
                            } else {
                                -(wr as ::core::ffi::c_double)
                            }) + (if wi >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                wi as ::core::ffi::c_double
                            } else {
                                -(wi as ::core::ffi::c_double)
                            }))) as doublereal;
                        smin = (if d__1 >= smlnum {
                            d__1 as ::core::ffi::c_double
                        } else {
                            smlnum as ::core::ffi::c_double
                        }) as doublereal;
                        if ip == 0 as ::core::ffi::c_long {
                            *work.offset((ki + *n) as isize) = 1.0f64 as doublereal;
                            i__2 = *n;
                            k = (ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                            while k <= i__2 {
                                *work.offset((k + *n) as isize) =
                                    -*t.offset((ki + k * t_dim1) as isize);
                                k += 1;
                            }
                            vmax = 1.0f64 as doublereal;
                            vcrit = bignum;
                            jnxt =
                                (ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                            i__2 = *n;
                            j = (ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                            while j <= i__2 {
                                if !(j < jnxt) {
                                    j1 = j;
                                    j2 = j;
                                    jnxt = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as integer;
                                    if j < *n {
                                        if *t.offset((j + 1 as integer + j * t_dim1) as isize)
                                            != 0.0f64
                                        {
                                            j2 = (j as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            jnxt = (j as ::core::ffi::c_long
                                                + 2 as ::core::ffi::c_long)
                                                as integer;
                                        }
                                    }
                                    if j1 == j2 {
                                        if *work.offset(j as isize) > vcrit {
                                            rec = 1.0f64 / vmax;
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut rec,
                                                work.offset((ki + *n) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            vmax = 1.0f64 as doublereal;
                                            vcrit = bignum;
                                        }
                                        i__3 = (j as ::core::ffi::c_long
                                            - ki as ::core::ffi::c_long
                                            - 1 as ::core::ffi::c_long)
                                            as integer;
                                        let ref mut fresh1 = *work.offset((j + *n) as isize);
                                        *fresh1 -= f2c_ddot_0(
                                            &raw mut i__3,
                                            t.offset((ki + 1 as integer + j * t_dim1) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((ki + 1 as integer + *n) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        )
                                            as ::core::ffi::c_double;
                                        dlaln2__0(
                                            &raw mut c_false,
                                            &raw mut c__1,
                                            &raw mut c__1,
                                            &raw mut smin,
                                            &raw mut c_b22,
                                            t.offset((j + j * t_dim1) as isize) as *mut doublereal,
                                            ldt,
                                            &raw mut c_b22,
                                            &raw mut c_b22,
                                            work.offset((j + *n) as isize) as *mut doublereal,
                                            n,
                                            &raw mut wr,
                                            &raw mut c_b25,
                                            &raw mut x as *mut doublereal,
                                            &raw mut c__2,
                                            &raw mut scale,
                                            &raw mut xnorm,
                                            &raw mut ierr,
                                        );
                                        if scale != 1.0f64 {
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut scale,
                                                work.offset((ki + *n) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                        }
                                        *work.offset((j + *n) as isize) =
                                            x[0 as ::core::ffi::c_int as usize];
                                        d__1 = *work.offset((j + *n) as isize);
                                        d__2 = (if d__1
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            -(d__1 as ::core::ffi::c_double)
                                        })
                                            as doublereal;
                                        vmax = (if d__2 >= vmax {
                                            d__2 as ::core::ffi::c_double
                                        } else {
                                            vmax as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        vcrit = bignum / vmax;
                                    } else {
                                        d__1 = *work.offset(j as isize);
                                        d__2 = *work.offset(
                                            (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                        beta = (if d__1 >= d__2 {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            d__2 as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        if beta > vcrit {
                                            rec = 1.0f64 / vmax;
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut rec,
                                                work.offset((ki + *n) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            vmax = 1.0f64 as doublereal;
                                            vcrit = bignum;
                                        }
                                        i__3 = (j as ::core::ffi::c_long
                                            - ki as ::core::ffi::c_long
                                            - 1 as ::core::ffi::c_long)
                                            as integer;
                                        let ref mut fresh2 = *work.offset((j + *n) as isize);
                                        *fresh2 -= f2c_ddot_0(
                                            &raw mut i__3,
                                            t.offset((ki + 1 as integer + j * t_dim1) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((ki + 1 as integer + *n) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        )
                                            as ::core::ffi::c_double;
                                        i__3 = (j as ::core::ffi::c_long
                                            - ki as ::core::ffi::c_long
                                            - 1 as ::core::ffi::c_long)
                                            as integer;
                                        let ref mut fresh3 =
                                            *work.offset((j + 1 as integer + *n) as isize);
                                        *fresh3 -= f2c_ddot_0(
                                            &raw mut i__3,
                                            t.offset(
                                                (ki + 1 as integer + (j + 1 as integer) * t_dim1)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((ki + 1 as integer + *n) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        )
                                            as ::core::ffi::c_double;
                                        dlaln2__0(
                                            &raw mut c_true,
                                            &raw mut c__2,
                                            &raw mut c__1,
                                            &raw mut smin,
                                            &raw mut c_b22,
                                            t.offset((j + j * t_dim1) as isize) as *mut doublereal,
                                            ldt,
                                            &raw mut c_b22,
                                            &raw mut c_b22,
                                            work.offset((j + *n) as isize) as *mut doublereal,
                                            n,
                                            &raw mut wr,
                                            &raw mut c_b25,
                                            &raw mut x as *mut doublereal,
                                            &raw mut c__2,
                                            &raw mut scale,
                                            &raw mut xnorm,
                                            &raw mut ierr,
                                        );
                                        if scale != 1.0f64 {
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut scale,
                                                work.offset((ki + *n) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                        }
                                        *work.offset((j + *n) as isize) =
                                            x[0 as ::core::ffi::c_int as usize];
                                        *work.offset((j + 1 as integer + *n) as isize) =
                                            x[1 as ::core::ffi::c_int as usize];
                                        d__1 = *work.offset((j + *n) as isize);
                                        d__3 = (if d__1
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            -(d__1 as ::core::ffi::c_double)
                                        })
                                            as doublereal;
                                        d__2 = *work.offset((j + 1 as integer + *n) as isize);
                                        d__4 = (if d__2
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            d__2 as ::core::ffi::c_double
                                        } else {
                                            -(d__2 as ::core::ffi::c_double)
                                        })
                                            as doublereal;
                                        d__3 = (if d__3 >= d__4 {
                                            d__3 as ::core::ffi::c_double
                                        } else {
                                            d__4 as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        vmax = (if d__3 >= vmax {
                                            d__3 as ::core::ffi::c_double
                                        } else {
                                            vmax as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        vcrit = bignum / vmax;
                                    }
                                }
                                j += 1;
                            }
                            if over == 0 {
                                i__2 = (*n - ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                f2c_dcopy_0(
                                    &raw mut i__2,
                                    work.offset((ki + *n) as isize) as *mut doublereal,
                                    &raw mut c__1,
                                    vl.offset((ki + is * vl_dim1) as isize) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                i__2 = (*n - ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                ii = (f2c_idamax_0(
                                    &raw mut i__2,
                                    vl.offset((ki + is * vl_dim1) as isize) as *mut doublereal,
                                    &raw mut c__1,
                                ) as ::core::ffi::c_long
                                    + ki as ::core::ffi::c_long
                                    - 1 as ::core::ffi::c_long)
                                    as integer;
                                d__1 = *vl.offset((ii + is * vl_dim1) as isize);
                                remax = (1.0f64
                                    / (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    })) as doublereal;
                                i__2 = (*n - ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                f2c_dscal_0(
                                    &raw mut i__2,
                                    &raw mut remax,
                                    vl.offset((ki + is * vl_dim1) as isize) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                i__2 = (ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                    as integer;
                                k = 1 as integer;
                                while k <= i__2 {
                                    *vl.offset((k + is * vl_dim1) as isize) = 0.0f64 as doublereal;
                                    k += 1;
                                }
                            } else {
                                if ki < *n {
                                    i__2 = *n - ki;
                                    f2c_dgemv_0(
                                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        n,
                                        &raw mut i__2,
                                        &raw mut c_b22,
                                        vl.offset(
                                            ((ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                * vl_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        ldvl,
                                        work.offset((ki + 1 as integer + *n) as isize)
                                            as *mut doublereal,
                                        &raw mut c__1,
                                        work.offset((ki + *n) as isize) as *mut doublereal,
                                        vl.offset(
                                            (ki as ::core::ffi::c_long
                                                * vl_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                }
                                ii = f2c_idamax_0(
                                    n,
                                    vl.offset(
                                        (ki as ::core::ffi::c_long * vl_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                d__1 = *vl.offset((ii + ki * vl_dim1) as isize);
                                remax = (1.0f64
                                    / (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    })) as doublereal;
                                f2c_dscal_0(
                                    n,
                                    &raw mut remax,
                                    vl.offset(
                                        (ki as ::core::ffi::c_long * vl_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                            }
                        } else {
                            d__1 = *t.offset((ki + (ki + 1 as integer) * t_dim1) as isize);
                            d__2 = *t.offset((ki + 1 as integer + ki * t_dim1) as isize);
                            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) >= (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__2 as ::core::ffi::c_double
                            } else {
                                -(d__2 as ::core::ffi::c_double)
                            }) {
                                *work.offset((ki + *n) as isize) =
                                    wi / *t.offset((ki + (ki + 1 as integer) * t_dim1) as isize);
                                *work.offset((ki + 1 as integer + n2) as isize) =
                                    1.0f64 as doublereal;
                            } else {
                                *work.offset((ki + *n) as isize) = 1.0f64 as doublereal;
                                *work.offset((ki + 1 as integer + n2) as isize) =
                                    -wi / *t.offset((ki + 1 as integer + ki * t_dim1) as isize);
                            }
                            *work.offset((ki + 1 as integer + *n) as isize) = 0.0f64 as doublereal;
                            *work.offset((ki + n2) as isize) = 0.0f64 as doublereal;
                            i__2 = *n;
                            k = (ki as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
                            while k <= i__2 {
                                *work.offset((k + *n) as isize) = -*work.offset((ki + *n) as isize)
                                    * *t.offset((ki + k * t_dim1) as isize);
                                *work.offset((k + n2) as isize) = -*work
                                    .offset((ki + 1 as integer + n2) as isize)
                                    * *t.offset((ki + 1 as integer + k * t_dim1) as isize);
                                k += 1;
                            }
                            vmax = 1.0f64 as doublereal;
                            vcrit = bignum;
                            jnxt =
                                (ki as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
                            i__2 = *n;
                            j = (ki as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
                            while j <= i__2 {
                                if !(j < jnxt) {
                                    j1 = j;
                                    j2 = j;
                                    jnxt = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as integer;
                                    if j < *n {
                                        if *t.offset((j + 1 as integer + j * t_dim1) as isize)
                                            != 0.0f64
                                        {
                                            j2 = (j as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            jnxt = (j as ::core::ffi::c_long
                                                + 2 as ::core::ffi::c_long)
                                                as integer;
                                        }
                                    }
                                    if j1 == j2 {
                                        if *work.offset(j as isize) > vcrit {
                                            rec = 1.0f64 / vmax;
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut rec,
                                                work.offset((ki + *n) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut rec,
                                                work.offset((ki + n2) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            vmax = 1.0f64 as doublereal;
                                            vcrit = bignum;
                                        }
                                        i__3 = (j as ::core::ffi::c_long
                                            - ki as ::core::ffi::c_long
                                            - 2 as ::core::ffi::c_long)
                                            as integer;
                                        let ref mut fresh4 = *work.offset((j + *n) as isize);
                                        *fresh4 -= f2c_ddot_0(
                                            &raw mut i__3,
                                            t.offset((ki + 2 as integer + j * t_dim1) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((ki + 2 as integer + *n) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        )
                                            as ::core::ffi::c_double;
                                        i__3 = (j as ::core::ffi::c_long
                                            - ki as ::core::ffi::c_long
                                            - 2 as ::core::ffi::c_long)
                                            as integer;
                                        let ref mut fresh5 = *work.offset((j + n2) as isize);
                                        *fresh5 -= f2c_ddot_0(
                                            &raw mut i__3,
                                            t.offset((ki + 2 as integer + j * t_dim1) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((ki + 2 as integer + n2) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        )
                                            as ::core::ffi::c_double;
                                        d__1 = -wi;
                                        dlaln2__0(
                                            &raw mut c_false,
                                            &raw mut c__1,
                                            &raw mut c__2,
                                            &raw mut smin,
                                            &raw mut c_b22,
                                            t.offset((j + j * t_dim1) as isize) as *mut doublereal,
                                            ldt,
                                            &raw mut c_b22,
                                            &raw mut c_b22,
                                            work.offset((j + *n) as isize) as *mut doublereal,
                                            n,
                                            &raw mut wr,
                                            &raw mut d__1,
                                            &raw mut x as *mut doublereal,
                                            &raw mut c__2,
                                            &raw mut scale,
                                            &raw mut xnorm,
                                            &raw mut ierr,
                                        );
                                        if scale != 1.0f64 {
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut scale,
                                                work.offset((ki + *n) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut scale,
                                                work.offset((ki + n2) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                        }
                                        *work.offset((j + *n) as isize) =
                                            x[0 as ::core::ffi::c_int as usize];
                                        *work.offset((j + n2) as isize) =
                                            x[2 as ::core::ffi::c_int as usize];
                                        d__1 = *work.offset((j + *n) as isize);
                                        d__3 = (if d__1
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            -(d__1 as ::core::ffi::c_double)
                                        })
                                            as doublereal;
                                        d__2 = *work.offset((j + n2) as isize);
                                        d__4 = (if d__2
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            d__2 as ::core::ffi::c_double
                                        } else {
                                            -(d__2 as ::core::ffi::c_double)
                                        })
                                            as doublereal;
                                        d__3 = (if d__3 >= d__4 {
                                            d__3 as ::core::ffi::c_double
                                        } else {
                                            d__4 as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        vmax = (if d__3 >= vmax {
                                            d__3 as ::core::ffi::c_double
                                        } else {
                                            vmax as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        vcrit = bignum / vmax;
                                    } else {
                                        d__1 = *work.offset(j as isize);
                                        d__2 = *work.offset(
                                            (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                as isize,
                                        );
                                        beta = (if d__1 >= d__2 {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            d__2 as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        if beta > vcrit {
                                            rec = 1.0f64 / vmax;
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut rec,
                                                work.offset((ki + *n) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut rec,
                                                work.offset((ki + n2) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            vmax = 1.0f64 as doublereal;
                                            vcrit = bignum;
                                        }
                                        i__3 = (j as ::core::ffi::c_long
                                            - ki as ::core::ffi::c_long
                                            - 2 as ::core::ffi::c_long)
                                            as integer;
                                        let ref mut fresh6 = *work.offset((j + *n) as isize);
                                        *fresh6 -= f2c_ddot_0(
                                            &raw mut i__3,
                                            t.offset((ki + 2 as integer + j * t_dim1) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((ki + 2 as integer + *n) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        )
                                            as ::core::ffi::c_double;
                                        i__3 = (j as ::core::ffi::c_long
                                            - ki as ::core::ffi::c_long
                                            - 2 as ::core::ffi::c_long)
                                            as integer;
                                        let ref mut fresh7 = *work.offset((j + n2) as isize);
                                        *fresh7 -= f2c_ddot_0(
                                            &raw mut i__3,
                                            t.offset((ki + 2 as integer + j * t_dim1) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((ki + 2 as integer + n2) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        )
                                            as ::core::ffi::c_double;
                                        i__3 = (j as ::core::ffi::c_long
                                            - ki as ::core::ffi::c_long
                                            - 2 as ::core::ffi::c_long)
                                            as integer;
                                        let ref mut fresh8 =
                                            *work.offset((j + 1 as integer + *n) as isize);
                                        *fresh8 -= f2c_ddot_0(
                                            &raw mut i__3,
                                            t.offset(
                                                (ki + 2 as integer + (j + 1 as integer) * t_dim1)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((ki + 2 as integer + *n) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        )
                                            as ::core::ffi::c_double;
                                        i__3 = (j as ::core::ffi::c_long
                                            - ki as ::core::ffi::c_long
                                            - 2 as ::core::ffi::c_long)
                                            as integer;
                                        let ref mut fresh9 =
                                            *work.offset((j + 1 as integer + n2) as isize);
                                        *fresh9 -= f2c_ddot_0(
                                            &raw mut i__3,
                                            t.offset(
                                                (ki + 2 as integer + (j + 1 as integer) * t_dim1)
                                                    as isize,
                                            )
                                                as *mut doublereal,
                                            &raw mut c__1,
                                            work.offset((ki + 2 as integer + n2) as isize)
                                                as *mut doublereal,
                                            &raw mut c__1,
                                        )
                                            as ::core::ffi::c_double;
                                        d__1 = -wi;
                                        dlaln2__0(
                                            &raw mut c_true,
                                            &raw mut c__2,
                                            &raw mut c__2,
                                            &raw mut smin,
                                            &raw mut c_b22,
                                            t.offset((j + j * t_dim1) as isize) as *mut doublereal,
                                            ldt,
                                            &raw mut c_b22,
                                            &raw mut c_b22,
                                            work.offset((j + *n) as isize) as *mut doublereal,
                                            n,
                                            &raw mut wr,
                                            &raw mut d__1,
                                            &raw mut x as *mut doublereal,
                                            &raw mut c__2,
                                            &raw mut scale,
                                            &raw mut xnorm,
                                            &raw mut ierr,
                                        );
                                        if scale != 1.0f64 {
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut scale,
                                                work.offset((ki + *n) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                            i__3 = (*n - ki as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as integer;
                                            f2c_dscal_0(
                                                &raw mut i__3,
                                                &raw mut scale,
                                                work.offset((ki + n2) as isize) as *mut doublereal,
                                                &raw mut c__1,
                                            );
                                        }
                                        *work.offset((j + *n) as isize) =
                                            x[0 as ::core::ffi::c_int as usize];
                                        *work.offset((j + n2) as isize) =
                                            x[2 as ::core::ffi::c_int as usize];
                                        *work.offset((j + 1 as integer + *n) as isize) =
                                            x[1 as ::core::ffi::c_int as usize];
                                        *work.offset((j + 1 as integer + n2) as isize) =
                                            x[3 as ::core::ffi::c_int as usize];
                                        d__1 = (if x[0 as ::core::ffi::c_int as usize]
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            x[0 as ::core::ffi::c_int as usize]
                                        } else {
                                            -x[0 as ::core::ffi::c_int as usize]
                                        })
                                            as doublereal;
                                        d__2 = (if x[2 as ::core::ffi::c_int as usize]
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            x[2 as ::core::ffi::c_int as usize]
                                        } else {
                                            -x[2 as ::core::ffi::c_int as usize]
                                        })
                                            as doublereal;
                                        d__1 = (if d__1 >= d__2 {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            d__2 as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        d__2 = (if x[1 as ::core::ffi::c_int as usize]
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            x[1 as ::core::ffi::c_int as usize]
                                        } else {
                                            -x[1 as ::core::ffi::c_int as usize]
                                        })
                                            as doublereal;
                                        d__1 = (if d__1 >= d__2 {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            d__2 as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        d__2 = (if x[3 as ::core::ffi::c_int as usize]
                                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                        {
                                            x[3 as ::core::ffi::c_int as usize]
                                        } else {
                                            -x[3 as ::core::ffi::c_int as usize]
                                        })
                                            as doublereal;
                                        d__1 = (if d__1 >= d__2 {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            d__2 as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        vmax = (if d__1 >= vmax {
                                            d__1 as ::core::ffi::c_double
                                        } else {
                                            vmax as ::core::ffi::c_double
                                        })
                                            as doublereal;
                                        vcrit = bignum / vmax;
                                    }
                                }
                                j += 1;
                            }
                            if over == 0 {
                                i__2 = (*n - ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                f2c_dcopy_0(
                                    &raw mut i__2,
                                    work.offset((ki + *n) as isize) as *mut doublereal,
                                    &raw mut c__1,
                                    vl.offset((ki + is * vl_dim1) as isize) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                i__2 = (*n - ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                f2c_dcopy_0(
                                    &raw mut i__2,
                                    work.offset((ki + n2) as isize) as *mut doublereal,
                                    &raw mut c__1,
                                    vl.offset((ki + (is + 1 as integer) * vl_dim1) as isize)
                                        as *mut doublereal,
                                    &raw mut c__1,
                                );
                                emax = 0.0f64 as doublereal;
                                i__2 = *n;
                                k = ki;
                                while k <= i__2 {
                                    d__3 = emax;
                                    d__1 = *vl.offset((k + is * vl_dim1) as isize);
                                    d__2 = *vl.offset((k + (is + 1 as integer) * vl_dim1) as isize);
                                    d__4 = ((if d__1
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    }) + (if d__2
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__2 as ::core::ffi::c_double
                                    } else {
                                        -(d__2 as ::core::ffi::c_double)
                                    })) as doublereal;
                                    emax = (if d__3 >= d__4 {
                                        d__3 as ::core::ffi::c_double
                                    } else {
                                        d__4 as ::core::ffi::c_double
                                    }) as doublereal;
                                    k += 1;
                                }
                                remax = 1.0f64 / emax;
                                i__2 = (*n - ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                f2c_dscal_0(
                                    &raw mut i__2,
                                    &raw mut remax,
                                    vl.offset((ki + is * vl_dim1) as isize) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                i__2 = (*n - ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    as integer;
                                f2c_dscal_0(
                                    &raw mut i__2,
                                    &raw mut remax,
                                    vl.offset((ki + (is + 1 as integer) * vl_dim1) as isize)
                                        as *mut doublereal,
                                    &raw mut c__1,
                                );
                                i__2 = (ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                    as integer;
                                k = 1 as integer;
                                while k <= i__2 {
                                    *vl.offset((k + is * vl_dim1) as isize) = 0.0f64 as doublereal;
                                    *vl.offset((k + (is + 1 as integer) * vl_dim1) as isize) =
                                        0.0f64 as doublereal;
                                    k += 1;
                                }
                            } else {
                                if ki < *n - 1 as ::core::ffi::c_long {
                                    i__2 =
                                        (*n - ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as integer;
                                    f2c_dgemv_0(
                                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        n,
                                        &raw mut i__2,
                                        &raw mut c_b22,
                                        vl.offset(
                                            ((ki as ::core::ffi::c_long + 2 as ::core::ffi::c_long)
                                                * vl_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        ldvl,
                                        work.offset((ki + 2 as integer + *n) as isize)
                                            as *mut doublereal,
                                        &raw mut c__1,
                                        work.offset((ki + *n) as isize) as *mut doublereal,
                                        vl.offset(
                                            (ki as ::core::ffi::c_long
                                                * vl_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                    i__2 =
                                        (*n - ki as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                            as integer;
                                    f2c_dgemv_0(
                                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        n,
                                        &raw mut i__2,
                                        &raw mut c_b22,
                                        vl.offset(
                                            ((ki as ::core::ffi::c_long + 2 as ::core::ffi::c_long)
                                                * vl_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        ldvl,
                                        work.offset((ki + 2 as integer + n2) as isize)
                                            as *mut doublereal,
                                        &raw mut c__1,
                                        work.offset((ki + 1 as integer + n2) as isize)
                                            as *mut doublereal,
                                        vl.offset(
                                            ((ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                * vl_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                } else {
                                    f2c_dscal_0(
                                        n,
                                        work.offset((ki + *n) as isize) as *mut doublereal,
                                        vl.offset(
                                            (ki as ::core::ffi::c_long
                                                * vl_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                    f2c_dscal_0(
                                        n,
                                        work.offset((ki + 1 as integer + n2) as isize)
                                            as *mut doublereal,
                                        vl.offset(
                                            ((ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                                * vl_dim1 as ::core::ffi::c_long
                                                + 1 as ::core::ffi::c_long)
                                                as isize,
                                        )
                                            as *mut doublereal,
                                        &raw mut c__1,
                                    );
                                }
                                emax = 0.0f64 as doublereal;
                                i__2 = *n;
                                k = 1 as integer;
                                while k <= i__2 {
                                    d__3 = emax;
                                    d__1 = *vl.offset((k + ki * vl_dim1) as isize);
                                    d__2 = *vl.offset((k + (ki + 1 as integer) * vl_dim1) as isize);
                                    d__4 = ((if d__1
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__1 as ::core::ffi::c_double
                                    } else {
                                        -(d__1 as ::core::ffi::c_double)
                                    }) + (if d__2
                                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        d__2 as ::core::ffi::c_double
                                    } else {
                                        -(d__2 as ::core::ffi::c_double)
                                    })) as doublereal;
                                    emax = (if d__3 >= d__4 {
                                        d__3 as ::core::ffi::c_double
                                    } else {
                                        d__4 as ::core::ffi::c_double
                                    }) as doublereal;
                                    k += 1;
                                }
                                remax = 1.0f64 / emax;
                                f2c_dscal_0(
                                    n,
                                    &raw mut remax,
                                    vl.offset(
                                        (ki as ::core::ffi::c_long * vl_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                                f2c_dscal_0(
                                    n,
                                    &raw mut remax,
                                    vl.offset(
                                        ((ki as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            * vl_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                );
                            }
                        }
                        is += 1;
                        if ip != 0 as ::core::ffi::c_long {
                            is += 1;
                        }
                    }
                }
            }
            if ip == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
                ip = 0 as integer;
            }
            if ip == 1 as ::core::ffi::c_long {
                ip = -(1 as ::core::ffi::c_int) as integer;
            }
            ki += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
