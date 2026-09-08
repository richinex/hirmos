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
pub unsafe extern "C" fn dgesdd_closure_dorgbr_(
    mut vect: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut k: *mut integer,
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
    let mut i__3: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut nb: integer = 0;
    let mut mn: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut iinfo: integer = 0;
    let mut wantq: logical = 0;
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
        #[link_name = "dgesdd_closure_dorglq_"]
        fn dorglq__0(
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
    extern "C" {
        #[link_name = "dgesdd_closure_dorgqr_"]
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
    wantq = lsame__0(
        vect,
        b"Q\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    mn = (if *m <= *n { *m } else { *n }) as integer;
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
    if wantq == 0
        && lsame__0(
            vect,
            b"P\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
    {
        *info = -(1 as ::core::ffi::c_int) as integer;
    } else if *m < 0 as ::core::ffi::c_long {
        *info = -(2 as ::core::ffi::c_int) as integer;
    } else if *n < 0 as ::core::ffi::c_long
        || wantq != 0 && (*n > *m || *n < (if *m <= *k { *m } else { *k }))
        || wantq == 0 && (*m > *n || *m < (if *n <= *k { *n } else { *k }))
    {
        *info = -(3 as ::core::ffi::c_int) as integer;
    } else if *k < 0 as ::core::ffi::c_long {
        *info = -(4 as ::core::ffi::c_int) as integer;
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
    {
        *info = -(6 as ::core::ffi::c_int) as integer;
    } else if *lwork
        < (if 1 as ::core::ffi::c_long >= mn {
            1 as ::core::ffi::c_long
        } else {
            mn as ::core::ffi::c_long
        })
        && lquery == 0
    {
        *info = -(9 as ::core::ffi::c_int) as integer;
    }
    if *info == 0 as ::core::ffi::c_long {
        if wantq != 0 {
            nb = ilaenv__0(
                &raw mut c__1,
                b"DORGQR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                k,
                &raw mut c_n1,
            );
        } else {
            nb = ilaenv__0(
                &raw mut c__1,
                b"DORGLQ\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                m,
                n,
                k,
                &raw mut c_n1,
            );
        }
        lwkopt = (if 1 as ::core::ffi::c_long >= mn {
            1 as integer
        } else {
            mn
        }) * nb;
        *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DORGBR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *m == 0 as ::core::ffi::c_long || *n == 0 as ::core::ffi::c_long {
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    if wantq != 0 {
        if *m >= *k {
            dorgqr__0(
                m,
                n,
                k,
                a.offset(a_offset as isize) as *mut doublereal,
                lda,
                tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                lwork,
                &raw mut iinfo,
            );
        } else {
            j = *m;
            while j >= 2 as ::core::ffi::c_long {
                *a.offset(
                    (j as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) = 0.0f64 as doublereal;
                i__1 = *m;
                i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                while i__ <= i__1 {
                    *a.offset((i__ + j * a_dim1) as isize) =
                        *a.offset((i__ + (j - 1 as integer) * a_dim1) as isize);
                    i__ += 1;
                }
                j -= 1;
            }
            *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
                1.0f64 as doublereal;
            i__1 = *m;
            i__ = 2 as integer;
            while i__ <= i__1 {
                *a.offset((i__ + a_dim1) as isize) = 0.0f64 as doublereal;
                i__ += 1;
            }
            if *m > 1 as ::core::ffi::c_long {
                i__1 = (*m - 1 as ::core::ffi::c_long) as integer;
                i__2 = (*m - 1 as ::core::ffi::c_long) as integer;
                i__3 = (*m - 1 as ::core::ffi::c_long) as integer;
                dorgqr__0(
                    &raw mut i__1,
                    &raw mut i__2,
                    &raw mut i__3,
                    a.offset(
                        (((a_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    lda,
                    tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    lwork,
                    &raw mut iinfo,
                );
            }
        }
    } else if *k < *n {
        dorglq__0(
            m,
            n,
            k,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            lwork,
            &raw mut iinfo,
        );
    } else {
        *a.offset((a_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) =
            1.0f64 as doublereal;
        i__1 = *n;
        i__ = 2 as integer;
        while i__ <= i__1 {
            *a.offset((i__ + a_dim1) as isize) = 0.0f64 as doublereal;
            i__ += 1;
        }
        i__1 = *n;
        j = 2 as integer;
        while j <= i__1 {
            i__ = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            while i__ >= 2 as ::core::ffi::c_long {
                *a.offset((i__ + j * a_dim1) as isize) =
                    *a.offset((i__ - 1 as integer + j * a_dim1) as isize);
                i__ -= 1;
            }
            *a.offset(
                (j as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) = 0.0f64 as doublereal;
            j += 1;
        }
        if *n > 1 as ::core::ffi::c_long {
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            i__2 = (*n - 1 as ::core::ffi::c_long) as integer;
            i__3 = (*n - 1 as ::core::ffi::c_long) as integer;
            dorglq__0(
                &raw mut i__1,
                &raw mut i__2,
                &raw mut i__3,
                a.offset(
                    (((a_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                lda,
                tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                lwork,
                &raw mut iinfo,
            );
        }
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    return 0 as ::core::ffi::c_int;
}
