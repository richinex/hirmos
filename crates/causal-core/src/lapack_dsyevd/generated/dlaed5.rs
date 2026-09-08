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
#[no_mangle]
pub unsafe extern "C" fn dsyevd_closure_dlaed5_(
    mut i__: *mut integer,
    mut d__: *mut doublereal,
    mut z__: *mut doublereal,
    mut delta: *mut doublereal,
    mut rho: *mut doublereal,
    mut dlam: *mut doublereal,
) -> ::core::ffi::c_int {
    let mut d__1: doublereal = 0.;
    let mut b: doublereal = 0.;
    let mut c__: doublereal = 0.;
    let mut w: doublereal = 0.;
    let mut del: doublereal = 0.;
    let mut tau: doublereal = 0.;
    let mut temp: doublereal = 0.;
    delta = delta.offset(-1);
    z__ = z__.offset(-1);
    d__ = d__.offset(-1);
    del = *d__.offset(2 as ::core::ffi::c_int as isize)
        - *d__.offset(1 as ::core::ffi::c_int as isize);
    if *i__ == 1 as ::core::ffi::c_long {
        w = (*rho
            * 2.0f64
            * (*z__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                * *z__.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                - *z__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    * *z__.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
            / del as ::core::ffi::c_double
            + 1.0f64) as doublereal;
        if w > 0.0f64 {
            b = del
                + *rho
                    * (*z__.offset(1 as ::core::ffi::c_int as isize)
                        * *z__.offset(1 as ::core::ffi::c_int as isize)
                        + *z__.offset(2 as ::core::ffi::c_int as isize)
                            * *z__.offset(2 as ::core::ffi::c_int as isize));
            c__ = *rho
                * *z__.offset(1 as ::core::ffi::c_int as isize)
                * *z__.offset(1 as ::core::ffi::c_int as isize)
                * del;
            d__1 = (b as ::core::ffi::c_double * b as ::core::ffi::c_double
                - c__ as ::core::ffi::c_double * 4.0f64) as doublereal;
            tau = (c__ as ::core::ffi::c_double * 2.0f64
                / (b as ::core::ffi::c_double
                    + sqrt(
                        (if d__1 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            d__1
                        } else {
                            -d__1
                        }),
                    ))) as doublereal;
            *dlam = *d__.offset(1 as ::core::ffi::c_int as isize) + tau;
            *delta.offset(1 as ::core::ffi::c_int as isize) =
                -*z__.offset(1 as ::core::ffi::c_int as isize) / tau;
            *delta.offset(2 as ::core::ffi::c_int as isize) =
                *z__.offset(2 as ::core::ffi::c_int as isize) / (del - tau);
        } else {
            b = -del
                + *rho
                    * (*z__.offset(1 as ::core::ffi::c_int as isize)
                        * *z__.offset(1 as ::core::ffi::c_int as isize)
                        + *z__.offset(2 as ::core::ffi::c_int as isize)
                            * *z__.offset(2 as ::core::ffi::c_int as isize));
            c__ = *rho
                * *z__.offset(2 as ::core::ffi::c_int as isize)
                * *z__.offset(2 as ::core::ffi::c_int as isize)
                * del;
            if b > 0.0f64 {
                tau = (c__ as ::core::ffi::c_double * -2.0f64
                    / (b as ::core::ffi::c_double + sqrt(b * b + c__ * 4.0f64)))
                    as doublereal;
            } else {
                tau = ((b as ::core::ffi::c_double - sqrt(b * b + c__ * 4.0f64)) / 2.0f64)
                    as doublereal;
            }
            *dlam = *d__.offset(2 as ::core::ffi::c_int as isize) + tau;
            *delta.offset(1 as ::core::ffi::c_int as isize) =
                -*z__.offset(1 as ::core::ffi::c_int as isize) / (del + tau);
            *delta.offset(2 as ::core::ffi::c_int as isize) =
                -*z__.offset(2 as ::core::ffi::c_int as isize) / tau;
        }
        temp = sqrt(
            *delta.offset(1 as ::core::ffi::c_int as isize)
                * *delta.offset(1 as ::core::ffi::c_int as isize)
                + *delta.offset(2 as ::core::ffi::c_int as isize)
                    * *delta.offset(2 as ::core::ffi::c_int as isize),
        ) as doublereal;
        let ref mut fresh0 = *delta.offset(1 as ::core::ffi::c_int as isize);
        *fresh0 /= temp as ::core::ffi::c_double;
        let ref mut fresh1 = *delta.offset(2 as ::core::ffi::c_int as isize);
        *fresh1 /= temp as ::core::ffi::c_double;
    } else {
        b = -del
            + *rho
                * (*z__.offset(1 as ::core::ffi::c_int as isize)
                    * *z__.offset(1 as ::core::ffi::c_int as isize)
                    + *z__.offset(2 as ::core::ffi::c_int as isize)
                        * *z__.offset(2 as ::core::ffi::c_int as isize));
        c__ = *rho
            * *z__.offset(2 as ::core::ffi::c_int as isize)
            * *z__.offset(2 as ::core::ffi::c_int as isize)
            * del;
        if b > 0.0f64 {
            tau =
                ((b as ::core::ffi::c_double + sqrt(b * b + c__ * 4.0f64)) / 2.0f64) as doublereal;
        } else {
            tau = (c__ as ::core::ffi::c_double * 2.0f64
                / (-(b as ::core::ffi::c_double) + sqrt(b * b + c__ * 4.0f64)))
                as doublereal;
        }
        *dlam = *d__.offset(2 as ::core::ffi::c_int as isize) + tau;
        *delta.offset(1 as ::core::ffi::c_int as isize) =
            -*z__.offset(1 as ::core::ffi::c_int as isize) / (del + tau);
        *delta.offset(2 as ::core::ffi::c_int as isize) =
            -*z__.offset(2 as ::core::ffi::c_int as isize) / tau;
        temp = sqrt(
            *delta.offset(1 as ::core::ffi::c_int as isize)
                * *delta.offset(1 as ::core::ffi::c_int as isize)
                + *delta.offset(2 as ::core::ffi::c_int as isize)
                    * *delta.offset(2 as ::core::ffi::c_int as isize),
        ) as doublereal;
        let ref mut fresh2 = *delta.offset(1 as ::core::ffi::c_int as isize);
        *fresh2 /= temp as ::core::ffi::c_double;
        let ref mut fresh3 = *delta.offset(2 as ::core::ffi::c_int as isize);
        *fresh3 /= temp as ::core::ffi::c_double;
    }
    return 0 as ::core::ffi::c_int;
}
