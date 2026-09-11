pub type integer = ::core::ffi::c_long;
pub type doublereal = ::core::ffi::c_double;
#[no_mangle]
pub unsafe extern "C" fn slsqp_closure_dlatrz_(
    mut m: *mut integer,
    mut n: *mut integer,
    mut l: *mut integer,
    mut a: *mut doublereal,
    mut lda: *mut integer,
    mut tau: *mut doublereal,
    mut work: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut a_dim1: integer = 0;
    let mut a_offset: integer = 0;
    let mut i__1: integer = 0;
    let mut i__2: integer = 0;
    let mut i__: integer = 0;
    extern "C" {
        #[link_name = "slsqp_closure_dlarz_"]
        fn slsqp_closure_dlarz__0(
            _: *mut ::core::ffi::c_char,
            _: *mut integer,
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
        #[link_name = "dgeev_closure_dlarfg_"]
        fn dgeev_closure_dlarfg__0(
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
    tau = tau.offset(-1);
    work = work.offset(-1);
    if *m == 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    } else if *m == *n {
        i__1 = *n;
        i__ = 1 as integer;
        while i__ <= i__1 {
            *tau.offset(i__ as isize) = 0.0f64 as doublereal;
            i__ += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    i__ = *m;
    while i__ >= 1 as ::core::ffi::c_long {
        i__1 = (*l + 1 as ::core::ffi::c_long) as integer;
        dgeev_closure_dlarfg__0(
            &raw mut i__1,
            a.offset((i__ + i__ * a_dim1) as isize) as *mut doublereal,
            a.offset((i__ + (*n - *l + 1 as integer) * a_dim1) as isize) as *mut doublereal,
            lda,
            tau.offset(i__ as isize) as *mut doublereal,
        );
        i__1 = (i__ as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as integer;
        i__2 = (*n - i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        slsqp_closure_dlarz__0(
            b"Right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut i__1,
            &raw mut i__2,
            l,
            a.offset((i__ + (*n - *l + 1 as integer) * a_dim1) as isize) as *mut doublereal,
            lda,
            tau.offset(i__ as isize) as *mut doublereal,
            a.offset(
                (i__ as ::core::ffi::c_long * a_dim1 as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as isize,
            ) as *mut doublereal,
            lda,
            work.offset(1 as ::core::ffi::c_int as isize) as *mut doublereal,
        );
        i__ -= 1;
    }
    return 0 as ::core::ffi::c_int;
}
