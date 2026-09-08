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
pub unsafe extern "C" fn dgesdd_closure_dorgl2_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut k: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut tau: *mut doublereal,
    mut work: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut l: integer = 0;
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
        #[link_name = "dsyevd_closure_dlarf_"]
        fn dlarf__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    tau = tau.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    if *m < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < *m {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *k < 0 as ::core::ffi::c_long || *k > *m {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        *info = -(5 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DORGL2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *m <= 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *k < *m {
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            i__2 = *m;
            l = (*k + 1 as ::core::ffi::c_long) as integer;
            while l <= i__2 {
                *a.offset((l + j * a_dim1) as isize) = 0.0f64 as doublereal;
                l += 1;
            }
            if j > *k && j <= *m {
                *a.offset((j + j * a_dim1) as isize) = 1.0f64 as doublereal;
            }
            j += 1;
        }
    }
    i__ = *k;
    while i__ >= 1 as ::core::ffi::c_long {
        if i__ < *n {
            if i__ < *m {
                *a.offset((i__ + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
                i__1 = *m - i__;
                i__2 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dlarf__0(
                    b"Right\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__1,
                    &raw mut i__2,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    tau.offset(i__ as isize) as *mut doublereal,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
            }
            i__1 = *n - i__;
            d__1 = -*tau.offset(i__ as isize);
            f2c_dscal_0(
                &raw mut i__1,
                &raw mut d__1,
                a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                lda,
            );
        }
        *a.offset((i__ + i__ * a_dim1) as isize) = 1.0f64 - *tau.offset(i__ as isize);
        i__1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        l = 1 as integer;
        while l <= i__1 {
            *a.offset((i__ + l * a_dim1) as isize) = 0.0f64 as doublereal;
            l += 1;
        }
        i__ -= 1;
    }
    return 0 as ::core::ffi::c_int;
}
