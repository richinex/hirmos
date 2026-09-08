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
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dorghr_(
    mut n: *mut integer,
    mut ilo: *mut integer,
    mut ihi: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut tau: *mut doublereal,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut nb: integer = 0;
    let mut nh: integer = 0;
    let mut iinfo: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgeev_closure_ilaenv_"]
        fn ilaenv__0(
            _: *mut integer,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
        ) -> integer;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dorgqr_"]
        fn dorgqr__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut lwkopt: integer = 0;
    let mut lquery: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    tau = tau.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    nh = *ihi - *ilo;
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
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
    } else if *lwork
        < (if 1 as ::core::ffi::c_long >= nh {
            1 as ::core::ffi::c_long
        } else {
            nh as ::core::ffi::c_long
        })
        && lquery == 0
    {
        *info = -(8 as ::core::ffi::c_int) as integer;
    }
    if *info == 0 as ::core::ffi::c_long {
        nb = ilaenv__0(
            &raw mut c__1,
            b"DORGQR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut nh,
            &raw mut nh,
            &raw mut nh,
            &raw mut c_n1,
        );
        lwkopt = (if 1 as ::core::ffi::c_long >= nh {
            1 as integer
        } else {
            nh
        }) * nb;
        *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DORGHR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *n == 0 as ::core::ffi::c_long {
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    i__1 = (*ilo + 1 as ::core::ffi::c_long) as integer;
    j = *ihi;
    while j >= i__1 {
        i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        i__ = 1 as integer;
        while i__ <= i__2 {
            *a.offset((i__ + j * a_dim1) as isize) = 0.0f64 as doublereal;
            i__ += 1;
        }
        i__2 = *ihi;
        i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        while i__ <= i__2 {
            *a.offset((i__ + j * a_dim1) as isize) =
                *a.offset((i__ + (j - 1 as integer) * a_dim1) as isize);
            i__ += 1;
        }
        i__2 = *n;
        i__ = (*ihi + 1 as ::core::ffi::c_long) as integer;
        while i__ <= i__2 {
            *a.offset((i__ + j * a_dim1) as isize) = 0.0f64 as doublereal;
            i__ += 1;
        }
        j -= 1;
    }
    i__1 = *ilo;
    j = 1 as integer;
    while j <= i__1 {
        i__2 = *n;
        i__ = 1 as integer;
        while i__ <= i__2 {
            *a.offset((i__ + j * a_dim1) as isize) = 0.0f64 as doublereal;
            i__ += 1;
        }
        *a.offset((j + j * a_dim1) as isize) = 1.0f64 as doublereal;
        j += 1;
    }
    i__1 = *n;
    j = (*ihi + 1 as ::core::ffi::c_long) as integer;
    while j <= i__1 {
        i__2 = *n;
        i__ = 1 as integer;
        while i__ <= i__2 {
            *a.offset((i__ + j * a_dim1) as isize) = 0.0f64 as doublereal;
            i__ += 1;
        }
        *a.offset((j + j * a_dim1) as isize) = 1.0f64 as doublereal;
        j += 1;
    }
    if nh > 0 as ::core::ffi::c_long {
        dorgqr__0(
            &raw mut nh,
            &raw mut nh,
            &raw mut nh,
            a.offset((*ilo + 1 as integer + (*ilo + 1 as integer) * a_dim1) as isize)
                as *mut doublereal,
            lda,
            tau.offset(*ilo as isize) as *mut doublereal,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            lwork,
            &raw mut iinfo,
        );
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    return 0 as ::core::ffi::c_int;
}
