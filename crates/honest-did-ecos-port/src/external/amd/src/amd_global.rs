use crate::runtime::{malloc,calloc,free,realloc};
extern "C" {
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[no_mangle]
pub static amd_malloc: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void> =
    unsafe { Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void) };
#[no_mangle]
pub static amd_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> =
    unsafe { Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()) };
#[no_mangle]
pub static amd_realloc: Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
> = unsafe {
    Some(
        realloc
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    )
};
#[no_mangle]
pub static amd_calloc: Option<
    unsafe extern "C" fn(size_t, size_t) -> *mut ::core::ffi::c_void,
> = unsafe { Some(calloc as unsafe extern "C" fn(size_t, size_t) -> *mut ::core::ffi::c_void) };
#[no_mangle]
pub static amd_printf: Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ::core::ffi::c_int,
> = None;
