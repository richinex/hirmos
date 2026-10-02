//! Convert R's 32-bit BLAS integer ABI to the existing translated LAPACK closure.
use core::ffi::c_long;
extern "C" {
    #[link_name = "dsyevd_closure_f2c_daxpy"]
    fn axpy(
        n: *mut c_long,
        a: *mut f64,
        x: *mut f64,
        ix: *mut c_long,
        y: *mut f64,
        iy: *mut c_long,
    ) -> i32;
    #[link_name = "dgelsd_closure_f2c_dcopy"]
    fn copy(n: *mut c_long, x: *mut f64, ix: *mut c_long, y: *mut f64, iy: *mut c_long) -> i32;
    #[link_name = "dgelsd_closure_f2c_dscal"]
    fn scale(n: *mut c_long, a: *mut f64, x: *mut f64, ix: *mut c_long) -> i32;
    #[link_name = "dgelsd_closure_f2c_idamax"]
    fn maximum(n: *mut c_long, x: *mut f64, ix: *mut c_long) -> c_long;
}
pub unsafe fn daxpy_(
    n: *const i32,
    a: *const f64,
    x: *const f64,
    ix: *const i32,
    y: *mut f64,
    iy: *const i32,
) {
    let (mut n, mut ix, mut iy, mut a) = (*n as c_long, *ix as c_long, *iy as c_long, *a);
    axpy(&mut n, &mut a, x.cast_mut(), &mut ix, y, &mut iy);
}
pub unsafe fn dcopy_(n: *const i32, x: *const f64, ix: *const i32, y: *mut f64, iy: *const i32) {
    let (mut n, mut ix, mut iy) = (*n as c_long, *ix as c_long, *iy as c_long);
    copy(&mut n, x.cast_mut(), &mut ix, y, &mut iy);
}
pub unsafe fn dscal_(n: *const i32, a: *const f64, x: *mut f64, ix: *const i32) {
    let (mut n, mut ix, mut a) = (*n as c_long, *ix as c_long, *a);
    scale(&mut n, &mut a, x, &mut ix);
}
pub unsafe fn idamax_(n: *const i32, x: *const f64, ix: *const i32) -> i32 {
    let (mut n, mut ix) = (*n as c_long, *ix as c_long);
    maximum(&mut n, x.cast_mut(), &mut ix) as i32
}
