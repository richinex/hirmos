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
static mut c__1: integer = 1 as integer;
static mut c_b30: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dlasd2_(
    mut nl: *mut integer,
    mut nr: *mut integer,
    mut sqre: *mut integer,
    mut k: *mut integer,
    mut d__: *mut doublereal,
    mut z__: *mut doublereal,
    mut alpha: *mut doublereal,
    mut beta: *mut doublereal,
    mut u: *mut doublereal,
    mut ldu: *mut integer,
    mut vt: *mut doublereal,
    mut ldvt: *mut integer,
    mut dsigma: *mut doublereal,
    mut u2: *mut doublereal,
    mut ldu2: *mut integer,
    mut vt2: *mut doublereal,
    mut ldvt2: *mut integer,
    mut idxp: *mut integer,
    mut idx: *mut integer,
    mut idxc: *mut integer,
    mut idxq: *mut integer,
    mut coltyp: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut u_dim1: integer = 0;
    let mut u_offset: integer = 0;
    let mut u2_dim1: integer = 0;
    let mut u2_offset: integer = 0;
    let mut vt_dim1: integer = 0;
    let mut vt_offset: integer = 0;
    let mut vt2_dim1: integer = 0;
    let mut vt2_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut c__: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut m: integer = 0;
    let mut n: integer = 0;
    let mut s: doublereal = 0.;
    let mut k2: integer = 0;
    let mut z1: doublereal = 0.;
    let mut ct: integer = 0;
    let mut jp: integer = 0;
    let mut eps: doublereal = 0.;
    let mut tau: doublereal = 0.;
    let mut tol: doublereal = 0.;
    let mut psm: [integer; 4] = [0; 4];
    let mut nlp1: integer = 0;
    let mut nlp2: integer = 0;
    let mut idxi: integer = 0;
    let mut idxj: integer = 0;
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
    let mut idxjp: integer = 0;
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
    let mut jprev: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlapy2_"]
        fn dlapy2__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
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
        #[link_name = "dgelsd_closure_dlaset_"]
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
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut hlftol: doublereal = 0.;
    d__ = d__.offset(-1);
    z__ = z__.offset(-1);
    u_dim1 = *ldu;
    u_offset = 1 as integer + u_dim1;
    u = u.offset(-(u_offset as isize));
    vt_dim1 = *ldvt;
    vt_offset = 1 as integer + vt_dim1;
    vt = vt.offset(-(vt_offset as isize));
    dsigma = dsigma.offset(-1);
    u2_dim1 = *ldu2;
    u2_offset = 1 as integer + u2_dim1;
    u2 = u2.offset(-(u2_offset as isize));
    vt2_dim1 = *ldvt2;
    vt2_offset = 1 as integer + vt2_dim1;
    vt2 = vt2.offset(-(vt2_offset as isize));
    idxp = idxp.offset(-1);
    idx = idx.offset(-1);
    idxc = idxc.offset(-1);
    idxq = idxq.offset(-1);
    coltyp = coltyp.offset(-1);
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
    if *ldu < n {
        *info = -(10 as ::core::ffi::c_int) as integer;
    } else if *ldvt < m {
        *info = -(12 as ::core::ffi::c_int) as integer;
    } else if *ldu2 < n {
        *info = -(15 as ::core::ffi::c_int) as integer;
    } else if *ldvt2 < m {
        *info = -(17 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLASD2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    nlp1 = (*nl + 1 as ::core::ffi::c_long) as integer;
    nlp2 = (*nl + 2 as ::core::ffi::c_long) as integer;
    z1 = *alpha * *vt.offset((nlp1 + nlp1 * vt_dim1) as isize);
    *z__.offset(1 as ::core::ffi::c_int as isize) = z1;
    i__ = *nl;
    while i__ >= 1 as ::core::ffi::c_long {
        *z__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
            *alpha * *vt.offset((i__ + nlp1 * vt_dim1) as isize);
        *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
            *d__.offset(i__ as isize);
        *idxq.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
            (*idxq.offset(i__ as isize) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as integer;
        i__ -= 1;
    }
    i__1 = m;
    i__ = nlp2;
    while i__ <= i__1 {
        *z__.offset(i__ as isize) = *beta * *vt.offset((i__ + nlp2 * vt_dim1) as isize);
        i__ += 1;
    }
    i__1 = nlp1;
    i__ = 2 as integer;
    while i__ <= i__1 {
        *coltyp.offset(i__ as isize) = 1 as integer;
        i__ += 1;
    }
    i__1 = n;
    i__ = nlp2;
    while i__ <= i__1 {
        *coltyp.offset(i__ as isize) = 2 as integer;
        i__ += 1;
    }
    i__1 = n;
    i__ = nlp2;
    while i__ <= i__1 {
        let ref mut fresh0 = *idxq.offset(i__ as isize);
        *fresh0 += nlp1 as ::core::ffi::c_long;
        i__ += 1;
    }
    i__1 = n;
    i__ = 2 as integer;
    while i__ <= i__1 {
        *dsigma.offset(i__ as isize) = *d__.offset(*idxq.offset(i__ as isize) as isize);
        *u2.offset((i__ + u2_dim1) as isize) = *z__.offset(*idxq.offset(i__ as isize) as isize);
        *idxc.offset(i__ as isize) = *coltyp.offset(*idxq.offset(i__ as isize) as isize);
        i__ += 1;
    }
    dlamrg__0(
        nl,
        nr,
        dsigma.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
        &raw mut c__1,
        idx.offset(2 as ::core::ffi::c_int as isize) as *mut integer,
    );
    i__1 = n;
    i__ = 2 as integer;
    while i__ <= i__1 {
        idxi = (*idx.offset(i__ as isize) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
            as integer;
        *d__.offset(i__ as isize) = *dsigma.offset(idxi as isize);
        *z__.offset(i__ as isize) = *u2.offset((idxi + u2_dim1) as isize);
        *coltyp.offset(i__ as isize) = *idxc.offset(idxi as isize);
        i__ += 1;
    }
    eps = dlamch__0(
        b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    d__1 = (if *alpha >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *alpha
    } else {
        -*alpha
    }) as doublereal;
    d__2 = (if *beta >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *beta
    } else {
        -*beta
    }) as doublereal;
    tol = (if d__1 >= d__2 {
        d__1 as ::core::ffi::c_double
    } else {
        d__2 as ::core::ffi::c_double
    }) as doublereal;
    d__1 = *d__.offset(n as isize);
    d__2 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        d__1 as ::core::ffi::c_double
    } else {
        -(d__1 as ::core::ffi::c_double)
    }) as doublereal;
    tol = (eps as ::core::ffi::c_double
        * 8.0f64
        * (if d__2 >= tol {
            d__2 as ::core::ffi::c_double
        } else {
            tol as ::core::ffi::c_double
        })) as doublereal;
    *k = 1 as integer;
    k2 = (n as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    i__1 = n;
    j = 2 as integer;
    loop {
        if !(j <= i__1) {
            current_block = 1586866904657518693;
            break;
        }
        d__1 = *z__.offset(j as isize);
        if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        }) <= tol
        {
            k2 -= 1;
            *idxp.offset(k2 as isize) = j;
            *coltyp.offset(j as isize) = 4 as integer;
            if j == n {
                current_block = 13733519225236719591;
                break;
            }
            j += 1;
        } else {
            jprev = j;
            current_block = 1586866904657518693;
            break;
        }
    }
    match current_block {
        1586866904657518693 => {
            j = jprev;
            loop {
                j += 1;
                if j > n {
                    break;
                }
                d__1 = *z__.offset(j as isize);
                if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1 as ::core::ffi::c_double
                } else {
                    -(d__1 as ::core::ffi::c_double)
                }) <= tol
                {
                    k2 -= 1;
                    *idxp.offset(k2 as isize) = j;
                    *coltyp.offset(j as isize) = 4 as integer;
                } else {
                    d__1 = *d__.offset(j as isize) - *d__.offset(jprev as isize);
                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) <= tol
                    {
                        s = *z__.offset(jprev as isize);
                        c__ = *z__.offset(j as isize);
                        tau = dlapy2__0(&raw mut c__, &raw mut s);
                        c__ /= tau as ::core::ffi::c_double;
                        s = -s / tau;
                        *z__.offset(j as isize) = tau;
                        *z__.offset(jprev as isize) = 0.0f64 as doublereal;
                        idxjp = *idxq.offset(
                            (*idx.offset(jprev as isize) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        );
                        idxj = *idxq.offset(
                            (*idx.offset(j as isize) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        );
                        if idxjp <= nlp1 {
                            idxjp -= 1;
                        }
                        if idxj <= nlp1 {
                            idxj -= 1;
                        }
                        f2c_drot_0(
                            &raw mut n,
                            u.offset(
                                (idxjp as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                            u.offset(
                                (idxj as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                                    + 1 as ::core::ffi::c_long)
                                    as isize,
                            ) as *mut doublereal,
                            &raw mut c__1,
                            &raw mut c__,
                            &raw mut s,
                        );
                        f2c_drot_0(
                            &raw mut m,
                            vt.offset((idxjp + vt_dim1) as isize) as *mut doublereal,
                            ldvt,
                            vt.offset((idxj + vt_dim1) as isize) as *mut doublereal,
                            ldvt,
                            &raw mut c__,
                            &raw mut s,
                        );
                        if *coltyp.offset(j as isize) != *coltyp.offset(jprev as isize) {
                            *coltyp.offset(j as isize) = 3 as integer;
                        }
                        *coltyp.offset(jprev as isize) = 4 as integer;
                        k2 -= 1;
                        *idxp.offset(k2 as isize) = jprev;
                        jprev = j;
                    } else {
                        *k += 1;
                        *u2.offset((*k + u2_dim1) as isize) = *z__.offset(jprev as isize);
                        *dsigma.offset(*k as isize) = *d__.offset(jprev as isize);
                        *idxp.offset(*k as isize) = jprev;
                        jprev = j;
                    }
                }
            }
            *k += 1;
            *u2.offset((*k + u2_dim1) as isize) = *z__.offset(jprev as isize);
            *dsigma.offset(*k as isize) = *d__.offset(jprev as isize);
            *idxp.offset(*k as isize) = jprev;
        }
        _ => {}
    }
    j = 1 as integer;
    while j <= 4 as ::core::ffi::c_long {
        ctot[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] = 0 as integer;
        j += 1;
    }
    i__1 = n;
    j = 2 as integer;
    while j <= i__1 {
        ct = *coltyp.offset(j as isize);
        ctot[(ct as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] += 1;
        j += 1;
    }
    psm[0 as ::core::ffi::c_int as usize] = 2 as integer;
    psm[1 as ::core::ffi::c_int as usize] =
        (ctot[0 as ::core::ffi::c_int as usize] + 2 as ::core::ffi::c_long) as integer;
    psm[2 as ::core::ffi::c_int as usize] =
        psm[1 as ::core::ffi::c_int as usize] + ctot[1 as ::core::ffi::c_int as usize];
    psm[3 as ::core::ffi::c_int as usize] =
        psm[2 as ::core::ffi::c_int as usize] + ctot[2 as ::core::ffi::c_int as usize];
    i__1 = n;
    j = 2 as integer;
    while j <= i__1 {
        jp = *idxp.offset(j as isize);
        ct = *coltyp.offset(jp as isize);
        *idxc.offset(
            psm[(ct as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] as isize,
        ) = j;
        psm[(ct as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize] += 1;
        j += 1;
    }
    i__1 = n;
    j = 2 as integer;
    while j <= i__1 {
        jp = *idxp.offset(j as isize);
        *dsigma.offset(j as isize) = *d__.offset(jp as isize);
        idxj = *idxq.offset(
            (*idx.offset(*idxp.offset(*idxc.offset(j as isize) as isize) as isize)
                as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as isize,
        );
        if idxj <= nlp1 {
            idxj -= 1;
        }
        f2c_dcopy_0(
            &raw mut n,
            u.offset(
                (idxj as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
            u2.offset(
                (j as ::core::ffi::c_long * u2_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
        );
        f2c_dcopy_0(
            &raw mut m,
            vt.offset((idxj + vt_dim1) as isize) as *mut doublereal,
            ldvt,
            vt2.offset((j + vt2_dim1) as isize) as *mut doublereal,
            ldvt2,
        );
        j += 1;
    }
    *dsigma.offset(1 as ::core::ffi::c_int as isize) = 0.0f64 as doublereal;
    hlftol = (tol as ::core::ffi::c_double / 2.0f64) as doublereal;
    if (if *dsigma.offset(2 as ::core::ffi::c_int as isize)
        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        *dsigma.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
    } else {
        -(*dsigma.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
    }) <= hlftol
    {
        *dsigma.offset(2 as ::core::ffi::c_int as isize) = hlftol;
    }
    if m > n {
        *z__.offset(1 as ::core::ffi::c_int as isize) =
            dlapy2__0(&raw mut z1, z__.offset(m as isize) as *mut doublereal);
        if *z__.offset(1 as ::core::ffi::c_int as isize) <= tol {
            c__ = 1.0f64 as doublereal;
            s = 0.0f64 as doublereal;
            *z__.offset(1 as ::core::ffi::c_int as isize) = tol;
        } else {
            c__ = z1 / *z__.offset(1 as ::core::ffi::c_int as isize);
            s = *z__.offset(m as isize) / *z__.offset(1 as ::core::ffi::c_int as isize);
        }
    } else if (if z1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        z1 as ::core::ffi::c_double
    } else {
        -(z1 as ::core::ffi::c_double)
    }) <= tol
    {
        *z__.offset(1 as ::core::ffi::c_int as isize) = tol;
    } else {
        *z__.offset(1 as ::core::ffi::c_int as isize) = z1;
    }
    i__1 = (*k - 1 as ::core::ffi::c_long) as integer;
    f2c_dcopy_0(
        &raw mut i__1,
        u2.offset((u2_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
            as *mut doublereal,
        &raw mut c__1,
        z__.offset(2 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
    );
    dlaset__0(
        b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut n,
        &raw mut c__1,
        &raw mut c_b30,
        &raw mut c_b30,
        u2.offset(u2_offset as isize) as *mut doublereal,
        ldu2,
    );
    *u2.offset((nlp1 + u2_dim1) as isize) = 1.0f64 as doublereal;
    if m > n {
        i__1 = nlp1;
        i__ = 1 as integer;
        while i__ <= i__1 {
            *vt.offset((m + i__ * vt_dim1) as isize) =
                -s * *vt.offset((nlp1 + i__ * vt_dim1) as isize);
            *vt2.offset(
                (i__ as ::core::ffi::c_long * vt2_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) = c__ * *vt.offset((nlp1 + i__ * vt_dim1) as isize);
            i__ += 1;
        }
        i__1 = m;
        i__ = nlp2;
        while i__ <= i__1 {
            *vt2.offset(
                (i__ as ::core::ffi::c_long * vt2_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) = s * *vt.offset((m + i__ * vt_dim1) as isize);
            *vt.offset((m + i__ * vt_dim1) as isize) =
                c__ * *vt.offset((m + i__ * vt_dim1) as isize);
            i__ += 1;
        }
    } else {
        f2c_dcopy_0(
            &raw mut m,
            vt.offset((nlp1 + vt_dim1) as isize) as *mut doublereal,
            ldvt,
            vt2.offset((vt2_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            ldvt2,
        );
    }
    if m > n {
        f2c_dcopy_0(
            &raw mut m,
            vt.offset((m + vt_dim1) as isize) as *mut doublereal,
            ldvt,
            vt2.offset((m + vt2_dim1) as isize) as *mut doublereal,
            ldvt2,
        );
    }
    if n > *k {
        i__1 = n - *k;
        f2c_dcopy_0(
            &raw mut i__1,
            dsigma.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            &raw mut c__1,
            d__.offset((*k + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            &raw mut c__1,
        );
        i__1 = n - *k;
        dlacpy__0(
            b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut n,
            &raw mut i__1,
            u2.offset(
                ((*k + 1 as ::core::ffi::c_long) * u2_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            ldu2,
            u.offset(
                ((*k + 1 as ::core::ffi::c_long) * u_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            ldu,
        );
        i__1 = n - *k;
        dlacpy__0(
            b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
            &raw mut m,
            vt2.offset((*k + 1 as integer + vt2_dim1) as isize) as *mut doublereal,
            ldvt2,
            vt.offset((*k + 1 as integer + vt_dim1) as isize) as *mut doublereal,
            ldvt,
        );
    }
    j = 1 as integer;
    while j <= 4 as ::core::ffi::c_long {
        *coltyp.offset(j as isize) =
            ctot[(j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as usize];
        j += 1;
    }
    return 0 as ::core::ffi::c_int;
}
