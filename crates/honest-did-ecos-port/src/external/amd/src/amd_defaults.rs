pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_CONTROL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const AMD_DENSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_DEFAULT_DENSE: ::core::ffi::c_double = 10.0f64;
pub const AMD_DEFAULT_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn amd_l_defaults(mut Control: *mut ::core::ffi::c_double) {
    let mut i: i64 = 0;
    if !Control.is_null() {
        i = 0 as i64;
        while i < AMD_CONTROL as i64 {
            *Control.offset(i as isize) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        *Control.offset(AMD_DENSE as isize) = AMD_DEFAULT_DENSE;
        *Control.offset(AMD_AGGRESSIVE as isize) = AMD_DEFAULT_AGGRESSIVE as ::core::ffi::c_double;
    }
}
