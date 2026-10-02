extern "C" {
    #[link_name = "honest_osqp_SuiteSparse_config"]
    static SuiteSparse_config: SuiteSparse_config_struct;
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
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_DENSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_DEFAULT_DENSE: ::core::ffi::c_double = 10.0f64;
pub const AMD_DEFAULT_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "honest_osqp_amd_control"]
pub unsafe extern "C" fn amd_control(mut Control: *mut OSQPFloat) {
    let mut alpha: OSQPFloat = 0.;
    let mut aggressive: ::core::ffi::c_int = 0;
    if !Control.is_null() {
        alpha = *Control.offset(AMD_DENSE as isize);
        aggressive = (*Control.offset(AMD_AGGRESSIVE as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_int;
    } else {
        alpha = AMD_DEFAULT_DENSE as OSQPFloat;
        aggressive = AMD_DEFAULT_AGGRESSIVE;
    }
    if SuiteSparse_config.printf_func.is_some() {
        SuiteSparse_config
            .printf_func
            .expect(
                "non-null function pointer",
            )(
            b"\nAMD version %d.%d.%d, %s: approximate minimum degree ordering\n    dense row parameter: %g\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            b"May 4, 2016\0" as *const u8 as *const ::core::ffi::c_char,
            alpha,
        );
    }
    if alpha < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if SuiteSparse_config.printf_func.is_some() {
            SuiteSparse_config
                .printf_func
                .expect("non-null function pointer")(
                b"    no rows treated as dense\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if SuiteSparse_config.printf_func.is_some() {
        SuiteSparse_config
            .printf_func
            .expect(
                "non-null function pointer",
            )(
            b"    (rows with more than max (%g * sqrt (n), 16) entries are\n    considered \"dense\", and placed last in output permutation)\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            alpha,
        );
    }
    if aggressive != 0 {
        if SuiteSparse_config.printf_func.is_some() {
            SuiteSparse_config
                .printf_func
                .expect("non-null function pointer")(
                b"    aggressive absorption:  yes\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if SuiteSparse_config.printf_func.is_some() {
        SuiteSparse_config
            .printf_func
            .expect("non-null function pointer")(
            b"    aggressive absorption:  no\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if SuiteSparse_config.printf_func.is_some() {
        SuiteSparse_config
            .printf_func
            .expect("non-null function pointer")(
            b"    size of AMD integer: %d\n\n\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<::core::ffi::c_int>(),
        );
    }
}
