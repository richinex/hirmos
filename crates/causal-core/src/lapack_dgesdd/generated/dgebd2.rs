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
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dgebd2_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut tauq: *mut doublereal,
    mut taup: *mut doublereal,
    mut work: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
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
    tauq = tauq.offset(-1);
    taup = taup.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    if *m < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        *info = -(4 as ::core::ffi::c_int) as integer;
    }
    if *info < 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DGEBD2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *m >= *n {
        i__1 = *n;
        i__ = 1 as integer;
        while i__ <= i__1 {
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
            *a.offset((i__ + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
            if i__ < *n {
                i__2 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                i__3 = *n - i__;
                dlarf__0(
                    b"Left\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    tauq.offset(i__ as isize) as *mut doublereal,
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    lda,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
            }
            *a.offset((i__ + i__ * a_dim1) as isize) = *d__.offset(i__ as isize);
            if i__ < *n {
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
                dlarf__0(
                    b"Right\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                    lda,
                    taup.offset(i__ as isize) as *mut doublereal,
                    a.offset((i__ + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize)
                        as *mut doublereal,
                    lda,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                *a.offset((i__ + (i__ + 1 as integer) * a_dim1) as isize) = *e.offset(i__ as isize);
            } else {
                *taup.offset(i__ as isize) = 0.0f64 as doublereal;
            }
            i__ += 1;
        }
    } else {
        i__1 = *m;
        i__ = 1 as integer;
        while i__ <= i__1 {
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
            *a.offset((i__ + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
            if i__ < *m {
                i__2 = *m - i__;
                i__3 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dlarf__0(
                    b"Right\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    taup.offset(i__ as isize) as *mut doublereal,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
            }
            *a.offset((i__ + i__ * a_dim1) as isize) = *d__.offset(i__ as isize);
            if i__ < *m {
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
                dlarf__0(
                    b"Left\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    &raw mut i__3,
                    a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    tauq.offset(i__ as isize) as *mut doublereal,
                    a.offset((i__ + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize)
                        as *mut doublereal,
                    lda,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
                *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) = *e.offset(i__ as isize);
            } else {
                *tauq.offset(i__ as isize) = 0.0f64 as doublereal;
            }
            i__ += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
