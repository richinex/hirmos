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
static mut c_b4: doublereal = -1.0f64;
static mut c_b5: doublereal = 1.0f64;
static mut c__1: integer = 1 as integer;
static mut c_b38: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlahr2_(
    mut n: *mut integer,
    mut k: *mut integer,
    mut nb: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut tau: *mut doublereal,
    mut t: *mut doublereal,
    mut ldt: *mut integer,
    mut y: *mut doublereal,
    mut ldy: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut t_dim1: integer = 0;
    let mut t_offset: integer = 0;
    let mut y_dim1: integer = 0;
    let mut y_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut i__: integer = 0;
    let mut ei: doublereal = 0.;
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
        #[link_name = "dsyevd_closure_f2c_dtrmm"]
        fn f2c_dtrmm_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
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
        #[link_name = "dsyevd_closure_f2c_dtrmv"]
        fn f2c_dtrmv_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlarfg_"]
        fn dlarfg__0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
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
    tau = tau.offset(-1);
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    t_dim1 = *ldt;
    t_offset = 1 as integer + t_dim1;
    t = t.offset(-(t_offset as isize));
    y_dim1 = *ldy;
    y_offset = 1 as integer + y_dim1;
    y = y.offset(-(y_offset as isize));
    if *n <= 1 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    i__1 = *nb;
    i__ = 1 as integer;
    while i__ <= i__1 {
        if i__ > 1 as ::core::ffi::c_long {
            i__2 = *n - *k;
            i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dgemv_0(
                b"NO TRANSPOSE\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__3,
                &raw mut c_b4,
                y.offset((*k + 1 as integer + y_dim1) as isize) as *mut doublereal,
                ldy,
                a.offset((*k + i__ - 1 as integer + a_dim1) as isize) as *mut doublereal,
                lda,
                &raw mut c_b5,
                a.offset((*k + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dcopy_0(
                &raw mut i__2,
                a.offset((*k + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                t.offset((*nb * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dtrmv_0(
                b"Lower\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"UNIT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__2,
                a.offset((*k + 1 as integer + a_dim1) as isize) as *mut doublereal,
                lda,
                t.offset((*nb * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (*n - *k - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dgemv_0(
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__3,
                &raw mut c_b5,
                a.offset((*k + i__ + a_dim1) as isize) as *mut doublereal,
                lda,
                a.offset((*k + i__ + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut c_b5,
                t.offset((*nb * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dtrmv_0(
                b"Upper\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"NON-UNIT\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__2,
                t.offset(t_offset as isize) as *mut doublereal,
                ldt,
                t.offset((*nb * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (*n - *k - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dgemv_0(
                b"NO TRANSPOSE\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__3,
                &raw mut c_b4,
                a.offset((*k + i__ + a_dim1) as isize) as *mut doublereal,
                lda,
                t.offset((*nb * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
                &raw mut c_b5,
                a.offset((*k + i__ + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dtrmv_0(
                b"Lower\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"NO TRANSPOSE\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"UNIT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__2,
                a.offset((*k + 1 as integer + a_dim1) as isize) as *mut doublereal,
                lda,
                t.offset((*nb * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_daxpy_0(
                &raw mut i__2,
                &raw mut c_b4,
                t.offset((*nb * t_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                &raw mut c__1,
                a.offset((*k + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
            *a.offset((*k + i__ - 1 as integer + (i__ - 1 as integer) * a_dim1) as isize) = ei;
        }
        i__2 = (*n - *k - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        i__3 = (*k + i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dlarfg__0(
            &raw mut i__2,
            a.offset((*k + i__ + i__ * a_dim1) as isize) as *mut doublereal,
            a.offset(((if i__3 <= *n { i__3 } else { *n }) + i__ * a_dim1) as isize)
                as *mut doublereal,
            &raw mut c__1,
            tau.offset(i__ as isize) as *mut doublereal,
        );
        ei = *a.offset((*k + i__ + i__ * a_dim1) as isize);
        *a.offset((*k + i__ + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
        i__2 = *n - *k;
        i__3 = (*n - *k - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        f2c_dgemv_0(
            b"NO TRANSPOSE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut i__2,
            &raw mut i__3,
            &raw mut c_b5,
            a.offset((*k + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize)
                as *mut doublereal,
            lda,
            a.offset((*k + i__ + i__ * a_dim1) as isize) as *mut doublereal,
            &raw mut c__1,
            &raw mut c_b38,
            y.offset((*k + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
            &raw mut c__1,
        );
        i__2 = (*n - *k - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        f2c_dgemv_0(
            b"Transpose\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__2,
            &raw mut i__3,
            &raw mut c_b5,
            a.offset((*k + i__ + a_dim1) as isize) as *mut doublereal,
            lda,
            a.offset((*k + i__ + i__ * a_dim1) as isize) as *mut doublereal,
            &raw mut c__1,
            &raw mut c_b38,
            t.offset(
                (i__ as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
        );
        i__2 = *n - *k;
        i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        f2c_dgemv_0(
            b"NO TRANSPOSE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut i__2,
            &raw mut i__3,
            &raw mut c_b4,
            y.offset((*k + 1 as integer + y_dim1) as isize) as *mut doublereal,
            ldy,
            t.offset(
                (i__ as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
            &raw mut c_b5,
            y.offset((*k + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
            &raw mut c__1,
        );
        i__2 = *n - *k;
        f2c_dscal_0(
            &raw mut i__2,
            tau.offset(i__ as isize) as *mut doublereal,
            y.offset((*k + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
            &raw mut c__1,
        );
        i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        d__1 = -*tau.offset(i__ as isize);
        f2c_dscal_0(
            &raw mut i__2,
            &raw mut d__1,
            t.offset(
                (i__ as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
        );
        i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        f2c_dtrmv_0(
            b"Upper\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"No Transpose\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"NON-UNIT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__2,
            t.offset(t_offset as isize) as *mut doublereal,
            ldt,
            t.offset(
                (i__ as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut c__1,
        );
        *t.offset((i__ + i__ * t_dim1) as isize) = *tau.offset(i__ as isize);
        i__ += 1;
    }
    *a.offset((*k + *nb + *nb * a_dim1) as isize) = ei;
    dlacpy__0(
        b"ALL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        k,
        nb,
        a.offset(
            (((a_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_long) as isize,
        ) as *mut doublereal,
        lda,
        y.offset(y_offset as isize) as *mut doublereal,
        ldy,
    );
    f2c_dtrmm_0(
        b"RIGHT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"Lower\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"NO TRANSPOSE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"UNIT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        k,
        nb,
        &raw mut c_b5,
        a.offset((*k + 1 as integer + a_dim1) as isize) as *mut doublereal,
        lda,
        y.offset(y_offset as isize) as *mut doublereal,
        ldy,
    );
    if *n > *k + *nb {
        i__1 = *n - *k - *nb;
        f2c_dgemm_0(
            b"NO TRANSPOSE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"NO TRANSPOSE\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            k,
            nb,
            &raw mut i__1,
            &raw mut c_b5,
            a.offset(
                ((*nb + 2 as ::core::ffi::c_long) * a_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            lda,
            a.offset((*k + 1 as integer + *nb + a_dim1) as isize) as *mut doublereal,
            lda,
            &raw mut c_b5,
            y.offset(y_offset as isize) as *mut doublereal,
            ldy,
        );
    }
    f2c_dtrmm_0(
        b"RIGHT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"Upper\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"NO TRANSPOSE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"NON-UNIT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        k,
        nb,
        &raw mut c_b5,
        t.offset(t_offset as isize) as *mut doublereal,
        ldt,
        y.offset(y_offset as isize) as *mut doublereal,
        ldy,
    );
    return 0 as ::core::ffi::c_int;
}
