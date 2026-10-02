use crate::runtime::{malloc,free,realloc,sqrt,fabs};
extern "C" {
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type OSQPFloat = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SuiteSparse_config_struct {
    pub malloc_func: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub realloc_func:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
    pub free_func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub printf_func:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ::core::ffi::c_int>,
    pub hypot_func: Option<unsafe extern "C" fn(OSQPFloat, OSQPFloat) -> OSQPFloat>,
    pub divcomplex_func: Option<
        unsafe extern "C" fn(
            OSQPFloat,
            OSQPFloat,
            OSQPFloat,
            OSQPFloat,
            *mut OSQPFloat,
            *mut OSQPFloat,
        ) -> ::core::ffi::c_int,
    >,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SUITESPARSE_MAIN_VERSION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SUITESPARSE_SUB_VERSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SUITESPARSE_SUBSUB_VERSION: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SUITESPARSE_VERSION: ::core::ffi::c_int =
    4 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int + 5 as ::core::ffi::c_int;
#[export_name = "honest_osqp_SuiteSparse_config"]
pub static SuiteSparse_config: SuiteSparse_config_struct = unsafe {
    SuiteSparse_config_struct {
        malloc_func: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        realloc_func: Some(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        free_func: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        printf_func: None,
        hypot_func: Some(
            SuiteSparse_hypot as unsafe extern "C" fn(OSQPFloat, OSQPFloat) -> OSQPFloat,
        ),
        divcomplex_func: Some(
            SuiteSparse_divcomplex
                as unsafe extern "C" fn(
                    OSQPFloat,
                    OSQPFloat,
                    OSQPFloat,
                    OSQPFloat,
                    *mut OSQPFloat,
                    *mut OSQPFloat,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[export_name = "honest_osqp_SuiteSparse_malloc"]
pub unsafe extern "C" fn SuiteSparse_malloc(
    mut nitems: size_t,
    mut size_of_item: size_t,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut size: size_t = 0;
    if nitems < 1 as size_t {
        nitems = 1 as size_t;
    }
    if size_of_item < 1 as size_t {
        size_of_item = 1 as size_t;
    }
    size = nitems.wrapping_mul(size_of_item);
    if size as ::core::ffi::c_double
        != nitems as ::core::ffi::c_double * size_of_item as ::core::ffi::c_double
    {
        p = NULL;
    } else {
        p = SuiteSparse_config
            .malloc_func
            .expect("non-null function pointer")(size);
    }
    return p;
}
#[export_name = "honest_osqp_SuiteSparse_realloc"]
pub unsafe extern "C" fn SuiteSparse_realloc(
    mut nitems_new: size_t,
    mut nitems_old: size_t,
    mut size_of_item: size_t,
    mut p: *mut ::core::ffi::c_void,
    mut ok: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut size: size_t = 0;
    if nitems_old < 1 as size_t {
        nitems_old = 1 as size_t;
    }
    if nitems_new < 1 as size_t {
        nitems_new = 1 as size_t;
    }
    if size_of_item < 1 as size_t {
        size_of_item = 1 as size_t;
    }
    size = nitems_new.wrapping_mul(size_of_item);
    if size as ::core::ffi::c_double
        != nitems_new as ::core::ffi::c_double * size_of_item as ::core::ffi::c_double
    {
        *ok = 0 as ::core::ffi::c_int;
    } else if p.is_null() {
        p = SuiteSparse_malloc(nitems_new, size_of_item);
        *ok = (p != NULL) as ::core::ffi::c_int;
    } else if nitems_old == nitems_new {
        *ok = 1 as ::core::ffi::c_int;
    } else {
        let mut pnew: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        pnew = realloc(p, size);
        if pnew.is_null() {
            if nitems_new < nitems_old {
                *ok = 1 as ::core::ffi::c_int;
            } else {
                *ok = 0 as ::core::ffi::c_int;
            }
        } else {
            p = pnew;
            *ok = 1 as ::core::ffi::c_int;
        }
    }
    return p;
}
#[export_name = "honest_osqp_SuiteSparse_free"]
pub unsafe extern "C" fn SuiteSparse_free(
    mut p: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    if !p.is_null() {
        SuiteSparse_config
            .free_func
            .expect("non-null function pointer")(p);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[export_name = "honest_osqp_SuiteSparse_tic"]
pub unsafe extern "C" fn SuiteSparse_tic(mut tic: *mut OSQPFloat) {
    *tic.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int as OSQPFloat;
    *tic.offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int as OSQPFloat;
}
#[export_name = "honest_osqp_SuiteSparse_toc"]
pub unsafe extern "C" fn SuiteSparse_toc(mut tic: *mut OSQPFloat) -> OSQPFloat {
    let mut toc: [OSQPFloat; 2] = [0.; 2];
    SuiteSparse_tic(&raw mut toc as *mut OSQPFloat);
    return toc[0 as ::core::ffi::c_int as usize] - *tic.offset(0 as ::core::ffi::c_int as isize)
        + 1e-9f64
            * (toc[1 as ::core::ffi::c_int as usize]
                - *tic.offset(1 as ::core::ffi::c_int as isize));
}
#[export_name = "honest_osqp_SuiteSparse_time"]
pub unsafe extern "C" fn SuiteSparse_time() -> OSQPFloat {
    let mut toc: [OSQPFloat; 2] = [0.; 2];
    SuiteSparse_tic(&raw mut toc as *mut OSQPFloat);
    return toc[0 as ::core::ffi::c_int as usize] + 1e-9f64 * toc[1 as ::core::ffi::c_int as usize];
}
#[export_name = "honest_osqp_SuiteSparse_version"]
pub unsafe extern "C" fn SuiteSparse_version(
    mut version: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !version.is_null() {
        *version.offset(0 as ::core::ffi::c_int as isize) = SUITESPARSE_MAIN_VERSION;
        *version.offset(1 as ::core::ffi::c_int as isize) = SUITESPARSE_SUB_VERSION;
        *version.offset(2 as ::core::ffi::c_int as isize) = SUITESPARSE_SUBSUB_VERSION;
    }
    return 4 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int + 5 as ::core::ffi::c_int;
}
#[export_name = "honest_osqp_SuiteSparse_hypot"]
pub unsafe extern "C" fn SuiteSparse_hypot(mut x: OSQPFloat, mut y: OSQPFloat) -> OSQPFloat {
    let mut s: OSQPFloat = 0.;
    let mut r: OSQPFloat = 0.;
    x = fabs(x as ::core::ffi::c_double) as OSQPFloat;
    y = fabs(y as ::core::ffi::c_double) as OSQPFloat;
    if x >= y {
        if x + y == x {
            s = x;
        } else {
            r = y / x;
            s = (x as ::core::ffi::c_double
                * sqrt(1.0f64 + r as ::core::ffi::c_double * r as ::core::ffi::c_double))
                as OSQPFloat;
        }
    } else if y + x == y {
        s = y;
    } else {
        r = x / y;
        s = (y as ::core::ffi::c_double
            * sqrt(1.0f64 + r as ::core::ffi::c_double * r as ::core::ffi::c_double))
            as OSQPFloat;
    }
    return s;
}
#[export_name = "honest_osqp_SuiteSparse_divcomplex"]
pub unsafe extern "C" fn SuiteSparse_divcomplex(
    mut ar: OSQPFloat,
    mut ai: OSQPFloat,
    mut br: OSQPFloat,
    mut bi: OSQPFloat,
    mut cr: *mut OSQPFloat,
    mut ci: *mut OSQPFloat,
) -> ::core::ffi::c_int {
    let mut tr: OSQPFloat = 0.;
    let mut ti: OSQPFloat = 0.;
    let mut r: OSQPFloat = 0.;
    let mut den: OSQPFloat = 0.;
    if fabs(br as ::core::ffi::c_double) >= fabs(bi as ::core::ffi::c_double) {
        r = bi / br;
        den = br + r * bi;
        tr = (ar + ai * r) / den;
        ti = (ai - ar * r) / den;
    } else {
        r = br / bi;
        den = r * br + bi;
        tr = (ar * r + ai) / den;
        ti = (ai * r - ar) / den;
    }
    *cr = tr;
    *ci = ti;
    return (den == 0.0f64) as ::core::ffi::c_int;
}
