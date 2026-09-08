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
pub unsafe extern "C" fn dsyevd_closure_dlaed8_(
    mut icompq: *mut integer,
    mut k: *mut integer,
    mut n: *mut integer,
    mut qsiz: *mut integer,
    mut d__: *mut doublereal,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut indxq: *mut integer,
    mut rho: *mut doublereal,
    mut cutpnt: *mut integer,
    mut z__: *mut doublereal,
    mut dlamda: *mut doublereal,
    mut q2: *mut doublereal,
    mut ldq2: *mut integer,
    mut w: *mut doublereal,
    mut perm: *mut integer,
    mut givptr: *mut integer,
    mut givcol: *mut integer,
    mut givnum: *mut doublereal,
    mut indxp: *mut integer,
    mut indx: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut q2_dim1: integer = 0;
    let mut q2_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut c__: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut s: doublereal = 0.;
    let mut t: doublereal = 0.;
    let mut k2: integer = 0;
    let mut n1: integer = 0;
    let mut n2: integer = 0;
    let mut jp: integer = 0;
    let mut n1p1: integer = 0;
    let mut eps: doublereal = 0.;
    let mut tau: doublereal = 0.;
    let mut tol: doublereal = 0.;
    let mut jlam: integer = 0;
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
    q2_dim1 = *ldq2;
    q2_offset = 1 as integer + q2_dim1;
    q2 = q2.offset(-(q2_offset as isize));
    w = w.offset(-1);
    perm = perm.offset(-1);
    givcol = givcol.offset(-(3 as ::core::ffi::c_int as isize));
    givnum = givnum.offset(-(3 as ::core::ffi::c_int as isize));
    indxp = indxp.offset(-1);
    indx = indx.offset(-1);
    *info = 0 as integer;
    if *icompq < 0 as ::core::ffi::c_long || *icompq > 1 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *icompq == 1 as ::core::ffi::c_long && *qsiz < *n {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *ldq
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(7 as ::core::ffi::c_int) as integer;
    } else if *cutpnt
        < (if 1 as ::core::ffi::c_long <= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
        || *cutpnt > *n
    {
        *info = -(10 as ::core::ffi::c_int) as integer;
    } else if *ldq2
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(14 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLAED8\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    n1 = *cutpnt;
    n2 = *n - n1;
    n1p1 = (n1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    if *rho < 0.0f64 {
        f2c_dscal_0(
            &raw mut n2,
            &raw mut c_b3,
            z__.offset(n1p1 as isize) as *mut doublereal,
            &raw mut c__1,
        );
    }
    t = (1.0f64 / sqrt(2.0f64)) as doublereal;
    i__1 = *n;
    j = 1 as integer;
    while j <= i__1 {
        *indx.offset(j as isize) = j;
        j += 1;
    }
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
    i__ = (*cutpnt + 1 as ::core::ffi::c_long) as integer;
    while i__ <= i__1 {
        let ref mut fresh0 = *indxq.offset(i__ as isize);
        *fresh0 += *cutpnt as ::core::ffi::c_long;
        i__ += 1;
    }
    i__1 = *n;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *dlamda.offset(i__ as isize) = *d__.offset(*indxq.offset(i__ as isize) as isize);
        *w.offset(i__ as isize) = *z__.offset(*indxq.offset(i__ as isize) as isize);
        i__ += 1;
    }
    i__ = 1 as integer;
    j = (*cutpnt + 1 as ::core::ffi::c_long) as integer;
    dlamrg__0(
        &raw mut n1,
        &raw mut n2,
        dlamda.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
        &raw mut c__1,
        indx.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
    );
    i__1 = *n;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *d__.offset(i__ as isize) = *dlamda.offset(*indx.offset(i__ as isize) as isize);
        *z__.offset(i__ as isize) = *w.offset(*indx.offset(i__ as isize) as isize);
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
    tol = (eps as ::core::ffi::c_double
        * 8.0f64
        * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
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
        if *icompq == 0 as ::core::ffi::c_long {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                *perm.offset(j as isize) = *indxq.offset(*indx.offset(j as isize) as isize);
                j += 1;
            }
        } else {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                *perm.offset(j as isize) = *indxq.offset(*indx.offset(j as isize) as isize);
                f2c_dcopy_0(
                    qsiz,
                    q.offset(
                        (*perm.offset(j as isize) as ::core::ffi::c_long
                            * q_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    q2.offset(
                        (j as ::core::ffi::c_long * q2_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                j += 1;
            }
            dlacpy__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                qsiz,
                n,
                q2.offset((q2_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldq2,
                q.offset((q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldq,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
    *k = 0 as integer;
    *givptr = 0 as integer;
    k2 = (*n + 1 as ::core::ffi::c_long) as integer;
    i__1 = *n;
    j = 1 as integer;
    loop {
        if !(j <= i__1) {
            current_block = 946983070274721668;
            break;
        }
        d__1 = *z__.offset(j as isize);
        if *rho
            * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            })
            <= tol
        {
            k2 -= 1;
            *indxp.offset(k2 as isize) = j;
            if j == *n {
                current_block = 4899247181024612788;
                break;
            }
            j += 1;
        } else {
            jlam = j;
            current_block = 946983070274721668;
            break;
        }
    }
    loop {
        match current_block {
            4899247181024612788 => {
                if *icompq == 0 as ::core::ffi::c_long {
                    i__1 = *n;
                    j = 1 as integer;
                    while j <= i__1 {
                        jp = *indxp.offset(j as isize);
                        *dlamda.offset(j as isize) = *d__.offset(jp as isize);
                        *perm.offset(j as isize) =
                            *indxq.offset(*indx.offset(jp as isize) as isize);
                        j += 1;
                    }
                } else {
                    i__1 = *n;
                    j = 1 as integer;
                    while j <= i__1 {
                        jp = *indxp.offset(j as isize);
                        *dlamda.offset(j as isize) = *d__.offset(jp as isize);
                        *perm.offset(j as isize) =
                            *indxq.offset(*indx.offset(jp as isize) as isize);
                        f2c_dcopy_0(
                            qsiz,
                            q.offset(
                                (*perm.offset(j as isize) as ::core::ffi::c_long
                                    * q_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                            q2.offset(
                                (j as ::core::ffi::c_long * q2_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                        );
                        j += 1;
                    }
                }
                break;
            }
            _ => {
                j += 1;
                if j > *n {
                    *k += 1;
                    *w.offset(*k as isize) = *z__.offset(jlam as isize);
                    *dlamda.offset(*k as isize) = *d__.offset(jlam as isize);
                    *indxp.offset(*k as isize) = jlam;
                    current_block = 4899247181024612788;
                } else {
                    d__1 = *z__.offset(j as isize);
                    if *rho
                        * (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        })
                        <= tol
                    {
                        k2 -= 1;
                        *indxp.offset(k2 as isize) = j;
                    } else {
                        s = *z__.offset(jlam as isize);
                        c__ = *z__.offset(j as isize);
                        tau = dlapy2__0(&raw mut c__, &raw mut s);
                        t = *d__.offset(j as isize) - *d__.offset(jlam as isize);
                        c__ /= tau as ::core::ffi::c_double;
                        s = -s / tau;
                        d__1 = t * c__ * s;
                        if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1 as ::core::ffi::c_double
                        } else {
                            -(d__1 as ::core::ffi::c_double)
                        }) <= tol
                        {
                            *z__.offset(j as isize) = tau;
                            *z__.offset(jlam as isize) = 0.0f64 as doublereal;
                            *givptr += 1;
                            *givcol.offset(
                                ((*givptr << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) = *indxq.offset(*indx.offset(jlam as isize) as isize);
                            *givcol.offset(
                                ((*givptr << 1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_long)
                                    as isize,
                            ) = *indxq.offset(*indx.offset(j as isize) as isize);
                            *givnum.offset(
                                ((*givptr << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) = c__;
                            *givnum.offset(
                                ((*givptr << 1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_long)
                                    as isize,
                            ) = s;
                            if *icompq == 1 as ::core::ffi::c_long {
                                f2c_drot_0(
                                    qsiz,
                                    q.offset(
                                        (*indxq.offset(*indx.offset(jlam as isize) as isize)
                                            as ::core::ffi::c_long
                                            * q_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                    q.offset(
                                        (*indxq.offset(*indx.offset(j as isize) as isize)
                                            as ::core::ffi::c_long
                                            * q_dim1 as ::core::ffi::c_long
                                            + 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) as *mut doublereal,
                                    &raw mut c__1,
                                    &raw mut c__,
                                    &raw mut s,
                                );
                            }
                            t = *d__.offset(jlam as isize) * c__ * c__
                                + *d__.offset(j as isize) * s * s;
                            *d__.offset(j as isize) = *d__.offset(jlam as isize) * s * s
                                + *d__.offset(j as isize) * c__ * c__;
                            *d__.offset(jlam as isize) = t;
                            k2 -= 1;
                            i__ = 1 as integer;
                            loop {
                                if k2 + i__ <= *n {
                                    if *d__.offset(jlam as isize)
                                        < *d__.offset(*indxp.offset((k2 + i__) as isize) as isize)
                                    {
                                        *indxp.offset(
                                            (k2 as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                                - 1 as ::core::ffi::c_long)
                                                as isize,
                                        ) = *indxp.offset((k2 + i__) as isize);
                                        *indxp.offset((k2 + i__) as isize) = jlam;
                                        i__ += 1;
                                    } else {
                                        *indxp.offset(
                                            (k2 as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                                - 1 as ::core::ffi::c_long)
                                                as isize,
                                        ) = jlam;
                                        break;
                                    }
                                } else {
                                    *indxp.offset(
                                        (k2 as ::core::ffi::c_long + i__ as ::core::ffi::c_long
                                            - 1 as ::core::ffi::c_long)
                                            as isize,
                                    ) = jlam;
                                    break;
                                }
                            }
                            jlam = j;
                        } else {
                            *k += 1;
                            *w.offset(*k as isize) = *z__.offset(jlam as isize);
                            *dlamda.offset(*k as isize) = *d__.offset(jlam as isize);
                            *indxp.offset(*k as isize) = jlam;
                            jlam = j;
                        }
                    }
                    current_block = 946983070274721668;
                }
            }
        }
    }
    if *k < *n {
        if *icompq == 0 as ::core::ffi::c_long {
            i__1 = *n - *k;
            f2c_dcopy_0(
                &raw mut i__1,
                dlamda.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
                &raw mut c__1,
                d__.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
                &raw mut c__1,
            );
        } else {
            i__1 = *n - *k;
            f2c_dcopy_0(
                &raw mut i__1,
                dlamda.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
                &raw mut c__1,
                d__.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
                &raw mut c__1,
            );
            i__1 = *n - *k;
            dlacpy__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                qsiz,
                &raw mut i__1,
                q2.offset(
                    ((*k + 1 as ::core::ffi::c_long) * q2_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                ldq2,
                q.offset(
                    ((*k + 1 as ::core::ffi::c_long) * q_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                ldq,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
