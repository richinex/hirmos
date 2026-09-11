pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlarfgp_(
    mut n: *mut integer,
    mut alpha: *mut doublereal,
    mut x: *mut doublereal,
    mut incx: *mut integer,
    mut tau: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_d_sign"]
        fn dgelsd_closure_d_sign_0(_: *mut doublereal, _: *mut doublereal)
            -> ::core::ffi::c_double;
    }
    let mut j: integer = 0;
    let mut savealpha: doublereal = 0.;
    let mut eps: doublereal = 0.;
    let mut knt: integer = 0;
    let mut beta: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dnrm2"]
        fn dgelsd_closure_f2c_dnrm2_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_f2c_dscal"]
        fn dgelsd_closure_f2c_dscal_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut xnorm: doublereal = 0.;
    extern "C" {
        #[link_name = "dgelsd_closure_dlapy2_"]
        fn dgelsd_closure_dlapy2__0(_: *mut doublereal, _: *mut doublereal) -> doublereal;
    }
    extern "C" {
        #[link_name = "dgelsd_closure_dlamch_"]
        fn dgelsd_closure_dlamch__0(_: *mut ::core::ffi::c_char) -> doublereal;
    }
    let mut bignum: doublereal = 0.;
    let mut smlnum: doublereal = 0.;
    x = x.offset(-1);
    if *n <= 0 as ::core::ffi::c_long {
        *tau = 0.0f64 as doublereal;
        return 0 as ::core::ffi::c_int;
    }
    eps = dgelsd_closure_dlamch__0(
        b"Precision\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
    xnorm = dgelsd_closure_f2c_dnrm2_0(
        &raw mut i__1,
        x.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        incx,
    );
    if xnorm
        <= eps as ::core::ffi::c_double
            * (if *alpha >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                *alpha
            } else {
                -*alpha
            })
    {
        if *alpha >= 0.0f64 {
            *tau = 0.0f64 as doublereal;
        } else {
            *tau = 2.0f64 as doublereal;
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            j = 1 as integer;
            while j <= i__1 {
                *x.offset(
                    ((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) * *incx
                        + 1 as ::core::ffi::c_long) as isize,
                ) = 0.0f64 as doublereal;
                j += 1;
            }
            *alpha = -*alpha;
        }
    } else {
        d__1 = dgelsd_closure_dlapy2__0(alpha, &raw mut xnorm);
        beta = dgelsd_closure_d_sign_0(&raw mut d__1, alpha) as doublereal;
        smlnum = dgelsd_closure_dlamch__0(
            b"S\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) / dgelsd_closure_dlamch__0(
            b"E\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        knt = 0 as integer;
        if (if beta >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            beta as ::core::ffi::c_double
        } else {
            -(beta as ::core::ffi::c_double)
        }) < smlnum
        {
            bignum = 1.0f64 / smlnum;
            loop {
                knt += 1;
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                dgelsd_closure_f2c_dscal_0(
                    &raw mut i__1,
                    &raw mut bignum,
                    x.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                    incx,
                );
                beta *= bignum as ::core::ffi::c_double;
                *alpha *= bignum as ::core::ffi::c_double;
                if !((if beta >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    beta as ::core::ffi::c_double
                } else {
                    -(beta as ::core::ffi::c_double)
                }) < smlnum
                    && knt < 20 as ::core::ffi::c_long)
                {
                    break;
                }
            }
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            xnorm = dgelsd_closure_f2c_dnrm2_0(
                &raw mut i__1,
                x.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                incx,
            );
            d__1 = dgelsd_closure_dlapy2__0(alpha, &raw mut xnorm);
            beta = dgelsd_closure_d_sign_0(&raw mut d__1, alpha) as doublereal;
        }
        savealpha = *alpha;
        *alpha += beta as ::core::ffi::c_double;
        if beta < 0.0f64 {
            beta = -beta;
            *tau = -*alpha / beta;
        } else {
            *alpha = xnorm * (xnorm / *alpha);
            *tau = *alpha / beta;
            *alpha = -*alpha;
        }
        if (if *tau >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            *tau
        } else {
            -*tau
        }) <= smlnum
        {
            if savealpha >= 0.0f64 {
                *tau = 0.0f64 as doublereal;
            } else {
                *tau = 2.0f64 as doublereal;
                i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
                j = 1 as integer;
                while j <= i__1 {
                    *x.offset(
                        ((j as ::core::ffi::c_long - 1 as ::core::ffi::c_long) * *incx
                            + 1 as ::core::ffi::c_long) as isize,
                    ) = 0.0f64 as doublereal;
                    j += 1;
                }
                beta = -savealpha;
            }
        } else {
            i__1 = (*n - 1 as ::core::ffi::c_long) as integer;
            d__1 = 1.0f64 / *alpha;
            dgelsd_closure_f2c_dscal_0(
                &raw mut i__1,
                &raw mut d__1,
                x.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                incx,
            );
        }
        i__1 = knt;
        j = 1 as integer;
        while j <= i__1 {
            beta *= smlnum as ::core::ffi::c_double;
            j += 1;
        }
        *alpha = beta;
    }
    return 0 as ::core::ffi::c_int;
}
