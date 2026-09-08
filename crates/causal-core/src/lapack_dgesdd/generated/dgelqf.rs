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
static mut c__3: integer = 3 as integer;
static mut c__2: integer = 2 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dgelqf_(
    mut m: *mut integer,
    mut n: *mut integer,
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
    let mut i__4: integer = 0;
    let mut i__: integer = 0;
    let mut k: integer = 0;
    let mut ib: integer = 0;
    let mut nb: integer = 0;
    let mut nx: integer = 0;
    let mut iws: integer = 0;
    let mut nbmin: integer = 0;
    let mut iinfo: integer = 0;
    extern "C" {
        #[link_name = "dgesdd_closure_dgelq2_"]
        fn dgelq2__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_dlarfb_"]
        fn dlarfb__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_dlarft_"]
        fn dlarft__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
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
    let mut ldwork: integer = 0;
    let mut lwkopt: integer = 0;
    let mut lquery: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    tau = tau.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    nb = ilaenv__0(
        &raw mut c__1,
        b"DGELQF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        m,
        n,
        &raw mut c_n1,
        &raw mut c_n1,
    );
    lwkopt = *m * nb;
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    lquery = (*lwork == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long) as ::core::ffi::c_int
        as logical;
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
    } else if *lwork
        < (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        })
        && lquery == 0
    {
        *info = -(7 as ::core::ffi::c_int) as integer;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DGELQF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    k = (if *m <= *n { *m } else { *n }) as integer;
    if k == 0 as ::core::ffi::c_long {
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    nbmin = 2 as integer;
    nx = 0 as integer;
    iws = *m;
    if nb > 1 as ::core::ffi::c_long && nb < k {
        i__1 = 0 as integer;
        i__2 = ilaenv__0(
            &raw mut c__3,
            b"DGELQF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            m,
            n,
            &raw mut c_n1,
            &raw mut c_n1,
        );
        nx = (if i__1 >= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        if nx < k {
            ldwork = *m;
            iws = ldwork * nb;
            if *lwork < iws {
                nb = *lwork / ldwork;
                i__1 = 2 as integer;
                i__2 = ilaenv__0(
                    &raw mut c__2,
                    b"DGELQF\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    &raw mut c_n1,
                    &raw mut c_n1,
                );
                nbmin = (if i__1 >= i__2 {
                    i__1 as ::core::ffi::c_long
                } else {
                    i__2 as ::core::ffi::c_long
                }) as integer;
            }
        }
    }
    if nb >= nbmin && nb < k && nx < k {
        i__1 = k - nx;
        i__2 = nb;
        i__ = 1 as integer;
        while if i__2 < 0 as ::core::ffi::c_long {
            (i__ >= i__1) as ::core::ffi::c_int
        } else {
            (i__ <= i__1) as ::core::ffi::c_int
        } != 0
        {
            i__3 = (k as ::core::ffi::c_long - i__ as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            ib = (if i__3 <= nb {
                i__3 as ::core::ffi::c_long
            } else {
                nb as ::core::ffi::c_long
            }) as integer;
            i__3 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgelq2__0(
                &raw mut ib,
                &raw mut i__3,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
                tau.offset(i__ as isize) as *mut doublereal,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut iinfo,
            );
            if i__ + ib <= *m {
                i__3 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dlarft__0(
                    b"Forward\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"Rowwise\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__3,
                    &raw mut ib,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    tau.offset(i__ as isize) as *mut doublereal,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut ldwork,
                );
                i__3 = (*m - i__ as ::core::ffi::c_long - ib as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as integer;
                i__4 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                dlarfb__0(
                    b"Right\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"Forward\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"Rowwise\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__3,
                    &raw mut i__4,
                    &raw mut ib,
                    a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut ldwork,
                    a.offset((i__ + ib + i__ * a_dim1) as isize) as *mut doublereal,
                    lda,
                    work.offset((ib as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                        as *mut doublereal,
                    &raw mut ldwork,
                );
            }
            i__ += i__2 as ::core::ffi::c_long;
        }
    } else {
        i__ = 1 as integer;
    }
    if i__ <= k {
        i__2 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        i__1 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dgelq2__0(
            &raw mut i__2,
            &raw mut i__1,
            a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
            lda,
            tau.offset(i__ as isize) as *mut doublereal,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut iinfo,
        );
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = iws as doublereal;
    return 0 as ::core::ffi::c_int;
}
