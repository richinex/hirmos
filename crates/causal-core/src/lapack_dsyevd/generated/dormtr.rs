#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type integer = ::core::ffi::c_long;
pub type address = *mut ::core::ffi::c_char;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
pub type ftnlen = ::core::ffi::c_long;
static mut c__1: integer = 1 as integer;
static mut c_n1: integer = -(1 as ::core::ffi::c_int) as integer;
static mut c__2: integer = 2 as integer;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dormtr_(
    mut side: *mut ::core::ffi::c_char,
    mut uplo: *mut ::core::ffi::c_char,
    mut trans: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut tau: *mut doublereal,
    mut c__: *mut doublereal,
    mut ldc: *mut integer,
    mut work: *mut doublereal,
    mut lwork: *mut integer,
    mut info: *mut integer,
) -> ::core::ffi::c_int {
    let mut a__1: [address; 2] = [::core::ptr::null_mut::<::core::ffi::c_char>(); 2];
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut i__1: [integer; 2] = [0; 2];
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut ch__1: [::core::ffi::c_char; 2] = [0; 2];
    extern "C" {
        #[link_name = "dsyevd_closure_s_cat"]
        fn s_cat_0(
            _: *mut ::core::ffi::c_char,
            _: *mut *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: ftnlen,
        ) -> ::core::ffi::c_int;
    }
    let mut i1: integer = 0;
    let mut i2: integer = 0;
    let mut nb: integer = 0;
    let mut mi: integer = 0;
    let mut ni: integer = 0;
    let mut nq: integer = 0;
    let mut nw: integer = 0;
    let mut left: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut iinfo: integer = 0;
    let mut upper: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn xerbla__0(_: *mut ::core::ffi::c_char, _: *mut integer) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_ilaenv_"]
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
        #[link_name = "dsyevd_closure_dormql_"]
        fn dormql__0(
            _: *mut ::core::ffi::c_char,
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
            _: *mut integer,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_dormqr_"]
        fn dormqr__0(
            _: *mut ::core::ffi::c_char,
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
    c_dim1 = *ldc;
    c_offset = 1 as integer + c_dim1;
    c__ = c__.offset(-(c_offset as isize));
    work = work.offset(-1);
    *info = 0 as integer;
    left = lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    upper = lsame__0(
        uplo,
        b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
    if left != 0 {
        nq = *m;
        nw = *n;
    } else {
        nq = *n;
        nw = *m;
    }
    if left == 0
        && lsame__0(
            side,
            b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if upper == 0
        && lsame__0(
            uplo,
            b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if lsame__0(
        trans,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
        && lsame__0(
            trans,
            b"T\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *m < 0 as ::core::ffi::c_long {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long {
        *info = -(5 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= nq {
            1 as ::core::ffi::c_long
        } else {
            nq as ::core::ffi::c_long
        })
    {
        *info = -(7 as ::core::ffi::c_int) as integer;
    } else if *ldc
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        *info = -(10 as ::core::ffi::c_int) as integer;
    } else if *lwork
        < (if 1 as ::core::ffi::c_long >= nw {
            1 as ::core::ffi::c_long
        } else {
            nw as ::core::ffi::c_long
        })
        && lquery == 0
    {
        *info = -(12 as ::core::ffi::c_int) as integer;
    }
    if *info == 0 as ::core::ffi::c_long {
        if upper != 0 {
            if left != 0 {
                i__1[0 as ::core::ffi::c_int as usize] = 1 as integer;
                a__1[0 as ::core::ffi::c_int as usize] = side as address;
                i__1[1 as ::core::ffi::c_int as usize] = 1 as integer;
                a__1[1 as ::core::ffi::c_int as usize] = trans as address;
                s_cat_0(
                    &raw mut ch__1 as *mut ::core::ffi::c_char,
                    &raw mut a__1 as *mut *mut ::core::ffi::c_char,
                    &raw mut i__1 as *mut integer,
                    &raw mut c__2,
                    2 as ::core::ffi::c_int as ftnlen,
                );
                i__2 = (*m - 1 as ::core::ffi::c_long) as integer;
                i__3 = (*m - 1 as ::core::ffi::c_long) as integer;
                nb = ilaenv__0(
                    &raw mut c__1,
                    b"DORMQL\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut ch__1 as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    n,
                    &raw mut i__3,
                    &raw mut c_n1,
                );
            } else {
                i__1[0 as ::core::ffi::c_int as usize] = 1 as integer;
                a__1[0 as ::core::ffi::c_int as usize] = side as address;
                i__1[1 as ::core::ffi::c_int as usize] = 1 as integer;
                a__1[1 as ::core::ffi::c_int as usize] = trans as address;
                s_cat_0(
                    &raw mut ch__1 as *mut ::core::ffi::c_char,
                    &raw mut a__1 as *mut *mut ::core::ffi::c_char,
                    &raw mut i__1 as *mut integer,
                    &raw mut c__2,
                    2 as ::core::ffi::c_int as ftnlen,
                );
                i__2 = (*n - 1 as ::core::ffi::c_long) as integer;
                i__3 = (*n - 1 as ::core::ffi::c_long) as integer;
                nb = ilaenv__0(
                    &raw mut c__1,
                    b"DORMQL\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut ch__1 as *mut ::core::ffi::c_char,
                    m,
                    &raw mut i__2,
                    &raw mut i__3,
                    &raw mut c_n1,
                );
            }
        } else if left != 0 {
            i__1[0 as ::core::ffi::c_int as usize] = 1 as integer;
            a__1[0 as ::core::ffi::c_int as usize] = side as address;
            i__1[1 as ::core::ffi::c_int as usize] = 1 as integer;
            a__1[1 as ::core::ffi::c_int as usize] = trans as address;
            s_cat_0(
                &raw mut ch__1 as *mut ::core::ffi::c_char,
                &raw mut a__1 as *mut *mut ::core::ffi::c_char,
                &raw mut i__1 as *mut integer,
                &raw mut c__2,
                2 as ::core::ffi::c_int as ftnlen,
            );
            i__2 = (*m - 1 as ::core::ffi::c_long) as integer;
            i__3 = (*m - 1 as ::core::ffi::c_long) as integer;
            nb = ilaenv__0(
                &raw mut c__1,
                b"DORMQR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut ch__1 as *mut ::core::ffi::c_char,
                &raw mut i__2,
                n,
                &raw mut i__3,
                &raw mut c_n1,
            );
        } else {
            i__1[0 as ::core::ffi::c_int as usize] = 1 as integer;
            a__1[0 as ::core::ffi::c_int as usize] = side as address;
            i__1[1 as ::core::ffi::c_int as usize] = 1 as integer;
            a__1[1 as ::core::ffi::c_int as usize] = trans as address;
            s_cat_0(
                &raw mut ch__1 as *mut ::core::ffi::c_char,
                &raw mut a__1 as *mut *mut ::core::ffi::c_char,
                &raw mut i__1 as *mut integer,
                &raw mut c__2,
                2 as ::core::ffi::c_int as ftnlen,
            );
            i__2 = (*n - 1 as ::core::ffi::c_long) as integer;
            i__3 = (*n - 1 as ::core::ffi::c_long) as integer;
            nb = ilaenv__0(
                &raw mut c__1,
                b"DORMQR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut ch__1 as *mut ::core::ffi::c_char,
                m,
                &raw mut i__2,
                &raw mut i__3,
                &raw mut c_n1,
            );
        }
        lwkopt = (if 1 as ::core::ffi::c_long >= nw {
            1 as integer
        } else {
            nw
        }) * nb;
        *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__2 = -*info;
        xerbla__0(
            b"DORMTR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__2,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *m == 0 as ::core::ffi::c_long
        || *n == 0 as ::core::ffi::c_long
        || nq == 1 as ::core::ffi::c_long
    {
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    if left != 0 {
        mi = (*m - 1 as ::core::ffi::c_long) as integer;
        ni = *n;
    } else {
        mi = *m;
        ni = (*n - 1 as ::core::ffi::c_long) as integer;
    }
    if upper != 0 {
        i__2 = (nq as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        dormql__0(
            side,
            trans,
            &raw mut mi,
            &raw mut ni,
            &raw mut i__2,
            a.offset(
                (((a_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            lda,
            tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            c__.offset(c_offset as isize) as *mut doublereal,
            ldc,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            lwork,
            &raw mut iinfo,
        );
    } else {
        if left != 0 {
            i1 = 2 as integer;
            i2 = 1 as integer;
        } else {
            i1 = 1 as integer;
            i2 = 2 as integer;
        }
        i__2 = (nq as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        dormqr__0(
            side,
            trans,
            &raw mut mi,
            &raw mut ni,
            &raw mut i__2,
            a.offset((a_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            lda,
            tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            c__.offset((i1 + i2 * c_dim1) as isize) as *mut doublereal,
            ldc,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            lwork,
            &raw mut iinfo,
        );
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    return 0 as ::core::ffi::c_int;
}
