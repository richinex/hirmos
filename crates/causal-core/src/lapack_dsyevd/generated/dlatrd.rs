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
static mut c_b5: doublereal = -1.0f64;
static mut c_b6: doublereal = 1.0f64;
static mut c__1: integer = 1 as integer;
static mut c_b16: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlatrd_(
    mut uplo: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut nb: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut e: *mut doublereal,
    mut tau: *mut doublereal,
    mut w: *mut doublereal,
    mut ldw: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut w_dim1: integer = 0;
    let mut w_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
    let mut iw: integer = 0;
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
    let mut alpha: doublereal = 0.;
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
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    e = e.offset(-1);
    tau = tau.offset(-1);
    w_dim1 = *ldw;
    w_offset = 1 as integer + w_dim1;
    w = w.offset(-(w_offset as isize));
    if *n <= 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        i__1 = (*n - *nb + 1 as ::core::ffi::c_long) as integer;
        i__ = *n;
        while i__ >= i__1 {
            iw = i__ - *n + *nb;
            if i__ < *n {
                i__2 = *n - i__;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__,
                    &raw mut i__2,
                    &raw mut c_b5,
                    a.offset(
                        ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    lda,
                    w.offset((i__ + (iw + 1 as integer) * w_dim1) as isize) as *mut doublereal,
                    ldw,
                    &raw mut c_b6,
                    a.offset(
                        (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__,
                    &raw mut i__2,
                    &raw mut c_b5,
                    w.offset(
                        ((iw as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            * w_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    ldw,
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    lda,
                    &raw mut c_b6,
                    a.offset(
                        (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
            }
            if i__ > 1 as ::core::ffi::c_long {
                i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                dlarfg__0(
                    &raw mut i__2,
                    a.offset((i__ - 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    a.offset(
                        (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    tau.offset((i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                );
                *e.offset((i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize) =
                    *a.offset((i__ - 1 as integer + i__ * a_dim1) as isize);
                *a.offset((i__ - 1 as integer + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
                i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dsymv_0(
                    b"Upper\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut c_b6,
                    a.offset(a_offset as isize) as *mut doublereal,
                    lda,
                    a.offset(
                        (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    w.offset(
                        (iw as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                if i__ < *n {
                    i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    i__3 = *n - i__;
                    f2c_dgemv_0(
                        b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut i__2,
                        &raw mut i__3,
                        &raw mut c_b6,
                        w.offset(
                            ((iw as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * w_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldw,
                        a.offset(
                            (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut c_b16,
                        w.offset((i__ + 1 as integer + iw * w_dim1) as isize) as *mut doublereal,
                        &raw mut c__1,
                    );
                    i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    i__3 = *n - i__;
                    f2c_dgemv_0(
                        b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut i__2,
                        &raw mut i__3,
                        &raw mut c_b5,
                        a.offset(
                            ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        lda,
                        w.offset((i__ + 1 as integer + iw * w_dim1) as isize) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut c_b6,
                        w.offset(
                            (iw as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                    i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    i__3 = *n - i__;
                    f2c_dgemv_0(
                        b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut i__2,
                        &raw mut i__3,
                        &raw mut c_b6,
                        a.offset(
                            ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        lda,
                        a.offset(
                            (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut c_b16,
                        w.offset((i__ + 1 as integer + iw * w_dim1) as isize) as *mut doublereal,
                        &raw mut c__1,
                    );
                    i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    i__3 = *n - i__;
                    f2c_dgemv_0(
                        b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut i__2,
                        &raw mut i__3,
                        &raw mut c_b5,
                        w.offset(
                            ((iw as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                                * w_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldw,
                        w.offset((i__ + 1 as integer + iw * w_dim1) as isize) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut c_b6,
                        w.offset(
                            (iw as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                }
                i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dscal_0(
                    &raw mut i__2,
                    tau.offset((i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                    w.offset(
                        (iw as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                alpha = *tau
                    .offset((i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as isize)
                    * -0.5f64
                    * f2c_ddot_0(
                        &raw mut i__2,
                        w.offset(
                            (iw as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                        a.offset(
                            (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_daxpy_0(
                    &raw mut i__2,
                    &raw mut alpha,
                    a.offset(
                        (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    w.offset(
                        (iw as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
            }
            i__ -= 1;
        }
    } else {
        i__1 = *nb;
        i__ = 1 as integer;
        while i__ <= i__1 {
            i__2 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dgemv_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__3,
                &raw mut c_b5,
                a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                lda,
                w.offset((i__ + w_dim1) as isize) as *mut doublereal,
                ldw,
                &raw mut c_b6,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dgemv_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__3,
                &raw mut c_b5,
                w.offset((i__ + w_dim1) as isize) as *mut doublereal,
                ldw,
                a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                lda,
                &raw mut c_b6,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
            if i__ < *n {
                i__2 = *n - i__;
                i__3 = (i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
                dlarfg__0(
                    &raw mut i__2,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    a.offset(((if i__3 <= *n { i__3 } else { *n }) + i__ * a_dim1) as isize)
                        as *mut doublereal,
                    &raw mut c__1,
                    tau.offset(i__ as isize) as *mut doublereal,
                );
                *e.offset(i__ as isize) = *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize);
                *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
                i__2 = *n - i__;
                f2c_dsymv_0(
                    b"Lower\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut c_b6,
                    a.offset((i__ + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize)
                        as *mut doublereal,
                    lda,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    w.offset((i__ + 1 as integer + i__ * w_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b6,
                    w.offset((i__ + 1 as integer + w_dim1) as isize) as *mut doublereal,
                    ldw,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    w.offset(
                        (i__ as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    a.offset((i__ + 1 as integer + a_dim1) as isize) as *mut doublereal,
                    lda,
                    w.offset(
                        (i__ as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b6,
                    w.offset((i__ + 1 as integer + i__ * w_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b6,
                    a.offset((i__ + 1 as integer + a_dim1) as isize) as *mut doublereal,
                    lda,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    w.offset(
                        (i__ as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    w.offset((i__ + 1 as integer + w_dim1) as isize) as *mut doublereal,
                    ldw,
                    w.offset(
                        (i__ as ::core::ffi::c_long * w_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b6,
                    w.offset((i__ + 1 as integer + i__ * w_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                f2c_dscal_0(
                    &raw mut i__2,
                    tau.offset(i__ as isize) as *mut doublereal,
                    w.offset((i__ + 1 as integer + i__ * w_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                alpha = *tau.offset(i__ as isize)
                    * -0.5f64
                    * f2c_ddot_0(
                        &raw mut i__2,
                        w.offset((i__ + 1 as integer + i__ * w_dim1) as isize) as *mut doublereal,
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
                    w.offset((i__ + 1 as integer + i__ * w_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
            }
            i__ += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
