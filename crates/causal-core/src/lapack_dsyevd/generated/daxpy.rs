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
pub unsafe extern "C" fn dsyevd_closure_f2c_daxpy(
    mut n: *mut integer,
    mut da: *mut doublereal,
    mut dx: *mut doublereal,
    mut incx: *mut integer,
    mut dy: *mut doublereal,
    mut incy: *mut integer,
) -> ::core::ffi::c_int {
    let mut i__1: integer = 0;
    let mut i__: integer = 0;
    let mut m: integer = 0;
    let mut ix: integer = 0;
    let mut iy: integer = 0;
    let mut mp1: integer = 0;
    dy = dy.offset(-1);
    dx = dx.offset(-1);
    if *n <= 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *da == 0.0f64 {
        return 0 as ::core::ffi::c_int;
    }
    if *incx == 1 as ::core::ffi::c_long && *incy == 1 as ::core::ffi::c_long {
        m = (*n % 4 as ::core::ffi::c_long) as integer;
        if !(m == 0 as ::core::ffi::c_long) {
            i__1 = m;
            i__ = 1 as integer;
            while i__ <= i__1 {
                let ref mut fresh1 = *dy.offset(i__ as isize);
                *fresh1 += (*da * *dx.offset(i__ as isize)) as ::core::ffi::c_double;
                i__ += 1;
            }
            if *n < 4 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
        }
        mp1 = (m as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as integer;
        i__1 = *n;
        i__ = mp1;
        while i__ <= i__1 {
            let ref mut fresh2 = *dy.offset(i__ as isize);
            *fresh2 += (*da * *dx.offset(i__ as isize)) as ::core::ffi::c_double;
            let ref mut fresh3 =
                *dy.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize);
            *fresh3 += (*da
                * *dx.offset((i__ as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as isize))
                as ::core::ffi::c_double;
            let ref mut fresh4 =
                *dy.offset((i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize);
            *fresh4 += (*da
                * *dx.offset((i__ as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as isize))
                as ::core::ffi::c_double;
            let ref mut fresh5 =
                *dy.offset((i__ as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize);
            *fresh5 += (*da
                * *dx.offset((i__ as ::core::ffi::c_long + 3 as ::core::ffi::c_long) as isize))
                as ::core::ffi::c_double;
            i__ += 4 as ::core::ffi::c_long;
        }
        return 0 as ::core::ffi::c_int;
    } else {
        ix = 1 as integer;
        iy = 1 as integer;
        if *incx < 0 as ::core::ffi::c_long {
            ix = ((-*n + 1 as ::core::ffi::c_long) * *incx + 1 as ::core::ffi::c_long) as integer;
        }
        if *incy < 0 as ::core::ffi::c_long {
            iy = ((-*n + 1 as ::core::ffi::c_long) * *incy + 1 as ::core::ffi::c_long) as integer;
        }
        i__1 = *n;
        i__ = 1 as integer;
        while i__ <= i__1 {
            let ref mut fresh0 = *dy.offset(iy as isize);
            *fresh0 += (*da * *dx.offset(ix as isize)) as ::core::ffi::c_double;
            ix += *incx as ::core::ffi::c_long;
            iy += *incy as ::core::ffi::c_long;
            i__ += 1;
        }
        return 0 as ::core::ffi::c_int;
    };
}
