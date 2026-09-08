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
static mut c__1: integer = 1 as integer;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
static mut c_true: logical = TRUE_ as logical;
static mut c_b17: doublereal = 0.0f64;
static mut c_b18: doublereal = 1.0f64;
static mut c__12: integer = 12 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlaqr3_(
    mut wantt: *mut logical,
    mut wantz: *mut logical,
    mut n: *mut integer,
    mut ktop: *mut integer,
    mut kbot: *mut integer,
    mut nw: *mut integer,
    mut h__: *mut doublereal,
    mut ldh: *mut integer,
    mut iloz: *mut integer,
    mut ihiz: *mut integer,
    mut z__: *mut doublereal,
    mut ldz: *mut integer,
    mut ns: *mut integer,
    mut nd: *mut integer,
    mut sr: *mut doublereal,
    mut si: *mut doublereal,
    mut v: *mut doublereal,
    mut ldv: *mut integer,
    mut nh: *mut integer,
    mut t: *mut doublereal,
    mut ldt: *mut integer,
    mut nv: *mut integer,
    mut wv: *mut doublereal,
    mut ldwv: *mut integer,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
) -> ::core::ffi::c_int {
    let mut h_dim1: integer = 0;
    let mut h_offset: integer = 0;
    let mut t_dim1: integer = 0;
    let mut t_offset: integer = 0;
    let mut v_dim1: integer = 0;
    let mut v_offset: integer = 0;
    let mut wv_dim1: integer = 0;
    let mut wv_offset: integer = 0;
    let mut z_dim1: integer = 0;
    let mut z_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__4: doublereal = 0.;
    let mut d__5: doublereal = 0.;
    let mut d__6: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut s: doublereal = 0.;
    let mut aa: doublereal = 0.;
    let mut bb: doublereal = 0.;
    let mut cc: doublereal = 0.;
    let mut dd: doublereal = 0.;
    let mut cs: doublereal = 0.;
    let mut sn: doublereal = 0.;
    let mut jw: integer = 0;
    let mut evi: doublereal = 0.;
    let mut evk: doublereal = 0.;
    let mut foo: doublereal = 0.;
    let mut kln: integer = 0;
    let mut tau: doublereal = 0.;
    let mut ulp: doublereal = 0.;
    let mut lwk1: integer = 0;
    let mut lwk2: integer = 0;
    let mut lwk3: integer = 0;
    let mut beta: doublereal = 0.;
    let mut kend: integer = 0;
    let mut kcol: integer = 0;
    let mut info: integer = 0;
    let mut nmin: integer = 0;
    let mut ifst: integer = 0;
    let mut ilst: integer = 0;
    let mut ltop: integer = 0;
    let mut krow: integer = 0;
    extern "C" {
        #[link_name = "dgeev_closure_dlarf_"]
        fn dlarf__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
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
    let mut bulge: logical = 0;
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
    let mut infqr: integer = 0;
    let mut kwtop: integer = 0;
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
        #[link_name = "dgeev_closure_dlaqr4_"]
        fn dlaqr4__0(
            _: *mut logical,
            _: *mut logical,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
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
        #[link_name = "dgeev_closure_dgehrd_"]
        fn dgehrd__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
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
        #[link_name = "dgeev_closure_dlahqr_"]
        fn dlahqr__0(
            _: *mut logical,
            _: *mut logical,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
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
        #[link_name = "dgeev_closure_ilaenv_"]
        fn ilaenv__0(
            _: *mut integer,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
        ) -> integer;
    }
    let mut safmax: doublereal = 0.;
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
    extern "C" {
        #[link_name = "dgeev_closure_dtrexc_"]
        fn dtrexc__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dormhr_"]
        fn dormhr__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut sorted: logical = 0;
    let mut smlnum: doublereal = 0.;
    let mut lwkopt: integer = 0;
    h_dim1 = *ldh;
    h_offset = 1 as integer + h_dim1;
    h__ = h__.offset(-(h_offset as isize));
    z_dim1 = *ldz;
    z_offset = 1 as integer + z_dim1;
    z__ = z__.offset(-(z_offset as isize));
    sr = sr.offset(-1);
    si = si.offset(-1);
    v_dim1 = *ldv;
    v_offset = 1 as integer + v_dim1;
    v = v.offset(-(v_offset as isize));
    t_dim1 = *ldt;
    t_offset = 1 as integer + t_dim1;
    t = t.offset(-(t_offset as isize));
    wv_dim1 = *ldwv;
    wv_offset = 1 as integer + wv_dim1;
    wv = wv.offset(-(wv_offset as isize));
    work = work.offset(-1);
    i__1 = *nw;
    i__2 = (*kbot - *ktop + 1 as ::core::ffi::c_long) as integer;
    jw = (if i__1 <= i__2 {
        i__1 as ::core::ffi::c_long
    } else {
        i__2 as ::core::ffi::c_long
    }) as integer;
    if jw <= 2 as ::core::ffi::c_long {
        lwkopt = 1 as integer;
    } else {
        i__1 = (jw as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        dgehrd__0(
            &raw mut jw,
            &raw mut c__1,
            &raw mut i__1,
            t.offset(t_offset as isize) as *mut doublereal,
            ldt,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c_n1,
            &raw mut info,
        );
        lwk1 = *work.offset(1 as ::core::ffi::c_int as isize) as integer;
        i__1 = (jw as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        dormhr__0(
            b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut jw,
            &raw mut jw,
            &raw mut c__1,
            &raw mut i__1,
            t.offset(t_offset as isize) as *mut doublereal,
            ldt,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            v.offset(v_offset as isize) as *mut doublereal,
            ldv,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c_n1,
            &raw mut info,
        );
        lwk2 = *work.offset(1 as ::core::ffi::c_int as isize) as integer;
        dlaqr4__0(
            &raw mut c_true,
            &raw mut c_true,
            &raw mut jw,
            &raw mut c__1,
            &raw mut jw,
            t.offset(t_offset as isize) as *mut doublereal,
            ldt,
            sr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            si.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            &raw mut jw,
            v.offset(v_offset as isize) as *mut doublereal,
            ldv,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c_n1,
            &raw mut infqr,
        );
        lwk3 = *work.offset(1 as ::core::ffi::c_int as isize) as integer;
        i__1 = (jw as ::core::ffi::c_long
            + (if lwk1 >= lwk2 {
                lwk1 as ::core::ffi::c_long
            } else {
                lwk2 as ::core::ffi::c_long
            })) as integer;
        lwkopt = (if i__1 >= lwk3 {
            i__1 as ::core::ffi::c_long
        } else {
            lwk3 as ::core::ffi::c_long
        }) as integer;
    }
    if *lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
        *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    *ns = 0 as integer;
    *nd = 0 as integer;
    *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
    if *ktop > *kbot {
        return 0 as ::core::ffi::c_int;
    }
    if *nw < 1 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    safmin = dlamch__0(
        b"SAFE MINIMUM\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    safmax = 1.0f64 / safmin;
    dlabad__0(&raw mut safmin, &raw mut safmax);
    ulp = dlamch__0(
        b"PRECISION\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    smlnum = safmin * (*n as doublereal / ulp);
    i__1 = *nw;
    i__2 = (*kbot - *ktop + 1 as ::core::ffi::c_long) as integer;
    jw = (if i__1 <= i__2 {
        i__1 as ::core::ffi::c_long
    } else {
        i__2 as ::core::ffi::c_long
    }) as integer;
    kwtop = (*kbot - jw as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    if kwtop == *ktop {
        s = 0.0f64 as doublereal;
    } else {
        s = *h__.offset((kwtop + (kwtop - 1 as integer) * h_dim1) as isize);
    }
    if *kbot == kwtop {
        *sr.offset(kwtop as isize) = *h__.offset((kwtop + kwtop * h_dim1) as isize);
        *si.offset(kwtop as isize) = 0.0f64 as doublereal;
        *ns = 1 as integer;
        *nd = 0 as integer;
        d__2 = smlnum;
        d__1 = *h__.offset((kwtop + kwtop * h_dim1) as isize);
        d__3 = (ulp as ::core::ffi::c_double
            * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            })) as doublereal;
        if (if s >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            s as ::core::ffi::c_double
        } else {
            -(s as ::core::ffi::c_double)
        }) <= (if d__2 >= d__3 {
            d__2 as ::core::ffi::c_double
        } else {
            d__3 as ::core::ffi::c_double
        }) {
            *ns = 0 as integer;
            *nd = 1 as integer;
            if kwtop > *ktop {
                *h__.offset((kwtop + (kwtop - 1 as integer) * h_dim1) as isize) =
                    0.0f64 as doublereal;
            }
        }
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    dlacpy__0(
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut jw,
        &raw mut jw,
        h__.offset((kwtop + kwtop * h_dim1) as isize) as *mut doublereal,
        ldh,
        t.offset(t_offset as isize) as *mut doublereal,
        ldt,
    );
    i__1 = (jw as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
    i__2 = (*ldh + 1 as ::core::ffi::c_long) as integer;
    i__3 = (*ldt + 1 as ::core::ffi::c_long) as integer;
    f2c_dcopy_0(
        &raw mut i__1,
        h__.offset((kwtop + 1 as integer + kwtop * h_dim1) as isize) as *mut doublereal,
        &raw mut i__2,
        t.offset((t_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
            as *mut doublereal,
        &raw mut i__3,
    );
    dlaset__0(
        b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut jw,
        &raw mut jw,
        &raw mut c_b17,
        &raw mut c_b18,
        v.offset(v_offset as isize) as *mut doublereal,
        ldv,
    );
    nmin = ilaenv__0(
        &raw mut c__12,
        b"DLAQR3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"SV\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut jw,
        &raw mut c__1,
        &raw mut jw,
        lwork,
    );
    if jw > nmin {
        dlaqr4__0(
            &raw mut c_true,
            &raw mut c_true,
            &raw mut jw,
            &raw mut c__1,
            &raw mut jw,
            t.offset(t_offset as isize) as *mut doublereal,
            ldt,
            sr.offset(kwtop as isize) as *mut doublereal,
            si.offset(kwtop as isize) as *mut doublereal,
            &raw mut c__1,
            &raw mut jw,
            v.offset(v_offset as isize) as *mut doublereal,
            ldv,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            lwork,
            &raw mut infqr,
        );
    } else {
        dlahqr__0(
            &raw mut c_true,
            &raw mut c_true,
            &raw mut jw,
            &raw mut c__1,
            &raw mut jw,
            t.offset(t_offset as isize) as *mut doublereal,
            ldt,
            sr.offset(kwtop as isize) as *mut doublereal,
            si.offset(kwtop as isize) as *mut doublereal,
            &raw mut c__1,
            &raw mut jw,
            v.offset(v_offset as isize) as *mut doublereal,
            ldv,
            &raw mut infqr,
        );
    }
    i__1 = (jw as ::core::ffi::c_long - 3 as ::core::ffi::c_long) as integer;
    j = 1 as integer;
    while j <= i__1 {
        *t.offset((j + 2 as integer + j * t_dim1) as isize) = 0.0f64 as doublereal;
        *t.offset((j + 3 as integer + j * t_dim1) as isize) = 0.0f64 as doublereal;
        j += 1;
    }
    if jw > 2 as ::core::ffi::c_long {
        *t.offset((jw + (jw - 2 as integer) * t_dim1) as isize) = 0.0f64 as doublereal;
    }
    *ns = jw;
    ilst = (infqr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    while ilst <= *ns {
        if *ns == 1 as ::core::ffi::c_long {
            bulge = FALSE_ as logical;
        } else {
            bulge = (*t.offset((*ns + (*ns - 1 as integer) * t_dim1) as isize) != 0.0f64)
                as ::core::ffi::c_int as logical;
        }
        if bulge == 0 {
            d__1 = *t.offset((*ns + *ns * t_dim1) as isize);
            foo = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            if foo == 0.0f64 {
                foo = (if s >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    s as ::core::ffi::c_double
                } else {
                    -(s as ::core::ffi::c_double)
                }) as doublereal;
            }
            d__2 = smlnum;
            d__3 = ulp * foo;
            d__1 = s * *v
                .offset((*ns * v_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) <= (if d__2 >= d__3 {
                d__2 as ::core::ffi::c_double
            } else {
                d__3 as ::core::ffi::c_double
            }) {
                *ns -= 1;
            } else {
                ifst = *ns;
                dtrexc__0(
                    b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut jw,
                    t.offset(t_offset as isize) as *mut doublereal,
                    ldt,
                    v.offset(v_offset as isize) as *mut doublereal,
                    ldv,
                    &raw mut ifst,
                    &raw mut ilst,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut info,
                );
                ilst += 1;
            }
        } else {
            d__3 = *t.offset((*ns + *ns * t_dim1) as isize);
            d__1 = *t.offset((*ns + (*ns - 1 as integer) * t_dim1) as isize);
            d__2 = *t.offset((*ns - 1 as integer + *ns * t_dim1) as isize);
            foo = ((if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__3 as ::core::ffi::c_double
            } else {
                -(d__3 as ::core::ffi::c_double)
            }) + sqrt(
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
            if foo == 0.0f64 {
                foo = (if s >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    s as ::core::ffi::c_double
                } else {
                    -(s as ::core::ffi::c_double)
                }) as doublereal;
            }
            d__1 = s * *v
                .offset((*ns * v_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
            d__2 = s * *v.offset(
                ((*ns - 1 as ::core::ffi::c_long) * v_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            );
            d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__2 as ::core::ffi::c_double
            } else {
                -(d__2 as ::core::ffi::c_double)
            }) as doublereal;
            d__5 = smlnum;
            d__6 = ulp * foo;
            if (if d__3 >= d__4 {
                d__3 as ::core::ffi::c_double
            } else {
                d__4 as ::core::ffi::c_double
            }) <= (if d__5 >= d__6 {
                d__5 as ::core::ffi::c_double
            } else {
                d__6 as ::core::ffi::c_double
            }) {
                *ns += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
            } else {
                ifst = *ns;
                dtrexc__0(
                    b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut jw,
                    t.offset(t_offset as isize) as *mut doublereal,
                    ldt,
                    v.offset(v_offset as isize) as *mut doublereal,
                    ldv,
                    &raw mut ifst,
                    &raw mut ilst,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut info,
                );
                ilst += 2 as ::core::ffi::c_long;
            }
        }
    }
    if *ns == 0 as ::core::ffi::c_long {
        s = 0.0f64 as doublereal;
    }
    if *ns < jw {
        sorted = FALSE_ as logical;
        i__ = (*ns + 1 as ::core::ffi::c_long) as integer;
        while !(sorted != 0) {
            sorted = TRUE_ as logical;
            kend = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            i__ = (infqr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            if i__ == *ns {
                k = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            } else if *t.offset((i__ + 1 as integer + i__ * t_dim1) as isize) == 0.0f64 {
                k = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            } else {
                k = (i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
            }
            while k <= kend {
                if k == i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long {
                    d__1 = *t.offset((i__ + i__ * t_dim1) as isize);
                    evi = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                } else {
                    d__3 = *t.offset((i__ + i__ * t_dim1) as isize);
                    d__1 = *t.offset((i__ + 1 as integer + i__ * t_dim1) as isize);
                    d__2 = *t.offset((i__ + (i__ + 1 as integer) * t_dim1) as isize);
                    evi = ((if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__3 as ::core::ffi::c_double
                    } else {
                        -(d__3 as ::core::ffi::c_double)
                    }) + sqrt(
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
                if k == kend {
                    d__1 = *t.offset((k + k * t_dim1) as isize);
                    evk = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                } else if *t.offset((k + 1 as integer + k * t_dim1) as isize) == 0.0f64 {
                    d__1 = *t.offset((k + k * t_dim1) as isize);
                    evk = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                } else {
                    d__3 = *t.offset((k + k * t_dim1) as isize);
                    d__1 = *t.offset((k + 1 as integer + k * t_dim1) as isize);
                    d__2 = *t.offset((k + (k + 1 as integer) * t_dim1) as isize);
                    evk = ((if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__3 as ::core::ffi::c_double
                    } else {
                        -(d__3 as ::core::ffi::c_double)
                    }) + sqrt(
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
                if evi >= evk {
                    i__ = k;
                } else {
                    sorted = FALSE_ as logical;
                    ifst = i__;
                    ilst = k;
                    dtrexc__0(
                        b"V\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut jw,
                        t.offset(t_offset as isize) as *mut doublereal,
                        ldt,
                        v.offset(v_offset as isize) as *mut doublereal,
                        ldv,
                        &raw mut ifst,
                        &raw mut ilst,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        &raw mut info,
                    );
                    if info == 0 as ::core::ffi::c_long {
                        i__ = ilst;
                    } else {
                        i__ = k;
                    }
                }
                if i__ == kend {
                    k = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                } else if *t.offset((i__ + 1 as integer + i__ * t_dim1) as isize) == 0.0f64 {
                    k = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                } else {
                    k = (i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
                }
            }
        }
    }
    i__ = jw;
    while i__ >= infqr as ::core::ffi::c_long + 1 as ::core::ffi::c_long {
        if i__ == infqr as ::core::ffi::c_long + 1 as ::core::ffi::c_long {
            *sr.offset(
                (kwtop as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as isize,
            ) = *t.offset((i__ + i__ * t_dim1) as isize);
            *si.offset(
                (kwtop as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as isize,
            ) = 0.0f64 as doublereal;
            i__ -= 1;
        } else if *t.offset((i__ + (i__ - 1 as integer) * t_dim1) as isize) == 0.0f64 {
            *sr.offset(
                (kwtop as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as isize,
            ) = *t.offset((i__ + i__ * t_dim1) as isize);
            *si.offset(
                (kwtop as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long) as isize,
            ) = 0.0f64 as doublereal;
            i__ -= 1;
        } else {
            aa = *t.offset((i__ - 1 as integer + (i__ - 1 as integer) * t_dim1) as isize);
            cc = *t.offset((i__ + (i__ - 1 as integer) * t_dim1) as isize);
            bb = *t.offset((i__ - 1 as integer + i__ * t_dim1) as isize);
            dd = *t.offset((i__ + i__ * t_dim1) as isize);
            dlanv2__0(
                &raw mut aa,
                &raw mut bb,
                &raw mut cc,
                &raw mut dd,
                sr.offset(
                    (kwtop as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                        - 2 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                si.offset(
                    (kwtop as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                        - 2 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                sr.offset(
                    (kwtop as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                si.offset(
                    (kwtop as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                        - 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut cs,
                &raw mut sn,
            );
            i__ += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
        }
    }
    if *ns < jw || s == 0.0f64 {
        if *ns > 1 as ::core::ffi::c_long && s != 0.0f64 {
            f2c_dcopy_0(
                ns,
                v.offset(v_offset as isize) as *mut doublereal,
                ldv,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            );
            beta = *work.offset(1 as ::core::ffi::c_int as isize);
            dlarfg__0(
                ns,
                &raw mut beta,
                work.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut tau,
            );
            *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
            i__1 = (jw as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as integer;
            i__2 = (jw as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as integer;
            dlaset__0(
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
                &raw mut i__2,
                &raw mut c_b17,
                &raw mut c_b17,
                t.offset((t_dim1 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldt,
            );
            dlarf__0(
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ns,
                &raw mut jw,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut tau,
                t.offset(t_offset as isize) as *mut doublereal,
                ldt,
                work.offset((jw as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
            );
            dlarf__0(
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ns,
                ns,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut tau,
                t.offset(t_offset as isize) as *mut doublereal,
                ldt,
                work.offset((jw as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
            );
            dlarf__0(
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut jw,
                ns,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut tau,
                v.offset(v_offset as isize) as *mut doublereal,
                ldv,
                work.offset((jw as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
            );
            i__1 = *lwork - jw;
            dgehrd__0(
                &raw mut jw,
                &raw mut c__1,
                ns,
                t.offset(t_offset as isize) as *mut doublereal,
                ldt,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset((jw as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut i__1,
                &raw mut info,
            );
        }
        if kwtop > 1 as ::core::ffi::c_long {
            *h__.offset((kwtop + (kwtop - 1 as integer) * h_dim1) as isize) =
                s * *v.offset((v_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
        }
        dlacpy__0(
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut jw,
            &raw mut jw,
            t.offset(t_offset as isize) as *mut doublereal,
            ldt,
            h__.offset((kwtop + kwtop * h_dim1) as isize) as *mut doublereal,
            ldh,
        );
        i__1 = (jw as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        i__2 = (*ldt + 1 as ::core::ffi::c_long) as integer;
        i__3 = (*ldh + 1 as ::core::ffi::c_long) as integer;
        f2c_dcopy_0(
            &raw mut i__1,
            t.offset((t_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            &raw mut i__2,
            h__.offset((kwtop + 1 as integer + kwtop * h_dim1) as isize) as *mut doublereal,
            &raw mut i__3,
        );
        if *ns > 1 as ::core::ffi::c_long && s != 0.0f64 {
            i__1 = *lwork - jw;
            dormhr__0(
                b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut jw,
                ns,
                &raw mut c__1,
                ns,
                t.offset(t_offset as isize) as *mut doublereal,
                ldt,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                v.offset(v_offset as isize) as *mut doublereal,
                ldv,
                work.offset((jw as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut i__1,
                &raw mut info,
            );
        }
        if *wantt != 0 {
            ltop = 1 as integer;
        } else {
            ltop = *ktop;
        }
        i__1 = (kwtop as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        i__2 = *nv;
        krow = ltop;
        while if i__2 < 0 as ::core::ffi::c_long {
            (krow >= i__1) as ::core::ffi::c_int
        } else {
            (krow <= i__1) as ::core::ffi::c_int
        } != 0
        {
            i__3 = *nv;
            i__4 = kwtop - krow;
            kln = (if i__3 <= i__4 {
                i__3 as ::core::ffi::c_long
            } else {
                i__4 as ::core::ffi::c_long
            }) as integer;
            f2c_dgemm_0(
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut kln,
                &raw mut jw,
                &raw mut jw,
                &raw mut c_b18,
                h__.offset((krow + kwtop * h_dim1) as isize) as *mut doublereal,
                ldh,
                v.offset(v_offset as isize) as *mut doublereal,
                ldv,
                &raw mut c_b17,
                wv.offset(wv_offset as isize) as *mut doublereal,
                ldwv,
            );
            dlacpy__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut kln,
                &raw mut jw,
                wv.offset(wv_offset as isize) as *mut doublereal,
                ldwv,
                h__.offset((krow + kwtop * h_dim1) as isize) as *mut doublereal,
                ldh,
            );
            krow += i__2 as ::core::ffi::c_long;
        }
        if *wantt != 0 {
            i__2 = *n;
            i__1 = *nh;
            kcol = (*kbot + 1 as ::core::ffi::c_long) as integer;
            while if i__1 < 0 as ::core::ffi::c_long {
                (kcol >= i__2) as ::core::ffi::c_int
            } else {
                (kcol <= i__2) as ::core::ffi::c_int
            } != 0
            {
                i__3 = *nh;
                i__4 = (*n - kcol as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                kln = (if i__3 <= i__4 {
                    i__3 as ::core::ffi::c_long
                } else {
                    i__4 as ::core::ffi::c_long
                }) as integer;
                f2c_dgemm_0(
                    b"C\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut jw,
                    &raw mut kln,
                    &raw mut jw,
                    &raw mut c_b18,
                    v.offset(v_offset as isize) as *mut doublereal,
                    ldv,
                    h__.offset((kwtop + kcol * h_dim1) as isize) as *mut doublereal,
                    ldh,
                    &raw mut c_b17,
                    t.offset(t_offset as isize) as *mut doublereal,
                    ldt,
                );
                dlacpy__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut jw,
                    &raw mut kln,
                    t.offset(t_offset as isize) as *mut doublereal,
                    ldt,
                    h__.offset((kwtop + kcol * h_dim1) as isize) as *mut doublereal,
                    ldh,
                );
                kcol += i__1 as ::core::ffi::c_long;
            }
        }
        if *wantz != 0 {
            i__1 = *ihiz;
            i__2 = *nv;
            krow = *iloz;
            while if i__2 < 0 as ::core::ffi::c_long {
                (krow >= i__1) as ::core::ffi::c_int
            } else {
                (krow <= i__1) as ::core::ffi::c_int
            } != 0
            {
                i__3 = *nv;
                i__4 = (*ihiz - krow as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                kln = (if i__3 <= i__4 {
                    i__3 as ::core::ffi::c_long
                } else {
                    i__4 as ::core::ffi::c_long
                }) as integer;
                f2c_dgemm_0(
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut kln,
                    &raw mut jw,
                    &raw mut jw,
                    &raw mut c_b18,
                    z__.offset((krow + kwtop * z_dim1) as isize) as *mut doublereal,
                    ldz,
                    v.offset(v_offset as isize) as *mut doublereal,
                    ldv,
                    &raw mut c_b17,
                    wv.offset(wv_offset as isize) as *mut doublereal,
                    ldwv,
                );
                dlacpy__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut kln,
                    &raw mut jw,
                    wv.offset(wv_offset as isize) as *mut doublereal,
                    ldwv,
                    z__.offset((krow + kwtop * z_dim1) as isize) as *mut doublereal,
                    ldz,
                );
                krow += i__2 as ::core::ffi::c_long;
            }
        }
    }
    *nd = jw - *ns;
    *ns -= infqr as ::core::ffi::c_long;
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    return 0 as ::core::ffi::c_int;
}
