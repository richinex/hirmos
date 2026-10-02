pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn amd_l_preprocess(
    mut n: i64,
    mut Ap: *const i64,
    mut Ai: *const i64,
    mut Rp: *mut i64,
    mut Ri: *mut i64,
    mut W: *mut i64,
    mut Flag: *mut i64,
) {
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut p: i64 = 0;
    let mut p2: i64 = 0;
    i = 0 as i64;
    while i < n {
        *W.offset(i as isize) = 0 as i64;
        *Flag.offset(i as isize) = EMPTY as i64;
        i += 1;
    }
    j = 0 as i64;
    while j < n {
        p2 = *Ap.offset((j + 1 as i64) as isize);
        p = *Ap.offset(j as isize);
        while p < p2 {
            i = *Ai.offset(p as isize);
            if *Flag.offset(i as isize) != j {
                let ref mut fresh0 = *W.offset(i as isize);
                *fresh0 += 1;
                *Flag.offset(i as isize) = j;
            }
            p += 1;
        }
        j += 1;
    }
    *Rp.offset(0 as ::core::ffi::c_int as isize) = 0 as i64;
    i = 0 as i64;
    while i < n {
        *Rp.offset((i + 1 as i64) as isize) =
            *Rp.offset(i as isize) + *W.offset(i as isize);
        i += 1;
    }
    i = 0 as i64;
    while i < n {
        *W.offset(i as isize) = *Rp.offset(i as isize);
        *Flag.offset(i as isize) = EMPTY as i64;
        i += 1;
    }
    j = 0 as i64;
    while j < n {
        p2 = *Ap.offset((j + 1 as i64) as isize);
        p = *Ap.offset(j as isize);
        while p < p2 {
            i = *Ai.offset(p as isize);
            if *Flag.offset(i as isize) != j {
                let ref mut fresh1 = *W.offset(i as isize);
                let fresh2 = *fresh1;
                *fresh1 = *fresh1 + 1;
                *Ri.offset(fresh2 as isize) = j;
                *Flag.offset(i as isize) = j;
            }
            p += 1;
        }
        j += 1;
    }
}
