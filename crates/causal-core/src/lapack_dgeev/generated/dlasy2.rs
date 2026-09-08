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
static mut c__4: integer = 4 as integer;
static mut c__1: integer = 1 as integer;
static mut c__16: integer = 16 as integer;
static mut c__0: integer = 0 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlasy2_(
    mut ltranl: *mut logical,
    mut ltranr: *mut logical,
    mut isgn: *mut integer,
    mut n1: *mut integer,
    mut n2: *mut integer,
    mut tl: *mut doublereal,
    mut ldtl: *mut integer,
    mut tr: *mut doublereal,
    mut ldtr: *mut integer,
    mut b: *mut doublereal,
    mut ldb: *mut integer,
    mut scale: *mut doublereal,
    mut x: *mut doublereal,
    mut ldx: *mut integer,
    mut xnorm: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    static mut locu12: [integer; 4] = [
        3 as ::core::ffi::c_int as integer,
        4 as ::core::ffi::c_int as integer,
        1 as ::core::ffi::c_int as integer,
        2 as ::core::ffi::c_int as integer,
    ];
    static mut locl21: [integer; 4] = [
        2 as ::core::ffi::c_int as integer,
        1 as ::core::ffi::c_int as integer,
        4 as ::core::ffi::c_int as integer,
        3 as ::core::ffi::c_int as integer,
    ];
    static mut locu22: [integer; 4] = [
        4 as ::core::ffi::c_int as integer,
        3 as ::core::ffi::c_int as integer,
        2 as ::core::ffi::c_int as integer,
        1 as ::core::ffi::c_int as integer,
    ];
    static mut xswpiv: [logical; 4] = [
        FALSE_ as logical,
        FALSE_ as logical,
        TRUE_ as logical,
        TRUE_ as logical,
    ];
    static mut bswpiv: [logical; 4] = [
        FALSE_ as logical,
        TRUE_ as logical,
        FALSE_ as logical,
        TRUE_ as logical,
    ];
    let mut b_dim1: integer = 0;
    let mut b_offset: integer = 0;
    let mut tl_dim1: integer = 0;
    let mut tl_offset: integer = 0;
    let mut tr_dim1: integer = 0;
    let mut tr_offset: integer = 0;
    let mut x_dim1: integer = 0;
    let mut x_offset: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__4: doublereal = 0.;
    let mut d__5: doublereal = 0.;
    let mut d__6: doublereal = 0.;
    let mut d__7: doublereal = 0.;
    let mut d__8: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut x2: [doublereal; 2] = [0.; 2];
    let mut l21: doublereal = 0.;
    let mut u11: doublereal = 0.;
    let mut u12: doublereal = 0.;
    let mut ip: integer = 0;
    let mut jp: integer = 0;
    let mut u22: doublereal = 0.;
    let mut t16: [doublereal; 16] = [0.; 16];
    let mut gam: doublereal = 0.;
    let mut bet: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut sgn: doublereal = 0.;
    let mut tmp: [doublereal; 4] = [0.; 4];
    let mut tau1: doublereal = 0.;
    let mut btmp: [doublereal; 4] = [0.; 4];
    let mut smin: doublereal = 0.;
    let mut ipiv: integer = 0;
    let mut temp: doublereal = 0.;
    let mut jpiv: [integer; 4] = [0; 4];
    let mut xmax: doublereal = 0.;
    let mut ipsv: integer = 0;
    let mut jpsv: integer = 0;
    let mut bswap: logical = 0;
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
        #[link_name = "dgelsd_closure_f2c_dswap"]
        fn f2c_dswap_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut xswap: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_idamax"]
        fn f2c_idamax_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> integer;
    }
    let mut smlnum: doublereal = 0.;
    tl_dim1 = *ldtl;
    tl_offset = 1 as integer + tl_dim1;
    tl = tl.offset(-(tl_offset as isize));
    tr_dim1 = *ldtr;
    tr_offset = 1 as integer + tr_dim1;
    tr = tr.offset(-(tr_offset as isize));
    b_dim1 = *ldb;
    b_offset = 1 as integer + b_dim1;
    b = b.offset(-(b_offset as isize));
    x_dim1 = *ldx;
    x_offset = 1 as integer + x_dim1;
    x = x.offset(-(x_offset as isize));
    *info = 0 as integer;
    if *n1 == 0 as ::core::ffi::c_long || *n2 == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    eps = dlamch__0(b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char);
    smlnum =
        dlamch__0(b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char)
            / eps;
    sgn = *isgn as doublereal;
    k = (*n1 + *n1 + *n2 - 2 as ::core::ffi::c_long) as integer;
    match k {
        2 => {
            d__1 = *tl.offset((tl_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__7 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = *tr.offset((tr_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__8 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            }) as doublereal;
            d__7 = (if d__7 >= d__8 {
                d__7 as ::core::ffi::c_double
            } else {
                d__8 as ::core::ffi::c_double
            }) as doublereal;
            d__3 = *tr.offset(
                (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            );
            d__8 = (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__3 as ::core::ffi::c_double
            } else {
                -(d__3 as ::core::ffi::c_double)
            }) as doublereal;
            d__7 = (if d__7 >= d__8 {
                d__7 as ::core::ffi::c_double
            } else {
                d__8 as ::core::ffi::c_double
            }) as doublereal;
            d__4 = *tr.offset((tr_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            d__8 = (if d__4 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__4 as ::core::ffi::c_double
            } else {
                -(d__4 as ::core::ffi::c_double)
            }) as doublereal;
            d__7 = (if d__7 >= d__8 {
                d__7 as ::core::ffi::c_double
            } else {
                d__8 as ::core::ffi::c_double
            }) as doublereal;
            d__5 = *tr.offset(
                (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            );
            d__8 = (if d__5 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__5 as ::core::ffi::c_double
            } else {
                -(d__5 as ::core::ffi::c_double)
            }) as doublereal;
            d__6 = (eps as ::core::ffi::c_double
                * (if d__7 >= d__8 {
                    d__7 as ::core::ffi::c_double
                } else {
                    d__8 as ::core::ffi::c_double
                })) as doublereal;
            smin = (if d__6 >= smlnum {
                d__6 as ::core::ffi::c_double
            } else {
                smlnum as ::core::ffi::c_double
            }) as doublereal;
            tmp[0 as ::core::ffi::c_int as usize] = *tl
                .offset((tl_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                + sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                    );
            tmp[3 as ::core::ffi::c_int as usize] = *tl
                .offset((tl_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                + sgn
                    * *tr.offset(
                        (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_long) as isize,
                    );
            if *ltranr != 0 {
                tmp[1 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    );
                tmp[2 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    );
            } else {
                tmp[1 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                tmp[2 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    );
            }
            btmp[0 as ::core::ffi::c_int as usize] =
                *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            btmp[1 as ::core::ffi::c_int as usize] = *b.offset(
                (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            );
        }
        3 => {
            d__1 = *tr.offset((tr_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__7 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = *tl.offset((tl_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__8 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            }) as doublereal;
            d__7 = (if d__7 >= d__8 {
                d__7 as ::core::ffi::c_double
            } else {
                d__8 as ::core::ffi::c_double
            }) as doublereal;
            d__3 = *tl.offset(
                (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            );
            d__8 = (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__3 as ::core::ffi::c_double
            } else {
                -(d__3 as ::core::ffi::c_double)
            }) as doublereal;
            d__7 = (if d__7 >= d__8 {
                d__7 as ::core::ffi::c_double
            } else {
                d__8 as ::core::ffi::c_double
            }) as doublereal;
            d__4 = *tl.offset((tl_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            d__8 = (if d__4 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__4 as ::core::ffi::c_double
            } else {
                -(d__4 as ::core::ffi::c_double)
            }) as doublereal;
            d__7 = (if d__7 >= d__8 {
                d__7 as ::core::ffi::c_double
            } else {
                d__8 as ::core::ffi::c_double
            }) as doublereal;
            d__5 = *tl.offset(
                (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            );
            d__8 = (if d__5 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__5 as ::core::ffi::c_double
            } else {
                -(d__5 as ::core::ffi::c_double)
            }) as doublereal;
            d__6 = (eps as ::core::ffi::c_double
                * (if d__7 >= d__8 {
                    d__7 as ::core::ffi::c_double
                } else {
                    d__8 as ::core::ffi::c_double
                })) as doublereal;
            smin = (if d__6 >= smlnum {
                d__6 as ::core::ffi::c_double
            } else {
                smlnum as ::core::ffi::c_double
            }) as doublereal;
            tmp[0 as ::core::ffi::c_int as usize] = *tl
                .offset((tl_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                + sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                    );
            tmp[3 as ::core::ffi::c_int as usize] = *tl.offset(
                (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            ) + sgn
                * *tr.offset((tr_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            if *ltranl != 0 {
                tmp[1 as ::core::ffi::c_int as usize] = *tl.offset(
                    (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
                tmp[2 as ::core::ffi::c_int as usize] = *tl
                    .offset((tl_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            } else {
                tmp[1 as ::core::ffi::c_int as usize] = *tl
                    .offset((tl_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                tmp[2 as ::core::ffi::c_int as usize] = *tl.offset(
                    (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
            }
            btmp[0 as ::core::ffi::c_int as usize] =
                *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            btmp[1 as ::core::ffi::c_int as usize] =
                *b.offset((b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
        }
        4 => {
            d__1 = *tr.offset((tr_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__5 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = *tr.offset(
                (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            );
            d__6 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            }) as doublereal;
            d__5 = (if d__5 >= d__6 {
                d__5 as ::core::ffi::c_double
            } else {
                d__6 as ::core::ffi::c_double
            }) as doublereal;
            d__3 = *tr.offset((tr_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            d__6 = (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__3 as ::core::ffi::c_double
            } else {
                -(d__3 as ::core::ffi::c_double)
            }) as doublereal;
            d__5 = (if d__5 >= d__6 {
                d__5 as ::core::ffi::c_double
            } else {
                d__6 as ::core::ffi::c_double
            }) as doublereal;
            d__4 = *tr.offset(
                (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            );
            d__6 = (if d__4 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__4 as ::core::ffi::c_double
            } else {
                -(d__4 as ::core::ffi::c_double)
            }) as doublereal;
            smin = (if d__5 >= d__6 {
                d__5 as ::core::ffi::c_double
            } else {
                d__6 as ::core::ffi::c_double
            }) as doublereal;
            d__5 = smin;
            d__1 = *tl.offset((tl_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__6 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            d__5 = (if d__5 >= d__6 {
                d__5 as ::core::ffi::c_double
            } else {
                d__6 as ::core::ffi::c_double
            }) as doublereal;
            d__2 = *tl.offset(
                (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            );
            d__6 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            }) as doublereal;
            d__5 = (if d__5 >= d__6 {
                d__5 as ::core::ffi::c_double
            } else {
                d__6 as ::core::ffi::c_double
            }) as doublereal;
            d__3 = *tl.offset((tl_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            d__6 = (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__3 as ::core::ffi::c_double
            } else {
                -(d__3 as ::core::ffi::c_double)
            }) as doublereal;
            d__5 = (if d__5 >= d__6 {
                d__5 as ::core::ffi::c_double
            } else {
                d__6 as ::core::ffi::c_double
            }) as doublereal;
            d__4 = *tl.offset(
                (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            );
            d__6 = (if d__4 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__4 as ::core::ffi::c_double
            } else {
                -(d__4 as ::core::ffi::c_double)
            }) as doublereal;
            smin = (if d__5 >= d__6 {
                d__5 as ::core::ffi::c_double
            } else {
                d__6 as ::core::ffi::c_double
            }) as doublereal;
            d__1 = eps * smin;
            smin = (if d__1 >= smlnum {
                d__1 as ::core::ffi::c_double
            } else {
                smlnum as ::core::ffi::c_double
            }) as doublereal;
            btmp[0 as ::core::ffi::c_int as usize] = 0.0f64 as doublereal;
            f2c_dcopy_0(
                &raw mut c__16,
                &raw mut btmp as *mut doublereal,
                &raw mut c__0,
                &raw mut t16 as *mut doublereal,
                &raw mut c__1,
            );
            t16[0 as ::core::ffi::c_int as usize] = *tl
                .offset((tl_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                + sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                    );
            t16[5 as ::core::ffi::c_int as usize] = *tl.offset(
                (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            ) + sgn
                * *tr.offset((tr_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            t16[10 as ::core::ffi::c_int as usize] = *tl
                .offset((tl_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                + sgn
                    * *tr.offset(
                        (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_long) as isize,
                    );
            t16[15 as ::core::ffi::c_int as usize] = *tl.offset(
                (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            ) + sgn
                * *tr.offset(
                    (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                );
            if *ltranl != 0 {
                t16[4 as ::core::ffi::c_int as usize] = *tl
                    .offset((tl_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                t16[1 as ::core::ffi::c_int as usize] = *tl.offset(
                    (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
                t16[14 as ::core::ffi::c_int as usize] = *tl
                    .offset((tl_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                t16[11 as ::core::ffi::c_int as usize] = *tl.offset(
                    (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
            } else {
                t16[4 as ::core::ffi::c_int as usize] = *tl.offset(
                    (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
                t16[1 as ::core::ffi::c_int as usize] = *tl
                    .offset((tl_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
                t16[14 as ::core::ffi::c_int as usize] = *tl.offset(
                    (((tl_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                );
                t16[11 as ::core::ffi::c_int as usize] = *tl
                    .offset((tl_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            }
            if *ltranr != 0 {
                t16[8 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                t16[13 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                t16[2 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    );
                t16[7 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    );
            } else {
                t16[8 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    );
                t16[13 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    );
                t16[2 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                t16[7 as ::core::ffi::c_int as usize] = sgn
                    * *tr.offset(
                        (((tr_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_long) as isize,
                    );
            }
            btmp[0 as ::core::ffi::c_int as usize] =
                *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            btmp[1 as ::core::ffi::c_int as usize] =
                *b.offset((b_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            btmp[2 as ::core::ffi::c_int as usize] = *b.offset(
                (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            );
            btmp[3 as ::core::ffi::c_int as usize] = *b.offset(
                (((b_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            );
            i__ = 1 as integer;
            while i__ <= 3 as ::core::ffi::c_long {
                xmax = 0.0f64 as doublereal;
                ip = i__;
                while ip <= 4 as ::core::ffi::c_long {
                    jp = i__;
                    while jp <= 4 as ::core::ffi::c_long {
                        d__1 = t16[(ip as ::core::ffi::c_long
                            + ((jp as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                            - 5 as ::core::ffi::c_long)
                            as usize];
                        if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) >= xmax
                        {
                            d__1 = t16[(ip as ::core::ffi::c_long
                                + ((jp as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                                - 5 as ::core::ffi::c_long)
                                as usize];
                            xmax = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) as doublereal;
                            ipsv = ip;
                            jpsv = jp;
                        }
                        jp += 1;
                    }
                    ip += 1;
                }
                if ipsv != i__ {
                    f2c_dswap_0(
                        &raw mut c__4,
                        (&raw mut t16 as *mut doublereal).offset(
                            (ipsv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__4,
                        (&raw mut t16 as *mut doublereal).offset(
                            (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__4,
                    );
                    temp = btmp[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
                    btmp[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] =
                        btmp[(ipsv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
                    btmp[(ipsv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] = temp;
                }
                if jpsv != i__ {
                    f2c_dswap_0(
                        &raw mut c__4,
                        (&raw mut t16 as *mut doublereal).offset(
                            (((jpsv as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                                - 4 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        (&raw mut t16 as *mut doublereal).offset(
                            (((i__ as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                                - 4 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                }
                jpiv[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] = jpsv;
                d__1 = t16[(i__ as ::core::ffi::c_long
                    + ((i__ as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                    - 5 as ::core::ffi::c_long) as usize];
                if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) < smin
                {
                    *info = 1 as integer;
                    t16[(i__ as ::core::ffi::c_long
                        + ((i__ as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                        - 5 as ::core::ffi::c_long) as usize] = smin;
                }
                j = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                while j <= 4 as ::core::ffi::c_long {
                    t16[(j as ::core::ffi::c_long
                        + ((i__ as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                        - 5 as ::core::ffi::c_long) as usize] /= t16[(i__ as ::core::ffi::c_long
                        + ((i__ as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                        - 5 as ::core::ffi::c_long)
                        as usize]
                        as ::core::ffi::c_double;
                    btmp[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] -= (t16[(j
                        as ::core::ffi::c_long
                        + ((i__ as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                        - 5 as ::core::ffi::c_long)
                        as usize]
                        * btmp[(i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize])
                        as ::core::ffi::c_double;
                    k = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    while k <= 4 as ::core::ffi::c_long {
                        t16[(j as ::core::ffi::c_long
                            + ((k as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                            - 5 as ::core::ffi::c_long) as usize] -= (t16[(j as ::core::ffi::c_long
                            + ((i__ as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                            - 5 as ::core::ffi::c_long)
                            as usize]
                            * t16[(i__ as ::core::ffi::c_long
                                + ((k as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                                - 5 as ::core::ffi::c_long)
                                as usize])
                            as ::core::ffi::c_double;
                        k += 1;
                    }
                    j += 1;
                }
                i__ += 1;
            }
            if (if t16[15 as ::core::ffi::c_int as usize]
                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                t16[15 as ::core::ffi::c_int as usize]
            } else {
                -t16[15 as ::core::ffi::c_int as usize]
            }) < smin
            {
                t16[15 as ::core::ffi::c_int as usize] = smin;
            }
            *scale = 1.0f64 as doublereal;
            if smlnum as ::core::ffi::c_double
                * 8.0f64
                * (if btmp[0 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    btmp[0 as ::core::ffi::c_int as usize]
                } else {
                    -btmp[0 as ::core::ffi::c_int as usize]
                })
                > (if t16[0 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    t16[0 as ::core::ffi::c_int as usize]
                } else {
                    -t16[0 as ::core::ffi::c_int as usize]
                })
                || smlnum as ::core::ffi::c_double
                    * 8.0f64
                    * (if btmp[1 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        btmp[1 as ::core::ffi::c_int as usize]
                    } else {
                        -btmp[1 as ::core::ffi::c_int as usize]
                    })
                    > (if t16[5 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        t16[5 as ::core::ffi::c_int as usize]
                    } else {
                        -t16[5 as ::core::ffi::c_int as usize]
                    })
                || smlnum as ::core::ffi::c_double
                    * 8.0f64
                    * (if btmp[2 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        btmp[2 as ::core::ffi::c_int as usize]
                    } else {
                        -btmp[2 as ::core::ffi::c_int as usize]
                    })
                    > (if t16[10 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        t16[10 as ::core::ffi::c_int as usize]
                    } else {
                        -t16[10 as ::core::ffi::c_int as usize]
                    })
                || smlnum as ::core::ffi::c_double
                    * 8.0f64
                    * (if btmp[3 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        btmp[3 as ::core::ffi::c_int as usize]
                    } else {
                        -btmp[3 as ::core::ffi::c_int as usize]
                    })
                    > (if t16[15 as ::core::ffi::c_int as usize]
                        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        t16[15 as ::core::ffi::c_int as usize]
                    } else {
                        -t16[15 as ::core::ffi::c_int as usize]
                    })
            {
                d__1 = (if btmp[0 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    btmp[0 as ::core::ffi::c_int as usize]
                } else {
                    -btmp[0 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__2 = (if btmp[1 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    btmp[1 as ::core::ffi::c_int as usize]
                } else {
                    -btmp[1 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__1 = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__2 = (if btmp[2 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    btmp[2 as ::core::ffi::c_int as usize]
                } else {
                    -btmp[2 as ::core::ffi::c_int as usize]
                }) as doublereal;
                d__1 = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                d__2 = (if btmp[3 as ::core::ffi::c_int as usize]
                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    btmp[3 as ::core::ffi::c_int as usize]
                } else {
                    -btmp[3 as ::core::ffi::c_int as usize]
                }) as doublereal;
                *scale = (0.125f64
                    / (if d__1 >= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    })) as doublereal;
                btmp[0 as ::core::ffi::c_int as usize] *= *scale as ::core::ffi::c_double;
                btmp[1 as ::core::ffi::c_int as usize] *= *scale as ::core::ffi::c_double;
                btmp[2 as ::core::ffi::c_int as usize] *= *scale as ::core::ffi::c_double;
                btmp[3 as ::core::ffi::c_int as usize] *= *scale as ::core::ffi::c_double;
            }
            i__ = 1 as integer;
            while i__ <= 4 as ::core::ffi::c_long {
                k = 5 as integer - i__;
                temp = 1.0f64
                    / t16[(k as ::core::ffi::c_long
                        + ((k as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                        - 5 as ::core::ffi::c_long) as usize];
                tmp[(k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] =
                    btmp[(k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] * temp;
                j = (k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                while j <= 4 as ::core::ffi::c_long {
                    tmp[(k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] -= (temp
                        * t16[(k as ::core::ffi::c_long
                            + ((j as ::core::ffi::c_long) << 2 as ::core::ffi::c_int)
                            - 5 as ::core::ffi::c_long) as usize]
                        * tmp[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize])
                        as ::core::ffi::c_double;
                    j += 1;
                }
                i__ += 1;
            }
            i__ = 1 as integer;
            while i__ <= 3 as ::core::ffi::c_long {
                if jpiv[(4 as ::core::ffi::c_long
                    - i__ as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as usize]
                    != 4 as integer - i__
                {
                    temp = tmp[(4 as ::core::ffi::c_long
                        - i__ as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as usize];
                    tmp[(4 as ::core::ffi::c_long
                        - i__ as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as usize] =
                        tmp[(jpiv[(4 as ::core::ffi::c_long
                            - i__ as ::core::ffi::c_long
                            - 1 as ::core::ffi::c_long) as usize]
                            - 1 as ::core::ffi::c_long) as usize];
                    tmp[(jpiv[(4 as ::core::ffi::c_long
                        - i__ as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as usize]
                        - 1 as ::core::ffi::c_long) as usize] = temp;
                }
                i__ += 1;
            }
            *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                tmp[0 as ::core::ffi::c_int as usize];
            *x.offset((x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) =
                tmp[1 as ::core::ffi::c_int as usize];
            *x.offset(
                (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            ) = tmp[2 as ::core::ffi::c_int as usize];
            *x.offset(
                (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_long) as isize,
            ) = tmp[3 as ::core::ffi::c_int as usize];
            d__1 = ((if tmp[0 as ::core::ffi::c_int as usize]
                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                tmp[0 as ::core::ffi::c_int as usize]
            } else {
                -tmp[0 as ::core::ffi::c_int as usize]
            }) + (if tmp[2 as ::core::ffi::c_int as usize]
                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                tmp[2 as ::core::ffi::c_int as usize]
            } else {
                -tmp[2 as ::core::ffi::c_int as usize]
            })) as doublereal;
            d__2 = ((if tmp[1 as ::core::ffi::c_int as usize]
                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                tmp[1 as ::core::ffi::c_int as usize]
            } else {
                -tmp[1 as ::core::ffi::c_int as usize]
            }) + (if tmp[3 as ::core::ffi::c_int as usize]
                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                tmp[3 as ::core::ffi::c_int as usize]
            } else {
                -tmp[3 as ::core::ffi::c_int as usize]
            })) as doublereal;
            *xnorm = (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            }) as doublereal;
            return 0 as ::core::ffi::c_int;
        }
        1 | _ => {
            tau1 = *tl.offset((tl_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                + sgn
                    * *tr.offset(
                        (tr_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                    );
            bet = (if tau1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                tau1 as ::core::ffi::c_double
            } else {
                -(tau1 as ::core::ffi::c_double)
            }) as doublereal;
            if bet <= smlnum {
                tau1 = smlnum;
                bet = smlnum;
                *info = 1 as integer;
            }
            *scale = 1.0f64 as doublereal;
            d__1 = *b.offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            gam = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            if smlnum * gam > bet {
                *scale = 1.0f64 / gam;
            }
            *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) = *b
                .offset((b_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                * *scale
                / tau1;
            d__1 = *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            *xnorm = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            return 0 as ::core::ffi::c_int;
        }
    }
    ipiv = f2c_idamax_0(
        &raw mut c__4,
        &raw mut tmp as *mut doublereal,
        &raw mut c__1,
    );
    u11 = tmp[(ipiv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
    if (if u11 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        u11 as ::core::ffi::c_double
    } else {
        -(u11 as ::core::ffi::c_double)
    }) <= smin
    {
        *info = 1 as integer;
        u11 = smin;
    }
    u12 = tmp[(locu12[(ipiv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize]
        - 1 as ::core::ffi::c_long) as usize];
    l21 = tmp[(locl21[(ipiv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize]
        - 1 as ::core::ffi::c_long) as usize]
        / u11;
    u22 = tmp[(locu22[(ipiv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize]
        - 1 as ::core::ffi::c_long) as usize]
        - u12 * l21;
    xswap = xswpiv[(ipiv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
    bswap = bswpiv[(ipiv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
    if (if u22 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        u22 as ::core::ffi::c_double
    } else {
        -(u22 as ::core::ffi::c_double)
    }) <= smin
    {
        *info = 1 as integer;
        u22 = smin;
    }
    if bswap != 0 {
        temp = btmp[1 as ::core::ffi::c_int as usize];
        btmp[1 as ::core::ffi::c_int as usize] =
            btmp[0 as ::core::ffi::c_int as usize] - l21 * temp;
        btmp[0 as ::core::ffi::c_int as usize] = temp;
    } else {
        btmp[1 as ::core::ffi::c_int as usize] -=
            (l21 * btmp[0 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double;
    }
    *scale = 1.0f64 as doublereal;
    if smlnum as ::core::ffi::c_double
        * 2.0f64
        * (if btmp[1 as ::core::ffi::c_int as usize]
            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            btmp[1 as ::core::ffi::c_int as usize]
        } else {
            -btmp[1 as ::core::ffi::c_int as usize]
        })
        > (if u22 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            u22 as ::core::ffi::c_double
        } else {
            -(u22 as ::core::ffi::c_double)
        })
        || smlnum as ::core::ffi::c_double
            * 2.0f64
            * (if btmp[0 as ::core::ffi::c_int as usize]
                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                btmp[0 as ::core::ffi::c_int as usize]
            } else {
                -btmp[0 as ::core::ffi::c_int as usize]
            })
            > (if u11 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                u11 as ::core::ffi::c_double
            } else {
                -(u11 as ::core::ffi::c_double)
            })
    {
        d__1 = (if btmp[0 as ::core::ffi::c_int as usize]
            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            btmp[0 as ::core::ffi::c_int as usize]
        } else {
            -btmp[0 as ::core::ffi::c_int as usize]
        }) as doublereal;
        d__2 = (if btmp[1 as ::core::ffi::c_int as usize]
            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            btmp[1 as ::core::ffi::c_int as usize]
        } else {
            -btmp[1 as ::core::ffi::c_int as usize]
        }) as doublereal;
        *scale = (0.5f64
            / (if d__1 >= d__2 {
                d__1 as ::core::ffi::c_double
            } else {
                d__2 as ::core::ffi::c_double
            })) as doublereal;
        btmp[0 as ::core::ffi::c_int as usize] *= *scale as ::core::ffi::c_double;
        btmp[1 as ::core::ffi::c_int as usize] *= *scale as ::core::ffi::c_double;
    }
    x2[1 as ::core::ffi::c_int as usize] = btmp[1 as ::core::ffi::c_int as usize] / u22;
    x2[0 as ::core::ffi::c_int as usize] = btmp[0 as ::core::ffi::c_int as usize] / u11
        - u12 / u11 * x2[1 as ::core::ffi::c_int as usize];
    if xswap != 0 {
        temp = x2[1 as ::core::ffi::c_int as usize];
        x2[1 as ::core::ffi::c_int as usize] = x2[0 as ::core::ffi::c_int as usize];
        x2[0 as ::core::ffi::c_int as usize] = temp;
    }
    *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
        x2[0 as ::core::ffi::c_int as usize];
    if *n1 == 1 as ::core::ffi::c_long {
        *x.offset(
            (((x_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_long) as isize,
        ) = x2[1 as ::core::ffi::c_int as usize];
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
    } else {
        *x.offset((x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) =
            x2[1 as ::core::ffi::c_int as usize];
        d__1 = *x.offset((x_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
        d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        }) as doublereal;
        d__2 = *x.offset((x_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
        d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__2 as ::core::ffi::c_double
        } else {
            -(d__2 as ::core::ffi::c_double)
        }) as doublereal;
        *xnorm = (if d__3 >= d__4 {
            d__3 as ::core::ffi::c_double
        } else {
            d__4 as ::core::ffi::c_double
        }) as doublereal;
    }
    return 0 as ::core::ffi::c_int;
}
