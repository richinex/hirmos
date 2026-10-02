//! Local allocation, C-string operations and IEEE scalar helpers.
#[path = "../../../../honest-did-ecos-port/src/runtime.rs"]
mod allocation;
pub use allocation::{calloc, fabs, free, malloc, realloc, sqrt};
use core::ffi::c_char;
#[cfg(target_arch="wasm32")]
pub fn unavailable<T>(operation:&str)->T {
    panic!("Unsupported native solver operation: {operation}")
}
pub unsafe extern "C" fn strlen(s: *const c_char) -> usize {
    let mut n = 0;
    while *s.add(n) != 0 {
        n += 1;
    }
    n
}
pub unsafe extern "C" fn strcpy(d: *mut c_char, s: *const c_char) -> *mut c_char {
    core::ptr::copy(s, d, strlen(s) + 1);
    d
}
pub unsafe extern "C" fn strncpy(d: *mut c_char, s: *const c_char, n: usize) -> *mut c_char {
    let mut ended = false;
    for i in 0..n {
        let v = if ended { 0 } else { *s.add(i) };
        ended |= v == 0;
        *d.add(i) = v;
    }
    d
}
pub unsafe extern "C" fn strcmp(a: *const c_char, b: *const c_char) -> i32 {
    let mut i = 0;
    loop {
        let (a, b) = (*a.add(i) as u8, *b.add(i) as u8);
        if a != b || a == 0 {
            return a as i32 - b as i32;
        }
        i += 1;
    }
}
pub unsafe extern "C" fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> i32 {
    for i in 0..n {
        let (a, b) = (*a.add(i) as u8, *b.add(i) as u8);
        if a != b || a == 0 {
            return a as i32 - b as i32;
        }
    }
    0
}
pub unsafe extern "C" fn strcat(d: *mut c_char, s: *const c_char) -> *mut c_char {
    strcpy(d.add(strlen(d)), s);
    d
}
pub unsafe extern "C" fn strdup(s: *const c_char) -> *mut c_char {
    let out: *mut c_char = malloc(strlen(s) + 1).cast();
    if !out.is_null() {
        strcpy(out, s);
    }
    out
}
pub unsafe extern "C" fn strchr(s: *const c_char, c: i32) -> *mut c_char {
    let mut p = s;
    loop {
        if *p as u8 == c as u8 {
            return p.cast_mut();
        }
        if *p == 0 {
            return core::ptr::null_mut();
        }
        p = p.add(1);
    }
}
pub unsafe extern "C" fn strrchr(s: *const c_char, c: i32) -> *mut c_char {
    let mut p = s;
    let mut last = core::ptr::null_mut();
    loop {
        if *p as u8 == c as u8 {
            last = p.cast_mut();
        }
        if *p == 0 {
            return last;
        }
        p = p.add(1);
    }
}
pub unsafe extern "C" fn modf(x: f64, integer: *mut f64) -> f64 {
    let (f, i) = libm::modf(x);
    *integer = i;
    f
}
pub unsafe extern "C" fn frexp(x: f64, exponent: *mut i32) -> f64 {
    let (f, e) = libm::frexp(x);
    *exponent = e;
    f
}
