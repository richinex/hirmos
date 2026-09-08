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
static mut c__2: integer = 2 as integer;
static mut c__1: integer = 1 as integer;
static mut c_b10: doublereal = 1.0f64;
static mut c_b11: doublereal = 0.0f64;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaed7_(
    mut icompq: *mut integer,
    mut n: *mut integer,
    mut qsiz: *mut integer,
    mut tlvls: *mut integer,
    mut curlvl: *mut integer,
    mut curpbm: *mut integer,
    mut d__: *mut doublereal,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut indxq: *mut integer,
    mut rho: *mut doublereal,
    mut cutpnt: *mut integer,
    mut qstore: *mut doublereal,
    mut qptr: *mut integer,
    mut prmptr: *mut integer,
    mut perm: *mut integer,
    mut givptr: *mut integer,
    mut givcol: *mut integer,
    mut givnum: *mut doublereal,
    mut work: *mut doublereal,
    mut iwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_pow_ii"]
        fn pow_ii_0(_: *mut integer, _: *mut integer) -> integer;
    }
    let mut i__: integer = 0;
    let mut k: integer = 0;
    let mut n1: integer = 0;
    let mut n2: integer = 0;
    let mut is: integer = 0;
    let mut iw: integer = 0;
    let mut iz: integer = 0;
    let mut iq2: integer = 0;
    let mut ptr: integer = 0;
    let mut ldq2: integer = 0;
    let mut indx: integer = 0;
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
    let mut indxc: integer = 0;
    let mut indxp: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dlaed8_"]
        fn dlaed8__0(
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
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_dlaed9_"]
        fn dlaed9__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_dlaeda_"]
        fn dlaeda__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut idlmda: integer = 0;
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
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut coltyp: integer = 0;
    d__ = d__.offset(-1);
    q_dim1 = *ldq;
    q_offset = 1 as integer + q_dim1;
    q = q.offset(-(q_offset as isize));
    indxq = indxq.offset(-1);
    qstore = qstore.offset(-1);
    qptr = qptr.offset(-1);
    prmptr = prmptr.offset(-1);
    perm = perm.offset(-1);
    givptr = givptr.offset(-1);
    givcol = givcol.offset(-(3 as ::core::ffi::c_int as isize));
    givnum = givnum.offset(-(3 as ::core::ffi::c_int as isize));
    work = work.offset(-1);
    iwork = iwork.offset(-1);
    *info = 0 as integer;
    if *icompq < 0 as ::core::ffi::c_long || *icompq > 1 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *icompq == 1 as ::core::ffi::c_long && *qsiz < *n {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *ldq
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(9 as ::core::ffi::c_int) as integer;
    } else if (if 1 as ::core::ffi::c_long <= *n {
        1 as ::core::ffi::c_long
    } else {
        *n
    }) > *cutpnt
        || *n < *cutpnt
    {
        *info = -(12 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLAED7\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *icompq == 1 as ::core::ffi::c_long {
        ldq2 = *qsiz;
    } else {
        ldq2 = *n;
    }
    iz = 1 as integer;
    idlmda = iz + *n;
    iw = idlmda + *n;
    iq2 = iw + *n;
    is = iq2 + *n * ldq2;
    indx = 1 as integer;
    indxc = indx + *n;
    coltyp = indxc + *n;
    indxp = coltyp + *n;
    ptr = (pow_ii_0(&raw mut c__2, tlvls) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
        as integer;
    i__1 = (*curlvl - 1 as ::core::ffi::c_long) as integer;
    i__ = 1 as integer;
    while i__ <= i__1 {
        i__2 = *tlvls - i__;
        ptr += pow_ii_0(&raw mut c__2, &raw mut i__2) as ::core::ffi::c_long;
        i__ += 1;
    }
    curr = ptr + *curpbm;
    dlaeda__0(
        n,
        tlvls,
        curlvl,
        curpbm,
        prmptr.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
        perm.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
        givptr.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
        givcol.offset(3 as ::core::ffi::c_int as isize) as *mut integer,
        givnum.offset(3 as ::core::ffi::c_int as isize) as *mut doublereal,
        qstore.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        qptr.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
        work.offset(iz as isize) as *mut doublereal,
        work.offset((iz + *n) as isize) as *mut doublereal,
        info,
    );
    if *curlvl == *tlvls {
        *qptr.offset(curr as isize) = 1 as integer;
        *prmptr.offset(curr as isize) = 1 as integer;
        *givptr.offset(curr as isize) = 1 as integer;
    }
    dlaed8__0(
        icompq,
        &raw mut k,
        n,
        qsiz,
        d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        q.offset(q_offset as isize) as *mut doublereal,
        ldq,
        indxq.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
        rho,
        cutpnt,
        work.offset(iz as isize) as *mut doublereal,
        work.offset(idlmda as isize) as *mut doublereal,
        work.offset(iq2 as isize) as *mut doublereal,
        &raw mut ldq2,
        work.offset(iw as isize) as *mut doublereal,
        perm.offset(*prmptr.offset(curr as isize) as isize) as *mut integer,
        givptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
            as *mut integer,
        givcol.offset(
            (((*givptr.offset(curr as isize) as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_long) as isize,
        ) as *mut integer,
        givnum.offset(
            (((*givptr.offset(curr as isize) as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_long) as isize,
        ) as *mut doublereal,
        iwork.offset(indxp as isize) as *mut integer,
        iwork.offset(indx as isize) as *mut integer,
        info,
    );
    *prmptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
        *prmptr.offset(curr as isize) + *n;
    let ref mut fresh0 =
        *givptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
    *fresh0 += *givptr.offset(curr as isize) as ::core::ffi::c_long;
    if k != 0 as ::core::ffi::c_long {
        dlaed9__0(
            &raw mut k,
            &raw mut c__1,
            &raw mut k,
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(is as isize) as *mut doublereal,
            &raw mut k,
            rho,
            work.offset(idlmda as isize) as *mut doublereal,
            work.offset(iw as isize) as *mut doublereal,
            qstore.offset(*qptr.offset(curr as isize) as isize) as *mut doublereal,
            &raw mut k,
            info,
        );
        if !(*info != 0 as ::core::ffi::c_long) {
            if *icompq == 1 as ::core::ffi::c_long {
                f2c_dgemm_0(
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    qsiz,
                    &raw mut k,
                    &raw mut k,
                    &raw mut c_b10,
                    work.offset(iq2 as isize) as *mut doublereal,
                    &raw mut ldq2,
                    qstore.offset(*qptr.offset(curr as isize) as isize) as *mut doublereal,
                    &raw mut k,
                    &raw mut c_b11,
                    q.offset(q_offset as isize) as *mut doublereal,
                    ldq,
                );
            }
            i__1 = k;
            *qptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                *qptr.offset(curr as isize) + i__1 * i__1;
            n1 = k;
            n2 = *n - k;
            dlamrg__0(
                &raw mut n1,
                &raw mut n2,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut c_n1,
                indxq.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
            );
        }
    } else {
        *qptr.offset((curr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
            *qptr.offset(curr as isize);
        i__1 = *n;
        i__ = 1 as integer;
        while i__ <= i__1 {
            *indxq.offset(i__ as isize) = i__;
            i__ += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
