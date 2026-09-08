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
pub type real = ::core::ffi::c_float;
pub type doublereal = ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_iparmq_(
    mut ispec: *mut integer,
    mut name__: *mut ::core::ffi::c_char,
    mut opts: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut ilo: *mut integer,
    mut ihi: *mut integer,
    mut lwork: *mut integer,
) -> integer {
    let mut ret_val: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut r__1: real = 0.;
    extern "C" {
        #[link_name = "dgeev_closure_i_nint"]
        fn i_nint_0(_: *mut real) -> integer;
    }
    let mut nh: integer = 0;
    let mut ns: integer = 0;
    if *ispec == 15 as ::core::ffi::c_long
        || *ispec == 13 as ::core::ffi::c_long
        || *ispec == 16 as ::core::ffi::c_long
    {
        nh = (*ihi - *ilo + 1 as ::core::ffi::c_long) as integer;
        ns = 2 as integer;
        if nh >= 30 as ::core::ffi::c_long {
            ns = 4 as integer;
        }
        if nh >= 60 as ::core::ffi::c_long {
            ns = 10 as integer;
        }
        if nh >= 150 as ::core::ffi::c_long {
            r__1 = (log(nh as real as doublereal) / log(2.0f64)) as real;
            i__1 = 10 as integer;
            i__2 = nh / i_nint_0(&raw mut r__1);
            ns = (if i__1 >= i__2 {
                i__1 as ::core::ffi::c_long
            } else {
                i__2 as ::core::ffi::c_long
            }) as integer;
        }
        if nh >= 590 as ::core::ffi::c_long {
            ns = 64 as integer;
        }
        if nh >= 3000 as ::core::ffi::c_long {
            ns = 128 as integer;
        }
        if nh >= 6000 as ::core::ffi::c_long {
            ns = 256 as integer;
        }
        i__1 = 2 as integer;
        i__2 = (ns as ::core::ffi::c_long - ns as ::core::ffi::c_long % 2 as ::core::ffi::c_long)
            as integer;
        ns = (if i__1 >= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
    }
    if *ispec == 12 as ::core::ffi::c_long {
        ret_val = 75 as integer;
    } else if *ispec == 14 as ::core::ffi::c_long {
        ret_val = 14 as integer;
    } else if *ispec == 15 as ::core::ffi::c_long {
        ret_val = ns;
    } else if *ispec == 13 as ::core::ffi::c_long {
        if nh <= 500 as ::core::ffi::c_long {
            ret_val = ns;
        } else {
            ret_val = (ns as ::core::ffi::c_long * 3 as ::core::ffi::c_long
                / 2 as ::core::ffi::c_long) as integer;
        }
    } else if *ispec == 16 as ::core::ffi::c_long {
        ret_val = 0 as integer;
        if ns >= 14 as ::core::ffi::c_long {
            ret_val = 1 as integer;
        }
        if ns >= 14 as ::core::ffi::c_long {
            ret_val = 2 as integer;
        }
    } else {
        ret_val = -(1 as ::core::ffi::c_int) as integer;
    }
    return ret_val;
}
