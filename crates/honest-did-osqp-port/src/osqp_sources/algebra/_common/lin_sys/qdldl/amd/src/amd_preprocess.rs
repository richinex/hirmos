pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[export_name = "honest_osqp_amd_preprocess"]
pub unsafe extern "C" fn amd_preprocess(
    mut n: ::core::ffi::c_int,
    mut Ap: *const ::core::ffi::c_int,
    mut Ai: *const ::core::ffi::c_int,
    mut Rp: *mut ::core::ffi::c_int,
    mut Ri: *mut ::core::ffi::c_int,
    mut W: *mut ::core::ffi::c_int,
    mut Flag: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *W.offset(i as isize) = 0 as ::core::ffi::c_int;
        *Flag.offset(i as isize) = EMPTY;
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < n {
        p2 = *Ap.offset((j + 1 as ::core::ffi::c_int) as isize);
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
    *Rp.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *Rp.offset((i + 1 as ::core::ffi::c_int) as isize) =
            *Rp.offset(i as isize) + *W.offset(i as isize);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *W.offset(i as isize) = *Rp.offset(i as isize);
        *Flag.offset(i as isize) = EMPTY;
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < n {
        p2 = *Ap.offset((j + 1 as ::core::ffi::c_int) as isize);
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
