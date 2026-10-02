use crate::runtime::{malloc,calloc,free,sqrt};
extern "C" {
}
pub type OSQPInt = ::core::ffi::c_int;
pub type OSQPFloat = ::core::ffi::c_double;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPVectori_ {
    pub values: *mut OSQPInt,
    pub length: OSQPInt,
}
pub type OSQPVectori = OSQPVectori_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPVectorf_ {
    pub values: *mut OSQPFloat,
    pub length: OSQPInt,
}
pub type OSQPVectorf = OSQPVectorf_;
pub const OSQP_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "honest_osqp_OSQPVectorf_is_eq"]
pub unsafe extern "C" fn OSQPVectorf_is_eq(
    mut A: *const OSQPVectorf,
    mut B: *const OSQPVectorf,
    mut tol: OSQPFloat,
) -> OSQPInt {
    if (*A).length != (*B).length {
        return 0 as OSQPInt;
    }
    let mut i: OSQPInt = 0;
    let mut retval: OSQPInt = 1 as OSQPInt;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < (*A).length {
        if (if *(*A).values.offset(i as isize) - *(*B).values.offset(i as isize)
            < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -(*(*A).values.offset(i as isize) as ::core::ffi::c_double
                - *(*B).values.offset(i as isize) as ::core::ffi::c_double)
        } else {
            *(*A).values.offset(i as isize) as ::core::ffi::c_double
                - *(*B).values.offset(i as isize) as ::core::ffi::c_double
        }) > tol
        {
            retval = 0 as ::core::ffi::c_int as OSQPInt;
        }
        i += 1;
    }
    return retval;
}
#[export_name = "honest_osqp_OSQPVectorf_new"]
pub unsafe extern "C" fn OSQPVectorf_new(
    mut a: *const OSQPFloat,
    mut length: OSQPInt,
) -> *mut OSQPVectorf {
    let mut out: *mut OSQPVectorf = OSQPVectorf_malloc(length);
    if out.is_null() {
        return ::core::ptr::null_mut::<OSQPVectorf>();
    }
    if length > 0 as ::core::ffi::c_int {
        OSQPVectorf_from_raw(out, a);
    }
    return out;
}
#[export_name = "honest_osqp_OSQPVectori_new"]
pub unsafe extern "C" fn OSQPVectori_new(
    mut a: *const OSQPInt,
    mut length: OSQPInt,
) -> *mut OSQPVectori {
    let mut out: *mut OSQPVectori = OSQPVectori_malloc(length);
    if out.is_null() {
        return ::core::ptr::null_mut::<OSQPVectori>();
    }
    if length > 0 as ::core::ffi::c_int {
        OSQPVectori_from_raw(out, a);
    }
    return out;
}
#[export_name = "honest_osqp_OSQPVectorf_malloc"]
pub unsafe extern "C" fn OSQPVectorf_malloc(mut length: OSQPInt) -> *mut OSQPVectorf {
    let mut b: *mut OSQPVectorf =
        malloc(::core::mem::size_of::<OSQPVectorf>() as size_t) as *mut OSQPVectorf;
    if !b.is_null() {
        (*b).length = length;
        if length != 0 {
            (*b).values = malloc(
                (length as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t),
            ) as *mut OSQPFloat;
            if (*b).values.is_null() {
                free(b as *mut ::core::ffi::c_void);
                b = ::core::ptr::null_mut::<OSQPVectorf>();
            }
        } else {
            (*b).values = ::core::ptr::null_mut::<OSQPFloat>();
        }
    }
    return b;
}
#[export_name = "honest_osqp_OSQPVectori_malloc"]
pub unsafe extern "C" fn OSQPVectori_malloc(mut length: OSQPInt) -> *mut OSQPVectori {
    let mut b: *mut OSQPVectori =
        malloc(::core::mem::size_of::<OSQPVectori>() as size_t) as *mut OSQPVectori;
    if !b.is_null() {
        (*b).length = length;
        if length != 0 {
            (*b).values = malloc(
                (length as size_t).wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t),
            ) as *mut OSQPInt;
            if (*b).values.is_null() {
                free(b as *mut ::core::ffi::c_void);
                b = ::core::ptr::null_mut::<OSQPVectori>();
            }
        } else {
            (*b).values = ::core::ptr::null_mut::<OSQPInt>();
        }
    }
    return b;
}
#[export_name = "honest_osqp_OSQPVectorf_calloc"]
pub unsafe extern "C" fn OSQPVectorf_calloc(mut length: OSQPInt) -> *mut OSQPVectorf {
    let mut b: *mut OSQPVectorf =
        malloc(::core::mem::size_of::<OSQPVectorf>() as size_t) as *mut OSQPVectorf;
    if !b.is_null() {
        (*b).length = length;
        if length != 0 {
            (*b).values = calloc(
                length as size_t,
                ::core::mem::size_of::<OSQPFloat>() as size_t,
            ) as *mut OSQPFloat;
            if (*b).values.is_null() {
                free(b as *mut ::core::ffi::c_void);
                b = ::core::ptr::null_mut::<OSQPVectorf>();
            }
        } else {
            (*b).values = ::core::ptr::null_mut::<OSQPFloat>();
        }
    }
    return b;
}
#[export_name = "honest_osqp_OSQPVectori_calloc"]
pub unsafe extern "C" fn OSQPVectori_calloc(mut length: OSQPInt) -> *mut OSQPVectori {
    let mut b: *mut OSQPVectori =
        malloc(::core::mem::size_of::<OSQPVectori>() as size_t) as *mut OSQPVectori;
    if !b.is_null() {
        (*b).length = length;
        if length != 0 {
            (*b).values = calloc(
                length as size_t,
                ::core::mem::size_of::<OSQPInt>() as size_t,
            ) as *mut OSQPInt;
            if (*b).values.is_null() {
                free(b as *mut ::core::ffi::c_void);
                b = ::core::ptr::null_mut::<OSQPVectori>();
            }
        } else {
            (*b).values = ::core::ptr::null_mut::<OSQPInt>();
        }
    }
    return b;
}
#[export_name = "honest_osqp_OSQPVectorf_copy_new"]
pub unsafe extern "C" fn OSQPVectorf_copy_new(mut a: *const OSQPVectorf) -> *mut OSQPVectorf {
    let mut b: *mut OSQPVectorf = OSQPVectorf_malloc((*a).length);
    if !b.is_null() {
        OSQPVectorf_copy(b, a);
    }
    return b;
}
#[export_name = "honest_osqp_OSQPVectorf_free"]
pub unsafe extern "C" fn OSQPVectorf_free(mut a: *mut OSQPVectorf) {
    if !a.is_null() {
        free((*a).values as *mut ::core::ffi::c_void);
    }
    free(a as *mut ::core::ffi::c_void);
}
#[export_name = "honest_osqp_OSQPVectori_free"]
pub unsafe extern "C" fn OSQPVectori_free(mut a: *mut OSQPVectori) {
    if !a.is_null() {
        free((*a).values as *mut ::core::ffi::c_void);
    }
    free(a as *mut ::core::ffi::c_void);
}
#[export_name = "honest_osqp_OSQPVectorf_subvector_assign"]
pub unsafe extern "C" fn OSQPVectorf_subvector_assign(
    mut A: *mut OSQPVectorf,
    mut b: *mut OSQPFloat,
    mut start: OSQPInt,
    mut length: OSQPInt,
    mut multiplier: OSQPFloat,
) {
    let mut i: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *(*A).values.offset((start + i) as isize) = multiplier * *b.offset(i as isize);
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_subvector_assign_scalar"]
pub unsafe extern "C" fn OSQPVectorf_subvector_assign_scalar(
    mut A: *mut OSQPVectorf,
    mut sc: OSQPFloat,
    mut start: OSQPInt,
    mut length: OSQPInt,
) {
    let mut i: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *(*A).values.offset((start + i) as isize) = sc;
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_subvector_byrows"]
pub unsafe extern "C" fn OSQPVectorf_subvector_byrows(
    mut A: *const OSQPVectorf,
    mut rows: *const OSQPVectori,
) -> *mut OSQPVectorf {
    let mut i: OSQPInt = 0;
    let mut rows_len: OSQPInt = 0 as OSQPInt;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < (*rows).length {
        if *(*rows).values.offset(i as isize) != 0 {
            rows_len += 1;
        }
        i += 1;
    }
    let mut out: *mut OSQPVectorf = OSQPVectorf_malloc(rows_len);
    if out.is_null() {
        return ::core::ptr::null_mut::<OSQPVectorf>();
    }
    let mut j: OSQPInt = 0 as OSQPInt;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < (*rows).length {
        if *(*rows).values.offset(i as isize) != 0 {
            *(*out).values.offset(j as isize) = *(*A).values.offset(i as isize);
            j += 1;
        }
        i += 1;
    }
    return out;
}
#[export_name = "honest_osqp_OSQPVectorf_concat"]
pub unsafe extern "C" fn OSQPVectorf_concat(
    mut A: *const OSQPVectorf,
    mut B: *const OSQPVectorf,
) -> *mut OSQPVectorf {
    let mut out: *mut OSQPVectorf = OSQPVectorf_malloc((*A).length + (*B).length);
    if out.is_null() {
        return ::core::ptr::null_mut::<OSQPVectorf>();
    }
    let mut i: OSQPInt = 0;
    let mut j: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < (*A).length {
        *(*out).values.offset(i as isize) = *(*A).values.offset(i as isize);
        i += 1;
    }
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*B).length {
        *(*out).values.offset((j + i) as isize) = *(*B).values.offset(j as isize);
        j += 1;
    }
    return out;
}
#[export_name = "honest_osqp_OSQPVectorf_view"]
pub unsafe extern "C" fn OSQPVectorf_view(
    mut a: *const OSQPVectorf,
    mut head: OSQPInt,
    mut length: OSQPInt,
) -> *mut OSQPVectorf {
    let mut view: *mut OSQPVectorf =
        malloc(::core::mem::size_of::<OSQPVectorf>() as size_t) as *mut OSQPVectorf;
    if !view.is_null() {
        OSQPVectorf_view_update(view, a, head, length);
    }
    return view;
}
#[export_name = "honest_osqp_OSQPVectorf_view_update"]
pub unsafe extern "C" fn OSQPVectorf_view_update(
    mut a: *mut OSQPVectorf,
    mut b: *const OSQPVectorf,
    mut head: OSQPInt,
    mut length: OSQPInt,
) {
    (*a).length = length;
    (*a).values = (*b).values.offset(head as isize);
}
#[export_name = "honest_osqp_OSQPVectorf_view_free"]
pub unsafe extern "C" fn OSQPVectorf_view_free(mut a: *mut OSQPVectorf) {
    free(a as *mut ::core::ffi::c_void);
}
#[export_name = "honest_osqp_OSQPVectorf_norm_2"]
pub unsafe extern "C" fn OSQPVectorf_norm_2(mut v: *const OSQPVectorf) -> OSQPFloat {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*v).length;
    let mut vv: *mut OSQPFloat = (*v).values;
    let mut normval: OSQPFloat = 0.0f64;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        normval += (*vv.offset(i as isize) * *vv.offset(i as isize)) as ::core::ffi::c_double;
        i += 1;
    }
    return sqrt(normval as ::core::ffi::c_double) as OSQPFloat;
}
#[export_name = "honest_osqp_OSQPVectorf_length"]
pub unsafe extern "C" fn OSQPVectorf_length(mut a: *const OSQPVectorf) -> OSQPInt {
    return (*a).length;
}
#[export_name = "honest_osqp_OSQPVectori_length"]
pub unsafe extern "C" fn OSQPVectori_length(mut a: *const OSQPVectori) -> OSQPInt {
    return (*a).length;
}
#[export_name = "honest_osqp_OSQPVectorf_data"]
pub unsafe extern "C" fn OSQPVectorf_data(mut a: *const OSQPVectorf) -> *mut OSQPFloat {
    return (*a).values;
}
#[export_name = "honest_osqp_OSQPVectorf_copy"]
pub unsafe extern "C" fn OSQPVectorf_copy(mut b: *mut OSQPVectorf, mut a: *const OSQPVectorf) {
    OSQPVectorf_from_raw(b, (*a).values);
}
#[export_name = "honest_osqp_OSQPVectorf_from_raw"]
pub unsafe extern "C" fn OSQPVectorf_from_raw(mut b: *mut OSQPVectorf, mut av: *const OSQPFloat) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*b).length;
    let mut bv: *mut OSQPFloat = (*b).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *bv.offset(i as isize) = *av.offset(i as isize);
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectori_from_raw"]
pub unsafe extern "C" fn OSQPVectori_from_raw(mut b: *mut OSQPVectori, mut av: *const OSQPInt) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*b).length;
    let mut bv: *mut OSQPInt = (*b).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *bv.offset(i as isize) = *av.offset(i as isize);
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_to_raw"]
pub unsafe extern "C" fn OSQPVectorf_to_raw(mut bv: *mut OSQPFloat, mut a: *const OSQPVectorf) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *bv.offset(i as isize) = *av.offset(i as isize);
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectori_to_raw"]
pub unsafe extern "C" fn OSQPVectori_to_raw(mut bv: *mut OSQPInt, mut a: *const OSQPVectori) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPInt = (*a).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *bv.offset(i as isize) = *av.offset(i as isize);
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_set_scalar"]
pub unsafe extern "C" fn OSQPVectorf_set_scalar(mut a: *mut OSQPVectorf, mut sc: OSQPFloat) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *av.offset(i as isize) = sc;
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_set_scalar_conditional"]
pub unsafe extern "C" fn OSQPVectorf_set_scalar_conditional(
    mut a: *mut OSQPVectorf,
    mut test: *const OSQPVectori,
    mut sc_if_neg: OSQPFloat,
    mut sc_if_zero: OSQPFloat,
    mut sc_if_pos: OSQPFloat,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut testv: *mut OSQPInt = (*test).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        if *testv.offset(i as isize) == 0 as ::core::ffi::c_int {
            *av.offset(i as isize) = sc_if_zero;
        } else if *testv.offset(i as isize) > 0 as ::core::ffi::c_int {
            *av.offset(i as isize) = sc_if_pos;
        } else {
            *av.offset(i as isize) = sc_if_neg;
        }
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_round_to_zero"]
pub unsafe extern "C" fn OSQPVectorf_round_to_zero(mut a: *mut OSQPVectorf, mut tol: OSQPFloat) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        if (if *av.offset(i as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -(*av.offset(i as isize) as ::core::ffi::c_double)
        } else {
            *av.offset(i as isize) as ::core::ffi::c_double
        }) < tol
        {
            *av.offset(i as isize) = 0.0f64;
        }
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_mult_scalar"]
pub unsafe extern "C" fn OSQPVectorf_mult_scalar(mut a: *mut OSQPVectorf, mut sc: OSQPFloat) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        let ref mut fresh0 = *av.offset(i as isize);
        *fresh0 *= sc as ::core::ffi::c_double;
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_plus"]
pub unsafe extern "C" fn OSQPVectorf_plus(
    mut x: *mut OSQPVectorf,
    mut a: *const OSQPVectorf,
    mut b: *const OSQPVectorf,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut xv: *mut OSQPFloat = (*x).values;
    if x == a as *mut OSQPVectorf {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            let ref mut fresh1 = *xv.offset(i as isize);
            *fresh1 += *bv.offset(i as isize) as ::core::ffi::c_double;
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            *xv.offset(i as isize) = *av.offset(i as isize) + *bv.offset(i as isize);
            i += 1;
        }
    };
}
#[export_name = "honest_osqp_OSQPVectorf_minus"]
pub unsafe extern "C" fn OSQPVectorf_minus(
    mut x: *mut OSQPVectorf,
    mut a: *const OSQPVectorf,
    mut b: *const OSQPVectorf,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut xv: *mut OSQPFloat = (*x).values;
    if x == a as *mut OSQPVectorf {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            let ref mut fresh2 = *xv.offset(i as isize);
            *fresh2 -= *bv.offset(i as isize) as ::core::ffi::c_double;
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            *xv.offset(i as isize) = *av.offset(i as isize) - *bv.offset(i as isize);
            i += 1;
        }
    };
}
#[export_name = "honest_osqp_OSQPVectorf_add_scaled"]
pub unsafe extern "C" fn OSQPVectorf_add_scaled(
    mut x: *mut OSQPVectorf,
    mut sca: OSQPFloat,
    mut a: *const OSQPVectorf,
    mut scb: OSQPFloat,
    mut b: *const OSQPVectorf,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*x).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut xv: *mut OSQPFloat = (*x).values;
    if x == a as *mut OSQPVectorf && sca == 1.0f64 {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            let ref mut fresh3 = *xv.offset(i as isize);
            *fresh3 += (scb * *bv.offset(i as isize)) as ::core::ffi::c_double;
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            *xv.offset(i as isize) = sca * *av.offset(i as isize) + scb * *bv.offset(i as isize);
            i += 1;
        }
    };
}
#[export_name = "honest_osqp_OSQPVectorf_add_scaled3"]
pub unsafe extern "C" fn OSQPVectorf_add_scaled3(
    mut x: *mut OSQPVectorf,
    mut sca: OSQPFloat,
    mut a: *const OSQPVectorf,
    mut scb: OSQPFloat,
    mut b: *const OSQPVectorf,
    mut scc: OSQPFloat,
    mut c: *const OSQPVectorf,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*x).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut cv: *mut OSQPFloat = (*c).values;
    let mut xv: *mut OSQPFloat = (*x).values;
    if x == a as *mut OSQPVectorf && sca == 1.0f64 {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            let ref mut fresh4 = *xv.offset(i as isize);
            *fresh4 += (scb * *bv.offset(i as isize) + scc * *cv.offset(i as isize))
                as ::core::ffi::c_double;
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            *xv.offset(i as isize) = sca * *av.offset(i as isize)
                + scb * *bv.offset(i as isize)
                + scc * *cv.offset(i as isize);
            i += 1;
        }
    };
}
#[export_name = "honest_osqp_OSQPVectorf_norm_inf"]
pub unsafe extern "C" fn OSQPVectorf_norm_inf(mut v: *const OSQPVectorf) -> OSQPFloat {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*v).length;
    let mut absval: OSQPFloat = 0.;
    let mut normval: OSQPFloat = 0.0f64;
    let mut vv: *mut OSQPFloat = (*v).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        absval = (if *vv.offset(i as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            -(*vv.offset(i as isize) as ::core::ffi::c_double)
        } else {
            *vv.offset(i as isize) as ::core::ffi::c_double
        }) as OSQPFloat;
        if absval > normval {
            normval = absval;
        }
        i += 1;
    }
    return normval;
}
#[export_name = "honest_osqp_OSQPVectorf_scaled_norm_inf"]
pub unsafe extern "C" fn OSQPVectorf_scaled_norm_inf(
    mut S: *const OSQPVectorf,
    mut v: *const OSQPVectorf,
) -> OSQPFloat {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*v).length;
    let mut vv: *mut OSQPFloat = (*v).values;
    let mut Sv: *mut OSQPFloat = (*S).values;
    let mut absval: OSQPFloat = 0.;
    let mut normval: OSQPFloat = 0.0f64;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        absval = (if *Sv.offset(i as isize) * *vv.offset(i as isize)
            < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -(*Sv.offset(i as isize) as ::core::ffi::c_double
                * *vv.offset(i as isize) as ::core::ffi::c_double)
        } else {
            *Sv.offset(i as isize) as ::core::ffi::c_double
                * *vv.offset(i as isize) as ::core::ffi::c_double
        }) as OSQPFloat;
        if absval > normval {
            normval = absval;
        }
        i += 1;
    }
    return normval;
}
#[export_name = "honest_osqp_OSQPVectorf_norm_inf_diff"]
pub unsafe extern "C" fn OSQPVectorf_norm_inf_diff(
    mut a: *const OSQPVectorf,
    mut b: *const OSQPVectorf,
) -> OSQPFloat {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut absval: OSQPFloat = 0.;
    let mut normDiff: OSQPFloat = 0.0f64;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        absval = (if *av.offset(i as isize) - *bv.offset(i as isize)
            < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            -(*av.offset(i as isize) as ::core::ffi::c_double
                - *bv.offset(i as isize) as ::core::ffi::c_double)
        } else {
            *av.offset(i as isize) as ::core::ffi::c_double
                - *bv.offset(i as isize) as ::core::ffi::c_double
        }) as OSQPFloat;
        if absval > normDiff {
            normDiff = absval;
        }
        i += 1;
    }
    return normDiff;
}
#[export_name = "honest_osqp_OSQPVectorf_dot_prod"]
pub unsafe extern "C" fn OSQPVectorf_dot_prod(
    mut a: *const OSQPVectorf,
    mut b: *const OSQPVectorf,
) -> OSQPFloat {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut dotprod: OSQPFloat = 0.0f64;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        dotprod += (*av.offset(i as isize) * *bv.offset(i as isize)) as ::core::ffi::c_double;
        i += 1;
    }
    return dotprod;
}
#[export_name = "honest_osqp_OSQPVectorf_dot_prod_signed"]
pub unsafe extern "C" fn OSQPVectorf_dot_prod_signed(
    mut a: *const OSQPVectorf,
    mut b: *const OSQPVectorf,
    mut sign: OSQPInt,
) -> OSQPFloat {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut dotprod: OSQPFloat = 0.0f64;
    if sign == 1 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            dotprod += *av.offset(i as isize) as ::core::ffi::c_double
                * (if *bv.offset(i as isize) > 0.0f64 {
                    *bv.offset(i as isize) as ::core::ffi::c_double
                } else {
                    0.0f64
                });
            i += 1;
        }
    } else if sign == -(1 as ::core::ffi::c_int) {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            dotprod += *av.offset(i as isize) as ::core::ffi::c_double
                * (if *bv.offset(i as isize) < 0.0f64 {
                    *bv.offset(i as isize) as ::core::ffi::c_double
                } else {
                    0.0f64
                });
            i += 1;
        }
    } else {
        dotprod = OSQPVectorf_dot_prod(a, b);
    }
    return dotprod;
}
#[export_name = "honest_osqp_OSQPVectorf_ew_prod"]
pub unsafe extern "C" fn OSQPVectorf_ew_prod(
    mut c: *mut OSQPVectorf,
    mut a: *const OSQPVectorf,
    mut b: *const OSQPVectorf,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut cv: *mut OSQPFloat = (*c).values;
    if c == a as *mut OSQPVectorf {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            let ref mut fresh5 = *cv.offset(i as isize);
            *fresh5 *= *bv.offset(i as isize) as ::core::ffi::c_double;
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            *cv.offset(i as isize) = *av.offset(i as isize) * *bv.offset(i as isize);
            i += 1;
        }
    };
}
#[export_name = "honest_osqp_OSQPVectorf_all_leq"]
pub unsafe extern "C" fn OSQPVectorf_all_leq(
    mut l: *const OSQPVectorf,
    mut u: *const OSQPVectorf,
) -> OSQPInt {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*l).length;
    let mut lv: *mut OSQPFloat = (*l).values;
    let mut uv: *mut OSQPFloat = (*u).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        if *lv.offset(i as isize) > *uv.offset(i as isize) {
            return 0 as OSQPInt;
        }
        i += 1;
    }
    return 1 as OSQPInt;
}
#[export_name = "honest_osqp_OSQPVectorf_ew_bound_vec"]
pub unsafe extern "C" fn OSQPVectorf_ew_bound_vec(
    mut x: *mut OSQPVectorf,
    mut z: *const OSQPVectorf,
    mut l: *const OSQPVectorf,
    mut u: *const OSQPVectorf,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*x).length;
    let mut xv: *mut OSQPFloat = (*x).values;
    let mut zv: *mut OSQPFloat = (*z).values;
    let mut lv: *mut OSQPFloat = (*l).values;
    let mut uv: *mut OSQPFloat = (*u).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *xv.offset(i as isize) = (if (if *zv.offset(i as isize) > *lv.offset(i as isize) {
            *zv.offset(i as isize) as ::core::ffi::c_double
        } else {
            *lv.offset(i as isize) as ::core::ffi::c_double
        }) < *uv.offset(i as isize)
        {
            if *zv.offset(i as isize) > *lv.offset(i as isize) {
                *zv.offset(i as isize) as ::core::ffi::c_double
            } else {
                *lv.offset(i as isize) as ::core::ffi::c_double
            }
        } else {
            *uv.offset(i as isize) as ::core::ffi::c_double
        }) as OSQPFloat;
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_project_polar_reccone"]
pub unsafe extern "C" fn OSQPVectorf_project_polar_reccone(
    mut y: *mut OSQPVectorf,
    mut l: *const OSQPVectorf,
    mut u: *const OSQPVectorf,
    mut infval: OSQPFloat,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*y).length;
    let mut yv: *mut OSQPFloat = (*y).values;
    let mut lv: *mut OSQPFloat = (*l).values;
    let mut uv: *mut OSQPFloat = (*u).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        if *uv.offset(i as isize) > infval {
            if *lv.offset(i as isize) < -infval {
                *yv.offset(i as isize) = 0.0f64 as OSQPFloat;
            } else {
                *yv.offset(i as isize) = (if *yv.offset(i as isize) < 0.0f64 {
                    *yv.offset(i as isize) as ::core::ffi::c_double
                } else {
                    0.0f64
                }) as OSQPFloat;
            }
        } else if *lv.offset(i as isize) < -infval {
            *yv.offset(i as isize) = (if *yv.offset(i as isize) > 0.0f64 {
                *yv.offset(i as isize) as ::core::ffi::c_double
            } else {
                0.0f64
            }) as OSQPFloat;
        }
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_in_reccone"]
pub unsafe extern "C" fn OSQPVectorf_in_reccone(
    mut y: *const OSQPVectorf,
    mut l: *const OSQPVectorf,
    mut u: *const OSQPVectorf,
    mut infval: OSQPFloat,
    mut tol: OSQPFloat,
) -> OSQPInt {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*y).length;
    let mut yv: *mut OSQPFloat = (*y).values;
    let mut lv: *mut OSQPFloat = (*l).values;
    let mut uv: *mut OSQPFloat = (*u).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        if *uv.offset(i as isize) < infval && *yv.offset(i as isize) > tol
            || *lv.offset(i as isize) > -infval && *yv.offset(i as isize) < -tol
        {
            return 0 as OSQPInt;
        }
        i += 1;
    }
    return 1 as OSQPInt;
}
#[export_name = "honest_osqp_OSQPVectorf_norm_1"]
pub unsafe extern "C" fn OSQPVectorf_norm_1(mut a: *const OSQPVectorf) -> OSQPFloat {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut val: OSQPFloat = 0.0f64;
    if length != 0 {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < length {
            val += if *av.offset(i as isize) < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -(*av.offset(i as isize) as ::core::ffi::c_double)
            } else {
                *av.offset(i as isize) as ::core::ffi::c_double
            };
            i += 1;
        }
    }
    return val;
}
#[export_name = "honest_osqp_OSQPVectorf_ew_reciprocal"]
pub unsafe extern "C" fn OSQPVectorf_ew_reciprocal(
    mut b: *mut OSQPVectorf,
    mut a: *const OSQPVectorf,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *bv.offset(i as isize) = 1.0f64 / *av.offset(i as isize);
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_ew_sqrt"]
pub unsafe extern "C" fn OSQPVectorf_ew_sqrt(mut a: *mut OSQPVectorf) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *av.offset(i as isize) = sqrt(*av.offset(i as isize) as ::core::ffi::c_double) as OSQPFloat;
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_ew_max_vec"]
pub unsafe extern "C" fn OSQPVectorf_ew_max_vec(
    mut c: *mut OSQPVectorf,
    mut a: *const OSQPVectorf,
    mut b: *const OSQPVectorf,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut cv: *mut OSQPFloat = (*c).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *cv.offset(i as isize) = (if *av.offset(i as isize) > *bv.offset(i as isize) {
            *av.offset(i as isize) as ::core::ffi::c_double
        } else {
            *bv.offset(i as isize) as ::core::ffi::c_double
        }) as OSQPFloat;
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_ew_min_vec"]
pub unsafe extern "C" fn OSQPVectorf_ew_min_vec(
    mut c: *mut OSQPVectorf,
    mut a: *const OSQPVectorf,
    mut b: *const OSQPVectorf,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*a).length;
    let mut av: *mut OSQPFloat = (*a).values;
    let mut bv: *mut OSQPFloat = (*b).values;
    let mut cv: *mut OSQPFloat = (*c).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *cv.offset(i as isize) = (if *av.offset(i as isize) < *bv.offset(i as isize) {
            *av.offset(i as isize) as ::core::ffi::c_double
        } else {
            *bv.offset(i as isize) as ::core::ffi::c_double
        }) as OSQPFloat;
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_ew_bounds_type"]
pub unsafe extern "C" fn OSQPVectorf_ew_bounds_type(
    mut iseq: *mut OSQPVectori,
    mut l: *const OSQPVectorf,
    mut u: *const OSQPVectorf,
    mut tol: OSQPFloat,
    mut infval: OSQPFloat,
) -> OSQPInt {
    let mut i: OSQPInt = 0;
    let mut old_value: OSQPInt = 0;
    let mut has_changed: OSQPInt = 0 as OSQPInt;
    let mut length: OSQPInt = (*iseq).length;
    let mut iseqv: *mut OSQPInt = (*iseq).values;
    let mut lv: *mut OSQPFloat = (*l).values;
    let mut uv: *mut OSQPFloat = (*u).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        old_value = *iseqv.offset(i as isize);
        if *lv.offset(i as isize) < -infval && *uv.offset(i as isize) > infval {
            *iseqv.offset(i as isize) = -(1 as ::core::ffi::c_int) as OSQPInt;
        } else if *uv.offset(i as isize) - *lv.offset(i as isize) < tol {
            *iseqv.offset(i as isize) = 1 as ::core::ffi::c_int as OSQPInt;
        } else {
            *iseqv.offset(i as isize) = 0 as ::core::ffi::c_int as OSQPInt;
        }
        has_changed = (has_changed != 0 || *iseqv.offset(i as isize) != old_value)
            as ::core::ffi::c_int as OSQPInt;
        i += 1;
    }
    return has_changed;
}
#[export_name = "honest_osqp_OSQPVectorf_set_scalar_if_lt"]
pub unsafe extern "C" fn OSQPVectorf_set_scalar_if_lt(
    mut x: *mut OSQPVectorf,
    mut z: *const OSQPVectorf,
    mut testval: OSQPFloat,
    mut newval: OSQPFloat,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*x).length;
    let mut xv: *mut OSQPFloat = (*x).values;
    let mut zv: *mut OSQPFloat = (*z).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *xv.offset(i as isize) = (if *zv.offset(i as isize) < testval {
            newval as ::core::ffi::c_double
        } else {
            *zv.offset(i as isize) as ::core::ffi::c_double
        }) as OSQPFloat;
        i += 1;
    }
}
#[export_name = "honest_osqp_OSQPVectorf_set_scalar_if_gt"]
pub unsafe extern "C" fn OSQPVectorf_set_scalar_if_gt(
    mut x: *mut OSQPVectorf,
    mut z: *const OSQPVectorf,
    mut testval: OSQPFloat,
    mut newval: OSQPFloat,
) {
    let mut i: OSQPInt = 0;
    let mut length: OSQPInt = (*x).length;
    let mut xv: *mut OSQPFloat = (*x).values;
    let mut zv: *mut OSQPFloat = (*z).values;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < length {
        *xv.offset(i as isize) = (if *zv.offset(i as isize) > testval {
            newval as ::core::ffi::c_double
        } else {
            *zv.offset(i as isize) as ::core::ffi::c_double
        }) as OSQPFloat;
        i += 1;
    }
}
