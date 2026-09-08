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
pub unsafe extern "C" fn dgeev_closure_dgehd2_(
    mut n: *mut integer,
    mut ilo: *mut integer,
    mut ihi: *mut integer,
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
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
    let mut aii: doublereal = 0.;
    extern "C" {
        #[link_name = "dgeev_closure_dlarf_"]
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
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    tau = tau.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    if *n < 0 as ::core::ffi::c_long {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *ilo < 1 as ::core::ffi::c_long
        || *ilo
            > (if 1 as ::core::ffi::c_long >= *n {
                1 as ::core::ffi::c_long
            } else {
                *n
            })
    {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *ihi < (if *ilo <= *n { *ilo } else { *n }) || *ihi > *n {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *n {
            1 as ::core::ffi::c_long
        } else {
            *n
        })
    {
        *info = -(5 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DGEHD2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    i__1 = (*ihi - 1 as ::core::ffi::c_long) as integer;
    i__ = *ilo;
    while i__ <= i__1 {
        i__2 = *ihi - i__;
        i__3 = (i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as integer;
        dlarfg__0(
            &raw mut i__2,
            a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
            a.offset(((if i__3 <= *n { i__3 } else { *n }) + i__ * a_dim1) as isize)
                as *mut doublereal,
            &raw mut c__1,
            tau.offset(i__ as isize) as *mut doublereal,
        );
        aii = *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize);
        *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) = 1.0f64 as doublereal;
        i__2 = *ihi - i__;
        dlarf__0(
            b"Right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ihi,
            &raw mut i__2,
            a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
            &raw mut c__1,
            tau.offset(i__ as isize) as *mut doublereal,
            a.offset(
                ((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    * a_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            lda,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        );
        i__2 = *ihi - i__;
        i__3 = *n - i__;
        dlarf__0(
            b"Left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__2,
            &raw mut i__3,
            a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
            &raw mut c__1,
            tau.offset(i__ as isize) as *mut doublereal,
            a.offset((i__ + 1 as integer + (i__ + 1 as integer) * a_dim1) as isize)
                as *mut doublereal,
            lda,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        );
        *a.offset((i__ + 1 as integer + i__ * a_dim1) as isize) = aii;
        i__ += 1;
    }
    return 0 as ::core::ffi::c_int;
}
