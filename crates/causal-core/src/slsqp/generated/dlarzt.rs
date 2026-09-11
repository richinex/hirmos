pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
static mut c_b8: doublereal = 0.0f64;
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlarzt_(
    mut direct: *mut ::core::ffi::c_char,
    mut storev: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut k: *mut integer,
    mut v: *mut doublereal,
    mut ldv: *mut integer,
    mut tau: *mut doublereal,
    mut t: *mut doublereal,
    mut ldt: *mut integer,
) -> ::core::ffi::c_int {
    let mut t_dim1: integer = 0;
    let mut t_offset: integer = 0;
    let mut v_dim1: integer = 0;
    let mut v_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut info: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn dgelsd_closure_lsame__0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
        ) -> logical;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dgemv"]
        fn dgelsd_closure_f2c_dgemv_0(
            _: *mut ::core::ffi::c_char,
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
        #[link_name = "dsyevd_closure_f2c_dtrmv"]
        fn dsyevd_closure_f2c_dtrmv_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
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
    v_dim1 = *ldv;
    v_offset = 1 as integer + v_dim1;
    v = v.offset(-(v_offset as isize));
    tau = tau.offset(-1);
    t_dim1 = *ldt;
    t_offset = 1 as integer + t_dim1;
    t = t.offset(-(t_offset as isize));
    info = 0 as integer;
    if dgelsd_closure_lsame__0(
        direct,
        b"B\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
    {
        info = -(1 as ::core::ffi::c_int) as integer;
    } else if dgelsd_closure_lsame__0(
        storev,
        b"R\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) == 0
    {
        info = -(2 as ::core::ffi::c_int) as integer;
    }
    if info != 0 as ::core::ffi::c_long {
        i__1 = -info;
        dgelsd_closure_xerbla__0(
            b"DLARZT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
        );
        return 0 as ::core::ffi::c_int;
    }
    i__ = *k;
    while i__ >= 1 as ::core::ffi::c_long {
        if *tau.offset(i__ as isize) == 0.0f64 {
            i__1 = *k;
            j = i__;
            while j <= i__1 {
                *t.offset((j + i__ * t_dim1) as isize) = 0.0f64 as doublereal;
                j += 1;
            }
        } else {
            if i__ < *k {
                i__1 = *k - i__;
                d__1 = -*tau.offset(i__ as isize);
                dgelsd_closure_f2c_dgemv_0(
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__1,
                    n,
                    &raw mut d__1,
                    v.offset((i__ + 1 as integer + v_dim1) as isize) as *mut doublereal,
                    ldv,
                    v.offset((i__ + v_dim1) as isize) as *mut doublereal,
                    ldv,
                    &raw mut c_b8,
                    t.offset((i__ + 1 as integer + i__ * t_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
                i__1 = *k - i__;
                dsyevd_closure_f2c_dtrmv_0(
                    b"Lower\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"Non-unit\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__1,
                    t.offset((i__ + 1 as integer + (i__ + 1 as integer) * t_dim1) as isize)
                        as *mut doublereal,
                    ldt,
                    t.offset((i__ + 1 as integer + i__ * t_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                );
            }
            *t.offset((i__ + i__ * t_dim1) as isize) = *tau.offset(i__ as isize);
        }
        i__ -= 1;
    }
    return 0 as ::core::ffi::c_int;
}
