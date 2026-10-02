extern "C" {
    #[link_name = "honest_osqp_amd_2"]
    fn amd_2(
        n: ::core::ffi::c_int,
        Pe: *mut ::core::ffi::c_int,
        Iw: *mut ::core::ffi::c_int,
        Len: *mut ::core::ffi::c_int,
        iwlen: ::core::ffi::c_int,
        pfree: ::core::ffi::c_int,
        Nv: *mut ::core::ffi::c_int,
        Next: *mut ::core::ffi::c_int,
        Last: *mut ::core::ffi::c_int,
        Head: *mut ::core::ffi::c_int,
        Elen: *mut ::core::ffi::c_int,
        Degree: *mut ::core::ffi::c_int,
        W: *mut ::core::ffi::c_int,
        Control: *mut OSQPFloat,
        Info: *mut OSQPFloat,
    );
}
pub type OSQPFloat = ::core::ffi::c_double;
#[export_name = "honest_osqp_amd_1"]
pub unsafe extern "C" fn amd_1(
    mut n: ::core::ffi::c_int,
    mut Ap: *const ::core::ffi::c_int,
    mut Ai: *const ::core::ffi::c_int,
    mut P: *mut ::core::ffi::c_int,
    mut Pinv: *mut ::core::ffi::c_int,
    mut Len: *mut ::core::ffi::c_int,
    mut slen: ::core::ffi::c_int,
    mut S: *mut ::core::ffi::c_int,
    mut Control: *mut OSQPFloat,
    mut Info: *mut OSQPFloat,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut pfree: ::core::ffi::c_int = 0;
    let mut iwlen: ::core::ffi::c_int = 0;
    let mut pj: ::core::ffi::c_int = 0;
    let mut p1: ::core::ffi::c_int = 0;
    let mut p2: ::core::ffi::c_int = 0;
    let mut pj2: ::core::ffi::c_int = 0;
    let mut Iw: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Pe: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Nv: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Head: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Elen: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Degree: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut s: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut W: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Sp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut Tp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    iwlen = slen - 6 as ::core::ffi::c_int * n;
    s = S as *mut ::core::ffi::c_int;
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
    pfree = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < n {
        *Pe.offset(j as isize) = pfree;
        *Sp.offset(j as isize) = pfree;
        pfree += *Len.offset(j as isize);
        j += 1;
    }
    k = 0 as ::core::ffi::c_int;
    while k < n {
        p1 = *Ap.offset(k as isize);
        p2 = *Ap.offset((k + 1 as ::core::ffi::c_int) as isize);
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
                pj2 = *Ap.offset((j + 1 as ::core::ffi::c_int) as isize);
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
    j = 0 as ::core::ffi::c_int;
    while j < n {
        pj = *Tp.offset(j as isize);
        while pj < *Ap.offset((j + 1 as ::core::ffi::c_int) as isize) {
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
    amd_2(
        n,
        Pe as *mut ::core::ffi::c_int,
        Iw as *mut ::core::ffi::c_int,
        Len,
        iwlen,
        pfree,
        Nv as *mut ::core::ffi::c_int,
        Pinv,
        P,
        Head as *mut ::core::ffi::c_int,
        Elen as *mut ::core::ffi::c_int,
        Degree as *mut ::core::ffi::c_int,
        W as *mut ::core::ffi::c_int,
        Control,
        Info,
    );
}
