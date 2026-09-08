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
static mut c__0: integer = 0 as integer;
static mut c_b7: doublereal = 1.0f64;
static mut c__1: integer = 1 as integer;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dlasd1_(
    mut nl: *mut integer,
    mut nr: *mut integer,
    mut sqre: *mut integer,
    mut d__: *mut doublereal,
    mut alpha: *mut doublereal,
    mut beta: *mut doublereal,
    mut u: *mut doublereal,
    mut ldu: *mut integer,
    mut vt: *mut doublereal,
    mut ldvt: *mut integer,
    mut idxq: *mut integer,
    mut iwork: *mut integer,
    mut work: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut u_dim1: integer = 0;
    let mut u_offset: integer = 0;
    let mut vt_dim1: integer = 0;
    let mut vt_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut i__: integer = 0;
    let mut k: integer = 0;
    let mut m: integer = 0;
    let mut n: integer = 0;
    let mut n1: integer = 0;
    let mut n2: integer = 0;
    let mut iq: integer = 0;
    let mut iz: integer = 0;
    let mut iu2: integer = 0;
    let mut ldq: integer = 0;
    let mut idx: integer = 0;
    let mut ldu2: integer = 0;
    let mut ivt2: integer = 0;
    let mut idxc: integer = 0;
    let mut idxp: integer = 0;
    let mut ldvt2: integer = 0;
    extern "C" {
        #[link_name = "dgesdd_closure_dlasd2_"]
        fn dlasd2__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgesdd_closure_dlasd3_"]
        fn dlasd3__0(
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
            _: *mut doublereal,
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
    let mut isigma: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut orgnrm: doublereal = 0.;
    let mut coltyp: integer = 0;
    d__ = d__.offset(-1);
    u_dim1 = *ldu;
    u_offset = 1 as integer + u_dim1;
    u = u.offset(-(u_offset as isize));
    vt_dim1 = *ldvt;
    vt_offset = 1 as integer + vt_dim1;
    vt = vt.offset(-(vt_offset as isize));
    idxq = idxq.offset(-1);
    iwork = iwork.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    if *nl < 1 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *nr < 1 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *sqre < 0 as ::core::ffi::c_long || *sqre > 1 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLASD1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    n = (*nl + *nr + 1 as ::core::ffi::c_long) as integer;
    m = n + *sqre;
    ldu2 = n;
    ldvt2 = m;
    iz = 1 as integer;
    isigma = iz + m;
    iu2 = isigma + n;
    ivt2 = iu2 + ldu2 * n;
    iq = ivt2 + ldvt2 * m;
    idx = 1 as integer;
    idxc = idx + n;
    coltyp = idxc + n;
    idxp = coltyp + n;
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
    orgnrm = (if d__1 >= d__2 {
        d__1 as ::core::ffi::c_double
    } else {
        d__2 as ::core::ffi::c_double
    }) as doublereal;
    *d__.offset((*nl + 1 as ::core::ffi::c_long) as isize) = 0.0f64 as doublereal;
    i__1 = n;
    i__ = 1 as integer;
    while i__ <= i__1 {
        d__1 = *d__.offset(i__ as isize);
        if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        }) > orgnrm
        {
            d__1 = *d__.offset(i__ as isize);
            orgnrm = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) as doublereal;
        }
        i__ += 1;
    }
    dlascl__0(
        b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut c__0,
        &raw mut c__0,
        &raw mut orgnrm,
        &raw mut c_b7,
        &raw mut n,
        &raw mut c__1,
        d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut n,
        info,
    );
    *alpha /= orgnrm as ::core::ffi::c_double;
    *beta /= orgnrm as ::core::ffi::c_double;
    dlasd2__0(
        nl,
        nr,
        sqre,
        &raw mut k,
        d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        work.offset(iz as isize) as *mut doublereal,
        alpha,
        beta,
        u.offset(u_offset as isize) as *mut doublereal,
        ldu,
        vt.offset(vt_offset as isize) as *mut doublereal,
        ldvt,
        work.offset(isigma as isize) as *mut doublereal,
        work.offset(iu2 as isize) as *mut doublereal,
        &raw mut ldu2,
        work.offset(ivt2 as isize) as *mut doublereal,
        &raw mut ldvt2,
        iwork.offset(idxp as isize) as *mut integer,
        iwork.offset(idx as isize) as *mut integer,
        iwork.offset(idxc as isize) as *mut integer,
        idxq.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
        iwork.offset(coltyp as isize) as *mut integer,
        info,
    );
    ldq = k;
    dlasd3__0(
        nl,
        nr,
        sqre,
        &raw mut k,
        d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        work.offset(iq as isize) as *mut doublereal,
        &raw mut ldq,
        work.offset(isigma as isize) as *mut doublereal,
        u.offset(u_offset as isize) as *mut doublereal,
        ldu,
        work.offset(iu2 as isize) as *mut doublereal,
        &raw mut ldu2,
        vt.offset(vt_offset as isize) as *mut doublereal,
        ldvt,
        work.offset(ivt2 as isize) as *mut doublereal,
        &raw mut ldvt2,
        iwork.offset(idxc as isize) as *mut integer,
        iwork.offset(coltyp as isize) as *mut integer,
        work.offset(iz as isize) as *mut doublereal,
        info,
    );
    if *info != 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    dlascl__0(
        b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut c__0,
        &raw mut c__0,
        &raw mut c_b7,
        &raw mut orgnrm,
        &raw mut n,
        &raw mut c__1,
        d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut n,
        info,
    );
    n1 = k;
    n2 = n - k;
    dlamrg__0(
        &raw mut n1,
        &raw mut n2,
        d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut c__1,
        &raw mut c_n1,
        idxq.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
    );
    return 0 as ::core::ffi::c_int;
}
