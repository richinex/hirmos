#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn log(_: doublereal) -> ::core::ffi::c_double;
}
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
static mut c__9: integer = 9 as integer;
static mut c__0: integer = 0 as integer;
static mut c__2: integer = 2 as integer;
static mut c_b23: doublereal = 1.0f64;
static mut c_b24: doublereal = 0.0f64;
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaed0_(
    mut icompq: *mut integer,
    mut qsiz: *mut integer,
    mut n: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut qstore: *mut doublereal,
    mut ldqs: *mut integer,
    mut work: *mut doublereal,
    mut iwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut qstore_dim1: integer = 0;
    let mut qstore_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_pow_ii"]
        fn pow_ii_0(_: *mut integer, _: *mut integer) -> integer;
    }
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut iq: integer = 0;
    let mut lgn: integer = 0;
    let mut msd2: integer = 0;
    let mut smm1: integer = 0;
    let mut spm1: integer = 0;
    let mut spm2: integer = 0;
    let mut temp: doublereal = 0.;
    let mut curr: integer = 0;
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
    let mut iperm: integer = 0;
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
    let mut indxq: integer = 0;
    let mut iwrem: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dlaed1_"]
        fn dlaed1__0(
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
    let mut iqptr: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dlaed7_"]
        fn dlaed7__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
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
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut tlvls: integer = 0;
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
    let mut igivcl: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_ilaenv_"]
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
    let mut igivnm: integer = 0;
    let mut submat: integer = 0;
    let mut curprb: integer = 0;
    let mut subpbs: integer = 0;
    let mut igivpt: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dsteqr_"]
        fn dsteqr__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut curlvl: integer = 0;
    let mut matsiz: integer = 0;
    let mut iprmpt: integer = 0;
    let mut smlsiz: integer = 0;
    d__ = d__.offset(-1);
    e = e.offset(-1);
    q_dim1 = *ldq;
    q_offset = 1 as integer + q_dim1;
    q = q.offset(-(q_offset as isize));
    qstore_dim1 = *ldqs;
    qstore_offset = 1 as integer + qstore_dim1;
    qstore = qstore.offset(-(qstore_offset as isize));
    work = work.offset(-1);
    iwork = iwork.offset(-1);
    *info = 0 as integer;
    if *icompq < 0 as ::core::ffi::c_long || *icompq > 2 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *icompq == 1 as ::core::ffi::c_long
        && *qsiz
            < (if 0 as ::core::ffi::c_long >= *n {
                0 as ::core::ffi::c_long
            } else {
                *n
            })
    {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *ldq
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(7 as ::core::ffi::c_int) as integer;
    } else if *ldqs
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(9 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLAED0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    smlsiz = ilaenv__0(
        &raw mut c__9,
        b"DLAED0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut c__0,
        &raw mut c__0,
        &raw mut c__0,
        &raw mut c__0,
    );
    *iwork.offset(1 as ::core::ffi::c_int as isize) = *n;
    subpbs = 1 as integer;
    tlvls = 0 as integer;
    while *iwork.offset(subpbs as isize) > smlsiz {
        j = subpbs;
        while j >= 1 as ::core::ffi::c_long {
            *iwork.offset((j as ::core::ffi::c_long * 2 as ::core::ffi::c_long) as isize) =
                ((*iwork.offset(j as isize) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    / 2 as ::core::ffi::c_long) as integer;
            *iwork.offset(
                (((j as ::core::ffi::c_long) << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long)
                    as isize,
            ) = (*iwork.offset(j as isize) as ::core::ffi::c_long / 2 as ::core::ffi::c_long)
                as integer;
            j -= 1;
        }
        tlvls += 1;
        subpbs <<= 1 as ::core::ffi::c_int;
    }
    i__1 = subpbs;
    j = 2 as integer;
    while j <= i__1 {
        let ref mut fresh0 = *iwork.offset(j as isize);
        *fresh0 += *iwork.offset((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
            as ::core::ffi::c_long;
        j += 1;
    }
    spm1 = (subpbs as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
    i__1 = spm1;
    i__ = 1 as integer;
    while i__ <= i__1 {
        submat = (*iwork.offset(i__ as isize) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
            as integer;
        smm1 = (submat as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        d__1 = *e.offset(smm1 as isize);
        let ref mut fresh1 = *d__.offset(smm1 as isize);
        *fresh1 -= (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        });
        d__1 = *e.offset(smm1 as isize);
        let ref mut fresh2 = *d__.offset(submat as isize);
        *fresh2 -= (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        });
        i__ += 1;
    }
    indxq = ((*n << 2 as ::core::ffi::c_int) + 3 as ::core::ffi::c_long) as integer;
    if *icompq != 2 as ::core::ffi::c_long {
        temp = (log(*n as doublereal) / log(2.0f64)) as doublereal;
        lgn = temp as integer;
        if pow_ii_0(&raw mut c__2, &raw mut lgn) < *n {
            lgn += 1;
        }
        if pow_ii_0(&raw mut c__2, &raw mut lgn) < *n {
            lgn += 1;
        }
        iprmpt = (indxq as ::core::ffi::c_long + *n + 1 as ::core::ffi::c_long) as integer;
        iperm = iprmpt + *n * lgn;
        iqptr = iperm + *n * lgn;
        igivpt = (iqptr as ::core::ffi::c_long + *n + 2 as ::core::ffi::c_long) as integer;
        igivcl = igivpt + *n * lgn;
        igivnm = 1 as integer;
        iq = igivnm + (*n << 1 as ::core::ffi::c_int) * lgn;
        i__1 = *n;
        iwrem = (iq as ::core::ffi::c_long
            + i__1 as ::core::ffi::c_long * i__1 as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as integer;
        i__1 = subpbs;
        i__ = 0 as integer;
        while i__ <= i__1 {
            *iwork.offset((iprmpt + i__) as isize) = 1 as integer;
            *iwork.offset((igivpt + i__) as isize) = 1 as integer;
            i__ += 1;
        }
        *iwork.offset(iqptr as isize) = 1 as integer;
    }
    curr = 0 as integer;
    i__1 = spm1;
    i__ = 0 as integer;
    loop {
        if !(i__ <= i__1) {
            current_block = 12369290732426379360;
            break;
        }
        if i__ == 0 as ::core::ffi::c_long {
            submat = 1 as integer;
            matsiz = *iwork.offset(1 as ::core::ffi::c_int as isize);
        } else {
            submat = (*iwork.offset(i__ as isize) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as integer;
            matsiz = *iwork
                .offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                - *iwork.offset(i__ as isize);
        }
        if *icompq == 2 as ::core::ffi::c_long {
            dsteqr__0(
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut matsiz,
                d__.offset(submat as isize) as *mut doublereal,
                e.offset(submat as isize) as *mut doublereal,
                q.offset((submat + submat * q_dim1) as isize) as *mut doublereal,
                ldq,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                info,
            );
            if *info != 0 as ::core::ffi::c_long {
                current_block = 7819383369193499845;
                break;
            }
        } else {
            dsteqr__0(
                b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut matsiz,
                d__.offset(submat as isize) as *mut doublereal,
                e.offset(submat as isize) as *mut doublereal,
                work.offset((iq - 1 as integer + *iwork.offset((iqptr + curr) as isize)) as isize)
                    as *mut doublereal,
                &raw mut matsiz,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                info,
            );
            if *info != 0 as ::core::ffi::c_long {
                current_block = 7819383369193499845;
                break;
            }
            if *icompq == 1 as ::core::ffi::c_long {
                f2c_dgemm_0(
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    qsiz,
                    &raw mut matsiz,
                    &raw mut matsiz,
                    &raw mut c_b23,
                    q.offset(
                        (submat as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    ldq,
                    work.offset(
                        (iq - 1 as integer + *iwork.offset((iqptr + curr) as isize)) as isize,
                    ) as *mut doublereal,
                    &raw mut matsiz,
                    &raw mut c_b24,
                    qstore.offset(
                        (submat as ::core::ffi::c_long * qstore_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    ldqs,
                );
            }
            i__2 = matsiz;
            *iwork.offset(
                (iqptr as ::core::ffi::c_long
                    + curr as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) = *iwork.offset((iqptr + curr) as isize) + i__2 * i__2;
            curr += 1;
        }
        k = 1 as integer;
        i__2 = *iwork.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
        j = submat;
        while j <= i__2 {
            *iwork.offset((indxq + j) as isize) = k;
            k += 1;
            j += 1;
        }
        i__ += 1;
    }
    match current_block {
        12369290732426379360 => {
            curlvl = 1 as integer;
            '_L80: loop {
                if subpbs > 1 as ::core::ffi::c_long {
                    spm2 = (subpbs as ::core::ffi::c_long - 2 as ::core::ffi::c_long) as integer;
                    i__1 = spm2;
                    i__ = 0 as integer;
                    while i__ <= i__1 {
                        if i__ == 0 as ::core::ffi::c_long {
                            submat = 1 as integer;
                            matsiz = *iwork.offset(2 as ::core::ffi::c_int as isize);
                            msd2 = *iwork.offset(1 as ::core::ffi::c_int as isize);
                            curprb = 0 as integer;
                        } else {
                            submat = (*iwork.offset(i__ as isize) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long)
                                as integer;
                            matsiz = *iwork.offset(
                                (i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                            ) - *iwork.offset(i__ as isize);
                            msd2 = (matsiz as ::core::ffi::c_long / 2 as ::core::ffi::c_long)
                                as integer;
                            curprb += 1;
                        }
                        if *icompq == 2 as ::core::ffi::c_long {
                            dlaed1__0(
                                &raw mut matsiz,
                                d__.offset(submat as isize) as *mut doublereal,
                                q.offset((submat + submat * q_dim1) as isize) as *mut doublereal,
                                ldq,
                                iwork.offset((indxq + submat) as isize) as *mut integer,
                                e.offset(
                                    (submat as ::core::ffi::c_long + msd2 as ::core::ffi::c_long
                                        - 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut msd2,
                                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                                iwork.offset(
                                    (subpbs as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut integer,
                                info,
                            );
                        } else {
                            dlaed7__0(
                                icompq,
                                &raw mut matsiz,
                                qsiz,
                                &raw mut tlvls,
                                &raw mut curlvl,
                                &raw mut curprb,
                                d__.offset(submat as isize) as *mut doublereal,
                                qstore.offset(
                                    (submat as ::core::ffi::c_long
                                        * qstore_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                ldqs,
                                iwork.offset((indxq + submat) as isize) as *mut integer,
                                e.offset(
                                    (submat as ::core::ffi::c_long + msd2 as ::core::ffi::c_long
                                        - 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut msd2,
                                work.offset(iq as isize) as *mut doublereal,
                                iwork.offset(iqptr as isize) as *mut integer,
                                iwork.offset(iprmpt as isize) as *mut integer,
                                iwork.offset(iperm as isize) as *mut integer,
                                iwork.offset(igivpt as isize) as *mut integer,
                                iwork.offset(igivcl as isize) as *mut integer,
                                work.offset(igivnm as isize) as *mut doublereal,
                                work.offset(iwrem as isize) as *mut doublereal,
                                iwork.offset(
                                    (subpbs as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut integer,
                                info,
                            );
                        }
                        if *info != 0 as ::core::ffi::c_long {
                            current_block = 7819383369193499845;
                            break '_L80;
                        }
                        *iwork.offset(
                            (i__ as ::core::ffi::c_long / 2 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) = *iwork.offset(
                            (i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                        );
                        i__ += 2 as ::core::ffi::c_long;
                    }
                    subpbs /= 2 as ::core::ffi::c_long;
                    curlvl += 1;
                } else {
                    if *icompq == 1 as ::core::ffi::c_long {
                        i__1 = *n;
                        i__ = 1 as integer;
                        while i__ <= i__1 {
                            j = *iwork.offset((indxq + i__) as isize);
                            *work.offset(i__ as isize) = *d__.offset(j as isize);
                            f2c_dcopy_0(
                                qsiz,
                                qstore.offset(
                                    (j as ::core::ffi::c_long * qstore_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                                q.offset(
                                    (i__ as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                            );
                            i__ += 1;
                        }
                        f2c_dcopy_0(
                            n,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            &raw mut c__1,
                            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            &raw mut c__1,
                        );
                    } else if *icompq == 2 as ::core::ffi::c_long {
                        i__1 = *n;
                        i__ = 1 as integer;
                        while i__ <= i__1 {
                            j = *iwork.offset((indxq + i__) as isize);
                            *work.offset(i__ as isize) = *d__.offset(j as isize);
                            f2c_dcopy_0(
                                n,
                                q.offset(
                                    (j as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                        + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                                work.offset(
                                    (*n * i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                        as isize,
                                ) as *mut doublereal,
                                &raw mut c__1,
                            );
                            i__ += 1;
                        }
                        f2c_dcopy_0(
                            n,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            &raw mut c__1,
                            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            &raw mut c__1,
                        );
                        dlacpy__0(
                            b"A\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            n,
                            n,
                            work.offset((*n + 1 as ::core::ffi::c_long) as isize)
                                as *mut doublereal,
                            n,
                            q.offset(q_offset as isize) as *mut doublereal,
                            ldq,
                        );
                    } else {
                        i__1 = *n;
                        i__ = 1 as integer;
                        while i__ <= i__1 {
                            j = *iwork.offset((indxq + i__) as isize);
                            *work.offset(i__ as isize) = *d__.offset(j as isize);
                            i__ += 1;
                        }
                        f2c_dcopy_0(
                            n,
                            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            &raw mut c__1,
                            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            &raw mut c__1,
                        );
                    }
                    current_block = 5301938843700733762;
                    break;
                }
            }
        }
        _ => {}
    }
    match current_block {
        7819383369193499845 => {
            *info = (submat as ::core::ffi::c_long * (*n + 1 as ::core::ffi::c_long)
                + submat as ::core::ffi::c_long
                + matsiz as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as integer;
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
