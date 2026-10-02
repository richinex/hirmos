pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type OSQPFloat = ::core::ffi::c_double;
pub const EMPTY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_INFO: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const AMD_STATUS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_N: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AMD_NZ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AMD_SYMMETRY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const AMD_NZDIAG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const AMD_NZ_A_PLUS_AT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const AMD_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "honest_osqp_amd_aat"]
pub unsafe extern "C" fn amd_aat(
    mut n: ::core::ffi::c_int,
    mut Ap: *const ::core::ffi::c_int,
    mut Ai: *const ::core::ffi::c_int,
    mut Len: *mut ::core::ffi::c_int,
    mut Tp: *mut ::core::ffi::c_int,
    mut Info: *mut OSQPFloat,
) -> size_t {
    let mut p1: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut pj: ::core::ffi::c_int = 0;
    let mut pj2: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut nzdiag: ::core::ffi::c_int = 0;
    let mut nzboth: ::core::ffi::c_int = 0;
    let mut nz: ::core::ffi::c_int = 0;
    let mut sym: OSQPFloat = 0.;
    let mut nzaat: size_t = 0;
    if !Info.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < AMD_INFO {
            *Info.offset(i as isize) = EMPTY as OSQPFloat;
            i += 1;
        }
        *Info.offset(AMD_STATUS as isize) = AMD_OK as OSQPFloat;
    }
    k = 0 as ::core::ffi::c_int;
    while k < n {
        *Len.offset(k as isize) = 0 as ::core::ffi::c_int;
        k += 1;
    }
    nzdiag = 0 as ::core::ffi::c_int;
    nzboth = 0 as ::core::ffi::c_int;
    nz = *Ap.offset(n as isize);
    k = 0 as ::core::ffi::c_int;
    while k < n {
        p1 = *Ap.offset(k as isize);
        p2 = *Ap.offset((k + 1 as ::core::ffi::c_int) as isize);
        p = p1;
        while p < p2 {
            j = *Ai.offset(p as isize);
            if j < k {
                let ref mut fresh0 = *Len.offset(j as isize);
                *fresh0 += 1;
                let ref mut fresh1 = *Len.offset(k as isize);
                *fresh1 += 1;
                p += 1;
                pj2 = *Ap.offset((j + 1 as ::core::ffi::c_int) as isize);
                pj = *Tp.offset(j as isize);
                while pj < pj2 {
                    i = *Ai.offset(pj as isize);
                    if i < k {
                        let ref mut fresh2 = *Len.offset(i as isize);
                        *fresh2 += 1;
                        let ref mut fresh3 = *Len.offset(j as isize);
                        *fresh3 += 1;
                        pj += 1;
                    } else if i == k {
                        pj += 1;
                        nzboth += 1;
                        break;
                    } else {
                        break;
                    }
                }
                *Tp.offset(j as isize) = pj;
            } else if j == k {
                p += 1;
                nzdiag += 1;
                break;
            } else {
                break;
            }
        }
        *Tp.offset(k as isize) = p;
        k += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < n {
        pj = *Tp.offset(j as isize);
        while pj < *Ap.offset((j + 1 as ::core::ffi::c_int) as isize) {
            i = *Ai.offset(pj as isize);
            let ref mut fresh4 = *Len.offset(i as isize);
            *fresh4 += 1;
            let ref mut fresh5 = *Len.offset(j as isize);
            *fresh5 += 1;
            pj += 1;
        }
        j += 1;
    }
    if nz == nzdiag {
        sym = 1 as ::core::ffi::c_int as OSQPFloat;
    } else {
        sym =
            2 as ::core::ffi::c_int as OSQPFloat * nzboth as OSQPFloat / (nz - nzdiag) as OSQPFloat;
    }
    nzaat = 0 as size_t;
    k = 0 as ::core::ffi::c_int;
    while k < n {
        nzaat = (nzaat as u64)
            .wrapping_add(*Len.offset(k as isize) as u64) as size_t
            as size_t;
        k += 1;
    }
    if !Info.is_null() {
        *Info.offset(AMD_STATUS as isize) = AMD_OK as OSQPFloat;
        *Info.offset(AMD_N as isize) = n as OSQPFloat;
        *Info.offset(AMD_NZ as isize) = nz as OSQPFloat;
        *Info.offset(AMD_SYMMETRY as isize) = sym;
        *Info.offset(AMD_NZDIAG as isize) = nzdiag as OSQPFloat;
        *Info.offset(AMD_NZ_A_PLUS_AT as isize) = nzaat as OSQPFloat;
    }
    return nzaat;
}
