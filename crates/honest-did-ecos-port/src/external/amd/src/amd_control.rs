extern "C" {
    static amd_printf:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ::core::ffi::c_int>;
}
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_DENSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_DEFAULT_DENSE: ::core::ffi::c_double = 10.0f64;
pub const AMD_DEFAULT_AGGRESSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn amd_l_control(mut Control: *mut ::core::ffi::c_double) {
    let mut alpha: ::core::ffi::c_double = 0.;
    let mut aggressive: i64 = 0;
    if !Control.is_null() {
        alpha = *Control.offset(AMD_DENSE as isize);
        aggressive = (*Control.offset(AMD_AGGRESSIVE as isize)
            != 0 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_int as i64;
    } else {
        alpha = AMD_DEFAULT_DENSE;
        aggressive = AMD_DEFAULT_AGGRESSIVE as i64;
    }
    if amd_printf.is_some() {
        amd_printf
            .expect(
                "non-null function pointer",
            )(
            b"\nAMD version %d.%d.%d, %s: approximate minimum degree ordering\n    dense row parameter: %g\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            b"Jun 20, 2012\0" as *const u8 as *const ::core::ffi::c_char,
            alpha,
        );
    }
    if alpha < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    no rows treated as dense\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if amd_printf.is_some() {
        amd_printf
            .expect(
                "non-null function pointer",
            )(
            b"    (rows with more than max (%g * sqrt (n), 16) entries are\n    considered \"dense\", and placed last in output permutation)\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            alpha,
        );
    }
    if aggressive != 0 {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    aggressive absorption:  yes\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if amd_printf.is_some() {
        amd_printf.expect("non-null function pointer")(
            b"    aggressive absorption:  no\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if amd_printf.is_some() {
        amd_printf.expect("non-null function pointer")(
            b"    size of AMD integer: %d\n\n\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<i64>(),
        );
    }
}
