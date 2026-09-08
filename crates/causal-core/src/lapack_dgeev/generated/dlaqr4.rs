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
static mut c__13: integer = 13 as integer;
static mut c__15: integer = 15 as integer;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
static mut c__12: integer = 12 as integer;
static mut c__14: integer = 14 as integer;
static mut c__16: integer = 16 as integer;
static mut c_false: logical = FALSE_ as logical;
static mut c__1: integer = 1 as integer;
static mut c__3: integer = 3 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlaqr4_(
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
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut h_dim1: integer = 0;
    let mut h_offset: integer = 0;
    let mut z_dim1: integer = 0;
    let mut z_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__4: doublereal = 0.;
    let mut i__: integer = 0;
    let mut k: integer = 0;
    let mut aa: doublereal = 0.;
    let mut bb: doublereal = 0.;
    let mut cc: doublereal = 0.;
    let mut dd: doublereal = 0.;
    let mut ld: integer = 0;
    let mut cs: doublereal = 0.;
    let mut nh: integer = 0;
    let mut it: integer = 0;
    let mut ks: integer = 0;
    let mut kt: integer = 0;
    let mut sn: doublereal = 0.;
    let mut ku: integer = 0;
    let mut kv: integer = 0;
    let mut ls: integer = 0;
    let mut ns: integer = 0;
    let mut ss: doublereal = 0.;
    let mut nw: integer = 0;
    let mut inf: integer = 0;
    let mut kdu: integer = 0;
    let mut nho: integer = 0;
    let mut nve: integer = 0;
    let mut kwh: integer = 0;
    let mut nsr: integer = 0;
    let mut nwr: integer = 0;
    let mut kwv: integer = 0;
    let mut ndec: integer = 0;
    let mut ndfl: integer = 0;
    let mut kbot: integer = 0;
    let mut nmin: integer = 0;
    let mut swap: doublereal = 0.;
    let mut ktop: integer = 0;
    let mut zdum: [doublereal; 1] = [0.; 1];
    let mut kacc22: integer = 0;
    let mut itmax: integer = 0;
    let mut nsmax: integer = 0;
    let mut nwmax: integer = 0;
    let mut kwtop: integer = 0;
    extern "C" {
        #[link_name = "dgeev_closure_dlaqr2_"]
        fn dlaqr2__0(
            _: *mut logical,
            _: *mut logical,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
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
        #[link_name = "dgeev_closure_dlaqr5_"]
        fn dlaqr5__0(
            _: *mut logical,
            _: *mut logical,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut nibble: integer = 0;
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
    let mut jbcmpz: [::core::ffi::c_char; 1] = [0; 1];
    let mut nwupbd: integer = 0;
    let mut sorted: logical = 0;
    let mut lwkopt: integer = 0;
    h_dim1 = *ldh;
    h_offset = 1 as integer + h_dim1;
    h__ = h__.offset(-(h_offset as isize));
    wr = wr.offset(-1);
    wi = wi.offset(-1);
    z_dim1 = *ldz;
    z_offset = 1 as integer + z_dim1;
    z__ = z__.offset(-(z_offset as isize));
    work = work.offset(-1);
    *info = 0 as integer;
    if *n == 0 as ::core::ffi::c_long {
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    if *n <= 11 as ::core::ffi::c_long {
        lwkopt = 1 as integer;
        if *lwork != -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
            dlahqr__0(
                wantt,
                wantz,
                n,
                ilo,
                ihi,
                h__.offset(h_offset as isize) as *mut doublereal,
                ldh,
                wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                iloz,
                ihiz,
                z__.offset(z_offset as isize) as *mut doublereal,
                ldz,
                info,
            );
        }
    } else {
        let mut current_block_208: u64;
        *info = 0 as integer;
        if *wantt != 0 {
            *(&raw mut jbcmpz as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                'S' as i32 as ::core::ffi::c_uchar;
        } else {
            *(&raw mut jbcmpz as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                'E' as i32 as ::core::ffi::c_uchar;
        }
        if *wantz != 0 {
            *((&raw mut jbcmpz as *mut ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                'V' as i32 as ::core::ffi::c_uchar;
        } else {
            *((&raw mut jbcmpz as *mut ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                'N' as i32 as ::core::ffi::c_uchar;
        }
        nwr = ilaenv__0(
            &raw mut c__13,
            b"DLAQR4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut jbcmpz as *mut ::core::ffi::c_char,
            n,
            ilo,
            ihi,
            lwork,
        );
        nwr = (if 2 as ::core::ffi::c_long >= nwr {
            2 as ::core::ffi::c_long
        } else {
            nwr as ::core::ffi::c_long
        }) as integer;
        i__1 = (*ihi - *ilo + 1 as ::core::ffi::c_long) as integer;
        i__2 = ((*n - 1 as ::core::ffi::c_long) / 3 as ::core::ffi::c_long) as integer;
        i__1 = (if i__1 <= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        nwr = (if i__1 <= nwr {
            i__1 as ::core::ffi::c_long
        } else {
            nwr as ::core::ffi::c_long
        }) as integer;
        nsr = ilaenv__0(
            &raw mut c__15,
            b"DLAQR4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut jbcmpz as *mut ::core::ffi::c_char,
            n,
            ilo,
            ihi,
            lwork,
        );
        i__1 = nsr;
        i__2 = ((*n + 6 as ::core::ffi::c_long) / 9 as ::core::ffi::c_long) as integer;
        i__1 = (if i__1 <= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        i__2 = *ihi - *ilo;
        nsr = (if i__1 <= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        i__1 = 2 as integer;
        i__2 = (nsr as ::core::ffi::c_long - nsr as ::core::ffi::c_long % 2 as ::core::ffi::c_long)
            as integer;
        nsr = (if i__1 >= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        i__1 = (nwr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dlaqr2__0(
            wantt,
            wantz,
            n,
            ilo,
            ihi,
            &raw mut i__1,
            h__.offset(h_offset as isize) as *mut doublereal,
            ldh,
            iloz,
            ihiz,
            z__.offset(z_offset as isize) as *mut doublereal,
            ldz,
            &raw mut ls,
            &raw mut ld,
            wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            h__.offset(h_offset as isize) as *mut doublereal,
            ldh,
            n,
            h__.offset(h_offset as isize) as *mut doublereal,
            ldh,
            n,
            h__.offset(h_offset as isize) as *mut doublereal,
            ldh,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c_n1,
        );
        i__1 = (nsr as ::core::ffi::c_long * 3 as ::core::ffi::c_long / 2 as ::core::ffi::c_long)
            as integer;
        i__2 = *work.offset(1 as ::core::ffi::c_int as isize) as integer;
        lwkopt = (if i__1 >= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        if *lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
            *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
            return 0 as ::core::ffi::c_int;
        }
        nmin = ilaenv__0(
            &raw mut c__12,
            b"DLAQR4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut jbcmpz as *mut ::core::ffi::c_char,
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
        nibble = ilaenv__0(
            &raw mut c__14,
            b"DLAQR4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut jbcmpz as *mut ::core::ffi::c_char,
            n,
            ilo,
            ihi,
            lwork,
        );
        nibble = (if 0 as ::core::ffi::c_long >= nibble {
            0 as ::core::ffi::c_long
        } else {
            nibble as ::core::ffi::c_long
        }) as integer;
        kacc22 = ilaenv__0(
            &raw mut c__16,
            b"DLAQR4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut jbcmpz as *mut ::core::ffi::c_char,
            n,
            ilo,
            ihi,
            lwork,
        );
        kacc22 = (if 0 as ::core::ffi::c_long >= kacc22 {
            0 as ::core::ffi::c_long
        } else {
            kacc22 as ::core::ffi::c_long
        }) as integer;
        kacc22 = (if 2 as ::core::ffi::c_long <= kacc22 {
            2 as ::core::ffi::c_long
        } else {
            kacc22 as ::core::ffi::c_long
        }) as integer;
        i__1 = ((*n - 1 as ::core::ffi::c_long) / 3 as ::core::ffi::c_long) as integer;
        i__2 = (*lwork / 2 as ::core::ffi::c_long) as integer;
        nwmax = (if i__1 <= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        nw = nwmax;
        i__1 = ((*n + 6 as ::core::ffi::c_long) / 9 as ::core::ffi::c_long) as integer;
        i__2 = ((*lwork << 1 as ::core::ffi::c_int) / 3 as ::core::ffi::c_long) as integer;
        nsmax = (if i__1 <= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        nsmax -= nsmax as ::core::ffi::c_long % 2 as ::core::ffi::c_long;
        ndfl = 1 as integer;
        i__1 = 10 as integer;
        i__2 = (*ihi - *ilo + 1 as ::core::ffi::c_long) as integer;
        itmax = ((if i__1 >= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) * 30 as ::core::ffi::c_long) as integer;
        kbot = *ihi;
        i__1 = itmax;
        it = 1 as integer;
        loop {
            if !(it <= i__1) {
                current_block_208 = 1918110639124887667;
                break;
            }
            if kbot < *ilo {
                current_block_208 = 2942604368452602584;
                break;
            }
            i__2 = (*ilo + 1 as ::core::ffi::c_long) as integer;
            k = kbot;
            loop {
                if !(k >= i__2) {
                    current_block_208 = 11441799814184323368;
                    break;
                }
                if *h__.offset((k + (k - 1 as integer) * h_dim1) as isize) == 0.0f64 {
                    current_block_208 = 1526947759412023129;
                    break;
                }
                k -= 1;
            }
            match current_block_208 {
                11441799814184323368 => {
                    k = *ilo;
                }
                _ => {}
            }
            ktop = k;
            nh = (kbot as ::core::ffi::c_long - ktop as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            nwupbd = (if nh <= nwmax {
                nh as ::core::ffi::c_long
            } else {
                nwmax as ::core::ffi::c_long
            }) as integer;
            if ndfl < 5 as ::core::ffi::c_long {
                nw = (if nwupbd <= nwr {
                    nwupbd as ::core::ffi::c_long
                } else {
                    nwr as ::core::ffi::c_long
                }) as integer;
            } else {
                i__2 = nwupbd;
                i__3 = nw << 1 as ::core::ffi::c_int;
                nw = (if i__2 <= i__3 {
                    i__2 as ::core::ffi::c_long
                } else {
                    i__3 as ::core::ffi::c_long
                }) as integer;
            }
            if nw < nwmax {
                if nw >= nh as ::core::ffi::c_long - 1 as ::core::ffi::c_long {
                    nw = nh;
                } else {
                    kwtop = (kbot as ::core::ffi::c_long - nw as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                    d__1 = *h__.offset((kwtop + (kwtop - 1 as integer) * h_dim1) as isize);
                    d__2 = *h__
                        .offset((kwtop - 1 as integer + (kwtop - 2 as integer) * h_dim1) as isize);
                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) > (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__2 as ::core::ffi::c_double
                    } else {
                        -(d__2 as ::core::ffi::c_double)
                    }) {
                        nw += 1;
                    }
                }
            }
            if ndfl < 5 as ::core::ffi::c_long {
                ndec = -(1 as ::core::ffi::c_int) as integer;
            } else if ndec >= 0 as ::core::ffi::c_long || nw >= nwupbd {
                ndec += 1;
                if nw - ndec < 2 as ::core::ffi::c_long {
                    ndec = 0 as integer;
                }
                nw -= ndec as ::core::ffi::c_long;
            }
            kv = (*n - nw as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            kt = (nw as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            nho = (*n
                - nw as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long
                - kt as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            kwv = (nw as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
            nve = (*n - nw as ::core::ffi::c_long - kwv as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            dlaqr2__0(
                wantt,
                wantz,
                n,
                &raw mut ktop,
                &raw mut kbot,
                &raw mut nw,
                h__.offset(h_offset as isize) as *mut doublereal,
                ldh,
                iloz,
                ihiz,
                z__.offset(z_offset as isize) as *mut doublereal,
                ldz,
                &raw mut ls,
                &raw mut ld,
                wr.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                wi.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                h__.offset((kv + h_dim1) as isize) as *mut doublereal,
                ldh,
                &raw mut nho,
                h__.offset((kv + kt * h_dim1) as isize) as *mut doublereal,
                ldh,
                &raw mut nve,
                h__.offset((kwv + h_dim1) as isize) as *mut doublereal,
                ldh,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                lwork,
            );
            kbot -= ld as ::core::ffi::c_long;
            ks = (kbot as ::core::ffi::c_long - ls as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            if ld == 0 as ::core::ffi::c_long
                || ld as ::core::ffi::c_long * 100 as ::core::ffi::c_long <= nw * nibble
                    && kbot as ::core::ffi::c_long - ktop as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long
                        > (if nmin <= nwmax {
                            nmin as ::core::ffi::c_long
                        } else {
                            nwmax as ::core::ffi::c_long
                        })
            {
                i__4 = 2 as integer;
                i__5 = kbot - ktop;
                i__2 = (if nsmax <= nsr {
                    nsmax as ::core::ffi::c_long
                } else {
                    nsr as ::core::ffi::c_long
                }) as integer;
                i__3 = (if i__4 >= i__5 {
                    i__4 as ::core::ffi::c_long
                } else {
                    i__5 as ::core::ffi::c_long
                }) as integer;
                ns = (if i__2 <= i__3 {
                    i__2 as ::core::ffi::c_long
                } else {
                    i__3 as ::core::ffi::c_long
                }) as integer;
                ns -= ns as ::core::ffi::c_long % 2 as ::core::ffi::c_long;
                if ndfl as ::core::ffi::c_long % 6 as ::core::ffi::c_long
                    == 0 as ::core::ffi::c_long
                {
                    ks = (kbot as ::core::ffi::c_long - ns as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                    i__3 = (ks as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    i__4 = (ktop as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
                    i__2 = (if i__3 >= i__4 {
                        i__3 as ::core::ffi::c_long
                    } else {
                        i__4 as ::core::ffi::c_long
                    }) as integer;
                    i__ = kbot;
                    while i__ >= i__2 {
                        d__1 = *h__.offset((i__ + (i__ - 1 as integer) * h_dim1) as isize);
                        d__2 = *h__
                            .offset((i__ - 1 as integer + (i__ - 2 as integer) * h_dim1) as isize);
                        ss = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        })) as doublereal;
                        aa = ss * 0.75f64 + *h__.offset((i__ + i__ * h_dim1) as isize);
                        bb = ss;
                        cc = (ss as ::core::ffi::c_double * -0.4375f64) as doublereal;
                        dd = aa;
                        dlanv2__0(
                            &raw mut aa,
                            &raw mut bb,
                            &raw mut cc,
                            &raw mut dd,
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
                        i__ += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
                    }
                    if ks == ktop {
                        *wr.offset(
                            (ks as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        ) = *h__
                            .offset((ks + 1 as integer + (ks + 1 as integer) * h_dim1) as isize);
                        *wi.offset(
                            (ks as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        ) = 0.0f64 as doublereal;
                        *wr.offset(ks as isize) = *wr.offset(
                            (ks as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        );
                        *wi.offset(ks as isize) = *wi.offset(
                            (ks as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                        );
                    }
                } else {
                    if kbot as ::core::ffi::c_long - ks as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long
                        <= ns as ::core::ffi::c_long / 2 as ::core::ffi::c_long
                    {
                        ks = (kbot as ::core::ffi::c_long - ns as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as integer;
                        kt = (*n - ns as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        dlacpy__0(
                            b"A\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut ns,
                            &raw mut ns,
                            h__.offset((ks + ks * h_dim1) as isize) as *mut doublereal,
                            ldh,
                            h__.offset((kt + h_dim1) as isize) as *mut doublereal,
                            ldh,
                        );
                        dlahqr__0(
                            &raw mut c_false,
                            &raw mut c_false,
                            &raw mut ns,
                            &raw mut c__1,
                            &raw mut ns,
                            h__.offset((kt + h_dim1) as isize) as *mut doublereal,
                            ldh,
                            wr.offset(ks as isize) as *mut doublereal,
                            wi.offset(ks as isize) as *mut doublereal,
                            &raw mut c__1,
                            &raw mut c__1,
                            &raw mut zdum as *mut doublereal,
                            &raw mut c__1,
                            &raw mut inf,
                        );
                        ks += inf as ::core::ffi::c_long;
                        if ks >= kbot {
                            aa = *h__.offset(
                                (kbot - 1 as integer + (kbot - 1 as integer) * h_dim1) as isize,
                            );
                            cc = *h__.offset((kbot + (kbot - 1 as integer) * h_dim1) as isize);
                            bb = *h__.offset((kbot - 1 as integer + kbot * h_dim1) as isize);
                            dd = *h__.offset((kbot + kbot * h_dim1) as isize);
                            dlanv2__0(
                                &raw mut aa,
                                &raw mut bb,
                                &raw mut cc,
                                &raw mut dd,
                                wr.offset(
                                    (kbot as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                wi.offset(
                                    (kbot as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                wr.offset(kbot as isize) as *mut doublereal,
                                wi.offset(kbot as isize) as *mut doublereal,
                                &raw mut cs,
                                &raw mut sn,
                            );
                            ks =
                                (kbot as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        }
                    }
                    if kbot as ::core::ffi::c_long - ks as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long
                        > ns
                    {
                        sorted = FALSE_ as logical;
                        i__2 = (ks as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                        k = kbot;
                        while k >= i__2 {
                            if sorted != 0 {
                                break;
                            }
                            sorted = TRUE_ as logical;
                            i__3 = (k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                            i__ = ks;
                            while i__ <= i__3 {
                                d__1 = *wr.offset(i__ as isize);
                                d__2 = *wi.offset(i__ as isize);
                                d__3 = *wr.offset(
                                    (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as isize,
                                );
                                d__4 = *wi.offset(
                                    (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as isize,
                                );
                                if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    d__1 as ::core::ffi::c_double
                                } else {
                                    -(d__1 as ::core::ffi::c_double)
                                }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    d__2 as ::core::ffi::c_double
                                } else {
                                    -(d__2 as ::core::ffi::c_double)
                                }) < (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    d__3 as ::core::ffi::c_double
                                } else {
                                    -(d__3 as ::core::ffi::c_double)
                                }) + (if d__4
                                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    d__4 as ::core::ffi::c_double
                                } else {
                                    -(d__4 as ::core::ffi::c_double)
                                }) {
                                    sorted = FALSE_ as logical;
                                    swap = *wr.offset(i__ as isize);
                                    *wr.offset(i__ as isize) = *wr.offset(
                                        (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    );
                                    *wr.offset(
                                        (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) = swap;
                                    swap = *wi.offset(i__ as isize);
                                    *wi.offset(i__ as isize) = *wi.offset(
                                        (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    );
                                    *wi.offset(
                                        (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) = swap;
                                }
                                i__ += 1;
                            }
                            k -= 1;
                        }
                    }
                    i__2 = (ks as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
                    i__ = kbot;
                    while i__ >= i__2 {
                        if *wi.offset(i__ as isize)
                            != -*wi.offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            )
                        {
                            swap = *wr.offset(i__ as isize);
                            *wr.offset(i__ as isize) = *wr.offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                            *wr.offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) = *wr.offset(
                                (i__ as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            );
                            *wr.offset(
                                (i__ as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            ) = swap;
                            swap = *wi.offset(i__ as isize);
                            *wi.offset(i__ as isize) = *wi.offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                            *wi.offset(
                                (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) = *wi.offset(
                                (i__ as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            );
                            *wi.offset(
                                (i__ as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as isize,
                            ) = swap;
                        }
                        i__ += -(2 as ::core::ffi::c_int) as ::core::ffi::c_long;
                    }
                }
                if kbot as ::core::ffi::c_long - ks as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long
                    == 2 as ::core::ffi::c_long
                {
                    if *wi.offset(kbot as isize) == 0.0f64 {
                        d__1 = *wr.offset(kbot as isize)
                            - *h__.offset((kbot + kbot * h_dim1) as isize);
                        d__2 = *wr.offset(
                            (kbot as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                        ) - *h__.offset((kbot + kbot * h_dim1) as isize);
                        if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) < (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__2 as ::core::ffi::c_double
                        } else {
                            -(d__2 as ::core::ffi::c_double)
                        }) {
                            *wr.offset(
                                (kbot as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            ) = *wr.offset(kbot as isize);
                        } else {
                            *wr.offset(kbot as isize) = *wr.offset(
                                (kbot as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize,
                            );
                        }
                    }
                }
                i__2 = ns;
                i__3 = (kbot as ::core::ffi::c_long - ks as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                ns = (if i__2 <= i__3 {
                    i__2 as ::core::ffi::c_long
                } else {
                    i__3 as ::core::ffi::c_long
                }) as integer;
                ns -= ns as ::core::ffi::c_long % 2 as ::core::ffi::c_long;
                ks = (kbot as ::core::ffi::c_long - ns as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                kdu = (ns as ::core::ffi::c_long * 3 as ::core::ffi::c_long
                    - 3 as ::core::ffi::c_long) as integer;
                ku = (*n - kdu as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                kwh = (kdu as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                nho = (*n
                    - kdu as ::core::ffi::c_long
                    - 3 as ::core::ffi::c_long
                    - (kdu as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    + 1 as ::core::ffi::c_long) as integer;
                kwv = (kdu as ::core::ffi::c_long + 4 as ::core::ffi::c_long) as integer;
                nve = (*n - kdu as ::core::ffi::c_long - kwv as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                dlaqr5__0(
                    wantt,
                    wantz,
                    &raw mut kacc22,
                    n,
                    &raw mut ktop,
                    &raw mut kbot,
                    &raw mut ns,
                    wr.offset(ks as isize) as *mut doublereal,
                    wi.offset(ks as isize) as *mut doublereal,
                    h__.offset(h_offset as isize) as *mut doublereal,
                    ldh,
                    iloz,
                    ihiz,
                    z__.offset(z_offset as isize) as *mut doublereal,
                    ldz,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c__3,
                    h__.offset((ku + h_dim1) as isize) as *mut doublereal,
                    ldh,
                    &raw mut nve,
                    h__.offset((kwv + h_dim1) as isize) as *mut doublereal,
                    ldh,
                    &raw mut nho,
                    h__.offset((ku + kwh * h_dim1) as isize) as *mut doublereal,
                    ldh,
                );
            }
            if ld > 0 as ::core::ffi::c_long {
                ndfl = 1 as integer;
            } else {
                ndfl += 1;
            }
            it += 1;
        }
        match current_block_208 {
            1918110639124887667 => {
                *info = kbot;
            }
            _ => {}
        }
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    return 0 as ::core::ffi::c_int;
}
