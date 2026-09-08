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
static mut c_b22: doublereal = 1.0f64;
static mut c_b23: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaed3_(
    mut k: *mut integer,
    mut n: *mut integer,
    mut n1: *mut integer,
    mut d__: *mut doublereal,
    mut q: *mut doublereal,
    mut ldq: *mut integer,
    mut rho: *mut doublereal,
    mut dlamda: *mut doublereal,
    mut q2: *mut doublereal,
    mut indx: *mut integer,
    mut ctot: *mut integer,
    mut w: *mut doublereal,
    mut s: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut q_dim1: integer = 0;
    let mut q_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
    }
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut n2: integer = 0;
    let mut n12: integer = 0;
    let mut ii: integer = 0;
    let mut n23: integer = 0;
    let mut iq2: integer = 0;
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
    d__ = d__.offset(-1);
    q_dim1 = *ldq;
    q_offset = 1 as integer + q_dim1;
    q = q.offset(-(q_offset as isize));
    dlamda = dlamda.offset(-1);
    q2 = q2.offset(-1);
    indx = indx.offset(-1);
    ctot = ctot.offset(-1);
    w = w.offset(-1);
    s = s.offset(-1);
    *info = 0 as integer;
    if *k < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < *k {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *ldq
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(6 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DLAED3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *k == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    i__1 = *k;
    i__ = 1 as integer;
    while i__ <= i__1 {
        *dlamda.offset(i__ as isize) = dlamc3__0(
            dlamda.offset(i__ as isize) as *mut doublereal,
            dlamda.offset(i__ as isize) as *mut doublereal,
        ) - *dlamda.offset(i__ as isize);
        i__ += 1;
    }
    i__1 = *k;
    j = 1 as integer;
    loop {
        if !(j <= i__1) {
            current_block = 652864300344834934;
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
            current_block = 16913121278416602805;
            break;
        }
        j += 1;
    }
    match current_block {
        652864300344834934 => {
            if !(*k == 1 as ::core::ffi::c_long) {
                if *k == 2 as ::core::ffi::c_long {
                    i__1 = *k;
                    j = 1 as integer;
                    while j <= i__1 {
                        *w.offset(1 as ::core::ffi::c_int as isize) = *q.offset(
                            (j as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        );
                        *w.offset(2 as ::core::ffi::c_int as isize) = *q.offset(
                            (j as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        );
                        ii = *indx.offset(1 as ::core::ffi::c_int as isize);
                        *q.offset(
                            (j as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) = *w.offset(ii as isize);
                        ii = *indx.offset(2 as ::core::ffi::c_int as isize);
                        *q.offset(
                            (j as ::core::ffi::c_long * q_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        ) = *w.offset(ii as isize);
                        j += 1;
                    }
                } else {
                    f2c_dcopy_0(
                        k,
                        w.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        &raw mut c__1,
                        s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
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
                        *w.offset(i__ as isize) =
                            d_sign_0(&raw mut d__1, s.offset(i__ as isize) as *mut doublereal)
                                as doublereal;
                        i__ += 1;
                    }
                    i__1 = *k;
                    j = 1 as integer;
                    while j <= i__1 {
                        i__2 = *k;
                        i__ = 1 as integer;
                        while i__ <= i__2 {
                            *s.offset(i__ as isize) =
                                *w.offset(i__ as isize) / *q.offset((i__ + j * q_dim1) as isize);
                            i__ += 1;
                        }
                        temp = f2c_dnrm2_0(
                            k,
                            s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                            &raw mut c__1,
                        );
                        i__2 = *k;
                        i__ = 1 as integer;
                        while i__ <= i__2 {
                            ii = *indx.offset(i__ as isize);
                            *q.offset((i__ + j * q_dim1) as isize) = *s.offset(ii as isize) / temp;
                            i__ += 1;
                        }
                        j += 1;
                    }
                }
            }
            n2 = *n - *n1;
            n12 = *ctot.offset(1 as ::core::ffi::c_int as isize)
                + *ctot.offset(2 as ::core::ffi::c_int as isize);
            n23 = *ctot.offset(2 as ::core::ffi::c_int as isize)
                + *ctot.offset(3 as ::core::ffi::c_int as isize);
            dlacpy__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut n23,
                k,
                q.offset(
                    (*ctot.offset(1 as ::core::ffi::c_int as isize) + 1 as integer + q_dim1)
                        as isize,
                ) as *mut doublereal,
                ldq,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut n23,
            );
            iq2 = (*n1 * n12 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            if n23 != 0 as ::core::ffi::c_long {
                f2c_dgemm_0(
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut n2,
                    k,
                    &raw mut n23,
                    &raw mut c_b22,
                    q2.offset(iq2 as isize) as *mut doublereal,
                    &raw mut n2,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut n23,
                    &raw mut c_b23,
                    q.offset((*n1 + 1 as integer + q_dim1) as isize) as *mut doublereal,
                    ldq,
                );
            } else {
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    &raw mut n2,
                    k,
                    &raw mut c_b23,
                    &raw mut c_b23,
                    q.offset((*n1 + 1 as integer + q_dim1) as isize) as *mut doublereal,
                    ldq,
                );
            }
            dlacpy__0(
                b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut n12,
                k,
                q.offset(q_offset as isize) as *mut doublereal,
                ldq,
                s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut n12,
            );
            if n12 != 0 as ::core::ffi::c_long {
                f2c_dgemm_0(
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n1,
                    k,
                    &raw mut n12,
                    &raw mut c_b22,
                    q2.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    n1,
                    s.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut n12,
                    &raw mut c_b23,
                    q.offset(q_offset as isize) as *mut doublereal,
                    ldq,
                );
            } else {
                dlaset__0(
                    b"A\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    n1,
                    k,
                    &raw mut c_b23,
                    &raw mut c_b23,
                    q.offset((q_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                    ldq,
                );
            }
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
