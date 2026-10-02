use crate::runtime::{malloc,calloc,free};
extern "C" {
    #[link_name = "honest_osqp__osqp_error"]
    fn _osqp_error(
        error_code: osqp_error_type,
        function_name: *const ::core::ffi::c_char,
    ) -> OSQPInt;
    #[link_name = "honest_osqp_QDLDL_etree"]
    fn QDLDL_etree(
        n: QDLDL_int,
        Ap: *const QDLDL_int,
        Ai: *const QDLDL_int,
        work: *mut QDLDL_int,
        Lnz: *mut QDLDL_int,
        etree: *mut QDLDL_int,
    ) -> QDLDL_int;
    #[link_name = "honest_osqp_QDLDL_factor"]
    fn QDLDL_factor(
        n: QDLDL_int,
        Ap: *const QDLDL_int,
        Ai: *const QDLDL_int,
        Ax: *const QDLDL_float,
        Lp: *mut QDLDL_int,
        Li: *mut QDLDL_int,
        Lx: *mut QDLDL_float,
        D: *mut QDLDL_float,
        Dinv: *mut QDLDL_float,
        Lnz: *const QDLDL_int,
        etree: *const QDLDL_int,
        bwork: *mut QDLDL_bool,
        iwork: *mut QDLDL_int,
        fwork: *mut QDLDL_float,
    ) -> QDLDL_int;
    #[link_name = "honest_osqp_QDLDL_solve"]
    fn QDLDL_solve(
        n: QDLDL_int,
        Lp: *const QDLDL_int,
        Li: *const QDLDL_int,
        Lx: *const QDLDL_float,
        Dinv: *const QDLDL_float,
        x: *mut QDLDL_float,
    );
    #[link_name = "honest_osqp_OSQPVectorf_malloc"]
    fn OSQPVectorf_malloc(length: OSQPInt) -> *mut OSQPVectorf;
    #[link_name = "honest_osqp_OSQPVectorf_new"]
    fn OSQPVectorf_new(a: *const OSQPFloat, length: OSQPInt) -> *mut OSQPVectorf;
    #[link_name = "honest_osqp_OSQPVectorf_free"]
    fn OSQPVectorf_free(a: *mut OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_subvector_assign"]
    fn OSQPVectorf_subvector_assign(
        A: *mut OSQPVectorf,
        b: *mut OSQPFloat,
        start: OSQPInt,
        length: OSQPInt,
        multiplier: OSQPFloat,
    );
    #[link_name = "honest_osqp_OSQPVectorf_length"]
    fn OSQPVectorf_length(a: *const OSQPVectorf) -> OSQPInt;
    #[link_name = "honest_osqp_OSQPVectorf_data"]
    fn OSQPVectorf_data(a: *const OSQPVectorf) -> *mut OSQPFloat;
    #[link_name = "honest_osqp_OSQPVectorf_copy"]
    fn OSQPVectorf_copy(b: *mut OSQPVectorf, a: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_minus"]
    fn OSQPVectorf_minus(x: *mut OSQPVectorf, a: *const OSQPVectorf, b: *const OSQPVectorf);
    #[link_name = "honest_osqp_OSQPVectorf_norm_2"]
    fn OSQPVectorf_norm_2(v: *const OSQPVectorf) -> OSQPFloat;
    #[link_name = "honest_osqp_OSQPMatrix_new_from_csc"]
    fn OSQPMatrix_new_from_csc(A: *const OSQPCscMatrix, is_triu: OSQPInt) -> *mut OSQPMatrix;
    #[link_name = "honest_osqp_OSQPMatrix_get_m"]
    fn OSQPMatrix_get_m(M: *const OSQPMatrix) -> OSQPInt;
    #[link_name = "honest_osqp_OSQPMatrix_get_nz"]
    fn OSQPMatrix_get_nz(M: *const OSQPMatrix) -> OSQPInt;
    #[link_name = "honest_osqp_OSQPMatrix_Axpy"]
    fn OSQPMatrix_Axpy(
        A: *const OSQPMatrix,
        x: *const OSQPVectorf,
        y: *mut OSQPVectorf,
        alpha: OSQPFloat,
        beta: OSQPFloat,
    );
    #[link_name = "honest_osqp_OSQPMatrix_free"]
    fn OSQPMatrix_free(M: *mut OSQPMatrix);
    #[link_name = "honest_osqp_amd_order"]
    fn amd_order(
        n: ::core::ffi::c_int,
        Ap: *const ::core::ffi::c_int,
        Ai: *const ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        Control: *mut OSQPFloat,
        Info: *mut OSQPFloat,
    ) -> ::core::ffi::c_int;
    #[link_name = "honest_osqp_csc_spalloc"]
    fn csc_spalloc(
        m: OSQPInt,
        n: OSQPInt,
        nzmax: OSQPInt,
        values: OSQPInt,
        triplet: OSQPInt,
    ) -> *mut OSQPCscMatrix;
    #[link_name = "honest_osqp_csc_spfree"]
    fn csc_spfree(A: *mut OSQPCscMatrix);
    #[link_name = "honest_osqp_csc_pinv"]
    fn csc_pinv(p: *const OSQPInt, n: OSQPInt) -> *mut OSQPInt;
    #[link_name = "honest_osqp_csc_symperm"]
    fn csc_symperm(
        A: *const OSQPCscMatrix,
        pinv: *const OSQPInt,
        AtoC: *mut OSQPInt,
        values: OSQPInt,
    ) -> *mut OSQPCscMatrix;
    #[link_name = "honest_osqp_form_KKT"]
    fn form_KKT(
        P: *mut OSQPCscMatrix,
        A: *mut OSQPCscMatrix,
        format: OSQPInt,
        param1: OSQPFloat,
        param2: *mut OSQPFloat,
        param2_sc: OSQPFloat,
        PtoKKT: *mut OSQPInt,
        AtoKKT: *mut OSQPInt,
        param2toKKT: *mut OSQPInt,
    ) -> *mut OSQPCscMatrix;
    #[link_name = "honest_osqp_update_KKT_P"]
    fn update_KKT_P(
        KKT: *mut OSQPCscMatrix,
        P: *mut OSQPCscMatrix,
        Px_new_idx: *const OSQPInt,
        P_new_n: OSQPInt,
        PtoKKT: *mut OSQPInt,
        param1: OSQPFloat,
        format: OSQPInt,
    );
    #[link_name = "honest_osqp_update_KKT_A"]
    fn update_KKT_A(
        KKT: *mut OSQPCscMatrix,
        A: *mut OSQPCscMatrix,
        Ax_new_idx: *const OSQPInt,
        A_new_n: OSQPInt,
        AtoKKT: *mut OSQPInt,
    );
    #[link_name = "honest_osqp_update_KKT_param2"]
    fn update_KKT_param2(
        KKT: *mut OSQPCscMatrix,
        param2: *mut OSQPFloat,
        param2_sc: OSQPFloat,
        param2toKKT: *mut OSQPInt,
        m: OSQPInt,
    );
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type osqp_linsys_solver_type = ::core::ffi::c_uint;
pub const OSQP_INDIRECT_SOLVER: osqp_linsys_solver_type = 2;
pub const OSQP_DIRECT_SOLVER: osqp_linsys_solver_type = 1;
pub const OSQP_UNKNOWN_SOLVER: osqp_linsys_solver_type = 0;
pub type osqp_precond_type = ::core::ffi::c_uint;
pub const OSQP_DIAGONAL_PRECONDITIONER: osqp_precond_type = 1;
pub const OSQP_NO_PRECONDITIONER: osqp_precond_type = 0;
pub type osqp_error_type = ::core::ffi::c_uint;
pub const OSQP_LAST_ERROR_PLACE: osqp_error_type = 12;
pub const OSQP_FUNC_NOT_IMPLEMENTED: osqp_error_type = 11;
pub const OSQP_DATA_NOT_INITIALIZED: osqp_error_type = 10;
pub const OSQP_CODEGEN_DEFINES_ERROR: osqp_error_type = 9;
pub const OSQP_FOPEN_ERROR: osqp_error_type = 8;
pub const OSQP_ALGEBRA_LOAD_ERROR: osqp_error_type = 7;
pub const OSQP_WORKSPACE_NOT_INIT_ERROR: osqp_error_type = 6;
pub const OSQP_MEM_ALLOC_ERROR: osqp_error_type = 5;
pub const OSQP_NONCVX_ERROR: osqp_error_type = 4;
pub const OSQP_LINSYS_SOLVER_INIT_ERROR: osqp_error_type = 3;
pub const OSQP_SETTINGS_VALIDATION_ERROR: osqp_error_type = 2;
pub const OSQP_DATA_VALIDATION_ERROR: osqp_error_type = 1;
pub const OSQP_NO_ERROR: osqp_error_type = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPSettings {
    pub device: OSQPInt,
    pub linsys_solver: osqp_linsys_solver_type,
    pub allocate_solution: OSQPInt,
    pub verbose: OSQPInt,
    pub profiler_level: OSQPInt,
    pub warm_starting: OSQPInt,
    pub scaling: OSQPInt,
    pub polishing: OSQPInt,
    pub rho: OSQPFloat,
    pub rho_is_vec: OSQPInt,
    pub sigma: OSQPFloat,
    pub alpha: OSQPFloat,
    pub cg_max_iter: OSQPInt,
    pub cg_tol_reduction: OSQPInt,
    pub cg_tol_fraction: OSQPFloat,
    pub cg_precond: osqp_precond_type,
    pub adaptive_rho: OSQPInt,
    pub adaptive_rho_interval: OSQPInt,
    pub adaptive_rho_fraction: OSQPFloat,
    pub adaptive_rho_tolerance: OSQPFloat,
    pub max_iter: OSQPInt,
    pub eps_abs: OSQPFloat,
    pub eps_rel: OSQPFloat,
    pub eps_prim_inf: OSQPFloat,
    pub eps_dual_inf: OSQPFloat,
    pub scaled_termination: OSQPInt,
    pub check_termination: OSQPInt,
    pub check_dualgap: OSQPInt,
    pub time_limit: OSQPFloat,
    pub delta: OSQPFloat,
    pub polish_refine_iter: OSQPInt,
}
pub type OSQPVectorf = OSQPVectorf_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSQPVectorf_ {
    pub values: *mut OSQPFloat,
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
pub type QDLDL_int = ::core::ffi::c_int;
pub type QDLDL_float = ::core::ffi::c_double;
pub type QDLDL_bool = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qdldl {
    pub type_0: osqp_linsys_solver_type,
    pub name: Option<unsafe extern "C" fn(*mut qdldl) -> *const ::core::ffi::c_char>,
    pub solve: Option<unsafe extern "C" fn(*mut qdldl, *mut OSQPVectorf, OSQPInt) -> OSQPInt>,
    pub update_settings: Option<unsafe extern "C" fn(*mut qdldl, *const OSQPSettings) -> ()>,
    pub warm_start: Option<unsafe extern "C" fn(*mut qdldl, *const OSQPVectorf) -> ()>,
    pub adjoint_derivative: Option<
        unsafe extern "C" fn(
            *mut *mut qdldl_solver,
            *const OSQPMatrix,
            *const OSQPMatrix,
            *const OSQPMatrix,
            *const OSQPMatrix,
            *const OSQPVectorf,
            *mut OSQPVectorf,
        ) -> OSQPInt,
    >,
    pub free: Option<unsafe extern "C" fn(*mut qdldl) -> ()>,
    pub update_matrices: Option<
        unsafe extern "C" fn(
            *mut qdldl,
            *const OSQPMatrix,
            *const OSQPInt,
            OSQPInt,
            *const OSQPMatrix,
            *const OSQPInt,
            OSQPInt,
        ) -> OSQPInt,
    >,
    pub update_rho_vec:
        Option<unsafe extern "C" fn(*mut qdldl, *const OSQPVectorf, OSQPFloat) -> OSQPInt>,
    pub nthreads: OSQPInt,
    pub L: *mut OSQPCscMatrix,
    pub Dinv: *mut OSQPFloat,
    pub P: *mut OSQPInt,
    pub bp: *mut OSQPFloat,
    pub sol: *mut OSQPFloat,
    pub rho_inv_vec: *mut OSQPFloat,
    pub sigma: OSQPFloat,
    pub rho_inv: OSQPFloat,
    pub polishing: OSQPInt,
    pub n: OSQPInt,
    pub m: OSQPInt,
    pub KKT: *mut OSQPCscMatrix,
    pub PtoKKT: *mut OSQPInt,
    pub AtoKKT: *mut OSQPInt,
    pub rhotoKKT: *mut OSQPInt,
    pub D: *mut QDLDL_float,
    pub etree: *mut QDLDL_int,
    pub Lnz: *mut QDLDL_int,
    pub iwork: *mut QDLDL_int,
    pub bwork: *mut QDLDL_bool,
    pub fwork: *mut QDLDL_float,
    pub adj: *mut OSQPCscMatrix,
}
pub type qdldl_solver = qdldl;
pub const OSQP_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AMD_INFO: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
#[export_name = "honest_osqp_update_settings_linsys_solver_qdldl"]
pub unsafe extern "C" fn update_settings_linsys_solver_qdldl(
    mut s: *mut qdldl_solver,
    mut settings: *const OSQPSettings,
) {
}
#[export_name = "honest_osqp_warm_start_linsys_solver_qdldl"]
pub unsafe extern "C" fn warm_start_linsys_solver_qdldl(
    mut s: *mut qdldl_solver,
    mut x: *const OSQPVectorf,
) {
}
#[export_name = "honest_osqp_free_linsys_solver_qdldl"]
pub unsafe extern "C" fn free_linsys_solver_qdldl(mut s: *mut qdldl_solver) {
    if !s.is_null() {
        if !(*s).L.is_null() {
            if !(*(*s).L).p.is_null() {
                free((*(*s).L).p as *mut ::core::ffi::c_void);
            }
            if !(*(*s).L).i.is_null() {
                free((*(*s).L).i as *mut ::core::ffi::c_void);
            }
            if !(*(*s).L).x.is_null() {
                free((*(*s).L).x as *mut ::core::ffi::c_void);
            }
            free((*s).L as *mut ::core::ffi::c_void);
        }
        if !(*s).P.is_null() {
            free((*s).P as *mut ::core::ffi::c_void);
        }
        if !(*s).Dinv.is_null() {
            free((*s).Dinv as *mut ::core::ffi::c_void);
        }
        if !(*s).bp.is_null() {
            free((*s).bp as *mut ::core::ffi::c_void);
        }
        if !(*s).sol.is_null() {
            free((*s).sol as *mut ::core::ffi::c_void);
        }
        if !(*s).rho_inv_vec.is_null() {
            free((*s).rho_inv_vec as *mut ::core::ffi::c_void);
        }
        if !(*s).KKT.is_null() {
            csc_spfree((*s).KKT);
        }
        if !(*s).PtoKKT.is_null() {
            free((*s).PtoKKT as *mut ::core::ffi::c_void);
        }
        if !(*s).AtoKKT.is_null() {
            free((*s).AtoKKT as *mut ::core::ffi::c_void);
        }
        if !(*s).rhotoKKT.is_null() {
            free((*s).rhotoKKT as *mut ::core::ffi::c_void);
        }
        if !(*s).adj.is_null() {
            free((*s).adj as *mut ::core::ffi::c_void);
        }
        if !(*s).D.is_null() {
            free((*s).D as *mut ::core::ffi::c_void);
        }
        if !(*s).etree.is_null() {
            free((*s).etree as *mut ::core::ffi::c_void);
        }
        if !(*s).Lnz.is_null() {
            free((*s).Lnz as *mut ::core::ffi::c_void);
        }
        if !(*s).iwork.is_null() {
            free((*s).iwork as *mut ::core::ffi::c_void);
        }
        if !(*s).bwork.is_null() {
            free((*s).bwork as *mut ::core::ffi::c_void);
        }
        if !(*s).fwork.is_null() {
            free((*s).fwork as *mut ::core::ffi::c_void);
        }
        free(s as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn LDL_factor(
    mut A: *mut OSQPCscMatrix,
    mut p: *mut qdldl_solver,
    mut nvar: OSQPInt,
) -> OSQPInt {
    let mut sum_Lnz: OSQPInt = 0;
    let mut factor_status: OSQPInt = 0;
    sum_Lnz = QDLDL_etree(
        (*A).n as QDLDL_int,
        (*A).p,
        (*A).i,
        (*p).iwork,
        (*p).Lnz,
        (*p).etree,
    ) as OSQPInt;
    if sum_Lnz < 0 as ::core::ffi::c_int {
        if !(sum_Lnz == -(1 as ::core::ffi::c_int)) {
            sum_Lnz == -(2 as ::core::ffi::c_int);
        }
        return sum_Lnz;
    }
    (*(*p).L).i =
        malloc((::core::mem::size_of::<OSQPInt>() as size_t).wrapping_mul(sum_Lnz as size_t))
            as *mut OSQPInt;
    (*(*p).L).x =
        malloc((::core::mem::size_of::<OSQPFloat>() as size_t).wrapping_mul(sum_Lnz as size_t))
            as *mut OSQPFloat;
    (*(*p).L).nzmax = sum_Lnz;
    factor_status = QDLDL_factor(
        (*A).n as QDLDL_int,
        (*A).p,
        (*A).i,
        (*A).x,
        (*(*p).L).p as *mut QDLDL_int,
        (*(*p).L).i as *mut QDLDL_int,
        (*(*p).L).x as *mut QDLDL_float,
        (*p).D,
        (*p).Dinv as *mut QDLDL_float,
        (*p).Lnz,
        (*p).etree,
        (*p).bwork,
        (*p).iwork,
        (*p).fwork,
    ) as OSQPInt;
    if factor_status < 0 as ::core::ffi::c_int {
        return factor_status;
    } else if factor_status < nvar {
        return -(2 as OSQPInt);
    }
    return 0 as OSQPInt;
}
unsafe extern "C" fn permute_KKT(
    mut KKT: *mut *mut OSQPCscMatrix,
    mut p: *mut qdldl_solver,
    mut Pnz: OSQPInt,
    mut Anz: OSQPInt,
    mut m: OSQPInt,
    mut PtoKKT: *mut OSQPInt,
    mut AtoKKT: *mut OSQPInt,
    mut rhotoKKT: *mut OSQPInt,
) -> OSQPInt {
    let mut info: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut amd_status: OSQPInt = 0;
    let mut Pinv: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut KtoPKPt: *mut OSQPInt = ::core::ptr::null_mut::<OSQPInt>();
    let mut i: OSQPInt = 0;
    let mut KKT_temp: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    info = malloc((AMD_INFO as size_t).wrapping_mul(::core::mem::size_of::<OSQPFloat>() as size_t))
        as *mut OSQPFloat;
    amd_status = amd_order(
        (**KKT).n as ::core::ffi::c_int,
        (**KKT).p as *const ::core::ffi::c_int,
        (**KKT).i as *const ::core::ffi::c_int,
        (*p).P as *mut ::core::ffi::c_int,
        ::core::ptr::null_mut::<OSQPFloat>(),
        info as *mut OSQPFloat,
    ) as OSQPInt;
    if amd_status < 0 as ::core::ffi::c_int {
        free(info as *mut ::core::ffi::c_void);
        return amd_status;
    }
    Pinv = csc_pinv((*p).P, (**KKT).n);
    if PtoKKT.is_null() && AtoKKT.is_null() && rhotoKKT.is_null() {
        KKT_temp = csc_symperm(*KKT, Pinv, ::core::ptr::null_mut::<OSQPInt>(), 1 as OSQPInt);
    } else {
        KtoPKPt = malloc(
            (*(**KKT).p.offset((**KKT).n as isize) as size_t)
                .wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t),
        ) as *mut OSQPInt;
        KKT_temp = csc_symperm(*KKT, Pinv, KtoPKPt, 1 as OSQPInt);
        if !PtoKKT.is_null() {
            i = 0 as ::core::ffi::c_int as OSQPInt;
            while i < Pnz {
                *PtoKKT.offset(i as isize) = *KtoPKPt.offset(*PtoKKT.offset(i as isize) as isize);
                i += 1;
            }
        }
        if !AtoKKT.is_null() {
            i = 0 as ::core::ffi::c_int as OSQPInt;
            while i < Anz {
                *AtoKKT.offset(i as isize) = *KtoPKPt.offset(*AtoKKT.offset(i as isize) as isize);
                i += 1;
            }
        }
        if !rhotoKKT.is_null() {
            i = 0 as ::core::ffi::c_int as OSQPInt;
            while i < m {
                *rhotoKKT.offset(i as isize) =
                    *KtoPKPt.offset(*rhotoKKT.offset(i as isize) as isize);
                i += 1;
            }
        }
        free(KtoPKPt as *mut ::core::ffi::c_void);
    }
    csc_spfree(*KKT);
    *KKT = KKT_temp;
    free(Pinv as *mut ::core::ffi::c_void);
    free(info as *mut ::core::ffi::c_void);
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_init_linsys_solver_qdldl"]
pub unsafe extern "C" fn init_linsys_solver_qdldl(
    mut sp: *mut *mut qdldl_solver,
    mut P: *const OSQPMatrix,
    mut A: *const OSQPMatrix,
    mut rho_vec: *const OSQPVectorf,
    mut settings: *const OSQPSettings,
    mut polishing: OSQPInt,
) -> OSQPInt {
    let mut KKT_temp: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    let mut i: OSQPInt = 0;
    let mut m: OSQPInt = 0;
    let mut n: OSQPInt = 0;
    let mut n_plus_m: OSQPInt = 0;
    let mut rhov: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    let mut sigma: OSQPFloat = (*settings).sigma;
    let mut s: *mut qdldl_solver = calloc(
        1 as size_t,
        ::core::mem::size_of::<qdldl_solver>() as size_t,
    ) as *mut qdldl_solver;
    *sp = s;
    n = (*(*P).csc).n;
    m = (*(*A).csc).m;
    (*s).n = n;
    (*s).m = m;
    n_plus_m = n + m;
    (*s).sigma = sigma;
    (*s).rho_inv = 1.0f64 / (*settings).rho;
    (*s).polishing = polishing;
    (*s).name =
        Some(name_qdldl as unsafe extern "C" fn(*mut qdldl_solver) -> *const ::core::ffi::c_char)
            as Option<unsafe extern "C" fn(*mut qdldl) -> *const ::core::ffi::c_char>;
    (*s).solve = Some(
        solve_linsys_qdldl
            as unsafe extern "C" fn(*mut qdldl_solver, *mut OSQPVectorf, OSQPInt) -> OSQPInt,
    )
        as Option<unsafe extern "C" fn(*mut qdldl, *mut OSQPVectorf, OSQPInt) -> OSQPInt>;
    (*s).update_settings = Some(
        update_settings_linsys_solver_qdldl
            as unsafe extern "C" fn(*mut qdldl_solver, *const OSQPSettings) -> (),
    )
        as Option<unsafe extern "C" fn(*mut qdldl, *const OSQPSettings) -> ()>;
    (*s).warm_start = Some(
        warm_start_linsys_solver_qdldl
            as unsafe extern "C" fn(*mut qdldl_solver, *const OSQPVectorf) -> (),
    ) as Option<unsafe extern "C" fn(*mut qdldl, *const OSQPVectorf) -> ()>;
    (*s).adjoint_derivative = Some(
        adjoint_derivative_qdldl
            as unsafe extern "C" fn(
                *mut *mut qdldl_solver,
                *const OSQPMatrix,
                *const OSQPMatrix,
                *const OSQPMatrix,
                *const OSQPMatrix,
                *const OSQPVectorf,
                *mut OSQPVectorf,
            ) -> OSQPInt,
    )
        as Option<
            unsafe extern "C" fn(
                *mut *mut qdldl_solver,
                *const OSQPMatrix,
                *const OSQPMatrix,
                *const OSQPMatrix,
                *const OSQPMatrix,
                *const OSQPVectorf,
                *mut OSQPVectorf,
            ) -> OSQPInt,
        >;
    (*s).free = Some(free_linsys_solver_qdldl as unsafe extern "C" fn(*mut qdldl_solver) -> ())
        as Option<unsafe extern "C" fn(*mut qdldl) -> ()>;
    (*s).update_matrices = Some(
        update_linsys_solver_matrices_qdldl
            as unsafe extern "C" fn(
                *mut qdldl_solver,
                *const OSQPMatrix,
                *const OSQPInt,
                OSQPInt,
                *const OSQPMatrix,
                *const OSQPInt,
                OSQPInt,
            ) -> OSQPInt,
    )
        as Option<
            unsafe extern "C" fn(
                *mut qdldl,
                *const OSQPMatrix,
                *const OSQPInt,
                OSQPInt,
                *const OSQPMatrix,
                *const OSQPInt,
                OSQPInt,
            ) -> OSQPInt,
        >;
    (*s).update_rho_vec = Some(
        update_linsys_solver_rho_vec_qdldl
            as unsafe extern "C" fn(*mut qdldl_solver, *const OSQPVectorf, OSQPFloat) -> OSQPInt,
    )
        as Option<unsafe extern "C" fn(*mut qdldl, *const OSQPVectorf, OSQPFloat) -> OSQPInt>;
    (*s).type_0 = OSQP_DIRECT_SOLVER;
    (*s).nthreads = 1 as ::core::ffi::c_int as OSQPInt;
    (*s).L = calloc(
        1 as size_t,
        ::core::mem::size_of::<OSQPCscMatrix>() as size_t,
    ) as *mut OSQPCscMatrix;
    (*(*s).L).m = n_plus_m;
    (*(*s).L).n = n_plus_m;
    (*(*s).L).nz = -(1 as ::core::ffi::c_int) as OSQPInt;
    (*(*s).L).p = malloc(
        ((n_plus_m as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<QDLDL_int>() as size_t),
    ) as *mut OSQPInt;
    (*s).Dinv =
        malloc((::core::mem::size_of::<QDLDL_float>() as size_t).wrapping_mul(n_plus_m as size_t))
            as *mut QDLDL_float as *mut OSQPFloat;
    (*s).D =
        malloc((::core::mem::size_of::<QDLDL_float>() as size_t).wrapping_mul(n_plus_m as size_t))
            as *mut QDLDL_float;
    (*s).P =
        malloc((::core::mem::size_of::<QDLDL_int>() as size_t).wrapping_mul(n_plus_m as size_t))
            as *mut QDLDL_int as *mut OSQPInt;
    (*s).bp =
        malloc((::core::mem::size_of::<QDLDL_float>() as size_t).wrapping_mul(n_plus_m as size_t))
            as *mut QDLDL_float as *mut OSQPFloat;
    (*s).sol =
        malloc((::core::mem::size_of::<QDLDL_float>() as size_t).wrapping_mul(n_plus_m as size_t))
            as *mut QDLDL_float as *mut OSQPFloat;
    if !rho_vec.is_null() {
        (*s).rho_inv_vec =
            malloc((::core::mem::size_of::<OSQPFloat>() as size_t).wrapping_mul(m as size_t))
                as *mut OSQPFloat;
    }
    (*s).etree =
        malloc((n_plus_m as size_t).wrapping_mul(::core::mem::size_of::<QDLDL_int>() as size_t))
            as *mut QDLDL_int;
    (*s).Lnz =
        malloc((n_plus_m as size_t).wrapping_mul(::core::mem::size_of::<QDLDL_int>() as size_t))
            as *mut QDLDL_int;
    (*(*s).L).i = ::core::ptr::null_mut::<OSQPInt>();
    (*(*s).L).x = ::core::ptr::null_mut::<OSQPFloat>();
    (*s).iwork = malloc(
        (::core::mem::size_of::<QDLDL_int>() as size_t)
            .wrapping_mul((3 as OSQPInt * n_plus_m) as size_t),
    ) as *mut QDLDL_int;
    (*s).bwork =
        malloc((::core::mem::size_of::<QDLDL_bool>() as size_t).wrapping_mul(n_plus_m as size_t))
            as *mut QDLDL_bool;
    (*s).fwork =
        malloc((::core::mem::size_of::<QDLDL_float>() as size_t).wrapping_mul(n_plus_m as size_t))
            as *mut QDLDL_float;
    if polishing != 0 {
        KKT_temp = form_KKT(
            (*P).csc,
            (*A).csc,
            0 as OSQPInt,
            sigma,
            (*s).rho_inv_vec,
            sigma,
            ::core::ptr::null_mut::<OSQPInt>(),
            ::core::ptr::null_mut::<OSQPInt>(),
            ::core::ptr::null_mut::<OSQPInt>(),
        );
        if !KKT_temp.is_null() {
            permute_KKT(
                &raw mut KKT_temp,
                s,
                OSQP_NULL,
                OSQP_NULL,
                OSQP_NULL,
                ::core::ptr::null_mut::<OSQPInt>(),
                ::core::ptr::null_mut::<OSQPInt>(),
                ::core::ptr::null_mut::<OSQPInt>(),
            );
        }
    } else {
        (*s).PtoKKT = malloc(
            (*(*(*P).csc).p.offset(n as isize) as size_t)
                .wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t),
        ) as *mut OSQPInt;
        (*s).AtoKKT = malloc(
            (*(*(*A).csc).p.offset(n as isize) as size_t)
                .wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t),
        ) as *mut OSQPInt;
        (*s).rhotoKKT =
            malloc((m as size_t).wrapping_mul(::core::mem::size_of::<OSQPInt>() as size_t))
                as *mut OSQPInt;
        if !rho_vec.is_null() {
            rhov = (*rho_vec).values;
            i = 0 as ::core::ffi::c_int as OSQPInt;
            while i < m {
                *(*s).rho_inv_vec.offset(i as isize) = 1.0f64 / *rhov.offset(i as isize);
                i += 1;
            }
        } else {
            (*s).rho_inv = 1.0f64 / (*settings).rho;
        }
        KKT_temp = form_KKT(
            (*P).csc,
            (*A).csc,
            0 as OSQPInt,
            sigma,
            (*s).rho_inv_vec,
            (*s).rho_inv,
            (*s).PtoKKT,
            (*s).AtoKKT,
            (*s).rhotoKKT,
        );
        if !KKT_temp.is_null() {
            permute_KKT(
                &raw mut KKT_temp,
                s,
                *(*(*P).csc).p.offset(n as isize),
                *(*(*A).csc).p.offset(n as isize),
                m,
                (*s).PtoKKT,
                (*s).AtoKKT,
                (*s).rhotoKKT,
            );
        }
    }
    if KKT_temp.is_null() {
        free_linsys_solver_qdldl(s);
        *sp = ::core::ptr::null_mut::<qdldl_solver>();
        return OSQP_LINSYS_SOLVER_INIT_ERROR as ::core::ffi::c_int as OSQPInt;
    }
    if LDL_factor(KKT_temp, s, n) < 0 as ::core::ffi::c_int {
        csc_spfree(KKT_temp);
        free_linsys_solver_qdldl(s);
        *sp = ::core::ptr::null_mut::<qdldl_solver>();
        return OSQP_NONCVX_ERROR as ::core::ffi::c_int as OSQPInt;
    }
    if polishing != 0 {
        csc_spfree(KKT_temp);
    } else {
        (*s).KKT = KKT_temp;
    }
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_name_qdldl"]
pub unsafe extern "C" fn name_qdldl(mut s: *mut qdldl_solver) -> *const ::core::ffi::c_char {
    return b"QDLDL v0.1.8\0" as *const u8 as *const ::core::ffi::c_char;
}
unsafe extern "C" fn LDLSolve(
    mut x: *mut OSQPFloat,
    mut b: *const OSQPFloat,
    mut L: *const OSQPCscMatrix,
    mut Dinv: *const OSQPFloat,
    mut P: *const OSQPInt,
    mut bp: *mut OSQPFloat,
) {
    let mut j: OSQPInt = 0;
    let mut n: OSQPInt = (*L).n;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        *bp.offset(j as isize) = *b.offset(*P.offset(j as isize) as isize);
        j += 1;
    }
    QDLDL_solve(
        (*L).n as QDLDL_int,
        (*L).p,
        (*L).i,
        (*L).x,
        Dinv as *const QDLDL_float,
        bp as *mut QDLDL_float,
    );
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        *x.offset(*P.offset(j as isize) as isize) = *bp.offset(j as isize);
        j += 1;
    }
}
#[export_name = "honest_osqp_solve_linsys_qdldl"]
pub unsafe extern "C" fn solve_linsys_qdldl(
    mut s: *mut qdldl_solver,
    mut b: *mut OSQPVectorf,
    mut admm_iter: OSQPInt,
) -> OSQPInt {
    let mut j: OSQPInt = 0;
    let mut n: OSQPInt = (*s).n;
    let mut m: OSQPInt = (*s).m;
    let mut bv: *mut OSQPFloat = (*b).values;
    if (*s).polishing != 0 {
        LDLSolve(bv, bv, (*s).L, (*s).Dinv, (*s).P, (*s).bp);
    } else {
        LDLSolve((*s).sol, bv, (*s).L, (*s).Dinv, (*s).P, (*s).bp);
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < n {
            *bv.offset(j as isize) = *(*s).sol.offset(j as isize);
            j += 1;
        }
        if !(*s).rho_inv_vec.is_null() {
            j = 0 as ::core::ffi::c_int as OSQPInt;
            while j < m {
                let ref mut fresh8 = *bv.offset((j + n) as isize);
                *fresh8 += (*(*s).rho_inv_vec.offset(j as isize)
                    * *(*s).sol.offset((j + n) as isize))
                    as ::core::ffi::c_double;
                j += 1;
            }
        } else {
            j = 0 as ::core::ffi::c_int as OSQPInt;
            while j < m {
                let ref mut fresh9 = *bv.offset((j + n) as isize);
                *fresh9 +=
                    ((*s).rho_inv * *(*s).sol.offset((j + n) as isize)) as ::core::ffi::c_double;
                j += 1;
            }
        }
    }
    return 0 as OSQPInt;
}
#[export_name = "honest_osqp_update_linsys_solver_matrices_qdldl"]
pub unsafe extern "C" fn update_linsys_solver_matrices_qdldl(
    mut s: *mut qdldl_solver,
    mut P: *const OSQPMatrix,
    mut Px_new_idx: *const OSQPInt,
    mut P_new_n: OSQPInt,
    mut A: *const OSQPMatrix,
    mut Ax_new_idx: *const OSQPInt,
    mut A_new_n: OSQPInt,
) -> OSQPInt {
    let mut pos_D_count: OSQPInt = 0;
    update_KKT_P(
        (*s).KKT,
        (*P).csc,
        Px_new_idx,
        P_new_n,
        (*s).PtoKKT,
        (*s).sigma,
        0 as OSQPInt,
    );
    update_KKT_A((*s).KKT, (*A).csc, Ax_new_idx, A_new_n, (*s).AtoKKT);
    pos_D_count = QDLDL_factor(
        (*(*s).KKT).n as QDLDL_int,
        (*(*s).KKT).p,
        (*(*s).KKT).i,
        (*(*s).KKT).x,
        (*(*s).L).p as *mut QDLDL_int,
        (*(*s).L).i as *mut QDLDL_int,
        (*(*s).L).x as *mut QDLDL_float,
        (*s).D,
        (*s).Dinv as *mut QDLDL_float,
        (*s).Lnz,
        (*s).etree,
        (*s).bwork,
        (*s).iwork,
        (*s).fwork,
    ) as OSQPInt;
    return if pos_D_count == (*(*P).csc).n {
        0 as OSQPInt
    } else {
        1 as OSQPInt
    };
}
#[export_name = "honest_osqp_update_linsys_solver_rho_vec_qdldl"]
pub unsafe extern "C" fn update_linsys_solver_rho_vec_qdldl(
    mut s: *mut qdldl_solver,
    mut rho_vec: *const OSQPVectorf,
    mut rho_sc: OSQPFloat,
) -> OSQPInt {
    let mut i: OSQPInt = 0;
    let mut retval: OSQPInt = 0 as OSQPInt;
    let mut m: OSQPInt = (*s).m;
    let mut rhov: *mut OSQPFloat = ::core::ptr::null_mut::<OSQPFloat>();
    if !(*s).rho_inv_vec.is_null() {
        rhov = (*rho_vec).values;
        i = 0 as ::core::ffi::c_int as OSQPInt;
        while i < m {
            *(*s).rho_inv_vec.offset(i as isize) = 1.0f64 / *rhov.offset(i as isize);
            i += 1;
        }
    } else {
        (*s).rho_inv = 1.0f64 / rho_sc;
    }
    update_KKT_param2(
        (*s).KKT,
        (*s).rho_inv_vec,
        (*s).rho_inv,
        (*s).rhotoKKT,
        (*s).m,
    );
    retval = QDLDL_factor(
        (*(*s).KKT).n as QDLDL_int,
        (*(*s).KKT).p,
        (*(*s).KKT).i,
        (*(*s).KKT).x,
        (*(*s).L).p as *mut QDLDL_int,
        (*(*s).L).i as *mut QDLDL_int,
        (*(*s).L).x as *mut QDLDL_float,
        (*s).D,
        (*s).Dinv as *mut QDLDL_float,
        (*s).Lnz,
        (*s).etree,
        (*s).bwork,
        (*s).iwork,
        (*s).fwork,
    ) as OSQPInt;
    return (retval < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn _colcount_diag(
    mut D: *mut OSQPCscMatrix,
    mut initcol: OSQPInt,
    mut blockcols: OSQPInt,
) {
    let mut j: OSQPInt = 0;
    j = initcol;
    while j < initcol + blockcols {
        let ref mut fresh5 = *(*D).p.offset(j as isize);
        *fresh5 += 1;
        j += 1;
    }
}
unsafe extern "C" fn _colcount_block(
    mut D: *mut OSQPCscMatrix,
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
            let ref mut fresh6 = *(*D)
                .p
                .offset((*(*M).i.offset(j as isize) + initcol) as isize);
            *fresh6 += 1;
            j += 1;
        }
    } else {
        j = 0 as ::core::ffi::c_int as OSQPInt;
        while j < (*M).n {
            let ref mut fresh7 = *(*D).p.offset((j + initcol) as isize);
            *fresh7 += (*(*M)
                .p
                .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                - *(*M).p.offset(j as isize)) as ::core::ffi::c_int;
            j += 1;
        }
    };
}
unsafe extern "C" fn _colcount_to_colptr(mut D: *mut OSQPCscMatrix) {
    let mut j: OSQPInt = 0;
    let mut count: OSQPInt = 0;
    let mut currentptr: OSQPInt = 0 as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j <= (*D).n {
        count = *(*D).p.offset(j as isize);
        *(*D).p.offset(j as isize) = currentptr;
        currentptr += count as ::core::ffi::c_int;
        j += 1;
    }
}
unsafe extern "C" fn _fill_block(
    mut K: *mut OSQPCscMatrix,
    mut M: *mut OSQPCscMatrix,
    mut index_mapping: *mut OSQPInt,
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
            let ref mut fresh3 = *(*K).p.offset(col as isize);
            let fresh4 = *fresh3;
            *fresh3 = *fresh3 + 1;
            dest = fresh4;
            *(*K).i.offset(dest as isize) = row;
            *(*K).x.offset(dest as isize) = *(*M).x.offset(jj as isize);
            if !index_mapping.is_null() {
                *index_mapping.offset(jj as isize) = dest;
            }
            jj += 1;
        }
        ii += 1;
    }
}
unsafe extern "C" fn _fill_diag_values(
    mut K: *mut OSQPCscMatrix,
    mut index_mapping: *mut OSQPInt,
    mut initrow: OSQPInt,
    mut initcol: OSQPInt,
    mut values: *mut OSQPFloat,
    mut value_scalar: OSQPFloat,
    mut n: OSQPInt,
) {
    let mut j: OSQPInt = 0;
    let mut dest: OSQPInt = 0;
    let mut row: OSQPInt = 0;
    let mut col: OSQPInt = 0;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < n {
        row = j + initrow;
        col = j + initcol;
        dest = *(*K).p.offset(col as isize);
        *(*K).i.offset(dest as isize) = row;
        if !values.is_null() {
            *(*K).x.offset(dest as isize) = *values.offset(j as isize);
        } else {
            *(*K).x.offset(dest as isize) = value_scalar;
        }
        let ref mut fresh2 = *(*K).p.offset(col as isize);
        *fresh2 += 1;
        if !index_mapping.is_null() {
            *index_mapping.offset(j as isize) = dest;
        }
        j += 1;
    }
}
unsafe extern "C" fn _backshift_colptrs(mut K: *mut OSQPCscMatrix) {
    let mut j: ::core::ffi::c_int = 0;
    j = (*K).n as ::core::ffi::c_int;
    while j > 0 as ::core::ffi::c_int {
        *(*K).p.offset(j as isize) = *(*K).p.offset((j - 1 as ::core::ffi::c_int) as isize);
        j -= 1;
    }
    *(*K).p.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int as OSQPInt;
}
unsafe extern "C" fn _adj_perturb(mut D: *mut OSQPCscMatrix, mut eps: OSQPFloat) {
    let mut j: OSQPInt = 0;
    let mut dest: OSQPInt = 0;
    dest = 0 as ::core::ffi::c_int as OSQPInt;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j < (*D).m as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
        dest = (*(*D)
            .p
            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as OSQPInt;
        let ref mut fresh0 = *(*D).x.offset(dest as isize);
        *fresh0 += eps as ::core::ffi::c_double;
        j += 1;
    }
    j = ((*D).m as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as OSQPInt;
    while j < (*D).m {
        dest = (*(*D)
            .p
            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as OSQPInt;
        let ref mut fresh1 = *(*D).x.offset(dest as isize);
        *fresh1 -= eps as ::core::ffi::c_double;
        j += 1;
    }
}
unsafe extern "C" fn _adj_assemble_csc(
    mut D: *mut OSQPCscMatrix,
    mut P_full: *const OSQPMatrix,
    mut G: *const OSQPMatrix,
    mut A_eq: *const OSQPMatrix,
    mut GDiagLambda: *const OSQPMatrix,
    mut slacks: *const OSQPVectorf,
) {
    let mut n: OSQPInt = OSQPMatrix_get_m(P_full);
    let mut x: OSQPInt = OSQPMatrix_get_m(G);
    let mut y: OSQPInt = OSQPMatrix_get_m(A_eq);
    let mut j: OSQPInt = 0;
    j = 0 as ::core::ffi::c_int as OSQPInt;
    while j <= 2 as OSQPInt * (n + x + y) {
        *(*D).p.offset(j as isize) = 0 as ::core::ffi::c_int as OSQPInt;
        j += 1;
    }
    _colcount_diag(D, 0 as OSQPInt, n + x + y);
    _colcount_block(D, (*P_full).csc, n + x + y, 0 as OSQPInt);
    _colcount_block(D, (*G).csc, n + x + y, 0 as OSQPInt);
    _colcount_block(D, (*A_eq).csc, n + x + y, 0 as OSQPInt);
    _colcount_block(D, (*GDiagLambda).csc, n + x + y + n, 1 as OSQPInt);
    _colcount_diag(D, n + x + y + n, x);
    _colcount_block(D, (*A_eq).csc, n + x + y + n + x, 1 as OSQPInt);
    _colcount_diag(D, n + x + y, n + x + y);
    _colcount_to_colptr(D);
    _fill_diag_values(
        D,
        ::core::ptr::null_mut::<OSQPInt>(),
        0 as OSQPInt,
        0 as OSQPInt,
        ::core::ptr::null_mut::<OSQPFloat>(),
        1 as ::core::ffi::c_int as OSQPFloat,
        n + x + y,
    );
    _fill_block(
        D,
        (*P_full).csc,
        ::core::ptr::null_mut::<OSQPInt>(),
        0 as OSQPInt,
        n + x + y,
        0 as OSQPInt,
    );
    _fill_block(
        D,
        (*G).csc,
        ::core::ptr::null_mut::<OSQPInt>(),
        n,
        n + x + y,
        0 as OSQPInt,
    );
    _fill_block(
        D,
        (*A_eq).csc,
        ::core::ptr::null_mut::<OSQPInt>(),
        n + x,
        n + x + y,
        0 as OSQPInt,
    );
    _fill_block(
        D,
        (*GDiagLambda).csc,
        ::core::ptr::null_mut::<OSQPInt>(),
        0 as OSQPInt,
        n + x + y + n,
        1 as OSQPInt,
    );
    _fill_diag_values(
        D,
        ::core::ptr::null_mut::<OSQPInt>(),
        n,
        n + x + y + n,
        (*slacks).values,
        0 as ::core::ffi::c_int as OSQPFloat,
        x,
    );
    _fill_block(
        D,
        (*A_eq).csc,
        ::core::ptr::null_mut::<OSQPInt>(),
        0 as OSQPInt,
        n + x + y + n + x,
        1 as OSQPInt,
    );
    _fill_diag_values(
        D,
        ::core::ptr::null_mut::<OSQPInt>(),
        n + x + y,
        n + x + y,
        ::core::ptr::null_mut::<OSQPFloat>(),
        0 as ::core::ffi::c_int as OSQPFloat,
        n + x + y,
    );
    _backshift_colptrs(D);
}
#[export_name = "honest_osqp_adjoint_derivative_qdldl"]
pub unsafe extern "C" fn adjoint_derivative_qdldl(
    mut s: *mut *mut qdldl_solver,
    mut P_full: *const OSQPMatrix,
    mut G: *const OSQPMatrix,
    mut A_eq: *const OSQPMatrix,
    mut GDiagLambda: *const OSQPMatrix,
    mut slacks: *const OSQPVectorf,
    mut rhs: *mut OSQPVectorf,
) -> OSQPInt {
    let mut k: OSQPInt = 0;
    let mut sol: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut residual: *mut OSQPVectorf = ::core::ptr::null_mut::<OSQPVectorf>();
    let mut adj_permuted: *mut OSQPCscMatrix = ::core::ptr::null_mut::<OSQPCscMatrix>();
    let mut amd_status: OSQPInt = 0;
    let mut An: QDLDL_int = 0;
    let mut i: QDLDL_int = 0;
    let mut Ln: QDLDL_int = 0;
    let mut Lx: *mut QDLDL_float = ::core::ptr::null_mut::<QDLDL_float>();
    let mut Li: *mut QDLDL_int = ::core::ptr::null_mut::<QDLDL_int>();
    let mut Lp: *mut QDLDL_int = ::core::ptr::null_mut::<QDLDL_int>();
    let mut D: *mut QDLDL_float = ::core::ptr::null_mut::<QDLDL_float>();
    let mut Dinv: *mut QDLDL_float = ::core::ptr::null_mut::<QDLDL_float>();
    let mut P: *mut QDLDL_int = ::core::ptr::null_mut::<QDLDL_int>();
    let mut Pinv: *mut QDLDL_int = ::core::ptr::null_mut::<QDLDL_int>();
    let mut etree: *mut QDLDL_int = ::core::ptr::null_mut::<QDLDL_int>();
    let mut Lnz: *mut QDLDL_int = ::core::ptr::null_mut::<QDLDL_int>();
    let mut sumLnz: QDLDL_int = 0;
    let mut iwork: *mut QDLDL_int = ::core::ptr::null_mut::<QDLDL_int>();
    let mut bwork: *mut QDLDL_bool = ::core::ptr::null_mut::<QDLDL_bool>();
    let mut fwork: *mut QDLDL_float = ::core::ptr::null_mut::<QDLDL_float>();
    let mut x: *mut QDLDL_float = ::core::ptr::null_mut::<QDLDL_float>();
    let mut x_work: *mut QDLDL_float = ::core::ptr::null_mut::<QDLDL_float>();
    let mut retval: OSQPInt = 0 as OSQPInt;
    let mut n: OSQPInt = OSQPMatrix_get_m(P_full);
    let mut n_ineq: OSQPInt = OSQPMatrix_get_m(G);
    let mut n_eq: OSQPInt = OSQPMatrix_get_m(A_eq);
    let mut P_full_nnz: OSQPInt = OSQPMatrix_get_nz(P_full);
    let mut G_nnz: OSQPInt = OSQPMatrix_get_nz(G);
    let mut A_eq_nnz: OSQPInt = OSQPMatrix_get_nz(A_eq);
    let mut nnzKKT: OSQPInt = n
        + n_ineq
        + n_eq
        + P_full_nnz
        + G_nnz
        + A_eq_nnz
        + G_nnz
        + n_ineq
        + A_eq_nnz
        + n
        + n_ineq
        + n_eq;
    let mut dim: OSQPInt = 2 as OSQPInt * (n + n_ineq + n_eq);
    let mut adj: *mut OSQPCscMatrix = csc_spalloc(dim, dim, nnzKKT, 1 as OSQPInt, 0 as OSQPInt);
    if adj.is_null() {
        return _osqp_error(
            OSQP_MEM_ALLOC_ERROR,
            b"adjoint_derivative_qdldl\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    _adj_assemble_csc(adj, P_full, G, A_eq, GDiagLambda, slacks);
    let mut adj_matrix: *mut OSQPMatrix = OSQPMatrix_new_from_csc(adj, 1 as OSQPInt);
    if adj_matrix.is_null() {
        retval = OSQP_MEM_ALLOC_ERROR as ::core::ffi::c_int as OSQPInt;
    } else {
        _adj_perturb(adj, 1e-6f64);
        An = dim as QDLDL_int;
        i = 0;
        Ln = An;
        Lx = ::core::ptr::null_mut::<QDLDL_float>();
        Li = ::core::ptr::null_mut::<QDLDL_int>();
        Lp = malloc(
            (::core::mem::size_of::<QDLDL_int>() as size_t)
                .wrapping_mul((An as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t),
        ) as *mut QDLDL_int;
        D = malloc((::core::mem::size_of::<QDLDL_float>() as size_t).wrapping_mul(An as size_t))
            as *mut QDLDL_float;
        Dinv = malloc((::core::mem::size_of::<QDLDL_float>() as size_t).wrapping_mul(An as size_t))
            as *mut QDLDL_float;
        P = malloc((::core::mem::size_of::<QDLDL_int>() as size_t).wrapping_mul(An as size_t))
            as *mut QDLDL_int;
        Pinv = ::core::ptr::null_mut::<QDLDL_int>();
        etree = malloc((::core::mem::size_of::<QDLDL_int>() as size_t).wrapping_mul(An as size_t))
            as *mut QDLDL_int;
        Lnz = malloc((::core::mem::size_of::<QDLDL_int>() as size_t).wrapping_mul(An as size_t))
            as *mut QDLDL_int;
        sumLnz = 0;
        iwork = malloc(
            (::core::mem::size_of::<QDLDL_int>() as size_t)
                .wrapping_mul((3 as QDLDL_int * An) as size_t),
        ) as *mut QDLDL_int;
        bwork = malloc((::core::mem::size_of::<QDLDL_bool>() as size_t).wrapping_mul(An as size_t))
            as *mut QDLDL_bool;
        fwork = malloc((::core::mem::size_of::<QDLDL_float>() as size_t).wrapping_mul(An as size_t))
            as *mut QDLDL_float;
        x = ::core::ptr::null_mut::<QDLDL_float>();
        x_work = ::core::ptr::null_mut::<QDLDL_float>();
        if Lp.is_null()
            || D.is_null()
            || Dinv.is_null()
            || P.is_null()
            || etree.is_null()
            || Lnz.is_null()
            || iwork.is_null()
            || bwork.is_null()
            || fwork.is_null()
        {
            retval = OSQP_MEM_ALLOC_ERROR as ::core::ffi::c_int as OSQPInt;
        } else {
            amd_status = 0;
            amd_status = amd_order(
                An as ::core::ffi::c_int,
                (*adj).p as *const ::core::ffi::c_int,
                (*adj).i as *const ::core::ffi::c_int,
                P as *mut ::core::ffi::c_int,
                ::core::ptr::null_mut::<OSQPFloat>(),
                ::core::ptr::null_mut::<OSQPFloat>(),
            ) as OSQPInt;
            if amd_status < 0 as ::core::ffi::c_int {
                retval = amd_status;
            } else {
                Pinv = csc_pinv(P, An as OSQPInt) as *mut QDLDL_int;
                if Pinv.is_null() {
                    retval = OSQP_MEM_ALLOC_ERROR as ::core::ffi::c_int as OSQPInt;
                } else {
                    adj_permuted = ::core::ptr::null_mut::<OSQPCscMatrix>();
                    adj_permuted =
                        csc_symperm(adj, Pinv, ::core::ptr::null_mut::<OSQPInt>(), 1 as OSQPInt);
                    if adj_permuted.is_null() {
                        retval = OSQP_MEM_ALLOC_ERROR as ::core::ffi::c_int as OSQPInt;
                    } else {
                        sumLnz = QDLDL_etree(
                            An,
                            (*adj_permuted).p,
                            (*adj_permuted).i,
                            iwork,
                            Lnz,
                            etree,
                        );
                        Li = malloc(
                            (::core::mem::size_of::<QDLDL_int>() as size_t)
                                .wrapping_mul(sumLnz as size_t),
                        ) as *mut QDLDL_int;
                        Lx = malloc(
                            (::core::mem::size_of::<QDLDL_float>() as size_t)
                                .wrapping_mul(sumLnz as size_t),
                        ) as *mut QDLDL_float;
                        if Li.is_null() || Lx.is_null() {
                            retval = OSQP_MEM_ALLOC_ERROR as ::core::ffi::c_int as OSQPInt;
                        } else {
                            QDLDL_factor(
                                An,
                                (*adj_permuted).p,
                                (*adj_permuted).i,
                                (*adj_permuted).x,
                                Lp,
                                Li,
                                Lx,
                                D,
                                Dinv,
                                Lnz,
                                etree,
                                bwork,
                                iwork,
                                fwork,
                            );
                            x = malloc(
                                (::core::mem::size_of::<QDLDL_float>() as size_t)
                                    .wrapping_mul(An as size_t),
                            ) as *mut QDLDL_float;
                            x_work = malloc(
                                (::core::mem::size_of::<QDLDL_float>() as size_t)
                                    .wrapping_mul(An as size_t),
                            ) as *mut QDLDL_float;
                            if x.is_null() || x_work.is_null() {
                                retval = OSQP_MEM_ALLOC_ERROR as ::core::ffi::c_int as OSQPInt;
                            } else {
                                i = 0 as ::core::ffi::c_int as QDLDL_int;
                                while i < An {
                                    *x_work.offset(i as isize) =
                                        *(*rhs).values.offset(*P.offset(i as isize) as isize)
                                            as QDLDL_float;
                                    i += 1;
                                }
                                QDLDL_solve(Ln, Lp, Li, Lx, Dinv, x_work);
                                i = 0 as ::core::ffi::c_int as QDLDL_int;
                                while i < An {
                                    *x.offset(*P.offset(i as isize) as isize) =
                                        *x_work.offset(i as isize);
                                    i += 1;
                                }
                                sol = OSQPVectorf_new(x, An as OSQPInt);
                                residual = OSQPVectorf_malloc(An as OSQPInt);
                                if sol.is_null() || residual.is_null() {
                                    retval = OSQP_MEM_ALLOC_ERROR as ::core::ffi::c_int as OSQPInt;
                                } else {
                                    k = 0;
                                    k = 0 as ::core::ffi::c_int as OSQPInt;
                                    while k < 200 as ::core::ffi::c_int {
                                        OSQPVectorf_copy(residual, rhs);
                                        OSQPMatrix_Axpy(
                                            adj_matrix,
                                            sol,
                                            residual,
                                            1 as ::core::ffi::c_int as OSQPFloat,
                                            -(1 as ::core::ffi::c_int) as OSQPFloat,
                                        );
                                        if OSQPVectorf_norm_2(residual) < 1e-12f64 {
                                            break;
                                        }
                                        i = 0 as ::core::ffi::c_int as QDLDL_int;
                                        while i < An {
                                            *x_work.offset(i as isize) = *(*residual)
                                                .values
                                                .offset(*P.offset(i as isize) as isize)
                                                as QDLDL_float;
                                            i += 1;
                                        }
                                        QDLDL_solve(Ln, Lp, Li, Lx, Dinv, x_work);
                                        i = 0 as ::core::ffi::c_int as QDLDL_int;
                                        while i < An {
                                            *(*residual)
                                                .values
                                                .offset(*P.offset(i as isize) as isize) =
                                                *x_work.offset(i as isize) as OSQPFloat;
                                            i += 1;
                                        }
                                        OSQPVectorf_minus(sol, sol, residual);
                                        k += 1;
                                    }
                                    OSQPVectorf_subvector_assign(
                                        rhs,
                                        OSQPVectorf_data(sol),
                                        0 as OSQPInt,
                                        OSQPVectorf_length(sol),
                                        1.0f64,
                                    );
                                }
                                OSQPVectorf_free(sol);
                                OSQPVectorf_free(residual);
                            }
                            free(x as *mut ::core::ffi::c_void);
                            free(x_work as *mut ::core::ffi::c_void);
                        }
                        free(Li as *mut ::core::ffi::c_void);
                        free(Lx as *mut ::core::ffi::c_void);
                    }
                    csc_spfree(adj_permuted);
                }
                free(Pinv as *mut ::core::ffi::c_void);
            }
        }
        free(Lp as *mut ::core::ffi::c_void);
        free(D as *mut ::core::ffi::c_void);
        free(Dinv as *mut ::core::ffi::c_void);
        free(P as *mut ::core::ffi::c_void);
        free(etree as *mut ::core::ffi::c_void);
        free(Lnz as *mut ::core::ffi::c_void);
        free(iwork as *mut ::core::ffi::c_void);
        free(bwork as *mut ::core::ffi::c_void);
        free(fwork as *mut ::core::ffi::c_void);
    }
    OSQPMatrix_free(adj_matrix);
    csc_spfree(adj);
    return retval;
}
