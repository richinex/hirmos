pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
pub type logical = ::core::ffi::c_long;
static mut c__1: integer = 1 as integer;
static mut c_b5: doublereal = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlarz_(
    mut side: *mut ::core::ffi::c_char,
    mut m: *mut integer,
    mut n: *mut integer,
    mut l: *mut integer,
    mut v: *mut doublereal,
    mut incv: *mut integer,
    mut tau: *mut doublereal,
    mut c__: *mut doublereal,
    mut ldc: *mut integer,
    mut work: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut c_dim1: integer = 0;
    let mut c_offset: integer = 0;
    let mut d__1: doublereal = 0.;
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
    v = v.offset(-1);
    c_dim1 = *ldc;
    c_offset = 1 as integer + c_dim1;
    c__ = c__.offset(-(c_offset as isize));
    work = work.offset(-1);
    if dgelsd_closure_lsame__0(
        side,
        b"L\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        if *tau != 0.0f64 {
            dgelsd_closure_f2c_dcopy_0(
                n,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            );
            dgelsd_closure_f2c_dgemv_0(
                b"Transpose\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                l,
                n,
                &raw mut c_b5,
                c__.offset((*m - *l + 1 as integer + c_dim1) as isize) as *mut doublereal,
                ldc,
                v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                incv,
                &raw mut c_b5,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
            );
            d__1 = -*tau;
            dsyevd_closure_f2c_daxpy_0(
                n,
                &raw mut d__1,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                c__.offset(c_offset as isize) as *mut doublereal,
                ldc,
            );
            d__1 = -*tau;
            dsyevd_closure_f2c_dger_0(
                l,
                n,
                &raw mut d__1,
                v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                incv,
                work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
                &raw mut c__1,
                c__.offset((*m - *l + 1 as integer + c_dim1) as isize) as *mut doublereal,
                ldc,
            );
        }
    } else if *tau != 0.0f64 {
        dgelsd_closure_f2c_dcopy_0(
            m,
            c__.offset(c_offset as isize) as *mut doublereal,
            &raw mut c__1,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        dgelsd_closure_f2c_dgemv_0(
            b"No transpose\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            m,
            l,
            &raw mut c_b5,
            c__.offset(
                ((*n - *l + 1 as ::core::ffi::c_long) * c_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            ldc,
            v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            incv,
            &raw mut c_b5,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
        );
        d__1 = -*tau;
        dsyevd_closure_f2c_daxpy_0(
            m,
            &raw mut d__1,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            c__.offset(c_offset as isize) as *mut doublereal,
            &raw mut c__1,
        );
        d__1 = -*tau;
        dsyevd_closure_f2c_dger_0(
            m,
            l,
            &raw mut d__1,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            &raw mut c__1,
            v.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
            incv,
            c__.offset(
                ((*n - *l + 1 as ::core::ffi::c_long) * c_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            ldc,
        );
    }
    return 0 as ::core::ffi::c_int;
}
