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
static mut c_b7: doublereal = 0.0f64;
static mut c_b8: doublereal = 1.0f64;
static mut c__3: integer = 3 as integer;
static mut c__1: integer = 1 as integer;
static mut c__2: integer = 2 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlaqr5_(
    mut wantt: *mut logical,
    mut wantz: *mut logical,
    mut kacc22: *mut integer,
    mut n: *mut integer,
    mut ktop: *mut integer,
    mut kbot: *mut integer,
    mut nshfts: *mut integer,
    mut sr: *mut doublereal,
    mut si: *mut doublereal,
    mut h__: *mut doublereal,
    mut ldh: *mut integer,
    mut iloz: *mut integer,
    mut ihiz: *mut integer,
    mut z__: *mut doublereal,
    mut ldz: *mut integer,
    mut v: *mut doublereal,
    mut ldv: *mut integer,
    mut u: *mut doublereal,
    mut ldu: *mut integer,
    mut nv: *mut integer,
    mut wv: *mut doublereal,
    mut ldwv: *mut integer,
    mut nh: *mut integer,
    mut wh: *mut doublereal,
    mut ldwh: *mut integer,
) -> ::core::ffi::c_int {
    let mut h_dim1: integer = 0;
    let mut h_offset: integer = 0;
    let mut u_dim1: integer = 0;
    let mut u_offset: integer = 0;
    let mut v_dim1: integer = 0;
    let mut v_offset: integer = 0;
    let mut wh_dim1: integer = 0;
    let mut wh_offset: integer = 0;
    let mut wv_dim1: integer = 0;
    let mut wv_offset: integer = 0;
    let mut z_dim1: integer = 0;
    let mut z_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut i__6: integer = 0;
    let mut i__7: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__4: doublereal = 0.;
    let mut d__5: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut m: integer = 0;
    let mut i2: integer = 0;
    let mut j2: integer = 0;
    let mut i4: integer = 0;
    let mut j4: integer = 0;
    let mut k1: integer = 0;
    let mut h11: doublereal = 0.;
    let mut h12: doublereal = 0.;
    let mut h21: doublereal = 0.;
    let mut h22: doublereal = 0.;
    let mut m22: integer = 0;
    let mut ns: integer = 0;
    let mut nu: integer = 0;
    let mut vt: [doublereal; 3] = [0.; 3];
    let mut scl: doublereal = 0.;
    let mut kdu: integer = 0;
    let mut kms: integer = 0;
    let mut ulp: doublereal = 0.;
    let mut knz: integer = 0;
    let mut kzs: integer = 0;
    let mut tst1: doublereal = 0.;
    let mut tst2: doublereal = 0.;
    let mut beta: doublereal = 0.;
    let mut blk22: logical = 0;
    let mut bmp22: logical = 0;
    let mut mend: integer = 0;
    let mut jcol: integer = 0;
    let mut jlen: integer = 0;
    let mut jbot: integer = 0;
    let mut mbot: integer = 0;
    let mut swap: doublereal = 0.;
    let mut jtop: integer = 0;
    let mut jrow: integer = 0;
    let mut mtop: integer = 0;
    let mut alpha: doublereal = 0.;
    let mut accum: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dgemm"]
        fn f2c_dgemm_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
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
    let mut ndcol: integer = 0;
    let mut incol: integer = 0;
    let mut krcol: integer = 0;
    let mut nbmps: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_dtrmm"]
        fn f2c_dtrmm_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlaqr1_"]
        fn dlaqr1__0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
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
    extern "C" {
        #[link_name = "dgelsd_closure_dlacpy_"]
        fn dlacpy__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut safmin: doublereal = 0.;
    extern "C" {
        #[link_name = "dgeev_closure_dlaset_"]
        fn dlaset__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut safmax: doublereal = 0.;
    let mut refsum: doublereal = 0.;
    let mut mstart: integer = 0;
    let mut smlnum: doublereal = 0.;
    sr = sr.offset(-1);
    si = si.offset(-1);
    h_dim1 = *ldh;
    h_offset = 1 as integer + h_dim1;
    h__ = h__.offset(-(h_offset as isize));
    z_dim1 = *ldz;
    z_offset = 1 as integer + z_dim1;
    z__ = z__.offset(-(z_offset as isize));
    v_dim1 = *ldv;
    v_offset = 1 as integer + v_dim1;
    v = v.offset(-(v_offset as isize));
    u_dim1 = *ldu;
    u_offset = 1 as integer + u_dim1;
    u = u.offset(-(u_offset as isize));
    wv_dim1 = *ldwv;
    wv_offset = 1 as integer + wv_dim1;
    wv = wv.offset(-(wv_offset as isize));
    wh_dim1 = *ldwh;
    wh_offset = 1 as integer + wh_dim1;
    wh = wh.offset(-(wh_offset as isize));
    if *nshfts < 2 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *ktop >= *kbot {
        return 0 as ::core::ffi::c_int;
    }
    i__1 = (*nshfts - 2 as ::core::ffi::c_long) as integer;
    i__ = 1 as integer;
    while i__ <= i__1 {
        if *si.offset(i__ as isize)
            != -*si.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
        {
            swap = *sr.offset(i__ as isize);
            *sr.offset(i__ as isize) =
                *sr.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            *sr.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                *sr.offset((i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            *sr.offset((i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) = swap;
            swap = *si.offset(i__ as isize);
            *si.offset(i__ as isize) =
                *si.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            *si.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                *si.offset((i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            *si.offset((i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize) = swap;
        }
        i__ += 2 as ::core::ffi::c_long;
    }
    ns = (*nshfts - *nshfts % 2 as ::core::ffi::c_long) as integer;
    safmin = dlamch__0(
        b"SAFE MINIMUM\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    safmax = 1.0f64 / safmin;
    dlabad__0(&raw mut safmin, &raw mut safmax);
    ulp = dlamch__0(
        b"PRECISION\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    smlnum = safmin * (*n as doublereal / ulp);
    accum = (*kacc22 == 1 as ::core::ffi::c_long || *kacc22 == 2 as ::core::ffi::c_long)
        as ::core::ffi::c_int as logical;
    blk22 = (ns > 2 as ::core::ffi::c_long && *kacc22 == 2 as ::core::ffi::c_long)
        as ::core::ffi::c_int as logical;
    if *ktop + 2 as ::core::ffi::c_long <= *kbot {
        *h__.offset((*ktop + 2 as integer + *ktop * h_dim1) as isize) = 0.0f64 as doublereal;
    }
    nbmps = (ns as ::core::ffi::c_long / 2 as ::core::ffi::c_long) as integer;
    kdu = (nbmps as ::core::ffi::c_long * 6 as ::core::ffi::c_long - 3 as ::core::ffi::c_long)
        as integer;
    i__1 = (*kbot - 2 as ::core::ffi::c_long) as integer;
    i__2 = (nbmps as ::core::ffi::c_long * 3 as ::core::ffi::c_long - 2 as ::core::ffi::c_long)
        as integer;
    incol = ((1 as ::core::ffi::c_long - nbmps as ::core::ffi::c_long) * 3 as ::core::ffi::c_long
        + *ktop
        - 1 as ::core::ffi::c_long) as integer;
    while if i__2 < 0 as ::core::ffi::c_long {
        (incol >= i__1) as ::core::ffi::c_int
    } else {
        (incol <= i__1) as ::core::ffi::c_int
    } != 0
    {
        ndcol = incol + kdu;
        if accum != 0 {
            dlaset__0(
                b"ALL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut kdu,
                &raw mut kdu,
                &raw mut c_b7,
                &raw mut c_b8,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
            );
        }
        i__4 = (incol as ::core::ffi::c_long
            + nbmps as ::core::ffi::c_long * 3 as ::core::ffi::c_long
            - 3 as ::core::ffi::c_long) as integer;
        i__5 = (*kbot - 2 as ::core::ffi::c_long) as integer;
        i__3 = (if i__4 <= i__5 {
            i__4 as ::core::ffi::c_long
        } else {
            i__5 as ::core::ffi::c_long
        }) as integer;
        krcol = incol;
        while krcol <= i__3 {
            i__4 = 1 as integer;
            i__5 = ((*ktop - 1 as ::core::ffi::c_long - krcol as ::core::ffi::c_long
                + 2 as ::core::ffi::c_long)
                / 3 as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            mtop = (if i__4 >= i__5 {
                i__4 as ::core::ffi::c_long
            } else {
                i__5 as ::core::ffi::c_long
            }) as integer;
            i__4 = nbmps;
            i__5 = ((*kbot - krcol as ::core::ffi::c_long) / 3 as ::core::ffi::c_long) as integer;
            mbot = (if i__4 <= i__5 {
                i__4 as ::core::ffi::c_long
            } else {
                i__5 as ::core::ffi::c_long
            }) as integer;
            m22 = (mbot as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            bmp22 = (mbot < nbmps
                && krcol as ::core::ffi::c_long
                    + (m22 as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                        * 3 as ::core::ffi::c_long
                    == *kbot - 2 as ::core::ffi::c_long) as ::core::ffi::c_int
                as logical;
            i__4 = mbot;
            m = mtop;
            while m <= i__4 {
                k = (krcol as ::core::ffi::c_long
                    + (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                        * 3 as ::core::ffi::c_long) as integer;
                if k == *ktop - 1 as ::core::ffi::c_long {
                    dlaqr1__0(
                        &raw mut c__3,
                        h__.offset((*ktop + *ktop * h_dim1) as isize) as *mut doublereal,
                        ldh,
                        sr.offset(
                            (((m as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        si.offset(
                            (((m as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        sr.offset((m as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize)
                            as *mut doublereal,
                        si.offset((m as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize)
                            as *mut doublereal,
                        v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                    );
                    alpha = *v.offset(
                        (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    dlarfg__0(
                        &raw mut c__3,
                        &raw mut alpha,
                        v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                    );
                } else {
                    beta = *h__.offset((k + 1 as integer + k * h_dim1) as isize);
                    *v.offset(
                        (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    ) = *h__.offset((k + 2 as integer + k * h_dim1) as isize);
                    *v.offset(
                        (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    ) = *h__.offset((k + 3 as integer + k * h_dim1) as isize);
                    dlarfg__0(
                        &raw mut c__3,
                        &raw mut beta,
                        v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                    );
                    if *h__.offset((k + 3 as integer + k * h_dim1) as isize) != 0.0f64
                        || *h__.offset((k + 3 as integer + (k + 1 as integer) * h_dim1) as isize)
                            != 0.0f64
                        || *h__.offset((k + 3 as integer + (k + 2 as integer) * h_dim1) as isize)
                            == 0.0f64
                    {
                        *h__.offset((k + 1 as integer + k * h_dim1) as isize) = beta;
                        *h__.offset((k + 2 as integer + k * h_dim1) as isize) =
                            0.0f64 as doublereal;
                        *h__.offset((k + 3 as integer + k * h_dim1) as isize) =
                            0.0f64 as doublereal;
                    } else {
                        dlaqr1__0(
                            &raw mut c__3,
                            h__.offset((k + 1 as integer + (k + 1 as integer) * h_dim1) as isize)
                                as *mut doublereal,
                            ldh,
                            sr.offset(
                                (((m as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            si.offset(
                                (((m as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            sr.offset(
                                (m as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize,
                            ) as *mut doublereal,
                            si.offset(
                                (m as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize,
                            ) as *mut doublereal,
                            &raw mut vt as *mut doublereal,
                        );
                        alpha = vt[0 as ::core::ffi::c_int as usize];
                        dlarfg__0(
                            &raw mut c__3,
                            &raw mut alpha,
                            (&raw mut vt as *mut doublereal)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut doublereal,
                            &raw mut c__1,
                            &raw mut vt as *mut doublereal,
                        );
                        refsum = vt[0 as ::core::ffi::c_int as usize]
                            * (*h__.offset((k + 1 as integer + k * h_dim1) as isize)
                                + vt[1 as ::core::ffi::c_int as usize]
                                    * *h__.offset((k + 2 as integer + k * h_dim1) as isize));
                        d__1 = *h__.offset((k + 2 as integer + k * h_dim1) as isize)
                            - refsum * vt[1 as ::core::ffi::c_int as usize];
                        d__2 = refsum * vt[2 as ::core::ffi::c_int as usize];
                        d__3 = *h__.offset((k + k * h_dim1) as isize);
                        d__4 =
                            *h__.offset((k + 1 as integer + (k + 1 as integer) * h_dim1) as isize);
                        d__5 =
                            *h__.offset((k + 2 as integer + (k + 2 as integer) * h_dim1) as isize);
                        if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) > ulp as ::core::ffi::c_double
                            * ((if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__3 as ::core::ffi::c_double
                            } else {
                                -(d__3 as ::core::ffi::c_double)
                            }) + (if d__4 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__4 as ::core::ffi::c_double
                            } else {
                                -(d__4 as ::core::ffi::c_double)
                            }) + (if d__5 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__5 as ::core::ffi::c_double
                            } else {
                                -(d__5 as ::core::ffi::c_double)
                            }))
                        {
                            *h__.offset((k + 1 as integer + k * h_dim1) as isize) = beta;
                            *h__.offset((k + 2 as integer + k * h_dim1) as isize) =
                                0.0f64 as doublereal;
                            *h__.offset((k + 3 as integer + k * h_dim1) as isize) =
                                0.0f64 as doublereal;
                        } else {
                            let ref mut fresh0 =
                                *h__.offset((k + 1 as integer + k * h_dim1) as isize);
                            *fresh0 -= refsum as ::core::ffi::c_double;
                            *h__.offset((k + 2 as integer + k * h_dim1) as isize) =
                                0.0f64 as doublereal;
                            *h__.offset((k + 3 as integer + k * h_dim1) as isize) =
                                0.0f64 as doublereal;
                            *v.offset(
                                (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) = vt[0 as ::core::ffi::c_int as usize];
                            *v.offset(
                                (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 2 as ::core::ffi::c_long)
                                    as isize,
                            ) = vt[1 as ::core::ffi::c_int as usize];
                            *v.offset(
                                (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 3 as ::core::ffi::c_long)
                                    as isize,
                            ) = vt[2 as ::core::ffi::c_int as usize];
                        }
                    }
                }
                m += 1;
            }
            k = (krcol as ::core::ffi::c_long
                + (m22 as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                    * 3 as ::core::ffi::c_long) as integer;
            if bmp22 != 0 {
                if k == *ktop - 1 as ::core::ffi::c_long {
                    dlaqr1__0(
                        &raw mut c__2,
                        h__.offset((k + 1 as integer + (k + 1 as integer) * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                        sr.offset(
                            (((m22 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        si.offset(
                            (((m22 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        sr.offset((m22 as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize)
                            as *mut doublereal,
                        si.offset((m22 as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize)
                            as *mut doublereal,
                        v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                    );
                    beta = *v.offset(
                        (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    dlarfg__0(
                        &raw mut c__2,
                        &raw mut beta,
                        v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                    );
                } else {
                    beta = *h__.offset((k + 1 as integer + k * h_dim1) as isize);
                    *v.offset(
                        (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    ) = *h__.offset((k + 2 as integer + k * h_dim1) as isize);
                    dlarfg__0(
                        &raw mut c__2,
                        &raw mut beta,
                        v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                    );
                    *h__.offset((k + 1 as integer + k * h_dim1) as isize) = beta;
                    *h__.offset((k + 2 as integer + k * h_dim1) as isize) = 0.0f64 as doublereal;
                }
            }
            if accum != 0 {
                jbot = (if ndcol <= *kbot {
                    ndcol as ::core::ffi::c_long
                } else {
                    *kbot
                }) as integer;
            } else if *wantt != 0 {
                jbot = *n;
            } else {
                jbot = *kbot;
            }
            i__4 = jbot;
            j = (if *ktop >= krcol {
                *ktop
            } else {
                krcol as ::core::ffi::c_long
            }) as integer;
            while j <= i__4 {
                i__5 = mbot;
                i__6 = ((j as ::core::ffi::c_long - krcol as ::core::ffi::c_long
                    + 2 as ::core::ffi::c_long)
                    / 3 as ::core::ffi::c_long) as integer;
                mend = (if i__5 <= i__6 {
                    i__5 as ::core::ffi::c_long
                } else {
                    i__6 as ::core::ffi::c_long
                }) as integer;
                i__5 = mend;
                m = mtop;
                while m <= i__5 {
                    k = (krcol as ::core::ffi::c_long
                        + (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                            * 3 as ::core::ffi::c_long) as integer;
                    refsum = *v.offset(
                        (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) * (*h__.offset((k + 1 as integer + j * h_dim1) as isize)
                        + *v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        ) * *h__.offset((k + 2 as integer + j * h_dim1) as isize)
                        + *v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        ) * *h__.offset((k + 3 as integer + j * h_dim1) as isize));
                    let ref mut fresh1 = *h__.offset((k + 1 as integer + j * h_dim1) as isize);
                    *fresh1 -= refsum as ::core::ffi::c_double;
                    let ref mut fresh2 = *h__.offset((k + 2 as integer + j * h_dim1) as isize);
                    *fresh2 -= (refsum
                        * *v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )) as ::core::ffi::c_double;
                    let ref mut fresh3 = *h__.offset((k + 3 as integer + j * h_dim1) as isize);
                    *fresh3 -= (refsum
                        * *v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        )) as ::core::ffi::c_double;
                    m += 1;
                }
                j += 1;
            }
            if bmp22 != 0 {
                k = (krcol as ::core::ffi::c_long
                    + (m22 as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                        * 3 as ::core::ffi::c_long) as integer;
                i__4 = (k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                i__5 = jbot;
                j = (if i__4 >= *ktop {
                    i__4 as ::core::ffi::c_long
                } else {
                    *ktop
                }) as integer;
                while j <= i__5 {
                    refsum = *v.offset(
                        (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) * (*h__.offset((k + 1 as integer + j * h_dim1) as isize)
                        + *v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        ) * *h__.offset((k + 2 as integer + j * h_dim1) as isize));
                    let ref mut fresh4 = *h__.offset((k + 1 as integer + j * h_dim1) as isize);
                    *fresh4 -= refsum as ::core::ffi::c_double;
                    let ref mut fresh5 = *h__.offset((k + 2 as integer + j * h_dim1) as isize);
                    *fresh5 -= (refsum
                        * *v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            if accum != 0 {
                jtop = (if *ktop >= incol {
                    *ktop
                } else {
                    incol as ::core::ffi::c_long
                }) as integer;
            } else if *wantt != 0 {
                jtop = 1 as integer;
            } else {
                jtop = *ktop;
            }
            i__5 = mbot;
            m = mtop;
            while m <= i__5 {
                if *v.offset(
                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) != 0.0f64
                {
                    k = (krcol as ::core::ffi::c_long
                        + (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                            * 3 as ::core::ffi::c_long) as integer;
                    i__6 = *kbot;
                    i__7 = (k as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as integer;
                    i__4 = (if i__6 <= i__7 {
                        i__6 as ::core::ffi::c_long
                    } else {
                        i__7 as ::core::ffi::c_long
                    }) as integer;
                    j = jtop;
                    while j <= i__4 {
                        refsum = *v.offset(
                            (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) * (*h__.offset((j + (k + 1 as integer) * h_dim1) as isize)
                            + *v.offset(
                                (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 2 as ::core::ffi::c_long)
                                    as isize,
                            ) * *h__.offset((j + (k + 2 as integer) * h_dim1) as isize)
                            + *v.offset(
                                (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 3 as ::core::ffi::c_long)
                                    as isize,
                            ) * *h__.offset((j + (k + 3 as integer) * h_dim1) as isize));
                        let ref mut fresh6 =
                            *h__.offset((j + (k + 1 as integer) * h_dim1) as isize);
                        *fresh6 -= refsum as ::core::ffi::c_double;
                        let ref mut fresh7 =
                            *h__.offset((j + (k + 2 as integer) * h_dim1) as isize);
                        *fresh7 -= (refsum
                            * *v.offset(
                                (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 2 as ::core::ffi::c_long)
                                    as isize,
                            )) as ::core::ffi::c_double;
                        let ref mut fresh8 =
                            *h__.offset((j + (k + 3 as integer) * h_dim1) as isize);
                        *fresh8 -= (refsum
                            * *v.offset(
                                (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 3 as ::core::ffi::c_long)
                                    as isize,
                            )) as ::core::ffi::c_double;
                        j += 1;
                    }
                    if accum != 0 {
                        kms = k - incol;
                        i__4 = 1 as integer;
                        i__6 = *ktop - incol;
                        i__7 = kdu;
                        j = (if i__4 >= i__6 {
                            i__4 as ::core::ffi::c_long
                        } else {
                            i__6 as ::core::ffi::c_long
                        }) as integer;
                        while j <= i__7 {
                            refsum = *v.offset(
                                (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) * (*u.offset((j + (kms + 1 as integer) * u_dim1) as isize)
                                + *v.offset(
                                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                        + 2 as ::core::ffi::c_long)
                                        as isize,
                                ) * *u.offset((j + (kms + 2 as integer) * u_dim1) as isize)
                                + *v.offset(
                                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                        + 3 as ::core::ffi::c_long)
                                        as isize,
                                ) * *u.offset((j + (kms + 3 as integer) * u_dim1) as isize));
                            let ref mut fresh9 =
                                *u.offset((j + (kms + 1 as integer) * u_dim1) as isize);
                            *fresh9 -= refsum as ::core::ffi::c_double;
                            let ref mut fresh10 =
                                *u.offset((j + (kms + 2 as integer) * u_dim1) as isize);
                            *fresh10 -= (refsum
                                * *v.offset(
                                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                        + 2 as ::core::ffi::c_long)
                                        as isize,
                                )) as ::core::ffi::c_double;
                            let ref mut fresh11 =
                                *u.offset((j + (kms + 3 as integer) * u_dim1) as isize);
                            *fresh11 -= (refsum
                                * *v.offset(
                                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                        + 3 as ::core::ffi::c_long)
                                        as isize,
                                )) as ::core::ffi::c_double;
                            j += 1;
                        }
                    } else if *wantz != 0 {
                        i__7 = *ihiz;
                        j = *iloz;
                        while j <= i__7 {
                            refsum = *v.offset(
                                (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) * (*z__.offset((j + (k + 1 as integer) * z_dim1) as isize)
                                + *v.offset(
                                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                        + 2 as ::core::ffi::c_long)
                                        as isize,
                                ) * *z__.offset((j + (k + 2 as integer) * z_dim1) as isize)
                                + *v.offset(
                                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                        + 3 as ::core::ffi::c_long)
                                        as isize,
                                ) * *z__.offset((j + (k + 3 as integer) * z_dim1) as isize));
                            let ref mut fresh12 =
                                *z__.offset((j + (k + 1 as integer) * z_dim1) as isize);
                            *fresh12 -= refsum as ::core::ffi::c_double;
                            let ref mut fresh13 =
                                *z__.offset((j + (k + 2 as integer) * z_dim1) as isize);
                            *fresh13 -= (refsum
                                * *v.offset(
                                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                        + 2 as ::core::ffi::c_long)
                                        as isize,
                                )) as ::core::ffi::c_double;
                            let ref mut fresh14 =
                                *z__.offset((j + (k + 3 as integer) * z_dim1) as isize);
                            *fresh14 -= (refsum
                                * *v.offset(
                                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                        + 3 as ::core::ffi::c_long)
                                        as isize,
                                )) as ::core::ffi::c_double;
                            j += 1;
                        }
                    }
                }
                m += 1;
            }
            k = (krcol as ::core::ffi::c_long
                + (m22 as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                    * 3 as ::core::ffi::c_long) as integer;
            if bmp22 != 0
                && *v.offset(
                    (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) != 0.0f64
            {
                i__7 = *kbot;
                i__4 = (k as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as integer;
                i__5 = (if i__7 <= i__4 {
                    i__7 as ::core::ffi::c_long
                } else {
                    i__4 as ::core::ffi::c_long
                }) as integer;
                j = jtop;
                while j <= i__5 {
                    refsum = *v.offset(
                        (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) * (*h__.offset((j + (k + 1 as integer) * h_dim1) as isize)
                        + *v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        ) * *h__.offset((j + (k + 2 as integer) * h_dim1) as isize));
                    let ref mut fresh15 = *h__.offset((j + (k + 1 as integer) * h_dim1) as isize);
                    *fresh15 -= refsum as ::core::ffi::c_double;
                    let ref mut fresh16 = *h__.offset((j + (k + 2 as integer) * h_dim1) as isize);
                    *fresh16 -= (refsum
                        * *v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )) as ::core::ffi::c_double;
                    j += 1;
                }
                if accum != 0 {
                    kms = k - incol;
                    i__5 = 1 as integer;
                    i__7 = *ktop - incol;
                    i__4 = kdu;
                    j = (if i__5 >= i__7 {
                        i__5 as ::core::ffi::c_long
                    } else {
                        i__7 as ::core::ffi::c_long
                    }) as integer;
                    while j <= i__4 {
                        refsum = *v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) * (*u.offset((j + (kms + 1 as integer) * u_dim1) as isize)
                            + *v.offset(
                                (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 2 as ::core::ffi::c_long)
                                    as isize,
                            ) * *u.offset((j + (kms + 2 as integer) * u_dim1) as isize));
                        let ref mut fresh17 =
                            *u.offset((j + (kms + 1 as integer) * u_dim1) as isize);
                        *fresh17 -= refsum as ::core::ffi::c_double;
                        let ref mut fresh18 =
                            *u.offset((j + (kms + 2 as integer) * u_dim1) as isize);
                        *fresh18 -= (refsum
                            * *v.offset(
                                (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 2 as ::core::ffi::c_long)
                                    as isize,
                            )) as ::core::ffi::c_double;
                        j += 1;
                    }
                } else if *wantz != 0 {
                    i__4 = *ihiz;
                    j = *iloz;
                    while j <= i__4 {
                        refsum = *v.offset(
                            (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) * (*z__.offset((j + (k + 1 as integer) * z_dim1) as isize)
                            + *v.offset(
                                (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 2 as ::core::ffi::c_long)
                                    as isize,
                            ) * *z__.offset((j + (k + 2 as integer) * z_dim1) as isize));
                        let ref mut fresh19 =
                            *z__.offset((j + (k + 1 as integer) * z_dim1) as isize);
                        *fresh19 -= refsum as ::core::ffi::c_double;
                        let ref mut fresh20 =
                            *z__.offset((j + (k + 2 as integer) * z_dim1) as isize);
                        *fresh20 -= (refsum
                            * *v.offset(
                                (m22 as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                    + 2 as ::core::ffi::c_long)
                                    as isize,
                            )) as ::core::ffi::c_double;
                        j += 1;
                    }
                }
            }
            mstart = mtop;
            if (krcol as ::core::ffi::c_long
                + (mstart as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                    * 3 as ::core::ffi::c_long)
                < *ktop
            {
                mstart += 1;
            }
            mend = mbot;
            if bmp22 != 0 {
                mend += 1;
            }
            if krcol == *kbot - 2 as ::core::ffi::c_long {
                mend += 1;
            }
            i__4 = mend;
            m = mstart;
            while m <= i__4 {
                i__5 = (*kbot - 1 as ::core::ffi::c_long) as integer;
                i__7 = (krcol as ::core::ffi::c_long
                    + (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                        * 3 as ::core::ffi::c_long) as integer;
                k = (if i__5 <= i__7 {
                    i__5 as ::core::ffi::c_long
                } else {
                    i__7 as ::core::ffi::c_long
                }) as integer;
                if *h__.offset((k + 1 as integer + k * h_dim1) as isize) != 0.0f64 {
                    d__1 = *h__.offset((k + k * h_dim1) as isize);
                    d__2 = *h__.offset((k + 1 as integer + (k + 1 as integer) * h_dim1) as isize);
                    tst1 = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__2 as ::core::ffi::c_double
                    } else {
                        -(d__2 as ::core::ffi::c_double)
                    })) as doublereal;
                    if tst1 == 0.0f64 {
                        if k >= *ktop + 1 as ::core::ffi::c_long {
                            d__1 = *h__.offset((k + (k - 1 as integer) * h_dim1) as isize);
                            tst1 += (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            });
                        }
                        if k >= *ktop + 2 as ::core::ffi::c_long {
                            d__1 = *h__.offset((k + (k - 2 as integer) * h_dim1) as isize);
                            tst1 += (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            });
                        }
                        if k >= *ktop + 3 as ::core::ffi::c_long {
                            d__1 = *h__.offset((k + (k - 3 as integer) * h_dim1) as isize);
                            tst1 += (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            });
                        }
                        if k <= *kbot - 2 as ::core::ffi::c_long {
                            d__1 = *h__
                                .offset((k + 2 as integer + (k + 1 as integer) * h_dim1) as isize);
                            tst1 += (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            });
                        }
                        if k <= *kbot - 3 as ::core::ffi::c_long {
                            d__1 = *h__
                                .offset((k + 3 as integer + (k + 1 as integer) * h_dim1) as isize);
                            tst1 += (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            });
                        }
                        if k <= *kbot - 4 as ::core::ffi::c_long {
                            d__1 = *h__
                                .offset((k + 4 as integer + (k + 1 as integer) * h_dim1) as isize);
                            tst1 += (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            });
                        }
                    }
                    d__2 = smlnum;
                    d__3 = ulp * tst1;
                    d__1 = *h__.offset((k + 1 as integer + k * h_dim1) as isize);
                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) <= (if d__2 >= d__3 {
                        d__2 as ::core::ffi::c_double
                    } else {
                        d__3 as ::core::ffi::c_double
                    }) {
                        d__1 = *h__.offset((k + 1 as integer + k * h_dim1) as isize);
                        d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        d__2 = *h__.offset((k + (k + 1 as integer) * h_dim1) as isize);
                        d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) as doublereal;
                        h12 = (if d__3 >= d__4 {
                            d__3 as ::core::ffi::c_double
                        } else {
                            d__4 as ::core::ffi::c_double
                        }) as doublereal;
                        d__1 = *h__.offset((k + 1 as integer + k * h_dim1) as isize);
                        d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        d__2 = *h__.offset((k + (k + 1 as integer) * h_dim1) as isize);
                        d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) as doublereal;
                        h21 = (if d__3 <= d__4 {
                            d__3 as ::core::ffi::c_double
                        } else {
                            d__4 as ::core::ffi::c_double
                        }) as doublereal;
                        d__1 =
                            *h__.offset((k + 1 as integer + (k + 1 as integer) * h_dim1) as isize);
                        d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        d__2 = *h__.offset((k + k * h_dim1) as isize)
                            - *h__
                                .offset((k + 1 as integer + (k + 1 as integer) * h_dim1) as isize);
                        d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) as doublereal;
                        h11 = (if d__3 >= d__4 {
                            d__3 as ::core::ffi::c_double
                        } else {
                            d__4 as ::core::ffi::c_double
                        }) as doublereal;
                        d__1 =
                            *h__.offset((k + 1 as integer + (k + 1 as integer) * h_dim1) as isize);
                        d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) as doublereal;
                        d__2 = *h__.offset((k + k * h_dim1) as isize)
                            - *h__
                                .offset((k + 1 as integer + (k + 1 as integer) * h_dim1) as isize);
                        d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) as doublereal;
                        h22 = (if d__3 <= d__4 {
                            d__3 as ::core::ffi::c_double
                        } else {
                            d__4 as ::core::ffi::c_double
                        }) as doublereal;
                        scl = h11 + h12;
                        tst2 = h22 * (h11 / scl);
                        d__1 = smlnum;
                        d__2 = ulp * tst2;
                        if tst2 == 0.0f64
                            || h21 * (h12 / scl)
                                <= (if d__1 >= d__2 {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    d__2 as ::core::ffi::c_double
                                })
                        {
                            *h__.offset((k + 1 as integer + k * h_dim1) as isize) =
                                0.0f64 as doublereal;
                        }
                    }
                }
                m += 1;
            }
            i__4 = nbmps;
            i__5 = ((*kbot - krcol as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                / 3 as ::core::ffi::c_long) as integer;
            mend = (if i__4 <= i__5 {
                i__4 as ::core::ffi::c_long
            } else {
                i__5 as ::core::ffi::c_long
            }) as integer;
            i__4 = mend;
            m = mtop;
            while m <= i__4 {
                k = (krcol as ::core::ffi::c_long
                    + (m as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                        * 3 as ::core::ffi::c_long) as integer;
                refsum = *v.offset(
                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) * *v.offset(
                    (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                        + 3 as ::core::ffi::c_long) as isize,
                ) * *h__.offset((k + 4 as integer + (k + 3 as integer) * h_dim1) as isize);
                *h__.offset((k + 4 as integer + (k + 1 as integer) * h_dim1) as isize) = -refsum;
                *h__.offset((k + 4 as integer + (k + 2 as integer) * h_dim1) as isize) = -refsum
                    * *v.offset(
                        (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                let ref mut fresh21 =
                    *h__.offset((k + 4 as integer + (k + 3 as integer) * h_dim1) as isize);
                *fresh21 -= (refsum
                    * *v.offset(
                        (m as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    )) as ::core::ffi::c_double;
                m += 1;
            }
            krcol += 1;
        }
        if accum != 0 {
            if *wantt != 0 {
                jtop = 1 as integer;
                jbot = *n;
            } else {
                jtop = *ktop;
                jbot = *kbot;
            }
            if blk22 == 0 || incol < *ktop || ndcol > *kbot || ns <= 2 as ::core::ffi::c_long {
                i__3 = 1 as integer;
                i__4 = *ktop - incol;
                k1 = (if i__3 >= i__4 {
                    i__3 as ::core::ffi::c_long
                } else {
                    i__4 as ::core::ffi::c_long
                }) as integer;
                i__3 = 0 as integer;
                i__4 = ndcol - *kbot;
                nu = (kdu as ::core::ffi::c_long
                    - (if i__3 >= i__4 {
                        i__3 as ::core::ffi::c_long
                    } else {
                        i__4 as ::core::ffi::c_long
                    })
                    - k1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                i__3 = jbot;
                i__4 = *nh;
                jcol = ((if ndcol <= *kbot {
                    ndcol as ::core::ffi::c_long
                } else {
                    *kbot
                }) + 1 as ::core::ffi::c_long) as integer;
                while if i__4 < 0 as ::core::ffi::c_long {
                    (jcol >= i__3) as ::core::ffi::c_int
                } else {
                    (jcol <= i__3) as ::core::ffi::c_int
                } != 0
                {
                    i__5 = *nh;
                    i__7 = (jbot as ::core::ffi::c_long - jcol as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                    jlen = (if i__5 <= i__7 {
                        i__5 as ::core::ffi::c_long
                    } else {
                        i__7 as ::core::ffi::c_long
                    }) as integer;
                    f2c_dgemm_0(
                        b"C\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut nu,
                        &raw mut jlen,
                        &raw mut nu,
                        &raw mut c_b8,
                        u.offset((k1 + k1 * u_dim1) as isize) as *mut doublereal,
                        ldu,
                        h__.offset((incol + k1 + jcol * h_dim1) as isize) as *mut doublereal,
                        ldh,
                        &raw mut c_b7,
                        wh.offset(wh_offset as isize) as *mut doublereal,
                        ldwh,
                    );
                    dlacpy__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut nu,
                        &raw mut jlen,
                        wh.offset(wh_offset as isize) as *mut doublereal,
                        ldwh,
                        h__.offset((incol + k1 + jcol * h_dim1) as isize) as *mut doublereal,
                        ldh,
                    );
                    jcol += i__4 as ::core::ffi::c_long;
                }
                i__4 = ((if *ktop >= incol {
                    *ktop
                } else {
                    incol as ::core::ffi::c_long
                }) - 1 as ::core::ffi::c_long) as integer;
                i__3 = *nv;
                jrow = jtop;
                while if i__3 < 0 as ::core::ffi::c_long {
                    (jrow >= i__4) as ::core::ffi::c_int
                } else {
                    (jrow <= i__4) as ::core::ffi::c_int
                } != 0
                {
                    i__5 = *nv;
                    i__7 = (if *ktop >= incol { *ktop } else { incol }) - jrow;
                    jlen = (if i__5 <= i__7 {
                        i__5 as ::core::ffi::c_long
                    } else {
                        i__7 as ::core::ffi::c_long
                    }) as integer;
                    f2c_dgemm_0(
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut nu,
                        &raw mut nu,
                        &raw mut c_b8,
                        h__.offset((jrow + (incol + k1) * h_dim1) as isize) as *mut doublereal,
                        ldh,
                        u.offset((k1 + k1 * u_dim1) as isize) as *mut doublereal,
                        ldu,
                        &raw mut c_b7,
                        wv.offset(wv_offset as isize) as *mut doublereal,
                        ldwv,
                    );
                    dlacpy__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut nu,
                        wv.offset(wv_offset as isize) as *mut doublereal,
                        ldwv,
                        h__.offset((jrow + (incol + k1) * h_dim1) as isize) as *mut doublereal,
                        ldh,
                    );
                    jrow += i__3 as ::core::ffi::c_long;
                }
                if *wantz != 0 {
                    i__3 = *ihiz;
                    i__4 = *nv;
                    jrow = *iloz;
                    while if i__4 < 0 as ::core::ffi::c_long {
                        (jrow >= i__3) as ::core::ffi::c_int
                    } else {
                        (jrow <= i__3) as ::core::ffi::c_int
                    } != 0
                    {
                        i__5 = *nv;
                        i__7 = (*ihiz - jrow as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            as integer;
                        jlen = (if i__5 <= i__7 {
                            i__5 as ::core::ffi::c_long
                        } else {
                            i__7 as ::core::ffi::c_long
                        }) as integer;
                        f2c_dgemm_0(
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut nu,
                            &raw mut nu,
                            &raw mut c_b8,
                            z__.offset((jrow + (incol + k1) * z_dim1) as isize) as *mut doublereal,
                            ldz,
                            u.offset((k1 + k1 * u_dim1) as isize) as *mut doublereal,
                            ldu,
                            &raw mut c_b7,
                            wv.offset(wv_offset as isize) as *mut doublereal,
                            ldwv,
                        );
                        dlacpy__0(
                            b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut nu,
                            wv.offset(wv_offset as isize) as *mut doublereal,
                            ldwv,
                            z__.offset((jrow + (incol + k1) * z_dim1) as isize) as *mut doublereal,
                            ldz,
                        );
                        jrow += i__4 as ::core::ffi::c_long;
                    }
                }
            } else {
                i2 = ((kdu as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    / 2 as ::core::ffi::c_long) as integer;
                i4 = kdu;
                j2 = i4 - i2;
                j4 = kdu;
                kzs = (j4 as ::core::ffi::c_long
                    - j2 as ::core::ffi::c_long
                    - (ns as ::core::ffi::c_long + 1 as ::core::ffi::c_long))
                    as integer;
                knz = (ns as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                i__4 = jbot;
                i__3 = *nh;
                jcol = ((if ndcol <= *kbot {
                    ndcol as ::core::ffi::c_long
                } else {
                    *kbot
                }) + 1 as ::core::ffi::c_long) as integer;
                while if i__3 < 0 as ::core::ffi::c_long {
                    (jcol >= i__4) as ::core::ffi::c_int
                } else {
                    (jcol <= i__4) as ::core::ffi::c_int
                } != 0
                {
                    i__5 = *nh;
                    i__7 = (jbot as ::core::ffi::c_long - jcol as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                    jlen = (if i__5 <= i__7 {
                        i__5 as ::core::ffi::c_long
                    } else {
                        i__7 as ::core::ffi::c_long
                    }) as integer;
                    dlacpy__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut knz,
                        &raw mut jlen,
                        h__.offset((incol + 1 as integer + j2 + jcol * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                        wh.offset((kzs + 1 as integer + wh_dim1) as isize) as *mut doublereal,
                        ldwh,
                    );
                    dlaset__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut kzs,
                        &raw mut jlen,
                        &raw mut c_b7,
                        &raw mut c_b7,
                        wh.offset(wh_offset as isize) as *mut doublereal,
                        ldwh,
                    );
                    f2c_dtrmm_0(
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"U\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"C\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut knz,
                        &raw mut jlen,
                        &raw mut c_b8,
                        u.offset((j2 + 1 as integer + (kzs + 1 as integer) * u_dim1) as isize)
                            as *mut doublereal,
                        ldu,
                        wh.offset((kzs + 1 as integer + wh_dim1) as isize) as *mut doublereal,
                        ldwh,
                    );
                    f2c_dgemm_0(
                        b"C\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut i2,
                        &raw mut jlen,
                        &raw mut j2,
                        &raw mut c_b8,
                        u.offset(u_offset as isize) as *mut doublereal,
                        ldu,
                        h__.offset((incol + 1 as integer + jcol * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                        &raw mut c_b8,
                        wh.offset(wh_offset as isize) as *mut doublereal,
                        ldwh,
                    );
                    dlacpy__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut j2,
                        &raw mut jlen,
                        h__.offset((incol + 1 as integer + jcol * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                        wh.offset((i2 + 1 as integer + wh_dim1) as isize) as *mut doublereal,
                        ldwh,
                    );
                    f2c_dtrmm_0(
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"C\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut j2,
                        &raw mut jlen,
                        &raw mut c_b8,
                        u.offset(
                            ((i2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * u_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldu,
                        wh.offset((i2 + 1 as integer + wh_dim1) as isize) as *mut doublereal,
                        ldwh,
                    );
                    i__5 = i4 - i2;
                    i__7 = j4 - j2;
                    f2c_dgemm_0(
                        b"C\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut i__5,
                        &raw mut jlen,
                        &raw mut i__7,
                        &raw mut c_b8,
                        u.offset((j2 + 1 as integer + (i2 + 1 as integer) * u_dim1) as isize)
                            as *mut doublereal,
                        ldu,
                        h__.offset((incol + 1 as integer + j2 + jcol * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                        &raw mut c_b8,
                        wh.offset((i2 + 1 as integer + wh_dim1) as isize) as *mut doublereal,
                        ldwh,
                    );
                    dlacpy__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut kdu,
                        &raw mut jlen,
                        wh.offset(wh_offset as isize) as *mut doublereal,
                        ldwh,
                        h__.offset((incol + 1 as integer + jcol * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                    );
                    jcol += i__3 as ::core::ffi::c_long;
                }
                i__3 = ((if incol >= *ktop {
                    incol as ::core::ffi::c_long
                } else {
                    *ktop
                }) - 1 as ::core::ffi::c_long) as integer;
                i__4 = *nv;
                jrow = jtop;
                while if i__4 < 0 as ::core::ffi::c_long {
                    (jrow >= i__3) as ::core::ffi::c_int
                } else {
                    (jrow <= i__3) as ::core::ffi::c_int
                } != 0
                {
                    i__5 = *nv;
                    i__7 = (if incol >= *ktop { incol } else { *ktop }) - jrow;
                    jlen = (if i__5 <= i__7 {
                        i__5 as ::core::ffi::c_long
                    } else {
                        i__7 as ::core::ffi::c_long
                    }) as integer;
                    dlacpy__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut knz,
                        h__.offset((jrow + (incol + 1 as integer + j2) * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                        wv.offset(
                            ((kzs as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * wv_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldwv,
                    );
                    dlaset__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut kzs,
                        &raw mut c_b7,
                        &raw mut c_b7,
                        wv.offset(wv_offset as isize) as *mut doublereal,
                        ldwv,
                    );
                    f2c_dtrmm_0(
                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"U\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut knz,
                        &raw mut c_b8,
                        u.offset((j2 + 1 as integer + (kzs + 1 as integer) * u_dim1) as isize)
                            as *mut doublereal,
                        ldu,
                        wv.offset(
                            ((kzs as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * wv_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldwv,
                    );
                    f2c_dgemm_0(
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut i2,
                        &raw mut j2,
                        &raw mut c_b8,
                        h__.offset((jrow + (incol + 1 as integer) * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                        u.offset(u_offset as isize) as *mut doublereal,
                        ldu,
                        &raw mut c_b8,
                        wv.offset(wv_offset as isize) as *mut doublereal,
                        ldwv,
                    );
                    dlacpy__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut j2,
                        h__.offset((jrow + (incol + 1 as integer) * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                        wv.offset(
                            ((i2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * wv_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldwv,
                    );
                    i__5 = i4 - i2;
                    f2c_dtrmm_0(
                        b"R\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"L\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut i__5,
                        &raw mut c_b8,
                        u.offset(
                            ((i2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * u_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldu,
                        wv.offset(
                            ((i2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * wv_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldwv,
                    );
                    i__5 = i4 - i2;
                    i__7 = j4 - j2;
                    f2c_dgemm_0(
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"N\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut i__5,
                        &raw mut i__7,
                        &raw mut c_b8,
                        h__.offset((jrow + (incol + 1 as integer + j2) * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                        u.offset((j2 + 1 as integer + (i2 + 1 as integer) * u_dim1) as isize)
                            as *mut doublereal,
                        ldu,
                        &raw mut c_b8,
                        wv.offset(
                            ((i2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * wv_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldwv,
                    );
                    dlacpy__0(
                        b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jlen,
                        &raw mut kdu,
                        wv.offset(wv_offset as isize) as *mut doublereal,
                        ldwv,
                        h__.offset((jrow + (incol + 1 as integer) * h_dim1) as isize)
                            as *mut doublereal,
                        ldh,
                    );
                    jrow += i__4 as ::core::ffi::c_long;
                }
                if *wantz != 0 {
                    i__4 = *ihiz;
                    i__3 = *nv;
                    jrow = *iloz;
                    while if i__3 < 0 as ::core::ffi::c_long {
                        (jrow >= i__4) as ::core::ffi::c_int
                    } else {
                        (jrow <= i__4) as ::core::ffi::c_int
                    } != 0
                    {
                        i__5 = *nv;
                        i__7 = (*ihiz - jrow as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            as integer;
                        jlen = (if i__5 <= i__7 {
                            i__5 as ::core::ffi::c_long
                        } else {
                            i__7 as ::core::ffi::c_long
                        }) as integer;
                        dlacpy__0(
                            b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut knz,
                            z__.offset((jrow + (incol + 1 as integer + j2) * z_dim1) as isize)
                                as *mut doublereal,
                            ldz,
                            wv.offset(
                                ((kzs as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    * wv_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldwv,
                        );
                        dlaset__0(
                            b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut kzs,
                            &raw mut c_b7,
                            &raw mut c_b7,
                            wv.offset(wv_offset as isize) as *mut doublereal,
                            ldwv,
                        );
                        f2c_dtrmm_0(
                            b"R\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"U\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut knz,
                            &raw mut c_b8,
                            u.offset((j2 + 1 as integer + (kzs + 1 as integer) * u_dim1) as isize)
                                as *mut doublereal,
                            ldu,
                            wv.offset(
                                ((kzs as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    * wv_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldwv,
                        );
                        f2c_dgemm_0(
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut i2,
                            &raw mut j2,
                            &raw mut c_b8,
                            z__.offset((jrow + (incol + 1 as integer) * z_dim1) as isize)
                                as *mut doublereal,
                            ldz,
                            u.offset(u_offset as isize) as *mut doublereal,
                            ldu,
                            &raw mut c_b8,
                            wv.offset(wv_offset as isize) as *mut doublereal,
                            ldwv,
                        );
                        dlacpy__0(
                            b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut j2,
                            z__.offset((jrow + (incol + 1 as integer) * z_dim1) as isize)
                                as *mut doublereal,
                            ldz,
                            wv.offset(
                                ((i2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    * wv_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldwv,
                        );
                        i__5 = i4 - i2;
                        f2c_dtrmm_0(
                            b"R\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"L\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut i__5,
                            &raw mut c_b8,
                            u.offset(
                                ((i2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    * u_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldu,
                            wv.offset(
                                ((i2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    * wv_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldwv,
                        );
                        i__5 = i4 - i2;
                        i__7 = j4 - j2;
                        f2c_dgemm_0(
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"N\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut i__5,
                            &raw mut i__7,
                            &raw mut c_b8,
                            z__.offset((jrow + (incol + 1 as integer + j2) * z_dim1) as isize)
                                as *mut doublereal,
                            ldz,
                            u.offset((j2 + 1 as integer + (i2 + 1 as integer) * u_dim1) as isize)
                                as *mut doublereal,
                            ldu,
                            &raw mut c_b8,
                            wv.offset(
                                ((i2 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                    * wv_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            ldwv,
                        );
                        dlacpy__0(
                            b"ALL\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut jlen,
                            &raw mut kdu,
                            wv.offset(wv_offset as isize) as *mut doublereal,
                            ldwv,
                            z__.offset((jrow + (incol + 1 as integer) * z_dim1) as isize)
                                as *mut doublereal,
                            ldz,
                        );
                        jrow += i__3 as ::core::ffi::c_long;
                    }
                }
            }
        }
        incol += i__2 as ::core::ffi::c_long;
    }
    return 0 as ::core::ffi::c_int;
}
