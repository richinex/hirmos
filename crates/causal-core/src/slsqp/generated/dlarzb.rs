pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
static mut c__1: integer = 1 as integer;
static mut c_b13: doublereal = 1.0f64;
static mut c_b23: doublereal = -1.0f64;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlarzb_(
    mut side: *mut ::core::ffi::c_char,
    mut trans: *mut ::core::ffi::c_char,
    mut direct: *mut ::core::ffi::c_char,
    mut storev: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut k: *mut integer,
    mut l: *mut integer,
    mut v: *mut doublereal,
    mut ldv: *mut integer,
    mut t: *mut doublereal,
    mut ldt: *mut integer,
    mut c__: *mut doublereal,
    mut ldc: *mut integer,
    mut work: *mut doublereal,
    mut ldwork: *mut integer,
) -> ::core::ffi::c_int {
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut t_dim1: integer = 0;
    let mut t_offset: integer = 0;
    let mut v_dim1: integer = 0;
    let mut v_offset: integer = 0;
    let mut work_dim1: integer = 0;
    let mut work_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut info: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dgemm"]
        fn dgelsd_closure_f2c_dgemm_0(
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
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn dgelsd_closure_lsame__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
        ) -> logical;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dcopy"]
        fn dgelsd_closure_f2c_dcopy_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_dtrmm"]
        fn dsyevd_closure_f2c_dtrmm_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_xerbla_"]
        fn dgelsd_closure_xerbla__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut transt: [::core::ffi::c_char; 1] = [0; 1];
    v_dim1 = *ldv;
    v_offset = 1 as integer + v_dim1;
    v = v.offset(-(v_offset as isize));
    t_dim1 = *ldt;
    t_offset = 1 as integer + t_dim1;
    t = t.offset(-(t_offset as isize));
    c_dim1 = *ldc;
    c_offset = 1 as integer + c_dim1;
    c__ = c__.offset(-(c_offset as isize));
    work_dim1 = *ldwork;
    work_offset = 1 as integer + work_dim1;
    work = work.offset(-(work_offset as isize));
    if *m <= 0 as ::core::ffi::c_long || *n <= 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    info = 0 as integer;
    if dgelsd_closure_lsame__0(
        direct,
        b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
    {
        info = -(3 as ::core::ffi::c_int) as integer;
    } else if dgelsd_closure_lsame__0(
        storev,
        b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
    {
        info = -(4 as ::core::ffi::c_int) as integer;
    }
    if info != 0 as ::core::ffi::c_long {
        i__1 = -info;
        dgelsd_closure_xerbla__0(
            b"DLARZB\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    if dgelsd_closure_lsame__0(
        trans,
        b"N\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        *(&raw mut transt as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
            'T' as i32 as ::core::ffi::c_uchar;
    } else {
        *(&raw mut transt as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar) =
            'N' as i32 as ::core::ffi::c_uchar;
    }
    if dgelsd_closure_lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        i__1 = *k;
        j = 1 as integer;
        while j <= i__1 {
            dgelsd_closure_f2c_dcopy_0(
                n,
                c__.offset((j + c_dim1) as isize) as *mut doublereal,
                ldc,
                work.offset(
                    (j as ::core::ffi::c_long * work_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
            );
            j += 1;
        }
        if *l > 0 as ::core::ffi::c_long {
            dgelsd_closure_f2c_dgemm_0(
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                n,
                k,
                l,
                &raw mut c_b13,
                c__.offset((*m - *l + 1 as integer + c_dim1) as isize) as *mut doublereal,
                ldc,
                v.offset(v_offset as isize) as *mut doublereal,
                ldv,
                &raw mut c_b13,
                work.offset(work_offset as isize) as *mut doublereal,
                ldwork,
            );
        }
        dsyevd_closure_f2c_dtrmm_0(
            b"Right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"Lower\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut transt as *mut ::core::ffi::c_char,
            b"Non-unit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n,
            k,
            &raw mut c_b13,
            t.offset(t_offset as isize) as *mut doublereal,
            ldt,
            work.offset(work_offset as isize) as *mut doublereal,
            ldwork,
        );
        i__1 = *n;
        j = 1 as integer;
        while j <= i__1 {
            i__2 = *k;
            i__ = 1 as integer;
            while i__ <= i__2 {
                let ref mut fresh0 = *c__.offset((i__ + j * c_dim1) as isize);
                *fresh0 -= *work.offset((j + i__ * work_dim1) as isize) as ::core::ffi::c_double;
                i__ += 1;
            }
            j += 1;
        }
        if *l > 0 as ::core::ffi::c_long {
            dgelsd_closure_f2c_dgemm_0(
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                l,
                n,
                k,
                &raw mut c_b23,
                v.offset(v_offset as isize) as *mut doublereal,
                ldv,
                work.offset(work_offset as isize) as *mut doublereal,
                ldwork,
                &raw mut c_b13,
                c__.offset((*m - *l + 1 as integer + c_dim1) as isize) as *mut doublereal,
                ldc,
            );
        }
    } else if dgelsd_closure_lsame__0(
        side,
        b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        i__1 = *k;
        j = 1 as integer;
        while j <= i__1 {
            dgelsd_closure_f2c_dcopy_0(
                m,
                c__.offset(
                    (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                work.offset(
                    (j as ::core::ffi::c_long * work_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
            );
            j += 1;
        }
        if *l > 0 as ::core::ffi::c_long {
            dgelsd_closure_f2c_dgemm_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                m,
                k,
                l,
                &raw mut c_b13,
                c__.offset(
                    ((*n - *l + 1 as ::core::ffi::c_long) * c_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                ldc,
                v.offset(v_offset as isize) as *mut doublereal,
                ldv,
                &raw mut c_b13,
                work.offset(work_offset as isize) as *mut doublereal,
                ldwork,
            );
        }
        dsyevd_closure_f2c_dtrmm_0(
            b"Right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"Lower\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            trans,
            b"Non-unit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            m,
            k,
            &raw mut c_b13,
            t.offset(t_offset as isize) as *mut doublereal,
            ldt,
            work.offset(work_offset as isize) as *mut doublereal,
            ldwork,
        );
        i__1 = *k;
        j = 1 as integer;
        while j <= i__1 {
            i__2 = *m;
            i__ = 1 as integer;
            while i__ <= i__2 {
                let ref mut fresh1 = *c__.offset((i__ + j * c_dim1) as isize);
                *fresh1 -= *work.offset((i__ + j * work_dim1) as isize) as ::core::ffi::c_double;
                i__ += 1;
            }
            j += 1;
        }
        if *l > 0 as ::core::ffi::c_long {
            dgelsd_closure_f2c_dgemm_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                m,
                l,
                k,
                &raw mut c_b23,
                work.offset(work_offset as isize) as *mut doublereal,
                ldwork,
                v.offset(v_offset as isize) as *mut doublereal,
                ldv,
                &raw mut c_b13,
                c__.offset(
                    ((*n - *l + 1 as ::core::ffi::c_long) * c_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                ldc,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
