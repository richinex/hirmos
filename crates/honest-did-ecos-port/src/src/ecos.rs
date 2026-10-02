use crate::runtime::{sqrt};
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
    fn vsubscale(n: idxint, a: pfloat, x: *mut pfloat, y: *mut pfloat);
    fn norm2(v: *mut pfloat, n: idxint) -> pfloat;
    fn eddot(n: idxint, x: *mut pfloat, y: *mut pfloat) -> pfloat;
    fn bring2cone(C: *mut cone, r: *mut pfloat, s: *mut pfloat);
    fn updateScalings(C: *mut cone, s: *mut pfloat, z: *mut pfloat, lambda: *mut pfloat) -> idxint;
    fn scale(z: *mut pfloat, C: *mut cone, lambda: *mut pfloat);
    fn conicProduct(u: *mut pfloat, v: *mut pfloat, C: *mut cone, w: *mut pfloat) -> pfloat;
    fn conicDivision(u: *mut pfloat, v: *mut pfloat, C: *mut cone, w: *mut pfloat);
    fn kkt_factor(KKT: *mut kkt, eps: pfloat, delta: pfloat) -> idxint;
    fn kkt_solve(
        KKT: *mut kkt,
        A: *mut spmat,
        G: *mut spmat,
        Pb: *mut pfloat,
        dx: *mut pfloat,
        dy: *mut pfloat,
        dz: *mut pfloat,
        n: idxint,
        p: idxint,
        m: idxint,
        C: *mut cone,
        isinit: idxint,
        nitref: idxint,
    ) -> idxint;
    fn kkt_update(PKP: *mut spmat, P: *mut idxint, C: *mut cone);
    fn kkt_init(PKP: *mut spmat, P: *mut idxint, C: *mut cone);
    fn set_equilibration(w: *mut pwork);
    fn unset_equilibration(w: *mut pwork);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct settings {
    pub gamma: pfloat,
    pub delta: pfloat,
    pub eps: pfloat,
    pub feastol: pfloat,
    pub abstol: pfloat,
    pub reltol: pfloat,
    pub feastol_inacc: pfloat,
    pub abstol_inacc: pfloat,
    pub reltol_inacc: pfloat,
    pub nitref: idxint,
    pub maxit: idxint,
    pub verbose: idxint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub pcost: pfloat,
    pub dcost: pfloat,
    pub pres: pfloat,
    pub dres: pfloat,
    pub pinf: pfloat,
    pub dinf: pfloat,
    pub pinfres: pfloat,
    pub dinfres: pfloat,
    pub gap: pfloat,
    pub relgap: pfloat,
    pub sigma: pfloat,
    pub mu: pfloat,
    pub step: pfloat,
    pub step_aff: pfloat,
    pub kapovert: pfloat,
    pub iter: idxint,
    pub nitref1: idxint,
    pub nitref2: idxint,
    pub nitref3: idxint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pwork {
    pub n: idxint,
    pub m: idxint,
    pub p: idxint,
    pub D: idxint,
    pub x: *mut pfloat,
    pub y: *mut pfloat,
    pub z: *mut pfloat,
    pub s: *mut pfloat,
    pub lambda: *mut pfloat,
    pub kap: pfloat,
    pub tau: pfloat,
    pub best_x: *mut pfloat,
    pub best_y: *mut pfloat,
    pub best_z: *mut pfloat,
    pub best_s: *mut pfloat,
    pub best_kap: pfloat,
    pub best_tau: pfloat,
    pub best_cx: pfloat,
    pub best_by: pfloat,
    pub best_hz: pfloat,
    pub best_info: *mut stats,
    pub dsaff: *mut pfloat,
    pub dzaff: *mut pfloat,
    pub W_times_dzaff: *mut pfloat,
    pub dsaff_by_W: *mut pfloat,
    pub saff: *mut pfloat,
    pub zaff: *mut pfloat,
    pub C: *mut cone,
    pub A: *mut spmat,
    pub G: *mut spmat,
    pub c: *mut pfloat,
    pub b: *mut pfloat,
    pub h: *mut pfloat,
    pub AtoK: *mut idxint,
    pub GtoK: *mut idxint,
    pub xequil: *mut pfloat,
    pub Aequil: *mut pfloat,
    pub Gequil: *mut pfloat,
    pub resx0: pfloat,
    pub resy0: pfloat,
    pub resz0: pfloat,
    pub rx: *mut pfloat,
    pub ry: *mut pfloat,
    pub rz: *mut pfloat,
    pub rt: pfloat,
    pub hresx: pfloat,
    pub hresy: pfloat,
    pub hresz: pfloat,
    pub nx: pfloat,
    pub ny: pfloat,
    pub nz: pfloat,
    pub ns: pfloat,
    pub cx: pfloat,
    pub by: pfloat,
    pub hz: pfloat,
    pub sz: pfloat,
    pub KKT: *mut kkt,
    pub info: *mut stats,
    pub stgs: *mut settings,
}
#[inline(always)]
unsafe extern "C" fn __inline_isnanf(mut __x: ::core::ffi::c_float) -> ::core::ffi::c_int {
    return (__x != __x) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnand(mut __x: ::core::ffi::c_double) -> ::core::ffi::c_int {
    return (__x != __x) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnanl(mut __x: f64) -> ::core::ffi::c_int {
    return (__x != __x) as ::core::ffi::c_int;
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const OUTSIDE_CONE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KKT_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ECOS_VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"2.0.7\0") };
pub const GAMMA: ::core::ffi::c_double = 0.99f64;
pub const EPS: ::core::ffi::c_double = 1E-13f64;
pub const SIGMAMIN: ::core::ffi::c_double = 1E-4f64;
pub const SIGMAMAX: ::core::ffi::c_double = 1.0f64;
pub const STEPMIN: ::core::ffi::c_double = 1E-6f64;
pub const STEPMAX: ::core::ffi::c_double = 0.999f64;
pub const SAFEGUARD: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const ECOS_OPTIMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ECOS_PINF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ECOS_DINF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ECOS_INACC_OFFSET: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const ECOS_MAXIT: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const ECOS_NUMERICS: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const ECOS_OUTCONE: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const ECOS_SIGINT: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const ECOS_FATAL: ::core::ffi::c_int = -(7 as ::core::ffi::c_int);
pub const ECOS_NOT_CONVERGED_YET: ::core::ffi::c_int = -(87 as ::core::ffi::c_int);
static mut thisVersion: *const ::core::ffi::c_char = ECOS_VERSION.as_ptr();
#[no_mangle]
pub unsafe extern "C" fn ECOS_ver() -> *const ::core::ffi::c_char {
    return thisVersion;
}
#[no_mangle]
pub unsafe extern "C" fn compareStatistics(mut infoA: *mut stats, mut infoB: *mut stats) -> idxint {
    if (*infoA).pinfres != R_NaN
        && (*infoA).kapovert > 1 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if (*infoB).pinfres != R_NaN {
            if (*infoA).gap > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && (*infoB).gap > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && (*infoA).gap < (*infoB).gap
                && ((*infoA).pinfres > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && (*infoA).pinfres < (*infoB).pres)
                && ((*infoA).mu > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && (*infoA).mu < (*infoB).mu)
            {
                return 1 as idxint;
            } else {
                return 0 as idxint;
            }
        } else if (*infoA).gap > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*infoB).gap > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*infoA).gap < (*infoB).gap
            && ((*infoA).mu > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && (*infoA).mu < (*infoB).mu)
        {
            return 1 as idxint;
        } else {
            return 0 as idxint;
        }
    } else if (*infoA).gap > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*infoB).gap > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*infoA).gap < (*infoB).gap
        && ((*infoA).pres > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*infoA).pres < (*infoB).pres)
        && ((*infoA).dres > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*infoA).dres < (*infoB).dres)
        && ((*infoA).kapovert > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*infoA).kapovert < (*infoB).kapovert)
        && ((*infoA).mu > 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*infoA).mu < (*infoB).mu)
    {
        return 1 as idxint;
    } else {
        return 0 as idxint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn saveIterateAsBest(mut w: *mut pwork) {
    let mut i: idxint = 0;
    i = 0 as idxint;
    while i < (*w).n {
        *(*w).best_x.offset(i as isize) = *(*w).x.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).p {
        *(*w).best_y.offset(i as isize) = *(*w).y.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).m {
        *(*w).best_z.offset(i as isize) = *(*w).z.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).m {
        *(*w).best_s.offset(i as isize) = *(*w).s.offset(i as isize);
        i += 1;
    }
    (*w).best_kap = (*w).kap;
    (*w).best_tau = (*w).tau;
    (*w).best_cx = (*w).cx;
    (*w).best_by = (*w).by;
    (*w).best_hz = (*w).hz;
    (*(*w).best_info).pcost = (*(*w).info).pcost;
    (*(*w).best_info).dcost = (*(*w).info).dcost;
    (*(*w).best_info).pres = (*(*w).info).pres;
    (*(*w).best_info).dres = (*(*w).info).dres;
    (*(*w).best_info).pinfres = (*(*w).info).pinfres;
    (*(*w).best_info).dinfres = (*(*w).info).dinfres;
    (*(*w).best_info).gap = (*(*w).info).gap;
    (*(*w).best_info).relgap = (*(*w).info).relgap;
    (*(*w).best_info).mu = (*(*w).info).mu;
    (*(*w).best_info).kapovert = (*(*w).info).kapovert;
    (*(*w).best_info).iter = (*(*w).info).iter;
}
#[no_mangle]
pub unsafe extern "C" fn restoreBestIterate(mut w: *mut pwork) {
    let mut i: idxint = 0;
    i = 0 as idxint;
    while i < (*w).n {
        *(*w).x.offset(i as isize) = *(*w).best_x.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).p {
        *(*w).y.offset(i as isize) = *(*w).best_y.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).m {
        *(*w).z.offset(i as isize) = *(*w).best_z.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).m {
        *(*w).s.offset(i as isize) = *(*w).best_s.offset(i as isize);
        i += 1;
    }
    (*w).kap = (*w).best_kap;
    (*w).tau = (*w).best_tau;
    (*w).cx = (*w).best_cx;
    (*w).by = (*w).best_by;
    (*w).hz = (*w).best_hz;
    (*(*w).info).pcost = (*(*w).best_info).pcost;
    (*(*w).info).dcost = (*(*w).best_info).dcost;
    (*(*w).info).pres = (*(*w).best_info).pres;
    (*(*w).info).dres = (*(*w).best_info).dres;
    (*(*w).info).pinfres = (*(*w).best_info).pinfres;
    (*(*w).info).dinfres = (*(*w).best_info).dinfres;
    (*(*w).info).gap = (*(*w).best_info).gap;
    (*(*w).info).relgap = (*(*w).best_info).relgap;
    (*(*w).info).mu = (*(*w).best_info).mu;
    (*(*w).info).kapovert = (*(*w).best_info).kapovert;
}
#[no_mangle]
pub unsafe extern "C" fn checkExitConditions(mut w: *mut pwork, mut mode: idxint) -> idxint {
    let mut feastol: pfloat = 0.;
    let mut abstol: pfloat = 0.;
    let mut reltol: pfloat = 0.;
    if mode == 0 as i64 {
        feastol = (*(*w).stgs).feastol;
        abstol = (*(*w).stgs).abstol;
        reltol = (*(*w).stgs).reltol;
    } else {
        feastol = (*(*w).stgs).feastol_inacc;
        abstol = (*(*w).stgs).abstol_inacc;
        reltol = (*(*w).stgs).reltol_inacc;
    }
    if (-(*w).cx > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        || -(*w).by - (*w).hz >= -abstol)
        && ((*(*w).info).pres < feastol && (*(*w).info).dres < feastol)
        && ((*(*w).info).gap < abstol || (*(*w).info).relgap < reltol)
    {
        (*(*w).info).pinf = 0 as ::core::ffi::c_int as pfloat;
        (*(*w).info).dinf = 0 as ::core::ffi::c_int as pfloat;
        return ECOS_OPTIMAL as idxint + mode;
    } else if (*(*w).info).dinfres != R_NaN && (*(*w).info).dinfres < feastol && (*w).tau < (*w).kap
    {
        (*(*w).info).pinf = 0 as ::core::ffi::c_int as pfloat;
        (*(*w).info).dinf = 1 as ::core::ffi::c_int as pfloat;
        return ECOS_DINF as idxint + mode;
    } else if (*(*w).info).pinfres != R_NaN && (*(*w).info).pinfres < feastol && (*w).tau < (*w).kap
        || (*w).tau < (*(*w).stgs).feastol
            && (*w).kap < (*(*w).stgs).feastol
            && (*(*w).info).pinfres < (*(*w).stgs).feastol
    {
        (*(*w).info).pinf = 1 as ::core::ffi::c_int as pfloat;
        (*(*w).info).dinf = 0 as ::core::ffi::c_int as pfloat;
        return ECOS_PINF as idxint + mode;
    } else {
        return ECOS_NOT_CONVERGED_YET as idxint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn init(mut w: *mut pwork) -> idxint {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    let mut l: idxint = 0;
    let mut KKT_FACTOR_RETURN_CODE: idxint = 0;
    let mut Pinv: *mut idxint = (*(*w).KKT).Pinv;
    let mut rx: pfloat = 0.;
    let mut ry: pfloat = 0.;
    let mut rz: pfloat = 0.;
    (*(*w).KKT).delta = (*(*w).stgs).delta;
    kkt_init((*(*w).KKT).PKPt, (*(*w).KKT).PK, (*w).C);
    k = 0 as idxint;
    j = 0 as idxint;
    i = 0 as idxint;
    while i < (*w).n {
        let fresh30 = k;
        k = k + 1;
        *(*(*w).KKT)
            .RHS1
            .offset(*(*(*w).KKT).Pinv.offset(fresh30 as isize) as isize) =
            0 as ::core::ffi::c_int as pfloat;
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).p {
        let fresh31 = k;
        k = k + 1;
        *(*(*w).KKT)
            .RHS1
            .offset(*(*(*w).KKT).Pinv.offset(fresh31 as isize) as isize) =
            *(*w).b.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*(*(*w).C).lpc).p {
        let fresh32 = k;
        k = k + 1;
        *(*(*w).KKT)
            .RHS1
            .offset(*(*(*w).KKT).Pinv.offset(fresh32 as isize) as isize) =
            *(*w).h.offset(i as isize);
        j += 1;
        i += 1;
    }
    l = 0 as idxint;
    while l < (*(*w).C).nsoc {
        i = 0 as idxint;
        while i < (*(*(*w).C).soc.offset(l as isize)).p {
            let fresh33 = j;
            j = j + 1;
            let fresh34 = k;
            k = k + 1;
            *(*(*w).KKT)
                .RHS1
                .offset(*(*(*w).KKT).Pinv.offset(fresh34 as isize) as isize) =
                *(*w).h.offset(fresh33 as isize);
            i += 1;
        }
        let fresh35 = k;
        k = k + 1;
        *(*(*w).KKT)
            .RHS1
            .offset(*(*(*w).KKT).Pinv.offset(fresh35 as isize) as isize) =
            0 as ::core::ffi::c_int as pfloat;
        let fresh36 = k;
        k = k + 1;
        *(*(*w).KKT)
            .RHS1
            .offset(*(*(*w).KKT).Pinv.offset(fresh36 as isize) as isize) =
            0 as ::core::ffi::c_int as pfloat;
        l += 1;
    }
    i = 0 as idxint;
    while i < (*w).n {
        *(*(*w).KKT)
            .RHS2
            .offset(*(*(*w).KKT).Pinv.offset(i as isize) as isize) = -*(*w).c.offset(i as isize);
        i += 1;
    }
    i = (*w).n;
    while i < (*(*(*w).KKT).PKPt).n {
        *(*(*w).KKT)
            .RHS2
            .offset(*(*(*w).KKT).Pinv.offset(i as isize) as isize) =
            0 as ::core::ffi::c_int as pfloat;
        i += 1;
    }
    rx = norm2((*w).c, (*w).n);
    (*w).resx0 = (if (1 as ::core::ffi::c_int as ::core::ffi::c_double) < rx {
        rx as ::core::ffi::c_double
    } else {
        1 as ::core::ffi::c_int as ::core::ffi::c_double
    }) as pfloat;
    ry = norm2((*w).b, (*w).p);
    (*w).resy0 = (if (1 as ::core::ffi::c_int as ::core::ffi::c_double) < ry {
        ry as ::core::ffi::c_double
    } else {
        1 as ::core::ffi::c_int as ::core::ffi::c_double
    }) as pfloat;
    rz = norm2((*w).h, (*w).m);
    (*w).resz0 = (if (1 as ::core::ffi::c_int as ::core::ffi::c_double) < rz {
        rz as ::core::ffi::c_double
    } else {
        1 as ::core::ffi::c_int as ::core::ffi::c_double
    }) as pfloat;
    KKT_FACTOR_RETURN_CODE = kkt_factor((*w).KKT, (*(*w).stgs).eps, (*(*w).stgs).delta);
    if KKT_FACTOR_RETURN_CODE != KKT_OK as i64 {
        return ECOS_FATAL as idxint;
    }
    (*(*w).info).nitref1 = kkt_solve(
        (*w).KKT,
        (*w).A,
        (*w).G,
        (*(*w).KKT).RHS1,
        (*(*w).KKT).dx1,
        (*(*w).KKT).dy1,
        (*(*w).KKT).dz1,
        (*w).n,
        (*w).p,
        (*w).m,
        (*w).C,
        1 as idxint,
        (*(*w).stgs).nitref,
    );
    i = 0 as idxint;
    while i < (*w).n {
        *(*w).x.offset(i as isize) = *(*(*w).KKT).dx1.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).m {
        *(*(*w).KKT).work1.offset(i as isize) = -*(*(*w).KKT).dz1.offset(i as isize);
        i += 1;
    }
    bring2cone((*w).C, (*(*w).KKT).work1, (*w).s);
    (*(*w).info).nitref2 = kkt_solve(
        (*w).KKT,
        (*w).A,
        (*w).G,
        (*(*w).KKT).RHS2,
        (*(*w).KKT).dx2,
        (*(*w).KKT).dy2,
        (*(*w).KKT).dz2,
        (*w).n,
        (*w).p,
        (*w).m,
        (*w).C,
        1 as idxint,
        (*(*w).stgs).nitref,
    );
    i = 0 as idxint;
    while i < (*w).p {
        *(*w).y.offset(i as isize) = *(*(*w).KKT).dy2.offset(i as isize);
        i += 1;
    }
    bring2cone((*w).C, (*(*w).KKT).dz2, (*w).z);
    i = 0 as idxint;
    while i < (*w).n {
        *(*(*w).KKT).RHS1.offset(*Pinv.offset(i as isize) as isize) = -*(*w).c.offset(i as isize);
        i += 1;
    }
    (*w).kap = 1.0f64 as pfloat;
    (*w).tau = 1.0f64 as pfloat;
    (*(*w).info).step = 0 as ::core::ffi::c_int as pfloat;
    (*(*w).info).step_aff = 0 as ::core::ffi::c_int as pfloat;
    (*(*w).info).dinf = 0 as ::core::ffi::c_int as pfloat;
    (*(*w).info).pinf = 0 as ::core::ffi::c_int as pfloat;
    return 0 as idxint;
}
#[no_mangle]
pub unsafe extern "C" fn computeResiduals(mut w: *mut pwork) {
    if (*w).p > 0 as i64 {
        sparseMtVm((*w).A, (*w).y, (*w).rx, 1 as idxint, 0 as idxint);
        sparseMtVm((*w).G, (*w).z, (*w).rx, 0 as idxint, 0 as idxint);
    } else {
        sparseMtVm((*w).G, (*w).z, (*w).rx, 1 as idxint, 0 as idxint);
    }
    (*w).hresx = norm2((*w).rx, (*w).n);
    vsubscale((*w).n, (*w).tau, (*w).c, (*w).rx);
    if (*w).p > 0 as i64 {
        sparseMV((*w).A, (*w).x, (*w).ry, 1 as idxint, 1 as idxint);
        (*w).hresy = norm2((*w).ry, (*w).p);
        vsubscale((*w).p, (*w).tau, (*w).b, (*w).ry);
    } else {
        (*w).hresy = 0 as ::core::ffi::c_int as pfloat;
        (*w).ry = ::core::ptr::null_mut::<pfloat>();
    }
    sparseMV((*w).G, (*w).x, (*w).rz, 1 as idxint, 1 as idxint);
    vadd((*w).m, (*w).s, (*w).rz);
    (*w).hresz = norm2((*w).rz, (*w).m);
    vsubscale((*w).m, (*w).tau, (*w).h, (*w).rz);
    (*w).cx = eddot((*w).n, (*w).c, (*w).x);
    (*w).by = (if (*w).p > 0 as i64 {
        eddot((*w).p, (*w).b, (*w).y) as ::core::ffi::c_double
    } else {
        0.0f64
    }) as pfloat;
    (*w).hz = eddot((*w).m, (*w).h, (*w).z);
    (*w).rt = (*w).kap + (*w).cx + (*w).by + (*w).hz;
    (*w).nx = norm2((*w).x, (*w).n);
    (*w).ny = norm2((*w).y, (*w).p);
    (*w).ns = norm2((*w).s, (*w).m);
    (*w).nz = norm2((*w).z, (*w).m);
}
#[no_mangle]
pub unsafe extern "C" fn updateStatistics(mut w: *mut pwork) {
    let mut nry: pfloat = 0.;
    let mut nrz: pfloat = 0.;
    let mut info: *mut stats = (*w).info;
    (*info).gap = eddot((*w).m, (*w).s, (*w).z);
    (*info).mu = (((*info).gap as ::core::ffi::c_double
        + (*w).kap as ::core::ffi::c_double * (*w).tau as ::core::ffi::c_double)
        / ((*w).D as i64 + 1 as i64) as ::core::ffi::c_double)
        as pfloat;
    (*info).kapovert = (*w).kap / (*w).tau;
    (*info).pcost = (*w).cx / (*w).tau;
    (*info).dcost = -((*w).hz + (*w).by) / (*w).tau;
    if (*info).pcost < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        (*info).relgap = (*info).gap / -(*info).pcost;
    } else if (*info).dcost > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        (*info).relgap = (*info).gap / (*info).dcost;
    } else {
        (*info).relgap = R_NaN as pfloat;
    }
    nry = (if (*w).p > 0 as i64 {
        norm2((*w).ry, (*w).p) as ::core::ffi::c_double
            / (if (*w).resy0 + (*w).nx < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                1 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                (*w).resy0 as ::core::ffi::c_double + (*w).nx as ::core::ffi::c_double
            })
    } else {
        0.0f64
    }) as pfloat;
    nrz = (norm2((*w).rz, (*w).m) as ::core::ffi::c_double
        / (if (*w).resz0 + (*w).nx + (*w).ns < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            1 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            (*w).resz0 as ::core::ffi::c_double
                + (*w).nx as ::core::ffi::c_double
                + (*w).ns as ::core::ffi::c_double
        })) as pfloat;
    (*info).pres = (if nry < nrz { nrz } else { nry }) / (*w).tau;
    (*info).dres = norm2((*w).rx, (*w).n)
        / (if (*w).resx0 + (*w).ny + (*w).nz < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            1 as ::core::ffi::c_int as pfloat
        } else {
            (*w).resx0 + (*w).ny + (*w).nz
        })
        / (*w).tau;
    (*info).pinfres = (if ((*w).hz as ::core::ffi::c_double + (*w).by as ::core::ffi::c_double)
        / (if (*w).ny + (*w).nz < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            1 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            (*w).ny as ::core::ffi::c_double + (*w).nz as ::core::ffi::c_double
        })
        < -(*(*w).stgs).reltol
    {
        (*w).hresx as ::core::ffi::c_double
            / (if (*w).ny + (*w).nz < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                1 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                (*w).ny as ::core::ffi::c_double + (*w).nz as ::core::ffi::c_double
            })
    } else {
        R_NaN
    }) as pfloat;
    (*info).dinfres = (if (*w).cx as ::core::ffi::c_double
        / (if (*w).nx < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
            1 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            (*w).nx as ::core::ffi::c_double
        })
        < -(*(*w).stgs).reltol
    {
        if (*w).hresy as ::core::ffi::c_double
            / (if (*w).nx < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                1 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                (*w).nx as ::core::ffi::c_double
            })
            < (*w).hresz as ::core::ffi::c_double
                / (if (*w).nx + (*w).ns < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                    1 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    (*w).nx as ::core::ffi::c_double + (*w).ns as ::core::ffi::c_double
                })
        {
            (*w).hresz as ::core::ffi::c_double
                / (if (*w).nx + (*w).ns < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                    1 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    (*w).nx as ::core::ffi::c_double + (*w).ns as ::core::ffi::c_double
                })
        } else {
            (*w).hresy as ::core::ffi::c_double
                / (if (*w).nx < 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                    1 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    (*w).nx as ::core::ffi::c_double
                })
        }
    } else {
        R_NaN
    }) as pfloat;
}
#[no_mangle]
pub unsafe extern "C" fn RHS_affine(mut w: *mut pwork) {
    let mut RHS: *mut pfloat = (*(*w).KKT).RHS2;
    let mut n: idxint = (*w).n;
    let mut p: idxint = (*w).p;
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    let mut l: idxint = 0;
    let mut Pinv: *mut idxint = (*(*w).KKT).Pinv;
    j = 0 as idxint;
    i = 0 as idxint;
    while i < n {
        let fresh24 = j;
        j = j + 1;
        *RHS.offset(*Pinv.offset(fresh24 as isize) as isize) = *(*w).rx.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < p {
        let fresh25 = j;
        j = j + 1;
        *RHS.offset(*Pinv.offset(fresh25 as isize) as isize) = -*(*w).ry.offset(i as isize);
        i += 1;
    }
    i = 0 as idxint;
    while i < (*(*(*w).C).lpc).p {
        let fresh26 = j;
        j = j + 1;
        *RHS.offset(*Pinv.offset(fresh26 as isize) as isize) =
            *(*w).s.offset(i as isize) - *(*w).rz.offset(i as isize);
        i += 1;
    }
    k = (*(*(*w).C).lpc).p;
    l = 0 as idxint;
    while l < (*(*w).C).nsoc {
        i = 0 as idxint;
        while i < (*(*(*w).C).soc.offset(l as isize)).p {
            let fresh27 = j;
            j = j + 1;
            *RHS.offset(*Pinv.offset(fresh27 as isize) as isize) =
                *(*w).s.offset(k as isize) - *(*w).rz.offset(k as isize);
            k += 1;
            i += 1;
        }
        let fresh28 = j;
        j = j + 1;
        *RHS.offset(*Pinv.offset(fresh28 as isize) as isize) = 0 as ::core::ffi::c_int as pfloat;
        let fresh29 = j;
        j = j + 1;
        *RHS.offset(*Pinv.offset(fresh29 as isize) as isize) = 0 as ::core::ffi::c_int as pfloat;
        l += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RHS_combined(mut w: *mut pwork) {
    let mut ds1: *mut pfloat = (*(*w).KKT).work1;
    let mut ds2: *mut pfloat = (*(*w).KKT).work2;
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    let mut l: idxint = 0;
    let mut sigmamu: pfloat = (*(*w).info).sigma * (*(*w).info).mu;
    let mut one_minus_sigma: pfloat = 1.0f64 - (*(*w).info).sigma;
    let mut Pinv: *mut idxint = (*(*w).KKT).Pinv;
    conicProduct((*w).lambda, (*w).lambda, (*w).C, ds1);
    conicProduct((*w).dsaff_by_W, (*w).W_times_dzaff, (*w).C, ds2);
    i = 0 as idxint;
    while i < (*(*(*w).C).lpc).p {
        let ref mut fresh13 = *ds1.offset(i as isize);
        *fresh13 += (*ds2.offset(i as isize) - sigmamu) as ::core::ffi::c_double;
        i += 1;
    }
    k = (*(*(*w).C).lpc).p;
    i = 0 as idxint;
    while i < (*(*w).C).nsoc {
        let ref mut fresh14 = *ds1.offset(k as isize);
        *fresh14 += (*ds2.offset(k as isize) - sigmamu) as ::core::ffi::c_double;
        k += 1;
        j = 1 as idxint;
        while j < (*(*(*w).C).soc.offset(i as isize)).p {
            let ref mut fresh15 = *ds1.offset(k as isize);
            *fresh15 += *ds2.offset(k as isize) as ::core::ffi::c_double;
            k += 1;
            j += 1;
        }
        i += 1;
    }
    conicDivision((*w).lambda, ds1, (*w).C, (*w).dsaff_by_W);
    scale((*w).dsaff_by_W, (*w).C, ds1);
    j = 0 as idxint;
    i = 0 as idxint;
    while i < (*w).n {
        let fresh16 = j;
        j = j + 1;
        let ref mut fresh17 = *(*(*w).KKT)
            .RHS2
            .offset(*Pinv.offset(fresh16 as isize) as isize);
        *fresh17 *= one_minus_sigma as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).p {
        let fresh18 = j;
        j = j + 1;
        let ref mut fresh19 = *(*(*w).KKT)
            .RHS2
            .offset(*Pinv.offset(fresh18 as isize) as isize);
        *fresh19 *= one_minus_sigma as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as idxint;
    while i < (*(*(*w).C).lpc).p {
        let fresh20 = j;
        j = j + 1;
        *(*(*w).KKT)
            .RHS2
            .offset(*Pinv.offset(fresh20 as isize) as isize) =
            -one_minus_sigma * *(*w).rz.offset(i as isize) + *ds1.offset(i as isize);
        i += 1;
    }
    k = (*(*(*w).C).lpc).p;
    l = 0 as idxint;
    while l < (*(*w).C).nsoc {
        i = 0 as idxint;
        while i < (*(*(*w).C).soc.offset(l as isize)).p {
            let fresh21 = j;
            j = j + 1;
            *(*(*w).KKT)
                .RHS2
                .offset(*Pinv.offset(fresh21 as isize) as isize) =
                -one_minus_sigma * *(*w).rz.offset(k as isize) + *ds1.offset(k as isize);
            k += 1;
            i += 1;
        }
        let fresh22 = j;
        j = j + 1;
        *(*(*w).KKT)
            .RHS2
            .offset(*Pinv.offset(fresh22 as isize) as isize) = 0 as ::core::ffi::c_int as pfloat;
        let fresh23 = j;
        j = j + 1;
        *(*(*w).KKT)
            .RHS2
            .offset(*Pinv.offset(fresh23 as isize) as isize) = 0 as ::core::ffi::c_int as pfloat;
        l += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lineSearch(
    mut lambda: *mut pfloat,
    mut ds: *mut pfloat,
    mut dz: *mut pfloat,
    mut tau: pfloat,
    mut dtau: pfloat,
    mut kap: pfloat,
    mut dkap: pfloat,
    mut C: *mut cone,
    mut KKT: *mut kkt,
) -> pfloat {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut cone_start: idxint = 0;
    let mut conesize: idxint = 0;
    let mut rhomin: pfloat = 0.;
    let mut sigmamin: pfloat = 0.;
    let mut alpha: pfloat = 0.;
    let mut lknorm2: pfloat = 0.;
    let mut lknorm: pfloat = 0.;
    let mut lknorminv: pfloat = 0.;
    let mut rhonorm: pfloat = 0.;
    let mut sigmanorm: pfloat = 0.;
    let mut conic_step: pfloat = 0.;
    let mut temp: pfloat = 0.;
    let mut lkbar_times_dsk: pfloat = 0.;
    let mut lkbar_times_dzk: pfloat = 0.;
    let mut factor: pfloat = 0.;
    let mut lk: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut dsk: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut dzk: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut lkbar: *mut pfloat = (*KKT).work1;
    let mut rho: *mut pfloat = (*KKT).work2;
    let mut sigma: *mut pfloat = (*KKT).work2;
    let mut minus_tau_by_dtau: pfloat = -tau / dtau;
    let mut minus_kap_by_dkap: pfloat = -kap / dkap;
    if (*(*C).lpc).p > 0 as i64 {
        rhomin = *ds.offset(0 as ::core::ffi::c_int as isize)
            / *lambda.offset(0 as ::core::ffi::c_int as isize);
        sigmamin = *dz.offset(0 as ::core::ffi::c_int as isize)
            / *lambda.offset(0 as ::core::ffi::c_int as isize);
        i = 1 as idxint;
        while i < (*(*C).lpc).p {
            *rho.offset(0 as ::core::ffi::c_int as isize) =
                *ds.offset(i as isize) / *lambda.offset(i as isize);
            if *rho.offset(0 as ::core::ffi::c_int as isize) < rhomin {
                rhomin = *rho.offset(0 as ::core::ffi::c_int as isize);
            }
            *sigma.offset(0 as ::core::ffi::c_int as isize) =
                *dz.offset(i as isize) / *lambda.offset(i as isize);
            if *sigma.offset(0 as ::core::ffi::c_int as isize) < sigmamin {
                sigmamin = *sigma.offset(0 as ::core::ffi::c_int as isize);
            }
            i += 1;
        }
        if -sigmamin > -rhomin {
            alpha = (if sigmamin < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                1.0f64 / -(sigmamin as ::core::ffi::c_double)
            } else {
                1.0f64 / EPS
            }) as pfloat;
        } else {
            alpha = (if rhomin < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                1.0f64 / -(rhomin as ::core::ffi::c_double)
            } else {
                1.0f64 / EPS
            }) as pfloat;
        }
    } else {
        alpha = 10 as ::core::ffi::c_int as pfloat;
    }
    if minus_tau_by_dtau > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && minus_tau_by_dtau < alpha
    {
        alpha = minus_tau_by_dtau;
    }
    if minus_kap_by_dkap > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && minus_kap_by_dkap < alpha
    {
        alpha = minus_kap_by_dkap;
    }
    cone_start = (*(*C).lpc).p;
    i = 0 as idxint;
    while i < (*C).nsoc {
        conesize = (*(*C).soc.offset(i as isize)).p;
        lk = lambda.offset(cone_start as isize);
        dsk = ds.offset(cone_start as isize);
        dzk = dz.offset(cone_start as isize);
        lknorm2 = *lk.offset(0 as ::core::ffi::c_int as isize)
            * *lk.offset(0 as ::core::ffi::c_int as isize)
            - eddot(
                conesize - 1 as idxint,
                lk.offset(1 as ::core::ffi::c_int as isize),
                lk.offset(1 as ::core::ffi::c_int as isize),
            );
        if !(lknorm2 <= 0.0f64) {
            lknorm = sqrt(lknorm2 as ::core::ffi::c_double) as pfloat;
            j = 0 as idxint;
            while j < conesize {
                *lkbar.offset(j as isize) = *lk.offset(j as isize) / lknorm;
                j += 1;
            }
            lknorminv = 1.0f64 / lknorm;
            lkbar_times_dsk = *lkbar.offset(0 as ::core::ffi::c_int as isize)
                * *dsk.offset(0 as ::core::ffi::c_int as isize);
            j = 1 as idxint;
            while j < conesize {
                lkbar_times_dsk -=
                    (*lkbar.offset(j as isize) * *dsk.offset(j as isize)) as ::core::ffi::c_double;
                j += 1;
            }
            lkbar_times_dzk = *lkbar.offset(0 as ::core::ffi::c_int as isize)
                * *dzk.offset(0 as ::core::ffi::c_int as isize);
            j = 1 as idxint;
            while j < conesize {
                lkbar_times_dzk -=
                    (*lkbar.offset(j as isize) * *dzk.offset(j as isize)) as ::core::ffi::c_double;
                j += 1;
            }
            *rho.offset(0 as ::core::ffi::c_int as isize) = lknorminv * lkbar_times_dsk;
            factor = ((lkbar_times_dsk as ::core::ffi::c_double
                + *dsk.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                / (*lkbar.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    + 1 as ::core::ffi::c_int as ::core::ffi::c_double))
                as pfloat;
            j = 1 as idxint;
            while j < conesize {
                *rho.offset(j as isize) =
                    lknorminv * (*dsk.offset(j as isize) - factor * *lkbar.offset(j as isize));
                j += 1;
            }
            rhonorm = norm2(
                rho.offset(1 as ::core::ffi::c_int as isize),
                conesize - 1 as idxint,
            ) - *rho.offset(0 as ::core::ffi::c_int as isize);
            *sigma.offset(0 as ::core::ffi::c_int as isize) = lknorminv * lkbar_times_dzk;
            factor = ((lkbar_times_dzk as ::core::ffi::c_double
                + *dzk.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_double)
                / (*lkbar.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
                    + 1 as ::core::ffi::c_int as ::core::ffi::c_double))
                as pfloat;
            j = 1 as idxint;
            while j < conesize {
                *sigma.offset(j as isize) =
                    lknorminv * (*dzk.offset(j as isize) - factor * *lkbar.offset(j as isize));
                j += 1;
            }
            sigmanorm = norm2(
                sigma.offset(1 as ::core::ffi::c_int as isize),
                conesize - 1 as idxint,
            ) - *sigma.offset(0 as ::core::ffi::c_int as isize);
            conic_step = 0 as ::core::ffi::c_int as pfloat;
            if rhonorm > conic_step {
                conic_step = rhonorm;
            }
            if sigmanorm > conic_step {
                conic_step = sigmanorm;
            }
            if conic_step != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                temp = 1.0f64 / conic_step;
                if temp < alpha {
                    alpha = temp;
                }
            }
            cone_start += (*(*C).soc.offset(i as isize)).p as i64;
        }
        i += 1;
    }
    if alpha > STEPMAX {
        alpha = STEPMAX as pfloat;
    }
    if alpha < STEPMIN {
        alpha = STEPMIN as pfloat;
    }
    return alpha;
}
#[no_mangle]
pub unsafe extern "C" fn backscale(mut w: *mut pwork) {
    let mut i: idxint = 0;
    i = 0 as idxint;
    while i < (*w).n {
        let ref mut fresh8 = *(*w).x.offset(i as isize);
        *fresh8 /= (*(*w).xequil.offset(i as isize) * (*w).tau) as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).p {
        let ref mut fresh9 = *(*w).y.offset(i as isize);
        *fresh9 /= (*(*w).Aequil.offset(i as isize) * (*w).tau) as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).m {
        let ref mut fresh10 = *(*w).z.offset(i as isize);
        *fresh10 /= (*(*w).Gequil.offset(i as isize) * (*w).tau) as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).m {
        let ref mut fresh11 = *(*w).s.offset(i as isize);
        *fresh11 *= (*(*w).Gequil.offset(i as isize) / (*w).tau) as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as idxint;
    while i < (*w).n {
        let ref mut fresh12 = *(*w).c.offset(i as isize);
        *fresh12 *= *(*w).xequil.offset(i as isize) as ::core::ffi::c_double;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ECOS_solve(mut w: *mut pwork) -> idxint {
    let mut i: idxint = 0;
    let mut initcode: idxint = 0;
    let mut KKT_FACTOR_RETURN_CODE: idxint = 0;
    let mut dtau_denom: pfloat = 0.;
    let mut dtauaff: pfloat = 0.;
    let mut dkapaff: pfloat = 0.;
    let mut sigma: pfloat = 0.;
    let mut dtau: pfloat = 0.;
    let mut dkap: pfloat = 0.;
    let mut bkap: pfloat = 0.;
    let mut exitcode: idxint = ECOS_FATAL as idxint;
    let mut interrupted: idxint = 0 as idxint;
    let mut pres_prev: pfloat = R_NaN;
    i = 0 as idxint;
    while i < (*w).n {
        let ref mut fresh0 = *(*w).c.offset(i as isize);
        *fresh0 /= *(*w).xequil.offset(i as isize) as ::core::ffi::c_double;
        i += 1;
    }
    initcode = init(w);
    if initcode == ECOS_FATAL as i64 {
        return ECOS_FATAL as idxint;
    }
    (*(*w).info).iter = 0 as idxint;
    while (*(*w).info).iter <= (*(*w).stgs).maxit {
        computeResiduals(w);
        updateStatistics(w);
        if (*(*w).info).iter > 0 as i64
            && ((*(*w).info).pres > SAFEGUARD as pfloat * pres_prev
                || (*(*w).info).gap < 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        {
            restoreBestIterate(w);
            exitcode = checkExitConditions(w, ECOS_INACC_OFFSET as idxint);
            if !(exitcode == ECOS_NOT_CONVERGED_YET as i64) {
                break;
            }
            exitcode = ECOS_NUMERICS as idxint;
            break;
        } else {
            pres_prev = (*(*w).info).pres;
            exitcode = checkExitConditions(w, 0 as idxint);
            if !(exitcode == ECOS_NOT_CONVERGED_YET as i64) {
                break;
            }
            if (*(*w).info).iter > 0 as i64 && (*(*w).info).step == STEPMIN * GAMMA
            {
                restoreBestIterate(w);
                exitcode = checkExitConditions(w, ECOS_INACC_OFFSET as idxint);
                if exitcode == ECOS_NOT_CONVERGED_YET as i64 {
                    exitcode = ECOS_NUMERICS as idxint;
                }
                break;
            } else if interrupted != 0 || (*(*w).info).iter == (*(*w).stgs).maxit {
                if !(compareStatistics((*w).info, (*w).best_info) != 0) {
                    restoreBestIterate(w);
                }
                exitcode = checkExitConditions(w, ECOS_INACC_OFFSET as idxint);
                if exitcode == ECOS_NOT_CONVERGED_YET as i64 {
                    exitcode = (if interrupted != 0 {
                        ECOS_SIGINT
                    } else {
                        ECOS_MAXIT
                    }) as idxint;
                }
                break;
            } else if if ::core::mem::size_of::<pfloat>() as usize
                == ::core::mem::size_of::<::core::ffi::c_float>() as usize
            {
                __inline_isnanf((*(*w).info).pcost as ::core::ffi::c_float)
            } else if ::core::mem::size_of::<pfloat>() as usize
                == ::core::mem::size_of::<::core::ffi::c_double>() as usize
            {
                __inline_isnand((*(*w).info).pcost)
            } else {
                __inline_isnanl((*(*w).info).pcost)
            } != 0
            {
                if !(compareStatistics((*w).info, (*w).best_info) != 0) {
                    restoreBestIterate(w);
                }
                exitcode = checkExitConditions(w, ECOS_INACC_OFFSET as idxint);
                if exitcode == ECOS_NOT_CONVERGED_YET as i64 {
                    exitcode = ECOS_NUMERICS as idxint;
                }
                break;
            } else {
                if (*(*w).info).iter == 0 as i64 {
                    saveIterateAsBest(w);
                } else if compareStatistics((*w).info, (*w).best_info) != 0 {
                    saveIterateAsBest(w);
                }
                if updateScalings((*w).C, (*w).s, (*w).z, (*w).lambda)
                    == OUTSIDE_CONE as i64
                {
                    restoreBestIterate(w);
                    exitcode = checkExitConditions(w, ECOS_INACC_OFFSET as idxint);
                    if !(exitcode == ECOS_NOT_CONVERGED_YET as i64) {
                        break;
                    }
                    return ECOS_OUTCONE as idxint;
                } else {
                    kkt_update((*(*w).KKT).PKPt, (*(*w).KKT).PK, (*w).C);
                    KKT_FACTOR_RETURN_CODE =
                        kkt_factor((*w).KKT, (*(*w).stgs).eps, (*(*w).stgs).delta);
                    if KKT_FACTOR_RETURN_CODE != KKT_OK as i64 {
                        return ECOS_FATAL as idxint;
                    }
                    (*(*w).info).nitref1 = kkt_solve(
                        (*w).KKT,
                        (*w).A,
                        (*w).G,
                        (*(*w).KKT).RHS1,
                        (*(*w).KKT).dx1,
                        (*(*w).KKT).dy1,
                        (*(*w).KKT).dz1,
                        (*w).n,
                        (*w).p,
                        (*w).m,
                        (*w).C,
                        0 as idxint,
                        (*(*w).stgs).nitref,
                    );
                    RHS_affine(w);
                    (*(*w).info).nitref2 = kkt_solve(
                        (*w).KKT,
                        (*w).A,
                        (*w).G,
                        (*(*w).KKT).RHS2,
                        (*(*w).KKT).dx2,
                        (*(*w).KKT).dy2,
                        (*(*w).KKT).dz2,
                        (*w).n,
                        (*w).p,
                        (*w).m,
                        (*w).C,
                        0 as idxint,
                        (*(*w).stgs).nitref,
                    );
                    dtau_denom = (*w).kap / (*w).tau
                        - eddot((*w).n, (*w).c, (*(*w).KKT).dx1)
                        - eddot((*w).p, (*w).b, (*(*w).KKT).dy1)
                        - eddot((*w).m, (*w).h, (*(*w).KKT).dz1);
                    dtauaff = ((*w).rt - (*w).kap
                        + eddot((*w).n, (*w).c, (*(*w).KKT).dx2)
                        + eddot((*w).p, (*w).b, (*(*w).KKT).dy2)
                        + eddot((*w).m, (*w).h, (*(*w).KKT).dz2))
                        / dtau_denom;
                    i = 0 as idxint;
                    while i < (*w).m {
                        *(*(*w).KKT).dz2.offset(i as isize) = *(*(*w).KKT).dz2.offset(i as isize)
                            + dtauaff * *(*(*w).KKT).dz1.offset(i as isize);
                        i += 1;
                    }
                    scale((*(*w).KKT).dz2, (*w).C, (*w).W_times_dzaff);
                    i = 0 as idxint;
                    while i < (*w).m {
                        *(*w).dsaff_by_W.offset(i as isize) =
                            -*(*w).W_times_dzaff.offset(i as isize)
                                - *(*w).lambda.offset(i as isize);
                        i += 1;
                    }
                    dkapaff = -(*w).kap - (*w).kap / (*w).tau * dtauaff;
                    (*(*w).info).step_aff = lineSearch(
                        (*w).lambda,
                        (*w).dsaff_by_W,
                        (*w).W_times_dzaff,
                        (*w).tau,
                        dtauaff,
                        (*w).kap,
                        dkapaff,
                        (*w).C,
                        (*w).KKT,
                    );
                    sigma = 1.0f64 - (*(*w).info).step_aff;
                    sigma = sigma * sigma * sigma;
                    if sigma > SIGMAMAX {
                        sigma = SIGMAMAX as pfloat;
                    }
                    if sigma < SIGMAMIN {
                        sigma = SIGMAMIN as pfloat;
                    }
                    (*(*w).info).sigma = sigma;
                    RHS_combined(w);
                    (*(*w).info).nitref3 = kkt_solve(
                        (*w).KKT,
                        (*w).A,
                        (*w).G,
                        (*(*w).KKT).RHS2,
                        (*(*w).KKT).dx2,
                        (*(*w).KKT).dy2,
                        (*(*w).KKT).dz2,
                        (*w).n,
                        (*w).p,
                        (*w).m,
                        (*w).C,
                        0 as idxint,
                        (*(*w).stgs).nitref,
                    );
                    bkap = (*w).kap * (*w).tau + dkapaff * dtauaff - sigma * (*(*w).info).mu;
                    dtau = ((1 as ::core::ffi::c_int as pfloat - sigma) * (*w).rt
                        - bkap / (*w).tau
                        + eddot((*w).n, (*w).c, (*(*w).KKT).dx2)
                        + eddot((*w).p, (*w).b, (*(*w).KKT).dy2)
                        + eddot((*w).m, (*w).h, (*(*w).KKT).dz2))
                        / dtau_denom;
                    i = 0 as idxint;
                    while i < (*w).n {
                        let ref mut fresh1 = *(*(*w).KKT).dx2.offset(i as isize);
                        *fresh1 +=
                            (dtau * *(*(*w).KKT).dx1.offset(i as isize)) as ::core::ffi::c_double;
                        i += 1;
                    }
                    i = 0 as idxint;
                    while i < (*w).p {
                        let ref mut fresh2 = *(*(*w).KKT).dy2.offset(i as isize);
                        *fresh2 +=
                            (dtau * *(*(*w).KKT).dy1.offset(i as isize)) as ::core::ffi::c_double;
                        i += 1;
                    }
                    i = 0 as idxint;
                    while i < (*w).m {
                        let ref mut fresh3 = *(*(*w).KKT).dz2.offset(i as isize);
                        *fresh3 +=
                            (dtau * *(*(*w).KKT).dz1.offset(i as isize)) as ::core::ffi::c_double;
                        i += 1;
                    }
                    scale((*(*w).KKT).dz2, (*w).C, (*w).W_times_dzaff);
                    i = 0 as idxint;
                    while i < (*w).m {
                        *(*w).dsaff_by_W.offset(i as isize) =
                            -(*(*w).dsaff_by_W.offset(i as isize)
                                + *(*w).W_times_dzaff.offset(i as isize));
                        i += 1;
                    }
                    dkap = -(bkap + (*w).kap * dtau) / (*w).tau;
                    (*(*w).info).step = lineSearch(
                        (*w).lambda,
                        (*w).dsaff_by_W,
                        (*w).W_times_dzaff,
                        (*w).tau,
                        dtau,
                        (*w).kap,
                        dkap,
                        (*w).C,
                        (*w).KKT,
                    ) * (*(*w).stgs).gamma;
                    scale((*w).dsaff_by_W, (*w).C, (*w).dsaff);
                    i = 0 as idxint;
                    while i < (*w).n {
                        let ref mut fresh4 = *(*w).x.offset(i as isize);
                        *fresh4 += ((*(*w).info).step * *(*(*w).KKT).dx2.offset(i as isize))
                            as ::core::ffi::c_double;
                        i += 1;
                    }
                    i = 0 as idxint;
                    while i < (*w).p {
                        let ref mut fresh5 = *(*w).y.offset(i as isize);
                        *fresh5 += ((*(*w).info).step * *(*(*w).KKT).dy2.offset(i as isize))
                            as ::core::ffi::c_double;
                        i += 1;
                    }
                    i = 0 as idxint;
                    while i < (*w).m {
                        let ref mut fresh6 = *(*w).z.offset(i as isize);
                        *fresh6 += ((*(*w).info).step * *(*(*w).KKT).dz2.offset(i as isize))
                            as ::core::ffi::c_double;
                        i += 1;
                    }
                    i = 0 as idxint;
                    while i < (*w).m {
                        let ref mut fresh7 = *(*w).s.offset(i as isize);
                        *fresh7 += ((*(*w).info).step * *(*w).dsaff.offset(i as isize))
                            as ::core::ffi::c_double;
                        i += 1;
                    }
                    (*w).kap += ((*(*w).info).step * dkap) as ::core::ffi::c_double;
                    (*w).tau += ((*(*w).info).step * dtau) as ::core::ffi::c_double;
                    (*(*w).info).iter += 1;
                }
            }
        }
    }
    backscale(w);
    return exitcode;
}
#[no_mangle]
pub unsafe extern "C" fn ecos_updateDataEntry_h(
    mut w: *mut pwork,
    mut idx: idxint,
    mut value: pfloat,
) {
    *(*w).h.offset(idx as isize) = value / *(*w).Gequil.offset(idx as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ecos_updateDataEntry_c(
    mut w: *mut pwork,
    mut idx: idxint,
    mut value: pfloat,
) {
    *(*w).c.offset(idx as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn ECOS_updateData(
    mut w: *mut pwork,
    mut Gpr: *mut pfloat,
    mut Apr: *mut pfloat,
    mut c: *mut pfloat,
    mut h: *mut pfloat,
    mut b: *mut pfloat,
) {
    let mut k: idxint = 0;
    if (Gpr.offset((*(*w).G).nnz as isize) < (*(*w).G).pr
        || (*(*w).G).pr.offset((*(*w).G).nnz as isize) < Gpr)
        && (Apr.offset((*(*w).A).nnz as isize) < (*(*w).A).pr
            || (*(*w).A).pr.offset((*(*w).A).nnz as isize) < Apr)
        && (c.offset((*w).n as isize) < (*w).c || (*w).c.offset((*w).n as isize) < c)
        && (h.offset((*w).m as isize) < (*w).h || (*w).h.offset((*w).m as isize) < h)
        && (b.offset((*w).p as isize) < (*w).b || (*w).b.offset((*w).p as isize) < b)
    {
        unset_equilibration(w);
    }
    if !(*w).G.is_null() {
        (*(*w).G).pr = Gpr;
        (*w).h = h;
    }
    if !(*w).A.is_null() {
        (*(*w).A).pr = Apr;
        (*w).b = b;
    }
    (*w).c = c;
    set_equilibration(w);
    if !(*w).A.is_null() {
        k = 0 as idxint;
        while k < (*(*w).A).nnz {
            *(*(*(*w).KKT).PKPt).pr.offset(
                *(*(*w).KKT)
                    .PK
                    .offset(*(*w).AtoK.offset(k as isize) as isize) as isize,
            ) = *Apr.offset(k as isize);
            k += 1;
        }
    }
    if !(*w).G.is_null() {
        k = 0 as idxint;
        while k < (*(*w).G).nnz {
            *(*(*(*w).KKT).PKPt).pr.offset(
                *(*(*w).KKT)
                    .PK
                    .offset(*(*w).GtoK.offset(k as isize) as isize) as isize,
            ) = *Gpr.offset(k as isize);
            k += 1;
        }
    }
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
