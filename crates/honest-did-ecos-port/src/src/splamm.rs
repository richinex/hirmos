use crate::runtime::{malloc,free};
extern "C" {
}
pub type pfloat = ::core::ffi::c_double;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type idxint = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spmat {
    pub jc: *mut idxint,
    pub ir: *mut idxint,
    pub pr: *mut pfloat,
    pub n: idxint,
    pub m: idxint,
    pub nnz: idxint,
}
#[no_mangle]
pub unsafe extern "C" fn spla_cumsum(mut p: *mut idxint, mut w: *mut idxint, mut m: idxint) {
    let mut cumsum: idxint = 0 as idxint;
    let mut i: idxint = 0;
    i = 0 as idxint;
    while i < m {
        *p.offset(i as isize) = cumsum;
        cumsum += *w.offset(i as isize) as i64;
        *w.offset(i as isize) = *p.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pinv(mut n: idxint, mut p: *mut idxint, mut pinv_0: *mut idxint) {
    let mut i: idxint = 0;
    i = 0 as idxint;
    while i < n {
        *pinv_0.offset(*p.offset(i as isize) as isize) = i;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn transposeSparseMatrix(
    mut M: *mut spmat,
    mut MtoMt: *mut idxint,
) -> *mut spmat {
    let mut j: idxint = 0;
    let mut i: idxint = 0;
    let mut k: idxint = 0;
    let mut q: idxint = 0;
    let mut w: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut A: *mut spmat = newSparseMatrix((*M).n, (*M).m, (*M).nnz);
    if (*M).nnz == 0 as i64 {
        return A;
    }
    w = malloc(((*M).m as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
        as *mut idxint;
    i = 0 as idxint;
    while i < (*M).m {
        *w.offset(i as isize) = 0 as idxint;
        i += 1;
    }
    k = 0 as idxint;
    while k < (*M).nnz {
        let ref mut fresh0 = *w.offset(*(*M).ir.offset(k as isize) as isize);
        *fresh0 += 1;
        k += 1;
    }
    spla_cumsum((*A).jc, w, (*M).m);
    j = 0 as idxint;
    while j < (*M).n {
        k = *(*M).jc.offset(j as isize);
        while k < *(*M)
            .jc
            .offset((j as i64 + 1 as i64) as isize)
        {
            let ref mut fresh1 = *w.offset(*(*M).ir.offset(k as isize) as isize);
            let fresh2 = *fresh1;
            *fresh1 = *fresh1 + 1;
            q = fresh2;
            *(*A).ir.offset(q as isize) = j;
            *(*A).pr.offset(q as isize) = *(*M).pr.offset(k as isize);
            *MtoMt.offset(k as isize) = q;
            k += 1;
        }
        j += 1;
    }
    free(w as *mut ::core::ffi::c_void);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn newSparseMatrix(
    mut m: idxint,
    mut n: idxint,
    mut nnz: idxint,
) -> *mut spmat {
    let mut jc: *mut idxint = malloc(
        ((n as i64 + 1 as i64) as size_t)
            .wrapping_mul(::core::mem::size_of::<idxint>() as size_t),
    ) as *mut idxint;
    let mut ir: *mut idxint =
        malloc((nnz as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
            as *mut idxint;
    let mut pr: *mut pfloat =
        malloc((nnz as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    *jc.offset(n as isize) = nnz;
    return ecoscreateSparseMatrix(m, n, nnz, jc, ir, pr);
}
#[no_mangle]
pub unsafe extern "C" fn ecoscreateSparseMatrix(
    mut m: idxint,
    mut n: idxint,
    mut nnz: idxint,
    mut jc: *mut idxint,
    mut ir: *mut idxint,
    mut pr: *mut pfloat,
) -> *mut spmat {
    let mut M: *mut spmat = malloc(::core::mem::size_of::<spmat>() as size_t) as *mut spmat;
    (*M).m = m;
    (*M).n = n;
    (*M).nnz = nnz;
    (*M).jc = jc;
    (*M).ir = ir;
    (*M).pr = pr;
    if !(*M).jc.is_null() {
        *(*M).jc.offset(n as isize) = nnz;
    }
    return M;
}
#[no_mangle]
pub unsafe extern "C" fn freeSparseMatrix(mut M: *mut spmat) {
    if !(*M).ir.is_null() {
        free((*M).ir as *mut ::core::ffi::c_void);
    }
    if !(*M).jc.is_null() {
        free((*M).jc as *mut ::core::ffi::c_void);
    }
    if !(*M).pr.is_null() {
        free((*M).pr as *mut ::core::ffi::c_void);
    }
    free(M as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn permuteSparseSymmetricMatrix(
    mut A: *mut spmat,
    mut pinv_0: *mut idxint,
    mut C: *mut spmat,
    mut PK: *mut idxint,
) {
    let mut i: idxint = 0;
    let mut i2: idxint = 0;
    let mut j: idxint = 0;
    let mut j2: idxint = 0;
    let mut k: idxint = 0;
    let mut q: idxint = 0;
    let mut w: *mut idxint = ::core::ptr::null_mut::<idxint>();
    w = malloc(((*A).n as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
        as *mut idxint;
    j = 0 as idxint;
    while j < (*A).n {
        *w.offset(j as isize) = 0 as idxint;
        j += 1;
    }
    j = 0 as idxint;
    while j < (*A).n {
        j2 = *pinv_0.offset(j as isize);
        k = *(*A).jc.offset(j as isize);
        while k < *(*A)
            .jc
            .offset((j as i64 + 1 as i64) as isize)
        {
            i = *(*A).ir.offset(k as isize);
            if !(i > j) {
                i2 = *pinv_0.offset(i as isize);
                let ref mut fresh3 = *w.offset(
                    (if i2 > j2 {
                        i2 as i64
                    } else {
                        j2 as i64
                    }) as isize,
                );
                *fresh3 += 1;
            }
            k += 1;
        }
        j += 1;
    }
    spla_cumsum((*C).jc, w, (*A).n);
    j = 0 as idxint;
    while j < (*A).n {
        j2 = *pinv_0.offset(j as isize);
        k = *(*A).jc.offset(j as isize);
        while k < *(*A)
            .jc
            .offset((j as i64 + 1 as i64) as isize)
        {
            i = *(*A).ir.offset(k as isize);
            if !(i > j) {
                i2 = *pinv_0.offset(i as isize);
                let ref mut fresh4 = *w.offset(
                    (if i2 > j2 {
                        i2 as i64
                    } else {
                        j2 as i64
                    }) as isize,
                );
                let fresh5 = *fresh4;
                *fresh4 = *fresh4 + 1;
                q = fresh5;
                *(*C).ir.offset(q as isize) = (if i2 < j2 {
                    i2 as i64
                } else {
                    j2 as i64
                }) as idxint;
                *(*C).pr.offset(q as isize) = *(*A).pr.offset(k as isize);
                if !PK.is_null() {
                    *PK.offset(k as isize) = q;
                }
            }
            k += 1;
        }
        j += 1;
    }
    free(w as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn copySparseMatrix(mut A: *mut spmat) -> *mut spmat {
    let mut i: idxint = 0;
    let mut B: *mut spmat = newSparseMatrix((*A).m, (*A).n, (*A).nnz);
    i = 0 as idxint;
    while i <= (*A).n {
        *(*B).jc.offset(i as isize) = *(*A).jc.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*A).nnz {
        *(*B).ir.offset(i as isize) = *(*A).ir.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*A).nnz {
        *(*B).pr.offset(i as isize) = *(*A).pr.offset(i as isize);
        i += 1;
    }
    return B;
}
