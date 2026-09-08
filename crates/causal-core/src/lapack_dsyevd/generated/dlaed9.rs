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
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaed9_(
    mut k: *mut integer,
    mut kstart: *mut integer,
    mut kstop: *mut integer,
    mut n: *mut integer,
    mut d__: *mut doublereal,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut rho: *mut doublereal,
    mut dlamda: *mut doublereal,
    mut w: *mut doublereal,
    mut s: *mut doublereal,
    mut lds: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut s_dim1: integer = 0;
    let mut s_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
    }
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut temp: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dnrm2"]
        fn f2c_dnrm2_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> doublereal;
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
        #[link_name = "dsyevd_closure_dlaed4_"]
        fn dlaed4__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamc3_"]
        fn dlamc3__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    d__ = d__.offset(-1);
    q_dim1 = *ldq;
    q_offset = 1 as integer + q_dim1;
    q = q.offset(-(q_offset as isize));
    dlamda = dlamda.offset(-1);
    w = w.offset(-1);
    s_dim1 = *lds;
    s_offset = 1 as integer + s_dim1;
    s = s.offset(-(s_offset as isize));
    *info = 0 as integer;
    if *k < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *kstart < 1 as ::core::ffi::c_long
        || *kstart
            > (if 1 as ::core::ffi::c_long >= *k {
                1 as ::core::ffi::c_long
            } else {
                *k
            })
    {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if (if 1 as ::core::ffi::c_long >= *kstop {
        1 as ::core::ffi::c_long
    } else {
        *kstop
    }) < *kstart
        || *kstop
            > (if 1 as ::core::ffi::c_long >= *k {
                1 as ::core::ffi::c_long
            } else {
                *k
            })
    {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *n < *k {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *ldq
        < (if 1 as ::core::ffi::c_long >= *k {
            1 as ::core::ffi::c_long
        } else {
            *k
        })
    {
        *info = -(7 as ::core::ffi::c_int) as integer;
    } else if *lds
        < (if 1 as ::core::ffi::c_long >= *k {
            1 as ::core::ffi::c_long
        } else {
            *k
        })
    {
        *info = -(12 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLAED9\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *k == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    i__1 = *n;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *dlamda.offset(i__ as isize) = dlamc3__0(
            dlamda.offset(i__ as isize) as *mut doublereal,
            dlamda.offset(i__ as isize) as *mut doublereal,
        ) - *dlamda.offset(i__ as isize);
        i__ += 1;
    }
    i__1 = *kstop;
    j = *kstart;
    loop {
        if !(j <= i__1) {
            current_block = 980989089337379490;
            break;
        }
        dlaed4__0(
            k,
            &raw mut j,
            dlamda.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            w.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            q.offset(
                (j as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            rho,
            d__.offset(j as isize) as *mut doublereal,
            info,
        );
        if *info != 0 as ::core::ffi::c_long {
            current_block = 3228386634285890927;
            break;
        }
        j += 1;
    }
    match current_block {
        980989089337379490 => {
            if *k == 1 as ::core::ffi::c_long || *k == 2 as ::core::ffi::c_long {
                i__1 = *k;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    i__2 = *k;
                    j = 1 as integer;
                    while j <= i__2 {
                        *s.offset((j + i__ * s_dim1) as isize) =
                            *q.offset((j + i__ * q_dim1) as isize);
                        j += 1;
                    }
                    i__ += 1;
                }
            } else {
                f2c_dcopy_0(
                    k,
                    w.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c__1,
                    s.offset(s_offset as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__1 = (*ldq + 1 as ::core::ffi::c_long) as integer;
                f2c_dcopy_0(
                    k,
                    q.offset(q_offset as isize) as *mut doublereal,
                    &raw mut i__1,
                    w.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__1 = *k;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        let ref mut fresh0 = *w.offset(i__ as isize);
                        *fresh0 *= (*q.offset((i__ + j * q_dim1) as isize)
                            / (*dlamda.offset(i__ as isize) - *dlamda.offset(j as isize)))
                            as ::core::ffi::c_double;
                        i__ += 1;
                    }
                    i__2 = *k;
                    i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    while i__ <= i__2 {
                        let ref mut fresh1 = *w.offset(i__ as isize);
                        *fresh1 *= (*q.offset((i__ + j * q_dim1) as isize)
                            / (*dlamda.offset(i__ as isize) - *dlamda.offset(j as isize)))
                            as ::core::ffi::c_double;
                        i__ += 1;
                    }
                    j += 1;
                }
                i__1 = *k;
                i__ = 1 as integer;
                while i__ <= i__1 {
                    d__1 = sqrt(-*w.offset(i__ as isize)) as doublereal;
                    *w.offset(i__ as isize) = d_sign_0(
                        &raw mut d__1,
                        s.offset((i__ + s_dim1) as isize) as *mut doublereal,
                    ) as doublereal;
                    i__ += 1;
                }
                i__1 = *k;
                j = 1 as integer;
                while j <= i__1 {
                    i__2 = *k;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *q.offset((i__ + j * q_dim1) as isize) =
                            *w.offset(i__ as isize) / *q.offset((i__ + j * q_dim1) as isize);
                        i__ += 1;
                    }
                    temp = f2c_dnrm2_0(
                        k,
                        q.offset(
                            (j as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    i__2 = *k;
                    i__ = 1 as integer;
                    while i__ <= i__2 {
                        *s.offset((i__ + j * s_dim1) as isize) =
                            *q.offset((i__ + j * q_dim1) as isize) / temp;
                        i__ += 1;
                    }
                    j += 1;
                }
            }
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
