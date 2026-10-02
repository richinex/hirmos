#[no_mangle]
pub unsafe extern "C" fn ldl_l_symbolic2(
    mut n: i64,
    mut Ap: *mut i64,
    mut Ai: *mut i64,
    mut Lp: *mut i64,
    mut Parent: *mut i64,
    mut Lnz: *mut i64,
    mut Flag: *mut i64,
) {
    let mut i: i64 = 0;
    let mut k: i64 = 0;
    let mut p: i64 = 0;
    let mut p2: i64 = 0;
    k = 0 as i64;
    while k < n {
        *Parent.offset(k as isize) = -(1 as ::core::ffi::c_int) as i64;
        *Flag.offset(k as isize) = k;
        *Lnz.offset(k as isize) = 0 as i64;
        p2 = *Ap.offset((k + 1 as i64) as isize);
        p = *Ap.offset(k as isize);
        while p < p2 {
            i = *Ai.offset(p as isize);
            while *Flag.offset(i as isize) != k {
                if *Parent.offset(i as isize) == -(1 as ::core::ffi::c_int) as i64 {
                    *Parent.offset(i as isize) = k;
                }
                let ref mut fresh0 = *Lnz.offset(i as isize);
                *fresh0 += 1;
                *Flag.offset(i as isize) = k;
                i = *Parent.offset(i as isize);
            }
            p += 1;
        }
        k += 1;
    }
    *Lp.offset(0 as ::core::ffi::c_int as isize) = 0 as i64;
    k = 0 as i64;
    while k < n {
        *Lp.offset((k + 1 as i64) as isize) =
            *Lp.offset(k as isize) + *Lnz.offset(k as isize);
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldl_l_numeric2(
    mut n: i64,
    mut Ap: *mut i64,
    mut Ai: *mut i64,
    mut Ax: *mut ::core::ffi::c_double,
    mut Lp: *mut i64,
    mut Parent: *mut i64,
    mut Sign: *mut i64,
    mut eps: ::core::ffi::c_double,
    mut delta: ::core::ffi::c_double,
    mut Lnz: *mut i64,
    mut Li: *mut i64,
    mut Lx: *mut ::core::ffi::c_double,
    mut D: *mut ::core::ffi::c_double,
    mut Y: *mut ::core::ffi::c_double,
    mut Pattern: *mut i64,
    mut Flag: *mut i64,
) -> i64 {
    let mut yi: ::core::ffi::c_double = 0.;
    let mut l_ki: ::core::ffi::c_double = 0.;
    let mut i: i64 = 0;
    let mut k: i64 = 0;
    let mut p: i64 = 0;
    let mut p2: i64 = 0;
    let mut len: i64 = 0;
    let mut top: i64 = 0;
    k = 0 as i64;
    while k < n {
        *Y.offset(k as isize) = 0.0f64;
        top = n;
        *Flag.offset(k as isize) = k;
        *Lnz.offset(k as isize) = 0 as i64;
        p2 = *Ap.offset((k + 1 as i64) as isize);
        p = *Ap.offset(k as isize);
        while p < p2 {
            i = *Ai.offset(p as isize);
            *Y.offset(i as isize) = *Ax.offset(p as isize);
            len = 0 as i64;
            while *Flag.offset(i as isize) != k {
                let fresh1 = len;
                len = len + 1;
                *Pattern.offset(fresh1 as isize) = i;
                *Flag.offset(i as isize) = k;
                i = *Parent.offset(i as isize);
            }
            while len > 0 as i64 {
                len -= 1;
                top -= 1;
                *Pattern.offset(top as isize) = *Pattern.offset(len as isize);
            }
            p += 1;
        }
        *D.offset(k as isize) = *Y.offset(k as isize);
        *Y.offset(k as isize) = 0.0f64;
        while top < n {
            i = *Pattern.offset(top as isize);
            yi = *Y.offset(i as isize);
            *Y.offset(i as isize) = 0.0f64;
            p2 = *Lp.offset(i as isize) + *Lnz.offset(i as isize);
            p = *Lp.offset(i as isize);
            while p < p2 {
                *Y.offset(*Li.offset(p as isize) as isize) -= *Lx.offset(p as isize) * yi;
                p += 1;
            }
            l_ki = yi / *D.offset(i as isize);
            *D.offset(k as isize) -= l_ki * yi;
            *Li.offset(p as isize) = k;
            *Lx.offset(p as isize) = l_ki;
            let ref mut fresh2 = *Lnz.offset(i as isize);
            *fresh2 += 1;
            top += 1;
        }
        *D.offset(k as isize) =
            if *Sign.offset(k as isize) as ::core::ffi::c_double * *D.offset(k as isize) <= eps {
                *Sign.offset(k as isize) as ::core::ffi::c_double * delta
            } else {
                *D.offset(k as isize)
            };
        k += 1;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn ldl_l_lsolve(
    mut n: i64,
    mut X: *mut ::core::ffi::c_double,
    mut Lp: *mut i64,
    mut Li: *mut i64,
    mut Lx: *mut ::core::ffi::c_double,
) {
    let mut j: i64 = 0;
    let mut p: i64 = 0;
    let mut p2: i64 = 0;
    j = 0 as i64;
    while j < n {
        p2 = *Lp.offset((j + 1 as i64) as isize);
        p = *Lp.offset(j as isize);
        while p < p2 {
            *X.offset(*Li.offset(p as isize) as isize) -=
                *Lx.offset(p as isize) * *X.offset(j as isize);
            p += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldl_l_lsolve2(
    mut n: i64,
    mut b: *mut ::core::ffi::c_double,
    mut Lp: *mut i64,
    mut Li: *mut i64,
    mut Lx: *mut ::core::ffi::c_double,
    mut x: *mut ::core::ffi::c_double,
) {
    let mut j: i64 = 0;
    let mut p: i64 = 0;
    let mut p2: i64 = 0;
    j = 0 as i64;
    while j < n {
        *x.offset(j as isize) = *b.offset(j as isize);
        j += 1;
    }
    j = 0 as i64;
    while j < n {
        p2 = *Lp.offset((j + 1 as i64) as isize);
        p = *Lp.offset(j as isize);
        while p < p2 {
            *x.offset(*Li.offset(p as isize) as isize) -=
                *Lx.offset(p as isize) * *x.offset(j as isize);
            p += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldl_l_dsolve(
    mut n: i64,
    mut X: *mut ::core::ffi::c_double,
    mut D: *mut ::core::ffi::c_double,
) {
    let mut j: i64 = 0;
    j = 0 as i64;
    while j < n {
        *X.offset(j as isize) /= *D.offset(j as isize);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldl_l_ltsolve(
    mut n: i64,
    mut X: *mut ::core::ffi::c_double,
    mut Lp: *mut i64,
    mut Li: *mut i64,
    mut Lx: *mut ::core::ffi::c_double,
) {
    let mut j: i64 = 0;
    let mut p: i64 = 0;
    let mut p2: i64 = 0;
    j = n - 1 as i64;
    while j >= 0 as i64 {
        p2 = *Lp.offset((j + 1 as i64) as isize);
        p = *Lp.offset(j as isize);
        while p < p2 {
            *X.offset(j as isize) -=
                *Lx.offset(p as isize) * *X.offset(*Li.offset(p as isize) as isize);
            p += 1;
        }
        j -= 1;
    }
}
