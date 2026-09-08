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
static mut c_b3: doublereal = -1.0f64;
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaed2_(
    mut k: *mut integer,
    mut n: *mut integer,
    mut n1: *mut integer,
    mut d__: *mut doublereal,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut indxq: *mut integer,
    mut rho: *mut doublereal,
    mut z__: *mut doublereal,
    mut dlamda: *mut doublereal,
    mut w: *mut doublereal,
    mut q2: *mut doublereal,
    mut indx: *mut integer,
    mut indxc: *mut integer,
    mut indxp: *mut integer,
    mut coltyp: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut d__4: doublereal = 0.;
    let mut c__: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut s: doublereal = 0.;
    let mut t: doublereal = 0.;
    let mut k2: integer = 0;
    let mut n2: integer = 0;
    let mut ct: integer = 0;
    let mut nj: integer = 0;
    let mut pj: integer = 0;
    let mut js: integer = 0;
    let mut iq1: integer = 0;
    let mut iq2: integer = 0;
    let mut n1p1: integer = 0;
    let mut eps: doublereal = 0.;
    let mut tau: doublereal = 0.;
    let mut tol: doublereal = 0.;
    let mut psm: [integer; 4] = [0; 4];
    let mut imax: integer = 0;
    let mut jmax: integer = 0;
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
    let mut ctot: [integer; 4] = [0; 4];
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dscal"]
        fn f2c_dscal_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
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
        #[link_name = "dgelsd_closure_dlapy2_"]
        fn dlapy2__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
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
        #[link_name = "dgelsd_closure_dlamrg_"]
        fn dlamrg__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
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
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    d__ = d__.offset(-1);
    q_dim1 = *ldq;
    q_offset = 1 as integer + q_dim1;
    q = q.offset(-(q_offset as isize));
    indxq = indxq.offset(-1);
    z__ = z__.offset(-1);
    dlamda = dlamda.offset(-1);
    w = w.offset(-1);
    q2 = q2.offset(-1);
    indx = indx.offset(-1);
    indxc = indxc.offset(-1);
    indxp = indxp.offset(-1);
    coltyp = coltyp.offset(-1);
    *info = 0 as integer;
    if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *ldq
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(6 as ::core::ffi::c_int) as integer;
    } else {
        i__1 = 1 as integer;
        i__2 = (*n / 2 as ::core::ffi::c_long) as integer;
        if (if i__1 <= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) > *n1
            || (*n / 2 as ::core::ffi::c_long) < *n1
        {
            *info = -(3 as ::core::ffi::c_int) as integer;
        }
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLAED2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    n2 = *n - *n1;
    n1p1 = (*n1 + 1 as ::core::ffi::c_long) as integer;
    if *rho < 0.0f64 {
        f2c_dscal_0(
            &raw mut n2,
            &raw mut c_b3,
            z__.offset(n1p1 as isize) as *mut doublereal,
            &raw mut c__1,
        );
    }
    t = (1.0f64 / sqrt(2.0f64)) as doublereal;
    f2c_dscal_0(
        n,
        &raw mut t,
        z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
    );
    d__1 = (*rho * 2.0f64) as doublereal;
    *rho = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        d__1 as ::core::ffi::c_double
    } else {
        -(d__1 as ::core::ffi::c_double)
    }) as doublereal;
    i__1 = *n;
    i__ = n1p1;
    while i__ <= i__1 {
        let ref mut fresh0 = *indxq.offset(i__ as isize);
        *fresh0 += *n1 as ::core::ffi::c_long;
        i__ += 1;
    }
    i__1 = *n;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *dlamda.offset(i__ as isize) = *d__.offset(*indxq.offset(i__ as isize) as isize);
        i__ += 1;
    }
    dlamrg__0(
        n1,
        &raw mut n2,
        dlamda.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
        &raw mut c__1,
        indxc.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
    );
    i__1 = *n;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *indx.offset(i__ as isize) = *indxq.offset(*indxc.offset(i__ as isize) as isize);
        i__ += 1;
    }
    imax = f2c_idamax_0(
        n,
        z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
    );
    jmax = f2c_idamax_0(
        n,
        d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
    );
    eps = dlamch__0(
        b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    d__1 = *d__.offset(jmax as isize);
    d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        d__1 as ::core::ffi::c_double
    } else {
        -(d__1 as ::core::ffi::c_double)
    }) as doublereal;
    d__2 = *z__.offset(imax as isize);
    d__4 = (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        d__2 as ::core::ffi::c_double
    } else {
        -(d__2 as ::core::ffi::c_double)
    }) as doublereal;
    tol = (eps as ::core::ffi::c_double
        * 8.0f64
        * (if d__3 >= d__4 {
            d__3 as ::core::ffi::c_double
        } else {
            d__4 as ::core::ffi::c_double
        })) as doublereal;
    d__1 = *z__.offset(imax as isize);
    if *rho
        * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        })
        <= tol
    {
        *k = 0 as integer;
        iq2 = 1 as integer;
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            i__ = *indx.offset(j as isize);
            f2c_dcopy_0(
                n,
                q.offset(
                    (i__ as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                q2.offset(iq2 as isize) as *mut doublereal,
                &raw mut c__1,
            );
            *dlamda.offset(j as isize) = *d__.offset(i__ as isize);
            iq2 += *n as ::core::ffi::c_long;
            j += 1;
        }
        dlacpy__0(
            b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            n,
            q2.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            n,
            q.offset(q_offset as isize) as *mut doublereal,
            ldq,
        );
        f2c_dcopy_0(
            n,
            dlamda.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
    } else {
        i__1 = *n1;
        i__ = 1 as integer;
        while i__ <= i__1 {
            *coltyp.offset(i__ as isize) = 1 as integer;
            i__ += 1;
        }
        i__1 = *n;
        i__ = n1p1;
        while i__ <= i__1 {
            *coltyp.offset(i__ as isize) = 3 as integer;
            i__ += 1;
        }
        *k = 0 as integer;
        k2 = (*n + 1 as ::core::ffi::c_long) as integer;
        i__1 = *n;
        j = 1 as integer;
        loop {
            if !(j <= i__1) {
                current_block = 12803210597711493252;
                break;
            }
            nj = *indx.offset(j as isize);
            d__1 = *z__.offset(nj as isize);
            if *rho
                * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                })
                <= tol
            {
                k2 -= 1;
                *coltyp.offset(nj as isize) = 4 as integer;
                *indxp.offset(k2 as isize) = nj;
                if j == *n {
                    current_block = 3617492229948649388;
                    break;
                }
                j += 1;
            } else {
                pj = nj;
                current_block = 12803210597711493252;
                break;
            }
        }
        loop {
            match current_block {
                3617492229948649388 => {
                    *k += 1;
                    break;
                }
                _ => {
                    j += 1;
                    nj = *indx.offset(j as isize);
                    if j > *n {
                        current_block = 3617492229948649388;
                        continue;
                    }
                    d__1 = *z__.offset(nj as isize);
                    if *rho
                        * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        })
                        <= tol
                    {
                        k2 -= 1;
                        *coltyp.offset(nj as isize) = 4 as integer;
                        *indxp.offset(k2 as isize) = nj;
                    } else {
                        s = *z__.offset(pj as isize);
                        c__ = *z__.offset(nj as isize);
                        tau = dlapy2__0(&raw mut c__, &raw mut s);
                        t = *d__.offset(nj as isize) - *d__.offset(pj as isize);
                        c__ /= tau as ::core::ffi::c_double;
                        s = -s / tau;
                        d__1 = t * c__ * s;
                        if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) <= tol
                        {
                            *z__.offset(nj as isize) = tau;
                            *z__.offset(pj as isize) = 0.0f64 as doublereal;
                            if *coltyp.offset(nj as isize) != *coltyp.offset(pj as isize) {
                                *coltyp.offset(nj as isize) = 2 as integer;
                            }
                            *coltyp.offset(pj as isize) = 4 as integer;
                            f2c_drot_0(
                                n,
                                q.offset(
                                    (pj as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                                q.offset(
                                    (nj as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                                &raw mut c__,
                                &raw mut s,
                            );
                            d__1 = c__;
                            d__2 = s;
                            t = *d__.offset(pj as isize) * (d__1 * d__1)
                                + *d__.offset(nj as isize) * (d__2 * d__2);
                            d__1 = s;
                            d__2 = c__;
                            *d__.offset(nj as isize) = *d__.offset(pj as isize) * (d__1 * d__1)
                                + *d__.offset(nj as isize) * (d__2 * d__2);
                            *d__.offset(pj as isize) = t;
                            k2 -= 1;
                            i__ = 1 as integer;
                            loop {
                                if k2 + i__ <= *n {
                                    if *d__.offset(pj as isize)
                                        < *d__.offset(*indxp.offset((k2 + i__) as isize) as isize)
                                    {
                                        *indxp.offset(
                                            (k2 as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                                - 1 as ::core::ffi::c_long)
                                                as isize,
                                        ) = *indxp.offset((k2 + i__) as isize);
                                        *indxp.offset((k2 + i__) as isize) = pj;
                                        i__ += 1;
                                    } else {
                                        *indxp.offset(
                                            (k2 as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                                - 1 as ::core::ffi::c_long)
                                                as isize,
                                        ) = pj;
                                        break;
                                    }
                                } else {
                                    *indxp.offset(
                                        (k2 as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                            - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) = pj;
                                    break;
                                }
                            }
                            pj = nj;
                        } else {
                            *k += 1;
                            *dlamda.offset(*k as isize) = *d__.offset(pj as isize);
                            *w.offset(*k as isize) = *z__.offset(pj as isize);
                            *indxp.offset(*k as isize) = pj;
                            pj = nj;
                        }
                    }
                    current_block = 12803210597711493252;
                }
            }
        }
        *dlamda.offset(*k as isize) = *d__.offset(pj as isize);
        *w.offset(*k as isize) = *z__.offset(pj as isize);
        *indxp.offset(*k as isize) = pj;
        j = 1 as integer;
        while j <= 4 as ::core::ffi::c_long {
            ctot[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] = 0 as integer;
            j += 1;
        }
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            ct = *coltyp.offset(j as isize);
            ctot[(ct as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] += 1;
            j += 1;
        }
        psm[0 as ::core::ffi::c_int as usize] = 1 as integer;
        psm[1 as ::core::ffi::c_int as usize] =
            (ctot[0 as ::core::ffi::c_int as usize] + 1 as ::core::ffi::c_long) as integer;
        psm[2 as ::core::ffi::c_int as usize] =
            psm[1 as ::core::ffi::c_int as usize] + ctot[1 as ::core::ffi::c_int as usize];
        psm[3 as ::core::ffi::c_int as usize] =
            psm[2 as ::core::ffi::c_int as usize] + ctot[2 as ::core::ffi::c_int as usize];
        *k = *n - ctot[3 as ::core::ffi::c_int as usize];
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            js = *indxp.offset(j as isize);
            ct = *coltyp.offset(js as isize);
            *indx.offset(
                psm[(ct as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] as isize,
            ) = js;
            *indxc.offset(
                psm[(ct as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] as isize,
            ) = j;
            psm[(ct as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] += 1;
            j += 1;
        }
        i__ = 1 as integer;
        iq1 = 1 as integer;
        iq2 = ((ctot[0 as ::core::ffi::c_int as usize] + ctot[1 as ::core::ffi::c_int as usize])
            * *n1
            + 1 as ::core::ffi::c_long) as integer;
        i__1 = ctot[0 as ::core::ffi::c_int as usize];
        j = 1 as integer;
        while j <= i__1 {
            js = *indx.offset(i__ as isize);
            f2c_dcopy_0(
                n1,
                q.offset(
                    (js as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                q2.offset(iq1 as isize) as *mut doublereal,
                &raw mut c__1,
            );
            *z__.offset(i__ as isize) = *d__.offset(js as isize);
            i__ += 1;
            iq1 += *n1 as ::core::ffi::c_long;
            j += 1;
        }
        i__1 = ctot[1 as ::core::ffi::c_int as usize];
        j = 1 as integer;
        while j <= i__1 {
            js = *indx.offset(i__ as isize);
            f2c_dcopy_0(
                n1,
                q.offset(
                    (js as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                q2.offset(iq1 as isize) as *mut doublereal,
                &raw mut c__1,
            );
            f2c_dcopy_0(
                &raw mut n2,
                q.offset((*n1 + 1 as integer + js * q_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                q2.offset(iq2 as isize) as *mut doublereal,
                &raw mut c__1,
            );
            *z__.offset(i__ as isize) = *d__.offset(js as isize);
            i__ += 1;
            iq1 += *n1 as ::core::ffi::c_long;
            iq2 += n2 as ::core::ffi::c_long;
            j += 1;
        }
        i__1 = ctot[2 as ::core::ffi::c_int as usize];
        j = 1 as integer;
        while j <= i__1 {
            js = *indx.offset(i__ as isize);
            f2c_dcopy_0(
                &raw mut n2,
                q.offset((*n1 + 1 as integer + js * q_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                q2.offset(iq2 as isize) as *mut doublereal,
                &raw mut c__1,
            );
            *z__.offset(i__ as isize) = *d__.offset(js as isize);
            i__ += 1;
            iq2 += n2 as ::core::ffi::c_long;
            j += 1;
        }
        iq1 = iq2;
        i__1 = ctot[3 as ::core::ffi::c_int as usize];
        j = 1 as integer;
        while j <= i__1 {
            js = *indx.offset(i__ as isize);
            f2c_dcopy_0(
                n,
                q.offset(
                    (js as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                q2.offset(iq2 as isize) as *mut doublereal,
                &raw mut c__1,
            );
            iq2 += *n as ::core::ffi::c_long;
            *z__.offset(i__ as isize) = *d__.offset(js as isize);
            i__ += 1;
            j += 1;
        }
        dlacpy__0(
            b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            (&raw mut ctot as *mut integer).offset(3 as ::core::ffi::c_int as isize)
                as *mut integer,
            q2.offset(iq1 as isize) as *mut doublereal,
            n,
            q.offset(
                ((*k + 1 as ::core::ffi::c_long) * q_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            ldq,
        );
        i__1 = *n - *k;
        f2c_dcopy_0(
            &raw mut i__1,
            z__.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            &raw mut c__1,
            d__.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            &raw mut c__1,
        );
        j = 1 as integer;
        while j <= 4 as ::core::ffi::c_long {
            *coltyp.offset(j as isize) =
                ctot[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
            j += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
