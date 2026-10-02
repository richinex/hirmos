use crate::runtime::{malloc,calloc,free};
extern "C" {
}
pub type OSQPInt = ::core::ffi::c_int;
pub type OSQPFloat = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPCscMatrix {
    pub m: OSQPInt,
    pub n: OSQPInt,
    pub p: *mut OSQPInt,
    pub i: *mut OSQPInt,
    pub x: *mut OSQPFloat,
    pub nzmax: OSQPInt,
    pub nz: OSQPInt,
    pub owned: OSQPInt,
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub const OSQP_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "honest_osqp_csc_is_eq"]
pub unsafe extern "C" fn csc_is_eq(
    mut A: *mut OSQPCscMatrix,
    mut B: *mut OSQPCscMatrix,
    mut tol: OSQPFloat,
) -> OSQPInt {
    let mut j: OSQPInt = 0;
    let mut i: OSQPInt = 0;
    if (*A).n != (*B).n {
        return 0 as OSQPInt;
    }
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*A).n {
        if *(*A)
            .p
            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            != *(*B)
                .p
                .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        {
            return 0 as OSQPInt;
        }
        i = *(*A).p.offset(j as isize);
        while i < *(*A)
            .p
            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        {
            if *(*A).i.offset(i as isize) != *(*B).i.offset(i as isize)
                || (if *(*A).x.offset(i as isize) - *(*B).x.offset(i as isize)
                    < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    -(*(*A).x.offset(i as isize) as ::core::ffi::c_double
                        - *(*B).x.offset(i as isize) as ::core::ffi::c_double)
                } else {
                    *(*A).x.offset(i as isize) as ::core::ffi::c_double
                        - *(*B).x.offset(i as isize) as ::core::ffi::c_double
                }) > tol
            {
                return 0 as OSQPInt;
            }
            i += 1;
        }
        j += 1;
    }
    return 1 as OSQPInt;
}
unsafe extern "C" fn csc_malloc(mut n: OSQPInt, mut size: OSQPInt) -> *mut ::core::ffi::c_void {
    return malloc((n * size) as size_t);
}
unsafe extern "C" fn csc_calloc(mut n: OSQPInt, mut size: OSQPInt) -> *mut ::core::ffi::c_void {
    return calloc(n as size_t, size as size_t);
}
unsafe extern "C" fn prea_int_vec_copy(mut a: *const OSQPInt, mut b: *mut OSQPInt, mut n: OSQPInt) {
    let mut i: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        *b.offset(i as isize) = *a.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C" fn prea_vec_copy(mut a: *const OSQPFloat, mut b: *mut OSQPFloat, mut n: OSQPInt) {
    let mut i: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        *b.offset(i as isize) = *a.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C" fn float_vec_set_scalar(
    mut a: *mut OSQPFloat,
    mut sc: OSQPFloat,
    mut n: OSQPInt,
) {
    let mut i: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        *a.offset(i as isize) = sc;
        i += 1;
    }
}
unsafe extern "C" fn int_vec_set_scalar(mut a: *mut OSQPInt, mut sc: OSQPInt, mut n: OSQPInt) {
    let mut i: OSQPInt = 0;
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        *a.offset(i as isize) = sc;
        i += 1;
    }
}
#[export_name = "honest_osqp_csc_cumsum"]
pub unsafe extern "C" fn csc_cumsum(
    mut p: *mut OSQPInt,
    mut c: *mut OSQPInt,
    mut n: OSQPInt,
) -> OSQPInt {
    let mut i: OSQPInt = 0;
    let mut nz: OSQPInt = 0 as OSQPInt;
    if p.is_null() || c.is_null() {
        return -(1 as OSQPInt);
    }
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        *p.offset(i as isize) = nz;
        nz += *c.offset(i as isize) as ::core::ffi::c_int;
        *c.offset(i as isize) = *p.offset(i as isize);
        i += 1;
    }
    *p.offset(n as isize) = nz;
    return nz;
}
#[export_name = "honest_osqp_csc_spalloc"]
pub unsafe extern "C" fn csc_spalloc(
    mut m: OSQPInt,
    mut n: OSQPInt,
    mut nzmax: OSQPInt,
    mut values: OSQPInt,
    mut triplet: OSQPInt,
) -> *mut OSQPCscMatrix {
    let mut A: *mut OSQPCscMatrix = calloc(
        1 as size_t,
        ::core::mem::size_of::<OSQPCscMatrix>() as size_t,
    ) as *mut OSQPCscMatrix;
    if A.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    (*A).m = m;
    (*A).n = n;
    nzmax = (if nzmax > 0 as ::core::ffi::c_int {
        nzmax as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as OSQPInt;
    (*A).nzmax = nzmax;
    (*A).nz = (if triplet != 0 {
        0 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    }) as OSQPInt;
    (*A).p = csc_malloc(
        if triplet != 0 {
            nzmax
        } else {
            n + 1 as OSQPInt
        },
        ::core::mem::size_of::<OSQPInt>() as OSQPInt,
    ) as *mut OSQPInt;
    (*A).i = (if values != 0 {
        csc_malloc(nzmax, ::core::mem::size_of::<OSQPInt>() as OSQPInt)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    }) as *mut OSQPInt;
    (*A).x = (if values != 0 {
        csc_malloc(nzmax, ::core::mem::size_of::<OSQPFloat>() as OSQPInt)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    }) as *mut OSQPFloat;
    if (*A).p.is_null() || values != 0 && (*A).i.is_null() || values != 0 && (*A).x.is_null() {
        csc_spfree(A);
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    } else {
        return A;
    };
}
#[export_name = "honest_osqp_csc_spfree"]
pub unsafe extern "C" fn csc_spfree(mut A: *mut OSQPCscMatrix) {
    if !A.is_null() {
        if !(*A).p.is_null() {
            free((*A).p as *mut ::core::ffi::c_void);
        }
        if !(*A).i.is_null() {
            free((*A).i as *mut ::core::ffi::c_void);
        }
        if !(*A).x.is_null() {
            free((*A).x as *mut ::core::ffi::c_void);
        }
        free(A as *mut ::core::ffi::c_void);
    }
}
#[export_name = "honest_osqp_csc_submatrix_byrows"]
pub unsafe extern "C" fn csc_submatrix_byrows(
    mut A: *const OSQPCscMatrix,
    mut rows: *mut OSQPInt,
) -> *mut OSQPCscMatrix {
    let mut j: OSQPInt = 0;
    let mut R: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    let mut nzR: OSQPInt = 0 as OSQPInt;
    let mut An: OSQPInt = (*A).n;
    let mut Am: OSQPInt = (*A).m;
    let mut Ap: *mut OSQPInt = (*A).p;
    let mut Ai: *mut OSQPInt = (*A).i;
    let mut Ax: *mut OSQPFloat = (*A).x;
    let mut Rp: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Ri: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Rx: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut Rm: OSQPInt = 0 as OSQPInt;
    let mut ptr: OSQPInt = 0;
    let mut rridx: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    rridx = malloc((Am as size_t).wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t))
        as *mut OSQPInt;
    if rridx.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    Rm = 0 as ::core::ffi::c_int as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < Am {
        if *rows.offset(j as isize) != 0 {
            let fresh0 = Rm;
            Rm = Rm + 1;
            *rridx.offset(j as isize) = fresh0;
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < *Ap.offset(An as isize) {
        if *rows.offset(*(*A).i.offset(j as isize) as isize) != 0 {
            nzR += 1;
        }
        j += 1;
    }
    R = csc_spalloc(Rm, An, nzR, 1 as OSQPInt, 0 as OSQPInt);
    if R.is_null() {
        free(rridx as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    if Rm == 0 as ::core::ffi::c_int {
        int_vec_set_scalar((*R).p, 0 as OSQPInt, An + 1 as OSQPInt);
    } else {
        nzR = 0 as ::core::ffi::c_int as OSQPInt;
        Rp = (*R).p;
        Ri = (*R).i;
        Rx = (*R).x;
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < An {
            *Rp.offset(j as isize) = nzR;
            ptr = *Ap.offset(j as isize);
            while ptr < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
                if *rows.offset(*(*A).i.offset(ptr as isize) as isize) != 0 {
                    *Ri.offset(nzR as isize) = *rridx.offset(*Ai.offset(ptr as isize) as isize);
                    *Rx.offset(nzR as isize) = *Ax.offset(ptr as isize);
                    nzR += 1;
                }
                ptr += 1;
            }
            j += 1;
        }
        *Rp.offset(An as isize) = nzR;
    }
    free(rridx as *mut ::core::ffi::c_void);
    return R;
}
#[export_name = "honest_osqp_triplet_to_csc"]
pub unsafe extern "C" fn triplet_to_csc(
    mut T: *const OSQPCscMatrix,
    mut TtoC: *mut OSQPInt,
) -> *mut OSQPCscMatrix {
    let mut m: OSQPInt = 0;
    let mut n: OSQPInt = 0;
    let mut nz: OSQPInt = 0;
    let mut p: OSQPInt = 0;
    let mut k: OSQPInt = 0;
    let mut Cp: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Ci: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut w: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Ti: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Tj: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Cx: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut Tx: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut C: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    m = (*T).m;
    n = (*T).n;
    Ti = (*T).i;
    Tj = (*T).p;
    Tx = (*T).x;
    nz = (*T).nz;
    C = csc_spalloc(
        m,
        n,
        nz,
        (Tx != ::core::ptr::null_mut::<OSQPFloat>()) as ::core::ffi::c_int,
        0 as OSQPInt,
    );
    w = csc_calloc(n, ::core::mem::size_of::<OSQPInt>() as OSQPInt) as *mut OSQPInt;
    if C.is_null() || w.is_null() {
        return csc_done(
            C,
            w as *mut ::core::ffi::c_void,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as OSQPInt,
        );
    }
    Cp = (*C).p;
    Ci = (*C).i;
    Cx = (*C).x;
    k = 0 as ::core::ffi::c_int as OSQPInt;
    while k < nz {
        let ref mut fresh1 = *w.offset(*Tj.offset(k as isize) as isize);
        *fresh1 += 1;
        k += 1;
    }
    csc_cumsum(Cp, w, n);
    k = 0 as ::core::ffi::c_int as OSQPInt;
    while k < nz {
        let ref mut fresh2 = *w.offset(*Tj.offset(k as isize) as isize);
        let fresh3 = *fresh2;
        *fresh2 = *fresh2 + 1;
        p = fresh3;
        *Ci.offset(p as isize) = *Ti.offset(k as isize);
        if !Cx.is_null() {
            *Cx.offset(p as isize) = *Tx.offset(k as isize);
            if !TtoC.is_null() {
                *TtoC.offset(k as isize) = p;
            }
        }
        k += 1;
    }
    return csc_done(
        C,
        w as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        1 as OSQPInt,
    );
}
#[export_name = "honest_osqp_triplet_to_csr"]
pub unsafe extern "C" fn triplet_to_csr(
    mut T: *const OSQPCscMatrix,
    mut TtoC: *mut OSQPInt,
) -> *mut OSQPCscMatrix {
    let mut m: OSQPInt = 0;
    let mut n: OSQPInt = 0;
    let mut nz: OSQPInt = 0;
    let mut p: OSQPInt = 0;
    let mut k: OSQPInt = 0;
    let mut Cp: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Cj: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut w: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Ti: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Tj: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Cx: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut Tx: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut C: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    m = (*T).m;
    n = (*T).n;
    Ti = (*T).i;
    Tj = (*T).p;
    Tx = (*T).x;
    nz = (*T).nz;
    C = csc_spalloc(
        m,
        n,
        nz,
        (Tx != ::core::ptr::null_mut::<OSQPFloat>()) as ::core::ffi::c_int,
        0 as OSQPInt,
    );
    w = csc_calloc(m, ::core::mem::size_of::<OSQPInt>() as OSQPInt) as *mut OSQPInt;
    if C.is_null() || w.is_null() {
        return csc_done(
            C,
            w as *mut ::core::ffi::c_void,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as OSQPInt,
        );
    }
    Cp = (*C).p;
    Cj = (*C).i;
    Cx = (*C).x;
    k = 0 as ::core::ffi::c_int as OSQPInt;
    while k < nz {
        let ref mut fresh4 = *w.offset(*Ti.offset(k as isize) as isize);
        *fresh4 += 1;
        k += 1;
    }
    csc_cumsum(Cp, w, m);
    k = 0 as ::core::ffi::c_int as OSQPInt;
    while k < nz {
        let ref mut fresh5 = *w.offset(*Ti.offset(k as isize) as isize);
        let fresh6 = *fresh5;
        *fresh5 = *fresh5 + 1;
        p = fresh6;
        *Cj.offset(p as isize) = *Tj.offset(k as isize);
        if !Cx.is_null() {
            *Cx.offset(p as isize) = *Tx.offset(k as isize);
            if !TtoC.is_null() {
                *TtoC.offset(k as isize) = p;
            }
        }
        k += 1;
    }
    return csc_done(
        C,
        w as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        1 as OSQPInt,
    );
}
#[export_name = "honest_osqp_csc_extract_diag"]
pub unsafe extern "C" fn csc_extract_diag(mut A: *const OSQPCscMatrix, mut d: *mut OSQPFloat) {
    let mut i: OSQPInt = 0;
    let mut ptr: OSQPInt = 0;
    let mut n: OSQPInt = (*A).n;
    let mut Ap: *mut OSQPInt = (*A).p;
    let mut Ai: *mut OSQPInt = (*A).i;
    let mut Ax: *mut OSQPFloat = (*A).x;
    float_vec_set_scalar(d, 0.0f64, n);
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        ptr = *Ap.offset(i as isize);
        while ptr < *Ap.offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
            if *Ai.offset(ptr as isize) == i {
                *d.offset(i as isize) = *Ax.offset(ptr as isize);
            }
            ptr += 1;
        }
        i += 1;
    }
}
#[export_name = "honest_osqp_csc_pinv"]
pub unsafe extern "C" fn csc_pinv(mut p: *const OSQPInt, mut n: OSQPInt) -> *mut OSQPInt {
    let mut k: OSQPInt = 0;
    let mut pinv: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    if p.is_null() {
        return ::core::ptr::null_mut::<OSQPInt>();
    }
    pinv = csc_malloc(n, ::core::mem::size_of::<OSQPInt>() as OSQPInt) as *mut OSQPInt;
    if pinv.is_null() {
        return ::core::ptr::null_mut::<OSQPInt>();
    }
    k = 0 as ::core::ffi::c_int as OSQPInt;
    while k < n {
        *pinv.offset(*p.offset(k as isize) as isize) = k;
        k += 1;
    }
    return pinv;
}
#[export_name = "honest_osqp_csc_symperm"]
pub unsafe extern "C" fn csc_symperm(
    mut A: *const OSQPCscMatrix,
    mut pinv: *const OSQPInt,
    mut AtoC: *mut OSQPInt,
    mut values: OSQPInt,
) -> *mut OSQPCscMatrix {
    let mut i: OSQPInt = 0;
    let mut j: OSQPInt = 0;
    let mut p: OSQPInt = 0;
    let mut q: OSQPInt = 0;
    let mut i2: OSQPInt = 0;
    let mut j2: OSQPInt = 0;
    let mut n: OSQPInt = 0;
    let mut Ap: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Ai: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Cp: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Ci: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut w: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut Cx: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut Ax: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut C: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    n = (*A).n;
    Ap = (*A).p;
    Ai = (*A).i;
    Ax = (*A).x;
    C = csc_spalloc(
        n,
        n,
        *Ap.offset(n as isize),
        (values != 0 && !Ax.is_null()) as ::core::ffi::c_int,
        0 as OSQPInt,
    );
    w = csc_calloc(n, ::core::mem::size_of::<OSQPInt>() as OSQPInt) as *mut OSQPInt;
    if C.is_null() || w.is_null() {
        return csc_done(
            C,
            w as *mut ::core::ffi::c_void,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as OSQPInt,
        );
    }
    Cp = (*C).p;
    Ci = (*C).i;
    Cx = (*C).x;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        j2 = (if !pinv.is_null() {
            *pinv.offset(j as isize) as ::core::ffi::c_int
        } else {
            j as ::core::ffi::c_int
        }) as OSQPInt;
        p = *Ap.offset(j as isize);
        while p < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
            i = *Ai.offset(p as isize);
            if !(i > j) {
                i2 = (if !pinv.is_null() {
                    *pinv.offset(i as isize) as ::core::ffi::c_int
                } else {
                    i as ::core::ffi::c_int
                }) as OSQPInt;
                let ref mut fresh7 = *w.offset(
                    (if i2 > j2 {
                        i2 as ::core::ffi::c_int
                    } else {
                        j2 as ::core::ffi::c_int
                    }) as isize,
                );
                *fresh7 += 1;
            }
            p += 1;
        }
        j += 1;
    }
    csc_cumsum(Cp, w, n);
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        j2 = (if !pinv.is_null() {
            *pinv.offset(j as isize) as ::core::ffi::c_int
        } else {
            j as ::core::ffi::c_int
        }) as OSQPInt;
        p = *Ap.offset(j as isize);
        while p < *Ap.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) {
            i = *Ai.offset(p as isize);
            if !(i > j) {
                i2 = (if !pinv.is_null() {
                    *pinv.offset(i as isize) as ::core::ffi::c_int
                } else {
                    i as ::core::ffi::c_int
                }) as OSQPInt;
                let ref mut fresh8 = *w.offset(
                    (if i2 > j2 {
                        i2 as ::core::ffi::c_int
                    } else {
                        j2 as ::core::ffi::c_int
                    }) as isize,
                );
                let fresh9 = *fresh8;
                *fresh8 = *fresh8 + 1;
                q = fresh9;
                *Ci.offset(q as isize) = (if i2 < j2 {
                    i2 as ::core::ffi::c_int
                } else {
                    j2 as ::core::ffi::c_int
                }) as OSQPInt;
                if !Cx.is_null() {
                    *Cx.offset(q as isize) = *Ax.offset(p as isize);
                }
                if !AtoC.is_null() {
                    *AtoC.offset(p as isize) = q;
                }
            }
            p += 1;
        }
        j += 1;
    }
    return csc_done(
        C,
        w as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        1 as OSQPInt,
    );
}
#[export_name = "honest_osqp_csc_copy"]
pub unsafe extern "C" fn csc_copy(mut A: *const OSQPCscMatrix) -> *mut OSQPCscMatrix {
    let mut B: *mut OSQPCscMatrix = csc_spalloc(
        (*A).m,
        (*A).n,
        *(*A).p.offset((*A).n as isize),
        ((*A).x != ::core::ptr::null_mut::<OSQPFloat>()) as ::core::ffi::c_int,
        0 as OSQPInt,
    );
    if B.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    prea_int_vec_copy((*A).p, (*B).p, (*A).n + 1 as OSQPInt);
    prea_int_vec_copy((*A).i, (*B).i, *(*A).p.offset((*A).n as isize));
    prea_vec_copy((*A).x, (*B).x, *(*A).p.offset((*A).n as isize));
    return B;
}
#[export_name = "honest_osqp_csc_to_dns"]
pub unsafe extern "C" fn csc_to_dns(mut M: *mut OSQPCscMatrix) -> *mut OSQPFloat {
    let mut i: OSQPInt = 0;
    let mut j: OSQPInt = 0 as OSQPInt;
    let mut idx: OSQPInt = 0;
    let mut A: *mut OSQPFloat = calloc(
        ((*M).m * (*M).n) as size_t,
        ::core::mem::size_of::<OSQPFloat>() as size_t,
    ) as *mut OSQPFloat;
    if A.is_null() {
        return ::core::ptr::null_mut::<OSQPFloat>();
    }
    idx = 0 as ::core::ffi::c_int as OSQPInt;
    while idx < *(*M).p.offset((*M).n as isize) {
        i = *(*M).i.offset(idx as isize);
        while *(*M)
            .p
            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            <= idx
        {
            j += 1;
        }
        *A.offset((j * (*M).m + i) as isize) = *(*M).x.offset(idx as isize);
        idx += 1;
    }
    return A;
}
#[export_name = "honest_osqp_csc_done"]
pub unsafe extern "C" fn csc_done(
    mut C: *mut OSQPCscMatrix,
    mut w: *mut ::core::ffi::c_void,
    mut x: *mut ::core::ffi::c_void,
    mut ok: OSQPInt,
) -> *mut OSQPCscMatrix {
    free(w);
    free(x);
    if ok != 0 {
        return C;
    } else {
        csc_spfree(C);
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    };
}
#[export_name = "honest_osqp_triu_to_csc"]
pub unsafe extern "C" fn triu_to_csc(mut M: *mut OSQPCscMatrix) -> *mut OSQPCscMatrix {
    let mut M_trip: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    let mut M_symm: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    let mut n: OSQPInt = 0;
    let mut ptr: OSQPInt = 0;
    let mut i: OSQPInt = 0;
    let mut j: OSQPInt = 0;
    let mut z_M: OSQPInt = 0 as OSQPInt;
    if (*M).m != (*M).n {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    n = (*M).n;
    M_trip = csc_spalloc(
        n,
        n,
        2 as OSQPInt * *(*M).p.offset(n as isize),
        1 as OSQPInt,
        1 as OSQPInt,
    );
    if M_trip.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        ptr = *(*M).p.offset(j as isize);
        while ptr
            < *(*M)
                .p
                .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        {
            i = *(*M).i.offset(ptr as isize);
            *(*M_trip).i.offset(z_M as isize) = i;
            *(*M_trip).p.offset(z_M as isize) = j;
            *(*M_trip).x.offset(z_M as isize) = *(*M).x.offset(ptr as isize);
            z_M += 1;
            if i < j {
                *(*M_trip).i.offset(z_M as isize) = j;
                *(*M_trip).p.offset(z_M as isize) = i;
                *(*M_trip).x.offset(z_M as isize) = *(*M).x.offset(ptr as isize);
                z_M += 1;
            }
            ptr += 1;
        }
        j += 1;
    }
    (*M_trip).nz = z_M;
    M_symm = triplet_to_csc(M_trip, ::core::ptr::null_mut::<OSQPInt>());
    (*M_symm).nzmax = z_M;
    csc_spfree(M_trip);
    return M_symm;
}
#[export_name = "honest_osqp_vstack"]
pub unsafe extern "C" fn vstack(
    mut A: *mut OSQPCscMatrix,
    mut B: *mut OSQPCscMatrix,
) -> *mut OSQPCscMatrix {
    let mut M_trip: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    let mut M: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    let mut m1: OSQPInt = 0;
    let mut m2: OSQPInt = 0;
    let mut n: OSQPInt = 0;
    let mut ptr: OSQPInt = 0;
    let mut i: OSQPInt = 0;
    let mut j: OSQPInt = 0;
    let mut z_M: OSQPInt = 0 as OSQPInt;
    if (*A).n != (*B).n {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    m1 = (*A).m;
    m2 = (*B).m;
    n = (*A).n;
    M_trip = csc_spalloc(
        m1 + m2,
        n,
        (*A).nzmax + (*B).nzmax,
        1 as OSQPInt,
        1 as OSQPInt,
    );
    if M_trip.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        ptr = *(*A).p.offset(j as isize);
        while ptr
            < *(*A)
                .p
                .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        {
            i = *(*A).i.offset(ptr as isize);
            *(*M_trip).i.offset(z_M as isize) = i;
            *(*M_trip).p.offset(z_M as isize) = j;
            *(*M_trip).x.offset(z_M as isize) = *(*A).x.offset(ptr as isize);
            z_M += 1;
            ptr += 1;
        }
        j += 1;
    }
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        ptr = *(*B).p.offset(j as isize);
        while ptr
            < *(*B)
                .p
                .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        {
            i = *(*B).i.offset(ptr as isize) + m1;
            *(*M_trip).i.offset(z_M as isize) = i;
            *(*M_trip).p.offset(z_M as isize) = j;
            *(*M_trip).x.offset(z_M as isize) = *(*B).x.offset(ptr as isize);
            z_M += 1;
            ptr += 1;
        }
        j += 1;
    }
    (*M_trip).nz = z_M;
    M = triplet_to_csc(M_trip, ::core::ptr::null_mut::<OSQPInt>());
    (*M).nzmax = z_M;
    csc_spfree(M_trip);
    return M;
}
