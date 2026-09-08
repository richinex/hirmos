#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sqrt(_: doublereal) -> ::core::ffi::c_double;
}
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlansy_(
    mut norm: *mut ::core::ffi::c_char,
    mut uplo: *mut ::core::ffi::c_char,
    mut n: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut work: *mut doublereal,
) -> doublereal {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut ret_val: doublereal = 0.;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut sum: doublereal = 0.;
    let mut absa: doublereal = 0.;
    let mut scale: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    let mut value: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_dlassq_"]
        fn dlassq__0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    work = work.offset(-1);
    if *n == 0 as ::core::ffi::c_long {
        value = 0.0f64 as doublereal;
    } else if lsame__0(
        norm,
        b"M\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        value = 0.0f64 as doublereal;
        if lsame__0(
            uplo,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                i__2 = j;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    d__2 = value;
                    d__1 = *a.offset((i__ + j * a_dim1) as isize);
                    d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                    value = (if d__2 >= d__3 {
                        d__2 as ::core::ffi::c_double
                    } else {
                        d__3 as ::core::ffi::c_double
                    }) as doublereal;
                    i__ += 1;
                }
                j += 1;
            }
        } else {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                i__2 = *n;
                i__ = j;
                while i__ <= i__2 {
                    d__2 = value;
                    d__1 = *a.offset((i__ + j * a_dim1) as isize);
                    d__3 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                    value = (if d__2 >= d__3 {
                        d__2 as ::core::ffi::c_double
                    } else {
                        d__3 as ::core::ffi::c_double
                    }) as doublereal;
                    i__ += 1;
                }
                j += 1;
            }
        }
    } else if lsame__0(
        norm,
        b"I\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
        || lsame__0(
            norm,
            b"O\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        || *(norm as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int == '1' as i32
    {
        value = 0.0f64 as doublereal;
        if lsame__0(
            uplo,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                sum = 0.0f64 as doublereal;
                i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                i__ = 1 as integer;
                while i__ <= i__2 {
                    d__1 = *a.offset((i__ + j * a_dim1) as isize);
                    absa = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                    sum += absa as ::core::ffi::c_double;
                    let ref mut fresh0 = *work.offset(i__ as isize);
                    *fresh0 += absa as ::core::ffi::c_double;
                    i__ += 1;
                }
                d__1 = *a.offset((j + j * a_dim1) as isize);
                *work.offset(j as isize) = (sum as ::core::ffi::c_double
                    + (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    })) as doublereal;
                j += 1;
            }
            i__1 = *n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                d__1 = value;
                d__2 = *work.offset(i__ as isize);
                value = (if d__1 >= d__2 {
                    d__1 as ::core::ffi::c_double
                } else {
                    d__2 as ::core::ffi::c_double
                }) as doublereal;
                i__ += 1;
            }
        } else {
            i__1 = *n;
            i__ = 1 as integer;
            while i__ <= i__1 {
                *work.offset(i__ as isize) = 0.0f64 as doublereal;
                i__ += 1;
            }
            i__1 = *n;
            j = 1 as integer;
            while j <= i__1 {
                d__1 = *a.offset((j + j * a_dim1) as isize);
                sum = (*work.offset(j as isize) as ::core::ffi::c_double
                    + (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    })) as doublereal;
                i__2 = *n;
                i__ = (j as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
                while i__ <= i__2 {
                    d__1 = *a.offset((i__ + j * a_dim1) as isize);
                    absa = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1 as ::core::ffi::c_double
                    } else {
                        -(d__1 as ::core::ffi::c_double)
                    }) as doublereal;
                    sum += absa as ::core::ffi::c_double;
                    let ref mut fresh1 = *work.offset(i__ as isize);
                    *fresh1 += absa as ::core::ffi::c_double;
                    i__ += 1;
                }
                value = (if value >= sum {
                    value as ::core::ffi::c_double
                } else {
                    sum as ::core::ffi::c_double
                }) as doublereal;
                j += 1;
            }
        }
    } else if lsame__0(
        norm,
        b"F\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
        || lsame__0(
            norm,
            b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
    {
        scale = 0.0f64 as doublereal;
        sum = 1.0f64 as doublereal;
        if lsame__0(
            uplo,
            b"U\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            i__1 = *n;
            j = 2 as integer;
            while j <= i__1 {
                i__2 = (j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
                dlassq__0(
                    &raw mut i__2,
                    a.offset(
                        (j as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut scale,
                    &raw mut sum,
                );
                j += 1;
            }
        } else {
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            j = 1 as integer;
            while j <= i__1 {
                i__2 = *n - j;
                dlassq__0(
                    &raw mut i__2,
                    a.offset((j + 1 as integer + j * a_dim1) as isize) as *mut doublereal,
                    &raw mut c__1,
                    &raw mut scale,
                    &raw mut sum,
                );
                j += 1;
            }
        }
        sum *= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        i__1 = (*lda + 1 as ::core::ffi::c_long) as integer;
        dlassq__0(
            n,
            a.offset(a_offset as isize) as *mut doublereal,
            &raw mut i__1,
            &raw mut scale,
            &raw mut sum,
        );
        value = (scale as ::core::ffi::c_double * sqrt(sum)) as doublereal;
    }
    ret_val = value;
    return ret_val;
}
