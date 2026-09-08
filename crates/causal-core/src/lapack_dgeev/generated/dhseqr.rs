#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type integer = ::core::ffi::c_long;
pub type address = *mut ::core::ffi::c_char;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
pub type ftnlen = ::core::ffi::c_long;
static mut c_b11: doublereal = 0.0f64;
static mut c_b12: doublereal = 1.0f64;
static mut c__12: integer = 12 as integer;
static mut c__2: integer = 2 as integer;
static mut c__49: integer = 49 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dhseqr_(
    mut job: *mut ::core::ffi::c_char,
    mut compz: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut ilo: *mut integer,
    mut ihi: *mut integer,
    mut h__: *mut doublereal,
    mut ldh: *mut integer,
    mut wr: *mut doublereal,
    mut wi: *mut doublereal,
    mut z__: *mut doublereal,
    mut ldz: *mut integer,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a__1: [address; 2] = [::core::ptr::null_mut::<::core::ffi::c_char>(); 2];
    let mut h_dim1: integer = 0;
    let mut h_offset: integer = 0;
    let mut z_dim1: integer = 0;
    let mut z_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: [integer; 2] = [0; 2];
    let mut i__3: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut ch__1: [::core::ffi::c_char; 2] = [0; 2];
    extern "C" {
        #[link_name = "dgeev_closure_s_cat"]
        fn s_cat_0(
            _: *mut ::core::ffi::c_char,
            _: *mut *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: ftnlen,
        ) -> ::core::ffi::c_int;
    }
    let mut i__: integer = 0;
    let mut hl: [doublereal; 2401] = [0.; 2401];
    let mut kbot: integer = 0;
    let mut nmin: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut initz: logical = 0;
    let mut workl: [doublereal; 49] = [0.; 49];
    let mut wantt: logical = 0;
    let mut wantz: logical = 0;
    extern "C" {
        #[link_name = "dgeev_closure_dlaqr0_"]
        fn dlaqr0__0(
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
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut lquery: logical = 0;
    h_dim1 = *ldh;
    h_offset = 1 as integer + h_dim1;
    h__ = h__.offset(-(h_offset as isize));
    wr = wr.offset(-1);
    wi = wi.offset(-1);
    z_dim1 = *ldz;
    z_offset = 1 as integer + z_dim1;
    z__ = z__.offset(-(z_offset as isize));
    work = work.offset(-1);
    wantt = lsame__0(
        job,
        b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    initz = lsame__0(
        compz,
        b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    wantz = (initz != 0
        || lsame__0(
            compz,
            b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0) as ::core::ffi::c_int as logical;
    *work.offset(1 as ::core::ffi::c_int as isize) = (if 1 as ::core::ffi::c_long >= *n {
        1 as ::core::ffi::c_long
    } else {
        *n
    }) as doublereal;
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
    *info = 0 as integer;
    if lsame__0(
        job,
        b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && wantt == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if lsame__0(
        compz,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && wantz == 0
    {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *ilo < 1 as ::core::ffi::c_long
        || *ilo
            > (if 1 as ::core::ffi::c_long >= *n {
                1 as ::core::ffi::c_long
            } else {
                *n
            })
    {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *ihi < (if *ilo <= *n { *ilo } else { *n }) || *ihi > *n {
        *info = -(5 as ::core::ffi::c_int) as integer;
    } else if *ldh
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(7 as ::core::ffi::c_int) as integer;
    } else if *ldz < 1 as ::core::ffi::c_long
        || wantz != 0
            && *ldz
                < (if 1 as ::core::ffi::c_long >= *n {
                    1 as ::core::ffi::c_long
                } else {
                    *n
                })
    {
        *info = -(11 as ::core::ffi::c_int) as integer;
    } else if *lwork
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
        && lquery == 0
    {
        *info = -(13 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DHSEQR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        dlaqr0__0(
            &raw mut wantt,
            &raw mut wantz,
            n,
            ilo,
            ihi,
            h__.offset(h_offset as isize) as *mut doublereal,
            ldh,
            wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            ilo,
            ihi,
            z__.offset(z_offset as isize) as *mut doublereal,
            ldz,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            lwork,
            info,
        );
        d__1 = (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        }) as doublereal;
        *work.offset(1 as ::core::ffi::c_int as isize) =
            (if d__1 >= *work.offset(1 as ::core::ffi::c_int as isize) {
                d__1 as ::core::ffi::c_double
            } else {
                *work.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            }) as doublereal;
        return 0 as ::core::ffi::c_int;
    } else {
        i__1 = (*ilo - 1 as ::core::ffi::c_long) as integer;
        i__ = 1 as integer;
        while i__ <= i__1 {
            *wr.offset(i__ as isize) = *h__.offset((i__ + i__ * h_dim1) as isize);
            *wi.offset(i__ as isize) = 0.0f64 as doublereal;
            i__ += 1;
        }
        i__1 = *n;
        i__ = (*ihi + 1 as ::core::ffi::c_long) as integer;
        while i__ <= i__1 {
            *wr.offset(i__ as isize) = *h__.offset((i__ + i__ * h_dim1) as isize);
            *wi.offset(i__ as isize) = 0.0f64 as doublereal;
            i__ += 1;
        }
        if initz != 0 {
            dlaset__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b11,
                &raw mut c_b12,
                z__.offset(z_offset as isize) as *mut doublereal,
                ldz,
            );
        }
        if *ilo == *ihi {
            *wr.offset(*ilo as isize) = *h__.offset((*ilo + *ilo * h_dim1) as isize);
            *wi.offset(*ilo as isize) = 0.0f64 as doublereal;
            return 0 as ::core::ffi::c_int;
        }
        i__2[0 as ::core::ffi::c_int as usize] = 1 as integer;
        a__1[0 as ::core::ffi::c_int as usize] = job as address;
        i__2[1 as ::core::ffi::c_int as usize] = 1 as integer;
        a__1[1 as ::core::ffi::c_int as usize] = compz as address;
        s_cat_0(
            &raw mut ch__1 as *mut ::core::ffi::c_char,
            &raw mut a__1 as *mut *mut ::core::ffi::c_char,
            &raw mut i__2 as *mut integer,
            &raw mut c__2,
            2 as ::core::ffi::c_int as ftnlen,
        );
        nmin = ilaenv__0(
            &raw mut c__12,
            b"DHSEQR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut ch__1 as *mut ::core::ffi::c_char,
            n,
            ilo,
            ihi,
            lwork,
        );
        nmin = (if 11 as ::core::ffi::c_long >= nmin {
            11 as ::core::ffi::c_long
        } else {
            nmin as ::core::ffi::c_long
        }) as integer;
        if *n > nmin {
            dlaqr0__0(
                &raw mut wantt,
                &raw mut wantz,
                n,
                ilo,
                ihi,
                h__.offset(h_offset as isize) as *mut doublereal,
                ldh,
                wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                ilo,
                ihi,
                z__.offset(z_offset as isize) as *mut doublereal,
                ldz,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                lwork,
                info,
            );
        } else {
            dlahqr__0(
                &raw mut wantt,
                &raw mut wantz,
                n,
                ilo,
                ihi,
                h__.offset(h_offset as isize) as *mut doublereal,
                ldh,
                wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                ilo,
                ihi,
                z__.offset(z_offset as isize) as *mut doublereal,
                ldz,
                info,
            );
            if *info > 0 as ::core::ffi::c_long {
                kbot = *info;
                if *n >= 49 as ::core::ffi::c_long {
                    dlaqr0__0(
                        &raw mut wantt,
                        &raw mut wantz,
                        n,
                        ilo,
                        &raw mut kbot,
                        h__.offset(h_offset as isize) as *mut doublereal,
                        ldh,
                        wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        ilo,
                        ihi,
                        z__.offset(z_offset as isize) as *mut doublereal,
                        ldz,
                        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        lwork,
                        info,
                    );
                } else {
                    dlacpy__0(
                        b"A\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        n,
                        n,
                        h__.offset(h_offset as isize) as *mut doublereal,
                        ldh,
                        &raw mut hl as *mut doublereal,
                        &raw mut c__49,
                    );
                    hl[(*n + 1 as ::core::ffi::c_long + *n * 49 as ::core::ffi::c_long
                        - 50 as ::core::ffi::c_long) as usize] = 0.0f64 as doublereal;
                    i__1 = 49 as integer - *n;
                    dlaset__0(
                        b"A\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut c__49,
                        &raw mut i__1,
                        &raw mut c_b11,
                        &raw mut c_b11,
                        (&raw mut hl as *mut doublereal).offset(
                            ((*n + 1 as ::core::ffi::c_long) * 49 as ::core::ffi::c_long
                                - 49 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__49,
                    );
                    dlaqr0__0(
                        &raw mut wantt,
                        &raw mut wantz,
                        &raw mut c__49,
                        ilo,
                        &raw mut kbot,
                        &raw mut hl as *mut doublereal,
                        &raw mut c__49,
                        wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        ilo,
                        ihi,
                        z__.offset(z_offset as isize) as *mut doublereal,
                        ldz,
                        &raw mut workl as *mut doublereal,
                        &raw mut c__49,
                        info,
                    );
                    if wantt != 0 || *info != 0 as ::core::ffi::c_long {
                        dlacpy__0(
                            b"A\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            &raw mut hl as *mut doublereal,
                            &raw mut c__49,
                            h__.offset(h_offset as isize) as *mut doublereal,
                            ldh,
                        );
                    }
                }
            }
        }
        if (wantt != 0 || *info != 0 as ::core::ffi::c_long) && *n > 2 as ::core::ffi::c_long {
            i__1 = (*n - 2 as ::core::ffi::c_long) as integer;
            i__3 = (*n - 2 as ::core::ffi::c_long) as integer;
            dlaset__0(
                b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__1,
                &raw mut i__3,
                &raw mut c_b11,
                &raw mut c_b11,
                h__.offset((h_dim1 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldh,
            );
        }
        d__1 = (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        }) as doublereal;
        *work.offset(1 as ::core::ffi::c_int as isize) =
            (if d__1 >= *work.offset(1 as ::core::ffi::c_int as isize) {
                d__1 as ::core::ffi::c_double
            } else {
                *work.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            }) as doublereal;
    }
    return 0 as ::core::ffi::c_int;
}
