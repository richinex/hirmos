extern "C" {
    static amd_printf:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ::core::ffi::c_int>;
}
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_STATUS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_N: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_LNZ: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const AMD_NDIV: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AMD_NMULTSUBS_LDL: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const AMD_NMULTSUBS_LU: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const AMD_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_OUT_OF_MEMORY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const AMD_INVALID: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const AMD_OK_BUT_JUMBLED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn amd_l_info(mut Info: *mut ::core::ffi::c_double) {
    let mut n: ::core::ffi::c_double = 0.;
    let mut ndiv: ::core::ffi::c_double = 0.;
    let mut nmultsubs_ldl: ::core::ffi::c_double = 0.;
    let mut nmultsubs_lu: ::core::ffi::c_double = 0.;
    let mut lnz: ::core::ffi::c_double = 0.;
    let mut lnzd: ::core::ffi::c_double = 0.;
    if amd_printf.is_some() {
        amd_printf.expect("non-null function pointer")(
            b"\nAMD version %d.%d.%d, %s, results:\n\0" as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            b"Jun 20, 2012\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if Info.is_null() {
        return;
    }
    n = *Info.offset(AMD_N as isize);
    ndiv = *Info.offset(AMD_NDIV as isize);
    nmultsubs_ldl = *Info.offset(AMD_NMULTSUBS_LDL as isize);
    nmultsubs_lu = *Info.offset(AMD_NMULTSUBS_LU as isize);
    lnz = *Info.offset(AMD_LNZ as isize);
    lnzd = if n >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && lnz >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        n + lnz
    } else {
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_double
    };
    if amd_printf.is_some() {
        amd_printf.expect("non-null function pointer")(
            b"    status: \0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *Info.offset(AMD_STATUS as isize) == AMD_OK as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"OK\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if *Info.offset(AMD_STATUS as isize) == AMD_OUT_OF_MEMORY as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"out of memory\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if *Info.offset(AMD_STATUS as isize) == AMD_INVALID as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"invalid matrix\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if *Info.offset(AMD_STATUS as isize) == AMD_OK_BUT_JUMBLED as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"OK, but jumbled\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if amd_printf.is_some() {
        amd_printf.expect("non-null function pointer")(
            b"unknown\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if n >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    n, dimension of A:                                  %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                n,
            );
        }
    }
    if *Info.offset(2 as ::core::ffi::c_int as isize)
        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    nz, number of nonzeros in A:                        %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *Info.offset(2 as ::core::ffi::c_int as isize),
            );
        }
    }
    if *Info.offset(3 as ::core::ffi::c_int as isize)
        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    symmetry of A:                                      %.4f\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *Info.offset(3 as ::core::ffi::c_int as isize),
            );
        }
    }
    if *Info.offset(4 as ::core::ffi::c_int as isize)
        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    number of nonzeros on diagonal:                     %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *Info.offset(4 as ::core::ffi::c_int as isize),
            );
        }
    }
    if *Info.offset(5 as ::core::ffi::c_int as isize)
        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    nonzeros in pattern of A+A' (excl. diagonal):       %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *Info.offset(5 as ::core::ffi::c_int as isize),
            );
        }
    }
    if *Info.offset(6 as ::core::ffi::c_int as isize)
        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    # dense rows/columns of A+A':                       %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *Info.offset(6 as ::core::ffi::c_int as isize),
            );
        }
    }
    if *Info.offset(7 as ::core::ffi::c_int as isize)
        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    memory used, in bytes:                              %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *Info.offset(7 as ::core::ffi::c_int as isize),
            );
        }
    }
    if *Info.offset(8 as ::core::ffi::c_int as isize)
        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    # of memory compactions:                            %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *Info.offset(8 as ::core::ffi::c_int as isize),
            );
        }
    }
    if amd_printf.is_some() {
        amd_printf
            .expect(
                "non-null function pointer",
            )(
            b"\n    The following approximate statistics are for a subsequent\n    factorization of A(P,P) + A(P,P)'.  They are slight upper\n    bounds if there are no dense rows/columns in A+A', and become\n    looser if dense rows/columns exist.\n\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if lnz >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    nonzeros in L (excluding diagonal):                 %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                lnz,
            );
        }
    }
    if lnzd >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    nonzeros in L (including diagonal):                 %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                lnzd,
            );
        }
    }
    if ndiv >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    # divide operations for LDL' or LU:                 %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                ndiv,
            );
        }
    }
    if nmultsubs_ldl >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    # multiply-subtract operations for LDL':            %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                nmultsubs_ldl,
            );
        }
    }
    if nmultsubs_lu >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    # multiply-subtract operations for LU:              %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                nmultsubs_lu,
            );
        }
    }
    if *Info.offset(13 as ::core::ffi::c_int as isize)
        >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if amd_printf.is_some() {
            amd_printf.expect("non-null function pointer")(
                b"    max nz. in any column of L (incl. diagonal):        %.20g\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *Info.offset(13 as ::core::ffi::c_int as isize),
            );
        }
    }
    if n >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && ndiv >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && nmultsubs_ldl >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && nmultsubs_lu >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if amd_printf.is_some() {
            amd_printf
                .expect(
                    "non-null function pointer",
                )(
                b"\n    chol flop count for real A, sqrt counted as 1 flop: %.20g\n    LDL' flop count for real A:                         %.20g\n    LDL' flop count for complex A:                      %.20g\n    LU flop count for real A (with no pivoting):        %.20g\n    LU flop count for complex A (with no pivoting):     %.20g\n\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                n + ndiv
                    + 2 as ::core::ffi::c_int as ::core::ffi::c_double * nmultsubs_ldl,
                ndiv + 2 as ::core::ffi::c_int as ::core::ffi::c_double * nmultsubs_ldl,
                9 as ::core::ffi::c_int as ::core::ffi::c_double * ndiv
                    + 8 as ::core::ffi::c_int as ::core::ffi::c_double * nmultsubs_ldl,
                ndiv + 2 as ::core::ffi::c_int as ::core::ffi::c_double * nmultsubs_lu,
                9 as ::core::ffi::c_int as ::core::ffi::c_double * ndiv
                    + 8 as ::core::ffi::c_int as ::core::ffi::c_double * nmultsubs_lu,
            );
        }
    }
}
