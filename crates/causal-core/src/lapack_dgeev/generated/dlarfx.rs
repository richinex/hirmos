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
#[no_mangle]
pub unsafe extern "C" fn dgeev_closure_dlarfx_(
    mut side: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut v: *mut doublereal,
    mut tau: *mut doublereal,
    mut c__: *mut doublereal,
    mut ldc: *mut integer,
    mut work: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut j: integer = 0;
    let mut t1: doublereal = 0.;
    let mut t2: doublereal = 0.;
    let mut t3: doublereal = 0.;
    let mut t4: doublereal = 0.;
    let mut t5: doublereal = 0.;
    let mut t6: doublereal = 0.;
    let mut t7: doublereal = 0.;
    let mut t8: doublereal = 0.;
    let mut t9: doublereal = 0.;
    let mut v1: doublereal = 0.;
    let mut v2: doublereal = 0.;
    let mut v3: doublereal = 0.;
    let mut v4: doublereal = 0.;
    let mut v5: doublereal = 0.;
    let mut v6: doublereal = 0.;
    let mut v7: doublereal = 0.;
    let mut v8: doublereal = 0.;
    let mut v9: doublereal = 0.;
    let mut t10: doublereal = 0.;
    let mut v10: doublereal = 0.;
    let mut sum: doublereal = 0.;
    extern "C" {
        #[link_name = "dgeev_closure_dlarf_"]
        fn dlarf__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_lsame_"]
        fn lsame__0(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> logical;
    }
    v = v.offset(-1);
    c_dim1 = *ldc;
    c_offset = 1 as integer + c_dim1;
    c__ = c__.offset(-(c_offset as isize));
    work = work.offset(-1);
    if *tau == 0.0f64 {
        return 0 as ::core::ffi::c_int;
    }
    if lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        match *m {
            1 => {
                t1 = 1.0f64
                    - *tau
                        * *v.offset(1 as ::core::ffi::c_int as isize)
                        * *v.offset(1 as ::core::ffi::c_int as isize);
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    ) = t1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        );
                    j += 1;
                }
            }
            2 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        )
                        + v2 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        );
                    let ref mut fresh0 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    *fresh0 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh1 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                    *fresh1 -= (sum * t2) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            3 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        )
                        + v2 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        );
                    let ref mut fresh2 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    *fresh2 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh3 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                    *fresh3 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh4 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    );
                    *fresh4 -= (sum * t3) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            4 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        )
                        + v2 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        )
                        + v4 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 4 as ::core::ffi::c_long) as isize,
                        );
                    let ref mut fresh5 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    *fresh5 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh6 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                    *fresh6 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh7 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    );
                    *fresh7 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh8 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 4 as ::core::ffi::c_long) as isize,
                    );
                    *fresh8 -= (sum * t4) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            5 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        )
                        + v2 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        )
                        + v4 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 4 as ::core::ffi::c_long) as isize,
                        )
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 5 as ::core::ffi::c_long) as isize,
                        );
                    let ref mut fresh9 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    *fresh9 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh10 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                    *fresh10 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh11 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    );
                    *fresh11 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh12 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 4 as ::core::ffi::c_long) as isize,
                    );
                    *fresh12 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh13 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 5 as ::core::ffi::c_long) as isize,
                    );
                    *fresh13 -= (sum * t5) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            6 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        )
                        + v2 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        )
                        + v4 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 4 as ::core::ffi::c_long) as isize,
                        )
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 5 as ::core::ffi::c_long) as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 6 as ::core::ffi::c_long) as isize,
                        );
                    let ref mut fresh14 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    *fresh14 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh15 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                    *fresh15 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh16 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    );
                    *fresh16 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh17 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 4 as ::core::ffi::c_long) as isize,
                    );
                    *fresh17 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh18 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 5 as ::core::ffi::c_long) as isize,
                    );
                    *fresh18 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh19 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 6 as ::core::ffi::c_long) as isize,
                    );
                    *fresh19 -= (sum * t6) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            7 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                v7 = *v.offset(7 as ::core::ffi::c_int as isize);
                t7 = *tau * v7;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        )
                        + v2 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        )
                        + v4 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 4 as ::core::ffi::c_long) as isize,
                        )
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 5 as ::core::ffi::c_long) as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 6 as ::core::ffi::c_long) as isize,
                        )
                        + v7 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 7 as ::core::ffi::c_long) as isize,
                        );
                    let ref mut fresh20 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    *fresh20 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh21 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                    *fresh21 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh22 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    );
                    *fresh22 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh23 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 4 as ::core::ffi::c_long) as isize,
                    );
                    *fresh23 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh24 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 5 as ::core::ffi::c_long) as isize,
                    );
                    *fresh24 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh25 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 6 as ::core::ffi::c_long) as isize,
                    );
                    *fresh25 -= (sum * t6) as ::core::ffi::c_double;
                    let ref mut fresh26 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 7 as ::core::ffi::c_long) as isize,
                    );
                    *fresh26 -= (sum * t7) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            8 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                v7 = *v.offset(7 as ::core::ffi::c_int as isize);
                t7 = *tau * v7;
                v8 = *v.offset(8 as ::core::ffi::c_int as isize);
                t8 = *tau * v8;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        )
                        + v2 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        )
                        + v4 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 4 as ::core::ffi::c_long) as isize,
                        )
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 5 as ::core::ffi::c_long) as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 6 as ::core::ffi::c_long) as isize,
                        )
                        + v7 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 7 as ::core::ffi::c_long) as isize,
                        )
                        + v8 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 8 as ::core::ffi::c_long) as isize,
                        );
                    let ref mut fresh27 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    *fresh27 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh28 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                    *fresh28 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh29 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    );
                    *fresh29 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh30 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 4 as ::core::ffi::c_long) as isize,
                    );
                    *fresh30 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh31 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 5 as ::core::ffi::c_long) as isize,
                    );
                    *fresh31 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh32 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 6 as ::core::ffi::c_long) as isize,
                    );
                    *fresh32 -= (sum * t6) as ::core::ffi::c_double;
                    let ref mut fresh33 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 7 as ::core::ffi::c_long) as isize,
                    );
                    *fresh33 -= (sum * t7) as ::core::ffi::c_double;
                    let ref mut fresh34 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 8 as ::core::ffi::c_long) as isize,
                    );
                    *fresh34 -= (sum * t8) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            9 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                v7 = *v.offset(7 as ::core::ffi::c_int as isize);
                t7 = *tau * v7;
                v8 = *v.offset(8 as ::core::ffi::c_int as isize);
                t8 = *tau * v8;
                v9 = *v.offset(9 as ::core::ffi::c_int as isize);
                t9 = *tau * v9;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        )
                        + v2 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        )
                        + v4 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 4 as ::core::ffi::c_long) as isize,
                        )
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 5 as ::core::ffi::c_long) as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 6 as ::core::ffi::c_long) as isize,
                        )
                        + v7 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 7 as ::core::ffi::c_long) as isize,
                        )
                        + v8 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 8 as ::core::ffi::c_long) as isize,
                        )
                        + v9 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 9 as ::core::ffi::c_long) as isize,
                        );
                    let ref mut fresh35 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    *fresh35 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh36 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                    *fresh36 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh37 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    );
                    *fresh37 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh38 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 4 as ::core::ffi::c_long) as isize,
                    );
                    *fresh38 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh39 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 5 as ::core::ffi::c_long) as isize,
                    );
                    *fresh39 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh40 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 6 as ::core::ffi::c_long) as isize,
                    );
                    *fresh40 -= (sum * t6) as ::core::ffi::c_double;
                    let ref mut fresh41 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 7 as ::core::ffi::c_long) as isize,
                    );
                    *fresh41 -= (sum * t7) as ::core::ffi::c_double;
                    let ref mut fresh42 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 8 as ::core::ffi::c_long) as isize,
                    );
                    *fresh42 -= (sum * t8) as ::core::ffi::c_double;
                    let ref mut fresh43 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 9 as ::core::ffi::c_long) as isize,
                    );
                    *fresh43 -= (sum * t9) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            10 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                v7 = *v.offset(7 as ::core::ffi::c_int as isize);
                t7 = *tau * v7;
                v8 = *v.offset(8 as ::core::ffi::c_int as isize);
                t8 = *tau * v8;
                v9 = *v.offset(9 as ::core::ffi::c_int as isize);
                t9 = *tau * v9;
                v10 = *v.offset(10 as ::core::ffi::c_int as isize);
                t10 = *tau * v10;
                i__1 = *n;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1
                        * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as isize,
                        )
                        + v2 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 2 as ::core::ffi::c_long) as isize,
                        )
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 3 as ::core::ffi::c_long) as isize,
                        )
                        + v4 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 4 as ::core::ffi::c_long) as isize,
                        )
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 5 as ::core::ffi::c_long) as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 6 as ::core::ffi::c_long) as isize,
                        )
                        + v7 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 7 as ::core::ffi::c_long) as isize,
                        )
                        + v8 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 8 as ::core::ffi::c_long) as isize,
                        )
                        + v9 * *c__.offset(
                            (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                + 9 as ::core::ffi::c_long) as isize,
                        )
                        + v10
                            * *c__.offset(
                                (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                                    + 10 as ::core::ffi::c_long)
                                    as isize,
                            );
                    let ref mut fresh44 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as isize,
                    );
                    *fresh44 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh45 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 2 as ::core::ffi::c_long) as isize,
                    );
                    *fresh45 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh46 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 3 as ::core::ffi::c_long) as isize,
                    );
                    *fresh46 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh47 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 4 as ::core::ffi::c_long) as isize,
                    );
                    *fresh47 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh48 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 5 as ::core::ffi::c_long) as isize,
                    );
                    *fresh48 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh49 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 6 as ::core::ffi::c_long) as isize,
                    );
                    *fresh49 -= (sum * t6) as ::core::ffi::c_double;
                    let ref mut fresh50 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 7 as ::core::ffi::c_long) as isize,
                    );
                    *fresh50 -= (sum * t7) as ::core::ffi::c_double;
                    let ref mut fresh51 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 8 as ::core::ffi::c_long) as isize,
                    );
                    *fresh51 -= (sum * t8) as ::core::ffi::c_double;
                    let ref mut fresh52 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 9 as ::core::ffi::c_long) as isize,
                    );
                    *fresh52 -= (sum * t9) as ::core::ffi::c_double;
                    let ref mut fresh53 = *c__.offset(
                        (j as ::core::ffi::c_long * c_dim1 as ::core::ffi::c_long
                            + 10 as ::core::ffi::c_long) as isize,
                    );
                    *fresh53 -= (sum * t10) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            _ => {
                dlarf__0(
                    side,
                    m,
                    n,
                    v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c__1,
                    tau,
                    c__.offset(c_offset as isize) as *mut doublereal,
                    ldc,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
            }
        }
    } else {
        match *n {
            1 => {
                t1 = 1.0f64
                    - *tau
                        * *v.offset(1 as ::core::ffi::c_int as isize)
                        * *v.offset(1 as ::core::ffi::c_int as isize);
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    *c__.offset((j + c_dim1) as isize) = t1 * *c__.offset((j + c_dim1) as isize);
                    j += 1;
                }
            }
            2 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1 * *c__.offset((j + c_dim1) as isize)
                        + v2 * *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    let ref mut fresh54 = *c__.offset((j + c_dim1) as isize);
                    *fresh54 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh55 =
                        *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    *fresh55 -= (sum * t2) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            3 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1 * *c__.offset((j + c_dim1) as isize)
                        + v2 * *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                                as isize,
                        );
                    let ref mut fresh56 = *c__.offset((j + c_dim1) as isize);
                    *fresh56 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh57 =
                        *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    *fresh57 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh58 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh58 -= (sum * t3) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            4 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1 * *c__.offset((j + c_dim1) as isize)
                        + v2 * *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v4 * *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize);
                    let ref mut fresh59 = *c__.offset((j + c_dim1) as isize);
                    *fresh59 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh60 =
                        *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    *fresh60 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh61 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh61 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh62 =
                        *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize);
                    *fresh62 -= (sum * t4) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            5 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1 * *c__.offset((j + c_dim1) as isize)
                        + v2 * *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v4 * *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize)
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                                as isize,
                        );
                    let ref mut fresh63 = *c__.offset((j + c_dim1) as isize);
                    *fresh63 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh64 =
                        *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    *fresh64 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh65 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh65 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh66 =
                        *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize);
                    *fresh66 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh67 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh67 -= (sum * t5) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            6 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1 * *c__.offset((j + c_dim1) as isize)
                        + v2 * *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v4 * *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize)
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                                as isize,
                        );
                    let ref mut fresh68 = *c__.offset((j + c_dim1) as isize);
                    *fresh68 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh69 =
                        *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    *fresh69 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh70 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh70 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh71 =
                        *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize);
                    *fresh71 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh72 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh72 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh73 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh73 -= (sum * t6) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            7 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                v7 = *v.offset(7 as ::core::ffi::c_int as isize);
                t7 = *tau * v7;
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1 * *c__.offset((j + c_dim1) as isize)
                        + v2 * *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v4 * *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize)
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v7 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 7 as ::core::ffi::c_long)
                                as isize,
                        );
                    let ref mut fresh74 = *c__.offset((j + c_dim1) as isize);
                    *fresh74 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh75 =
                        *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    *fresh75 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh76 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh76 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh77 =
                        *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize);
                    *fresh77 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh78 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh78 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh79 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh79 -= (sum * t6) as ::core::ffi::c_double;
                    let ref mut fresh80 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 7 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh80 -= (sum * t7) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            8 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                v7 = *v.offset(7 as ::core::ffi::c_int as isize);
                t7 = *tau * v7;
                v8 = *v.offset(8 as ::core::ffi::c_int as isize);
                t8 = *tau * v8;
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1 * *c__.offset((j + c_dim1) as isize)
                        + v2 * *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v4 * *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize)
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v7 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 7 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v8 * *c__.offset((j + (c_dim1 << 3 as ::core::ffi::c_int)) as isize);
                    let ref mut fresh81 = *c__.offset((j + c_dim1) as isize);
                    *fresh81 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh82 =
                        *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    *fresh82 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh83 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh83 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh84 =
                        *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize);
                    *fresh84 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh85 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh85 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh86 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh86 -= (sum * t6) as ::core::ffi::c_double;
                    let ref mut fresh87 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 7 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh87 -= (sum * t7) as ::core::ffi::c_double;
                    let ref mut fresh88 =
                        *c__.offset((j + (c_dim1 << 3 as ::core::ffi::c_int)) as isize);
                    *fresh88 -= (sum * t8) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            9 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                v7 = *v.offset(7 as ::core::ffi::c_int as isize);
                t7 = *tau * v7;
                v8 = *v.offset(8 as ::core::ffi::c_int as isize);
                t8 = *tau * v8;
                v9 = *v.offset(9 as ::core::ffi::c_int as isize);
                t9 = *tau * v9;
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1 * *c__.offset((j + c_dim1) as isize)
                        + v2 * *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v4 * *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize)
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v7 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 7 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v8 * *c__.offset((j + (c_dim1 << 3 as ::core::ffi::c_int)) as isize)
                        + v9 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 9 as ::core::ffi::c_long)
                                as isize,
                        );
                    let ref mut fresh89 = *c__.offset((j + c_dim1) as isize);
                    *fresh89 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh90 =
                        *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    *fresh90 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh91 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh91 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh92 =
                        *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize);
                    *fresh92 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh93 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh93 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh94 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh94 -= (sum * t6) as ::core::ffi::c_double;
                    let ref mut fresh95 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 7 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh95 -= (sum * t7) as ::core::ffi::c_double;
                    let ref mut fresh96 =
                        *c__.offset((j + (c_dim1 << 3 as ::core::ffi::c_int)) as isize);
                    *fresh96 -= (sum * t8) as ::core::ffi::c_double;
                    let ref mut fresh97 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 9 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh97 -= (sum * t9) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            10 => {
                v1 = *v.offset(1 as ::core::ffi::c_int as isize);
                t1 = *tau * v1;
                v2 = *v.offset(2 as ::core::ffi::c_int as isize);
                t2 = *tau * v2;
                v3 = *v.offset(3 as ::core::ffi::c_int as isize);
                t3 = *tau * v3;
                v4 = *v.offset(4 as ::core::ffi::c_int as isize);
                t4 = *tau * v4;
                v5 = *v.offset(5 as ::core::ffi::c_int as isize);
                t5 = *tau * v5;
                v6 = *v.offset(6 as ::core::ffi::c_int as isize);
                t6 = *tau * v6;
                v7 = *v.offset(7 as ::core::ffi::c_int as isize);
                t7 = *tau * v7;
                v8 = *v.offset(8 as ::core::ffi::c_int as isize);
                t8 = *tau * v8;
                v9 = *v.offset(9 as ::core::ffi::c_int as isize);
                t9 = *tau * v9;
                v10 = *v.offset(10 as ::core::ffi::c_int as isize);
                t10 = *tau * v10;
                i__1 = *m;
                j = 1 as integer;
                while j <= i__1 {
                    sum = v1 * *c__.offset((j + c_dim1) as isize)
                        + v2 * *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize)
                        + v3 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v4 * *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize)
                        + v5 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v6 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v7 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 7 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v8 * *c__.offset((j + (c_dim1 << 3 as ::core::ffi::c_int)) as isize)
                        + v9 * *c__.offset(
                            (j as ::core::ffi::c_long
                                + c_dim1 as ::core::ffi::c_long * 9 as ::core::ffi::c_long)
                                as isize,
                        )
                        + v10
                            * *c__.offset(
                                (j as ::core::ffi::c_long
                                    + c_dim1 as ::core::ffi::c_long * 10 as ::core::ffi::c_long)
                                    as isize,
                            );
                    let ref mut fresh98 = *c__.offset((j + c_dim1) as isize);
                    *fresh98 -= (sum * t1) as ::core::ffi::c_double;
                    let ref mut fresh99 =
                        *c__.offset((j + (c_dim1 << 1 as ::core::ffi::c_int)) as isize);
                    *fresh99 -= (sum * t2) as ::core::ffi::c_double;
                    let ref mut fresh100 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 3 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh100 -= (sum * t3) as ::core::ffi::c_double;
                    let ref mut fresh101 =
                        *c__.offset((j + (c_dim1 << 2 as ::core::ffi::c_int)) as isize);
                    *fresh101 -= (sum * t4) as ::core::ffi::c_double;
                    let ref mut fresh102 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 5 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh102 -= (sum * t5) as ::core::ffi::c_double;
                    let ref mut fresh103 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 6 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh103 -= (sum * t6) as ::core::ffi::c_double;
                    let ref mut fresh104 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 7 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh104 -= (sum * t7) as ::core::ffi::c_double;
                    let ref mut fresh105 =
                        *c__.offset((j + (c_dim1 << 3 as ::core::ffi::c_int)) as isize);
                    *fresh105 -= (sum * t8) as ::core::ffi::c_double;
                    let ref mut fresh106 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 9 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh106 -= (sum * t9) as ::core::ffi::c_double;
                    let ref mut fresh107 = *c__.offset(
                        (j as ::core::ffi::c_long
                            + c_dim1 as ::core::ffi::c_long * 10 as ::core::ffi::c_long)
                            as isize,
                    );
                    *fresh107 -= (sum * t10) as ::core::ffi::c_double;
                    j += 1;
                }
            }
            _ => {
                dlarf__0(
                    side,
                    m,
                    n,
                    v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    &raw mut c__1,
                    tau,
                    c__.offset(c_offset as isize) as *mut doublereal,
                    ldc,
                    work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                );
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
