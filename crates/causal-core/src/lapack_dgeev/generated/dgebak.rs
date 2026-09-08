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
pub type logical = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dgebak_(
    mut job: *mut ::core::ffi::c_char,
    mut side: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut ilo: *mut integer,
    mut ihi: *mut integer,
    mut scale: *mut doublereal,
    mut m: *mut integer,
    mut v: *mut doublereal,
    mut ldv: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut v_dim1: integer = 0;
    let mut v_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__: integer = 0;
    let mut k: integer = 0;
    let mut s: doublereal = 0.;
    let mut ii: integer = 0;
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
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
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
    let mut leftv: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    let mut rightv: logical = 0;
    scale = scale.offset(-1);
    v_dim1 = *ldv;
    v_offset = 1 as integer + v_dim1;
    v = v.offset(-(v_offset as isize));
    rightv = lsame__0(
        side,
        b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    leftv = lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    *info = 0 as integer;
    if lsame__0(
        job,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && lsame__0(
            job,
            b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
        && lsame__0(
            job,
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
        && lsame__0(
            job,
            b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if rightv == 0 && leftv == 0 {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *ilo < 1 as ::core::ffi::c_long
        || *ilo
            > (if 1 as ::core::ffi::c_long >= *n {
                1 as ::core::ffi::c_long
            } else {
                *n
            })
    {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *ihi < (if *ilo <= *n { *ilo } else { *n }) || *ihi > *n {
        *info = -(5 as ::core::ffi::c_int) as integer;
    } else if *m < 0 as ::core::ffi::c_long {
        *info = -(7 as ::core::ffi::c_int) as integer;
    } else if *ldv
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
            b"DGEBAK\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *m == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if lsame__0(
        job,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*ilo == *ihi) {
        if lsame__0(
            job,
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
            || lsame__0(
                job,
                b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) != 0
        {
            if rightv != 0 {
                i__1 = *ihi;
                i__ = *ilo;
                while i__ <= i__1 {
                    s = *scale.offset(i__ as isize);
                    f2c_dscal_0(
                        m,
                        &raw mut s,
                        v.offset((i__ + v_dim1) as isize) as *mut doublereal,
                        ldv,
                    );
                    i__ += 1;
                }
            }
            if leftv != 0 {
                i__1 = *ihi;
                i__ = *ilo;
                while i__ <= i__1 {
                    s = 1.0f64 / *scale.offset(i__ as isize);
                    f2c_dscal_0(
                        m,
                        &raw mut s,
                        v.offset((i__ + v_dim1) as isize) as *mut doublereal,
                        ldv,
                    );
                    i__ += 1;
                }
            }
        }
    }
    if lsame__0(
        job,
        b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
        || lsame__0(
            job,
            b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
    {
        if rightv != 0 {
            i__1 = *n;
            ii = 1 as integer;
            while ii <= i__1 {
                i__ = ii;
                if !(i__ >= *ilo && i__ <= *ihi) {
                    if i__ < *ilo {
                        i__ = *ilo - ii;
                    }
                    k = *scale.offset(i__ as isize) as integer;
                    if !(k == i__) {
                        f2c_dswap_0(
                            m,
                            v.offset((i__ + v_dim1) as isize) as *mut doublereal,
                            ldv,
                            v.offset((k + v_dim1) as isize) as *mut doublereal,
                            ldv,
                        );
                    }
                }
                ii += 1;
            }
        }
        if leftv != 0 {
            i__1 = *n;
            ii = 1 as integer;
            while ii <= i__1 {
                i__ = ii;
                if !(i__ >= *ilo && i__ <= *ihi) {
                    if i__ < *ilo {
                        i__ = *ilo - ii;
                    }
                    k = *scale.offset(i__ as isize) as integer;
                    if !(k == i__) {
                        f2c_dswap_0(
                            m,
                            v.offset((i__ + v_dim1) as isize) as *mut doublereal,
                            ldv,
                            v.offset((k + v_dim1) as isize) as *mut doublereal,
                            ldv,
                        );
                    }
                }
                ii += 1;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
