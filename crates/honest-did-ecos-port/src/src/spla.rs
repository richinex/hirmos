use crate::runtime::{sqrt};
extern "C" {
}
pub type pfloat = ::core::ffi::c_double;
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
pub unsafe extern "C" fn sparseMV(
    mut A: *mut spmat,
    mut x: *mut pfloat,
    mut y: *mut pfloat,
    mut a: idxint,
    mut newVector: idxint,
) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    if newVector > 0 as i64 {
        i = 0 as idxint;
        while i < (*A).m {
            *y.offset(i as isize) = 0 as ::core::ffi::c_int as pfloat;
            i += 1;
        }
    }
    if (*A).nnz == 0 as i64 {
        return;
    }
    if a > 0 as i64 {
        j = 0 as idxint;
        while j < (*A).n {
            i = *(*A).jc.offset(j as isize);
            while i < *(*A)
                .jc
                .offset((j as i64 + 1 as i64) as isize)
            {
                let ref mut fresh0 = *y.offset(*(*A).ir.offset(i as isize) as isize);
                *fresh0 +=
                    (*(*A).pr.offset(i as isize) * *x.offset(j as isize)) as ::core::ffi::c_double;
                i += 1;
            }
            j += 1;
        }
    } else {
        j = 0 as idxint;
        while j < (*A).n {
            i = *(*A).jc.offset(j as isize);
            while i < *(*A)
                .jc
                .offset((j as i64 + 1 as i64) as isize)
            {
                let ref mut fresh1 = *y.offset(*(*A).ir.offset(i as isize) as isize);
                *fresh1 -=
                    (*(*A).pr.offset(i as isize) * *x.offset(j as isize)) as ::core::ffi::c_double;
                i += 1;
            }
            j += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sparseMtVm(
    mut A: *mut spmat,
    mut x: *mut pfloat,
    mut y: *mut pfloat,
    mut newVector: idxint,
    mut skipDiagonal: idxint,
) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    if newVector > 0 as i64 {
        j = 0 as idxint;
        while j < (*A).n {
            *y.offset(j as isize) = 0 as ::core::ffi::c_int as pfloat;
            j += 1;
        }
    }
    if (*A).nnz == 0 as i64 {
        return;
    }
    if skipDiagonal != 0 {
        j = 0 as idxint;
        while j < (*A).n {
            k = *(*A).jc.offset(j as isize);
            while k < *(*A)
                .jc
                .offset((j as i64 + 1 as i64) as isize)
            {
                i = *(*A).ir.offset(k as isize);
                let ref mut fresh2 = *y.offset(j as isize);
                *fresh2 -= if i == j {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    *(*A).pr.offset(k as isize) as ::core::ffi::c_double
                        * *x.offset(i as isize) as ::core::ffi::c_double
                };
                k += 1;
            }
            j += 1;
        }
    } else {
        j = 0 as idxint;
        while j < (*A).n {
            k = *(*A).jc.offset(j as isize);
            while k < *(*A)
                .jc
                .offset((j as i64 + 1 as i64) as isize)
            {
                let ref mut fresh3 = *y.offset(j as isize);
                *fresh3 -= (*(*A).pr.offset(k as isize)
                    * *x.offset(*(*A).ir.offset(k as isize) as isize))
                    as ::core::ffi::c_double;
                k += 1;
            }
            j += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn vadd(mut n: idxint, mut x: *mut pfloat, mut y: *mut pfloat) {
    let mut i: idxint = 0;
    i = 0 as idxint;
    while i < n {
        let ref mut fresh4 = *y.offset(i as isize);
        *fresh4 += *x.offset(i as isize) as ::core::ffi::c_double;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vsubscale(
    mut n: idxint,
    mut a: pfloat,
    mut x: *mut pfloat,
    mut y: *mut pfloat,
) {
    let mut i: idxint = 0;
    i = 0 as idxint;
    while i < n {
        let ref mut fresh5 = *y.offset(i as isize);
        *fresh5 -= (a * *x.offset(i as isize)) as ::core::ffi::c_double;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn norm2(mut v: *mut pfloat, mut n: idxint) -> pfloat {
    let mut i: idxint = 0;
    let mut normsquare: pfloat = 0 as ::core::ffi::c_int as pfloat;
    i = 0 as idxint;
    while i < n {
        normsquare += (*v.offset(i as isize) * *v.offset(i as isize)) as ::core::ffi::c_double;
        i += 1;
    }
    return sqrt(normsquare as ::core::ffi::c_double) as pfloat;
}
#[no_mangle]
pub unsafe extern "C" fn norminf(mut v: *mut pfloat, mut n: idxint) -> pfloat {
    let mut i: idxint = 0;
    let mut norm: pfloat = 0 as ::core::ffi::c_int as pfloat;
    let mut mv: pfloat = 0.;
    i = 0 as idxint;
    while i < n {
        if *v.offset(i as isize) > norm {
            norm = *v.offset(i as isize);
        }
        mv = -*v.offset(i as isize);
        if mv > norm {
            norm = mv;
        }
        i += 1;
    }
    return norm;
}
#[no_mangle]
pub unsafe extern "C" fn eddot(mut n: idxint, mut x: *mut pfloat, mut y: *mut pfloat) -> pfloat {
    let mut z: pfloat = 0 as ::core::ffi::c_int as pfloat;
    let mut i: idxint = 0;
    i = 0 as idxint;
    while i < n {
        z += (*x.offset(i as isize) * *y.offset(i as isize)) as ::core::ffi::c_double;
        i += 1;
    }
    return z;
}
