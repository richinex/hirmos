pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dgerq2_(
    mut m: *mut integer,
    mut n: *mut integer,
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
    let mut i__: integer = 0;
    let mut k: integer = 0;
    extern "C" {
        #[link_name = "dgeev_closure_dlarfg_"]
        fn dgeev_closure_dlarfg__0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn dgelsd_closure_xerbla__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "slsqp_closure_dlarf1l_"]
        fn slsqp_closure_dlarf1l__0(
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
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    tau = tau.offset(-1);
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
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        dgelsd_closure_xerbla__0(
            b"DGERQ2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    k = (if *m <= *n { *m } else { *n }) as integer;
    i__ = k;
    while i__ >= 1 as ::core::ffi::c_long {
        i__1 = *n - k + i__;
        dgeev_closure_dlarfg__0(
            &raw mut i__1,
            a.offset((*m - k + i__ + (*n - k + i__) * a_dim1) as isize) as *mut doublereal,
            a.offset((*m - k + i__ + a_dim1) as isize) as *mut doublereal,
            lda,
            tau.offset(i__ as isize) as *mut doublereal,
        );
        i__1 = (*m - k as ::core::ffi::c_long + i__ as ::core::ffi::c_long
            - 1 as ::core::ffi::c_long) as integer;
        i__2 = *n - k + i__;
        slsqp_closure_dlarf1l__0(
            b"Right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
            &raw mut i__2,
            a.offset((*m - k + i__ + a_dim1) as isize) as *mut doublereal,
            lda,
            tau.offset(i__ as isize) as *mut doublereal,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        );
        i__ -= 1;
    }
    return 0 as ::core::ffi::c_int;
}
