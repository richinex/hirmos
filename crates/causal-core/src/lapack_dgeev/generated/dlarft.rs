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
static mut c_b8: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlarft_(
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
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut prevlastv: integer = 0;
    let mut vii: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dgemv"]
        fn f2c_dgemv_0(
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
    let mut lastv: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_dtrmv"]
        fn f2c_dtrmv_0(
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
    v_dim1 = *ldv;
    v_offset = 1 as integer + v_dim1;
    v = v.offset(-(v_offset as isize));
    tau = tau.offset(-1);
    t_dim1 = *ldt;
    t_offset = 1 as integer + t_dim1;
    t = t.offset(-(t_offset as isize));
    if *n == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if lsame__0(
        direct,
        b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        prevlastv = *n;
        i__1 = *k;
        i__ = 1 as integer;
        while i__ <= i__1 {
            prevlastv = (if i__ >= prevlastv {
                i__ as ::core::ffi::c_long
            } else {
                prevlastv as ::core::ffi::c_long
            }) as integer;
            if *tau.offset(i__ as isize) == 0.0f64 {
                i__2 = i__;
                j = 1 as integer;
                while j <= i__2 {
                    *t.offset((j + i__ * t_dim1) as isize) = 0.0f64 as doublereal;
                    j += 1;
                }
            } else {
                vii = *v.offset((i__ + i__ * v_dim1) as isize);
                *v.offset((i__ + i__ * v_dim1) as isize) = 1.0f64 as doublereal;
                if lsame__0(
                    storev,
                    b"C\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                ) != 0
                {
                    i__2 = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    lastv = *n;
                    while lastv >= i__2 {
                        if *v.offset((lastv + i__ * v_dim1) as isize) != 0.0f64 {
                            break;
                        }
                        lastv -= 1;
                    }
                    j = (if lastv <= prevlastv {
                        lastv as ::core::ffi::c_long
                    } else {
                        prevlastv as ::core::ffi::c_long
                    }) as integer;
                    i__2 = (j as ::core::ffi::c_long - i__ as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                    i__3 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    d__1 = -*tau.offset(i__ as isize);
                    f2c_dgemv_0(
                        b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut i__2,
                        &raw mut i__3,
                        &raw mut d__1,
                        v.offset((i__ + v_dim1) as isize) as *mut doublereal,
                        ldv,
                        v.offset((i__ + i__ * v_dim1) as isize) as *mut doublereal,
                        &raw mut c__1,
                        &raw mut c_b8,
                        t.offset(
                            (i__ as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                } else {
                    i__2 = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                    lastv = *n;
                    while lastv >= i__2 {
                        if *v.offset((i__ + lastv * v_dim1) as isize) != 0.0f64 {
                            break;
                        }
                        lastv -= 1;
                    }
                    j = (if lastv <= prevlastv {
                        lastv as ::core::ffi::c_long
                    } else {
                        prevlastv as ::core::ffi::c_long
                    }) as integer;
                    i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                    i__3 = (j as ::core::ffi::c_long - i__ as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as integer;
                    d__1 = -*tau.offset(i__ as isize);
                    f2c_dgemv_0(
                        b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut i__2,
                        &raw mut i__3,
                        &raw mut d__1,
                        v.offset(
                            (i__ as ::core::ffi::c_long * v_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        ldv,
                        v.offset((i__ + i__ * v_dim1) as isize) as *mut doublereal,
                        ldv,
                        &raw mut c_b8,
                        t.offset(
                            (i__ as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        ) as *mut doublereal,
                        &raw mut c__1,
                    );
                }
                *v.offset((i__ + i__ * v_dim1) as isize) = vii;
                i__2 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                f2c_dtrmv_0(
                    b"Upper\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"Non-unit\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut i__2,
                    t.offset(t_offset as isize) as *mut doublereal,
                    ldt,
                    t.offset(
                        (i__ as ::core::ffi::c_long * t_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                );
                *t.offset((i__ + i__ * t_dim1) as isize) = *tau.offset(i__ as isize);
                if i__ > 1 as ::core::ffi::c_long {
                    prevlastv = (if prevlastv >= lastv {
                        prevlastv as ::core::ffi::c_long
                    } else {
                        lastv as ::core::ffi::c_long
                    }) as integer;
                } else {
                    prevlastv = lastv;
                }
            }
            i__ += 1;
        }
    } else {
        prevlastv = 1 as integer;
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
                    if lsame__0(
                        storev,
                        b"C\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    ) != 0
                    {
                        vii = *v.offset((*n - *k + i__ + i__ * v_dim1) as isize);
                        *v.offset((*n - *k + i__ + i__ * v_dim1) as isize) = 1.0f64 as doublereal;
                        i__1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        lastv = 1 as integer;
                        while lastv <= i__1 {
                            if *v.offset((lastv + i__ * v_dim1) as isize) != 0.0f64 {
                                break;
                            }
                            lastv += 1;
                        }
                        j = (if lastv >= prevlastv {
                            lastv as ::core::ffi::c_long
                        } else {
                            prevlastv as ::core::ffi::c_long
                        }) as integer;
                        i__1 = (*n - *k + i__ as ::core::ffi::c_long - j as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as integer;
                        i__2 = *k - i__;
                        d__1 = -*tau.offset(i__ as isize);
                        f2c_dgemv_0(
                            b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut i__1,
                            &raw mut i__2,
                            &raw mut d__1,
                            v.offset((j + (i__ + 1 as integer) * v_dim1) as isize)
                                as *mut doublereal,
                            ldv,
                            v.offset((j + i__ * v_dim1) as isize) as *mut doublereal,
                            &raw mut c__1,
                            &raw mut c_b8,
                            t.offset((i__ + 1 as integer + i__ * t_dim1) as isize)
                                as *mut doublereal,
                            &raw mut c__1,
                        );
                        *v.offset((*n - *k + i__ + i__ * v_dim1) as isize) = vii;
                    } else {
                        vii = *v.offset((i__ + (*n - *k + i__) * v_dim1) as isize);
                        *v.offset((i__ + (*n - *k + i__) * v_dim1) as isize) = 1.0f64 as doublereal;
                        i__1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                        lastv = 1 as integer;
                        while lastv <= i__1 {
                            if *v.offset((i__ + lastv * v_dim1) as isize) != 0.0f64 {
                                break;
                            }
                            lastv += 1;
                        }
                        j = (if lastv >= prevlastv {
                            lastv as ::core::ffi::c_long
                        } else {
                            prevlastv as ::core::ffi::c_long
                        }) as integer;
                        i__1 = *k - i__;
                        i__2 = (*n - *k + i__ as ::core::ffi::c_long - j as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as integer;
                        d__1 = -*tau.offset(i__ as isize);
                        f2c_dgemv_0(
                            b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut i__1,
                            &raw mut i__2,
                            &raw mut d__1,
                            v.offset((i__ + 1 as integer + j * v_dim1) as isize) as *mut doublereal,
                            ldv,
                            v.offset((i__ + j * v_dim1) as isize) as *mut doublereal,
                            ldv,
                            &raw mut c_b8,
                            t.offset((i__ + 1 as integer + i__ * t_dim1) as isize)
                                as *mut doublereal,
                            &raw mut c__1,
                        );
                        *v.offset((i__ + (*n - *k + i__) * v_dim1) as isize) = vii;
                    }
                    i__1 = *k - i__;
                    f2c_dtrmv_0(
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
                    if i__ > 1 as ::core::ffi::c_long {
                        prevlastv = (if prevlastv <= lastv {
                            prevlastv as ::core::ffi::c_long
                        } else {
                            lastv as ::core::ffi::c_long
                        }) as integer;
                    } else {
                        prevlastv = lastv;
                    }
                }
                *t.offset((i__ + i__ * t_dim1) as isize) = *tau.offset(i__ as isize);
            }
            i__ -= 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
