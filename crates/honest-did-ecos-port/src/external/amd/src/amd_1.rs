extern "C" {
    fn amd_l2(
        n: i64,
        Pe: *mut i64,
        Iw: *mut i64,
        Len: *mut i64,
        iwlen: i64,
        pfree: i64,
        Nv: *mut i64,
        Next: *mut i64,
        Last: *mut i64,
        Head: *mut i64,
        Elen: *mut i64,
        Degree: *mut i64,
        W: *mut i64,
        Control: *mut ::core::ffi::c_double,
        Info: *mut ::core::ffi::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn amd_l1(
    mut n: i64,
    mut Ap: *const i64,
    mut Ai: *const i64,
    mut P: *mut i64,
    mut Pinv: *mut i64,
    mut Len: *mut i64,
    mut slen: i64,
    mut S: *mut i64,
    mut Control: *mut ::core::ffi::c_double,
    mut Info: *mut ::core::ffi::c_double,
) {
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut k: i64 = 0;
    let mut p: i64 = 0;
    let mut pfree: i64 = 0;
    let mut iwlen: i64 = 0;
    let mut pj: i64 = 0;
    let mut p1: i64 = 0;
    let mut p2: i64 = 0;
    let mut pj2: i64 = 0;
    let mut Iw: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Pe: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Nv: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Head: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Elen: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Degree: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut s: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut W: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Sp: *mut i64 = ::core::ptr::null_mut::<i64>();
    let mut Tp: *mut i64 = ::core::ptr::null_mut::<i64>();
    iwlen = slen - 6 as i64 * n;
    s = S as *mut i64;
    Pe = s;
    s = s.offset(n as isize);
    Nv = s;
    s = s.offset(n as isize);
    Head = s;
    s = s.offset(n as isize);
    Elen = s;
    s = s.offset(n as isize);
    Degree = s;
    s = s.offset(n as isize);
    W = s;
    s = s.offset(n as isize);
    Iw = s;
    s = s.offset(iwlen as isize);
    Sp = Nv;
    Tp = W;
    pfree = 0 as i64;
    j = 0 as i64;
    while j < n {
        *Pe.offset(j as isize) = pfree;
        *Sp.offset(j as isize) = pfree;
        pfree += *Len.offset(j as isize);
        j += 1;
    }
    k = 0 as i64;
    while k < n {
        p1 = *Ap.offset(k as isize);
        p2 = *Ap.offset((k + 1 as i64) as isize);
        p = p1;
        while p < p2 {
            j = *Ai.offset(p as isize);
            if j < k {
                let ref mut fresh0 = *Sp.offset(j as isize);
                let fresh1 = *fresh0;
                *fresh0 = *fresh0 + 1;
                *Iw.offset(fresh1 as isize) = k;
                let ref mut fresh2 = *Sp.offset(k as isize);
                let fresh3 = *fresh2;
                *fresh2 = *fresh2 + 1;
                *Iw.offset(fresh3 as isize) = j;
                p += 1;
                pj2 = *Ap.offset((j + 1 as i64) as isize);
                pj = *Tp.offset(j as isize);
                while pj < pj2 {
                    i = *Ai.offset(pj as isize);
                    if i < k {
                        let ref mut fresh4 = *Sp.offset(i as isize);
                        let fresh5 = *fresh4;
                        *fresh4 = *fresh4 + 1;
                        *Iw.offset(fresh5 as isize) = j;
                        let ref mut fresh6 = *Sp.offset(j as isize);
                        let fresh7 = *fresh6;
                        *fresh6 = *fresh6 + 1;
                        *Iw.offset(fresh7 as isize) = i;
                        pj += 1;
                    } else if i == k {
                        pj += 1;
                        break;
                    } else {
                        break;
                    }
                }
                *Tp.offset(j as isize) = pj;
            } else if j == k {
                p += 1;
                break;
            } else {
                break;
            }
        }
        *Tp.offset(k as isize) = p;
        k += 1;
    }
    j = 0 as i64;
    while j < n {
        pj = *Tp.offset(j as isize);
        while pj < *Ap.offset((j + 1 as i64) as isize) {
            i = *Ai.offset(pj as isize);
            let ref mut fresh8 = *Sp.offset(i as isize);
            let fresh9 = *fresh8;
            *fresh8 = *fresh8 + 1;
            *Iw.offset(fresh9 as isize) = j;
            let ref mut fresh10 = *Sp.offset(j as isize);
            let fresh11 = *fresh10;
            *fresh10 = *fresh10 + 1;
            *Iw.offset(fresh11 as isize) = i;
            pj += 1;
        }
        j += 1;
    }
    amd_l2(
        n,
        Pe as *mut i64,
        Iw as *mut i64,
        Len,
        iwlen,
        pfree,
        Nv as *mut i64,
        Pinv,
        P,
        Head as *mut i64,
        Elen as *mut i64,
        Degree as *mut i64,
        W as *mut i64,
        Control,
        Info,
    );
}
