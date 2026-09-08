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
static mut c_b4: doublereal = 1.0f64;
static mut c_b5: doublereal = 0.0f64;
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlarf_(
    mut side: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut v: *mut doublereal,
    mut incv: *mut integer,
    mut tau: *mut doublereal,
    mut c__: *mut doublereal,
    mut ldc: *mut integer,
    mut work: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut i__: integer = 0;
    let mut applyleft: logical = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_dger"]
        fn f2c_dger_0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dgemv"]
        fn f2c_dgemv_0(
            _: *mut ::core::ffi::c_char,
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
    let mut lastc: integer = 0;
    let mut lastv: integer = 0;
    extern "C" {
        #[link_name = "dgeev_closure_iladlc_"]
        fn iladlc__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> integer;
    }
    extern "C" {
        #[link_name = "dgeev_closure_iladlr_"]
        fn iladlr__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> integer;
    }
    v = v.offset(-1);
    c_dim1 = *ldc;
    c_offset = 1 as integer + c_dim1;
    c__ = c__.offset(-(c_offset as isize));
    work = work.offset(-1);
    applyleft = lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    lastv = 0 as integer;
    lastc = 0 as integer;
    if *tau != 0.0f64 {
        if applyleft != 0 {
            lastv = *m;
        } else {
            lastv = *n;
        }
        if *incv > 0 as ::core::ffi::c_long {
            i__ = ((lastv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) * *incv
                + 1 as ::core::ffi::c_long) as integer;
        } else {
            i__ = 1 as integer;
        }
        while lastv > 0 as ::core::ffi::c_long && *v.offset(i__ as isize) == 0.0f64 {
            lastv -= 1;
            i__ -= *incv as ::core::ffi::c_long;
        }
        if applyleft != 0 {
            lastc = iladlc__0(
                &raw mut lastv,
                n,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
            );
        } else {
            lastc = iladlr__0(
                m,
                &raw mut lastv,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
            );
        }
    }
    if applyleft != 0 {
        if lastv > 0 as ::core::ffi::c_long {
            f2c_dgemv_0(
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut lastv,
                &raw mut lastc,
                &raw mut c_b4,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
                v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                incv,
                &raw mut c_b5,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            );
            d__1 = -*tau;
            f2c_dger_0(
                &raw mut lastv,
                &raw mut lastc,
                &raw mut d__1,
                v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                incv,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
            );
        }
    } else if lastv > 0 as ::core::ffi::c_long {
        f2c_dgemv_0(
            b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut lastc,
            &raw mut lastv,
            &raw mut c_b4,
            c__.offset(c_offset as isize) as *mut doublereal,
            ldc,
            v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            incv,
            &raw mut c_b5,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        d__1 = -*tau;
        f2c_dger_0(
            &raw mut lastc,
            &raw mut lastv,
            &raw mut d__1,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            incv,
            c__.offset(c_offset as isize) as *mut doublereal,
            ldc,
        );
    }
    return 0 as ::core::ffi::c_int;
}
