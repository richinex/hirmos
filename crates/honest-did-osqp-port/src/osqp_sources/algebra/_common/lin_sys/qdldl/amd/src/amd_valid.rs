pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_INVALID: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const AMD_OK_BUT_JUMBLED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "honest_osqp_amd_valid"]
pub unsafe extern "C" fn amd_valid(
    mut n_row: ::core::ffi::c_int,
    mut n_col: ::core::ffi::c_int,
    mut Ap: *const ::core::ffi::c_int,
    mut Ai: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nz: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p1: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    let mut ilast: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_int = AMD_OK;
    if n_row < 0 as ::core::ffi::c_int
        || n_col < 0 as ::core::ffi::c_int
        || Ap.is_null()
        || Ai.is_null()
    {
        return -(2 as ::core::ffi::c_int);
    }
    nz = *Ap.offset(n_col as isize);
    if *Ap.offset(0 as ::core::ffi::c_int as isize) != 0 as ::core::ffi::c_int
        || nz < 0 as ::core::ffi::c_int
    {
        return -(2 as ::core::ffi::c_int);
    }
    j = 0 as ::core::ffi::c_int;
    while j < n_col {
        p1 = *Ap.offset(j as isize);
        p2 = *Ap.offset((j + 1 as ::core::ffi::c_int) as isize);
        if p1 > p2 {
            return -(2 as ::core::ffi::c_int);
        }
        ilast = EMPTY;
        p = p1;
        while p < p2 {
            i = *Ai.offset(p as isize);
            if i < 0 as ::core::ffi::c_int || i >= n_row {
                return -(2 as ::core::ffi::c_int);
            }
            if i <= ilast {
                result = AMD_OK_BUT_JUMBLED;
            }
            ilast = i;
            p += 1;
        }
        j += 1;
    }
    return result;
}
