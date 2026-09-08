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
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaed1_(
    mut n: *mut integer,
    mut d__: *mut doublereal,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut indxq: *mut integer,
    mut rho: *mut doublereal,
    mut cutpnt: *mut integer,
    mut work: *mut doublereal,
    mut iwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__: integer = 0;
    let mut k: integer = 0;
    let mut n1: integer = 0;
    let mut n2: integer = 0;
    let mut is: integer = 0;
    let mut iw: integer = 0;
    let mut iz: integer = 0;
    let mut iq2: integer = 0;
    let mut zpp1: integer = 0;
    let mut indx: integer = 0;
    let mut indxc: integer = 0;
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
    let mut indxp: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_dlaed2_"]
        fn dlaed2__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_dlaed3_"]
        fn dlaed3__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
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
    work = work.offset(-1);
    iwork = iwork.offset(-1);
    *info = 0 as integer;
    if *n < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *ldq
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else {
        i__1 = 1 as integer;
        i__2 = (*n / 2 as ::core::ffi::c_long) as integer;
        if (if i__1 <= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) > *cutpnt
            || (*n / 2 as ::core::ffi::c_long) < *cutpnt
        {
            *info = -(7 as ::core::ffi::c_int) as integer;
        }
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLAED1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    iz = 1 as integer;
    idlmda = iz + *n;
    iw = idlmda + *n;
    iq2 = iw + *n;
    indx = 1 as integer;
    indxc = indx + *n;
    coltyp = indxc + *n;
    indxp = coltyp + *n;
    f2c_dcopy_0(
        cutpnt,
        q.offset((*cutpnt + q_dim1) as isize) as *mut doublereal,
        ldq,
        work.offset(iz as isize) as *mut doublereal,
        &raw mut c__1,
    );
    zpp1 = (*cutpnt + 1 as ::core::ffi::c_long) as integer;
    i__1 = *n - *cutpnt;
    f2c_dcopy_0(
        &raw mut i__1,
        q.offset((zpp1 + zpp1 * q_dim1) as isize) as *mut doublereal,
        ldq,
        work.offset((iz + *cutpnt) as isize) as *mut doublereal,
        &raw mut c__1,
    );
    dlaed2__0(
        &raw mut k,
        n,
        cutpnt,
        d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        q.offset(q_offset as isize) as *mut doublereal,
        ldq,
        indxq.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
        rho,
        work.offset(iz as isize) as *mut doublereal,
        work.offset(idlmda as isize) as *mut doublereal,
        work.offset(iw as isize) as *mut doublereal,
        work.offset(iq2 as isize) as *mut doublereal,
        iwork.offset(indx as isize) as *mut integer,
        iwork.offset(indxc as isize) as *mut integer,
        iwork.offset(indxp as isize) as *mut integer,
        iwork.offset(coltyp as isize) as *mut integer,
        info,
    );
    if !(*info != 0 as ::core::ffi::c_long) {
        if k != 0 as ::core::ffi::c_long {
            is = (*iwork.offset(coltyp as isize)
                + *iwork
                    .offset((coltyp as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize))
                * *cutpnt
                + (*iwork
                    .offset((coltyp as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    + *iwork.offset(
                        (coltyp as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize,
                    ))
                    * (*n - *cutpnt)
                + iq2;
            dlaed3__0(
                &raw mut k,
                n,
                cutpnt,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                q.offset(q_offset as isize) as *mut doublereal,
                ldq,
                rho,
                work.offset(idlmda as isize) as *mut doublereal,
                work.offset(iq2 as isize) as *mut doublereal,
                iwork.offset(indxc as isize) as *mut integer,
                iwork.offset(coltyp as isize) as *mut integer,
                work.offset(iw as isize) as *mut doublereal,
                work.offset(is as isize) as *mut doublereal,
                info,
            );
            if !(*info != 0 as ::core::ffi::c_long) {
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
            i__1 = *n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                *indxq.offset(i__ as isize) = i__;
                i__ += 1;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
