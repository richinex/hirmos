extern "C" {
    static mut R_NaN: ::core::ffi::c_double;
    fn sparseMV(A: *mut spmat, x: *mut pfloat, y: *mut pfloat, a: idxint, newVector: idxint);
    fn sparseMtVm(
        A: *mut spmat,
        x: *mut pfloat,
        y: *mut pfloat,
        newVector: idxint,
        skipDiagonal: idxint,
    );
    fn vadd(n: idxint, x: *mut pfloat, y: *mut pfloat);
    fn norminf(v: *mut pfloat, n: idxint) -> pfloat;
    fn scale2add(x: *mut pfloat, y: *mut pfloat, C: *mut cone);
    fn getSOCDetails(
        soc: *mut socone,
        conesize: *mut idxint,
        eta_square: *mut pfloat,
        d1: *mut pfloat,
        u0: *mut pfloat,
        u1: *mut pfloat,
        v1: *mut pfloat,
        q: *mut *mut pfloat,
    );
    fn unstretch(
        n: idxint,
        p: idxint,
        C: *mut cone,
        Pinv: *mut idxint,
        Px: *mut pfloat,
        dx: *mut pfloat,
        dy: *mut pfloat,
        dz: *mut pfloat,
    );
    fn ldl_l_numeric2(
        n: i64,
        Ap: *mut i64,
        Ai: *mut i64,
        Ax: *mut ::core::ffi::c_double,
        Lp: *mut i64,
        Parent: *mut i64,
        Sign: *mut i64,
        eps: ::core::ffi::c_double,
        delta: ::core::ffi::c_double,
        Lnz: *mut i64,
        Li: *mut i64,
        Lx: *mut ::core::ffi::c_double,
        D: *mut ::core::ffi::c_double,
        Y: *mut ::core::ffi::c_double,
        Pattern: *mut i64,
        Flag: *mut i64,
    ) -> i64;
    fn ldl_l_lsolve2(
        n: i64,
        B: *mut ::core::ffi::c_double,
        Lp: *mut i64,
        Li: *mut i64,
        Lx: *mut ::core::ffi::c_double,
        X: *mut ::core::ffi::c_double,
    );
    fn ldl_l_dsolve(
        n: i64,
        X: *mut ::core::ffi::c_double,
        D: *mut ::core::ffi::c_double,
    );
    fn ldl_l_ltsolve(
        n: i64,
        X: *mut ::core::ffi::c_double,
        Lp: *mut i64,
        Li: *mut i64,
        Lx: *mut ::core::ffi::c_double,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lpcone {
    pub p: idxint,
    pub w: *mut pfloat,
    pub v: *mut pfloat,
    pub kkt_idx: *mut idxint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct socone {
    pub p: idxint,
    pub skbar: *mut pfloat,
    pub zkbar: *mut pfloat,
    pub a: pfloat,
    pub d1: pfloat,
    pub w: pfloat,
    pub eta: pfloat,
    pub eta_square: pfloat,
    pub q: *mut pfloat,
    pub Didx: *mut idxint,
    pub u0: pfloat,
    pub u1: pfloat,
    pub v1: pfloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cone {
    pub lpc: *mut lpcone,
    pub soc: *mut socone,
    pub nsoc: idxint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kkt {
    pub PKPt: *mut spmat,
    pub L: *mut spmat,
    pub D: *mut pfloat,
    pub work1: *mut pfloat,
    pub work2: *mut pfloat,
    pub work3: *mut pfloat,
    pub work4: *mut pfloat,
    pub work5: *mut pfloat,
    pub work6: *mut pfloat,
    pub RHS1: *mut pfloat,
    pub RHS2: *mut pfloat,
    pub dx1: *mut pfloat,
    pub dx2: *mut pfloat,
    pub dy1: *mut pfloat,
    pub dy2: *mut pfloat,
    pub dz1: *mut pfloat,
    pub dz2: *mut pfloat,
    pub P: *mut idxint,
    pub Pinv: *mut idxint,
    pub PK: *mut idxint,
    pub Parent: *mut idxint,
    pub Sign: *mut idxint,
    pub Pattern: *mut idxint,
    pub Flag: *mut idxint,
    pub Lnz: *mut idxint,
    pub delta: pfloat,
}
pub const KKT_PROBLEM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KKT_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DELTASTAT: ::core::ffi::c_double = 7E-8f64;
pub const IRERRFACT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LINSYSACC: ::core::ffi::c_double = 1E-14f64;
#[no_mangle]
pub unsafe extern "C" fn kkt_factor(
    mut KKT: *mut kkt,
    mut eps: pfloat,
    mut delta: pfloat,
) -> idxint {
    let mut nd: idxint = 0;
    nd = ldl_l_numeric2(
        (*(*KKT).PKPt).n as i64,
        (*(*KKT).PKPt).jc as *mut i64,
        (*(*KKT).PKPt).ir as *mut i64,
        (*(*KKT).PKPt).pr as *mut ::core::ffi::c_double,
        (*(*KKT).L).jc as *mut i64,
        (*KKT).Parent as *mut i64,
        (*KKT).Sign as *mut i64,
        eps as ::core::ffi::c_double,
        delta as ::core::ffi::c_double,
        (*KKT).Lnz as *mut i64,
        (*(*KKT).L).ir as *mut i64,
        (*(*KKT).L).pr as *mut ::core::ffi::c_double,
        (*KKT).D as *mut ::core::ffi::c_double,
        (*KKT).work1 as *mut ::core::ffi::c_double,
        (*KKT).Pattern as *mut i64,
        (*KKT).Flag as *mut i64,
    ) as idxint;
    return (if nd == (*(*KKT).PKPt).n {
        KKT_OK
    } else {
        KKT_PROBLEM
    }) as idxint;
}
#[no_mangle]
pub unsafe extern "C" fn kkt_solve(
    mut KKT: *mut kkt,
    mut A: *mut spmat,
    mut G: *mut spmat,
    mut Pb: *mut pfloat,
    mut dx: *mut pfloat,
    mut dy: *mut pfloat,
    mut dz: *mut pfloat,
    mut n: idxint,
    mut p: idxint,
    mut m: idxint,
    mut C: *mut cone,
    mut isinit: idxint,
    mut nitref: idxint,
) -> idxint {
    let mut i: idxint = 0;
    let mut k: idxint = 0;
    let mut l: idxint = 0;
    let mut j: idxint = 0;
    let mut kk: idxint = 0;
    let mut kItRef: idxint = 0;
    let mut dzoffset: idxint = 0;
    let mut Pinv: *mut idxint = (*KKT).Pinv;
    let mut Px: *mut pfloat = (*KKT).work1;
    let mut dPx: *mut pfloat = (*KKT).work2;
    let mut e: *mut pfloat = (*KKT).work3;
    let mut Pe: *mut pfloat = (*KKT).work4;
    let mut truez: *mut pfloat = (*KKT).work5;
    let mut Gdx: *mut pfloat = (*KKT).work6;
    let mut ex: *mut pfloat = e;
    let mut ey: *mut pfloat = e.offset(n as isize);
    let mut ez: *mut pfloat = e.offset(n as isize).offset(p as isize);
    let mut bnorm: pfloat = 1.0f64 + norminf(Pb, n + p + (m + 2 as idxint * (*C).nsoc));
    let mut nex: pfloat = 0 as ::core::ffi::c_int as pfloat;
    let mut ney: pfloat = 0 as ::core::ffi::c_int as pfloat;
    let mut nez: pfloat = 0 as ::core::ffi::c_int as pfloat;
    let mut nerr: pfloat = 0.;
    let mut nerr_prev: pfloat = R_NaN;
    let mut error_threshold: pfloat = bnorm * LINSYSACC;
    let mut nK: idxint = (*(*KKT).PKPt).n;
    ldl_l_lsolve2(
        nK as i64,
        Pb as *mut ::core::ffi::c_double,
        (*(*KKT).L).jc as *mut i64,
        (*(*KKT).L).ir as *mut i64,
        (*(*KKT).L).pr as *mut ::core::ffi::c_double,
        Px as *mut ::core::ffi::c_double,
    );
    ldl_l_dsolve(
        nK as i64,
        Px as *mut ::core::ffi::c_double,
        (*KKT).D as *mut ::core::ffi::c_double,
    );
    ldl_l_ltsolve(
        nK as i64,
        Px as *mut ::core::ffi::c_double,
        (*(*KKT).L).jc as *mut i64,
        (*(*KKT).L).ir as *mut i64,
        (*(*KKT).L).pr as *mut ::core::ffi::c_double,
    );
    kItRef = 0 as idxint;
    while kItRef <= nitref {
        unstretch(n, p, C, Pinv, Px, dx, dy, dz);
        k = 0 as idxint;
        j = 0 as idxint;
        i = 0 as idxint;
        while i < n {
            let fresh0 = k;
            k = k + 1;
            *ex.offset(i as isize) = *Pb.offset(*Pinv.offset(fresh0 as isize) as isize)
                - DELTASTAT * *dx.offset(i as isize);
            i += 1;
        }
        if !A.is_null() {
            sparseMtVm(A, dy, ex, 0 as idxint, 0 as idxint);
        }
        sparseMtVm(G, dz, ex, 0 as idxint, 0 as idxint);
        nex = norminf(ex, n);
        if p > 0 as i64 {
            i = 0 as idxint;
            while i < p {
                let fresh1 = k;
                k = k + 1;
                *ey.offset(i as isize) = *Pb.offset(*Pinv.offset(fresh1 as isize) as isize)
                    + DELTASTAT * *dy.offset(i as isize);
                i += 1;
            }
            sparseMV(A, dx, ey, -(1 as ::core::ffi::c_int) as idxint, 0 as idxint);
            ney = norminf(ey, p);
        }
        kk = 0 as idxint;
        j = 0 as idxint;
        dzoffset = 0 as idxint;
        sparseMV(G, dx, Gdx, 1 as idxint, 1 as idxint);
        i = 0 as idxint;
        while i < (*(*C).lpc).p {
            let fresh2 = k;
            k = k + 1;
            let fresh3 = j;
            j = j + 1;
            let fresh4 = dzoffset;
            dzoffset = dzoffset + 1;
            let fresh5 = kk;
            kk = kk + 1;
            *ez.offset(fresh5 as isize) = *Pb.offset(*Pinv.offset(fresh2 as isize) as isize)
                - *Gdx.offset(fresh3 as isize)
                + DELTASTAT * *dz.offset(fresh4 as isize);
            i += 1;
        }
        l = 0 as idxint;
        while l < (*C).nsoc {
            i = 0 as idxint;
            while i < (*(*C).soc.offset(l as isize)).p {
                let fresh12 = kk;
                kk = kk + 1;
                *ez.offset(fresh12 as isize) = (if i
                    < (*(*C).soc.offset(l as isize)).p as i64
                        - 1 as i64
                {
                    let fresh6 = k;
                    k = k + 1;
                    let fresh7 = j;
                    j = j + 1;
                    let fresh8 = dzoffset;
                    dzoffset = dzoffset + 1;
                    *Pb.offset(*Pinv.offset(fresh6 as isize) as isize) as ::core::ffi::c_double
                        - *Gdx.offset(fresh7 as isize) as ::core::ffi::c_double
                        + DELTASTAT * *dz.offset(fresh8 as isize) as ::core::ffi::c_double
                } else {
                    let fresh9 = k;
                    k = k + 1;
                    let fresh10 = j;
                    j = j + 1;
                    let fresh11 = dzoffset;
                    dzoffset = dzoffset + 1;
                    *Pb.offset(*Pinv.offset(fresh9 as isize) as isize) as ::core::ffi::c_double
                        - *Gdx.offset(fresh10 as isize) as ::core::ffi::c_double
                        - DELTASTAT * *dz.offset(fresh11 as isize) as ::core::ffi::c_double
                }) as pfloat;
                i += 1;
            }
            *ez.offset(kk as isize) = 0 as ::core::ffi::c_int as pfloat;
            *ez.offset((kk as i64 + 1 as i64) as isize) =
                0 as ::core::ffi::c_int as pfloat;
            k += 2 as i64;
            kk += 2 as i64;
            l += 1;
        }
        i = 0 as idxint;
        while i < m + 2 as idxint * (*C).nsoc {
            *truez.offset(i as isize) = *Px.offset(*Pinv.offset((n + p + i) as isize) as isize);
            i += 1;
        }
        if isinit == 0 as i64 {
            scale2add(truez, ez, C);
        } else {
            vadd(m + 2 as idxint * (*C).nsoc, truez, ez);
        }
        nez = norminf(ez, m + 2 as idxint * (*C).nsoc);
        nerr = (if nex < nez {
            nez as ::core::ffi::c_double
        } else {
            nex as ::core::ffi::c_double
        }) as pfloat;
        if p > 0 as i64 {
            nerr = (if nerr < ney {
                ney as ::core::ffi::c_double
            } else {
                nerr as ::core::ffi::c_double
            }) as pfloat;
        }
        if kItRef > 0 as i64 && nerr > nerr_prev {
            i = 0 as idxint;
            while i < nK {
                let ref mut fresh13 = *Px.offset(i as isize);
                *fresh13 -= *dPx.offset(i as isize) as ::core::ffi::c_double;
                i += 1;
            }
            kItRef -= 1;
            break;
        } else {
            if kItRef == nitref
                || nerr < error_threshold
                || kItRef > 0 as i64 && nerr_prev < IRERRFACT as pfloat * nerr
            {
                break;
            }
            nerr_prev = nerr;
            i = 0 as idxint;
            while i < nK {
                *Pe.offset(*Pinv.offset(i as isize) as isize) = *e.offset(i as isize);
                i += 1;
            }
            ldl_l_lsolve2(
                nK as i64,
                Pe as *mut ::core::ffi::c_double,
                (*(*KKT).L).jc as *mut i64,
                (*(*KKT).L).ir as *mut i64,
                (*(*KKT).L).pr as *mut ::core::ffi::c_double,
                dPx as *mut ::core::ffi::c_double,
            );
            ldl_l_dsolve(
                nK as i64,
                dPx as *mut ::core::ffi::c_double,
                (*KKT).D as *mut ::core::ffi::c_double,
            );
            ldl_l_ltsolve(
                nK as i64,
                dPx as *mut ::core::ffi::c_double,
                (*(*KKT).L).jc as *mut i64,
                (*(*KKT).L).ir as *mut i64,
                (*(*KKT).L).pr as *mut ::core::ffi::c_double,
            );
            i = 0 as idxint;
            while i < nK {
                let ref mut fresh14 = *Px.offset(i as isize);
                *fresh14 += *dPx.offset(i as isize) as ::core::ffi::c_double;
                i += 1;
            }
            kItRef += 1;
        }
    }
    unstretch(n, p, C, Pinv, Px, dx, dy, dz);
    return kItRef;
}
#[no_mangle]
pub unsafe extern "C" fn kkt_update(mut PKP: *mut spmat, mut P: *mut idxint, mut C: *mut cone) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    let mut conesize: idxint = 0;
    let mut eta_square: pfloat = 0.;
    let mut q: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut d1: pfloat = 0.;
    let mut u0: pfloat = 0.;
    let mut u1: pfloat = 0.;
    let mut v1: pfloat = 0.;
    let mut conesize_m1: idxint = 0;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        *(*PKP)
            .pr
            .offset(*P.offset(*(*(*C).lpc).kkt_idx.offset(i as isize) as isize) as isize) =
            (-(*(*(*C).lpc).v.offset(i as isize) as ::core::ffi::c_double) - DELTASTAT) as pfloat;
        i += 1;
    }
    i = 0 as idxint;
    while i < (*C).nsoc {
        getSOCDetails(
            (*C).soc.offset(i as isize) as *mut socone,
            &raw mut conesize,
            &raw mut eta_square,
            &raw mut d1,
            &raw mut u0,
            &raw mut u1,
            &raw mut v1,
            &raw mut q,
        );
        conesize_m1 = (conesize as i64 - 1 as i64) as idxint;
        *(*PKP).pr.offset(
            *P.offset(
                *(*(*C).soc.offset(i as isize))
                    .Didx
                    .offset(0 as ::core::ffi::c_int as isize) as isize,
            ) as isize,
        ) = (-(eta_square as ::core::ffi::c_double) * d1 as ::core::ffi::c_double - DELTASTAT)
            as pfloat;
        k = 1 as idxint;
        while k < conesize {
            *(*PKP).pr.offset(
                *P.offset(*(*(*C).soc.offset(i as isize)).Didx.offset(k as isize) as isize)
                    as isize,
            ) = (-(eta_square as ::core::ffi::c_double) - DELTASTAT) as pfloat;
            k += 1;
        }
        j = 1 as idxint;
        k = 0 as idxint;
        while k < conesize_m1 {
            let fresh15 = j;
            j = j + 1;
            *(*PKP).pr.offset(
                *P.offset(
                    (*(*(*C).soc.offset(i as isize))
                        .Didx
                        .offset(conesize_m1 as isize)
                        + fresh15) as isize,
                ) as isize,
            ) = -eta_square * v1 * *q.offset(k as isize);
            k += 1;
        }
        let fresh16 = j;
        j = j + 1;
        *(*PKP).pr.offset(
            *P.offset(
                (*(*(*C).soc.offset(i as isize))
                    .Didx
                    .offset(conesize_m1 as isize)
                    + fresh16) as isize,
            ) as isize,
        ) = -eta_square;
        let fresh17 = j;
        j = j + 1;
        *(*PKP).pr.offset(
            *P.offset(
                (*(*(*C).soc.offset(i as isize))
                    .Didx
                    .offset(conesize_m1 as isize)
                    + fresh17) as isize,
            ) as isize,
        ) = -eta_square * u0;
        k = 0 as idxint;
        while k < conesize_m1 {
            let fresh18 = j;
            j = j + 1;
            *(*PKP).pr.offset(
                *P.offset(
                    (*(*(*C).soc.offset(i as isize))
                        .Didx
                        .offset(conesize_m1 as isize)
                        + fresh18) as isize,
                ) as isize,
            ) = -eta_square * u1 * *q.offset(k as isize);
            k += 1;
        }
        let fresh19 = j;
        j = j + 1;
        *(*PKP).pr.offset(
            *P.offset(
                (*(*(*C).soc.offset(i as isize))
                    .Didx
                    .offset(conesize_m1 as isize)
                    + fresh19) as isize,
            ) as isize,
        ) = (eta_square as ::core::ffi::c_double + DELTASTAT) as pfloat;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn kkt_init(mut PKP: *mut spmat, mut P: *mut idxint, mut C: *mut cone) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    let mut conesize: idxint = 0;
    let mut eta_square: pfloat = 0.;
    let mut q: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut d1: pfloat = 0.;
    let mut u0: pfloat = 0.;
    let mut u1: pfloat = 0.;
    let mut v1: pfloat = 0.;
    let mut conesize_m1: idxint = 0;
    i = 0 as idxint;
    while i < (*(*C).lpc).p {
        *(*PKP)
            .pr
            .offset(*P.offset(*(*(*C).lpc).kkt_idx.offset(i as isize) as isize) as isize) =
            -1.0f64 as pfloat;
        i += 1;
    }
    i = 0 as idxint;
    while i < (*C).nsoc {
        getSOCDetails(
            (*C).soc.offset(i as isize) as *mut socone,
            &raw mut conesize,
            &raw mut eta_square,
            &raw mut d1,
            &raw mut u0,
            &raw mut u1,
            &raw mut v1,
            &raw mut q,
        );
        conesize_m1 = (conesize as i64 - 1 as i64) as idxint;
        *(*PKP).pr.offset(
            *P.offset(
                *(*(*C).soc.offset(i as isize))
                    .Didx
                    .offset(0 as ::core::ffi::c_int as isize) as isize,
            ) as isize,
        ) = -1.0f64 as pfloat;
        k = 1 as idxint;
        while k < conesize {
            *(*PKP).pr.offset(
                *P.offset(*(*(*C).soc.offset(i as isize)).Didx.offset(k as isize) as isize)
                    as isize,
            ) = -1.0f64 as pfloat;
            k += 1;
        }
        j = 1 as idxint;
        k = 0 as idxint;
        while k < conesize_m1 {
            let fresh20 = j;
            j = j + 1;
            *(*PKP).pr.offset(
                *P.offset(
                    (*(*(*C).soc.offset(i as isize))
                        .Didx
                        .offset(conesize_m1 as isize)
                        + fresh20) as isize,
                ) as isize,
            ) = 0.0f64 as pfloat;
            k += 1;
        }
        let fresh21 = j;
        j = j + 1;
        *(*PKP).pr.offset(
            *P.offset(
                (*(*(*C).soc.offset(i as isize))
                    .Didx
                    .offset(conesize_m1 as isize)
                    + fresh21) as isize,
            ) as isize,
        ) = -1.0f64 as pfloat;
        let fresh22 = j;
        j = j + 1;
        *(*PKP).pr.offset(
            *P.offset(
                (*(*(*C).soc.offset(i as isize))
                    .Didx
                    .offset(conesize_m1 as isize)
                    + fresh22) as isize,
            ) as isize,
        ) = 0.0f64 as pfloat;
        k = 0 as idxint;
        while k < conesize_m1 {
            let fresh23 = j;
            j = j + 1;
            *(*PKP).pr.offset(
                *P.offset(
                    (*(*(*C).soc.offset(i as isize))
                        .Didx
                        .offset(conesize_m1 as isize)
                        + fresh23) as isize,
                ) as isize,
            ) = 0.0f64 as pfloat;
            k += 1;
        }
        let fresh24 = j;
        j = j + 1;
        *(*PKP).pr.offset(
            *P.offset(
                (*(*(*C).soc.offset(i as isize))
                    .Didx
                    .offset(conesize_m1 as isize)
                    + fresh24) as isize,
            ) as isize,
        ) = 1.0f64 as pfloat;
        i += 1;
    }
}
