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
pub type logical = ::core::ffi::c_long;
static mut c__9: integer = 9 as integer;
static mut c__0: integer = 0 as integer;
static mut c_b15: doublereal = 1.0f64;
static mut c__1: integer = 1 as integer;
static mut c_b29: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dbdsdc_(
    mut uplo: *mut ::core::ffi::c_char,
    mut compq: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut u: *mut doublereal,
    mut ldu: *mut integer,
    mut vt: *mut doublereal,
    mut ldvt: *mut integer,
    mut q: *mut doublereal,
    mut iq: *mut integer,
    mut work: *mut doublereal,
    mut iwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut u_dim1: integer = 0;
    let mut u_offset: integer = 0;
    let mut vt_dim1: integer = 0;
    let mut vt_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
    }
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut p: doublereal = 0.;
    let mut r__: doublereal = 0.;
    let mut z__: integer = 0;
    let mut ic: integer = 0;
    let mut ii: integer = 0;
    let mut kk: integer = 0;
    let mut cs: doublereal = 0.;
    let mut is: integer = 0;
    let mut iu: integer = 0;
    let mut sn: doublereal = 0.;
    let mut nm1: integer = 0;
    let mut eps: doublereal = 0.;
    let mut ivt: integer = 0;
    let mut difl: integer = 0;
    let mut difr: integer = 0;
    let mut ierr: integer = 0;
    let mut perm: integer = 0;
    let mut mlvl: integer = 0;
    let mut sqre: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlasr_"]
        fn dlasr__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
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
        #[link_name = "dgelsd_closure_f2c_dswap"]
        fn f2c_dswap_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut poles: integer = 0;
    let mut iuplo: integer = 0;
    let mut nsize: integer = 0;
    let mut start: integer = 0;
    extern "C" {
        #[link_name = "dgesdd_closure_dlasd0_"]
        fn dlasd0__0(
            _: *mut integer,
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
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlasda_"]
        fn dlasda__0(
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
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
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
        #[link_name = "dgelsd_closure_dlartg_"]
        fn dlartg__0(
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
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
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut givcol: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_dlanst_"]
        fn dlanst__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> doublereal;
    }
    let mut icompq: integer = 0;
    let mut orgnrm: doublereal = 0.;
    let mut givnum: integer = 0;
    let mut givptr: integer = 0;
    let mut qstart: integer = 0;
    let mut smlsiz: integer = 0;
    let mut wstart: integer = 0;
    let mut smlszp: integer = 0;
    d__ = d__.offset(-1);
    e = e.offset(-1);
    u_dim1 = *ldu;
    u_offset = 1 as integer + u_dim1;
    u = u.offset(-(u_offset as isize));
    vt_dim1 = *ldvt;
    vt_offset = 1 as integer + vt_dim1;
    vt = vt.offset(-(vt_offset as isize));
    q = q.offset(-1);
    iq = iq.offset(-1);
    work = work.offset(-1);
    iwork = iwork.offset(-1);
    *info = 0 as integer;
    iuplo = 0 as integer;
    if lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        iuplo = 1 as integer;
    }
    if lsame__0(
        uplo,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        iuplo = 2 as integer;
    }
    if lsame__0(
        compq,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        icompq = 0 as integer;
    } else if lsame__0(
        compq,
        b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        icompq = 1 as integer;
    } else if lsame__0(
        compq,
        b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        icompq = 2 as integer;
    } else {
        icompq = -(1 as ::core::ffi::c_int) as integer;
    }
    if iuplo == 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if icompq < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *ldu < 1 as ::core::ffi::c_long || icompq == 2 as ::core::ffi::c_long && *ldu < *n {
        *info = -(7 as ::core::ffi::c_int) as integer;
    } else if *ldvt < 1 as ::core::ffi::c_long || icompq == 2 as ::core::ffi::c_long && *ldvt < *n {
        *info = -(9 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DBDSDC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    smlsiz = ilaenv__0(
        &raw mut c__9,
        b"DBDSDC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut c__0,
        &raw mut c__0,
        &raw mut c__0,
        &raw mut c__0,
    );
    if *n == 1 as ::core::ffi::c_long {
        if icompq == 1 as ::core::ffi::c_long {
            *q.offset(1 as ::core::ffi::c_int as isize) = d_sign_0(
                &raw mut c_b15,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            ) as doublereal;
            *q.offset((smlsiz as ::core::ffi::c_long * *n + 1 as ::core::ffi::c_long) as isize) =
                1.0f64 as doublereal;
        } else if icompq == 2 as ::core::ffi::c_long {
            *u.offset((u_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                d_sign_0(
                    &raw mut c_b15,
                    d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                ) as doublereal;
            *vt.offset((vt_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                1.0f64 as doublereal;
        }
        *d__.offset(1 as ::core::ffi::c_int as isize) = (if *d__
            .offset(1 as ::core::ffi::c_int as isize)
            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            *d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
        } else {
            -(*d__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
        }) as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    nm1 = (*n - 1 as ::core::ffi::c_long) as integer;
    wstart = 1 as integer;
    qstart = 3 as integer;
    if icompq == 1 as ::core::ffi::c_long {
        f2c_dcopy_0(
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            q.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
        f2c_dcopy_0(
            &raw mut i__1,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            q.offset((*n + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            &raw mut c__1,
        );
    }
    if iuplo == 2 as ::core::ffi::c_long {
        qstart = 5 as integer;
        wstart = ((*n << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long) as integer;
        i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
        i__ = 1 as integer;
        while i__ <= i__1 {
            dlartg__0(
                d__.offset(i__ as isize) as *mut doublereal,
                e.offset(i__ as isize) as *mut doublereal,
                &raw mut cs,
                &raw mut sn,
                &raw mut r__,
            );
            *d__.offset(i__ as isize) = r__;
            *e.offset(i__ as isize) =
                sn * *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                cs * *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            if icompq == 1 as ::core::ffi::c_long {
                *q.offset((i__ + (*n << 1 as ::core::ffi::c_int)) as isize) = cs;
                *q.offset((i__ as ::core::ffi::c_long + *n * 3 as ::core::ffi::c_long) as isize) =
                    sn;
            } else if icompq == 2 as ::core::ffi::c_long {
                *work.offset(i__ as isize) = cs;
                *work.offset((nm1 + i__) as isize) = -sn;
            }
            i__ += 1;
        }
    }
    if icompq == 0 as ::core::ffi::c_long {
        dlasdq__0(
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            n,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut c__0,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            vt.offset(vt_offset as isize) as *mut doublereal,
            ldvt,
            u.offset(u_offset as isize) as *mut doublereal,
            ldu,
            u.offset(u_offset as isize) as *mut doublereal,
            ldu,
            work.offset(wstart as isize) as *mut doublereal,
            info,
        );
    } else if *n <= smlsiz {
        if icompq == 2 as ::core::ffi::c_long {
            dlaset__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b29,
                &raw mut c_b15,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
            );
            dlaset__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b29,
                &raw mut c_b15,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
            );
            dlasdq__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                n,
                n,
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
                work.offset(wstart as isize) as *mut doublereal,
                info,
            );
        } else if icompq == 1 as ::core::ffi::c_long {
            iu = 1 as integer;
            ivt = iu + *n;
            dlaset__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b29,
                &raw mut c_b15,
                q.offset((iu + (qstart - 1 as integer) * *n) as isize) as *mut doublereal,
                n,
            );
            dlaset__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b29,
                &raw mut c_b15,
                q.offset((ivt + (qstart - 1 as integer) * *n) as isize) as *mut doublereal,
                n,
            );
            dlasdq__0(
                b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut c__0,
                n,
                n,
                n,
                &raw mut c__0,
                d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                q.offset((ivt + (qstart - 1 as integer) * *n) as isize) as *mut doublereal,
                n,
                q.offset((iu + (qstart - 1 as integer) * *n) as isize) as *mut doublereal,
                n,
                q.offset((iu + (qstart - 1 as integer) * *n) as isize) as *mut doublereal,
                n,
                work.offset(wstart as isize) as *mut doublereal,
                info,
            );
        }
    } else {
        if icompq == 2 as ::core::ffi::c_long {
            dlaset__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b29,
                &raw mut c_b15,
                u.offset(u_offset as isize) as *mut doublereal,
                ldu,
            );
            dlaset__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                n,
                n,
                &raw mut c_b29,
                &raw mut c_b15,
                vt.offset(vt_offset as isize) as *mut doublereal,
                ldvt,
            );
        }
        orgnrm = dlanst__0(
            b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        );
        if orgnrm == 0.0f64 {
            return 0 as ::core::ffi::c_int;
        }
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut orgnrm,
            &raw mut c_b15,
            n,
            &raw mut c__1,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            n,
            &raw mut ierr,
        );
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut orgnrm,
            &raw mut c_b15,
            &raw mut nm1,
            &raw mut c__1,
            e.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut nm1,
            &raw mut ierr,
        );
        eps = dlamch__0(
            b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        mlvl = ((log(*n as doublereal
            / (smlsiz as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as doublereal)
            / log(2.0f64)) as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as integer;
        smlszp = (smlsiz as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        if icompq == 1 as ::core::ffi::c_long {
            iu = 1 as integer;
            ivt = (smlsiz as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            difl = ivt + smlszp;
            difr = difl + mlvl;
            z__ = difr + (mlvl << 1 as ::core::ffi::c_int);
            ic = z__ + mlvl;
            is = (ic as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            poles = (is as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            givnum = poles + (mlvl << 1 as ::core::ffi::c_int);
            k = 1 as integer;
            givptr = 2 as integer;
            perm = 3 as integer;
            givcol = perm + mlvl;
        }
        i__1 = *n;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = *d__.offset(i__ as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) < eps
            {
                *d__.offset(i__ as isize) =
                    d_sign_0(&raw mut eps, d__.offset(i__ as isize) as *mut doublereal)
                        as doublereal;
            }
            i__ += 1;
        }
        start = 1 as integer;
        sqre = 0 as integer;
        i__1 = nm1;
        i__ = 1 as integer;
        while i__ <= i__1 {
            d__1 = *e.offset(i__ as isize);
            if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                d__1 as ::core::ffi::c_double
            } else {
                -(d__1 as ::core::ffi::c_double)
            }) < eps
                || i__ == nm1
            {
                if i__ < nm1 {
                    nsize = (i__ as ::core::ffi::c_long - start as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                } else {
                    d__1 = *e.offset(i__ as isize);
                    if (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) >= eps
                    {
                        nsize = (*n - start as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            as integer;
                    } else {
                        nsize = (i__ as ::core::ffi::c_long - start as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as integer;
                        if icompq == 2 as ::core::ffi::c_long {
                            *u.offset((*n + *n * u_dim1) as isize) = d_sign_0(
                                &raw mut c_b15,
                                d__.offset(*n as isize) as *mut doublereal,
                            )
                                as doublereal;
                            *vt.offset((*n + *n * vt_dim1) as isize) = 1.0f64 as doublereal;
                        } else if icompq == 1 as ::core::ffi::c_long {
                            *q.offset((*n + (qstart - 1 as integer) * *n) as isize) = d_sign_0(
                                &raw mut c_b15,
                                d__.offset(*n as isize) as *mut doublereal,
                            )
                                as doublereal;
                            *q.offset((*n + (smlsiz + qstart - 1 as integer) * *n) as isize) =
                                1.0f64 as doublereal;
                        }
                        d__1 = *d__.offset(*n as isize);
                        *d__.offset(*n as isize) =
                            (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                d__1 as ::core::ffi::c_double
                            } else {
                                -(d__1 as ::core::ffi::c_double)
                            }) as doublereal;
                    }
                }
                if icompq == 2 as ::core::ffi::c_long {
                    dlasd0__0(
                        &raw mut nsize,
                        &raw mut sqre,
                        d__.offset(start as isize) as *mut doublereal,
                        e.offset(start as isize) as *mut doublereal,
                        u.offset((start + start * u_dim1) as isize) as *mut doublereal,
                        ldu,
                        vt.offset((start + start * vt_dim1) as isize) as *mut doublereal,
                        ldvt,
                        &raw mut smlsiz,
                        iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                        work.offset(wstart as isize) as *mut doublereal,
                        info,
                    );
                } else {
                    dlasda__0(
                        &raw mut icompq,
                        &raw mut smlsiz,
                        &raw mut nsize,
                        &raw mut sqre,
                        d__.offset(start as isize) as *mut doublereal,
                        e.offset(start as isize) as *mut doublereal,
                        q.offset((start + (iu + qstart - 2 as integer) * *n) as isize)
                            as *mut doublereal,
                        n,
                        q.offset((start + (ivt + qstart - 2 as integer) * *n) as isize)
                            as *mut doublereal,
                        iq.offset((start + k * *n) as isize) as *mut integer,
                        q.offset((start + (difl + qstart - 2 as integer) * *n) as isize)
                            as *mut doublereal,
                        q.offset((start + (difr + qstart - 2 as integer) * *n) as isize)
                            as *mut doublereal,
                        q.offset((start + (z__ + qstart - 2 as integer) * *n) as isize)
                            as *mut doublereal,
                        q.offset((start + (poles + qstart - 2 as integer) * *n) as isize)
                            as *mut doublereal,
                        iq.offset((start + givptr * *n) as isize) as *mut integer,
                        iq.offset((start + givcol * *n) as isize) as *mut integer,
                        n,
                        iq.offset((start + perm * *n) as isize) as *mut integer,
                        q.offset((start + (givnum + qstart - 2 as integer) * *n) as isize)
                            as *mut doublereal,
                        q.offset((start + (ic + qstart - 2 as integer) * *n) as isize)
                            as *mut doublereal,
                        q.offset((start + (is + qstart - 2 as integer) * *n) as isize)
                            as *mut doublereal,
                        work.offset(wstart as isize) as *mut doublereal,
                        iwork.offset(1 as ::core::ffi::c_int as isize) as *mut integer,
                        info,
                    );
                    if *info != 0 as ::core::ffi::c_long {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                start = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            }
            i__ += 1;
        }
        dlascl__0(
            b"G\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut c__0,
            &raw mut c__0,
            &raw mut c_b15,
            &raw mut orgnrm,
            n,
            &raw mut c__1,
            d__.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            n,
            &raw mut ierr,
        );
    }
    i__1 = *n;
    ii = 2 as integer;
    while ii <= i__1 {
        i__ = (ii as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        kk = i__;
        p = *d__.offset(i__ as isize);
        i__2 = *n;
        j = ii;
        while j <= i__2 {
            if *d__.offset(j as isize) > p {
                kk = j;
                p = *d__.offset(j as isize);
            }
            j += 1;
        }
        if kk != i__ {
            *d__.offset(kk as isize) = *d__.offset(i__ as isize);
            *d__.offset(i__ as isize) = p;
            if icompq == 1 as ::core::ffi::c_long {
                *iq.offset(i__ as isize) = kk;
            } else if icompq == 2 as ::core::ffi::c_long {
                f2c_dswap_0(
                    n,
                    u.offset(
                        (i__ as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    u.offset(
                        (kk as ::core::ffi::c_long * u_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                f2c_dswap_0(
                    n,
                    vt.offset((i__ + vt_dim1) as isize) as *mut doublereal,
                    ldvt,
                    vt.offset((kk + vt_dim1) as isize) as *mut doublereal,
                    ldvt,
                );
            }
        } else if icompq == 1 as ::core::ffi::c_long {
            *iq.offset(i__ as isize) = i__;
        }
        ii += 1;
    }
    if icompq == 1 as ::core::ffi::c_long {
        if iuplo == 1 as ::core::ffi::c_long {
            *iq.offset(*n as isize) = 1 as integer;
        } else {
            *iq.offset(*n as isize) = 0 as integer;
        }
    }
    if iuplo == 2 as ::core::ffi::c_long && icompq == 2 as ::core::ffi::c_long {
        dlasr__0(
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"V\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            n,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(*n as isize) as *mut doublereal,
            u.offset(u_offset as isize) as *mut doublereal,
            ldu,
        );
    }
    return 0 as ::core::ffi::c_int;
}
