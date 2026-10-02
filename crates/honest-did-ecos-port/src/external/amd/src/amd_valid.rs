pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_INVALID: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const AMD_OK_BUT_JUMBLED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn amd_l_valid(
    mut n_row: i64,
    mut n_col: i64,
    mut Ap: *const i64,
    mut Ai: *const i64,
) -> i64 {
    let mut nz: i64 = 0;
    let mut j: i64 = 0;
    let mut p1: i64 = 0;
    let mut p2: i64 = 0;
    let mut ilast: i64 = 0;
    let mut i: i64 = 0;
    let mut p: i64 = 0;
    let mut result: i64 = AMD_OK as i64;
    if n_row < 0 as i64
        || n_col < 0 as i64
        || Ap.is_null()
        || Ai.is_null()
    {
        return -(2 as ::core::ffi::c_int) as i64;
    }
    nz = *Ap.offset(n_col as isize);
    if *Ap.offset(0 as ::core::ffi::c_int as isize) != 0 as i64
        || nz < 0 as i64
    {
        return -(2 as ::core::ffi::c_int) as i64;
    }
    j = 0 as i64;
    while j < n_col {
        p1 = *Ap.offset(j as isize);
        p2 = *Ap.offset((j + 1 as i64) as isize);
        if p1 > p2 {
            return -(2 as ::core::ffi::c_int) as i64;
        }
        ilast = EMPTY as i64;
        p = p1;
        while p < p2 {
            i = *Ai.offset(p as isize);
            if i < 0 as i64 || i >= n_row {
                return -(2 as ::core::ffi::c_int) as i64;
            }
            if i <= ilast {
                result = AMD_OK_BUT_JUMBLED as i64;
            }
            ilast = i;
            p += 1;
        }
        j += 1;
    }
    return result;
}
