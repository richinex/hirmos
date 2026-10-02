extern "C" {
    #[link_name = "honest_osqp_csc_spalloc"]
    fn csc_spalloc(
        m: OSQPInt,
        n: OSQPInt,
        nzmax: OSQPInt,
        values: OSQPInt,
        triplet: OSQPInt,
    ) -> *mut OSQPCscMatrix;
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
pub const OSQP_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn _kkt_shifts_param1(
    mut KKT: *mut OSQPCscMatrix,
    mut param1: OSQPFloat,
    mut n: OSQPInt,
    mut format: OSQPInt,
) {
    let mut i: OSQPInt = 0;
    let mut offset: OSQPInt = if format == 0 as ::core::ffi::c_int {
        1 as OSQPInt
    } else {
        0 as OSQPInt
    };
    i = 0 as ::core::ffi::c_int as OSQPInt;
    while i < n {
        let ref mut fresh2 = *(*KKT)
            .x
            .offset((*(*KKT).p.offset((i + offset) as isize) - offset) as isize);
        *fresh2 += param1 as ::core::ffi::c_double;
        i += 1;
    }
}
unsafe extern "C" fn _kkt_shifts_param2(
    mut KKT: *mut OSQPCscMatrix,
    mut param2: *mut OSQPFloat,
    mut param2_sc: OSQPFloat,
    mut startcol: OSQPInt,
    mut blockwidth: OSQPInt,
    mut format: OSQPInt,
) {
    let mut i: OSQPInt = 0;
    let mut offset: OSQPInt = if format == 0 as ::core::ffi::c_int {
        1 as OSQPInt
    } else {
        0 as OSQPInt
    };
    if !param2.is_null() {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < blockwidth {
            let ref mut fresh0 = *(*KKT)
                .x
                .offset((*(*KKT).p.offset((i + startcol + offset) as isize) - offset) as isize);
            *fresh0 -= *param2.offset(i as isize) as ::core::ffi::c_double;
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < blockwidth {
            let ref mut fresh1 = *(*KKT)
                .x
                .offset((*(*KKT).p.offset((i + startcol + offset) as isize) - offset) as isize);
            *fresh1 -= param2_sc as ::core::ffi::c_double;
            i += 1;
        }
    };
}
unsafe extern "C" fn _kkt_colcount_diag(
    mut K: *mut OSQPCscMatrix,
    mut initcol: OSQPInt,
    mut blockcols: OSQPInt,
) {
    let mut j: OSQPInt = 0;
    j = initcol;
    while j < initcol + blockcols {
        let ref mut fresh7 = *(*K).p.offset(j as isize);
        *fresh7 += 1;
        j += 1;
    }
}
unsafe extern "C" fn _kkt_colcount_missing_diag(
    mut K: *mut OSQPCscMatrix,
    mut M: *mut OSQPCscMatrix,
    mut initcol: OSQPInt,
) {
    let mut j: OSQPInt = 0;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*M).n {
        if *(*M).p.offset(j as isize)
            == *(*M)
                .p
                .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            || *(*M).i.offset(
                (*(*M)
                    .p
                    .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as isize,
            ) != j
        {
            let ref mut fresh10 = *(*K).p.offset((j + initcol) as isize);
            *fresh10 += 1;
        }
        j += 1;
    }
}
unsafe extern "C" fn _kkt_colcount_block(
    mut K: *mut OSQPCscMatrix,
    mut M: *mut OSQPCscMatrix,
    mut initcol: OSQPInt,
    mut istranspose: OSQPInt,
) {
    let mut nnzM: OSQPInt = 0;
    let mut j: OSQPInt = 0;
    if istranspose != 0 {
        nnzM = *(*M).p.offset((*M).n as isize);
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < nnzM {
            let ref mut fresh8 = *(*K)
                .p
                .offset((*(*M).i.offset(j as isize) + initcol) as isize);
            *fresh8 += 1;
            j += 1;
        }
    } else {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < (*M).n {
            let ref mut fresh9 = *(*K).p.offset((j + initcol) as isize);
            *fresh9 += (*(*M)
                .p
                .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                - *(*M).p.offset(j as isize)) as ::core::ffi::c_int;
            j += 1;
        }
    };
}
unsafe extern "C" fn _kkt_fill_block(
    mut K: *mut OSQPCscMatrix,
    mut M: *mut OSQPCscMatrix,
    mut MtoKKT: *mut OSQPInt,
    mut initrow: OSQPInt,
    mut initcol: OSQPInt,
    mut istranspose: OSQPInt,
) {
    let mut ii: OSQPInt = 0;
    let mut jj: OSQPInt = 0;
    let mut row: OSQPInt = 0;
    let mut col: OSQPInt = 0;
    let mut dest: OSQPInt = 0;
    ii = 0 as ::core::ffi::c_int as OSQPInt;
    while ii < (*M).n {
        jj = *(*M).p.offset(ii as isize);
        while jj
            < *(*M)
                .p
                .offset((ii as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        {
            if istranspose != 0 {
                col = *(*M).i.offset(jj as isize) + initcol;
                row = ii + initrow;
            } else {
                col = ii + initcol;
                row = *(*M).i.offset(jj as isize) + initrow;
            }
            let ref mut fresh4 = *(*K).p.offset(col as isize);
            let fresh5 = *fresh4;
            *fresh4 = *fresh4 + 1;
            dest = fresh5;
            *(*K).i.offset(dest as isize) = row;
            *(*K).x.offset(dest as isize) = *(*M).x.offset(jj as isize);
            if !MtoKKT.is_null() {
                *MtoKKT.offset(jj as isize) = dest;
            }
            jj += 1;
        }
        ii += 1;
    }
}
unsafe extern "C" fn _kkt_fill_diag_zeros(
    mut K: *mut OSQPCscMatrix,
    mut rhotoKKT: *mut OSQPInt,
    mut offset: OSQPInt,
    mut blockdim: OSQPInt,
) {
    let mut j: OSQPInt = 0;
    let mut dest: OSQPInt = 0;
    let mut col: OSQPInt = 0;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < blockdim {
        col = j + offset;
        dest = *(*K).p.offset(col as isize);
        *(*K).i.offset(dest as isize) = col;
        *(*K).x.offset(dest as isize) = 0.0f64 as OSQPFloat;
        let ref mut fresh3 = *(*K).p.offset(col as isize);
        *fresh3 += 1;
        if !rhotoKKT.is_null() {
            *rhotoKKT.offset(j as isize) = dest;
        }
        j += 1;
    }
}
unsafe extern "C" fn _kkt_fill_missing_diag_zeros(
    mut K: *mut OSQPCscMatrix,
    mut M: *mut OSQPCscMatrix,
    mut offset: OSQPInt,
) {
    let mut j: OSQPInt = 0;
    let mut dest: OSQPInt = 0;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*M).n {
        if *(*M).p.offset(j as isize)
            == *(*M)
                .p
                .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            || *(*M).i.offset(
                (*(*M)
                    .p
                    .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as isize,
            ) != j
        {
            dest = *(*K).p.offset((j + offset) as isize);
            *(*K).i.offset(dest as isize) = j + offset;
            *(*K).x.offset(dest as isize) = 0.0f64 as OSQPFloat;
            let ref mut fresh6 = *(*K).p.offset(j as isize);
            *fresh6 += 1;
        }
        j += 1;
    }
}
unsafe extern "C" fn _kkt_colcount_to_colptr(mut K: *mut OSQPCscMatrix) {
    let mut j: OSQPInt = 0;
    let mut count: OSQPInt = 0;
    let mut currentptr: OSQPInt = 0 as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j <= (*K).n {
        count = *(*K).p.offset(j as isize);
        *(*K).p.offset(j as isize) = currentptr;
        currentptr += count as ::core::ffi::c_int;
        j += 1;
    }
}
unsafe extern "C" fn _kkt_backshift_colptrs(mut K: *mut OSQPCscMatrix) {
    let mut j: ::core::ffi::c_int = 0;
    j = (*K).n as ::core::ffi::c_int;
    while j > 0 as ::core::ffi::c_int {
        *(*K).p.offset(j as isize) = *(*K).p.offset((j - 1 as ::core::ffi::c_int) as isize);
        j -= 1;
    }
    *(*K).p.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int as OSQPInt;
}
unsafe extern "C" fn _count_diagonal_entries(mut P: *mut OSQPCscMatrix) -> OSQPInt {
    let mut j: OSQPInt = 0;
    let mut count: OSQPInt = 0 as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*P).n {
        if *(*P)
            .p
            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            != *(*P).p.offset(j as isize)
            && *(*P).i.offset(
                (*(*P)
                    .p
                    .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as isize,
            ) == j
        {
            count += 1;
        }
        j += 1;
    }
    return count;
}
unsafe extern "C" fn _kkt_assemble_csr(
    mut K: *mut OSQPCscMatrix,
    mut PtoKKT: *mut OSQPInt,
    mut AtoKKT: *mut OSQPInt,
    mut rhotoKKT: *mut OSQPInt,
    mut P: *mut OSQPCscMatrix,
    mut A: *mut OSQPCscMatrix,
) {
    let mut j: OSQPInt = 0;
    let mut m: OSQPInt = (*A).m;
    let mut n: OSQPInt = (*P).n;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j <= m + n {
        *(*K).p.offset(j as isize) = 0 as ::core::ffi::c_int as OSQPInt;
        j += 1;
    }
    _kkt_colcount_missing_diag(K, P, 0 as OSQPInt);
    _kkt_colcount_block(K, P, 0 as OSQPInt, 1 as OSQPInt);
    _kkt_colcount_block(K, A, 0 as OSQPInt, 0 as OSQPInt);
    _kkt_colcount_diag(K, n, m);
    _kkt_colcount_to_colptr(K);
    _kkt_fill_missing_diag_zeros(K, P, 0 as OSQPInt);
    _kkt_fill_block(K, P, PtoKKT, 0 as OSQPInt, 0 as OSQPInt, 1 as OSQPInt);
    _kkt_fill_block(K, A, AtoKKT, n, 0 as OSQPInt, 0 as OSQPInt);
    _kkt_fill_diag_zeros(K, rhotoKKT, n, m);
    _kkt_backshift_colptrs(K);
}
unsafe extern "C" fn _kkt_assemble_csc(
    mut K: *mut OSQPCscMatrix,
    mut PtoKKT: *mut OSQPInt,
    mut AtoKKT: *mut OSQPInt,
    mut rhotoKKT: *mut OSQPInt,
    mut P: *mut OSQPCscMatrix,
    mut A: *mut OSQPCscMatrix,
) {
    let mut j: OSQPInt = 0;
    let mut m: OSQPInt = (*A).m;
    let mut n: OSQPInt = (*P).n;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j <= m + n {
        *(*K).p.offset(j as isize) = 0 as ::core::ffi::c_int as OSQPInt;
        j += 1;
    }
    _kkt_colcount_block(K, P, 0 as OSQPInt, 0 as OSQPInt);
    _kkt_colcount_missing_diag(K, P, 0 as OSQPInt);
    _kkt_colcount_block(K, A, n, 1 as OSQPInt);
    _kkt_colcount_diag(K, n, m);
    _kkt_colcount_to_colptr(K);
    _kkt_fill_block(K, P, PtoKKT, 0 as OSQPInt, 0 as OSQPInt, 0 as OSQPInt);
    _kkt_fill_missing_diag_zeros(K, P, 0 as OSQPInt);
    _kkt_fill_block(K, A, AtoKKT, 0 as OSQPInt, n, 1 as OSQPInt);
    _kkt_fill_diag_zeros(K, rhotoKKT, n, m);
    _kkt_backshift_colptrs(K);
}
#[export_name = "honest_osqp_form_KKT"]
pub unsafe extern "C" fn form_KKT(
    mut P: *mut OSQPCscMatrix,
    mut A: *mut OSQPCscMatrix,
    mut format: OSQPInt,
    mut param1: OSQPFloat,
    mut param2: *mut OSQPFloat,
    mut param2_sc: OSQPFloat,
    mut PtoKKT: *mut OSQPInt,
    mut AtoKKT: *mut OSQPInt,
    mut rhotoKKT: *mut OSQPInt,
) -> *mut OSQPCscMatrix {
    let mut m: OSQPInt = 0;
    let mut n: OSQPInt = 0;
    let mut nKKT: OSQPInt = 0;
    let mut nnzKKT: OSQPInt = 0;
    let mut ndiagP: OSQPInt = 0;
    let mut KKT: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    m = (*A).m;
    n = (*P).n;
    nKKT = m + n;
    ndiagP = _count_diagonal_entries(P);
    nnzKKT = *(*P).p.offset(n as isize) + n - ndiagP + *(*A).p.offset(n as isize) + m;
    KKT = csc_spalloc(nKKT, nKKT, nnzKKT, 1 as OSQPInt, 0 as OSQPInt);
    if KKT.is_null() {
        return ::core::ptr::null_mut::<OSQPCscMatrix>();
    }
    if format == 0 as ::core::ffi::c_int {
        _kkt_assemble_csc(KKT, PtoKKT, AtoKKT, rhotoKKT, P, A);
    } else {
        _kkt_assemble_csr(KKT, PtoKKT, AtoKKT, rhotoKKT, P, A);
    }
    _kkt_shifts_param1(KKT, param1, n, format);
    _kkt_shifts_param2(KKT, param2, param2_sc, n, m, format);
    return KKT;
}
#[export_name = "honest_osqp_update_KKT_P"]
pub unsafe extern "C" fn update_KKT_P(
    mut KKT: *mut OSQPCscMatrix,
    mut P: *mut OSQPCscMatrix,
    mut Px_new_idx: *const OSQPInt,
    mut P_new_n: OSQPInt,
    mut PtoKKT: *mut OSQPInt,
    mut param1: OSQPFloat,
    mut format: OSQPInt,
) {
    let mut j: OSQPInt = 0;
    let mut Pidx: OSQPInt = 0;
    let mut Kidx: OSQPInt = 0;
    let mut row: OSQPInt = 0;
    let mut offset: OSQPInt = 0;
    let mut doall: OSQPInt = 0;
    if P_new_n <= 0 as ::core::ffi::c_int {
        return;
    }
    doall = (if Px_new_idx.is_null() {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as OSQPInt;
    offset = (if format == 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < P_new_n {
        Pidx = (if doall != 0 {
            j as ::core::ffi::c_int
        } else {
            *Px_new_idx.offset(j as isize) as ::core::ffi::c_int
        }) as OSQPInt;
        Kidx = *PtoKKT.offset(Pidx as isize);
        *(*KKT).x.offset(Kidx as isize) = *(*P).x.offset(Pidx as isize);
        row = *(*P).i.offset(Pidx as isize);
        if *(*P).p.offset(row as isize)
            < *(*P)
                .p
                .offset((row as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            && *(*P).p.offset((row + offset) as isize) - offset == Pidx
        {
            let ref mut fresh11 = *(*KKT).x.offset(Kidx as isize);
            *fresh11 += param1 as ::core::ffi::c_double;
        }
        j += 1;
    }
}
#[export_name = "honest_osqp_update_KKT_A"]
pub unsafe extern "C" fn update_KKT_A(
    mut KKT: *mut OSQPCscMatrix,
    mut A: *mut OSQPCscMatrix,
    mut Ax_new_idx: *const OSQPInt,
    mut A_new_n: OSQPInt,
    mut AtoKKT: *mut OSQPInt,
) {
    let mut j: OSQPInt = 0;
    let mut Aidx: OSQPInt = 0;
    let mut Kidx: OSQPInt = 0;
    let mut doall: OSQPInt = 0;
    if A_new_n <= 0 as ::core::ffi::c_int {
        return;
    }
    doall = (if Ax_new_idx.is_null() {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < A_new_n {
        Aidx = (if doall != 0 {
            j as ::core::ffi::c_int
        } else {
            *Ax_new_idx.offset(j as isize) as ::core::ffi::c_int
        }) as OSQPInt;
        Kidx = *AtoKKT.offset(Aidx as isize);
        *(*KKT).x.offset(Kidx as isize) = *(*A).x.offset(Aidx as isize);
        j += 1;
    }
}
#[export_name = "honest_osqp_update_KKT_param2"]
pub unsafe extern "C" fn update_KKT_param2(
    mut KKT: *mut OSQPCscMatrix,
    mut param2: *mut OSQPFloat,
    mut param2_sc: OSQPFloat,
    mut param2toKKT: *mut OSQPInt,
    mut m: OSQPInt,
) {
    let mut i: OSQPInt = 0;
    if !param2.is_null() {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < m {
            *(*KKT).x.offset(*param2toKKT.offset(i as isize) as isize) =
                -*param2.offset(i as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < m {
            *(*KKT).x.offset(*param2toKKT.offset(i as isize) as isize) = -param2_sc;
            i += 1;
        }
    };
}
