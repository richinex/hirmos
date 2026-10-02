//! Local runtime for the translated solver. Every allocation belongs to this
//! adapter, so no platform libc allocation or browser imports are required.
use core::ffi::c_void;
use std::alloc::{alloc, dealloc, Layout};
const ALIGN: usize = 16;

pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    let Some(total) = size.max(1).checked_add(ALIGN) else { return core::ptr::null_mut(); };
    let Ok(layout) = Layout::from_size_align(total, ALIGN) else { return core::ptr::null_mut(); };
    let pointer = alloc(layout);
    if pointer.is_null() { return pointer.cast(); }
    pointer.cast::<usize>().write(total);
    pointer.add(ALIGN).cast()
}
pub unsafe extern "C" fn free(pointer: *mut c_void) {
    if pointer.is_null() { return; }
    let base = pointer.cast::<u8>().sub(ALIGN);
    let total = base.cast::<usize>().read();
    dealloc(base, Layout::from_size_align_unchecked(total, ALIGN));
}
pub unsafe extern "C" fn calloc(count: usize, size: usize) -> *mut c_void {
    let Some(total) = count.checked_mul(size) else { return core::ptr::null_mut(); };
    let pointer = malloc(total);
    if !pointer.is_null() { core::ptr::write_bytes(pointer.cast::<u8>(), 0, total); }
    pointer
}
pub unsafe extern "C" fn realloc(pointer: *mut c_void, size: usize) -> *mut c_void {
    if pointer.is_null() { return malloc(size); }
    let old = pointer.cast::<u8>().sub(ALIGN).cast::<usize>().read() - ALIGN;
    let replacement = malloc(size);
    if !replacement.is_null() {
        core::ptr::copy_nonoverlapping(pointer.cast::<u8>(), replacement.cast::<u8>(), old.min(size));
        free(pointer);
    }
    replacement
}
pub unsafe extern "C" fn sqrt(value: f64) -> f64 { value.sqrt() }
pub unsafe extern "C" fn fabs(value: f64) -> f64 { value.abs() }
