pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dormr3_(
    mut side: *mut ::core::ffi::c_char,
    mut trans: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut k: *mut integer,
    mut l: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut tau: *mut doublereal,
    mut c__: *mut doublereal,
    mut ldc: *mut integer,
    mut work: *mut doublereal,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__: integer = 0;
    let mut i1: integer = 0;
    let mut i2: integer = 0;
    let mut i3: integer = 0;
    let mut ja: integer = 0;
    let mut ic: integer = 0;
    let mut jc: integer = 0;
    let mut mi: integer = 0;
    let mut ni: integer = 0;
    let mut nq: integer = 0;
    let mut left: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn dgelsd_closure_lsame__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
        ) -> logical;
    }
    extern "C" {
        #[link_name = "slsqp_closure_dlarz_"]
        fn slsqp_closure_dlarz__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
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
        fn dgelsd_closure_xerbla__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut notran: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    tau = tau.offset(-1);
    c_dim1 = *ldc;
    c_offset = 1 as integer + c_dim1;
    c__ = c__.offset(-(c_offset as isize));
    work = work.offset(-1);
    *info = 0 as integer;
    left = dgelsd_closure_lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    notran = dgelsd_closure_lsame__0(
        trans,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if left != 0 {
        nq = *m;
    } else {
        nq = *n;
    }
    if left == 0
        && dgelsd_closure_lsame__0(
            side,
            b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if notran == 0
        && dgelsd_closure_lsame__0(
            trans,
            b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *m < 0 as ::core::ffi::c_long {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *k < 0 as ::core::ffi::c_long || *k > nq {
        *info = -(5 as ::core::ffi::c_int) as integer;
    } else if *l < 0 as ::core::ffi::c_long || left != 0 && *l > *m || left == 0 && *l > *n {
        *info = -(6 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *k {
            1 as ::core::ffi::c_long
        } else {
            *k
        })
    {
        *info = -(8 as ::core::ffi::c_int) as integer;
    } else if *ldc
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        *info = -(11 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        dgelsd_closure_xerbla__0(
            b"DORMR3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if *m == 0 as ::core::ffi::c_long
        || *n == 0 as ::core::ffi::c_long
        || *k == 0 as ::core::ffi::c_long
    {
        return 0 as ::core::ffi::c_int;
    }
    if left != 0 && notran == 0 || left == 0 && notran != 0 {
        i1 = 1 as integer;
        i2 = *k;
        i3 = 1 as integer;
    } else {
        i1 = *k;
        i2 = 1 as integer;
        i3 = -(1 as ::core::ffi::c_int) as integer;
    }
    if left != 0 {
        ni = *n;
        ja = (*m - *l + 1 as ::core::ffi::c_long) as integer;
        jc = 1 as integer;
    } else {
        mi = *m;
        ja = (*n - *l + 1 as ::core::ffi::c_long) as integer;
        ic = 1 as integer;
    }
    i__1 = i2;
    i__2 = i3;
    i__ = i1;
    while if i__2 < 0 as ::core::ffi::c_long {
        (i__ >= i__1) as ::core::ffi::c_int
    } else {
        (i__ <= i__1) as ::core::ffi::c_int
    } != 0
    {
        if left != 0 {
            mi = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            ic = i__;
        } else {
            ni = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            jc = i__;
        }
        slsqp_closure_dlarz__0(
            side,
            &raw mut mi,
            &raw mut ni,
            l,
            a.offset((i__ + ja * a_dim1) as isize) as *mut doublereal,
            lda,
            tau.offset(i__ as isize) as *mut doublereal,
            c__.offset((ic + jc * c_dim1) as isize) as *mut doublereal,
            ldc,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        );
        i__ += i__2 as ::core::ffi::c_long;
    }
    return 0 as ::core::ffi::c_int;
}
