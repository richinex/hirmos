use crate::runtime::{malloc,free};
extern "C" {
    fn ecoscreateSparseMatrix(
        m: idxint,
        n: idxint,
        nnz: idxint,
        jc: *mut idxint,
        ir: *mut idxint,
        pr: *mut pfloat,
    ) -> *mut spmat;
    fn newSparseMatrix(m: idxint, n: idxint, nnz: idxint) -> *mut spmat;
    fn freeSparseMatrix(M: *mut spmat);
    fn transposeSparseMatrix(M: *mut spmat, MtoMt: *mut idxint) -> *mut spmat;
    fn permuteSparseSymmetricMatrix(
        A: *mut spmat,
        pinv_0: *mut idxint,
        C: *mut spmat,
        PK: *mut idxint,
    );
    fn pinv(n: idxint, p: *mut idxint, pinv_0: *mut idxint);
    fn set_equilibration(w: *mut pwork);
    fn unset_equilibration(w: *mut pwork);
    fn amd_l_order(
        n: i64,
        Ap: *const i64,
        Ai: *const i64,
        P: *mut i64,
        Control: *mut ::core::ffi::c_double,
        Info: *mut ::core::ffi::c_double,
    ) -> i64;
    fn amd_l_defaults(Control: *mut ::core::ffi::c_double);
    fn ldl_l_symbolic2(
        n: i64,
        Ap: *mut i64,
        Ai: *mut i64,
        Lp: *mut i64,
        Parent: *mut i64,
        Lnz: *mut i64,
        Flag: *mut i64,
    );
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
pub const MAXIT: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const FEASTOL: ::core::ffi::c_double = 1E-8f64;
pub const ABSTOL: ::core::ffi::c_double = 1E-8f64;
pub const RELTOL: ::core::ffi::c_double = 1E-8f64;
pub const FTOL_INACC: ::core::ffi::c_double = 1E-4f64;
pub const ATOL_INACC: ::core::ffi::c_double = 5E-5f64;
pub const RTOL_INACC: ::core::ffi::c_double = 5E-5f64;
pub const GAMMA: ::core::ffi::c_double = 0.99f64;
pub const DELTASTAT: ::core::ffi::c_double = 7E-8f64;
pub const DELTA: ::core::ffi::c_double = 2E-7f64;
pub const EPS: ::core::ffi::c_double = 1E-13f64;
pub const VERBOSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NITREF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const AMD_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn createKKT_U(
    mut Gt: *mut spmat,
    mut At: *mut spmat,
    mut C: *mut cone,
    mut S: *mut *mut idxint,
    mut K: *mut *mut spmat,
    mut AttoK: *mut idxint,
    mut GttoK: *mut idxint,
) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut k: idxint = 0;
    let mut l: idxint = 0;
    let mut r: idxint = 0;
    let mut row_stop: idxint = 0;
    let mut row: idxint = 0;
    let mut cone_strt: idxint = 0;
    let mut ks: idxint = 0;
    let mut conesize: idxint = 0;
    let mut n: idxint = (*Gt).m;
    let mut m: idxint = (*Gt).n;
    let mut p: idxint = if !At.is_null() { (*At).n } else { 0 as idxint };
    let mut nK: idxint = 0;
    let mut nnzK: idxint = 0;
    let mut Kpr: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut Kjc: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut Kir: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut Sign: *mut idxint = ::core::ptr::null_mut::<idxint>();
    nK = n + p + m;
    nK += (2 as idxint * (*C).nsoc) as i64;
    nnzK = (if !At.is_null() {
        (*At).nnz
    } else {
        0 as idxint
    }) + (*Gt).nnz
        + (*(*C).lpc).p;
    nnzK += (n + p) as i64;
    i = 0 as idxint;
    while i < (*C).nsoc {
        nnzK += 3 as i64 * (*(*C).soc.offset(i as isize)).p as i64
            + 1 as i64;
        i += 1;
    }
    Kpr = malloc((nnzK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    Kir = malloc((nnzK as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
        as *mut idxint;
    Kjc = malloc(
        ((nK as i64 + 1 as i64) as size_t)
            .wrapping_mul(::core::mem::size_of::<idxint>() as size_t),
    ) as *mut idxint;
    Sign = malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
        as *mut idxint;
    ks = 0 as idxint;
    while ks < n {
        *Sign.offset(ks as isize) = 1 as ::core::ffi::c_int as idxint;
        ks += 1;
    }
    ks = n;
    while ks < n + p {
        *Sign.offset(ks as isize) = -(1 as ::core::ffi::c_int) as idxint;
        ks += 1;
    }
    ks = n + p;
    while ks < n + p + (*(*C).lpc).p {
        *Sign.offset(ks as isize) = -(1 as ::core::ffi::c_int) as idxint;
        ks += 1;
    }
    ks = n + p + (*(*C).lpc).p;
    l = 0 as idxint;
    while l < (*C).nsoc {
        i = 0 as idxint;
        while i < (*(*C).soc.offset(l as isize)).p {
            let fresh4 = ks;
            ks = ks + 1;
            *Sign.offset(fresh4 as isize) = -(1 as ::core::ffi::c_int) as idxint;
            i += 1;
        }
        let fresh5 = ks;
        ks = ks + 1;
        *Sign.offset(fresh5 as isize) = -(1 as ::core::ffi::c_int) as idxint;
        let fresh6 = ks;
        ks = ks + 1;
        *Sign.offset(fresh6 as isize) = 1 as ::core::ffi::c_int as idxint;
        l += 1;
    }
    k = 0 as idxint;
    j = 0 as idxint;
    while j < n {
        *Kjc.offset(j as isize) = j;
        *Kir.offset(j as isize) = j;
        let fresh7 = k;
        k = k + 1;
        *Kpr.offset(fresh7 as isize) = DELTASTAT as pfloat;
        j += 1;
    }
    i = 0 as idxint;
    j = 0 as idxint;
    while j < p {
        row = *(*At).jc.offset(j as isize);
        row_stop = *(*At)
            .jc
            .offset((j as i64 + 1 as i64) as isize);
        if row <= row_stop {
            *Kjc.offset((n + j) as isize) = k;
            loop {
                let fresh8 = row;
                row = row + 1;
                if !(fresh8 < row_stop) {
                    break;
                }
                *Kir.offset(k as isize) = *(*At).ir.offset(i as isize);
                *Kpr.offset(k as isize) = *(*At).pr.offset(i as isize);
                let fresh9 = k;
                k = k + 1;
                let fresh10 = i;
                i = i + 1;
                *AttoK.offset(fresh10 as isize) = fresh9;
            }
        }
        *Kir.offset(k as isize) = n + j;
        let fresh11 = k;
        k = k + 1;
        *Kpr.offset(fresh11 as isize) = -DELTASTAT as pfloat;
        j += 1;
    }
    i = 0 as idxint;
    j = 0 as idxint;
    while j < (*(*C).lpc).p {
        row = *(*Gt).jc.offset(j as isize);
        row_stop = *(*Gt)
            .jc
            .offset((j as i64 + 1 as i64) as isize);
        if row <= row_stop {
            *Kjc.offset((n + p + j) as isize) = k;
            loop {
                let fresh12 = row;
                row = row + 1;
                if !(fresh12 < row_stop) {
                    break;
                }
                *Kir.offset(k as isize) = *(*Gt).ir.offset(i as isize);
                *Kpr.offset(k as isize) = *(*Gt).pr.offset(i as isize);
                let fresh13 = k;
                k = k + 1;
                let fresh14 = i;
                i = i + 1;
                *GttoK.offset(fresh14 as isize) = fresh13;
            }
        }
        *(*(*C).lpc).kkt_idx.offset(j as isize) = k;
        *Kir.offset(k as isize) = n + p + j;
        *Kpr.offset(k as isize) = -1.0f64 as pfloat;
        k += 1;
        j += 1;
    }
    cone_strt = (*(*C).lpc).p;
    l = 0 as idxint;
    while l < (*C).nsoc {
        conesize = (*(*C).soc.offset(l as isize)).p;
        j = 0 as idxint;
        while j < conesize {
            row = *(*Gt).jc.offset((cone_strt + j) as isize);
            row_stop = *(*Gt).jc.offset(
                (cone_strt as i64
                    + j as i64
                    + 1 as i64) as isize,
            );
            if row <= row_stop {
                *Kjc.offset((n + p + cone_strt + 2 as idxint * l + j) as isize) = k;
                loop {
                    let fresh15 = row;
                    row = row + 1;
                    if !(fresh15 < row_stop) {
                        break;
                    }
                    *Kir.offset(k as isize) = *(*Gt).ir.offset(i as isize);
                    *Kpr.offset(k as isize) = *(*Gt).pr.offset(i as isize);
                    let fresh16 = k;
                    k = k + 1;
                    let fresh17 = i;
                    i = i + 1;
                    *GttoK.offset(fresh17 as isize) = fresh16;
                }
            }
            *Kir.offset(k as isize) = n + p + cone_strt + 2 as idxint * l + j;
            *Kpr.offset(k as isize) = -1.0f64 as pfloat;
            *(*(*C).soc.offset(l as isize)).Didx.offset(j as isize) = k;
            k += 1;
            j += 1;
        }
        *Kjc.offset((n + p + cone_strt + 2 as idxint * l + conesize) as isize) = k;
        r = 1 as idxint;
        while r < conesize {
            *Kir.offset(k as isize) = n + p + cone_strt + 2 as idxint * l + r;
            *Kpr.offset(k as isize) = 0 as ::core::ffi::c_int as pfloat;
            k += 1;
            r += 1;
        }
        *Kir.offset(k as isize) = n + p + cone_strt + 2 as idxint * l + conesize;
        *Kpr.offset(k as isize) = -(1 as ::core::ffi::c_int) as pfloat;
        k += 1;
        *Kjc.offset(
            (n as i64
                + p as i64
                + cone_strt as i64
                + 2 as i64 * l as i64
                + conesize as i64
                + 1 as i64) as isize,
        ) = k;
        r = 0 as idxint;
        while r < conesize {
            *Kir.offset(k as isize) = n + p + cone_strt + 2 as idxint * l + r;
            *Kpr.offset(k as isize) = 0 as ::core::ffi::c_int as pfloat;
            k += 1;
            r += 1;
        }
        *Kir.offset(k as isize) = (n as i64
            + p as i64
            + cone_strt as i64
            + 2 as i64 * l as i64
            + conesize as i64
            + 1 as i64) as idxint;
        *Kpr.offset(k as isize) = 1 as ::core::ffi::c_int as pfloat;
        k += 1;
        cone_strt += (*(*C).soc.offset(l as isize)).p as i64;
        l += 1;
    }
    *S = Sign;
    *K = ecoscreateSparseMatrix(nK, nK, nnzK, Kjc, Kir, Kpr);
}
#[no_mangle]
pub unsafe extern "C" fn ECOS_cleanup(mut w: *mut pwork, mut keepvars: idxint) {
    let mut i: idxint = 0;
    unset_equilibration(w);
    free((*(*w).KKT).D as *mut ::core::ffi::c_void);
    free((*(*w).KKT).dx1 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).dx2 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).dy1 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).dy2 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).dz1 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).dz2 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).Flag as *mut ::core::ffi::c_void);
    freeSparseMatrix((*(*w).KKT).L);
    free((*(*w).KKT).Lnz as *mut ::core::ffi::c_void);
    free((*(*w).KKT).Parent as *mut ::core::ffi::c_void);
    free((*(*w).KKT).Pattern as *mut ::core::ffi::c_void);
    free((*(*w).KKT).Sign as *mut ::core::ffi::c_void);
    free((*(*w).KKT).Pinv as *mut ::core::ffi::c_void);
    free((*(*w).KKT).P as *mut ::core::ffi::c_void);
    free((*(*w).KKT).PK as *mut ::core::ffi::c_void);
    freeSparseMatrix((*(*w).KKT).PKPt);
    free((*(*w).KKT).RHS1 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).RHS2 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).work1 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).work2 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).work3 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).work4 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).work5 as *mut ::core::ffi::c_void);
    free((*(*w).KKT).work6 as *mut ::core::ffi::c_void);
    free((*w).KKT as *mut ::core::ffi::c_void);
    if !(*w).A.is_null() {
        free((*w).AtoK as *mut ::core::ffi::c_void);
    }
    free((*w).GtoK as *mut ::core::ffi::c_void);
    if (*(*(*w).C).lpc).p > 0 as i64 {
        free((*(*(*w).C).lpc).kkt_idx as *mut ::core::ffi::c_void);
        free((*(*(*w).C).lpc).v as *mut ::core::ffi::c_void);
        free((*(*(*w).C).lpc).w as *mut ::core::ffi::c_void);
    }
    free((*(*w).C).lpc as *mut ::core::ffi::c_void);
    i = 0 as idxint;
    while i < (*(*w).C).nsoc {
        free((*(*(*w).C).soc.offset(i as isize)).q as *mut ::core::ffi::c_void);
        free((*(*(*w).C).soc.offset(i as isize)).skbar as *mut ::core::ffi::c_void);
        free((*(*(*w).C).soc.offset(i as isize)).zkbar as *mut ::core::ffi::c_void);
        free((*(*(*w).C).soc.offset(i as isize)).Didx as *mut ::core::ffi::c_void);
        i += 1;
    }
    if (*(*w).C).nsoc > 0 as i64 {
        free((*(*w).C).soc as *mut ::core::ffi::c_void);
    }
    free((*w).C as *mut ::core::ffi::c_void);
    free((*w).W_times_dzaff as *mut ::core::ffi::c_void);
    free((*w).dsaff_by_W as *mut ::core::ffi::c_void);
    free((*w).dzaff as *mut ::core::ffi::c_void);
    free((*w).dsaff as *mut ::core::ffi::c_void);
    free((*w).zaff as *mut ::core::ffi::c_void);
    free((*w).saff as *mut ::core::ffi::c_void);
    free((*w).info as *mut ::core::ffi::c_void);
    free((*w).best_info as *mut ::core::ffi::c_void);
    free((*w).lambda as *mut ::core::ffi::c_void);
    free((*w).rx as *mut ::core::ffi::c_void);
    free((*w).ry as *mut ::core::ffi::c_void);
    free((*w).rz as *mut ::core::ffi::c_void);
    free((*w).stgs as *mut ::core::ffi::c_void);
    free((*w).G as *mut ::core::ffi::c_void);
    if !(*w).A.is_null() {
        free((*w).A as *mut ::core::ffi::c_void);
    }
    free((*w).best_z as *mut ::core::ffi::c_void);
    free((*w).best_s as *mut ::core::ffi::c_void);
    free((*w).best_y as *mut ::core::ffi::c_void);
    free((*w).best_x as *mut ::core::ffi::c_void);
    if keepvars < 4 as i64 {
        free((*w).z as *mut ::core::ffi::c_void);
    }
    if keepvars < 3 as i64 {
        free((*w).s as *mut ::core::ffi::c_void);
    }
    if keepvars < 2 as i64 {
        free((*w).y as *mut ::core::ffi::c_void);
    }
    if keepvars < 1 as i64 {
        free((*w).x as *mut ::core::ffi::c_void);
    }
    free((*w).xequil as *mut ::core::ffi::c_void);
    free((*w).Aequil as *mut ::core::ffi::c_void);
    free((*w).Gequil as *mut ::core::ffi::c_void);
    free(w as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ECOS_setup(
    mut n: idxint,
    mut m: idxint,
    mut p: idxint,
    mut l: idxint,
    mut ncones: idxint,
    mut q: *mut idxint,
    mut nexc: idxint,
    mut Gpr: *mut pfloat,
    mut Gjc: *mut idxint,
    mut Gir: *mut idxint,
    mut Apr: *mut pfloat,
    mut Ajc: *mut idxint,
    mut Air: *mut idxint,
    mut c: *mut pfloat,
    mut h: *mut pfloat,
    mut b: *mut pfloat,
) -> *mut pwork {
    let mut i: idxint = 0;
    let mut cidx: idxint = 0;
    let mut conesize: idxint = 0;
    let mut lnz: idxint = 0;
    let mut amd_result: idxint = 0;
    let mut nK: idxint = 0;
    let mut Ljc: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut Lir: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut P: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut Pinv: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut Sign: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut mywork: *mut pwork = ::core::ptr::null_mut::<pwork>();
    let mut Control: [::core::ffi::c_double; 5] = [0.; 5];
    let mut Info: [::core::ffi::c_double; 20] = [0.; 20];
    let mut Lpr: *mut pfloat = ::core::ptr::null_mut::<pfloat>();
    let mut At: *mut spmat = ::core::ptr::null_mut::<spmat>();
    let mut Gt: *mut spmat = ::core::ptr::null_mut::<spmat>();
    let mut KU: *mut spmat = ::core::ptr::null_mut::<spmat>();
    let mut AtoAt: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut GtoGt: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut AttoK: *mut idxint = ::core::ptr::null_mut::<idxint>();
    let mut GttoK: *mut idxint = ::core::ptr::null_mut::<idxint>();
    mywork = malloc(::core::mem::size_of::<pwork>() as size_t) as *mut pwork;
    (*mywork).n = n;
    (*mywork).m = m;
    (*mywork).p = p;
    (*mywork).D = l + ncones;
    (*mywork).x = malloc((n as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    (*mywork).y = malloc((p as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    (*mywork).z = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    (*mywork).s = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    (*mywork).lambda =
        malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).dsaff_by_W =
        malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).dsaff = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    (*mywork).dzaff = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    (*mywork).saff = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    (*mywork).zaff = malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    (*mywork).W_times_dzaff =
        malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).best_x =
        malloc((n as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).best_y =
        malloc((p as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).best_z =
        malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).best_s =
        malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).best_info = malloc(::core::mem::size_of::<stats>() as size_t) as *mut stats;
    (*mywork).C = malloc(::core::mem::size_of::<cone>() as size_t) as *mut cone;
    (*(*mywork).C).lpc = malloc(::core::mem::size_of::<lpcone>() as size_t) as *mut lpcone;
    (*(*(*mywork).C).lpc).p = l;
    if l > 0 as i64 {
        (*(*(*mywork).C).lpc).w =
            malloc((l as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
                as *mut pfloat;
        (*(*(*mywork).C).lpc).v =
            malloc((l as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
                as *mut pfloat;
        (*(*(*mywork).C).lpc).kkt_idx =
            malloc((l as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
                as *mut idxint;
    } else {
        (*(*(*mywork).C).lpc).w = ::core::ptr::null_mut::<pfloat>();
        (*(*(*mywork).C).lpc).v = ::core::ptr::null_mut::<pfloat>();
        (*(*(*mywork).C).lpc).kkt_idx = ::core::ptr::null_mut::<idxint>();
    }
    (*(*mywork).C).soc = if ncones == 0 as i64 {
        ::core::ptr::null_mut::<socone>()
    } else {
        malloc((ncones as size_t).wrapping_mul(::core::mem::size_of::<socone>() as size_t))
            as *mut socone
    };
    (*(*mywork).C).nsoc = ncones;
    cidx = 0 as idxint;
    i = 0 as idxint;
    while i < ncones {
        conesize = *q.offset(i as isize);
        (*(*(*mywork).C).soc.offset(i as isize)).p = conesize;
        (*(*(*mywork).C).soc.offset(i as isize)).a = 0 as ::core::ffi::c_int as pfloat;
        (*(*(*mywork).C).soc.offset(i as isize)).eta = 0 as ::core::ffi::c_int as pfloat;
        let ref mut fresh0 = (*(*(*mywork).C).soc.offset(i as isize)).q;
        *fresh0 = malloc(
            ((conesize as i64 - 1 as i64) as size_t)
                .wrapping_mul(::core::mem::size_of::<pfloat>() as size_t),
        ) as *mut pfloat;
        let ref mut fresh1 = (*(*(*mywork).C).soc.offset(i as isize)).skbar;
        *fresh1 =
            malloc((conesize as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
                as *mut pfloat;
        let ref mut fresh2 = (*(*(*mywork).C).soc.offset(i as isize)).zkbar;
        *fresh2 =
            malloc((conesize as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
                as *mut pfloat;
        let ref mut fresh3 = (*(*(*mywork).C).soc.offset(i as isize)).Didx;
        *fresh3 =
            malloc((conesize as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
                as *mut idxint;
        cidx += conesize as i64;
        i += 1;
    }
    if cidx + l != m {
        return ::core::ptr::null_mut::<pwork>();
    }
    (*mywork).info = malloc(::core::mem::size_of::<stats>() as size_t) as *mut stats;
    (*mywork).xequil =
        malloc((n as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).Aequil =
        malloc((p as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).Gequil =
        malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*mywork).stgs = malloc(::core::mem::size_of::<settings>() as size_t) as *mut settings;
    (*(*mywork).stgs).maxit = MAXIT as idxint;
    (*(*mywork).stgs).gamma = GAMMA as pfloat;
    (*(*mywork).stgs).delta = DELTA as pfloat;
    (*(*mywork).stgs).eps = EPS as pfloat;
    (*(*mywork).stgs).nitref = NITREF as idxint;
    (*(*mywork).stgs).abstol = ABSTOL as pfloat;
    (*(*mywork).stgs).feastol = FEASTOL as pfloat;
    (*(*mywork).stgs).reltol = RELTOL as pfloat;
    (*(*mywork).stgs).abstol_inacc = ATOL_INACC as pfloat;
    (*(*mywork).stgs).feastol_inacc = FTOL_INACC as pfloat;
    (*(*mywork).stgs).reltol_inacc = RTOL_INACC as pfloat;
    (*(*mywork).stgs).verbose = VERBOSE as idxint;
    (*mywork).c = c;
    (*mywork).h = h;
    (*mywork).b = b;
    if !Apr.is_null() && !Ajc.is_null() && !Air.is_null() {
        (*mywork).A = ecoscreateSparseMatrix(p, n, *Ajc.offset(n as isize), Ajc, Air, Apr);
    } else {
        (*mywork).A = ::core::ptr::null_mut::<spmat>();
    }
    if !Gpr.is_null() && !Gjc.is_null() && !Gir.is_null() {
        (*mywork).G = ecoscreateSparseMatrix(m, n, *Gjc.offset(n as isize), Gjc, Gir, Gpr);
    } else {
        (*mywork).G = ecoscreateSparseMatrix(m, n, 0 as idxint, Gjc, Gir, Gpr);
    }
    set_equilibration(mywork);
    if !(*mywork).A.is_null() {
        AtoAt = malloc(
            ((*(*mywork).A).nnz as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t),
        ) as *mut idxint;
        At = transposeSparseMatrix((*mywork).A, AtoAt);
    } else {
        At = ::core::ptr::null_mut::<spmat>();
        AtoAt = ::core::ptr::null_mut::<idxint>();
    }
    GtoGt = malloc(
        ((*(*mywork).G).nnz as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t),
    ) as *mut idxint;
    Gt = transposeSparseMatrix((*mywork).G, GtoGt);
    if !(*mywork).A.is_null() {
        AttoK = malloc(
            ((*(*mywork).A).nnz as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t),
        ) as *mut idxint;
    } else {
        AttoK = ::core::ptr::null_mut::<idxint>();
    }
    GttoK = malloc(
        ((*(*mywork).G).nnz as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t),
    ) as *mut idxint;
    createKKT_U(
        Gt,
        At,
        (*mywork).C,
        &raw mut Sign,
        &raw mut KU,
        AttoK,
        GttoK,
    );
    if !(*mywork).A.is_null() {
        (*mywork).AtoK = malloc(
            ((*(*mywork).A).nnz as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t),
        ) as *mut idxint;
        i = 0 as idxint;
        while i < (*(*mywork).A).nnz {
            *(*mywork).AtoK.offset(i as isize) = *AttoK.offset(*AtoAt.offset(i as isize) as isize);
            i += 1;
        }
    } else {
        (*mywork).AtoK = ::core::ptr::null_mut::<idxint>();
    }
    (*mywork).GtoK = malloc(
        ((*(*mywork).G).nnz as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t),
    ) as *mut idxint;
    i = 0 as idxint;
    while i < (*(*mywork).G).nnz {
        *(*mywork).GtoK.offset(i as isize) = *GttoK.offset(*GtoGt.offset(i as isize) as isize);
        i += 1;
    }
    nK = (*KU).n;
    (*mywork).KKT = malloc(::core::mem::size_of::<kkt>() as size_t) as *mut kkt;
    (*(*mywork).KKT).D =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).Parent =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
            as *mut idxint;
    (*(*mywork).KKT).Pinv =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
            as *mut idxint;
    (*(*mywork).KKT).work1 =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).work2 =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).work3 =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).work4 =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).work5 =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).work6 =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).Flag =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
            as *mut idxint;
    (*(*mywork).KKT).Pattern =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
            as *mut idxint;
    (*(*mywork).KKT).Lnz =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
            as *mut idxint;
    (*(*mywork).KKT).RHS1 =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).RHS2 =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).dx1 =
        malloc(((*mywork).n as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).dx2 =
        malloc(((*mywork).n as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).dy1 =
        malloc(((*mywork).p as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).dy2 =
        malloc(((*mywork).p as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).dz1 =
        malloc(((*mywork).m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).dz2 =
        malloc(((*mywork).m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat;
    (*(*mywork).KKT).Sign =
        malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
            as *mut idxint;
    (*(*mywork).KKT).PKPt = newSparseMatrix(nK, nK, (*KU).nnz);
    (*(*mywork).KKT).PK =
        malloc(((*KU).nnz as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
            as *mut idxint;
    P = malloc((nK as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
        as *mut idxint;
    amd_l_defaults(&raw mut Control as *mut ::core::ffi::c_double);
    amd_result = amd_l_order(
        nK as i64,
        (*KU).jc as *const i64,
        (*KU).ir as *const i64,
        P as *mut i64,
        &raw mut Control as *mut ::core::ffi::c_double,
        &raw mut Info as *mut ::core::ffi::c_double,
    ) as idxint;
    if amd_result == AMD_OK as i64 {
    } else {
        return ::core::ptr::null_mut::<pwork>();
    }
    pinv(nK, P, (*(*mywork).KKT).Pinv);
    Pinv = (*(*mywork).KKT).Pinv;
    permuteSparseSymmetricMatrix(
        KU,
        (*(*mywork).KKT).Pinv,
        (*(*mywork).KKT).PKPt,
        (*(*mywork).KKT).PK,
    );
    i = 0 as idxint;
    while i < nK {
        *(*(*mywork).KKT)
            .Sign
            .offset(*Pinv.offset(i as isize) as isize) = *Sign.offset(i as isize);
        i += 1;
    }
    Ljc = malloc(
        ((nK as i64 + 1 as i64) as size_t)
            .wrapping_mul(::core::mem::size_of::<idxint>() as size_t),
    ) as *mut idxint;
    ldl_l_symbolic2(
        (*(*(*mywork).KKT).PKPt).n as i64,
        (*(*(*mywork).KKT).PKPt).jc as *mut i64,
        (*(*(*mywork).KKT).PKPt).ir as *mut i64,
        Ljc as *mut i64,
        (*(*mywork).KKT).Parent as *mut i64,
        (*(*mywork).KKT).Lnz as *mut i64,
        (*(*mywork).KKT).Flag as *mut i64,
    );
    lnz = *Ljc.offset(nK as isize);
    Lir = malloc((lnz as size_t).wrapping_mul(::core::mem::size_of::<idxint>() as size_t))
        as *mut idxint;
    Lpr = malloc((lnz as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
        as *mut pfloat;
    (*(*mywork).KKT).L = ecoscreateSparseMatrix(nK, nK, lnz, Ljc, Lir, Lpr);
    permuteSparseSymmetricMatrix(
        KU,
        (*(*mywork).KKT).Pinv,
        (*(*mywork).KKT).PKPt,
        ::core::ptr::null_mut::<idxint>(),
    );
    (*mywork).rx = if n == 0 as i64 {
        ::core::ptr::null_mut::<pfloat>()
    } else {
        malloc((n as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat
    };
    (*mywork).ry = if p == 0 as i64 {
        ::core::ptr::null_mut::<pfloat>()
    } else {
        malloc((p as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat
    };
    (*mywork).rz = if m == 0 as i64 {
        ::core::ptr::null_mut::<pfloat>()
    } else {
        malloc((m as size_t).wrapping_mul(::core::mem::size_of::<pfloat>() as size_t))
            as *mut pfloat
    };
    (*(*mywork).KKT).P = P;
    free(Sign as *mut ::core::ffi::c_void);
    if !At.is_null() {
        freeSparseMatrix(At);
        free(AtoAt as *mut ::core::ffi::c_void);
        free(AttoK as *mut ::core::ffi::c_void);
    }
    freeSparseMatrix(Gt);
    freeSparseMatrix(KU);
    free(GtoGt as *mut ::core::ffi::c_void);
    free(GttoK as *mut ::core::ffi::c_void);
    return mywork;
}
