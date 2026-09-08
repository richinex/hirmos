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
static mut c__65: integer = 65 as integer;
#[no_mangle]
pub unsafe extern "C" fn dgesdd_closure_dormlq_(
    mut side: *mut ::core::ffi::c_char,
    mut trans: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut k: *mut integer,
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
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: [integer; 2] = [0; 2];
    let mut i__4: integer = 0;
    let mut i__5: integer = 0;
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
    let mut i__: integer = 0;
    let mut t: [doublereal; 4160] = [0.; 4160];
    let mut i1: integer = 0;
    let mut i2: integer = 0;
    let mut i3: integer = 0;
    let mut ib: integer = 0;
    let mut ic: integer = 0;
    let mut jc: integer = 0;
    let mut nb: integer = 0;
    let mut mi: integer = 0;
    let mut ni: integer = 0;
    let mut nq: integer = 0;
    let mut nw: integer = 0;
    let mut iws: integer = 0;
    let mut left: logical = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut nbmin: integer = 0;
    let mut iinfo: integer = 0;
    extern "C" {
        #[link_name = "dgesdd_closure_dorml2_"]
        fn dorml2__0(
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
    let mut notran: logical = 0;
    let mut ldwork: integer = 0;
    let mut transt: [::core::ffi::c_char; 1] = [0; 1];
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
    notran = lsame__0(
        trans,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
    } else if notran == 0
        && lsame__0(
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
    } else if *lda
        < (if 1 as ::core::ffi::c_long >= *k {
            1 as ::core::ffi::c_long
        } else {
            *k
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
        i__3[0 as ::core::ffi::c_int as usize] = 1 as integer;
        a__1[0 as ::core::ffi::c_int as usize] = side as address;
        i__3[1 as ::core::ffi::c_int as usize] = 1 as integer;
        a__1[1 as ::core::ffi::c_int as usize] = trans as address;
        s_cat_0(
            &raw mut ch__1 as *mut ::core::ffi::c_char,
            &raw mut a__1 as *mut *mut ::core::ffi::c_char,
            &raw mut i__3 as *mut integer,
            &raw mut c__2,
            2 as ::core::ffi::c_int as ftnlen,
        );
        i__1 = 64 as integer;
        i__2 = ilaenv__0(
            &raw mut c__1,
            b"DORMLQ\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut ch__1 as *mut ::core::ffi::c_char,
            m,
            n,
            k,
            &raw mut c_n1,
        );
        nb = (if i__1 <= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        }) as integer;
        lwkopt = (if 1 as ::core::ffi::c_long >= nw {
            1 as integer
        } else {
            nw
        }) * nb;
        *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    }
    if *info != 0 as ::core::ffi::c_long {
        i__1 = -*info;
        xerbla__0(
            b"DORMLQ\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    } else if lquery != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *m == 0 as ::core::ffi::c_long
        || *n == 0 as ::core::ffi::c_long
        || *k == 0 as ::core::ffi::c_long
    {
        *work.offset(1 as ::core::ffi::c_int as isize) = 1.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    nbmin = 2 as integer;
    ldwork = nw;
    if nb > 1 as ::core::ffi::c_long && nb < *k {
        iws = nw * nb;
        if *lwork < iws {
            nb = *lwork / ldwork;
            i__3[0 as ::core::ffi::c_int as usize] = 1 as integer;
            a__1[0 as ::core::ffi::c_int as usize] = side as address;
            i__3[1 as ::core::ffi::c_int as usize] = 1 as integer;
            a__1[1 as ::core::ffi::c_int as usize] = trans as address;
            s_cat_0(
                &raw mut ch__1 as *mut ::core::ffi::c_char,
                &raw mut a__1 as *mut *mut ::core::ffi::c_char,
                &raw mut i__3 as *mut integer,
                &raw mut c__2,
                2 as ::core::ffi::c_int as ftnlen,
            );
            i__1 = 2 as integer;
            i__2 = ilaenv__0(
                &raw mut c__2,
                b"DORMLQ\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut ch__1 as *mut ::core::ffi::c_char,
                m,
                n,
                k,
                &raw mut c_n1,
            );
            nbmin = (if i__1 >= i__2 {
                i__1 as ::core::ffi::c_long
            } else {
                i__2 as ::core::ffi::c_long
            }) as integer;
        }
    } else {
        iws = nw;
    }
    if nb < nbmin || nb >= *k {
        dorml2__0(
            side,
            trans,
            m,
            n,
            k,
            a.offset(a_offset as isize) as *mut doublereal,
            lda,
            tau.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            c__.offset(c_offset as isize) as *mut doublereal,
            ldc,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut iinfo,
        );
    } else {
        if left != 0 && notran != 0 || left == 0 && notran == 0 {
            i1 = 1 as integer;
            i2 = *k;
            i3 = nb;
        } else {
            i1 = ((*k - 1 as ::core::ffi::c_long) / nb as ::core::ffi::c_long
                * nb as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            i2 = 1 as integer;
            i3 = -nb;
        }
        if left != 0 {
            ni = *n;
            jc = 1 as integer;
        } else {
            mi = *m;
            ic = 1 as integer;
        }
        if notran != 0 {
            *(&raw mut transt as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                'T' as i32 as ::core::ffi::c_uchar;
        } else {
            *(&raw mut transt as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
                'N' as i32 as ::core::ffi::c_uchar;
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
            i__4 = nb;
            i__5 = (*k - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            ib = (if i__4 <= i__5 {
                i__4 as ::core::ffi::c_long
            } else {
                i__5 as ::core::ffi::c_long
            }) as integer;
            i__4 = (nq as ::core::ffi::c_long - i__ as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as integer;
            dlarft__0(
                b"Forward\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"Rowwise\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__4,
                &raw mut ib,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
                tau.offset(i__ as isize) as *mut doublereal,
                &raw mut t as *mut doublereal,
                &raw mut c__65,
            );
            if left != 0 {
                mi = (*m - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                ic = i__;
            } else {
                ni = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                jc = i__;
            }
            dlarfb__0(
                side,
                &raw mut transt as *mut ::core::ffi::c_char,
                b"Forward\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"Rowwise\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut mi,
                &raw mut ni,
                &raw mut ib,
                a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
                lda,
                &raw mut t as *mut doublereal,
                &raw mut c__65,
                c__.offset((ic + jc * c_dim1) as isize) as *mut doublereal,
                ldc,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut ldwork,
            );
            i__ += i__2 as ::core::ffi::c_long;
        }
    }
    *work.offset(1 as ::core::ffi::c_int as isize) = lwkopt as doublereal;
    return 0 as ::core::ffi::c_int;
}
