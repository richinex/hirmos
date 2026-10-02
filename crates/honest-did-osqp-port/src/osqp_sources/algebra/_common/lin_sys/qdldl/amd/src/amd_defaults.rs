pub type OSQPFloat = ::core::ffi::c_double;
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_CONTROL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const AMD_DENSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_DEFAULT_DENSE: ::core::ffi::c_double = 10.0f64;
pub const AMD_DEFAULT_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "honest_osqp_amd_defaults"]
pub unsafe extern "C" fn amd_defaults(mut Control: *mut OSQPFloat) {
    let mut i: ::core::ffi::c_int = 0;
    if !Control.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < AMD_CONTROL {
            *Control.offset(i as isize) = 0 as ::core::ffi::c_int as OSQPFloat;
            i += 1;
        }
        *Control.offset(AMD_DENSE as isize) = AMD_DEFAULT_DENSE as OSQPFloat;
        *Control.offset(AMD_AGGRESSIVE as isize) = AMD_DEFAULT_AGGRESSIVE as OSQPFloat;
    }
}
