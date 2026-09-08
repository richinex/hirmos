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
static mut c_b21: doublereal = -1.0f64;
static mut c_b22: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dgebrd_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut d__: *mut doublereal,
    mut e: *mut doublereal,
    mut tauq: *mut doublereal,
    mut taup: *mut doublereal,
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
    let mut j: integer = 0;
    let mut nb: integer = 0;
    let mut nx: integer = 0;
    let mut ws: doublereal = 0.;
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
    let mut nbmin: integer = 0;
    let mut iinfo: integer = 0;
    let mut minmn: integer = 0;
    extern "C" {
        #[link_name = "dgesdd_closure_dgebd2_"]
        fn dgebd2__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgesdd_closure_dlabrd_"]
        fn dlabrd__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
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
    let mut ldwrkx: integer = 0;
    let mut ldwrky: integer = 0;
    let mut lwkopt: integer = 0;
    let mut lquery: logical = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    d__ = d__.offset(-1);
    e = e.offset(-1);
    tauq = tauq.offset(-1);
    taup = taup.offset(-1);
    work = work.offset(-1);
    *info = 0 as integer;
    i__1 = 1 as integer;
    i__2 = ilaenv__0(
        &raw mut c__1,
        b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        m,
        n,
        &raw mut c_n1,
        &raw mut c_n1,
    );
    nb = (if i__1 >= i__2 {
        i__1 as ::core::ffi::c_long
    } else {
        i__2 as ::core::ffi::c_long
    }) as integer;
    lwkopt = (*m + *n) * nb;
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
    } else {
        i__1 = (if 1 as ::core::ffi::c_long >= *m {
            1 as ::core::ffi::c_long
        } else {
            *m
        }) as integer;
        if *lwork
            < (if i__1 >= *n {
                i__1 as ::core::ffi::c_long
            } else {
                *n
            })
            && lquery == 0
        {
            *info = -(10 as ::core::ffi::c_int) as integer;
        }
    }
    if *info < 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    minmn = (if *m <= *n { *m } else { *n }) as integer;
    if minmn == 0 as ::core::ffi::c_long {
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    ws = (if *m >= *n { *m } else { *n }) as doublereal;
    ldwrkx = *m;
    ldwrky = *n;
    if nb > 1 as ::core::ffi::c_long && nb < minmn {
        i__1 = nb;
        i__2 = ilaenv__0(
            &raw mut c__3,
            b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
        if nx < minmn {
            ws = ((*m + *n) * nb) as doublereal;
            if (*lwork as doublereal) < ws {
                nbmin = ilaenv__0(
                    &raw mut c__2,
                    b"DGEBRD\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    m,
                    n,
                    &raw mut c_n1,
                    &raw mut c_n1,
                );
                if *lwork >= (*m + *n) * nbmin {
                    nb = *lwork / (*m + *n);
                } else {
                    nb = 1 as integer;
                    nx = minmn;
                }
            }
        }
    } else {
        nx = minmn;
    }
    i__1 = minmn - nx;
    i__2 = nb;
    i__ = 1 as integer;
    while if i__2 < 0 as ::core::ffi::c_long {
        (i__ >= i__1) as ::core::ffi::c_int
    } else {
        (i__ <= i__1) as ::core::ffi::c_int
    } != 0
    {
        i__3 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        i__4 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        dlabrd__0(
            &raw mut i__3,
            &raw mut i__4,
            &raw mut nb,
            a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
            lda,
            d__.offset(i__ as isize) as *mut doublereal,
            e.offset(i__ as isize) as *mut doublereal,
            tauq.offset(i__ as isize) as *mut doublereal,
            taup.offset(i__ as isize) as *mut doublereal,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut ldwrkx,
            work.offset(
                (ldwrkx as ::core::ffi::c_long * nb as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut ldwrky,
        );
        i__3 = (*m - i__ as ::core::ffi::c_long - nb as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as integer;
        i__4 = (*n - i__ as ::core::ffi::c_long - nb as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as integer;
        f2c_dgemm_0(
            b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"Transpose\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__3,
            &raw mut i__4,
            &raw mut nb,
            &raw mut c_b21,
            a.offset((i__ + nb + i__ * a_dim1) as isize) as *mut doublereal,
            lda,
            work.offset(
                (ldwrkx as ::core::ffi::c_long * nb as ::core::ffi::c_long
                    + nb as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            &raw mut ldwrky,
            &raw mut c_b22,
            a.offset((i__ + nb + (i__ + nb) * a_dim1) as isize) as *mut doublereal,
            lda,
        );
        i__3 = (*m - i__ as ::core::ffi::c_long - nb as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as integer;
        i__4 = (*n - i__ as ::core::ffi::c_long - nb as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as integer;
        f2c_dgemm_0(
            b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut i__3,
            &raw mut i__4,
            &raw mut nb,
            &raw mut c_b21,
            work.offset((nb as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                as *mut doublereal,
            &raw mut ldwrkx,
            a.offset((i__ + (i__ + nb) * a_dim1) as isize) as *mut doublereal,
            lda,
            &raw mut c_b22,
            a.offset((i__ + nb + (i__ + nb) * a_dim1) as isize) as *mut doublereal,
            lda,
        );
        if *m >= *n {
            i__3 = (i__ as ::core::ffi::c_long + nb as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as integer;
            j = i__;
            while j <= i__3 {
                *a.offset((j + j * a_dim1) as isize) = *d__.offset(j as isize);
                *a.offset((j + (j + 1 as integer) * a_dim1) as isize) = *e.offset(j as isize);
                j += 1;
            }
        } else {
            i__3 = (i__ as ::core::ffi::c_long + nb as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as integer;
            j = i__;
            while j <= i__3 {
                *a.offset((j + j * a_dim1) as isize) = *d__.offset(j as isize);
                *a.offset((j + 1 as integer + j * a_dim1) as isize) = *e.offset(j as isize);
                j += 1;
            }
        }
        i__ += i__2 as ::core::ffi::c_long;
    }
    i__2 = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    i__1 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
    dgebd2__0(
        &raw mut i__2,
        &raw mut i__1,
        a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
        lda,
        d__.offset(i__ as isize) as *mut doublereal,
        e.offset(i__ as isize) as *mut doublereal,
        tauq.offset(i__ as isize) as *mut doublereal,
        taup.offset(i__ as isize) as *mut doublereal,
        work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        &raw mut iinfo,
    );
    *work.offset(1 as ::core::ffi::c_int as isize) = ws;
    return 0 as ::core::ffi::c_int;
}
