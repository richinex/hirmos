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
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlaqr1_(
    mut n: *mut integer,
    mut h__: *mut doublereal,
    mut ldh: *mut integer,
    mut sr1: *mut doublereal,
    mut si1: *mut doublereal,
    mut sr2: *mut doublereal,
    mut si2: *mut doublereal,
    mut v: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut h_dim1: integer = 0;
    let mut h_offset: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut d__3: doublereal = 0.;
    let mut s: doublereal = 0.;
    let mut h21s: doublereal = 0.;
    let mut h31s: doublereal = 0.;
    h_dim1 = *ldh;
    h_offset = 1 as integer + h_dim1;
    h__ = h__.offset(-(h_offset as isize));
    v = v.offset(-1);
    if *n == 2 as ::core::ffi::c_long {
        d__1 =
            *h__.offset((h_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) - *sr2;
        d__2 = *h__.offset((h_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
        s = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        }) + (if *si2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *si2
        } else {
            -*si2
        }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__2 as ::core::ffi::c_double
        } else {
            -(d__2 as ::core::ffi::c_double)
        })) as doublereal;
        if s == 0.0f64 {
            *v.offset(1 as ::core::ffi::c_int as isize) = 0.0f64 as doublereal;
            *v.offset(2 as ::core::ffi::c_int as isize) = 0.0f64 as doublereal;
        } else {
            h21s = *h__.offset((h_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                / s;
            *v.offset(1 as ::core::ffi::c_int as isize) =
                h21s * *h__.offset(
                    (((h_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) + (*h__
                    .offset((h_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    - *sr1)
                    * ((*h__.offset(
                        (h_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize,
                    ) - *sr2)
                        / s)
                    - *si1 * (*si2 / s);
            *v.offset(2 as ::core::ffi::c_int as isize) = h21s
                * (*h__
                    .offset((h_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    + *h__.offset(
                        (((h_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_long) as isize,
                    )
                    - *sr1
                    - *sr2);
        }
    } else {
        d__1 =
            *h__.offset((h_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize) - *sr2;
        d__2 = *h__.offset((h_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
        d__3 = *h__.offset((h_dim1 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize);
        s = ((if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__1 as ::core::ffi::c_double
        } else {
            -(d__1 as ::core::ffi::c_double)
        }) + (if *si2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *si2
        } else {
            -*si2
        }) + (if d__2 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__2 as ::core::ffi::c_double
        } else {
            -(d__2 as ::core::ffi::c_double)
        }) + (if d__3 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            d__3 as ::core::ffi::c_double
        } else {
            -(d__3 as ::core::ffi::c_double)
        })) as doublereal;
        if s == 0.0f64 {
            *v.offset(1 as ::core::ffi::c_int as isize) = 0.0f64 as doublereal;
            *v.offset(2 as ::core::ffi::c_int as isize) = 0.0f64 as doublereal;
            *v.offset(3 as ::core::ffi::c_int as isize) = 0.0f64 as doublereal;
        } else {
            h21s = *h__.offset((h_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                / s;
            h31s = *h__.offset((h_dim1 as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize)
                / s;
            *v.offset(1 as ::core::ffi::c_int as isize) = (*h__
                .offset((h_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                - *sr1)
                * ((*h__
                    .offset((h_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    - *sr2)
                    / s)
                - *si1 * (*si2 / s)
                + *h__.offset(
                    (((h_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_long) as isize,
                ) * h21s
                + *h__.offset(
                    (h_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) * h31s;
            *v.offset(2 as ::core::ffi::c_int as isize) = h21s
                * (*h__
                    .offset((h_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    + *h__.offset(
                        (((h_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_long) as isize,
                    )
                    - *sr1
                    - *sr2)
                + *h__.offset(
                    (h_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long
                        + 2 as ::core::ffi::c_long) as isize,
                ) * h31s;
            *v.offset(3 as ::core::ffi::c_int as isize) = h31s
                * (*h__
                    .offset((h_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    + *h__.offset(
                        (h_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    )
                    - *sr1
                    - *sr2)
                + h21s
                    * *h__.offset(
                        (((h_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                            + 3 as ::core::ffi::c_long) as isize,
                    );
        }
    }
    return 0 as ::core::ffi::c_int;
}
