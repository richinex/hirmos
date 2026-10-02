pub type osqp_error_type = ::core::ffi::c_uint;
pub const OSQP_LAST_ERROR_PLACE: osqp_error_type = 12;
pub const OSQP_FUNC_NOT_IMPLEMENTED: osqp_error_type = 11;
pub const OSQP_DATA_NOT_INITIALIZED: osqp_error_type = 10;
pub const OSQP_CODEGEN_DEFINES_ERROR: osqp_error_type = 9;
pub const OSQP_FOPEN_ERROR: osqp_error_type = 8;
pub const OSQP_ALGEBRA_LOAD_ERROR: osqp_error_type = 7;
pub const OSQP_WORKSPACE_NOT_INIT_ERROR: osqp_error_type = 6;
pub const OSQP_MEM_ALLOC_ERROR: osqp_error_type = 5;
pub const OSQP_NONCVX_ERROR: osqp_error_type = 4;
pub const OSQP_LINSYS_SOLVER_INIT_ERROR: osqp_error_type = 3;
pub const OSQP_SETTINGS_VALIDATION_ERROR: osqp_error_type = 2;
pub const OSQP_DATA_VALIDATION_ERROR: osqp_error_type = 1;
pub const OSQP_NO_ERROR: osqp_error_type = 0;
pub type OSQPInt = ::core::ffi::c_int;
#[export_name = "honest_osqp_OSQP_ERROR_MESSAGE"]
pub static mut OSQP_ERROR_MESSAGE: [*const ::core::ffi::c_char; 12] = [
    b"Problem data validation.\0" as *const u8 as *const ::core::ffi::c_char,
    b"Solver settings validation.\0" as *const u8 as *const ::core::ffi::c_char,
    b"Linear system solver initialization.\0" as *const u8 as *const ::core::ffi::c_char,
    b"KKT matrix factorization.\nThe problem seems to be non-convex.\0" as *const u8
        as *const ::core::ffi::c_char,
    b"Memory allocation.\0" as *const u8 as *const ::core::ffi::c_char,
    b"Solver workspace not initialized.\0" as *const u8 as *const ::core::ffi::c_char,
    b"Algebra libraries not loaded.\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unable to open file for writing.\0" as *const u8 as *const ::core::ffi::c_char,
    b"Invalid defines for codegen\0" as *const u8 as *const ::core::ffi::c_char,
    b"Vector/matrix not initialized.\0" as *const u8 as *const ::core::ffi::c_char,
    b"Function not implemented.\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unknown error code.\0" as *const u8 as *const ::core::ffi::c_char,
];
#[export_name = "honest_osqp__osqp_error"]
pub unsafe extern "C" fn _osqp_error(
    mut error_code: osqp_error_type,
    mut function_name: *const ::core::ffi::c_char,
) -> OSQPInt {
    error_code as ::core::ffi::c_uint != OSQP_NO_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint;
    return error_code as OSQPInt;
}
#[export_name = "honest_osqp__osqp_error_line"]
pub unsafe extern "C" fn _osqp_error_line(
    mut error_code: osqp_error_type,
    mut function_name: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut line_number: OSQPInt,
) -> OSQPInt {
    error_code as ::core::ffi::c_uint != OSQP_NO_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint;
    return error_code as OSQPInt;
}
