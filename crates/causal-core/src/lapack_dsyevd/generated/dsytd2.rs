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
static mut c__1: integer = 1 as integer;
static mut c_b8: doublereal = 0.0f64;
static mut c_b14: doublereal = -1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dsytd2_(
    mut uplo: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut tau: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_ddot"]
        fn f2c_ddot_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> doublereal;
    }
    let mut taui: doublereal = 0.;
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_dsyr2"]
        fn f2c_dsyr2_0(
            _: *mut ::core::ffi::c_char,
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
    let mut alpha: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_daxpy"]
        fn f2c_daxpy_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut upper: logical = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_dsymv"]
        fn f2c_dsymv_0(
            _: *mut ::core::ffi::c_char,
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
        #[link_name = "dsyevd_closure_dlarfg_"]
        fn dlarfg__0(
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
    d__ = d__.offset(-1);
    e = e.offset(-1);
    tau = tau.offset(-1);
    *info = 0 as integer;
    upper = lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if upper == 0
        && lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(4 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DSYTD2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *n <= 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if upper != 0 {
        i__ = (*n - 1 as ::core::ffi::c_long) as integer;
        while i__ >= 1 as ::core::ffi::c_long {
            dlarfg__0(
                &raw mut i__,
                a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                a.offset(
                    ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                        * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                &raw mut taui,
            );
            *e.offset(i__ as isize) = *a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize);
            if taui != 0.0f64 {
                *a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) = 1.0f64 as doublereal;
                f2c_dsymv_0(
                    uplo,
                    &raw mut i__,
                    &raw mut taui,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    a.offset(
                        ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b8,
                    tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                alpha = taui
                    * -0.5f64
                    * f2c_ddot_0(
                        &raw mut i__,
                        tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                        &raw mut c__1,
                        a.offset(
                            ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                f2c_daxpy_0(
                    &raw mut i__,
                    &raw mut alpha,
                    a.offset(
                        ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                f2c_dsyr2_0(
                    uplo,
                    &raw mut i__,
                    &raw mut c_b14,
                    a.offset(
                        ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c__1,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                );
                *a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) = *e.offset(i__ as isize);
            }
            *d__.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                *a.offset((i__ + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize);
            *tau.offset(i__ as isize) = taui;
            i__ -= 1;
        }
        *d__.offset(1 as ::core::ffi::c_int as isize) =
            *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
    } else {
        i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
        i__ = 1 as integer;
        while i__ <= i__1 {
            i__2 = *n - i__;
            i__3 = (i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
            dlarfg__0(
                &raw mut i__2,
                a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                a.offset(((if i__3 <= *n { i__3 } else { *n }) + i__ * a_dim1) as isize)
                    as *mut doublereal,
                &raw mut c__1,
                &raw mut taui,
            );
            *e.offset(i__ as isize) = *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize);
            if taui != 0.0f64 {
                *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
                i__2 = *n - i__;
                f2c_dsymv_0(
                    uplo,
                    &raw mut i__2,
                    &raw mut taui,
                    a.offset((i__ + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize)
                        as *mut doublereal,
                    lda,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b8,
                    tau.offset(i__ as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                alpha = taui
                    * -0.5f64
                    * f2c_ddot_0(
                        &raw mut i__2,
                        tau.offset(i__ as isize) as *mut doublereal,
                        &raw mut c__1,
                        a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                        &raw mut c__1,
                    );
                i__2 = *n - i__;
                f2c_daxpy_0(
                    &raw mut i__2,
                    &raw mut alpha,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    tau.offset(i__ as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                f2c_dsyr2_0(
                    uplo,
                    &raw mut i__2,
                    &raw mut c_b14,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    tau.offset(i__ as isize) as *mut doublereal,
                    &raw mut c__1,
                    a.offset((i__ + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize)
                        as *mut doublereal,
                    lda,
                );
                *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) = *e.offset(i__ as isize);
            }
            *d__.offset(i__ as isize) = *a.offset((i__ + i__ * a_dim1) as isize);
            *tau.offset(i__ as isize) = taui;
            i__ += 1;
        }
        *d__.offset(*n as isize) = *a.offset((*n + *n * a_dim1) as isize);
    }
    return 0 as ::core::ffi::c_int;
}
