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
static mut c__1: integer = 1 as integer;
static mut c__0: integer = 0 as integer;
static mut c_b13: doublereal = 1.0f64;
static mut c_b26: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dlasd3_(
    mut nl: *mut integer,
    mut nr: *mut integer,
    mut sqre: *mut integer,
    mut k: *mut integer,
    mut d__: *mut doublereal,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut dsigma: *mut doublereal,
    mut u: *mut doublereal,
    mut ldu: *mut integer,
    mut u2: *mut doublereal,
    mut ldu2: *mut integer,
    mut vt: *mut doublereal,
    mut ldvt: *mut integer,
    mut vt2: *mut doublereal,
    mut ldvt2: *mut integer,
    mut idxc: *mut integer,
    mut ctot: *mut integer,
    mut z__: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut u_dim1: integer = 0;
    let mut u_offset: integer = 0;
    let mut u2_dim1: integer = 0;
    let mut u2_offset: integer = 0;
    let mut vt_dim1: integer = 0;
    let mut vt_offset: integer = 0;
    let mut vt2_dim1: integer = 0;
    let mut vt2_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
    }
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut m: integer = 0;
    let mut n: integer = 0;
    let mut jc: integer = 0;
    let mut rho: doublereal = 0.;
    let mut nlp1: integer = 0;
    let mut nlp2: integer = 0;
    let mut nrp1: integer = 0;
    let mut temp: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dnrm2"]
        fn f2c_dnrm2_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> doublereal;
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
    let mut ctemp: integer = 0;
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
    let mut ktemp: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlamc3_"]
        fn dlamc3__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlasd4_"]
        fn dlasd4__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlascl_"]
        fn dlascl__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
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
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    d__ = d__.offset(-1);
    q_dim1 = *ldq;
    q_offset = 1 as integer + q_dim1;
    q = q.offset(-(q_offset as isize));
    dsigma = dsigma.offset(-1);
    u_dim1 = *ldu;
    u_offset = 1 as integer + u_dim1;
    u = u.offset(-(u_offset as isize));
    u2_dim1 = *ldu2;
    u2_offset = 1 as integer + u2_dim1;
    u2 = u2.offset(-(u2_offset as isize));
    vt_dim1 = *ldvt;
    vt_offset = 1 as integer + vt_dim1;
    vt = vt.offset(-(vt_offset as isize));
    vt2_dim1 = *ldvt2;
    vt2_offset = 1 as integer + vt2_dim1;
    vt2 = vt2.offset(-(vt2_offset as isize));
    idxc = idxc.offset(-1);
    ctot = ctot.offset(-1);
    z__ = z__.offset(-1);
    *info = 0 as integer;
    if *nl < 1 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *nr < 1 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *sqre != 1 as ::core::ffi::c_long && *sqre != 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    }
    n = (*nl + *nr + 1 as ::core::ffi::c_long) as integer;
    m = n + *sqre;
    nlp1 = (*nl + 1 as ::core::ffi::c_long) as integer;
    nlp2 = (*nl + 2 as ::core::ffi::c_long) as integer;
    if *k < 1 as ::core::ffi::c_long || *k > n {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *ldq < *k {
        *info = -(7 as ::core::ffi::c_int) as integer;
    } else if *ldu < n {
        *info = -(10 as ::core::ffi::c_int) as integer;
    } else if *ldu2 < n {
        *info = -(12 as ::core::ffi::c_int) as integer;
    } else if *ldvt < m {
        *info = -(14 as ::core::ffi::c_int) as integer;
    } else if *ldvt2 < m {
        *info = -(16 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLASD3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *k == 1 as ::core::ffi::c_long {
        *d__.offset(1 as ::core::ffi::c_int as isize) = (if *z__
            .offset(1 as ::core::ffi::c_int as isize)
            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *z__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
        } else {
            -(*z__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
        }) as doublereal;
        f2c_dcopy_0(
            &raw mut m,
            vt2.offset((vt2_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            ldvt2,
            vt.offset((vt_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            ldvt,
        );
        if *z__.offset(1 as ::core::ffi::c_int as isize) > 0.0f64 {
            f2c_dcopy_0(
                &raw mut n,
                u2.offset((u2_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
                u.offset((u_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
            );
        } else {
            i__1 = n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                *u.offset((i__ + u_dim1) as isize) = -*u2.offset((i__ + u2_dim1) as isize);
                i__ += 1;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    i__1 = *k;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *dsigma.offset(i__ as isize) = dlamc3__0(
            dsigma.offset(i__ as isize) as *mut doublereal,
            dsigma.offset(i__ as isize) as *mut doublereal,
        ) - *dsigma.offset(i__ as isize);
        i__ += 1;
    }
    f2c_dcopy_0(
        k,
        z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
        q.offset(q_offset as isize) as *mut doublereal,
        &raw mut c__1,
    );
    rho = f2c_dnrm2_0(
        k,
        z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
    );
    dlascl__0(
        b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut c__0,
        &raw mut c__0,
        &raw mut rho,
        &raw mut c_b13,
        k,
        &raw mut c__1,
        z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        k,
        info,
    );
    rho *= rho as ::core::ffi::c_double;
    i__1 = *k;
    j = 1 as integer;
    while j <= i__1 {
        dlasd4__0(
            k,
            &raw mut j,
            dsigma.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            z__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            u.offset(
                (j as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut rho,
            d__.offset(j as isize) as *mut doublereal,
            vt.offset(
                (j as ::core::ffi::c_long * vt_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            info,
        );
        if *info != 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        j += 1;
    }
    i__1 = *k;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *z__.offset(i__ as isize) =
            *u.offset((i__ + *k * u_dim1) as isize) * *vt.offset((i__ + *k * vt_dim1) as isize);
        i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        j = 1 as integer;
        while j <= i__2 {
            let ref mut fresh0 = *z__.offset(i__ as isize);
            *fresh0 *= (*u.offset((i__ + j * u_dim1) as isize)
                * *vt.offset((i__ + j * vt_dim1) as isize)
                / (*dsigma.offset(i__ as isize) - *dsigma.offset(j as isize))
                / (*dsigma.offset(i__ as isize) + *dsigma.offset(j as isize)))
                as ::core::ffi::c_double;
            j += 1;
        }
        i__2 = (*k - 1 as ::core::ffi::c_long) as integer;
        j = i__;
        while j <= i__2 {
            let ref mut fresh1 = *z__.offset(i__ as isize);
            *fresh1 *= (*u.offset((i__ + j * u_dim1) as isize)
                * *vt.offset((i__ + j * vt_dim1) as isize)
                / (*dsigma.offset(i__ as isize)
                    - *dsigma
                        .offset((j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize))
                / (*dsigma.offset(i__ as isize)
                    + *dsigma
                        .offset((j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)))
                as ::core::ffi::c_double;
            j += 1;
        }
        d__1 = *z__.offset(i__ as isize);
        d__2 = sqrt(
            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1
            } else {
                -d__1
            }),
        ) as doublereal;
        *z__.offset(i__ as isize) = d_sign_0(
            &raw mut d__2,
            q.offset((i__ + q_dim1) as isize) as *mut doublereal,
        ) as doublereal;
        i__ += 1;
    }
    i__1 = *k;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *vt.offset(
            (i__ as ::core::ffi::c_long * vt_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as isize,
        ) = *z__.offset(1 as ::core::ffi::c_int as isize)
            / *u.offset(
                (i__ as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            )
            / *vt.offset(
                (i__ as ::core::ffi::c_long * vt_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            );
        *u.offset(
            (i__ as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as isize,
        ) = -1.0f64 as doublereal;
        i__2 = *k;
        j = 2 as integer;
        while j <= i__2 {
            *vt.offset((j + i__ * vt_dim1) as isize) = *z__.offset(j as isize)
                / *u.offset((j + i__ * u_dim1) as isize)
                / *vt.offset((j + i__ * vt_dim1) as isize);
            *u.offset((j + i__ * u_dim1) as isize) =
                *dsigma.offset(j as isize) * *vt.offset((j + i__ * vt_dim1) as isize);
            j += 1;
        }
        temp = f2c_dnrm2_0(
            k,
            u.offset(
                (i__ as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
        );
        *q.offset(
            (i__ as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as isize,
        ) = *u.offset(
            (i__ as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as isize,
        ) / temp;
        i__2 = *k;
        j = 2 as integer;
        while j <= i__2 {
            jc = *idxc.offset(j as isize);
            *q.offset((j + i__ * q_dim1) as isize) = *u.offset((jc + i__ * u_dim1) as isize) / temp;
            j += 1;
        }
        i__ += 1;
    }
    if *k == 2 as ::core::ffi::c_long {
        f2c_dgemm_0(
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut n,
            k,
            k,
            &raw mut c_b13,
            u2.offset(u2_offset as isize) as *mut doublereal,
            ldu2,
            q.offset(q_offset as isize) as *mut doublereal,
            ldq,
            &raw mut c_b26,
            u.offset(u_offset as isize) as *mut doublereal,
            ldu,
        );
    } else {
        if *ctot.offset(1 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_long {
            f2c_dgemm_0(
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                nl,
                k,
                ctot.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                &raw mut c_b13,
                u2.offset(
                    (((u2_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                ldu2,
                q.offset((q_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldq,
                &raw mut c_b26,
                u.offset((u_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldu,
            );
            if *ctot.offset(3 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_long {
                ktemp = *ctot.offset(1 as ::core::ffi::c_int as isize)
                    + 2 as integer
                    + *ctot.offset(2 as ::core::ffi::c_int as isize);
                f2c_dgemm_0(
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    nl,
                    k,
                    ctot.offset(3 as ::core::ffi::c_int as isize) as *mut integer,
                    &raw mut c_b13,
                    u2.offset(
                        (ktemp as ::core::ffi::c_long * u2_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    ldu2,
                    q.offset((ktemp + q_dim1) as isize) as *mut doublereal,
                    ldq,
                    &raw mut c_b13,
                    u.offset((u_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                    ldu,
                );
            }
        } else if *ctot.offset(3 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_long {
            ktemp = *ctot.offset(1 as ::core::ffi::c_int as isize)
                + 2 as integer
                + *ctot.offset(2 as ::core::ffi::c_int as isize);
            f2c_dgemm_0(
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                nl,
                k,
                ctot.offset(3 as ::core::ffi::c_int as isize) as *mut integer,
                &raw mut c_b13,
                u2.offset(
                    (ktemp as ::core::ffi::c_long * u2_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                ldu2,
                q.offset((ktemp + q_dim1) as isize) as *mut doublereal,
                ldq,
                &raw mut c_b26,
                u.offset((u_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldu,
            );
        } else {
            dlacpy__0(
                b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                nl,
                k,
                u2.offset(u2_offset as isize) as *mut doublereal,
                ldu2,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
            );
        }
        f2c_dcopy_0(
            k,
            q.offset((q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            ldq,
            u.offset((nlp1 + u_dim1) as isize) as *mut doublereal,
            ldu,
        );
        ktemp = (*ctot.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as integer;
        ctemp = *ctot.offset(2 as ::core::ffi::c_int as isize)
            + *ctot.offset(3 as ::core::ffi::c_int as isize);
        f2c_dgemm_0(
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            nr,
            k,
            &raw mut ctemp,
            &raw mut c_b13,
            u2.offset((nlp2 + ktemp * u2_dim1) as isize) as *mut doublereal,
            ldu2,
            q.offset((ktemp + q_dim1) as isize) as *mut doublereal,
            ldq,
            &raw mut c_b26,
            u.offset((nlp2 + u_dim1) as isize) as *mut doublereal,
            ldu,
        );
    }
    i__1 = *k;
    i__ = 1 as integer;
    while i__ <= i__1 {
        temp = f2c_dnrm2_0(
            k,
            vt.offset(
                (i__ as ::core::ffi::c_long * vt_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
        );
        *q.offset((i__ + q_dim1) as isize) = *vt.offset(
            (i__ as ::core::ffi::c_long * vt_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as isize,
        ) / temp;
        i__2 = *k;
        j = 2 as integer;
        while j <= i__2 {
            jc = *idxc.offset(j as isize);
            *q.offset((i__ + j * q_dim1) as isize) =
                *vt.offset((jc + i__ * vt_dim1) as isize) / temp;
            j += 1;
        }
        i__ += 1;
    }
    if *k == 2 as ::core::ffi::c_long {
        f2c_dgemm_0(
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            k,
            &raw mut m,
            k,
            &raw mut c_b13,
            q.offset(q_offset as isize) as *mut doublereal,
            ldq,
            vt2.offset(vt2_offset as isize) as *mut doublereal,
            ldvt2,
            &raw mut c_b26,
            vt.offset(vt_offset as isize) as *mut doublereal,
            ldvt,
        );
        return 0 as ::core::ffi::c_int;
    }
    ktemp = (*ctot.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_long
        + 1 as ::core::ffi::c_long) as integer;
    f2c_dgemm_0(
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        k,
        &raw mut nlp1,
        &raw mut ktemp,
        &raw mut c_b13,
        q.offset((q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
            as *mut doublereal,
        ldq,
        vt2.offset((vt2_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
            as *mut doublereal,
        ldvt2,
        &raw mut c_b26,
        vt.offset((vt_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
            as *mut doublereal,
        ldvt,
    );
    ktemp = *ctot.offset(1 as ::core::ffi::c_int as isize)
        + 2 as integer
        + *ctot.offset(2 as ::core::ffi::c_int as isize);
    if ktemp <= *ldvt2 {
        f2c_dgemm_0(
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            k,
            &raw mut nlp1,
            ctot.offset(3 as ::core::ffi::c_int as isize) as *mut integer,
            &raw mut c_b13,
            q.offset(
                (ktemp as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            ldq,
            vt2.offset((ktemp + vt2_dim1) as isize) as *mut doublereal,
            ldvt2,
            &raw mut c_b13,
            vt.offset((vt_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            ldvt,
        );
    }
    ktemp = (*ctot.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_long
        + 1 as ::core::ffi::c_long) as integer;
    nrp1 = *nr + *sqre;
    if ktemp > 1 as ::core::ffi::c_long {
        i__1 = *k;
        i__ = 1 as integer;
        while i__ <= i__1 {
            *q.offset((i__ + ktemp * q_dim1) as isize) = *q.offset((i__ + q_dim1) as isize);
            i__ += 1;
        }
        i__1 = m;
        i__ = nlp2;
        while i__ <= i__1 {
            *vt2.offset((ktemp + i__ * vt2_dim1) as isize) = *vt2.offset(
                (i__ as ::core::ffi::c_long * vt2_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            );
            i__ += 1;
        }
    }
    ctemp = *ctot.offset(2 as ::core::ffi::c_int as isize)
        + 1 as integer
        + *ctot.offset(3 as ::core::ffi::c_int as isize);
    f2c_dgemm_0(
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        k,
        &raw mut nrp1,
        &raw mut ctemp,
        &raw mut c_b13,
        q.offset(
            (ktemp as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as isize,
        ) as *mut doublereal,
        ldq,
        vt2.offset((ktemp + nlp2 * vt2_dim1) as isize) as *mut doublereal,
        ldvt2,
        &raw mut c_b26,
        vt.offset(
            (nlp2 as ::core::ffi::c_long * vt_dim1 as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as isize,
        ) as *mut doublereal,
        ldvt,
    );
    return 0 as ::core::ffi::c_int;
}
