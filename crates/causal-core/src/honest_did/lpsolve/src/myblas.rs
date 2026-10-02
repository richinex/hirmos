use crate::honest_did::lpsolve::blas_bridge::{daxpy_,dcopy_,dscal_,idamax_};
#[export_name="honest_lpsolve_mustinitBLAS"]
pub static mut mustinitBLAS: ::core::ffi::c_uchar = FALSE as ::core::ffi::c_uchar;
#[export_name="honest_lpsolve_hBLAS"]
pub static mut hBLAS: *mut ::core::ffi::c_void = NULL;
#[export_name="honest_lpsolve_init_BLAS"]
pub unsafe extern "C" fn init_BLAS() {
    if mustinitBLAS != 0 {
        load_BLAS(::core::ptr::null_mut::<::core::ffi::c_char>());
        mustinitBLAS = FALSE as ::core::ffi::c_uchar;
    }
}
#[export_name="honest_lpsolve_is_nativeBLAS"]
pub unsafe extern "C" fn is_nativeBLAS() -> ::core::ffi::c_uchar {
    return 1 as ::core::ffi::c_uchar;
}
#[export_name="honest_lpsolve_load_BLAS"]
pub unsafe extern "C" fn load_BLAS(mut libname: *mut ::core::ffi::c_char) -> ::core::ffi::c_uchar {
    let mut result: ::core::ffi::c_uchar = TRUE as ::core::ffi::c_uchar;
    return result;
}
#[export_name="honest_lpsolve_unload_BLAS"]
pub unsafe extern "C" fn unload_BLAS() -> ::core::ffi::c_uchar {
    return load_BLAS(::core::ptr::null_mut::<::core::ffi::c_char>());
}
#[export_name="honest_lpsolve_lps_daxpy"]
pub unsafe extern "C" fn lps_daxpy(
    mut n: ::core::ffi::c_int,
    mut da: ::core::ffi::c_double,
    mut dx: *mut ::core::ffi::c_double,
    mut incx: ::core::ffi::c_int,
    mut dy: *mut ::core::ffi::c_double,
    mut incy: ::core::ffi::c_int,
) {
    dx = dx.offset(1);
    dy = dy.offset(1);
    daxpy_(
        &raw mut n,
        &raw mut da,
        dx,
        &raw mut incx,
        dy,
        &raw mut incy,
    );
}
#[export_name="honest_lpsolve_lps_dcopy"]
pub unsafe extern "C" fn lps_dcopy(
    mut n: ::core::ffi::c_int,
    mut dx: *mut ::core::ffi::c_double,
    mut incx: ::core::ffi::c_int,
    mut dy: *mut ::core::ffi::c_double,
    mut incy: ::core::ffi::c_int,
) {
    dx = dx.offset(1);
    dy = dy.offset(1);
    dcopy_(&raw mut n, dx, &raw mut incx, dy, &raw mut incy);
}
#[export_name="honest_lpsolve_lps_dscal"]
pub unsafe extern "C" fn lps_dscal(
    mut n: ::core::ffi::c_int,
    mut da: ::core::ffi::c_double,
    mut dx: *mut ::core::ffi::c_double,
    mut incx: ::core::ffi::c_int,
) {
    dx = dx.offset(1);
    dscal_(&raw mut n, &raw mut da, dx, &raw mut incx);
}
#[export_name="honest_lpsolve_lps_idamax"]
pub unsafe extern "C" fn lps_idamax(
    mut n: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
    mut is: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    x = x.offset(1);
    return idamax_(&raw mut n, x, &raw mut is);
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
