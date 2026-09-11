extern "C" {
    fn sqrt(_: doublereal) -> ::core::ffi::c_double;
}
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlaqp2_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut offset: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut jpvt: *mut integer,
    mut tau: *mut doublereal,
    mut vn1: *mut doublereal,
    mut vn2: *mut doublereal,
    mut work: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__3: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    let mut i__: integer = 0;
    let mut j: integer = 0;
    let mut mn: integer = 0;
    let mut pvt: integer = 0;
    let mut temp: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dnrm2"]
        fn dgelsd_closure_f2c_dnrm2_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> doublereal;
    }
    let mut temp2: doublereal = 0.;
    let mut tol3z: doublereal = 0.;
    let mut offpi: integer = 0;
    let mut itemp: integer = 0;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dswap"]
        fn dgelsd_closure_f2c_dswap_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dgelsd_closure_dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgeev_closure_dlarfg_"]
        fn dgeev_closure_dlarfg__0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_idamax"]
        fn dgelsd_closure_f2c_idamax_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> integer;
    }
    extern "C" {
        #[link_name = "slsqp_closure_dlarf1f_"]
        fn slsqp_closure_dlarf1f__0(
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
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    jpvt = jpvt.offset(-1);
    tau = tau.offset(-1);
    vn1 = vn1.offset(-1);
    vn2 = vn2.offset(-1);
    work = work.offset(-1);
    i__1 = *m - *offset;
    mn = (if i__1 <= *n {
        i__1 as ::core::ffi::c_long
    } else {
        *n
    }) as integer;
    tol3z = sqrt(dgelsd_closure_dlamch__0(
        b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    )) as doublereal;
    i__1 = mn;
    i__ = 1 as integer;
    while i__ <= i__1 {
        offpi = *offset + i__;
        i__2 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        pvt = i__ - 1 as integer
            + dgelsd_closure_f2c_idamax_0(
                &raw mut i__2,
                vn1.offset(i__ as isize) as *mut doublereal,
                &raw mut c__1,
            );
        if pvt != i__ {
            dgelsd_closure_f2c_dswap_0(
                m,
                a.offset(
                    (pvt as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                a.offset(
                    (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
            );
            itemp = *jpvt.offset(pvt as isize);
            *jpvt.offset(pvt as isize) = *jpvt.offset(i__ as isize);
            *jpvt.offset(i__ as isize) = itemp;
            *vn1.offset(pvt as isize) = *vn1.offset(i__ as isize);
            *vn2.offset(pvt as isize) = *vn2.offset(i__ as isize);
        }
        if offpi < *m {
            i__2 = (*m - offpi as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgeev_closure_dlarfg__0(
                &raw mut i__2,
                a.offset((offpi + i__ * a_dim1) as isize) as *mut doublereal,
                a.offset((offpi + 1 as integer + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                tau.offset(i__ as isize) as *mut doublereal,
            );
        } else {
            dgeev_closure_dlarfg__0(
                &raw mut c__1,
                a.offset((*m + i__ * a_dim1) as isize) as *mut doublereal,
                a.offset((*m + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                tau.offset(i__ as isize) as *mut doublereal,
            );
        }
        if i__ < *n {
            i__2 = (*m - offpi as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__3 = *n - i__;
            slsqp_closure_dlarf1f__0(
                b"Left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                &raw mut i__2,
                &raw mut i__3,
                a.offset((offpi + i__ * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                tau.offset(i__ as isize) as *mut doublereal,
                a.offset((offpi + (i__ + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                lda,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            );
        }
        i__2 = *n;
        j = (i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        while j <= i__2 {
            if *vn1.offset(j as isize) != 0.0f64 {
                d__1 = *a.offset((offpi + j * a_dim1) as isize);
                d__2 = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    d__1
                } else {
                    -d__1
                }) / *vn1.offset(j as isize);
                temp = 1.0f64 - d__2 * d__2;
                temp = (if temp >= 0.0f64 {
                    temp as ::core::ffi::c_double
                } else {
                    0.0f64
                }) as doublereal;
                d__1 = *vn1.offset(j as isize) / *vn2.offset(j as isize);
                temp2 = temp * (d__1 * d__1);
                if temp2 <= tol3z {
                    if offpi < *m {
                        i__3 = *m - offpi;
                        *vn1.offset(j as isize) = dgelsd_closure_f2c_dnrm2_0(
                            &raw mut i__3,
                            a.offset((offpi + 1 as integer + j * a_dim1) as isize)
                                as *mut doublereal,
                            &raw mut c__1,
                        );
                        *vn2.offset(j as isize) = *vn1.offset(j as isize);
                    } else {
                        *vn1.offset(j as isize) = 0.0f64 as doublereal;
                        *vn2.offset(j as isize) = 0.0f64 as doublereal;
                    }
                } else {
                    let ref mut fresh0 = *vn1.offset(j as isize);
                    *fresh0 *= sqrt(temp);
                }
            }
            j += 1;
        }
        i__ += 1;
    }
    return 0 as ::core::ffi::c_int;
}
