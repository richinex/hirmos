pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
static mut c_b4: doublereal = 1.0f64;
static mut c_b5: doublereal = 0.0f64;
static mut c__1: integer = 1 as integer;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlarf1f_(
    mut side: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut v: *mut doublereal,
    mut incv: *mut integer,
    mut tau: *mut doublereal,
    mut c__: *mut doublereal,
    mut ldc: *mut integer,
    mut work: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut d__1: doublereal = 0.;
    let mut i__: integer = 0;
    let mut applyleft: logical = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_dger"]
        fn dsyevd_closure_f2c_dger_0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
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
    let mut lastc: integer = 0;
    extern "C" {
        #[link_name = "dsyevd_closure_f2c_daxpy"]
        fn dsyevd_closure_f2c_daxpy_0(
            _: *mut integer,
            _: *mut doublereal,
            _: *mut doublereal,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> ::core::ffi::c_int;
    }
    let mut lastv: integer = 0;
    extern "C" {
        #[link_name = "dgeev_closure_iladlc_"]
        fn dgeev_closure_iladlc__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> integer;
    }
    extern "C" {
        #[link_name = "dgeev_closure_iladlr_"]
        fn dgeev_closure_iladlr__0(
            _: *mut integer,
            _: *mut integer,
            _: *mut doublereal,
            _: *mut integer,
        ) -> integer;
    }
    v = v.offset(-1);
    c_dim1 = *ldc;
    c_offset = 1 as integer + c_dim1;
    c__ = c__.offset(-(c_offset as isize));
    work = work.offset(-1);
    applyleft = dgelsd_closure_lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    lastv = 1 as integer;
    lastc = 0 as integer;
    if *tau != 0.0f64 {
        if applyleft != 0 {
            lastv = *m;
        } else {
            lastv = *n;
        }
        if *incv > 0 as ::core::ffi::c_long {
            i__ = ((lastv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) * *incv
                + 1 as ::core::ffi::c_long) as integer;
        } else {
            i__ = 1 as integer;
        }
        while lastv > 1 as ::core::ffi::c_long && *v.offset(i__ as isize) == 0.0f64 {
            lastv -= 1;
            i__ -= *incv as ::core::ffi::c_long;
        }
        if applyleft != 0 {
            lastc = dgeev_closure_iladlc__0(
                &raw mut lastv,
                n,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
            );
        } else {
            lastc = dgeev_closure_iladlr__0(
                m,
                &raw mut lastv,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
            );
        }
    }
    if lastc == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if applyleft != 0 {
        if lastv == 1 as ::core::ffi::c_long {
            d__1 = 1.0f64 - *tau;
            dgelsd_closure_f2c_dscal_0(
                &raw mut lastc,
                &raw mut d__1,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
            );
        } else {
            i__1 = (lastv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            dgelsd_closure_f2c_dgemv_0(
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut i__1,
                &raw mut lastc,
                &raw mut c_b4,
                c__.offset((c_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldc,
                v.offset((*incv + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
                incv,
                &raw mut c_b5,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            );
            dsyevd_closure_f2c_daxpy_0(
                &raw mut lastc,
                &raw mut c_b4,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            );
            d__1 = -*tau;
            dsyevd_closure_f2c_daxpy_0(
                &raw mut lastc,
                &raw mut d__1,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
            );
            i__1 = (lastv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
            d__1 = -*tau;
            dsyevd_closure_f2c_dger_0(
                &raw mut i__1,
                &raw mut lastc,
                &raw mut d__1,
                v.offset((*incv + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
                incv,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                c__.offset((c_dim1 as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize)
                    as *mut doublereal,
                ldc,
            );
        }
    } else if lastv == 1 as ::core::ffi::c_long {
        d__1 = 1.0f64 - *tau;
        dgelsd_closure_f2c_dscal_0(
            &raw mut lastc,
            &raw mut d__1,
            c__.offset(c_offset as isize) as *mut doublereal,
            &raw mut c__1,
        );
    } else {
        i__1 = (lastv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        dgelsd_closure_f2c_dgemv_0(
            b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut lastc,
            &raw mut i__1,
            &raw mut c_b4,
            c__.offset(
                (((c_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            ldc,
            v.offset((*incv + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            incv,
            &raw mut c_b5,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        dsyevd_closure_f2c_daxpy_0(
            &raw mut lastc,
            &raw mut c_b4,
            c__.offset(c_offset as isize) as *mut doublereal,
            &raw mut c__1,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        d__1 = -*tau;
        dsyevd_closure_f2c_daxpy_0(
            &raw mut lastc,
            &raw mut d__1,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            c__.offset(c_offset as isize) as *mut doublereal,
            &raw mut c__1,
        );
        i__1 = (lastv as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        d__1 = -*tau;
        dsyevd_closure_f2c_dger_0(
            &raw mut lastc,
            &raw mut i__1,
            &raw mut d__1,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            v.offset((*incv + 1 as ::core::ffi::c_long) as isize) as *mut doublereal,
            incv,
            c__.offset(
                (((c_dim1 as ::core::ffi::c_long) << 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            ldc,
        );
    }
    return 0 as ::core::ffi::c_int;
}
