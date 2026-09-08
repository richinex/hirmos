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
static mut c_b16: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dlabrd_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut nb: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut tauq: *mut doublereal,
    mut taup: *mut doublereal,
    mut x: *mut doublereal,
    mut ldx: *mut integer,
    mut y: *mut doublereal,
    mut ldy: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut x_dim1: integer = 0;
    let mut x_offset: integer = 0;
    let mut y_dim1: integer = 0;
    let mut y_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
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
    d__ = d__.offset(-1);
    e = e.offset(-1);
    tauq = tauq.offset(-1);
    taup = taup.offset(-1);
    x_dim1 = *ldx;
    x_offset = 1 as integer + x_dim1;
    x = x.offset(-(x_offset as isize));
    y_dim1 = *ldy;
    y_offset = 1 as integer + y_dim1;
    y = y.offset(-(y_offset as isize));
    if *m <= 0 as ::core::ffi::c_long || *n <= 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *m >= *n {
        i__1 = *nb;
        i__ = 1 as integer;
        while i__ <= i__1 {
            i__2 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dgemv_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__3,
                &raw mut c_b4,
                a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                lda,
                y.offset((i__ + y_dim1) as isize) as *mut doublereal,
                ldy,
                &raw mut c_b5,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            f2c_dgemv_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__3,
                &raw mut c_b4,
                x.offset((i__ + x_dim1) as isize) as *mut doublereal,
                ldx,
                a.offset(
                    (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                &raw mut c_b5,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
            i__2 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__3 = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dlarfg__0(
                &raw mut i__2,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                a.offset(((if i__3 <= *m { i__3 } else { *m }) + i__ * a_dim1) as isize)
                    as *mut doublereal,
                &raw mut c__1,
                tauq.offset(i__ as isize) as *mut doublereal,
            );
            *d__.offset(i__ as isize) = *a.offset((i__ + i__ * a_dim1) as isize);
            if i__ < *n {
                *a.offset((i__ + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
                i__2 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                i__3 = *n - i__;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    lda,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    y.offset((i__ + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                    lda,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    y.offset(
                        (i__ as ::core::ffi::c_long * y_dim1 as ::core::ffi::c_long
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
                    &raw mut c_b4,
                    y.offset((i__ + 1 as integer + y_dim1) as isize) as *mut doublereal,
                    ldy,
                    y.offset(
                        (i__ as ::core::ffi::c_long * y_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b5,
                    y.offset((i__ + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    x.offset((i__ + x_dim1) as isize) as *mut doublereal,
                    ldx,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    y.offset(
                        (i__ as ::core::ffi::c_long * y_dim1 as ::core::ffi::c_long
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
                    &raw mut c_b4,
                    a.offset(
                        ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    lda,
                    y.offset(
                        (i__ as ::core::ffi::c_long * y_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b5,
                    y.offset((i__ + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                f2c_dscal_0(
                    &raw mut i__2,
                    tauq.offset(i__ as isize) as *mut doublereal,
                    y.offset((i__ + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__,
                    &raw mut c_b4,
                    y.offset((i__ + 1 as integer + y_dim1) as isize) as *mut doublereal,
                    ldy,
                    a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                    lda,
                    &raw mut c_b5,
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    lda,
                );
                i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                i__3 = *n - i__;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b4,
                    a.offset(
                        ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    lda,
                    x.offset((i__ + x_dim1) as isize) as *mut doublereal,
                    ldx,
                    &raw mut c_b5,
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    lda,
                );
                i__2 = *n - i__;
                i__3 = (i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
                dlarfg__0(
                    &raw mut i__2,
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    a.offset((i__ + (if i__3 <= *n { i__3 } else { *n }) * a_dim1) as isize)
                        as *mut doublereal,
                    lda,
                    taup.offset(i__ as isize) as *mut doublereal,
                );
                *e.offset(i__ as isize) = *a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize);
                *a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) = 1.0f64 as doublereal;
                i__2 = *m - i__;
                i__3 = *n - i__;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    a.offset((i__ + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize)
                        as *mut doublereal,
                    lda,
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    lda,
                    &raw mut c_b16,
                    x.offset((i__ + 1 as integer + i__ * x_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__,
                    &raw mut c_b5,
                    y.offset((i__ + 1 as integer + y_dim1) as isize) as *mut doublereal,
                    ldy,
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    lda,
                    &raw mut c_b16,
                    x.offset(
                        (i__ as ::core::ffi::c_long * x_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__,
                    &raw mut c_b4,
                    a.offset((i__ + 1 as integer + a_dim1) as isize) as *mut doublereal,
                    lda,
                    x.offset(
                        (i__ as ::core::ffi::c_long * x_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b5,
                    x.offset((i__ + 1 as integer + i__ * x_dim1) as isize) as *mut doublereal,
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
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    lda,
                    &raw mut c_b16,
                    x.offset(
                        (i__ as ::core::ffi::c_long * x_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b4,
                    x.offset((i__ + 1 as integer + x_dim1) as isize) as *mut doublereal,
                    ldx,
                    x.offset(
                        (i__ as ::core::ffi::c_long * x_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b5,
                    x.offset((i__ + 1 as integer + i__ * x_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                f2c_dscal_0(
                    &raw mut i__2,
                    taup.offset(i__ as isize) as *mut doublereal,
                    x.offset((i__ + 1 as integer + i__ * x_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
            }
            i__ += 1;
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
                &raw mut c_b4,
                y.offset((i__ + y_dim1) as isize) as *mut doublereal,
                ldy,
                a.offset((i__ + a_dim1) as isize) as *mut doublereal,
                lda,
                &raw mut c_b5,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
            );
            i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            i__3 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            f2c_dgemv_0(
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__3,
                &raw mut c_b4,
                a.offset(
                    (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                lda,
                x.offset((i__ + x_dim1) as isize) as *mut doublereal,
                ldx,
                &raw mut c_b5,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
            );
            i__2 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__3 = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dlarfg__0(
                &raw mut i__2,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                a.offset((i__ + (if i__3 <= *n { i__3 } else { *n }) * a_dim1) as isize)
                    as *mut doublereal,
                lda,
                taup.offset(i__ as isize) as *mut doublereal,
            );
            *d__.offset(i__ as isize) = *a.offset((i__ + i__ * a_dim1) as isize);
            if i__ < *m {
                *a.offset((i__ + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
                i__2 = *m - i__;
                i__3 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    &raw mut c_b16,
                    x.offset((i__ + 1 as integer + i__ * x_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    y.offset((i__ + y_dim1) as isize) as *mut doublereal,
                    ldy,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    &raw mut c_b16,
                    x.offset(
                        (i__ as ::core::ffi::c_long * x_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b4,
                    a.offset((i__ + 1 as integer + a_dim1) as isize) as *mut doublereal,
                    lda,
                    x.offset(
                        (i__ as ::core::ffi::c_long * x_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b5,
                    x.offset((i__ + 1 as integer + i__ * x_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                i__3 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    a.offset(
                        (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    lda,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    &raw mut c_b16,
                    x.offset(
                        (i__ as ::core::ffi::c_long * x_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b4,
                    x.offset((i__ + 1 as integer + x_dim1) as isize) as *mut doublereal,
                    ldx,
                    x.offset(
                        (i__ as ::core::ffi::c_long * x_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b5,
                    x.offset((i__ + 1 as integer + i__ * x_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                f2c_dscal_0(
                    &raw mut i__2,
                    taup.offset(i__ as isize) as *mut doublereal,
                    x.offset((i__ + 1 as integer + i__ * x_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b4,
                    a.offset((i__ + 1 as integer + a_dim1) as isize) as *mut doublereal,
                    lda,
                    y.offset((i__ + y_dim1) as isize) as *mut doublereal,
                    ldy,
                    &raw mut c_b5,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__,
                    &raw mut c_b4,
                    x.offset((i__ + 1 as integer + x_dim1) as isize) as *mut doublereal,
                    ldx,
                    a.offset(
                        (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b5,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                i__3 = (i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
                dlarfg__0(
                    &raw mut i__2,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    a.offset(((if i__3 <= *m { i__3 } else { *m }) + i__ * a_dim1) as isize)
                        as *mut doublereal,
                    &raw mut c__1,
                    tauq.offset(i__ as isize) as *mut doublereal,
                );
                *e.offset(i__ as isize) = *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize);
                *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
                i__2 = *m - i__;
                i__3 = *n - i__;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    a.offset((i__ + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize)
                        as *mut doublereal,
                    lda,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    y.offset((i__ + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_b5,
                    a.offset((i__ + 1 as integer + a_dim1) as isize) as *mut doublereal,
                    lda,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    y.offset(
                        (i__ as ::core::ffi::c_long * y_dim1 as ::core::ffi::c_long
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
                    &raw mut c_b4,
                    y.offset((i__ + 1 as integer + y_dim1) as isize) as *mut doublereal,
                    ldy,
                    y.offset(
                        (i__ as ::core::ffi::c_long * y_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b5,
                    y.offset((i__ + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *m - i__;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__,
                    &raw mut c_b5,
                    x.offset((i__ + 1 as integer + x_dim1) as isize) as *mut doublereal,
                    ldx,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b16,
                    y.offset(
                        (i__ as ::core::ffi::c_long * y_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                f2c_dgemv_0(
                    b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__,
                    &raw mut i__2,
                    &raw mut c_b4,
                    a.offset(
                        ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                            * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    lda,
                    y.offset(
                        (i__ as ::core::ffi::c_long * y_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut c_b5,
                    y.offset((i__ + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__2 = *n - i__;
                f2c_dscal_0(
                    &raw mut i__2,
                    tauq.offset(i__ as isize) as *mut doublereal,
                    y.offset((i__ + 1 as integer + i__ * y_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
            }
            i__ += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
