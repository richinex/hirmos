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
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlarfg_(
    mut n: *mut integer,
    mut alpha: *mut doublereal,
    mut x: *mut doublereal,
    mut incx: *mut integer,
    mut tau: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn d_sign_0(_: *mut doublereal, _: *mut doublereal) -> ::core::ffi::c_double;
    }
    let mut j: integer = 0;
    let mut knt: integer = 0;
    let mut beta: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dnrm2"]
        fn f2c_dnrm2_0(_: *mut integer, _: *mut doublereal, _: *mut integer) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dscal"]
        fn f2c_dscal_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut xnorm: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_dlapy2_"]
        fn dlapy2__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    let mut safmin: doublereal = 0.;
    let mut rsafmn: doublereal = 0.;
    x = x.offset(-1);
    if *n <= 1 as ::core::ffi::c_long {
        *tau = 0.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
    xnorm = f2c_dnrm2_0(
        &raw mut i__1,
        x.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        incx,
    );
    if xnorm == 0.0f64 {
        *tau = 0.0f64 as doublereal;
    } else {
        d__1 = dlapy2__0(alpha, &raw mut xnorm);
        beta = -d_sign_0(&raw mut d__1, alpha) as doublereal;
        safmin = dlamch__0(
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) / dlamch__0(
            b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        knt = 0 as integer;
        if (if beta >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            beta as ::core::ffi::c_double
        } else {
            -(beta as ::core::ffi::c_double)
        }) < safmin
        {
            rsafmn = 1.0f64 / safmin;
            loop {
                knt += 1;
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                f2c_dscal_0(
                    &raw mut i__1,
                    &raw mut rsafmn,
                    x.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    incx,
                );
                beta *= rsafmn as ::core::ffi::c_double;
                *alpha *= rsafmn as ::core::ffi::c_double;
                if !((if beta >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    beta as ::core::ffi::c_double
                } else {
                    -(beta as ::core::ffi::c_double)
                }) < safmin)
                {
                    break;
                }
            }
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            xnorm = f2c_dnrm2_0(
                &raw mut i__1,
                x.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                incx,
            );
            d__1 = dlapy2__0(alpha, &raw mut xnorm);
            beta = -d_sign_0(&raw mut d__1, alpha) as doublereal;
        }
        *tau = (beta - *alpha) / beta;
        i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
        d__1 = 1.0f64 / (*alpha - beta);
        f2c_dscal_0(
            &raw mut i__1,
            &raw mut d__1,
            x.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            incx,
        );
        i__1 = knt;
        j = 1 as integer;
        while j <= i__1 {
            beta *= safmin as ::core::ffi::c_double;
            j += 1;
        }
        *alpha = beta;
    }
    return 0 as ::core::ffi::c_int;
}
