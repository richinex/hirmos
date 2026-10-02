use crate::runtime::{calloc,free,sqrt,fabs};
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
pub const EQUIL_ITERS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn max_rows(mut E: *mut pfloat, mut mat: *const spmat) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut row: idxint = 0;
    i = 0 as idxint;
    while i < (*mat).n {
        j = *(*mat).jc.offset(i as isize);
        while j < *(*mat)
            .jc
            .offset((i as i64 + 1 as i64) as isize)
        {
            row = *(*mat).ir.offset(j as isize);
            *E.offset(row as isize) =
                (if fabs(*(*mat).pr.offset(j as isize) as ::core::ffi::c_double)
                    < *E.offset(row as isize)
                {
                    *E.offset(row as isize) as ::core::ffi::c_double
                } else {
                    fabs(*(*mat).pr.offset(j as isize) as ::core::ffi::c_double)
                }) as pfloat;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn max_cols(mut E: *mut pfloat, mut mat: *const spmat) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    i = 0 as idxint;
    while i < (*mat).n {
        j = *(*mat).jc.offset(i as isize);
        while j < *(*mat)
            .jc
            .offset((i as i64 + 1 as i64) as isize)
        {
            *E.offset(i as isize) = (if fabs(*(*mat).pr.offset(j as isize) as ::core::ffi::c_double)
                < *E.offset(i as isize)
            {
                *E.offset(i as isize) as ::core::ffi::c_double
            } else {
                fabs(*(*mat).pr.offset(j as isize) as ::core::ffi::c_double)
            }) as pfloat;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sum_sq_rows(mut E: *mut pfloat, mut mat: *const spmat) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut row: idxint = 0;
    i = 0 as idxint;
    while i < (*mat).n {
        j = *(*mat).jc.offset(i as isize);
        while j < *(*mat)
            .jc
            .offset((i as i64 + 1 as i64) as isize)
        {
            row = *(*mat).ir.offset(j as isize);
            let ref mut fresh10 = *E.offset(row as isize);
            *fresh10 += (*(*mat).pr.offset(j as isize) * *(*mat).pr.offset(j as isize))
                as ::core::ffi::c_double;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sum_sq_cols(mut E: *mut pfloat, mut mat: *const spmat) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    i = 0 as idxint;
    while i < (*mat).n {
        j = *(*mat).jc.offset(i as isize);
        while j < *(*mat)
            .jc
            .offset((i as i64 + 1 as i64) as isize)
        {
            let ref mut fresh11 = *E.offset(i as isize);
            *fresh11 += (*(*mat).pr.offset(j as isize) * *(*mat).pr.offset(j as isize))
                as ::core::ffi::c_double;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn equilibrate_rows(mut E: *const pfloat, mut mat: *mut spmat) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut row: idxint = 0;
    i = 0 as idxint;
    while i < (*mat).n {
        j = *(*mat).jc.offset(i as isize);
        while j < *(*mat)
            .jc
            .offset((i as i64 + 1 as i64) as isize)
        {
            row = *(*mat).ir.offset(j as isize);
            let ref mut fresh6 = *(*mat).pr.offset(j as isize);
            *fresh6 /= *E.offset(row as isize) as ::core::ffi::c_double;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn equilibrate_cols(mut E: *const pfloat, mut mat: *mut spmat) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    i = 0 as idxint;
    while i < (*mat).n {
        j = *(*mat).jc.offset(i as isize);
        while j < *(*mat)
            .jc
            .offset((i as i64 + 1 as i64) as isize)
        {
            let ref mut fresh5 = *(*mat).pr.offset(j as isize);
            *fresh5 /= *E.offset(i as isize) as ::core::ffi::c_double;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn restore(mut D: *const pfloat, mut E: *const pfloat, mut mat: *mut spmat) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut row: idxint = 0;
    i = 0 as idxint;
    while i < (*mat).n {
        j = *(*mat).jc.offset(i as isize);
        while j < *(*mat)
            .jc
            .offset((i as i64 + 1 as i64) as isize)
        {
            row = *(*mat).ir.offset(j as isize);
            let ref mut fresh9 = *(*mat).pr.offset(j as isize);
            *fresh9 *= (*D.offset(row as isize) * *E.offset(i as isize)) as ::core::ffi::c_double;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn use_alternating_norm_equilibration(mut w: *mut pwork) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut ind: idxint = 0;
    let mut num_cols: idxint = if !(*w).A.is_null() {
        (*(*w).A).n
    } else {
        (*(*w).G).n
    };
    let mut num_A_rows: idxint = if !(*w).A.is_null() {
        (*(*w).A).m
    } else {
        0 as idxint
    };
    let mut num_G_rows: idxint = (*(*w).G).m;
    let mut sum: pfloat = 0.;
    i = 0 as idxint;
    while i < num_cols {
        *(*w).xequil.offset(i as isize) = 0.0f64 as pfloat;
        i += 1;
    }
    i = 0 as idxint;
    while i < num_A_rows {
        *(*w).Aequil.offset(i as isize) = 0.0f64 as pfloat;
        i += 1;
    }
    i = 0 as idxint;
    while i < num_G_rows {
        *(*w).Gequil.offset(i as isize) = 0.0f64 as pfloat;
        i += 1;
    }
    if !(*w).A.is_null() {
        sum_sq_rows((*w).Aequil, (*w).A);
    }
    if num_G_rows > 0 as i64 {
        sum_sq_rows((*w).Gequil, (*w).G);
    }
    ind = (*(*(*w).C).lpc).p;
    i = 0 as idxint;
    while i < (*(*w).C).nsoc {
        sum = 0.0f64 as pfloat;
        j = 0 as idxint;
        while j < (*(*(*w).C).soc.offset(i as isize)).p {
            sum += *(*w).Gequil.offset((ind + j) as isize) as ::core::ffi::c_double;
            j += 1;
        }
        j = 0 as idxint;
        while j < (*(*(*w).C).soc.offset(i as isize)).p {
            *(*w).Gequil.offset((ind + j) as isize) = (sum as ::core::ffi::c_double
                / (*(*(*w).C).soc.offset(i as isize)).p as ::core::ffi::c_double)
                as pfloat;
            j += 1;
        }
        ind += (*(*(*w).C).soc.offset(i as isize)).p as i64;
        i += 1;
    }
    i = 0 as idxint;
    while i < num_A_rows {
        *(*w).Aequil.offset(i as isize) =
            (if fabs(*(*w).Aequil.offset(i as isize) as ::core::ffi::c_double) < 1e-6f64 {
                1.0f64
            } else {
                sqrt(*(*w).Aequil.offset(i as isize) as ::core::ffi::c_double)
            }) as pfloat;
        i += 1;
    }
    i = 0 as idxint;
    while i < num_G_rows {
        *(*w).Gequil.offset(i as isize) =
            (if fabs(*(*w).Gequil.offset(i as isize) as ::core::ffi::c_double) < 1e-6f64 {
                1.0f64
            } else {
                sqrt(*(*w).Gequil.offset(i as isize) as ::core::ffi::c_double)
            }) as pfloat;
        i += 1;
    }
    if !(*w).A.is_null() {
        equilibrate_rows((*w).Aequil, (*w).A);
    }
    if num_G_rows > 0 as i64 {
        equilibrate_rows((*w).Gequil, (*w).G);
    }
    if !(*w).A.is_null() {
        sum_sq_cols((*w).xequil, (*w).A);
    }
    if num_G_rows > 0 as i64 {
        sum_sq_cols((*w).xequil, (*w).G);
    }
    i = 0 as idxint;
    while i < num_cols {
        *(*w).xequil.offset(i as isize) =
            (if fabs(*(*w).xequil.offset(i as isize) as ::core::ffi::c_double) < 1e-6f64 {
                1.0f64
            } else {
                sqrt(*(*w).xequil.offset(i as isize) as ::core::ffi::c_double)
            }) as pfloat;
        i += 1;
    }
    if !(*w).A.is_null() {
        equilibrate_cols((*w).xequil, (*w).A);
    }
    if num_G_rows > 0 as i64 {
        equilibrate_cols((*w).xequil, (*w).G);
    }
    i = 0 as idxint;
    while i < num_A_rows {
        let ref mut fresh12 = *(*w).b.offset(i as isize);
        *fresh12 /= *(*w).Aequil.offset(i as isize) as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as idxint;
    while i < num_G_rows {
        let ref mut fresh13 = *(*w).h.offset(i as isize);
        *fresh13 /= *(*w).Gequil.offset(i as isize) as ::core::ffi::c_double;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn use_ruiz_equilibration(mut w: *mut pwork) {
    let mut i: idxint = 0;
    let mut j: idxint = 0;
    let mut ind: idxint = 0;
    let mut iter: idxint = 0;
    let mut num_cols: idxint = if !(*w).A.is_null() {
        (*(*w).A).n
    } else {
        (*(*w).G).n
    };
    let mut num_A_rows: idxint = if !(*w).A.is_null() {
        (*(*w).A).m
    } else {
        0 as idxint
    };
    let mut num_G_rows: idxint = (*(*w).G).m;
    let mut xtmp: *mut pfloat = calloc(
        num_cols as size_t,
        ::core::mem::size_of::<pfloat>() as size_t,
    ) as *mut pfloat;
    let mut Atmp: *mut pfloat = calloc(
        num_A_rows as size_t,
        ::core::mem::size_of::<pfloat>() as size_t,
    ) as *mut pfloat;
    let mut Gtmp: *mut pfloat = calloc(
        num_G_rows as size_t,
        ::core::mem::size_of::<pfloat>() as size_t,
    ) as *mut pfloat;
    let mut total: pfloat = 0.;
    i = 0 as idxint;
    while i < num_cols {
        *(*w).xequil.offset(i as isize) = 1.0f64 as pfloat;
        i += 1;
    }
    i = 0 as idxint;
    while i < num_A_rows {
        *(*w).Aequil.offset(i as isize) = 1.0f64 as pfloat;
        i += 1;
    }
    i = 0 as idxint;
    while i < num_G_rows {
        *(*w).Gequil.offset(i as isize) = 1.0f64 as pfloat;
        i += 1;
    }
    iter = 0 as idxint;
    while iter < EQUIL_ITERS as i64 {
        i = 0 as idxint;
        while i < num_cols {
            *xtmp.offset(i as isize) = 0.0f64 as pfloat;
            i += 1;
        }
        i = 0 as idxint;
        while i < num_A_rows {
            *Atmp.offset(i as isize) = 0.0f64 as pfloat;
            i += 1;
        }
        i = 0 as idxint;
        while i < num_G_rows {
            *Gtmp.offset(i as isize) = 0.0f64 as pfloat;
            i += 1;
        }
        if !(*w).A.is_null() {
            max_cols(xtmp, (*w).A);
        }
        if num_G_rows > 0 as i64 {
            max_cols(xtmp, (*w).G);
        }
        if !(*w).A.is_null() {
            max_rows(Atmp, (*w).A);
        }
        if num_G_rows > 0 as i64 {
            max_rows(Gtmp, (*w).G);
        }
        ind = (*(*(*w).C).lpc).p;
        i = 0 as idxint;
        while i < (*(*w).C).nsoc {
            total = 0.0f64 as pfloat;
            j = 0 as idxint;
            while j < (*(*(*w).C).soc.offset(i as isize)).p {
                total += *Gtmp.offset((ind + j) as isize) as ::core::ffi::c_double;
                j += 1;
            }
            j = 0 as idxint;
            while j < (*(*(*w).C).soc.offset(i as isize)).p {
                *Gtmp.offset((ind + j) as isize) = total;
                j += 1;
            }
            ind += (*(*(*w).C).soc.offset(i as isize)).p as i64;
            i += 1;
        }
        i = 0 as idxint;
        while i < num_cols {
            *xtmp.offset(i as isize) =
                (if fabs(*xtmp.offset(i as isize) as ::core::ffi::c_double) < 1e-6f64 {
                    1.0f64
                } else {
                    sqrt(*xtmp.offset(i as isize) as ::core::ffi::c_double)
                }) as pfloat;
            i += 1;
        }
        i = 0 as idxint;
        while i < num_A_rows {
            *Atmp.offset(i as isize) =
                (if fabs(*Atmp.offset(i as isize) as ::core::ffi::c_double) < 1e-6f64 {
                    1.0f64
                } else {
                    sqrt(*Atmp.offset(i as isize) as ::core::ffi::c_double)
                }) as pfloat;
            i += 1;
        }
        i = 0 as idxint;
        while i < num_G_rows {
            *Gtmp.offset(i as isize) =
                (if fabs(*Gtmp.offset(i as isize) as ::core::ffi::c_double) < 1e-6f64 {
                    1.0f64
                } else {
                    sqrt(*Gtmp.offset(i as isize) as ::core::ffi::c_double)
                }) as pfloat;
            i += 1;
        }
        if !(*w).A.is_null() {
            equilibrate_rows(Atmp, (*w).A);
        }
        if num_G_rows > 0 as i64 {
            equilibrate_rows(Gtmp, (*w).G);
        }
        if !(*w).A.is_null() {
            equilibrate_cols(xtmp, (*w).A);
        }
        if num_G_rows > 0 as i64 {
            equilibrate_cols(xtmp, (*w).G);
        }
        i = 0 as idxint;
        while i < num_cols {
            let ref mut fresh0 = *(*w).xequil.offset(i as isize);
            *fresh0 *= *xtmp.offset(i as isize) as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as idxint;
        while i < num_A_rows {
            let ref mut fresh1 = *(*w).Aequil.offset(i as isize);
            *fresh1 *= *Atmp.offset(i as isize) as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as idxint;
        while i < num_G_rows {
            let ref mut fresh2 = *(*w).Gequil.offset(i as isize);
            *fresh2 *= *Gtmp.offset(i as isize) as ::core::ffi::c_double;
            i += 1;
        }
        iter += 1;
    }
    i = 0 as idxint;
    while i < num_A_rows {
        let ref mut fresh3 = *(*w).b.offset(i as isize);
        *fresh3 /= *(*w).Aequil.offset(i as isize) as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as idxint;
    while i < num_G_rows {
        let ref mut fresh4 = *(*w).h.offset(i as isize);
        *fresh4 /= *(*w).Gequil.offset(i as isize) as ::core::ffi::c_double;
        i += 1;
    }
    free(xtmp as *mut ::core::ffi::c_void);
    free(Atmp as *mut ::core::ffi::c_void);
    free(Gtmp as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn set_equilibration(mut w: *mut pwork) {
    use_ruiz_equilibration(w);
}
#[no_mangle]
pub unsafe extern "C" fn unset_equilibration(mut w: *mut pwork) {
    let mut i: idxint = 0;
    let mut num_A_rows: idxint = if !(*w).A.is_null() {
        (*(*w).A).m
    } else {
        0 as idxint
    };
    let mut num_G_rows: idxint = (*(*w).G).m;
    if !(*w).A.is_null() {
        restore((*w).Aequil, (*w).xequil, (*w).A);
    }
    if num_G_rows > 0 as i64 {
        restore((*w).Gequil, (*w).xequil, (*w).G);
    }
    i = 0 as idxint;
    while i < num_A_rows {
        let ref mut fresh7 = *(*w).b.offset(i as isize);
        *fresh7 *= *(*w).Aequil.offset(i as isize) as ::core::ffi::c_double;
        i += 1;
    }
    i = 0 as idxint;
    while i < num_G_rows {
        let ref mut fresh8 = *(*w).h.offset(i as isize);
        *fresh8 *= *(*w).Gequil.offset(i as isize) as ::core::ffi::c_double;
        i += 1;
    }
}
