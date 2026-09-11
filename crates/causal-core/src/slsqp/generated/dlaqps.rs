extern "C" {
    fn sqrt(_: doublereal) -> ::core::ffi::c_double;
}
pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
static mut c__1: integer = 1 as integer;
static mut c_b8: doublereal = -1.0f64;
static mut c_b9: doublereal = 1.0f64;
static mut c_b16: doublereal = 0.0f64;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlaqps_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut offset: *mut integer,
    mut nb: *mut integer,
    mut kb: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut jpvt: *mut integer,
    mut tau: *mut doublereal,
    mut vn1: *mut doublereal,
    mut vn2: *mut doublereal,
    mut auxv: *mut doublereal,
    mut f: *mut doublereal,
    mut ldf: *mut integer,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut f_dim1: integer = 0;
    let mut f_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut d__2: doublereal = 0.;
    extern "C" {
        #[link_name = "slsqp_closure_i_dnnt"]
        fn i_dnnt_0(_: *mut doublereal) -> integer;
    }
    let mut j: integer = 0;
    let mut k: integer = 0;
    let mut rk: integer = 0;
    let mut akk: doublereal = 0.;
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
    let mut lsticc: integer = 0;
    let mut lastrk: integer = 0;
    a_dim1 = *lda;
    a_offset = 1 as integer + a_dim1;
    a = a.offset(-(a_offset as isize));
    jpvt = jpvt.offset(-1);
    tau = tau.offset(-1);
    vn1 = vn1.offset(-1);
    vn2 = vn2.offset(-1);
    auxv = auxv.offset(-1);
    f_dim1 = *ldf;
    f_offset = 1 as integer + f_dim1;
    f = f.offset(-(f_offset as isize));
    i__1 = *m;
    i__2 = *n + *offset;
    lastrk = (if i__1 <= i__2 {
        i__1 as ::core::ffi::c_long
    } else {
        i__2 as ::core::ffi::c_long
    }) as integer;
    lsticc = 0 as integer;
    k = 0 as integer;
    tol3z = sqrt(dgelsd_closure_dlamch__0(
        b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    )) as doublereal;
    while k < *nb && lsticc == 0 as ::core::ffi::c_long {
        k += 1;
        rk = *offset + k;
        i__1 = (*n - k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        pvt = k - 1 as integer
            + dgelsd_closure_f2c_idamax_0(
                &raw mut i__1,
                vn1.offset(k as isize) as *mut doublereal,
                &raw mut c__1,
            );
        if pvt != k {
            dgelsd_closure_f2c_dswap_0(
                m,
                a.offset(
                    (pvt as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
                a.offset(
                    (k as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
            );
            i__1 = (k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            dgelsd_closure_f2c_dswap_0(
                &raw mut i__1,
                f.offset((pvt + f_dim1) as isize) as *mut doublereal,
                ldf,
                f.offset((k + f_dim1) as isize) as *mut doublereal,
                ldf,
            );
            itemp = *jpvt.offset(pvt as isize);
            *jpvt.offset(pvt as isize) = *jpvt.offset(k as isize);
            *jpvt.offset(k as isize) = itemp;
            *vn1.offset(pvt as isize) = *vn1.offset(k as isize);
            *vn2.offset(pvt as isize) = *vn2.offset(k as isize);
        }
        if k > 1 as ::core::ffi::c_long {
            i__1 = (*m - rk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__2 = (k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            dgelsd_closure_f2c_dgemv_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__1,
                &raw mut i__2,
                &raw mut c_b8,
                a.offset((rk + a_dim1) as isize) as *mut doublereal,
                lda,
                f.offset((k + f_dim1) as isize) as *mut doublereal,
                ldf,
                &raw mut c_b9,
                a.offset((rk + k * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
        }
        if rk < *m {
            i__1 = (*m - rk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            dgeev_closure_dlarfg__0(
                &raw mut i__1,
                a.offset((rk + k * a_dim1) as isize) as *mut doublereal,
                a.offset((rk + 1 as integer + k * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                tau.offset(k as isize) as *mut doublereal,
            );
        } else {
            dgeev_closure_dlarfg__0(
                &raw mut c__1,
                a.offset((rk + k * a_dim1) as isize) as *mut doublereal,
                a.offset((rk + k * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                tau.offset(k as isize) as *mut doublereal,
            );
        }
        akk = *a.offset((rk + k * a_dim1) as isize);
        *a.offset((rk + k * a_dim1) as isize) = 1.0f64 as doublereal;
        if k < *n {
            i__1 = (*m - rk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__2 = *n - k;
            dgelsd_closure_f2c_dgemv_0(
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__1,
                &raw mut i__2,
                tau.offset(k as isize) as *mut doublereal,
                a.offset((rk + (k + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                lda,
                a.offset((rk + k * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut c_b16,
                f.offset((k + 1 as integer + k * f_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
            );
        }
        i__1 = k;
        j = 1 as integer;
        while j <= i__1 {
            *f.offset((j + k * f_dim1) as isize) = 0.0f64 as doublereal;
            j += 1;
        }
        if k > 1 as ::core::ffi::c_long {
            i__1 = (*m - rk as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            i__2 = (k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            d__1 = -*tau.offset(k as isize);
            dgelsd_closure_f2c_dgemv_0(
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__1,
                &raw mut i__2,
                &raw mut d__1,
                a.offset((rk + a_dim1) as isize) as *mut doublereal,
                lda,
                a.offset((rk + k * a_dim1) as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut c_b16,
                auxv.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            );
            i__1 = (k as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            dgelsd_closure_f2c_dgemv_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                n,
                &raw mut i__1,
                &raw mut c_b9,
                f.offset((f_dim1 as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldf,
                auxv.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                &raw mut c_b9,
                f.offset(
                    (k as ::core::ffi::c_long * f_dim1 as ::core::ffi::c_long
                        + 1 as ::core::ffi::c_long) as isize,
                ) as *mut doublereal,
                &raw mut c__1,
            );
        }
        if k < *n {
            i__1 = *n - k;
            dgelsd_closure_f2c_dgemv_0(
                b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__1,
                &raw mut k,
                &raw mut c_b8,
                f.offset((k + 1 as integer + f_dim1) as isize) as *mut doublereal,
                ldf,
                a.offset((rk + a_dim1) as isize) as *mut doublereal,
                lda,
                &raw mut c_b9,
                a.offset((rk + (k + 1 as integer) * a_dim1) as isize) as *mut doublereal,
                lda,
            );
        }
        if rk < lastrk {
            i__1 = *n;
            j = (k as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
            while j <= i__1 {
                if *vn1.offset(j as isize) != 0.0f64 {
                    d__1 = *a.offset((rk + j * a_dim1) as isize);
                    temp = (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        d__1
                    } else {
                        -d__1
                    }) / *vn1.offset(j as isize);
                    d__1 = 0.0f64 as doublereal;
                    d__2 = (temp + 1.0f64) * (1.0f64 - temp);
                    temp = (if d__1 >= d__2 {
                        d__1 as ::core::ffi::c_double
                    } else {
                        d__2 as ::core::ffi::c_double
                    }) as doublereal;
                    d__1 = *vn1.offset(j as isize) / *vn2.offset(j as isize);
                    temp2 = temp * (d__1 * d__1);
                    if temp2 <= tol3z {
                        *vn2.offset(j as isize) = lsticc as doublereal;
                        lsticc = j;
                    } else {
                        let ref mut fresh0 = *vn1.offset(j as isize);
                        *fresh0 *= sqrt(temp);
                    }
                }
                j += 1;
            }
        }
        *a.offset((rk + k * a_dim1) as isize) = akk;
    }
    *kb = k;
    rk = *offset + *kb;
    i__1 = *n;
    i__2 = *m - *offset;
    if *kb
        < (if i__1 <= i__2 {
            i__1 as ::core::ffi::c_long
        } else {
            i__2 as ::core::ffi::c_long
        })
    {
        i__1 = *m - rk;
        i__2 = *n - *kb;
        dgelsd_closure_f2c_dgemm_0(
            b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"Transpose\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
            &raw mut i__2,
            kb,
            &raw mut c_b8,
            a.offset((rk + 1 as integer + a_dim1) as isize) as *mut doublereal,
            lda,
            f.offset((*kb + 1 as integer + f_dim1) as isize) as *mut doublereal,
            ldf,
            &raw mut c_b9,
            a.offset((rk + 1 as integer + (*kb + 1 as integer) * a_dim1) as isize)
                as *mut doublereal,
            lda,
        );
    }
    while lsticc > 0 as ::core::ffi::c_long {
        itemp = i_dnnt_0(vn2.offset(lsticc as isize) as *mut doublereal);
        i__1 = *m - rk;
        *vn1.offset(lsticc as isize) = dgelsd_closure_f2c_dnrm2_0(
            &raw mut i__1,
            a.offset((rk + 1 as integer + lsticc * a_dim1) as isize) as *mut doublereal,
            &raw mut c__1,
        );
        *vn2.offset(lsticc as isize) = *vn1.offset(lsticc as isize);
        lsticc = itemp;
    }
    return 0 as ::core::ffi::c_int;
}
