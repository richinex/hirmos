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
static mut c__2: integer = 2 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dlasd0_(
    mut n: *mut integer,
    mut sqre: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut u: *mut doublereal,
    mut ldu: *mut integer,
    mut vt: *mut doublereal,
    mut ldvt: *mut integer,
    mut smlsiz: *mut integer,
    mut iwork: *mut integer,
    mut work: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut u_dim1: integer = 0;
    let mut u_offset: integer = 0;
    let mut vt_dim1: integer = 0;
    let mut vt_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_pow_ii"]
        fn pow_ii_0(_: *mut integer, _: *mut integer) -> integer;
    }
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut m: integer = 0;
    let mut i1: integer = 0;
    let mut ic: integer = 0;
    let mut lf: integer = 0;
    let mut nd: integer = 0;
    let mut ll: integer = 0;
    let mut nl: integer = 0;
    let mut nr: integer = 0;
    let mut im1: integer = 0;
    let mut ncc: integer = 0;
    let mut nlf: integer = 0;
    let mut nrf: integer = 0;
    let mut iwk: integer = 0;
    let mut lvl: integer = 0;
    let mut ndb1: integer = 0;
    let mut nlp1: integer = 0;
    let mut nrp1: integer = 0;
    let mut beta: doublereal = 0.;
    let mut idxq: integer = 0;
    let mut nlvl: integer = 0;
    let mut alpha: doublereal = 0.;
    let mut inode: integer = 0;
    let mut ndiml: integer = 0;
    let mut idxqc: integer = 0;
    let mut ndimr: integer = 0;
    let mut itemp: integer = 0;
    let mut sqrei: integer = 0;
    extern "C" {
        #[link_name = "dgesdd_closure_dlasd1_"]
        fn dlasd1__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
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
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlasdq_"]
        fn dlasdq__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlasdt_"]
        fn dlasdt__0(
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
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    d__ = d__.offset(-1);
    e = e.offset(-1);
    u_dim1 = *ldu;
    u_offset = 1 as integer + u_dim1;
    u = u.offset(-(u_offset as isize));
    vt_dim1 = *ldvt;
    vt_offset = 1 as integer + vt_dim1;
    vt = vt.offset(-(vt_offset as isize));
    iwork = iwork.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    if *n < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *sqre < 0 as ::core::ffi::c_long || *sqre > 1 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    }
    m = *n + *sqre;
    if *ldu < *n {
        *info = -(6 as ::core::ffi::c_int) as integer;
    } else if *ldvt < m {
        *info = -(8 as ::core::ffi::c_int) as integer;
    } else if *smlsiz < 3 as ::core::ffi::c_long {
        *info = -(9 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLASD0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n <= *smlsiz {
        dlasdq__0(
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            sqre,
            n,
            &raw mut m,
            n,
            &raw mut c__0,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            vt.offset(vt_offset as isize) as *mut doublereal,
            ldvt,
            u.offset(u_offset as isize) as *mut doublereal,
            ldu,
            u.offset(u_offset as isize) as *mut doublereal,
            ldu,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            info,
        );
        return 0 as ::core::ffi::c_int;
    }
    inode = 1 as integer;
    ndiml = inode + *n;
    ndimr = ndiml + *n;
    idxq = ndimr + *n;
    iwk = idxq + *n;
    dlasdt__0(
        n,
        &raw mut nlvl,
        &raw mut nd,
        iwork.offset(inode as isize) as *mut integer,
        iwork.offset(ndiml as isize) as *mut integer,
        iwork.offset(ndimr as isize) as *mut integer,
        smlsiz,
    );
    ndb1 = ((nd as ::core::ffi::c_long + 1 as ::core::ffi::c_long) / 2 as ::core::ffi::c_long)
        as integer;
    ncc = 0 as integer;
    i__1 = nd;
    i__ = ndb1;
    while i__ <= i__1 {
        i1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        ic = *iwork.offset((inode + i1) as isize);
        nl = *iwork.offset((ndiml + i1) as isize);
        nlp1 = (nl as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        nr = *iwork.offset((ndimr + i1) as isize);
        nrp1 = (nr as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        nlf = ic - nl;
        nrf = (ic as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        sqrei = 1 as integer;
        dlasdq__0(
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut sqrei,
            &raw mut nl,
            &raw mut nlp1,
            &raw mut nl,
            &raw mut ncc,
            d__.offset(nlf as isize) as *mut doublereal,
            e.offset(nlf as isize) as *mut doublereal,
            vt.offset((nlf + nlf * vt_dim1) as isize) as *mut doublereal,
            ldvt,
            u.offset((nlf + nlf * u_dim1) as isize) as *mut doublereal,
            ldu,
            u.offset((nlf + nlf * u_dim1) as isize) as *mut doublereal,
            ldu,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            info,
        );
        if *info != 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        itemp = (idxq as ::core::ffi::c_long + nlf as ::core::ffi::c_long
            - 2 as ::core::ffi::c_long) as integer;
        i__2 = nl;
        j = 1 as integer;
        while j <= i__2 {
            *iwork.offset((itemp + j) as isize) = j;
            j += 1;
        }
        if i__ == nd {
            sqrei = *sqre;
        } else {
            sqrei = 1 as integer;
        }
        nrp1 = nr + sqrei;
        dlasdq__0(
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut sqrei,
            &raw mut nr,
            &raw mut nrp1,
            &raw mut nr,
            &raw mut ncc,
            d__.offset(nrf as isize) as *mut doublereal,
            e.offset(nrf as isize) as *mut doublereal,
            vt.offset((nrf + nrf * vt_dim1) as isize) as *mut doublereal,
            ldvt,
            u.offset((nrf + nrf * u_dim1) as isize) as *mut doublereal,
            ldu,
            u.offset((nrf + nrf * u_dim1) as isize) as *mut doublereal,
            ldu,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            info,
        );
        if *info != 0 as ::core::ffi::c_long {
            return 0 as ::core::ffi::c_int;
        }
        itemp = idxq + ic;
        i__2 = nr;
        j = 1 as integer;
        while j <= i__2 {
            *iwork.offset(
                (itemp as ::core::ffi::c_long + j as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                    as isize,
            ) = j;
            j += 1;
        }
        i__ += 1;
    }
    lvl = nlvl;
    while lvl >= 1 as ::core::ffi::c_long {
        if lvl == 1 as ::core::ffi::c_long {
            lf = 1 as integer;
            ll = 1 as integer;
        } else {
            i__1 = (lvl as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            lf = pow_ii_0(&raw mut c__2, &raw mut i__1);
            ll = (((lf as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_long) as integer;
        }
        i__1 = ll;
        i__ = lf;
        while i__ <= i__1 {
            im1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            ic = *iwork.offset((inode + im1) as isize);
            nl = *iwork.offset((ndiml + im1) as isize);
            nr = *iwork.offset((ndimr + im1) as isize);
            nlf = ic - nl;
            if *sqre == 0 as ::core::ffi::c_long && i__ == ll {
                sqrei = *sqre;
            } else {
                sqrei = 1 as integer;
            }
            idxqc = (idxq as ::core::ffi::c_long + nlf as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as integer;
            alpha = *d__.offset(ic as isize);
            beta = *e.offset(ic as isize);
            dlasd1__0(
                &raw mut nl,
                &raw mut nr,
                &raw mut sqrei,
                d__.offset(nlf as isize) as *mut doublereal,
                &raw mut alpha,
                &raw mut beta,
                u.offset((nlf + nlf * u_dim1) as isize) as *mut doublereal,
                ldu,
                vt.offset((nlf + nlf * vt_dim1) as isize) as *mut doublereal,
                ldvt,
                iwork.offset(idxqc as isize) as *mut integer,
                iwork.offset(iwk as isize) as *mut integer,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                info,
            );
            if *info != 0 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
            i__ += 1;
        }
        lvl -= 1;
    }
    return 0 as ::core::ffi::c_int;
}
