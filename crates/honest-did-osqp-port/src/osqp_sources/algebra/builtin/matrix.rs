use crate::runtime::{malloc,free};
extern "C" {
    #[link_name = "honest_osqp_OSQPVectorf_data"]
    fn OSQPVectorf_data(a: *const OSQPVectorf) -> *mut OSQPFloat;
    #[link_name = "honest_osqp_csc_update_values"]
    fn csc_update_values(
        M: *mut OSQPCscMatrix,
        Mx_new: *const OSQPFloat,
        Mx_new_idx: *const OSQPInt,
        P_new_n: OSQPInt,
    );
    #[link_name = "honest_osqp_csc_scale"]
    fn csc_scale(A: *mut OSQPCscMatrix, sc: OSQPFloat);
    #[link_name = "honest_osqp_csc_lmult_diag"]
    fn csc_lmult_diag(A: *mut OSQPCscMatrix, L: *const OSQPFloat);
    #[link_name = "honest_osqp_csc_rmult_diag"]
    fn csc_rmult_diag(A: *mut OSQPCscMatrix, R: *const OSQPFloat);
    #[link_name = "honest_osqp_csc_AtDA_extract_diag"]
    fn csc_AtDA_extract_diag(A: *const OSQPCscMatrix, D: *const OSQPFloat, d: *mut OSQPFloat);
    #[link_name = "honest_osqp_csc_Axpy_sym_triu"]
    fn csc_Axpy_sym_triu(
        A: *const OSQPCscMatrix,
        x: *const OSQPFloat,
        y: *mut OSQPFloat,
        alpha: OSQPFloat,
        beta: OSQPFloat,
    );
    #[link_name = "honest_osqp_csc_Axpy"]
    fn csc_Axpy(
        A: *const OSQPCscMatrix,
        x: *const OSQPFloat,
        y: *mut OSQPFloat,
        alpha: OSQPFloat,
        beta: OSQPFloat,
    );
    #[link_name = "honest_osqp_csc_Atxpy"]
    fn csc_Atxpy(
        A: *const OSQPCscMatrix,
        x: *const OSQPFloat,
        y: *mut OSQPFloat,
        alpha: OSQPFloat,
        beta: OSQPFloat,
    );
    #[link_name = "honest_osqp_csc_col_norm_inf"]
    fn csc_col_norm_inf(M: *const OSQPCscMatrix, E: *mut OSQPFloat);
    #[link_name = "honest_osqp_csc_row_norm_inf"]
    fn csc_row_norm_inf(M: *const OSQPCscMatrix, E: *mut OSQPFloat);
    #[link_name = "honest_osqp_csc_row_norm_inf_sym_triu"]
    fn csc_row_norm_inf_sym_triu(M: *const OSQPCscMatrix, E: *mut OSQPFloat);
    #[link_name = "honest_osqp_csc_is_eq"]
    fn csc_is_eq(A: *mut OSQPCscMatrix, B: *mut OSQPCscMatrix, tol: OSQPFloat) -> OSQPInt;
    #[link_name = "honest_osqp_csc_spfree"]
    fn csc_spfree(A: *mut OSQPCscMatrix);
    #[link_name = "honest_osqp_csc_submatrix_byrows"]
    fn csc_submatrix_byrows(A: *const OSQPCscMatrix, rows: *mut OSQPInt) -> *mut OSQPCscMatrix;
    #[link_name = "honest_osqp_csc_copy"]
    fn csc_copy(A: *const OSQPCscMatrix) -> *mut OSQPCscMatrix;
    #[link_name = "honest_osqp_triu_to_csc"]
    fn triu_to_csc(M: *mut OSQPCscMatrix) -> *mut OSQPCscMatrix;
    #[link_name = "honest_osqp_vstack"]
    fn vstack(A: *mut OSQPCscMatrix, B: *mut OSQPCscMatrix) -> *mut OSQPCscMatrix;
    #[link_name = "honest_osqp_csc_extract_diag"]
    fn csc_extract_diag(A: *const OSQPCscMatrix, d: *mut OSQPFloat);
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
pub type OSQPVectorf = OSQPVectorf_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPVectorf_ {
    pub values: *mut OSQPFloat,
    pub length: OSQPInt,
}
pub type OSQPVectori = OSQPVectori_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPVectori_ {
    pub values: *mut OSQPInt,
    pub length: OSQPInt,
}
pub type OSQPMatrix = OSQPMatrix_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPMatrix_ {
    pub csc: *mut OSQPCscMatrix,
    pub symmetry: OSQPMatrix_symmetry_type,
}
pub type OSQPMatrix_symmetry_type = ::core::ffi::c_uint;
pub const TRIU: OSQPMatrix_symmetry_type = 1;
pub const NONE: OSQPMatrix_symmetry_type = 0;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub const OSQP_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "honest_osqp_OSQPMatrix_is_eq"]
pub unsafe extern "C" fn OSQPMatrix_is_eq(
    mut A: *const OSQPMatrix,
    mut B: *const OSQPMatrix,
    mut tol: OSQPFloat,
) -> OSQPInt {
    return ((*A).symmetry as ::core::ffi::c_uint == (*B).symmetry as ::core::ffi::c_uint
        && csc_is_eq((*A).csc, (*B).csc, tol) != 0) as ::core::ffi::c_int;
}
#[export_name = "honest_osqp_OSQPMatrix_new_from_csc"]
pub unsafe extern "C" fn OSQPMatrix_new_from_csc(
    mut A: *const OSQPCscMatrix,
    mut is_triu: OSQPInt,
) -> *mut OSQPMatrix {
    let mut out: *mut OSQPMatrix =
        malloc(::core::mem::size_of::<OSQPMatrix>() as size_t) as *mut OSQPMatrix;
    if out.is_null() {
        return ::core::ptr::null_mut::<OSQPMatrix>();
    }
    if is_triu != 0 {
        (*out).symmetry = TRIU;
    } else {
        (*out).symmetry = NONE;
    }
    (*out).csc = csc_copy(A);
    if (*out).csc.is_null() {
        free(out as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<OSQPMatrix>();
    } else {
        return out;
    };
}
#[export_name = "honest_osqp_OSQPMatrix_get_csc"]
pub unsafe extern "C" fn OSQPMatrix_get_csc(mut M: *const OSQPMatrix) -> *mut OSQPCscMatrix {
    return csc_copy((*M).csc);
}
#[export_name = "honest_osqp_OSQPMatrix_copy_new"]
pub unsafe extern "C" fn OSQPMatrix_copy_new(mut A: *const OSQPMatrix) -> *mut OSQPMatrix {
    let mut out: *mut OSQPMatrix =
        malloc(::core::mem::size_of::<OSQPMatrix>() as size_t) as *mut OSQPMatrix;
    if out.is_null() {
        return ::core::ptr::null_mut::<OSQPMatrix>();
    }
    (*out).symmetry = (*A).symmetry;
    (*out).csc = csc_copy((*A).csc);
    if (*out).csc.is_null() {
        free(out as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<OSQPMatrix>();
    } else {
        return out;
    };
}
#[export_name = "honest_osqp_OSQPMatrix_triu_to_symm"]
pub unsafe extern "C" fn OSQPMatrix_triu_to_symm(mut A: *const OSQPMatrix) -> *mut OSQPMatrix {
    if (*A).symmetry as ::core::ffi::c_uint == TRIU as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut out: *mut OSQPMatrix =
            malloc(::core::mem::size_of::<OSQPMatrix>() as size_t) as *mut OSQPMatrix;
        if out.is_null() {
            return ::core::ptr::null_mut::<OSQPMatrix>();
        }
        (*out).symmetry = NONE;
        (*out).csc = triu_to_csc((*A).csc);
        if (*out).csc.is_null() {
            free(out as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<OSQPMatrix>();
        } else {
            return out;
        }
    } else {
        return ::core::ptr::null_mut::<OSQPMatrix>();
    };
}
#[export_name = "honest_osqp_OSQPMatrix_vstack"]
pub unsafe extern "C" fn OSQPMatrix_vstack(
    mut A: *const OSQPMatrix,
    mut B: *const OSQPMatrix,
) -> *mut OSQPMatrix {
    if (*A).symmetry as ::core::ffi::c_uint == NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*B).symmetry as ::core::ffi::c_uint == NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut out: *mut OSQPMatrix =
            malloc(::core::mem::size_of::<OSQPMatrix>() as size_t) as *mut OSQPMatrix;
        if out.is_null() {
            return ::core::ptr::null_mut::<OSQPMatrix>();
        }
        (*out).symmetry = NONE;
        (*out).csc = vstack((*A).csc, (*B).csc);
        if (*out).csc.is_null() {
            free(out as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<OSQPMatrix>();
        } else {
            return out;
        }
    } else {
        return ::core::ptr::null_mut::<OSQPMatrix>();
    };
}
#[export_name = "honest_osqp_OSQPMatrix_update_values"]
pub unsafe extern "C" fn OSQPMatrix_update_values(
    mut M: *mut OSQPMatrix,
    mut Mx_new: *const OSQPFloat,
    mut Mx_new_idx: *const OSQPInt,
    mut M_new_n: OSQPInt,
) {
    csc_update_values((*M).csc, Mx_new, Mx_new_idx, M_new_n);
}
#[export_name = "honest_osqp_OSQPMatrix_get_m"]
pub unsafe extern "C" fn OSQPMatrix_get_m(mut M: *const OSQPMatrix) -> OSQPInt {
    return (*(*M).csc).m;
}
#[export_name = "honest_osqp_OSQPMatrix_get_n"]
pub unsafe extern "C" fn OSQPMatrix_get_n(mut M: *const OSQPMatrix) -> OSQPInt {
    return (*(*M).csc).n;
}
#[export_name = "honest_osqp_OSQPMatrix_get_x"]
pub unsafe extern "C" fn OSQPMatrix_get_x(mut M: *const OSQPMatrix) -> *mut OSQPFloat {
    return (*(*M).csc).x;
}
#[export_name = "honest_osqp_OSQPMatrix_get_i"]
pub unsafe extern "C" fn OSQPMatrix_get_i(mut M: *const OSQPMatrix) -> *mut OSQPInt {
    return (*(*M).csc).i;
}
#[export_name = "honest_osqp_OSQPMatrix_get_p"]
pub unsafe extern "C" fn OSQPMatrix_get_p(mut M: *const OSQPMatrix) -> *mut OSQPInt {
    return (*(*M).csc).p;
}
#[export_name = "honest_osqp_OSQPMatrix_get_nz"]
pub unsafe extern "C" fn OSQPMatrix_get_nz(mut M: *const OSQPMatrix) -> OSQPInt {
    return *(*(*M).csc).p.offset((*(*M).csc).n as isize);
}
#[export_name = "honest_osqp_OSQPMatrix_mult_scalar"]
pub unsafe extern "C" fn OSQPMatrix_mult_scalar(mut A: *mut OSQPMatrix, mut sc: OSQPFloat) {
    csc_scale((*A).csc, sc);
}
#[export_name = "honest_osqp_OSQPMatrix_lmult_diag"]
pub unsafe extern "C" fn OSQPMatrix_lmult_diag(mut A: *mut OSQPMatrix, mut L: *const OSQPVectorf) {
    csc_lmult_diag((*A).csc, OSQPVectorf_data(L));
}
#[export_name = "honest_osqp_OSQPMatrix_rmult_diag"]
pub unsafe extern "C" fn OSQPMatrix_rmult_diag(mut A: *mut OSQPMatrix, mut R: *const OSQPVectorf) {
    csc_rmult_diag((*A).csc, (*R).values);
}
#[export_name = "honest_osqp_OSQPMatrix_AtDA_extract_diag"]
pub unsafe extern "C" fn OSQPMatrix_AtDA_extract_diag(
    mut A: *const OSQPMatrix,
    mut D: *const OSQPVectorf,
    mut d: *mut OSQPVectorf,
) {
    csc_AtDA_extract_diag((*A).csc, OSQPVectorf_data(D), OSQPVectorf_data(d));
}
#[export_name = "honest_osqp_OSQPMatrix_extract_diag"]
pub unsafe extern "C" fn OSQPMatrix_extract_diag(
    mut A: *const OSQPMatrix,
    mut d: *mut OSQPVectorf,
) {
    csc_extract_diag((*A).csc, OSQPVectorf_data(d));
}
#[export_name = "honest_osqp_OSQPMatrix_Axpy"]
pub unsafe extern "C" fn OSQPMatrix_Axpy(
    mut A: *const OSQPMatrix,
    mut x: *const OSQPVectorf,
    mut y: *mut OSQPVectorf,
    mut alpha: OSQPFloat,
    mut beta: OSQPFloat,
) {
    if (*A).symmetry as ::core::ffi::c_uint == NONE as ::core::ffi::c_int as ::core::ffi::c_uint {
        csc_Axpy((*A).csc, (*x).values, (*y).values, alpha, beta);
    } else {
        csc_Axpy_sym_triu((*A).csc, (*x).values, (*y).values, alpha, beta);
    };
}
#[export_name = "honest_osqp_OSQPMatrix_Atxpy"]
pub unsafe extern "C" fn OSQPMatrix_Atxpy(
    mut A: *const OSQPMatrix,
    mut x: *const OSQPVectorf,
    mut y: *mut OSQPVectorf,
    mut alpha: OSQPFloat,
    mut beta: OSQPFloat,
) {
    if (*A).symmetry as ::core::ffi::c_uint == NONE as ::core::ffi::c_int as ::core::ffi::c_uint {
        csc_Atxpy((*A).csc, (*x).values, (*y).values, alpha, beta);
    } else {
        csc_Axpy_sym_triu((*A).csc, (*x).values, (*y).values, alpha, beta);
    };
}
#[export_name = "honest_osqp_OSQPMatrix_col_norm_inf"]
pub unsafe extern "C" fn OSQPMatrix_col_norm_inf(
    mut M: *const OSQPMatrix,
    mut E: *mut OSQPVectorf,
) {
    csc_col_norm_inf((*M).csc, OSQPVectorf_data(E));
}
#[export_name = "honest_osqp_OSQPMatrix_row_norm_inf"]
pub unsafe extern "C" fn OSQPMatrix_row_norm_inf(
    mut M: *const OSQPMatrix,
    mut E: *mut OSQPVectorf,
) {
    if (*M).symmetry as ::core::ffi::c_uint == NONE as ::core::ffi::c_int as ::core::ffi::c_uint {
        csc_row_norm_inf((*M).csc, OSQPVectorf_data(E));
    } else {
        csc_row_norm_inf_sym_triu((*M).csc, OSQPVectorf_data(E));
    };
}
#[export_name = "honest_osqp_OSQPMatrix_free"]
pub unsafe extern "C" fn OSQPMatrix_free(mut M: *mut OSQPMatrix) {
    if !M.is_null() {
        csc_spfree((*M).csc);
    }
    free(M as *mut ::core::ffi::c_void);
}
#[export_name = "honest_osqp_OSQPMatrix_submatrix_byrows"]
pub unsafe extern "C" fn OSQPMatrix_submatrix_byrows(
    mut A: *const OSQPMatrix,
    mut rows: *const OSQPVectori,
) -> *mut OSQPMatrix {
    let mut M: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    let mut out: *mut OSQPMatrix = ::core::ptr::null_mut::<OSQPMatrix>();
    if (*A).symmetry as ::core::ffi::c_uint == TRIU as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ::core::ptr::null_mut::<OSQPMatrix>();
    }
    M = csc_submatrix_byrows((*A).csc, (*rows).values);
    if M.is_null() {
        return ::core::ptr::null_mut::<OSQPMatrix>();
    }
    out = malloc(::core::mem::size_of::<OSQPMatrix>() as size_t) as *mut OSQPMatrix;
    if out.is_null() {
        csc_spfree(M);
        return ::core::ptr::null_mut::<OSQPMatrix>();
    }
    (*out).symmetry = NONE;
    (*out).csc = M;
    return out;
}
